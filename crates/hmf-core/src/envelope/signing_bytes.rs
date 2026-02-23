use sha2::{Digest, Sha256};

use crate::envelope::*;

const DOMAIN_TAG: &[u8] = b"HMFv1:envelope-signature";

fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().into()
}

fn put_u8(buf: &mut Vec<u8>, v: u8) {
    buf.push(v);
}
fn put_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_be_bytes());
}
fn put_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_be_bytes());
}
fn put_i32(buf: &mut Vec<u8>, v: i32) {
    buf.extend_from_slice(&v.to_be_bytes());
}
fn put_i64(buf: &mut Vec<u8>, v: i64) {
    buf.extend_from_slice(&v.to_be_bytes());
}
fn put_bool(buf: &mut Vec<u8>, v: bool) {
    buf.push(if v { 1 } else { 0 });
}
fn put_f64(buf: &mut Vec<u8>, v: f64) {
    buf.extend_from_slice(&v.to_bits().to_be_bytes());
}

fn put_bytes(buf: &mut Vec<u8>, b: &[u8]) {
    put_u32(buf, b.len() as u32);
    buf.extend_from_slice(b);
}
fn put_str(buf: &mut Vec<u8>, s: &str) {
    put_bytes(buf, s.as_bytes());
}

fn put_opt<T>(buf: &mut Vec<u8>, o: Option<&T>, f: impl FnOnce(&mut Vec<u8>, &T)) {
    match o {
        Some(v) => {
            put_u8(buf, 1);
            f(buf, v);
        }
        None => put_u8(buf, 0),
    }
}

fn put_map_sorted(buf: &mut Vec<u8>, map: &std::collections::BTreeMap<String, String>) {
    put_u32(buf, map.len() as u32);
    for (k, v) in map {
        put_str(buf, k);
        put_str(buf, v);
    }
}

pub fn canonical_payload_bytes(payload: &Payload) -> Vec<u8> {
    let mut buf = Vec::with_capacity(256);
    match payload {
        Payload::Telemetry(t) => {
            put_u8(&mut buf, 1);
            canonical_telemetry(&mut buf, t);
        }
        Payload::Command(c) => {
            put_u8(&mut buf, 2);
            canonical_command(&mut buf, c);
        }
        Payload::Config(c) => {
            put_u8(&mut buf, 3);
            canonical_config(&mut buf, c);
        }
        Payload::Engineering(e) => {
            put_u8(&mut buf, 4);
            canonical_engineering(&mut buf, e);
        }
    }
    buf
}

pub fn payload_hash(payload: &Payload) -> [u8; 32] {
    sha256(&canonical_payload_bytes(payload))
}

pub fn envelope_signing_bytes(env: &Envelope) -> Vec<u8> {
    let mut buf = Vec::with_capacity(512);

    put_bytes(&mut buf, DOMAIN_TAG);

    // header
    put_u32(&mut buf, env.proto_ver);
    put_i32(&mut buf, env.msg_class.to_i32());
    put_str(&mut buf, env.sender_id.as_str());
    put_str(&mut buf, env.sender_instance.as_str());
    put_u64(&mut buf, env.counter);
    put_u32(&mut buf, env.ttl_ms);
    put_str(&mut buf, env.transaction_id.as_str());
    put_str(&mut buf, env.idempotency_key.as_str());
    put_i32(&mut buf, env.delivery_profile.to_i32());

    // routing hints
    put_str(&mut buf, &env.topic);
    put_str(&mut buf, &env.target);
    put_str(&mut buf, &env.scope);

    // security header (exclude signature bytes)
    put_i32(&mut buf, env.sig_alg.to_i32());
    put_str(&mut buf, &env.key_id);
    let ac_hash = sha256(&env.auth_context);
    buf.extend_from_slice(&ac_hash);

    // payload hash
    let payload = env
        .payload
        .as_ref()
        .expect("validated envelope must have payload");
    let ph = payload_hash(payload);
    buf.extend_from_slice(&ph);

    buf
}

fn canonical_ack(buf: &mut Vec<u8>, a: &Ack) {
    put_i32(buf, a.status.to_i32());
    put_str(buf, &a.detail);
}

fn canonical_op_result(buf: &mut Vec<u8>, r: &OpResult) {
    put_i32(buf, r.status.to_i32());
    put_str(buf, &r.detail);
}

fn canonical_telemetry(buf: &mut Vec<u8>, t: &Telemetry) {
    match t.payload.as_ref() {
        None => put_u8(buf, 0),
        Some(TelemetryPayload::Birth(b)) => {
            put_u8(buf, 1);
            canonical_birth(buf, b);
        }
        Some(TelemetryPayload::Heartbeat(h)) => {
            put_u8(buf, 2);
            canonical_heartbeat(buf, h);
        }
        Some(TelemetryPayload::Death(d)) => {
            put_u8(buf, 3);
            canonical_death(buf, d);
        }
        Some(TelemetryPayload::State(s)) => {
            put_u8(buf, 4);
            canonical_state_update(buf, s);
        }
        Some(TelemetryPayload::Alarm(a)) => {
            put_u8(buf, 5);
            canonical_alarm(buf, a);
        }
        Some(TelemetryPayload::Observation(o)) => {
            put_u8(buf, 6);
            canonical_observation(buf, o);
        }
    }
}

fn canonical_birth(buf: &mut Vec<u8>, b: &LifecycleBirth) {
    put_str(buf, &b.vendor);
    put_str(buf, &b.model);
    put_str(buf, &b.serial);
    put_str(buf, &b.hw_rev);
    put_str(buf, &b.fw_rev);
    put_u64(buf, b.uptime_ms);
    put_u32(buf, b.capabilities.len() as u32);
    for c in &b.capabilities {
        put_str(buf, c);
    }
}

