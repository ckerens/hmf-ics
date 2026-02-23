use std::io::Read;

use crate::error::RecordError;

pub const MAX_RECORD_LEN: usize = 1024 * 1024; // 1 MiB

/// Encode a payload into an HMF Record.
/// Format: [u32_be length][payload bytes]
pub fn record_encode(payload: &[u8]) -> Result<Vec<u8>, RecordError> {
    let len = payload.len();

    if len > MAX_RECORD_LEN {
        return Err(RecordError::TooLarge {
            len,
            max: MAX_RECORD_LEN,
        });
    }

    if len > u32::MAX as usize {
        return Err(RecordError::TooLarge {
            len,
            max: u32::MAX as usize,
        });
    }

    let mut out = Vec::with_capacity(4 + len);
    out.extend_from_slice(&(len as u32).to_be_bytes());
    out.extend_from_slice(payload);
    Ok(out)
}

/// Decode one HMF Record from a stream.
/// Reads exactly: [u32_be length][payload bytes]
pub fn record_decode<R: Read>(reader: &mut R, max_len: usize) -> Result<Vec<u8>, RecordError> {
    let mut len_buf = [0u8; 4];

    reader.read_exact(&mut len_buf)?;

    let len = u32::from_be_bytes(len_buf) as usize;
    if len > max_len {
        return Err(RecordError::TooLarge { len, max: max_len });
    }

    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload).map_err(|e| {
        if e.kind() == std::io::ErrorKind::UnexpectedEof {
            RecordError::Truncated {
                expected: len,
                got: 0,
            }
        } else {
            RecordError::Io(e)
        }
    })?;
    Ok(payload)
}
