use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

pub fn sign(signing_key: &SigningKey, msg: &[u8]) -> [u8; 64] {
    signing_key.sign(msg).to_bytes()
}

pub fn verify(verifying_key: &VerifyingKey, msg: &[u8], sig: &[u8]) -> bool {
    let Ok(sig) = Signature::from_slice(sig) else {
        return false;
    };
    verifying_key.verify(msg, &sig).is_ok()
}
