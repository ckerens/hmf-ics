pub mod sign;
pub mod signing_bytes;
mod types;

pub use types::*;

use crate::error::ValidateError;
use crate::ids::{DeviceId, IdempotencyKey, InstanceId, TransactionId};
use std::marker::PhantomData;

pub struct Unset;
pub struct Set;

/// A small builder used in the demo binaries
/// Not intended as the final API
pub struct EnvelopeBuilder<R, I, C, D> {
    proto_ver: u32,
    msg_class: MsgClass,
    delivery_profile: DeliveryProfile,

    sender_id: String,
    sender_instance: String,

    counter: u64,
    ttl_ms: u32,

    transaction_id: TransactionId,
    idempotency_key: IdempotencyKey,

    topic: String,
    target: String,
    scope: String,

    payload: Option<Payload>,

    sig_alg: SigAlg,
    signature: Vec<u8>,
    key_id: String,
    auth_context: Vec<u8>,

    _req: std::marker::PhantomData<R>,
    _inst: std::marker::PhantomData<I>,
    _ctr: std::marker::PhantomData<C>,
    _del: std::marker::PhantomData<D>,
}

impl EnvelopeBuilder<Unset, Unset, Unset, Unset> {
    pub fn new() -> Self {
        Self {
            proto_ver: EXPECTED_PROTO_VER,
            msg_class: MsgClass::Unspecified,
            delivery_profile: DeliveryProfile::BestEffort,

            sender_id: String::new(),
            sender_instance: String::new(),

            counter: 0,
            ttl_ms: 0,

            transaction_id: crate::ids::new_transaction_id(),
            idempotency_key: crate::ids::new_idempotency_key(),

            topic: String::new(),
            target: String::new(),
            scope: String::new(),

            payload: None,

            sig_alg: SigAlg::Unspecified,
            signature: Vec::new(),
            key_id: String::new(),
            auth_context: Vec::new(),

            _req: std::marker::PhantomData,
            _inst: std::marker::PhantomData,
            _ctr: std::marker::PhantomData,
            _del: std::marker::PhantomData,
        }
    }
}

impl<R, I, C, D> EnvelopeBuilder<R, I, C, D> {
    pub fn with_sender_id(self, sender_id: &str) -> EnvelopeBuilder<Set, I, C, D> {
        EnvelopeBuilder {
            proto_ver: self.proto_ver,
            msg_class: self.msg_class,
            delivery_profile: self.delivery_profile,

            sender_id: sender_id.to_string(),
            sender_instance: self.sender_instance,

            counter: self.counter,
            ttl_ms: self.ttl_ms,

            transaction_id: self.transaction_id,
            idempotency_key: self.idempotency_key,

            topic: self.topic,
            target: self.target,
            scope: self.scope,

            payload: self.payload,

            sig_alg: self.sig_alg,
            signature: self.signature,
            key_id: self.key_id,
            auth_context: self.auth_context,

            _req: PhantomData::<Set>,
            _inst: PhantomData::<I>,
            _ctr: PhantomData::<C>,
            _del: PhantomData::<D>,
        }
    }

    pub fn with_sender_instance(self, instance: &str) -> EnvelopeBuilder<R, Set, C, D> {
        EnvelopeBuilder {
            proto_ver: self.proto_ver,
            msg_class: self.msg_class,
            delivery_profile: self.delivery_profile,

            sender_id: self.sender_id,
            sender_instance: instance.to_string(),

            counter: self.counter,
            ttl_ms: self.ttl_ms,

            transaction_id: self.transaction_id,
            idempotency_key: self.idempotency_key,

            topic: self.topic,
            target: self.target,
            scope: self.scope,

            payload: self.payload,

            sig_alg: self.sig_alg,
            signature: self.signature,
            key_id: self.key_id,
            auth_context: self.auth_context,

            _req: PhantomData::<R>,
            _inst: PhantomData::<Set>,
            _ctr: PhantomData::<C>,
            _del: PhantomData::<D>,
        }
    }

    pub fn with_counter(self, counter: u64) -> EnvelopeBuilder<R, I, Set, D> {
        EnvelopeBuilder {
            proto_ver: self.proto_ver,
            msg_class: self.msg_class,
            delivery_profile: self.delivery_profile,

            sender_id: self.sender_id,
            sender_instance: self.sender_instance,

            counter,
            ttl_ms: self.ttl_ms,

            transaction_id: self.transaction_id,
            idempotency_key: self.idempotency_key,

            topic: self.topic,
            target: self.target,
            scope: self.scope,

            payload: self.payload,

            sig_alg: self.sig_alg,
            signature: self.signature,
            key_id: self.key_id,
            auth_context: self.auth_context,

            _req: PhantomData::<R>,
            _inst: PhantomData::<I>,
            _ctr: PhantomData::<Set>,
            _del: PhantomData::<D>,
        }
    }

