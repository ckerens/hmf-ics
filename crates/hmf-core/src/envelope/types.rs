use std::collections::BTreeMap;

use crate::ids::{DeviceId, IdempotencyKey, InstanceId, TransactionId};

pub const EXPECTED_PROTO_VER: u32 = 1;

// ---------- Envelope + header enums ----------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MsgClass {
    Unspecified,
    Telemetry,
    Command,
    Config,
    Engineering,
    Unknown(i32),
}
impl MsgClass {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::Telemetry,
            2 => Self::Command,
            3 => Self::Config,
            4 => Self::Engineering,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::Telemetry => 1,
            Self::Command => 2,
            Self::Config => 3,
            Self::Engineering => 4,
            Self::Unknown(x) => x,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeliveryProfile {
    Unspecified,
    BestEffort,
    AtLeastOnce,
    OrderedReliable,
    CriticalExec,
    Unknown(i32),
}
impl DeliveryProfile {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::BestEffort,
            2 => Self::AtLeastOnce,
            3 => Self::OrderedReliable,
            4 => Self::CriticalExec,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::BestEffort => 1,
            Self::AtLeastOnce => 2,
            Self::OrderedReliable => 3,
            Self::CriticalExec => 4,
            Self::Unknown(x) => x,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigAlg {
    Unspecified,
    Ed25519,
    Unknown(i32),
}
impl SigAlg {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::Ed25519,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::Ed25519 => 1,
            Self::Unknown(x) => x,
        }
    }
}

// ---------- Common enums ----------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AckStatus {
    Unspecified,
    Received,
    Rejected,
    Unknown(i32),
}
impl AckStatus {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::Received,
            2 => Self::Rejected,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::Received => 1,
            Self::Rejected => 2,
            Self::Unknown(x) => x,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResultStatus {
    Unspecified,
    Completed,
    Applied,
    Rejected,
    Failed,
    Unknown(i32),
}
impl ResultStatus {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::Completed,
            2 => Self::Applied,
            3 => Self::Rejected,
            4 => Self::Failed,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::Completed => 1,
            Self::Applied => 2,
            Self::Rejected => 3,
            Self::Failed => 4,
            Self::Unknown(x) => x,
        }
    }
}

// ---------- Telemetry enums ----------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeathReason {
    Unspecified,
    GracefulShutdown,
    LostNetwork,
    WatchdogReset,
    FatalFault,
    Unknown(i32),
}
impl DeathReason {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::GracefulShutdown,
            2 => Self::LostNetwork,
            3 => Self::WatchdogReset,
            4 => Self::FatalFault,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::GracefulShutdown => 1,
            Self::LostNetwork => 2,
            Self::WatchdogReset => 3,
            Self::FatalFault => 4,
            Self::Unknown(x) => x,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Health {
    Unspecified,
    Ok,
    Degraded,
    Faulted,
    Unknown(i32),
}
impl Health {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::Ok,
            2 => Self::Degraded,
            3 => Self::Faulted,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::Ok => 1,
            Self::Degraded => 2,
            Self::Faulted => 3,
            Self::Unknown(x) => x,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Quality {
    Unspecified,
    Good,
    Uncertain,
    Bad,
    Unknown(i32),
}
impl Quality {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::Good,
            2 => Self::Uncertain,
            3 => Self::Bad,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::Good => 1,
            Self::Uncertain => 2,
            Self::Bad => 3,
            Self::Unknown(x) => x,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlarmSeverity {
    Unspecified,
    Info,
    Warning,
    Critical,
    Unknown(i32),
}
impl AlarmSeverity {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::Info,
            2 => Self::Warning,
            3 => Self::Critical,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::Info => 1,
            Self::Warning => 2,
            Self::Critical => 3,
            Self::Unknown(x) => x,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObservationType {
    Unspecified,
    Threat,
    Anomaly,
    Compliance,
    Unknown(i32),
}
impl ObservationType {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::Unspecified,
            1 => Self::Threat,
            2 => Self::Anomaly,
            3 => Self::Compliance,
            x => Self::Unknown(x),
        }
    }
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::Unspecified => 0,
            Self::Threat => 1,
            Self::Anomaly => 2,
            Self::Compliance => 3,
            Self::Unknown(x) => x,
        }
    }
}

