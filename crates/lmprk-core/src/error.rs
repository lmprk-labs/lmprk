//! Error variants used across the core crate.

use thiserror::Error;

/// Anything that can go wrong while building or verifying a state proof.
#[derive(Debug, Error)]
pub enum LmprkError {
    /// The merkle path did not reproduce the expected root.
    #[error("merkle root mismatch: expected {expected}, got {actual}")]
    MerkleRootMismatch {
        /// The root the caller expected (hex encoded).
        expected: String,
        /// The root recomputed from the path (hex encoded).
        actual: String,
    },

    /// Signature aggregation did not reach the configured threshold.
    #[error("insufficient signatures: have {have}, need at least {need}")]
    InsufficientSignatures {
        /// Number of valid signatures collected.
        have: usize,
        /// Minimum required for the threshold.
        need: usize,
    },