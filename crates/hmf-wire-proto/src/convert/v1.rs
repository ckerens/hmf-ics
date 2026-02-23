use std::collections::{BTreeMap, HashMap};

use crate::error::WireError;
use crate::proto::v1 as proto;
use hmf_core::envelope as core;

fn map_to_btree(m: HashMap<String, String>) -> BTreeMap<String, String> {
    m.into_iter().collect()
}

fn btree_to_hash(m: &BTreeMap<String, String>) -> HashMap<String, String> {
    m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

pub fn envelope_proto_to_core(p: proto::Envelope) -> Result<core::Envelope, WireError> {
    Ok(core::Envelope {
        proto_ver: p.proto_ver,
        msg_class: core::MsgClass::from_i32(p.msg_class),
        sender_id: hmf_core::ids::DeviceId::new(p.sender_id),
        sender_instance: hmf_core::ids::InstanceId::new(p.sender_instance),
        counter: p.counter,
        ttl_ms: p.ttl_ms,
        transaction_id: hmf_core::ids::TransactionId::new(p.transaction_id),
        idempotency_key: hmf_core::ids::IdempotencyKey::new(p.idempotency_key),
        delivery_profile: core::DeliveryProfile::from_i32(p.delivery_profile),

        topic: p.topic,
        target: p.target,
        scope: p.scope,

        payload: p.payload.map(payload_proto_to_core).transpose()?,

        sig_alg: core::SigAlg::from_i32(p.sig_alg),
        signature: p.signature,
        key_id: p.key_id,
        auth_context: p.auth_context,
    })
}

pub fn envelope_core_to_proto(c: &core::Envelope) -> proto::Envelope {
    proto::Envelope {
        proto_ver: c.proto_ver,
        msg_class: c.msg_class.to_i32(),
        sender_id: c.sender_id.as_str().to_string(),
        sender_instance: c.sender_instance.as_str().to_string(),
        counter: c.counter,
        ttl_ms: c.ttl_ms,
        transaction_id: c.transaction_id.as_str().to_string(),
        idempotency_key: c.idempotency_key.as_str().to_string(),
        delivery_profile: c.delivery_profile.to_i32(),

        topic: c.topic.clone(),
        target: c.target.clone(),
        scope: c.scope.clone(),

        payload: c.payload.as_ref().map(payload_core_to_proto),

        sig_alg: c.sig_alg.to_i32(),
        signature: c.signature.clone(),
        key_id: c.key_id.clone(),
        auth_context: c.auth_context.clone(),
    }
}

fn payload_proto_to_core(p: proto::envelope::Payload) -> Result<core::Payload, WireError> {
    Ok(match p {
        proto::envelope::Payload::Telemetry(t) => {
            core::Payload::Telemetry(telemetry_proto_to_core(t)?)
        }
        proto::envelope::Payload::Command(c) => core::Payload::Command(command_proto_to_core(c)?),
        proto::envelope::Payload::Config(c) => core::Payload::Config(config_proto_to_core(c)?),
        proto::envelope::Payload::Engineering(e) => {
            core::Payload::Engineering(engineering_proto_to_core(e)?)
        }
    })
}

fn payload_core_to_proto(c: &core::Payload) -> proto::envelope::Payload {
    match c {
        core::Payload::Telemetry(t) => {
            proto::envelope::Payload::Telemetry(telemetry_core_to_proto(t))
        }
        core::Payload::Command(c) => proto::envelope::Payload::Command(command_core_to_proto(c)),
        core::Payload::Config(c) => proto::envelope::Payload::Config(config_core_to_proto(c)),
        core::Payload::Engineering(e) => {
            proto::envelope::Payload::Engineering(engineering_core_to_proto(e))
        }
    }
}

// ---------------- Telemetry ----------------

fn telemetry_proto_to_core(p: proto::Telemetry) -> Result<core::Telemetry, WireError> {
    Ok(core::Telemetry {
        payload: p.payload.map(telemetry_payload_proto_to_core).transpose()?,
    })
}

fn telemetry_core_to_proto(c: &core::Telemetry) -> proto::Telemetry {
    proto::Telemetry {
        payload: c.payload.as_ref().map(telemetry_payload_core_to_proto),
    }
}

fn telemetry_payload_proto_to_core(
    p: proto::telemetry::Payload,
) -> Result<core::TelemetryPayload, WireError> {
    Ok(match p {
        proto::telemetry::Payload::Birth(b) => {
            core::TelemetryPayload::Birth(core::LifecycleBirth {
                vendor: b.vendor,
                model: b.model,
                serial: b.serial,
                hw_rev: b.hw_rev,
                fw_rev: b.fw_rev,
                uptime_ms: b.uptime_ms,
                capabilities: b.capabilities,
            })
        }
        proto::telemetry::Payload::Heartbeat(h) => {
            core::TelemetryPayload::Heartbeat(core::LifecycleHeartbeat {
                uptime_ms: h.uptime_ms,
                health: core::Health::from_i32(h.health),
            })
        }
        proto::telemetry::Payload::Death(d) => {
            core::TelemetryPayload::Death(core::LifecycleDeath {
                reason: core::DeathReason::from_i32(d.reason),
                uptime_ms: d.uptime_ms,
                detail: d.detail,
            })
        }
        proto::telemetry::Payload::State(s) => core::TelemetryPayload::State(core::StateUpdate {
            local_seq: s.local_seq,
            items: s
                .items
                .into_iter()
                .map(state_item_proto_to_core)
                .collect::<Result<_, _>>()?,
        }),
        proto::telemetry::Payload::Alarm(a) => core::TelemetryPayload::Alarm(core::Alarm {
            severity: core::AlarmSeverity::from_i32(a.severity),
            code: a.code,
            summary: a.summary,
            detail: a.detail,
            related_key: a.related_key,
            recommended_action: a.recommended_action,
        }),
        proto::telemetry::Payload::Observation(o) => {
            core::TelemetryPayload::Observation(core::Observation {
                subject: o.subject.map(|s| core::Subject {
                    subject_id: s.subject_id,
                }),
                r#type: core::ObservationType::from_i32(o.r#type),
                confidence_pct: o.confidence_pct,
                summary: o.summary,
                detail: o.detail,
            })
        }
    })
}

fn telemetry_payload_core_to_proto(c: &core::TelemetryPayload) -> proto::telemetry::Payload {
    match c {
        core::TelemetryPayload::Birth(b) => {
            proto::telemetry::Payload::Birth(proto::LifecycleBirth {
                vendor: b.vendor.clone(),
                model: b.model.clone(),
                serial: b.serial.clone(),
                hw_rev: b.hw_rev.clone(),
                fw_rev: b.fw_rev.clone(),
                uptime_ms: b.uptime_ms,
                capabilities: b.capabilities.clone(),
            })
        }
        core::TelemetryPayload::Heartbeat(h) => {
            proto::telemetry::Payload::Heartbeat(proto::LifecycleHeartbeat {
                uptime_ms: h.uptime_ms,
                health: h.health.to_i32(),
            })
        }
        core::TelemetryPayload::Death(d) => {
            proto::telemetry::Payload::Death(proto::LifecycleDeath {
                reason: d.reason.to_i32(),
                uptime_ms: d.uptime_ms,
                detail: d.detail.clone(),
            })
        }
        core::TelemetryPayload::State(s) => proto::telemetry::Payload::State(proto::StateUpdate {
            local_seq: s.local_seq,
            items: s.items.iter().map(state_item_core_to_proto).collect(),
        }),
        core::TelemetryPayload::Alarm(a) => proto::telemetry::Payload::Alarm(proto::Alarm {
            severity: a.severity.to_i32(),
            code: a.code.clone(),
            summary: a.summary.clone(),
            detail: a.detail.clone(),
            related_key: a.related_key.clone(),
            recommended_action: a.recommended_action.clone(),
        }),
        core::TelemetryPayload::Observation(o) => {
            proto::telemetry::Payload::Observation(proto::Observation {
                subject: o.subject.as_ref().map(|s| proto::Subject {
                    subject_id: s.subject_id.clone(),
                }),
                r#type: o.r#type.to_i32(),
                confidence_pct: o.confidence_pct,
                summary: o.summary.clone(),
                detail: o.detail.clone(),
            })
        }
    }
}

fn state_item_proto_to_core(p: proto::StateItem) -> Result<core::StateItem, WireError> {
    Ok(core::StateItem {
        key: p.key,
        value: p.value.map(state_value_proto_to_core).transpose()?,
        unit: p.unit,
        quality: core::Quality::from_i32(p.quality),
    })
}

fn state_item_core_to_proto(c: &core::StateItem) -> proto::StateItem {
    proto::StateItem {
        key: c.key.clone(),
        value: c.value.as_ref().map(state_value_core_to_proto),
        unit: c.unit.clone(),
        quality: c.quality.to_i32(),
    }
}

fn state_value_proto_to_core(p: proto::state_item::Value) -> Result<core::StateValue, WireError> {
    Ok(match p {
        proto::state_item::Value::F64(v) => core::StateValue::F64(v),
        proto::state_item::Value::I64(v) => core::StateValue::I64(v),
        proto::state_item::Value::U64(v) => core::StateValue::U64(v),
        proto::state_item::Value::B(v) => core::StateValue::B(v),
        proto::state_item::Value::S(v) => core::StateValue::S(v),
        proto::state_item::Value::Blob(v) => core::StateValue::Blob(v),
    })
}

fn state_value_core_to_proto(c: &core::StateValue) -> proto::state_item::Value {
    match c {
        core::StateValue::F64(v) => proto::state_item::Value::F64(*v),
        core::StateValue::I64(v) => proto::state_item::Value::I64(*v),
        core::StateValue::U64(v) => proto::state_item::Value::U64(*v),
        core::StateValue::B(v) => proto::state_item::Value::B(*v),
        core::StateValue::S(v) => proto::state_item::Value::S(v.clone()),
        core::StateValue::Blob(v) => proto::state_item::Value::Blob(v.clone()),
    }
}

// ---------------- Command ----------------

fn command_proto_to_core(p: proto::Command) -> Result<core::Command, WireError> {
    Ok(core::Command {
        payload: p.payload.map(command_payload_proto_to_core).transpose()?,
    })
}

fn command_core_to_proto(c: &core::Command) -> proto::Command {
    proto::Command {
        payload: c.payload.as_ref().map(command_payload_core_to_proto),
    }
}

fn command_payload_proto_to_core(
    p: proto::command::Payload,
) -> Result<core::CommandPayload, WireError> {
    Ok(match p {
        proto::command::Payload::Request(r) => {
            core::CommandPayload::Request(core::CommandRequest {
                request_id: r.request_id,
                command: r.command,
                target: r.target,
                params: map_to_btree(r.params),
                blob: r.blob,
                requires_confirmation: r.requires_confirmation,
            })
        }
        proto::command::Payload::Ack(a) => core::CommandPayload::Ack(core::Ack {
            status: core::AckStatus::from_i32(a.status),
            detail: a.detail,
        }),
        proto::command::Payload::Result(r) => core::CommandPayload::Result(core::OpResult {
            status: core::ResultStatus::from_i32(r.status),
            detail: r.detail,
        }),
    })
}

fn command_payload_core_to_proto(c: &core::CommandPayload) -> proto::command::Payload {
    match c {
        core::CommandPayload::Request(r) => {
            proto::command::Payload::Request(proto::CommandRequest {
                request_id: r.request_id.clone(),
                command: r.command.clone(),
                target: r.target.clone(),
                params: btree_to_hash(&r.params),
                blob: r.blob.clone(),
                requires_confirmation: r.requires_confirmation,
            })
        }
        core::CommandPayload::Ack(a) => proto::command::Payload::Ack(proto::Ack {
            status: a.status.to_i32(),
            detail: a.detail.clone(),
        }),
        core::CommandPayload::Result(r) => proto::command::Payload::Result(proto::Result {
            status: r.status.to_i32(),
            detail: r.detail.clone(),
        }),
    }
}

// ---------------- Config ----------------

fn config_proto_to_core(p: proto::Config) -> Result<core::Config, WireError> {
    Ok(core::Config {
        payload: p.payload.map(config_payload_proto_to_core).transpose()?,
    })
}

fn config_core_to_proto(c: &core::Config) -> proto::Config {
    proto::Config {
        payload: c.payload.as_ref().map(config_payload_core_to_proto),
    }
}

fn config_payload_proto_to_core(
    p: proto::config::Payload,
) -> Result<core::ConfigPayload, WireError> {
    Ok(match p {
        proto::config::Payload::Query(q) => {
            core::ConfigPayload::Query(core::ConfigQuery { keys: q.keys })
        }
        proto::config::Payload::Snapshot(s) => {
            core::ConfigPayload::Snapshot(core::ConfigSnapshot {
                config_version: s.config_version,
                params: map_to_btree(s.params),
            })
        }
        proto::config::Payload::Update(u) => core::ConfigPayload::Update(core::ConfigUpdate {
            update_id: u.update_id,
            strict: u.strict,
            params: map_to_btree(u.params),
        }),
        proto::config::Payload::Ack(a) => core::ConfigPayload::Ack(core::Ack {
            status: core::AckStatus::from_i32(a.status),
            detail: a.detail,
        }),
        proto::config::Payload::Result(r) => core::ConfigPayload::Result(core::OpResult {
            status: core::ResultStatus::from_i32(r.status),
            detail: r.detail,
        }),
    })
}

fn config_payload_core_to_proto(c: &core::ConfigPayload) -> proto::config::Payload {
    match c {
        core::ConfigPayload::Query(q) => proto::config::Payload::Query(proto::ConfigQuery {
            keys: q.keys.clone(),
        }),
        core::ConfigPayload::Snapshot(s) => {
            proto::config::Payload::Snapshot(proto::ConfigSnapshot {
                config_version: s.config_version,
                params: btree_to_hash(&s.params),
            })
        }
        core::ConfigPayload::Update(u) => proto::config::Payload::Update(proto::ConfigUpdate {
            update_id: u.update_id.clone(),
            strict: u.strict,
            params: btree_to_hash(&u.params),
        }),
        core::ConfigPayload::Ack(a) => proto::config::Payload::Ack(proto::Ack {
            status: a.status.to_i32(),
            detail: a.detail.clone(),
        }),
        core::ConfigPayload::Result(r) => proto::config::Payload::Result(proto::Result {
            status: r.status.to_i32(),
            detail: r.detail.clone(),
        }),
    }
}

// ---------------- Engineering ----------------

fn engineering_proto_to_core(p: proto::Engineering) -> Result<core::Engineering, WireError> {
    Ok(core::Engineering {
        payload: p
            .payload
            .map(engineering_payload_proto_to_core)
            .transpose()?,
    })
}

fn engineering_core_to_proto(c: &core::Engineering) -> proto::Engineering {
    proto::Engineering {
        payload: c.payload.as_ref().map(engineering_payload_core_to_proto),
    }
}

fn engineering_payload_proto_to_core(
    p: proto::engineering::Payload,
) -> Result<core::EngineeringPayload, WireError> {
    Ok(match p {
        proto::engineering::Payload::Request(r) => {
            core::EngineeringPayload::Request(core::EngineeringRequest {
                request_id: r.request_id,
                action: r.action,
                target: r.target,
                params: map_to_btree(r.params),
                blob: r.blob,
                requires_confirmation: r.requires_confirmation,
            })
        }
        proto::engineering::Payload::Ack(a) => core::EngineeringPayload::Ack(core::Ack {
            status: core::AckStatus::from_i32(a.status),
            detail: a.detail,
        }),
        proto::engineering::Payload::Result(r) => {
            core::EngineeringPayload::Result(core::EngineeringResult {
                status: core::ResultStatus::from_i32(r.status),
                detail: r.detail,
                outputs: map_to_btree(r.outputs),
                blob: r.blob,
            })
        }
    })
}

fn engineering_payload_core_to_proto(c: &core::EngineeringPayload) -> proto::engineering::Payload {
    match c {
        core::EngineeringPayload::Request(r) => {
            proto::engineering::Payload::Request(proto::EngineeringRequest {
                request_id: r.request_id.clone(),
                action: r.action.clone(),
                target: r.target.clone(),
                params: btree_to_hash(&r.params),
                blob: r.blob.clone(),
                requires_confirmation: r.requires_confirmation,
            })
        }
        core::EngineeringPayload::Ack(a) => proto::engineering::Payload::Ack(proto::Ack {
            status: a.status.to_i32(),
            detail: a.detail.clone(),
        }),
        core::EngineeringPayload::Result(r) => {
            proto::engineering::Payload::Result(proto::EngineeringResult {
                status: r.status.to_i32(),
                detail: r.detail.clone(),
                outputs: btree_to_hash(&r.outputs),
                blob: r.blob.clone(),
            })
        }
    }
}
