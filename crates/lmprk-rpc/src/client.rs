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