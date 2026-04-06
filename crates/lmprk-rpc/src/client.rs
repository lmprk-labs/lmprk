//! A failover-aware RPC client model.
//!
//! This is the in-process abstraction the light client uses to pick the next
//! healthy endpoint. The actual HTTP transport is provided by the host service;
//! this crate only owns the policy.

use std::time::Duration;