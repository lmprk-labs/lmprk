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