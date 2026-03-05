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

- The normative protocol specification under `docs/02-protocol/` is the authoritative source of required behavior.
- The reference implementation MUST implement the normative specification.
- If behavior is not explicitly defined in a normative document, it is undefined and MUST NOT be assumed.
- Protocol-breaking changes require a protocol version increment (`proto_ver`).

---

## Cryptographic invariants

- Envelope signatures MUST use Ed25519 for v1.
- There is no runtime algorithm negotiation, downgrade, or fallback.
- Signatures MUST be computed using canonical signing bytes, not raw protobuf bytes.
- Canonical signing bytes MUST:
  - include domain separation tag `HMFv1:envelope-signature`
  - use deterministic field ordering
  - use stable numeric encoding (big-endian)
  - encode map fields deterministically (sorted key order)
  - include `auth_context` as `SHA-256(auth_context)`
  - include payload via `payload_hash = SHA-256(canonical_payload_bytes(payload))`
- The `signature` field MUST NOT be included in signing bytes.

Changes to canonicalization require:

- protocol version increment
- ADR update
- new test vectors

---

## Envelope invariants

- `proto_ver` MUST equal 1 for v1 messages.
- `msg_class` MUST match the payload type.
- Required security fields MUST be present and bounded:
  - `ttl_ms` MUST be non-zero
  - `counter` MUST be non-zero
  - `key_id` MUST be non-empty
  - `sig_alg` MUST be ED25519
  - Ed25519 signatures MUST be exactly 64 bytes
- Routing hints (`topic`, `target`, `scope`) are security-relevant and MUST be covered by the signature.

---

## Receiver validation invariants

Receiver processing MUST follow strict validation ordering and fail closed.

No semantic execution may occur until validation succeeds.

Recommended phase ordering:

1. Structural validation
2. Freshness validation (TTL)
3. Signature verification
4. Replay protection
5. Authorization
6. Semantic execution
7. Audit emission

Side effects MUST NOT occur before validation completes successfully.

---

## Freshness and replay invariants

- Freshness and uniqueness are enforced by the receiver.
- TTL MUST be evaluated using receiver-local monotonic time.
- Replay protection MUST use monotonic counters scoped to `(endpoint, sender_instance)`.
- Counter regression MUST be rejected.
- Replay state MUST NOT update for unsigned or invalid-signature messages.
- Sender restart requires a new `sender_instance`.

---

## Authorization invariants

- Authority MUST be cryptographically bound and verifiable by the receiver.
- Authorization MUST be evaluated locally by the receiver.
- Network position MUST NOT grant implicit privilege.
- `auth_context` is evaluated by the receiver as authorization input.
- Authorization failures MUST prevent semantic execution.

---

## Key management invariants

- Endpoints are not trusted by default.
- Trust roots are managed by the **Warden** (site authority).
- `key_id` MUST uniquely identify a specific public key.
- Unknown keys MUST be rejected.
- Revocation MUST immediately invalidate the key locally.
- If trust registry state is lost, receivers MUST reject unknown keys and fail conservatively.

---

## Audit invariants

The audit plane provides durable security and operational evidence.

- `AUDIT` is a first-class message class.
- Receivers applying physical effect SHOULD emit an audit **receipt**.
- Rejected state-changing commands SHOULD emit an audit security event where feasible.
- Audit delivery is best-effort and partition tolerant.
- Failure to emit audit events MUST NOT weaken validation behavior.

---

## Transport invariants

Transport provides confidentiality and integrity but does not establish authority.

Compliant implementations MUST support at least one transport:

- TLS 1.3 over TCP
- QUIC using TLS 1.3

Transport MUST NOT:

- allow plaintext fallback
- negotiate TLS versions older than 1.3

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

- Data plane MUST remain operational during Warden outages.
- Warden acts as authority and registry, not a telemetry broker.
- Federation is export-only and MUST NOT introduce cross-site control dependency.
- Consensus or quorum mechanisms are out of scope for v1.

---

## Implementation layering invariants

Protocol semantics and wire serialization are intentionally separated.

- `hmf-core` defines protocol semantics.
- `hmf-wire-proto` handles protobuf serialization.
- `hmf-core` MUST NOT depend on protobuf or prost-generated types.
- Applications MUST NOT depend directly on generated protobuf structures.

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