// ---------- Domain structs ----------

#[derive(Clone, Debug, PartialEq)]
pub struct Envelope {
    pub proto_ver: u32,
    pub msg_class: MsgClass,
    pub sender_id: DeviceId,
    pub sender_instance: InstanceId,
    pub counter: u64,
    pub ttl_ms: u32,
    pub transaction_id: TransactionId,
    pub idempotency_key: IdempotencyKey,
    pub delivery_profile: DeliveryProfile,

    pub topic: String,
    pub target: String,
    pub scope: String,

    pub payload: Option<Payload>,

    pub sig_alg: SigAlg,
    pub signature: Vec<u8>,
    pub key_id: String,
    pub auth_context: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Payload {
    Telemetry(Telemetry),
    Command(Command),
    Config(Config),
    Engineering(Engineering),
}

// ----- Telemetry -----

#[derive(Clone, Debug, PartialEq)]
pub struct Telemetry {
    pub payload: Option<TelemetryPayload>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TelemetryPayload {
    Birth(LifecycleBirth),
    Heartbeat(LifecycleHeartbeat),
    Death(LifecycleDeath),
    State(StateUpdate),
    Alarm(Alarm),
    Observation(Observation),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LifecycleBirth {
    pub vendor: String,
    pub model: String,
    pub serial: String,
    pub hw_rev: String,
    pub fw_rev: String,
    pub uptime_ms: u64,
    pub capabilities: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LifecycleHeartbeat {
    pub uptime_ms: u64,
    pub health: Health,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LifecycleDeath {
    pub reason: DeathReason,
    pub uptime_ms: u64,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StateUpdate {
    pub local_seq: u64,
    pub items: Vec<StateItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StateItem {
    pub key: String,
    pub value: Option<StateValue>,
    pub unit: String,
    pub quality: Quality,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StateValue {
    F64(f64),
    I64(i64),
    U64(u64),
    B(bool),
    S(String),
    Blob(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alarm {
    pub severity: AlarmSeverity,
    pub code: String,
    pub summary: String,
    pub detail: String,
    pub related_key: String,
    pub recommended_action: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Observation {
    pub subject: Option<Subject>,
    pub r#type: ObservationType,
    pub confidence_pct: u32,
    pub summary: String,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Subject {
    pub subject_id: String,
}

// ----- Command -----

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command {
    pub payload: Option<CommandPayload>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandPayload {
    Request(CommandRequest),
    Ack(Ack),
    Result(OpResult),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandRequest {
    pub request_id: String,
    pub command: String,
    pub target: String,
    pub params: BTreeMap<String, String>,
    pub blob: Vec<u8>,
    pub requires_confirmation: bool,
}

// ----- Config -----

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub payload: Option<ConfigPayload>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigPayload {
    Query(ConfigQuery),
    Snapshot(ConfigSnapshot),
    Update(ConfigUpdate),
    Ack(Ack),
    Result(OpResult),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigQuery {
    pub keys: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigSnapshot {
    pub config_version: u64,
    pub params: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigUpdate {
    pub update_id: String,
    pub strict: bool,
    pub params: BTreeMap<String, String>,
}

// ----- Engineering -----

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Engineering {
    pub payload: Option<EngineeringPayload>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineeringPayload {
    Request(EngineeringRequest),
    Ack(Ack),
    Result(EngineeringResult),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineeringRequest {
    pub request_id: String,
    pub action: String,
    pub target: String,
    pub params: BTreeMap<String, String>,
    pub blob: Vec<u8>,
    pub requires_confirmation: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineeringResult {
    pub status: ResultStatus,
    pub detail: String,
    pub outputs: BTreeMap<String, String>,
    pub blob: Vec<u8>,
}

// ----- Common -----

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ack {
    pub status: AckStatus,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpResult {
    pub status: ResultStatus,
    pub detail: String,
}
