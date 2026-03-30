//! Integration test: full aggregate proof verification.

use ed25519_dalek::{Signer, SigningKey};
use rand_core::OsRng;

use lmprk_verifier::aggregation::{AggregateProof, SignatureBundle, SignatureSet, VerifierConfig};
use lmprk_verifier::commitment;
use lmprk_verifier::ed25519::SignerKey;

#[test]
fn aggregate_signatures_meet_threshold() {
    let mut signers = Vec::new();
    let mut keys = Vec::new();
    for _ in 0..5 {
        let signing = SigningKey::generate(&mut OsRng);
        let label = bs58::encode(signing.verifying_key().to_bytes()).into_string();
        signers.push(SignerKey::from_base58(&label, &label).expect("parses"));
        keys.push(signing);
    }
    let config = VerifierConfig::new(signers, 4);

    let slot = 12345u64;
    let root = [3u8; 32];
    let msg = commitment(slot, &root).to_vec();

    let mut set = SignatureSet::default();
    for (i, k) in keys.iter().enumerate().take(4) {
        set.push(SignatureBundle {
            signer_index: i as u32,
            signature: k.sign(&msg).to_bytes().to_vec(),
        });
    }

    let proof = AggregateProof { message: msg, signatures: set };
    let valid = proof.verify(&config).expect("verifies");
    assert!(valid >= 4);
}
