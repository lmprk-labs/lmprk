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