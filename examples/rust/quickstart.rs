//! Quickstart: build a state proof and verify it locally.

use lmprk_core::hash::Hash;
use lmprk_core::merkle::Side;
use lmprk_core::{
    hash_leaf, hash_node, MerklePath, MerkleStep, SlotInfo, SlotSnapshot, StateProofBuilder,
};

fn main() {
    let data = b"example-account-state".to_vec();
    let leaf = hash_leaf(&data);
    let sibling = hash_leaf(b"example-sibling");
    let root = hash_node(&leaf, &sibling);

    let snapshot = SlotSnapshot {
        head: SlotInfo {
            slot: 999_000,
            blockhash: Hash([4u8; 32]),
            parent_slot: 998_999,
            signer_count: 110,
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
        .address("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin")
        .account_data(data)
        .path(path)
        .build()
        .expect("builds");

    proof.verify().expect("verifies");
    println!(
        "proof verified for {} ({} bytes)",
        proof.address,
        proof.byte_size()
    );
}
