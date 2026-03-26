//! Threshold signature aggregation.

use serde::{Deserialize, Serialize};

use lmprk_core::error::{LmprkError, Result};

use crate::ed25519::{verify_signature, SignerKey};

/// Configuration for a verifier instance.
#[derive(Debug, Clone)]
pub struct VerifierConfig {
    /// The known set of signer keys, indexed by their position in the set.
    pub signers: Vec<SignerKey>,
    /// Minimum number of valid signatures required.
    pub threshold: usize,
}

impl VerifierConfig {
    /// Construct a new config; threshold is clamped to `signers.len()`.
    pub fn new(signers: Vec<SignerKey>, threshold: usize) -> Self {
        let t = threshold.min(signers.len());
        Self { signers, threshold: t }
    }
}