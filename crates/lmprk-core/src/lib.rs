//! Lmprk core: light-client state proof primitives.
//!
//! The core crate exposes the building blocks for constructing and consuming
//! Solana light-client proofs: slot snapshots, merkle inclusion paths, and the
//! aggregate state proof that wraps them.

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod error;
pub mod hash;
pub mod merkle;
pub mod proof;
pub mod slot;

pub use error::{LmprkError, Result};
pub use hash::{hash_leaf, hash_node, Hash};
pub use merkle::{MerklePath, MerkleStep};
pub use proof::{StateProof, StateProofBuilder};
pub use slot::{SlotInfo, SlotSnapshot};