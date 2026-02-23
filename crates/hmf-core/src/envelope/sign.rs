use ed25519_dalek::{SigningKey, VerifyingKey};

use crate::crypto::ed25519;
use crate::envelope::signing_bytes::envelope_signing_bytes;
use crate::envelope::{Envelope, SigAlg};

pub fn sign_envelope_ed25519(env: &mut Envelope, key_id: &str, signing_key: &SigningKey) {
    env.sig_alg = SigAlg::Ed25519;
    env.key_id = key_id.to_string();

    let msg = envelope_signing_bytes(env);
    let sig = ed25519::sign(signing_key, &msg);

    env.signature = sig.to_vec();
}

pub fn verify_envelope_ed25519(env: &Envelope, verifying_key: &VerifyingKey) -> bool {
    let SigAlg::Ed25519 = env.sig_alg else {
        return false;
    };

    if env.signature.len() != 64 {
        return false;
    }

    let msg = envelope_signing_bytes(env);
    ed25519::verify(verifying_key, &msg, &env.signature)
}
