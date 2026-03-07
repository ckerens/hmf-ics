# HMF-ICS Architectural Invariants

Status: Informative (boundary-setting)
Scope: All protocol-compliant implementations and the reference implementation

This document extracts the non-negotiable invariants of HMF-ICS v1 into a single reference.
It exists to prevent architectural drift during refactors, rapid iteration, and AI-assisted development.

If an invariant must change, it requires:

- an Architecture Decision Record (ADR)
- updates to all impacted normative documents
- updated conformance tests and (where applicable) golden test vectors

If any security invariant is violated, the system is considered insecure even if functionality appears correct.

---

## Protocol authority invariants

### INV-AUTHORITY-001

The normative protocol specification under `docs/02-protocol/` is the authoritative source of required behavior.

### INV-AUTHORITY-002

The reference implementation MUST implement the normative specification.

### INV-AUTHORITY-003

If behavior is not explicitly defined in a normative document, it is undefined and MUST NOT be assumed.

### INV-AUTHORITY-004

Protocol-breaking changes require a protocol version increment (`proto_ver`).

---

## Cryptographic invariants

### INV-SIGN-001

Envelope signatures MUST use Ed25519 for v1.

### INV-SIGN-002

There is no runtime algorithm negotiation, downgrade, or fallback.

### INV-SIGN-003

Signatures MUST be computed using canonical signing bytes, not raw protobuf bytes.

Canonical signing bytes MUST:

- include domain separation tag `HMFv1:envelope-signature`
- use deterministic field ordering
- use stable numeric encoding (big-endian)
- encode map fields deterministically (sorted key order)
- include `auth_context` as `SHA-256(auth_context)`
- include payload via `payload_hash = SHA-256(canonical_payload_bytes(payload))`

### INV-SIGN-004

The `signature` field MUST NOT be included in signing bytes.

Changes to canonicalization require:

- protocol version increment
- ADR update
- new test vectors

---

## Envelope invariants

### INV-ENVELOPE-001

`proto_ver` MUST equal 1 for v1 messages.

### INV-ENVELOPE-002

`msg_class` MUST match the payload type.

Required security fields MUST be present and bounded:

- `ttl_ms` MUST be non-zero
- `counter` MUST be non-zero
- `key_id` MUST be non-empty
- `sig_alg` MUST be ED25519
- Ed25519 signatures MUST be exactly 64 bytes

### INV-ENVELOPE-003

Routing hints (`topic`, `target`, `scope`) are security-relevant and MUST be covered by the signature.

---

## Receiver validation invariants

### INV-PIPE-001

Receiver processing MUST follow strict validation ordering and fail closed.

No semantic execution may occur until validation succeeds.

Recommended phase ordering:

1. Structural validation
2. Freshness validation (TTL)
3. Signature verification
4. Replay protection
5. Idempotency check
6. Authorization
7. Semantic execution
8. Audit emission

Phase responsibilities:

- **Structural validation** ensures the envelope and payload are well-formed.
- **Freshness validation** prevents expired messages from being processed.
- **Signature verification** ensures message authenticity and integrity.
- **Replay protection** prevents duplicate transport-level message instances.
- **Idempotency check** prevents duplicate semantic execution of the same logical command or transaction.
- **Authorization** determines whether the sender is permitted to perform the requested operation.
- **Semantic execution** applies the requested operation.
- **Audit emission** records security and operational evidence of the action.

Side effects MUST NOT occur before validation completes successfully.

---

## Freshness and replay invariants

### INV-REPLAY-001

Freshness and uniqueness are enforced by the receiver.

### INV-REPLAY-002

TTL MUST be evaluated using receiver-local monotonic time.

### INV-REPLAY-003

Replay protection MUST use monotonic counters scoped to `(endpoint, sender_instance)`.

### INV-REPLAY-004

Counter regression MUST be rejected.

### INV-REPLAY-005

Replay state MUST NOT update for unsigned or invalid-signature messages.

### INV-REPLAY-006

