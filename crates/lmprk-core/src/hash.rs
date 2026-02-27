//! Hashing primitives.
//!
//! All on-the-wire hashes are blake3 with a domain separator. The two helpers
//! `hash_leaf` and `hash_node` follow the convention used by the verifier so
//! that proofs round-trip cleanly between producers and consumers.

use serde::{Deserialize, Serialize};