//! Lmprk verifier: aggregates Ed25519 signatures over slot roots.
//!
//! The verifier consumes a set of validator signatures over a slot/state-root
//! commitment and checks that at least `threshold` of them are valid for the
//! configured validator set.