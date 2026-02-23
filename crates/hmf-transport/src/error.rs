use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Record(#[from] RecordError),

    #[error(transparent)]
    Wire(#[from] hmf_wire_proto::error::WireError),

    #[error(transparent)]
    Validate(#[from] hmf_core::error::ValidateError),
}

#[derive(Debug, Error)]
pub enum RecordError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("record too large: {len} > {max}")]
    TooLarge { len: usize, max: usize },

    #[error("truncated record: expected {expected} bytes, got {got}")]
    Truncated { expected: usize, got: usize },
}
