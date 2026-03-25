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

impl SignerKey {
    /// Parse a base58 encoded 32-byte Solana public key.
    pub fn from_base58(label: impl Into<String>, value: &str) -> Result<Self> {
        let label = label.into();
        let raw = bs58::decode(value)
            .into_vec()
            .map_err(|_| LmprkError::InvalidSignature(label.clone()))?;
        if raw.len() != 32 {
            return Err(LmprkError::InvalidSignature(label));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&raw);
        let key = VerifyingKey::from_bytes(&arr)
            .map_err(|_| LmprkError::InvalidSignature(label.clone()))?;
        Ok(Self { key, label })
    }
}

/// Verify a 64-byte Ed25519 signature over `message` using the given key.
pub fn verify_signature(key: &SignerKey, message: &[u8], signature: &[u8]) -> Result<()> {
    if signature.len() != 64 {
        return Err(LmprkError::InvalidSignature(key.label.clone()));
    }
    let mut sig_bytes = [0u8; 64];
    sig_bytes.copy_from_slice(signature);
    let sig = Signature::from_bytes(&sig_bytes);
    key.key
        .verify(message, &sig)
        .map_err(|_| LmprkError::InvalidSignature(key.label.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use rand_core::OsRng;

    #[test]
    fn signature_round_trips() {
        let signing = SigningKey::generate(&mut OsRng);
        let verifying = signing.verifying_key();
        let label = bs58::encode(verifying.to_bytes()).into_string();
        let key = SignerKey::from_base58(&label, &label).expect("parses");
        let msg = b"slot-commitment";
        let sig = signing.sign(msg);
        verify_signature(&key, msg, &sig.to_bytes()).expect("verifies");
    }

    #[test]
    fn malformed_signature_is_rejected() {
        let signing = SigningKey::generate(&mut OsRng);
        let verifying = signing.verifying_key();
        let label = bs58::encode(verifying.to_bytes()).into_string();
        let key = SignerKey::from_base58(&label, &label).expect("parses");
        let bad = [0u8; 32];
        assert!(verify_signature(&key, b"x", &bad).is_err());
    }
}
