//! Lmprk verifier: aggregates Ed25519 signatures over slot roots.
//!
//! The verifier consumes a set of validator signatures over a slot/state-root
//! commitment and checks that at least `threshold` of them are valid for the
//! configured validator set.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub mod aggregation;
pub mod ed25519;

pub use aggregation::{AggregateProof, SignatureBundle, SignatureSet, VerifierConfig};
pub use ed25519::{verify_signature, SignerKey};

/// Domain tag mixed into every commitment the verifier signs over.
pub const VERIFIER_DOMAIN: &[u8] = b"lmprk:verifier:v1";

/// Hash a slot/state-root commitment with the verifier domain, returning 32 bytes.
pub fn commitment(slot: u64, state_root: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(VERIFIER_DOMAIN);
    hasher.update(&slot.to_le_bytes());
    hasher.update(state_root);
    *hasher.finalize().as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commitment_is_deterministic() {
        let root = [7u8; 32];
        assert_eq!(commitment(1, &root), commitment(1, &root));
    }

    #[test]
    fn commitment_changes_with_slot() {
        let root = [7u8; 32];
        assert_ne!(commitment(1, &root), commitment(2, &root));
    }
}
