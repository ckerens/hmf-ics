# HMF Workspace Architecture

**Date:** 2026-02-20  
**Goal:** Enforce a clean separation between protocol semantics (core) and wire format (protobuf), while keeping transports and applications simple and testable.

## Crates and responsibilities

### `hmf-core` — Protocol semantics (no protobuf / no prost)
Owns the protocol *meaning* and security invariants:

- Domain types: `Envelope`, payload types, IDs, records
- Validation rules (cheap checks first)
- Canonical bytes for signing (map sorting, stable field order)
- Hashing utilities
- Signature algorithms (e.g., Ed25519) and verification helpers
- Security policy hooks (capabilities, `auth_context` interpretation — as it evolves)

**Hard rule:** `hmf-core` must not depend on `prost`, `protobuf`, or generated schema types.

### `hmf-wire-proto` — Protobuf wire implementation (prost lives here)
Owns the protocol *serialization format*:

- Generated protobuf types (prost output)
- Encode/decode bytes for the wire
- Conversion layer:
  - `TryFrom<proto::Envelope> for core::Envelope`
  - `From<core::Envelope> for proto::Envelope`
- Wire-compat details (unknown fields behavior, future schema migrations)

**Hard rule:** application crates should not depend on generated protobuf types directly — they should use `hmf-core` types.

### `hmf-transport` — Transports (TCP, later UDP/serial)
Owns framing and IO:

- TCP stream read/write, framing, backpressure
- Uses `hmf-wire-proto` for encode/decode
- Produces/consumes `hmf-core::Envelope`


### `hmf-device` / `hmf-warden` — Applications
Own behavior and orchestration:

- Business logic (heartbeat, enrollment, policy decisions)
- Use `hmf-core` domain types
- Use `hmf-transport` to send/receive
- Should not touch protobuf-generated types

## Dependency graph

```
hmf-core  (NO prost / NO protobuf)
   ^
   |
hmf-wire-proto (prost + conversions)
   ^
   |
hmf-transport (tcp framing; uses wire-proto)
   ^
   |
hmf-device / hmf-warden
```

## Practical guidance

### Where to put canonicalization for signatures
- `hmf-core/src/envelope/signing_bytes.rs`
- Canonicalization is a security semantic (“what is signed”), not a wire concern (“how bytes are encoded”).

### Where to put encode/decode
- `hmf-wire-proto/src/wire/protobuf.rs`

### Map fields (`map<>`) policy
- Canonical signing bytes must treat maps as unordered.
- Canonicalization must sort map entries before encoding into signing bytes.
- Never sign raw protobuf bytes for map-bearing messages unless using a proven deterministic mode.

### Tests
- `hmf-core`: stability tests for canonicalization (same logical value → same signing bytes), replay guards, and validation ordering.
- `hmf-wire-proto`: round-trip tests (core → proto → bytes → proto → core).
- `hmf-transport`: framing tests (partial reads, concatenated frames, oversized frames).

## Non-goals for now
- Multi-encoding support (CBOR, JSON) — the crate boundaries make it possible later.
- Key provisioning and PKI — separate ADR when enrollment is implemented.
