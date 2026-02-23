use std::io::{ErrorKind, Write};
use std::net::TcpStream;

use hmf_core::envelope::{Envelope, envelope_validate};
use hmf_wire_proto::wire::protobuf::{envelope_decode, envelope_encode};

use crate::error::{RecordError, TransportError};
use crate::record::{MAX_RECORD_LEN, record_decode, record_encode};

pub fn write_record(stream: &mut TcpStream, env: &Envelope) -> Result<(), TransportError> {
    envelope_validate(env)?;
    let payload = envelope_encode(env)?;
    let record = record_encode(&payload)?;
    stream.write_all(&record)?;
    Ok(())
}

pub fn read_record(stream: &mut TcpStream) -> Result<Option<Envelope>, TransportError> {
    let record_bytes = match record_decode(stream, MAX_RECORD_LEN) {
        Ok(b) => b,
        Err(RecordError::Io(e)) if is_closed(e.kind()) => return Ok(None),
        Err(e) => return Err(e.into()),
    };

    let env = envelope_decode(&record_bytes)?;
    envelope_validate(&env)?;
    Ok(Some(env))
}

fn is_closed(kind: ErrorKind) -> bool {
    matches!(
        kind,
        ErrorKind::UnexpectedEof | ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted
    )
}