fn canonical_heartbeat(buf: &mut Vec<u8>, h: &LifecycleHeartbeat) {
    put_u64(buf, h.uptime_ms);
    put_i32(buf, h.health.to_i32());
}

fn canonical_death(buf: &mut Vec<u8>, d: &LifecycleDeath) {
    put_i32(buf, d.reason.to_i32());
    put_u64(buf, d.uptime_ms);
    put_str(buf, &d.detail);
}

fn canonical_state_update(buf: &mut Vec<u8>, s: &StateUpdate) {
    put_u64(buf, s.local_seq);
    put_u32(buf, s.items.len() as u32);
    for it in &s.items {
        canonical_state_item(buf, it);
    }
}

fn canonical_state_item(buf: &mut Vec<u8>, it: &StateItem) {
    put_str(buf, &it.key);
    put_str(buf, &it.unit);
    put_i32(buf, it.quality.to_i32());

    match it.value.as_ref() {
        None => put_u8(buf, 0),
        Some(StateValue::F64(v)) => {
            put_u8(buf, 1);
            put_f64(buf, *v);
        }
        Some(StateValue::I64(v)) => {
            put_u8(buf, 2);
            put_i64(buf, *v);
        }
        Some(StateValue::U64(v)) => {
            put_u8(buf, 3);
            put_u64(buf, *v);
        }
        Some(StateValue::B(v)) => {
            put_u8(buf, 4);
            put_bool(buf, *v);
        }
        Some(StateValue::S(v)) => {
            put_u8(buf, 5);
            put_str(buf, v);
        }
        Some(StateValue::Blob(v)) => {
            put_u8(buf, 6);
            put_bytes(buf, v);
        }
    }
}

fn canonical_alarm(buf: &mut Vec<u8>, a: &Alarm) {
    put_i32(buf, a.severity.to_i32());
    put_str(buf, &a.code);
    put_str(buf, &a.summary);
    put_str(buf, &a.detail);
    put_str(buf, &a.related_key);
    put_str(buf, &a.recommended_action);
}

fn canonical_observation(buf: &mut Vec<u8>, o: &Observation) {
    put_opt(buf, o.subject.as_ref(), |b, s| {
        put_str(b, &s.subject_id);
    });
    put_i32(buf, o.r#type.to_i32());
    put_u32(buf, o.confidence_pct);
    put_str(buf, &o.summary);
    put_str(buf, &o.detail);
}

fn canonical_command(buf: &mut Vec<u8>, c: &Command) {
    match c.payload.as_ref() {
        None => put_u8(buf, 0),
        Some(CommandPayload::Request(r)) => {
            put_u8(buf, 1);
            canonical_command_request(buf, r);
        }
        Some(CommandPayload::Ack(a)) => {
            put_u8(buf, 2);
            canonical_ack(buf, a);
        }
        Some(CommandPayload::Result(r)) => {
            put_u8(buf, 3);
            canonical_op_result(buf, r);
        }
    }
}

fn canonical_command_request(buf: &mut Vec<u8>, r: &CommandRequest) {
    put_str(buf, &r.request_id);
    put_str(buf, &r.command);
    put_str(buf, &r.target);
    put_map_sorted(buf, &r.params);
    put_bytes(buf, &r.blob);
    put_bool(buf, r.requires_confirmation);
}

fn canonical_config(buf: &mut Vec<u8>, c: &Config) {
    match c.payload.as_ref() {
        None => put_u8(buf, 0),
        Some(ConfigPayload::Query(q)) => {
            put_u8(buf, 1);
            put_u32(buf, q.keys.len() as u32);
            for k in &q.keys {
                put_str(buf, k);
            }
        }
        Some(ConfigPayload::Snapshot(s)) => {
            put_u8(buf, 2);
            put_u64(buf, s.config_version);
            put_map_sorted(buf, &s.params);
        }
        Some(ConfigPayload::Update(u)) => {
            put_u8(buf, 3);
            put_str(buf, &u.update_id);
            put_bool(buf, u.strict);
            put_map_sorted(buf, &u.params);
        }
        Some(ConfigPayload::Ack(a)) => {
            put_u8(buf, 4);
            canonical_ack(buf, a);
        }
        Some(ConfigPayload::Result(r)) => {
            put_u8(buf, 5);
            canonical_op_result(buf, r);
        }
    }
}

fn canonical_engineering(buf: &mut Vec<u8>, e: &Engineering) {
    match e.payload.as_ref() {
        None => put_u8(buf, 0),
        Some(EngineeringPayload::Request(r)) => {
            put_u8(buf, 1);
            canonical_engineering_request(buf, r);
        }
        Some(EngineeringPayload::Ack(a)) => {
            put_u8(buf, 2);
            canonical_ack(buf, a);
        }
        Some(EngineeringPayload::Result(r)) => {
            put_u8(buf, 3);
            canonical_engineering_result(buf, r);
        }
    }
}

fn canonical_engineering_request(buf: &mut Vec<u8>, r: &EngineeringRequest) {
    put_str(buf, &r.request_id);
    put_str(buf, &r.action);
    put_str(buf, &r.target);
    put_map_sorted(buf, &r.params);
    put_bytes(buf, &r.blob);
    put_bool(buf, r.requires_confirmation);
}

fn canonical_engineering_result(buf: &mut Vec<u8>, r: &EngineeringResult) {
    put_i32(buf, r.status.to_i32());
    put_str(buf, &r.detail);
    put_map_sorted(buf, &r.outputs);
    put_bytes(buf, &r.blob);
}
