use std::time::{SystemTime, UNIX_EPOCH};
use std::{net::TcpStream, thread, time::Duration};

use anyhow::Result;
use ed25519_dalek::SigningKey;
use hmf_core::envelope::sign::sign_envelope_ed25519;

use hmf_core::envelope::*;
use hmf_core::ids::{DeviceId, IdempotencyKey, InstanceId, TransactionId};
use hmf_transport::transport::tcp::write_record;

// dev/test only
const DEVICE1_SK_BYTES: [u8; 32] = [7u8; 32];

fn main() -> Result<()> {
    let signing_key = SigningKey::from_bytes(&DEVICE1_SK_BYTES);
    let verifying_key = signing_key.verifying_key();
    println!("device pubkey = {:?}", verifying_key.to_bytes());

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut counter: u64 = 1;

    loop {
        let uptime_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let telemetry = Telemetry {
            payload: Some(TelemetryPayload::Heartbeat(LifecycleHeartbeat {
                uptime_ms,
                health: Health::Ok,
            })),
        };

        let mut env = Envelope {
            proto_ver: EXPECTED_PROTO_VER,
            msg_class: MsgClass::Telemetry,
            sender_id: DeviceId::new("device-1"),
            sender_instance: InstanceId::new("device-1:boot-1"),
            counter,
            ttl_ms: 5_000,
            transaction_id: TransactionId::new(format!("txn-{counter}")),
            idempotency_key: IdempotencyKey::new(format!("idem-{counter}")),
            delivery_profile: DeliveryProfile::BestEffort,

            topic: "zone:demo".to_string(),
            target: "site-warden".to_string(),
            scope: "hmf/telemetry/lifecycle_heartbeat".to_string(),

            payload: Some(Payload::Telemetry(telemetry)),

            sig_alg: SigAlg::Unspecified,
            signature: Vec::new(),
            key_id: String::new(),
            auth_context: Vec::new(),
        };

        sign_envelope_ed25519(&mut env, "device-1:ed25519:v1", &signing_key);
        write_record(&mut stream, &env)?;
        println!("sent heartbeat #{counter}");

        counter += 1;
        thread::sleep(Duration::from_secs(2));
    }
}
