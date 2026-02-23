use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidateError {
    #[error("unexpected proto version: {got}, expected {expected}")]
    BadProtoVer { got: u32, expected: u32 },

    #[error("missing payload")]
    MissingPayload,

    #[error("msg_class {msg_class:?} does not match payload kind {payload_kind}")]
    MsgClassMismatch {
        msg_class: crate::envelope::MsgClass,
        payload_kind: &'static str,
    },

    #[error("ttl_ms must be > 0")]
    BadTtl,

    #[error("counter must be > 0")]
    BadCounter,

    #[error("unsupported signature algorithm: {got:?}")]
    BadSigAlg { got: crate::envelope::SigAlg },

    #[error("invalid signature length: expected {expected} bytes, got {got}")]
    BadSignatureLen { expected: usize, got: usize },

    #[error("key_id must not be empty")]
    BadKeyId,
}
