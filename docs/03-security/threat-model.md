# Threat model (v1)

Status: Informative (security planning)

This threat model is scoped to protocol and system boundaries, not physical safety engineering.

## Assumptions

- OT networks may be partially trusted but are not assumed benign.
- Partitions between OT and site services are expected.
- Attackers may obtain network access (including via compromised IT assets).
- Attackers may attempt replay, spoofing, and downgrade.
- Devices may be resource-constrained.

## Primary threats and mitigations

### Spoofed command injection

Mitigation:

- Receiver verifies Ed25519 signature bound to full envelope security context.
- Receiver enforces authorization from auth_context.
- Receivers are fail-closed.

### Replay and duplicate execution

Mitigation:

- Receiver-enforced TTL using receiver-local time.
- Monotonic counters scoped to sender_id and sender_instance.
- Idempotency window and duplicate suppression.

### Algorithm downgrade

Mitigation:

- Fixed crypto profile for v1.
- No runtime negotiation.

### Lateral movement and enumeration

Mitigation:

- Discovery is sensitive and capability-gated.
- Routing hints treated as security-relevant and signed.
- Default deployment segments OT from site services and IT.

### Denial of service at protocol boundary

Mitigation:

- Transport adapters enforce size caps, connection limits, and backpressure.
- Structural validation before signature verification.
- Audit emission best-effort, not on the critical path.

### Audit forgery and attribution gaps

Mitigation:

- Receipts are emitted by the actor applying physical effect (PLC).
- Audit messages are signed and validated like any other message.
- Event log is append-only and durable before ACK.

## Out of scope

- Safety engineering of actuator interlocks
- PLC vendor-specific runtime hardening
- Physical tampering detection
