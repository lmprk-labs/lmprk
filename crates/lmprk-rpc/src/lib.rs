//! Lmprk RPC: a small, failover-aware client policy.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod client;

pub use client::{RpcClient, RpcEndpoint, RpcError};
