//! Merkle inclusion proofs.
//!
//! A `MerklePath` is the sequence of sibling hashes required to walk from a
//! leaf up to the root. Each `MerkleStep` carries the sibling hash and a
//! direction flag indicating whether the sibling sits on the left or right.

use serde::{Deserialize, Serialize};

use crate::error::{LmprkError, Result};
use crate::hash::{hash_node, Hash};

/// Direction of a sibling in the merkle path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    /// The sibling sits to the left; the current node is on the right.
    Left,
    /// The sibling sits to the right; the current node is on the left.
    Right,
}

/// One step in a merkle path: a sibling hash plus its side.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleStep {
    /// The sibling hash at this level.
    pub sibling: Hash,
    /// Which side the sibling is on.
    pub side: Side,
}

impl MerkleStep {
    /// Walk one level upwards, given the current accumulated hash.
    pub fn fold(&self, acc: &Hash) -> Hash {
        match self.side {
            Side::Left => hash_node(&self.sibling, acc),
            Side::Right => hash_node(acc, &self.sibling),
        }
    }
}

/// A complete merkle inclusion proof from a leaf to a root.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerklePath {
    /// The ordered steps from leaf upward.
    pub steps: Vec<MerkleStep>,
}

impl MerklePath {
    /// An empty path, used for single-leaf trees.
    pub fn empty() -> Self {
        Self { steps: Vec::new() }
    }

    /// Number of levels above the leaf.
    pub fn depth(&self) -> usize {
        self.steps.len()
    }

    /// Recompute the root, starting from the given leaf hash.
    pub fn compute_root(&self, leaf: &Hash) -> Hash {
        let mut acc = *leaf;
        for step in &self.steps {
            acc = step.fold(&acc);
        }
        acc
    }

    /// Verify the path against an expected root.
    pub fn verify(&self, leaf: &Hash, root: &Hash) -> Result<()> {
        let actual = self.compute_root(leaf);
        if &actual == root {
            Ok(())
        } else {
            Err(LmprkError::MerkleRootMismatch {
                expected: root.to_hex(),
                actual: actual.to_hex(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::hash_leaf;