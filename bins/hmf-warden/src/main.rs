use std::net::TcpListener;

use anyhow::Result;
use ed25519_dalek::SigningKey;

use hmf_core::envelope::{DeliveryProfile, MsgClass, sign::verify_envelope_ed25519};
use hmf_transport::transport::tcp::read_record;

const DEVICE1_SK_BYTES: [u8; 32] = [7u8; 32];

fn main() -> Result<()> {
    let verifying_key = SigningKey::from_bytes(&DEVICE1_SK_BYTES).verifying_key();
    println!("warden expects pubkey = {:?}", verifying_key.to_bytes());

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    loop {
        println!("warden listening on 127.0.0.1:7878");
        let (mut stream, addr) = listener.accept()?;
        println!("hmf-warden: accepted connection from {addr}");

        loop {
            let maybe_env = read_record(&mut stream)?;
            let Some(env) = maybe_env else {
                println!("hmf-warden: connection closed (EOF)");
                break;
            };
            if !verify_envelope_ed25519(&env, &verifying_key) {
                println!("signature invalid — dropping envelope");
                continue;
            }

            let msg_class = MsgClass::try_from(env.msg_class).ok();
            let delivery = DeliveryProfile::try_from(env.delivery_profile).ok();

            println!("hmf-warden: received envelope:");
            println!("  proto_ver: {}", env.proto_ver);
            println!("  msg_class: {:?}", msg_class);
            println!("  sender_id: {}", env.sender_id);
            println!("  sender_instance: {}", env.sender_instance);
            println!("  counter: {}", env.counter);
            println!("  ttl_ms: {}", env.ttl_ms);
            println!("  transaction_id: {}", env.transaction_id);
            println!("  idempotency_key: {}", env.idempotency_key);
            println!("  delivery_profile: {:?}", delivery);
            println!("  scope: {}", env.scope);
            println!("  target: {}", env.target);
            println!("  topic: {}", env.topic);
        }
    }
}
