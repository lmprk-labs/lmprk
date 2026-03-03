//! Hashing primitives.
//!
//! All on-the-wire hashes are blake3 with a domain separator. The two helpers
//! `hash_leaf` and `hash_node` follow the convention used by the verifier so
//! that proofs round-trip cleanly between producers and consumers.

use serde::{Deserialize, Serialize};

/// A 32-byte hash output.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    /// Construct from raw bytes.
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// View as a slice.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Hex-encoded form.
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hash({})", self.to_hex())
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

const LEAF_DOMAIN: &[u8] = b"lmprk:leaf:v1";
const NODE_DOMAIN: &[u8] = b"lmprk:node:v1";

/// Hash a leaf payload with the leaf domain separator.
pub fn hash_leaf(bytes: &[u8]) -> Hash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(LEAF_DOMAIN);
    hasher.update(bytes);
    Hash(*hasher.finalize().as_bytes())
}

/// Combine two child hashes into a parent hash with the node domain separator.
pub fn hash_node(left: &Hash, right: &Hash) -> Hash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(NODE_DOMAIN);
    hasher.update(&left.0);
    hasher.update(&right.0);
    Hash(*hasher.finalize().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;