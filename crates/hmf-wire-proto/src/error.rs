use thiserror::Error;

#[derive(Debug, Error)]
pub enum WireError {
    #[error("protobuf decode error: {0}")]
    Decode(#[from] prost::DecodeError),

    #[error("protobuf encode error: {0}")]
    Encode(String),

    #[error("conversion error: {0}")]
    Convert(String),
}
