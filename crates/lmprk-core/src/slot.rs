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

impl SlotInfo {
    /// Number of slots between this snapshot and its parent.
    pub fn slot_distance(&self) -> u64 {
        self.slot.saturating_sub(self.parent_slot)
    }
}

/// A snapshot of slot information used as the anchor for a proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotSnapshot {
    /// The most recent slot covered by the snapshot.
    pub head: SlotInfo,
    /// The state root committed at `head`.
    pub state_root: Hash,
    /// The number of validators considered in the aggregation set.
    pub validator_set_size: usize,
    /// The minimum signatures required to consider a slot finalized.
    pub threshold: usize,
}

impl SlotSnapshot {
    /// Returns true when the snapshot has reached the configured threshold.
    pub fn is_finalized(&self) -> bool {
        self.head.signer_count >= self.threshold
    }

    /// The slot range this snapshot is valid for.
    pub fn window(&self) -> (u64, u64) {
        let head = self.head.slot;
        let start = head.saturating_sub(WINDOW_RADIUS);
        (start, head)
    }
}

/// How many slots before `head` are considered part of the snapshot window.
pub const WINDOW_RADIUS: u64 = 32;

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_snapshot(slot: u64, signers: usize, threshold: usize) -> SlotSnapshot {
        SlotSnapshot {
            head: SlotInfo {
                slot,
                blockhash: Hash([0u8; 32]),
                parent_slot: slot.saturating_sub(1),
                signer_count: signers,
            },
            state_root: Hash([0u8; 32]),
            validator_set_size: 128,
            threshold,
        }
    }

    #[test]
    fn finalized_when_above_threshold() {
        let s = fake_snapshot(1000, 90, 86);
        assert!(s.is_finalized());
    }

    #[test]
    fn not_finalized_when_below_threshold() {
        let s = fake_snapshot(1000, 50, 86);
        assert!(!s.is_finalized());
    }

    #[test]
    fn window_respects_radius() {
        let s = fake_snapshot(1000, 90, 86);
        assert_eq!(s.window(), (968, 1000));
    }
}
