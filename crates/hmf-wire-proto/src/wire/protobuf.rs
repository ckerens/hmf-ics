use prost::Message;

use crate::convert::v1::{envelope_core_to_proto, envelope_proto_to_core};
use crate::error::WireError;
use crate::proto::v1 as proto;
use hmf_core::envelope::Envelope;

pub fn envelope_encode(env: &Envelope) -> Result<Vec<u8>, WireError> {
    let p: proto::Envelope = envelope_core_to_proto(env);
    Ok(p.encode_to_vec())
}

pub fn envelope_decode(bytes: &[u8]) -> Result<Envelope, WireError> {
    let p = proto::Envelope::decode(bytes)?;
    envelope_proto_to_core(p)
}
