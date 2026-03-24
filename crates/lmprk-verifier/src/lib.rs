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