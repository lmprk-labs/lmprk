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