    pub fn with_delivery_profile(self, profile: DeliveryProfile) -> EnvelopeBuilder<R, I, C, Set> {
        EnvelopeBuilder {
            proto_ver: self.proto_ver,
            msg_class: self.msg_class,
            delivery_profile: profile,

            sender_id: self.sender_id,
            sender_instance: self.sender_instance,

            counter: self.counter,
            ttl_ms: self.ttl_ms,

            transaction_id: self.transaction_id,
            idempotency_key: self.idempotency_key,

            topic: self.topic,
            target: self.target,
            scope: self.scope,

            payload: self.payload,

            sig_alg: self.sig_alg,
            signature: self.signature,
            key_id: self.key_id,
            auth_context: self.auth_context,

            _req: PhantomData::<R>,
            _inst: PhantomData::<I>,
            _ctr: PhantomData::<C>,
            _del: PhantomData::<Set>,
        }
    }

    pub fn with_ttl_ms(mut self, ttl_ms: u32) -> Self {
        self.ttl_ms = ttl_ms;
        self
    }

    pub fn with_topic(mut self, topic: &str) -> Self {
        self.topic = topic.to_string();
        self
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = target.to_string();
        self
    }

    pub fn with_scope(mut self, scope: &str) -> Self {
        self.scope = scope.to_string();
        self
    }

    pub fn with_payload(mut self, payload: Payload) -> Self {
        self.msg_class = match payload {
            Payload::Telemetry(_) => MsgClass::Telemetry,
            Payload::Command(_) => MsgClass::Command,
            Payload::Config(_) => MsgClass::Config,
            Payload::Engineering(_) => MsgClass::Engineering,
        };
        self.payload = Some(payload);
        self
    }

    pub fn with_security(
        mut self,
        sig_alg: SigAlg,
        key_id: &str,
        auth_context: Vec<u8>,
        signature: Vec<u8>,
    ) -> Self {
        self.sig_alg = sig_alg;
        self.key_id = key_id.to_string();
        self.auth_context = auth_context;
        self.signature = signature;
        self
    }
}

impl EnvelopeBuilder<Set, Set, Set, Set> {
    pub fn build(self) -> Envelope {
        Envelope {
            proto_ver: self.proto_ver,
            msg_class: self.msg_class,
            sender_id: DeviceId::new(self.sender_id),
            sender_instance: InstanceId::new(self.sender_instance),
            counter: self.counter,
            ttl_ms: self.ttl_ms,
            transaction_id: self.transaction_id,
            idempotency_key: self.idempotency_key,
            delivery_profile: self.delivery_profile,
            topic: self.topic,
            target: self.target,
            scope: self.scope,
            payload: self.payload,
            sig_alg: self.sig_alg,
            signature: self.signature,
            key_id: self.key_id,
            auth_context: self.auth_context,
        }
    }
}

pub fn envelope_validate(env: &Envelope) -> Result<(), ValidateError> {
    if env.proto_ver != EXPECTED_PROTO_VER {
        return Err(ValidateError::BadProtoVer {
            got: env.proto_ver,
            expected: EXPECTED_PROTO_VER,
        });
    }
    if env.ttl_ms == 0 {
        return Err(ValidateError::BadTtl);
    }
    if env.counter == 0 {
        return Err(ValidateError::BadCounter);
    }

    // Require Ed25519 only (strict mode)
    let crate::envelope::SigAlg::Ed25519 = env.sig_alg else {
        return Err(ValidateError::BadSigAlg {
            got: env.sig_alg.clone(),
        });
    };
    if env.signature.len() != 64 {
        return Err(ValidateError::BadSignatureLen {
            expected: 64,
            got: env.signature.len(),
        });
    }
    if env.key_id.is_empty() {
        return Err(ValidateError::BadKeyId);
    }

    let payload = env.payload.as_ref().ok_or(ValidateError::MissingPayload)?;

    let (expected, kind) = match payload {
        Payload::Telemetry(_) => (MsgClass::Telemetry, "telemetry"),
        Payload::Command(_) => (MsgClass::Command, "command"),
        Payload::Config(_) => (MsgClass::Config, "config"),
        Payload::Engineering(_) => (MsgClass::Engineering, "engineering"),
    };

    if env.msg_class != expected {
        return Err(ValidateError::MsgClassMismatch {
            msg_class: env.msg_class.clone(),
            payload_kind: kind,
        });
    }

    Ok(())
}
