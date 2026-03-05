# HMF-ICS v1 Implementation Guide

Status: Informative

This guide is intended for engineers or agents beginning a new implementation.

## 1. Recommended Crate / Module Structure

- hmf-core
  - Domain types (Envelope, Payload variants)
  - Canonical signing bytes
  - Replay logic
  - Authorization interface
  - Validation pipeline

- hmf-wire-proto
  - Protobuf schema
  - Encode/decode
  - Conversion to/from hmf-core types

- hmf-transport
  - TLS/QUIC listeners
  - Framing
  - Size caps
  - Connection management

- hmf-device (PLC/HMI runtime)
  - Endpoint context
  - Asset bindings
  - Command handlers
  - Audit emission

- hmf-warden
  - Key registry
  - Device registry
  - Enrollment workflows
  - Capability issuance
  - Policy distribution

## 2. Implementation Order (Suggested)

1. Implement canonical signing bytes and signature verification.
2. Add structural validation and TTL enforcement.
3. Implement replay logic (counter + idempotency).
4. Implement authorization interface (even if stubbed initially).
5. Implement COMMAND and TELEMETRY payloads.
6. Implement AUDIT payload and receipt generation.
7. Add transport layer.
8. Add Warden and enrollment workflows.

Do not start with UI or persistence before validation logic is correct.

## 3. Hard Boundaries

Never:

- Parse protobuf types directly in application logic.
- Execute command handlers before full validation.
- Allow transport-level identity to replace envelope signature.
- Couple replay logic to transport session state.

## 4. Audit First Mental Model

For state-changing commands:

- Validate.
- Execute safely.
- Emit receipt.

Audit emission must not be allowed to block safety.

## 5. Common Pitfalls

- Forgetting to hash auth_context in signing bytes.
- Signing raw protobuf bytes instead of canonical bytes.
- Updating replay state before signature verification.
- Allowing sender_instance reuse without reset.
- Letting Warden drift into the data plane.

## 6. Minimal Demonstration Profile

To demonstrate compliance:

- Single PLC endpoint.
- Single HMI endpoint.
- Direct TLS 1.3 connection.
- Local in-memory replay store.
- Simple file-backed event log.
- Warden running locally for enrollment only.

This profile is sufficient to prove protocol correctness without building a full industrial platform.
