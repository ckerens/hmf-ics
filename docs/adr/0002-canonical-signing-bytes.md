# ADR 0002: Canonical signing bytes and deterministic map handling

Status: Accepted
Date: 2026-02-20

## Context

Protobuf encoding permits arbitrary field order and map iteration order may be nondeterministic.

## Decision

Define canonical signing bytes in hmf-core and sign those bytes, not raw protobuf bytes.

- Domain separation tag first: HMFv1:envelope-signature
- Fixed field order and stable numeric encoding (big-endian)
- Map fields encoded by sorting entries deterministically
- auth_context included as SHA-256(auth_context)
- payload signed indirectly via payload_hash = SHA-256(canonical_payload_bytes(payload))

## Consequences

- Signatures verify across encoders and languages.
- Requires canonicalization implementation and test vectors.
