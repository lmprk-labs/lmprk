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

/// A single (signer_index, signature) pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBundle {
    /// Index into the configured signer set.
    pub signer_index: u32,
    /// Raw 64-byte Ed25519 signature.
    pub signature: Vec<u8>,
}

/// A collection of signature bundles for the same message.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignatureSet {
    /// All collected bundles.
    pub bundles: Vec<SignatureBundle>,
}