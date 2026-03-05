# Hardened Management Fabric for Industrial Control Systems (HMF-ICS)

> **Status:** Security research / engineering project (not production-ready).

HMF-ICS is a security-first OT/ICS protocol and reference implementation for environments where operational reliability and security must coexist under real-world industrial constraints.

Industrial control systems operate under conditions that differ fundamentally from typical IT systems:

- Devices operate for decades
- Connectivity may be intermittent or segmented
- Control loops cannot depend on central services
- Safety and traceability requirements are strict
- Network trust assumptions are weak or incorrect

HMF-ICS explores what an OT protocol looks like when **security correctness, deterministic behavior, and partition tolerance are treated as first-order architectural requirements rather than operational add-ons.**

This repository defines:

- A protocol specification
- A reference architecture
- Security invariants and threat model
- A Rust reference implementation

The goal is not to build a SCADA platform or monitoring product, but to demonstrate a **secure control coordination fabric** suitable for industrial environments.

---

## What makes this interesting

- **Receiver-enforced validation** — endpoints independently verify freshness, identity, authorization, and replay protection before any state change occurs.
- **Control-loop independence from authority services** — routine operations continue even if enrollment or policy infrastructure is unavailable.
- **Cryptographically attributable audit events** — security-relevant actions produce signed evidence at the point of physical effect.

---

## Security invariants

HMF-ICS is built around receiver-enforced security invariants. If any invariant is violated, the system is considered insecure even if functionality appears correct.

- **Explicit authority only:** Authority is explicit, cryptographically bound, verifiable by the receiver, scope-limited, and time-bounded. Network position does not grant privilege.
- **Message-level authentication:** Every state-changing message is signed and verified before semantic processing. Transport security alone is insufficient to establish trust.
- **Receiver-enforced freshness:** Receivers reject messages that exceed TTL, regress counters, or duplicate idempotency identity within the enforcement window. Freshness does not depend on sender wall-clock time.
- **Deterministic validation order:** Security validation occurs before execution. No side effects occur prior to full validation.
- **No algorithm downgrade:** Implementations do not negotiate weaker algorithms or silently fallback. Cryptographic changes require a protocol version increment.
- **Discovery is sensitive:** Enumeration of topology and authority metadata requires explicit authorization.
- **Partition-safe degradation:** Loss of dependencies reduces privilege, not expands it. Instability does not relax authorization.
- **No implicit trust from infrastructure:** Compromise of VPNs, gateways, firewalls, proxies, or cloud ingress does not confer protocol authority — all protocol decisions remain independently verifiable.
- **Identity integrity:** Device identity is unique and resistant to cloning within the deployment threat model. Sender instance reuse without reset semantics is prohibited for state-changing operations.

---

## Architectural Model

HMF-ICS separates responsibilities into explicit planes:

| Plane | Responsibility |
| ------ | ---------------- |
| **Data plane** | Direct operational communication between endpoints (e.g., HMI ↔ PLC) |
| **Control plane** | Site-local authority for enrollment, policy distribution, and revocation |
| **Event & audit plane** | Durable, signed receipts and security events |
| **Read plane** | Operator-facing projections and analytics views |

The separation ensures that:

- routine operations continue during control-plane outages
- authority mutation does not sit in the control loop
- audit evidence is produced at the point of physical effect

---

## Core Security Properties

All state-changing messages follow a deterministic validation pipeline enforced by the receiver.

Receivers enforce, in strict order:

1. Structural validation
2. Freshness enforcement (TTL using receiver-local time)
3. Signature verification (Ed25519)
4. Replay protection via monotonic counters
5. Idempotency enforcement
6. Authorization
7. Execution
8. Audit emission

No side effects occur until validation succeeds.

This model prevents entire classes of failures common in broker-centric messaging systems.

---

## Envelope-Centric Security

Every message carries a signed envelope containing:

- **Identity** (`sender_id`, `sender_instance`)
- **Freshness** (`counter`, `ttl_ms`)
- **Routing context** (`scope`, `target`, `topic`)
- **Correlation** (`transaction_id`, `idempotency_key`)
- **Security metadata** (`key_id`, `signature`, `auth_context`)

Security-relevant routing fields are **signature-bound**, preventing tampering by intermediate infrastructure.

Transport security protects the channel.

The **envelope signature establishes authority.**

---

## Authority Model

HMF-ICS assumes **no implicit trust from infrastructure**.

Authority is:

- cryptographically bound
- explicit
- scope-limited
- time-bounded
- locally enforceable

Each endpoint possesses a unique Ed25519 signing key.

A site-local **Warden** manages:

- enrollment
- policy issuance
- revocation

Loss of Warden availability reduces privilege but **does not halt routine operations.**

---

## Repository Structure

```text
crates/
  hmf-core/         Protocol semantics, validation, replay, signing
  hmf-wire-proto/   Protobuf schema and encoding
  hmf-transport/    TLS / QUIC adapters

bins/
  hmf-device/       Reference PLC-style endpoint
  hmf-operator/     Reference HMI endpoint
  hmf-warden/       Reference authority

```

The binaries exercise protocol invariants and validation flows.
They are reference implementations, not production components.

---

## Project Status

Active development.

Current implementation includes:

- canonical signing model
- receiver validation pipeline
- replay protection primitives
- initial HMI ↔ PLC command path

Current focus:

- capability enforcement
- durable replay state
- expanded negative-path testing
- simulation environment for protocol behavior

---

## License

Apache License 2.0
