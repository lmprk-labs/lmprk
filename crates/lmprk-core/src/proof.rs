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