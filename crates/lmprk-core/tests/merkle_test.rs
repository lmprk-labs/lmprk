//! Integration test: deep merkle paths still verify in linear time.

use lmprk_core::hash::{hash_leaf, hash_node, Hash};
use lmprk_core::merkle::{MerklePath, MerkleStep, Side};

#[test]
fn deep_path_verifies() {
    let leaf = hash_leaf(b"deep-leaf");

    let mut current = leaf;
    let mut steps = Vec::new();

    for i in 0..16u32 {
        let sib_bytes = format!("sib-{}", i);
        let sib = hash_leaf(sib_bytes.as_bytes());
        if i % 2 == 0 {
            current = hash_node(&current, &sib);
            steps.push(MerkleStep { sibling: sib, side: Side::Right });
        } else {
            current = hash_node(&sib, &current);
            steps.push(MerkleStep { sibling: sib, side: Side::Left });
        }
    }

    let path = MerklePath { steps };
    assert_eq!(path.compute_root(&leaf), current);
    path.verify(&leaf, &current).expect("verifies");

    let wrong: Hash = hash_leaf(b"not-the-root");
    assert!(path.verify(&leaf, &wrong).is_err());
}
