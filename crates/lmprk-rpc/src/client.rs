//! A failover-aware RPC client model.
//!
//! This is the in-process abstraction the light client uses to pick the next
//! healthy endpoint. The actual HTTP transport is provided by the host service;
//! this crate only owns the policy.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors the RPC client can surface.
#[derive(Debug, Error)]
pub enum RpcError {
    /// Every endpoint has been tried and failed.
    #[error("no healthy endpoint remaining")]
    AllEndpointsExhausted,
    /// An endpoint returned a non-success status code.
    #[error("endpoint {endpoint} returned status {status}")]
    BadStatus {
        /// The endpoint URL.
        endpoint: String,
        /// The status code returned.
        status: u16,
    },
    /// The endpoint replied but with a payload that could not be parsed.
    #[error("malformed response from {0}")]
    Malformed(String),
}

/// An RPC endpoint plus its current health bookkeeping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcEndpoint {
    /// Endpoint URL (no API key embedded; keys belong on the server only).
    pub url: String,
    /// A short human label.
    pub label: String,
    /// Number of consecutive failures.
    pub consecutive_failures: u32,
    /// Minimum backoff before retrying after a failure.
    pub backoff: Duration,
}

impl RpcEndpoint {
    /// Construct a fresh endpoint with zero failures.
    pub fn new(url: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            label: label.into(),
            consecutive_failures: 0,
            backoff: Duration::from_millis(250),
        }
    }

    /// Mark this endpoint as having failed; doubles the backoff up to 30s.
    pub fn note_failure(&mut self) {
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        let doubled = self.backoff.saturating_mul(2);
        let cap = Duration::from_secs(30);
        self.backoff = if doubled > cap { cap } else { doubled };
    }

    /// Mark this endpoint as healthy; resets the backoff.
    pub fn note_success(&mut self) {
        self.consecutive_failures = 0;
        self.backoff = Duration::from_millis(250);
    }

    /// True once an endpoint has failed enough times to be skipped.
    pub fn is_degraded(&self) -> bool {
        self.consecutive_failures >= 3
    }
}

/// Policy layer that picks the next endpoint to use.
#[derive(Debug, Clone)]
pub struct RpcClient {
    endpoints: Vec<RpcEndpoint>,
    cursor: usize,
}

impl RpcClient {
    /// Construct from a list of endpoints.
    pub fn new(endpoints: Vec<RpcEndpoint>) -> Self {
        Self { endpoints, cursor: 0 }
    }

    /// Number of registered endpoints.
    pub fn len(&self) -> usize {
        self.endpoints.len()
    }

    /// True if there are no endpoints.
    pub fn is_empty(&self) -> bool {
        self.endpoints.is_empty()
    }

    /// Pick the next healthy endpoint, skipping degraded ones.
    pub fn pick(&mut self) -> Result<&mut RpcEndpoint, RpcError> {
        if self.endpoints.is_empty() {
            return Err(RpcError::AllEndpointsExhausted);
        }
        for _ in 0..self.endpoints.len() {
            let idx = self.cursor % self.endpoints.len();
            self.cursor = self.cursor.wrapping_add(1);
            if !self.endpoints[idx].is_degraded() {
                return Ok(&mut self.endpoints[idx]);
            }
        }
        Err(RpcError::AllEndpointsExhausted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ep(label: &str) -> RpcEndpoint {
        RpcEndpoint::new(format!("https://{}.example", label), label)
    }

    #[test]
    fn pick_rotates_through_endpoints() {
        let mut client = RpcClient::new(vec![ep("a"), ep("b"), ep("c")]);
        let mut seen = Vec::new();
        for _ in 0..3 {
            let label = client.pick().expect("has endpoint").label.clone();
            seen.push(label);
        }
        seen.sort();
        assert_eq!(seen, vec!["a", "b", "c"]);
    }

    #[test]
    fn degraded_endpoint_is_skipped() {
        let mut client = RpcClient::new(vec![ep("a"), ep("b")]);
        for _ in 0..3 {
            client.endpoints[0].note_failure();
        }
        assert!(client.endpoints[0].is_degraded());
        let next = client.pick().expect("picks");
        assert_eq!(next.label, "b");
    }

    #[test]
    fn note_success_resets_backoff() {
        let mut e = ep("a");
        e.note_failure();
        e.note_failure();
        assert!(e.backoff > Duration::from_millis(250));
        e.note_success();
        assert_eq!(e.backoff, Duration::from_millis(250));
    }
}