Sender restart requires a new `sender_instance`.

---

## Authorization invariants

### INV-AUTH-001

Authority MUST be cryptographically bound and verifiable by the receiver.

### INV-AUTH-002

Authorization MUST be evaluated locally by the receiver.

### INV-AUTH-003

Network position MUST NOT grant implicit privilege.

### INV-AUTH-004

`auth_context` is evaluated by the receiver as authorization input.

### INV-AUTH-005

Authorization failures MUST prevent semantic execution.

---

## Key management invariants

### INV-KEY-001

Endpoints are not trusted by default.

### INV-KEY-002

Trust roots are managed by the **Warden** (site authority).

### INV-KEY-003

`key_id` MUST uniquely identify a specific public key.

### INV-KEY-004

Unknown keys MUST be rejected.

### INV-KEY-005

Revocation MUST immediately invalidate the key locally.

### INV-KEY-006

If trust registry state is lost, receivers MUST reject unknown keys and fail conservatively.

---

## Audit invariants

### INV-AUDIT-001

`AUDIT` is a first-class message class.

### INV-AUDIT-002

Receivers applying physical effect SHOULD emit an audit **receipt**.

### INV-AUDIT-003

Rejected state-changing commands SHOULD emit an audit security event where feasible.

### INV-AUDIT-004

Audit delivery is best-effort and partition tolerant.

### INV-AUDIT-005

Failure to emit audit events MUST NOT weaken validation behavior.

---

## Transport invariants

Transport provides confidentiality and integrity but does not establish authority.

Compliant implementations MUST support at least one transport:

- TLS 1.3 over TCP
- QUIC using TLS 1.3

### INV-TRANSPORT-001

Transport MUST NOT allow plaintext fallback.

### INV-TRANSPORT-002

Transport MUST NOT negotiate TLS versions older than 1.3.

Transport adapters SHOULD enforce:

- connection limits
- frame size caps
- rate limiting
- resource accounting

---

## Architectural plane invariants

HMF-ICS separates four logical planes:

- **Data plane** — direct operational communication between OT endpoints (for example HMI ↔ PLC)
- **Control plane** — enrollment, issuance, revocation, policy distribution (Warden authority)
- **Event and audit plane** — durable capture of telemetry, receipts, and security events
- **Read plane** — queryable projections derived from the event log

Logical separation MUST remain intact even if components are co-hosted.

Operational guardrails:

### INV-MODEL-001

Data plane MUST remain operational during Warden outages.

### INV-MODEL-002

Warden acts as authority and registry, not a telemetry broker.

### INV-MODEL-003

Federation is export-only and MUST NOT introduce cross-site control dependency.

### INV-MODEL-004

Consensus or quorum mechanisms are out of scope for v1.

---

## Implementation layering invariants

Protocol semantics and wire serialization are intentionally separated.

### INV-LAYER-001

`hmf-core` defines protocol semantics.

### INV-LAYER-002

`hmf-wire-proto` handles protobuf serialization.

### INV-LAYER-003

`hmf-core` MUST NOT depend on protobuf or prost-generated types.

### INV-LAYER-004

Applications MUST NOT depend directly on generated protobuf structures.

### INV-LAYER-005

Conversions between wire types and core types MUST occur only in the wire layer.

---

## Conformance invariants

A conformant implementation MUST test:

- canonical signing bytes determinism
- signature verification
- structural validation failures
- TTL enforcement
- monotonic counter enforcement
- sender instance restart behavior
- authorization success and failure
- fail-closed rejection behavior
- audit receipt generation where applicable

Golden signing test vectors SHOULD exist for:

- TELEMETRY
- COMMAND
- AUDIT

---

## Changing invariants

Changing any invariant requires:

1. An ADR documenting the change and rationale.
2. Updates to normative protocol documentation.
3. Updated conformance tests and test vectors.
4. A protocol version increment if the change is breaking.

Invariants define the protocol boundary. They are not implementation preferences.
