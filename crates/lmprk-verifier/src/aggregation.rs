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

impl SignatureSet {
    /// Append a bundle to the set.
    pub fn push(&mut self, bundle: SignatureBundle) {
        self.bundles.push(bundle);
    }

    /// Number of collected bundles (not de-duplicated).
    pub fn len(&self) -> usize {
        self.bundles.len()
    }

    /// True when no bundles have been collected.
    pub fn is_empty(&self) -> bool {
        self.bundles.is_empty()
    }
}

/// The aggregate proof a verifier consumes: the message plus the signature set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateProof {
    /// The commitment bytes that every signer signed over.
    pub message: Vec<u8>,
    /// Signatures collected so far.
    pub signatures: SignatureSet,
}

impl AggregateProof {
    /// Verify the aggregate proof against a verifier configuration.
    ///
    /// Returns `Ok(valid_count)` when the threshold is reached, otherwise the
    /// corresponding `LmprkError` variant.
    pub fn verify(&self, config: &VerifierConfig) -> Result<usize> {
        let mut valid = 0usize;
        let mut seen = vec![false; config.signers.len()];
        for bundle in &self.signatures.bundles {
            let idx = bundle.signer_index as usize;
            if idx >= config.signers.len() || seen[idx] {
                continue;
            }
            let signer = &config.signers[idx];
            if verify_signature(signer, &self.message, &bundle.signature).is_ok() {
                seen[idx] = true;
                valid += 1;
            }
        }
        if valid >= config.threshold {
            Ok(valid)
        } else {
            Err(LmprkError::InsufficientSignatures {
                have: valid,
                need: config.threshold,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use rand_core::OsRng;