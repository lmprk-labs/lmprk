//! Slot snapshots: the per-slot state captured by the light client.

use serde::{Deserialize, Serialize};

use crate::hash::Hash;

/// Lightweight metadata for a Solana slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotInfo {
    /// Slot number.
    pub slot: u64,
    /// Block hash at this slot.
    pub blockhash: Hash,
    /// Parent slot.
    pub parent_slot: u64,
    /// Number of validators that signed this slot in the snapshot window.
    pub signer_count: usize,
}