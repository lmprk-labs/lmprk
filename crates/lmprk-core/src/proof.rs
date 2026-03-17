//! State proofs: the bundle a client uses to verify a Solana account.

use serde::{Deserialize, Serialize};

use crate::error::{LmprkError, Result};
use crate::hash::{hash_leaf, Hash};
use crate::merkle::MerklePath;
use crate::slot::SlotSnapshot;
use crate::{PROTOCOL_NAME, PROTOCOL_VERSION};

/// A complete light-client proof for a single account at a given slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateProof {
    /// Protocol identifier; verifiers reject anything else.
    pub protocol: String,
    /// Protocol version.
    pub version: u16,
    /// The slot snapshot the proof anchors against.
    pub snapshot: SlotSnapshot,
    /// The account address being proven, base58 encoded.
    pub address: String,
    /// Raw account data; the verifier hashes this as the leaf.
    pub account_data: Vec<u8>,
    /// Merkle path from the account leaf to `snapshot.state_root`.
    pub path: MerklePath,
}

impl StateProof {
    /// Verify the proof against its embedded snapshot root.
    pub fn verify(&self) -> Result<()> {
        if self.protocol != PROTOCOL_NAME || self.version != PROTOCOL_VERSION {
            return Err(LmprkError::Malformed("protocol mismatch"));
        }
        if !self.snapshot.is_finalized() {
            return Err(LmprkError::InsufficientSignatures {
                have: self.snapshot.head.signer_count,
                need: self.snapshot.threshold,
            });
        }
        let leaf = hash_leaf(&self.account_data);
        self.path.verify(&leaf, &self.snapshot.state_root)
    }

    /// Best-effort byte size estimate, useful for diagnostics.
    pub fn byte_size(&self) -> usize {
        8 + 96 + self.account_data.len() + self.path.steps.len() * 33
    }
}

/// Builder for `StateProof`. Convenient when assembling a proof in stages.
#[derive(Debug, Default)]
pub struct StateProofBuilder {
    snapshot: Option<SlotSnapshot>,
    address: Option<String>,
    account_data: Option<Vec<u8>>,
    path: Option<MerklePath>,
}

impl StateProofBuilder {
    /// Start a fresh builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach the slot snapshot.
    pub fn snapshot(mut self, snapshot: SlotSnapshot) -> Self {
        self.snapshot = Some(snapshot);
        self
    }

    /// Set the account address (base58).
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Set the raw account data.
    pub fn account_data(mut self, data: Vec<u8>) -> Self {
        self.account_data = Some(data);
        self
    }

    /// Attach the merkle inclusion path.
    pub fn path(mut self, path: MerklePath) -> Self {
        self.path = Some(path);
        self
    }

    /// Finalize into a `StateProof`. Returns an error if any required field is missing.
    pub fn build(self) -> Result<StateProof> {
        let snapshot = self.snapshot.ok_or(LmprkError::Malformed("snapshot missing"))?;
        let address = self.address.ok_or(LmprkError::Malformed("address missing"))?;
        let account_data = self
            .account_data
            .ok_or(LmprkError::Malformed("account_data missing"))?;
        let path = self.path.unwrap_or_else(MerklePath::empty);
        Ok(StateProof {
            protocol: PROTOCOL_NAME.to_string(),
            version: PROTOCOL_VERSION,
            snapshot,
            address,
            account_data,
            path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::{hash_leaf, hash_node};
    use crate::merkle::{MerkleStep, Side};
    use crate::slot::SlotInfo;

    fn snap(root: Hash) -> SlotSnapshot {
        SlotSnapshot {
            head: SlotInfo {
                slot: 1000,
                blockhash: Hash([0u8; 32]),
                parent_slot: 999,
                signer_count: 100,
            },
            state_root: root,
            validator_set_size: 128,
            threshold: 86,
        }
    }

    #[test]
    fn proof_round_trips() {
        let data = b"some-account-data".to_vec();
        let leaf = hash_leaf(&data);
        let sibling = hash_leaf(b"sibling");
        let root = hash_node(&leaf, &sibling);
        let path = MerklePath {
            steps: vec![MerkleStep { sibling, side: Side::Right }],
        };
        let proof = StateProofBuilder::new()
            .snapshot(snap(root))
            .address("So11111111111111111111111111111111111111112")
            .account_data(data)
            .path(path)
            .build()
            .expect("builds");
        proof.verify().expect("verifies");
    }

    #[test]
    fn missing_field_is_rejected() {
        let err = StateProofBuilder::new().build().unwrap_err();
        assert!(matches!(err, LmprkError::Malformed(_)));
    }
}
