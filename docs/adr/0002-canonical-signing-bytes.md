# ADR 0002 — Canonical signing bytes and deterministic map handling

**Status:** Accepted  
**Date:** 2026-02-20

## Context

HMF uses protobuf (`prost`) as its current wire format. Protobuf encoding permits fields to appear in any order on the wire. In addition, protobuf `map<>` fields are typically represented as hash maps whose iteration order is not stable across runs, platforms, or implementations.

If HMF were to sign raw protobuf-serialized bytes, then:

- Logically identical messages could produce different byte streams.
- `map<>` fields could change order nondeterministically.
- Signatures could fail to verify even when the message is semantically unchanged.
- Security semantics would be coupled to a particular encoder implementation.

HMF is intended to be long-lived, security-first, and eventually amenable to multi-language implementations. Therefore, signature verification must be defined over protocol semantics, not over an incidental wire encoding.

## Decision

HMF will define a canonical “signing surface” in `hmf-core` and will sign those canonical bytes rather than signing raw protobuf bytes.

Specifically:

1. **Envelope signatures**
   - Signature input is `envelope_signing_bytes(envelope)` defined in `hmf-core`.
   - `signature` bytes are excluded from the signing input.

2. **Payload hashing**
   - The envelope signing bytes include a `payload_hash`.
   - `payload_hash = sha256(canonical_payload_bytes(payload))` where canonical payload bytes are defined in `hmf-core`.

3. **Deterministic maps**
   - All `map<>`-backed fields are treated as unordered.
   - Canonicalization sorts map entries deterministically (lexicographic by key bytes, then value bytes as a tiebreaker) before encoding.

4. **Security header included**
   - The canonical envelope signing bytes include:
     - `sig_alg`
     - `key_id`
     - `auth_context_hash = sha256(auth_context)`
   - `auth_context` is hashed for bounded signing input size and to avoid accidental coupling to a particular auth context encoding.

5. **Protobuf isolation**
   - Protobuf encode/decode is confined to `hmf-wire-proto`.
   - The canonicalization and signing rules are part of protocol semantics and remain in `hmf-core`.

## Consequences

### Positive
- Signatures are stable across:
  - different protobuf encoders,
  - different field ordering,
  - nondeterministic `map<>` iteration order,
  - future multi-language implementations.
- Security semantics are explicit, auditable, and testable.
- The protocol can evolve its wire format without redefining its security model.

### Negative
- Requires implementation and maintenance of canonicalization code.
- Requires conversion code between wire types and core types (see ADR 0001).

## Canonicalization rules (normative)

### Domain separation
The signing bytes MUST include a domain-separation tag (e.g., `HMFv1:envelope-signature`) as the first element to prevent cross-protocol signature reuse.

### Field encoding
Canonical bytes are built using:
- fixed field ordering,
- explicit discriminators for `oneof` / enum variants,
- length-prefixing for variable-length fields (strings, bytes),
- stable numeric encoding (big-endian).

### Map encoding
For each map field:
1. Convert entries to `(key_bytes, value_bytes)` pairs.
2. Sort by `key_bytes` ascending; if equal, sort by `value_bytes` ascending.
3. Encode:
   - entry count (u32),
   - then each entry as:
     - `len(key)` + `key` + `len(value)` + `value`.

## Implementation notes

- Canonical signing bytes live in `hmf-core/src/envelope/signing_bytes.rs`.
- `hmf-core` should include tests that:
  - verify canonical bytes are identical for logically identical inputs,
  - verify map ordering is deterministic,
  - verify signature round-trips (`sign` then `verify`) across multiple randomized map insertion orders.

## Related decisions
- ADR 0001 — Separate `hmf-core` semantics from protobuf wire types
