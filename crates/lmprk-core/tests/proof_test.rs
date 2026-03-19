//! Integration test: build a proof and verify it end-to-end.

use lmprk_core::hash::Hash;
use lmprk_core::merkle::Side;
use lmprk_core::{
    hash_leaf, hash_node, MerklePath, MerkleStep, SlotInfo, SlotSnapshot, StateProofBuilder,
};

#[test]
fn end_to_end_single_level_path() {
    let data = b"account-payload".to_vec();
    let leaf = hash_leaf(&data);
    let sibling = hash_leaf(b"some-sibling");
    let root = hash_node(&leaf, &sibling);

    let snapshot = SlotSnapshot {
        head: SlotInfo {
            slot: 12345,
            blockhash: Hash([1u8; 32]),
            parent_slot: 12344,
            signer_count: 100,
        },
        state_root: root,
        validator_set_size: 128,
        threshold: 86,
    };

    let path = MerklePath {
        steps: vec![MerkleStep { sibling, side: Side::Right }],
    };

    let proof = StateProofBuilder::new()
        .snapshot(snapshot)
        .address("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
        .account_data(data)
        .path(path)
        .build()
        .expect("builds");

    proof.verify().expect("verifies");
    assert!(proof.byte_size() > 0);
}
