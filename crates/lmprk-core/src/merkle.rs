//! Merkle inclusion proofs.
//!
//! A `MerklePath` is the sequence of sibling hashes required to walk from a
//! leaf up to the root. Each `MerkleStep` carries the sibling hash and a
//! direction flag indicating whether the sibling sits on the left or right.

use serde::{Deserialize, Serialize};