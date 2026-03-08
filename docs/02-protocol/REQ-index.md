# HMF-ICS REQ Index

>Created: 2026-03-07T22:14:21.402693+00:00

## Purpose

This is an initial draft requirement index for HMF-ICS Strict / Spec Mode.

It is intended to:

- provide stable anchors for work items
- support Spec Sync traceability
- support PR and test references
- improve planning determinism

This draft is intentionally conservative:

- it only covers the highest-risk surfaces first
- it stays close to existing authoritative wording
- it should be reviewed against the normative protocol docs before being treated as authoritative

---

## TTL

### REQ-TTL-001

Receiver MUST reject envelopes with `ttl_ms == 0`.

### REQ-TTL-002

Receiver MUST evaluate TTL using receiver-local monotonic time.

---

## SIG

### REQ-SIG-001

Envelope signatures MUST use Ed25519 for v1.

### REQ-SIG-002

There MUST be no runtime algorithm negotiation, downgrade, or fallback for envelope signatures in v1.

### REQ-SIG-003

Signatures MUST be computed using canonical signing bytes, not raw protobuf bytes.

### REQ-SIG-004

Canonical signing bytes MUST include domain separation tag `HMFv1:envelope-signature`.

### REQ-SIG-005

Canonical signing bytes MUST use deterministic field ordering.

### REQ-SIG-006

Canonical signing bytes MUST use stable numeric encoding (big-endian).

### REQ-SIG-007

Canonical signing bytes MUST encode map fields deterministically using sorted key order.

### REQ-SIG-008

Canonical signing bytes MUST include `auth_context` as `SHA-256(auth_context)`.

### REQ-SIG-009

Canonical signing bytes MUST include payload via `payload_hash = SHA-256(canonical_payload_bytes(payload))`.

### REQ-SIG-010

The `signature` field MUST NOT be included in signing bytes.

### REQ-SIG-011

Changes to canonicalization MUST require a protocol version increment, ADR update, and new test vectors.

---

## ENVELOPE

### REQ-ENVELOPE-001

`proto_ver` MUST equal 1 for v1 messages.

### REQ-ENVELOPE-002

`msg_class` MUST match the payload type.

### REQ-ENVELOPE-003

`ttl_ms` MUST be non-zero.

### REQ-ENVELOPE-004

`counter` MUST be non-zero.

### REQ-ENVELOPE-005

`key_id` MUST be non-empty.

### REQ-ENVELOPE-006

`sig_alg` MUST be ED25519.

### REQ-ENVELOPE-007

Ed25519 signatures MUST be exactly 64 bytes.

### REQ-ENVELOPE-008

Routing hints (`topic`, `target`, `scope`) are security-relevant and MUST be covered by the signature.

---

## PIPE

### REQ-PIPE-001

Receiver processing MUST follow strict validation ordering and fail closed.

### REQ-PIPE-002

No semantic execution may occur until validation succeeds.

### REQ-PIPE-003

Side effects MUST NOT occur before validation completes successfully.

---

## REPLAY

### REQ-REPLAY-001

Freshness and uniqueness MUST be enforced by the receiver.

### REQ-REPLAY-002

Replay protection MUST use monotonic counters scoped to `(endpoint, sender_instance)`.

### REQ-REPLAY-003

Counter regression MUST be rejected.

### REQ-REPLAY-004

Replay state MUST NOT update for unsigned or invalid-signature messages.

### REQ-REPLAY-005

Sender restart MUST require a new `sender_instance`.

---

## AUTH

### REQ-AUTH-001

Authority MUST be cryptographically bound and verifiable by the receiver.

### REQ-AUTH-002

Authorization MUST be evaluated locally by the receiver.

### REQ-AUTH-003

Network position MUST NOT grant implicit privilege.

### REQ-AUTH-004

`auth_context` MUST be evaluated by the receiver as authorization input.

### REQ-AUTH-005

Authorization failures MUST prevent semantic execution.

---

## KEY

### REQ-KEY-001

Endpoints MUST NOT be trusted by default.

### REQ-KEY-002

Trust roots MUST be managed by the Warden.

### REQ-KEY-003

`key_id` MUST uniquely identify a specific public key.

### REQ-KEY-004

Unknown keys MUST be rejected.

### REQ-KEY-005

Revocation MUST immediately invalidate the key locally.

### REQ-KEY-006

If trust registry state is lost, receivers MUST reject unknown keys and fail conservatively.

---

## AUDIT

### REQ-AUDIT-001

`AUDIT` MUST be a first-class message class.

### REQ-AUDIT-002

Receivers applying physical effect SHOULD emit an audit receipt.

### REQ-AUDIT-003

Rejected state-changing commands SHOULD emit an audit security event where feasible.

### REQ-AUDIT-004

Audit delivery SHOULD be best-effort and partition tolerant.

### REQ-AUDIT-005

Failure to emit audit events MUST NOT weaken validation behavior.

---

## TRANSPORT

### REQ-TRANSPORT-001

Compliant implementations MUST support at least one transport:

- TLS 1.3 over TCP
- QUIC using TLS 1.3

### REQ-TRANSPORT-002

Transport MUST NOT allow plaintext fallback.

### REQ-TRANSPORT-003

Transport MUST NOT negotiate TLS versions older than 1.3.

### REQ-TRANSPORT-004

Transport adapters SHOULD enforce connection limits, frame size caps, rate limiting, and resource accounting.

---

## OPS

### REQ-OPS-001

Data plane communication between endpoints MUST be direct.

### REQ-OPS-002

The Warden MUST act as authority and registry, not a telemetry broker.

### REQ-OPS-003

Endpoints MUST remain operational during Warden outages.

### REQ-OPS-004

Federation, if implemented, MUST be export-only and MUST NOT introduce cross-site control dependency.

### REQ-OPS-005

Consensus or quorum mechanisms are out of scope for v1.

---

## LAYER

### REQ-LAYER-001

`hmf-core` MUST define protocol semantics.

### REQ-LAYER-002

`hmf-wire-proto` MUST handle protobuf serialization.

### REQ-LAYER-003

`hmf-core` MUST NOT depend on protobuf or prost-generated types.

### REQ-LAYER-004

Applications MUST NOT depend directly on generated protobuf structures.

### REQ-LAYER-005

Conversions between wire types and core types MUST occur only in the wire layer.

---

## TEST

### REQ-TEST-001

A conformant implementation MUST test canonical signing bytes determinism.

### REQ-TEST-002

A conformant implementation MUST test signature verification.

### REQ-TEST-003

A conformant implementation MUST test structural validation failures.

### REQ-TEST-004

A conformant implementation MUST test TTL enforcement.

### REQ-TEST-005

A conformant implementation MUST test monotonic counter enforcement.

### REQ-TEST-006

A conformant implementation MUST test sender instance restart behavior.

### REQ-TEST-007

A conformant implementation MUST test authorization success and failure.

### REQ-TEST-008

A conformant implementation MUST test fail-closed rejection behavior.

### REQ-TEST-009

A conformant implementation MUST test audit receipt generation where applicable.

### REQ-TEST-010

Golden signing test vectors SHOULD exist for TELEMETRY, COMMAND, and AUDIT.
