//! Ed25519 helpers; thin layer above `ed25519-dalek`.

use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use lmprk_core::error::{LmprkError, Result};

/// A validator's Ed25519 verifying key, base58 encoded externally.
#[derive(Debug, Clone)]
pub struct SignerKey {
    /// The parsed verifying key.
    pub key: VerifyingKey,
    /// The base58 identifier kept around for error messages.
    pub label: String,
}