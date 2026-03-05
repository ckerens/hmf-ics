# Envelope specification (v1)

Status: Normative
Applies to: All compliant senders and receivers

This document defines the HMF-ICS v1 envelope.
If behavior is not explicitly defined here, it is undefined and MUST NOT be assumed.

## Normative language

The key words MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY are to be interpreted as described in RFC 2119.

## Data model

The envelope is represented in the wire format as protobuf (`proto3`), but security semantics are defined in
`hmf-core` domain types (see ADR 0001 and ADR 0002).

### Header fields

- proto_ver (uint32)
  - MUST equal 1.
- msg_class (enum MsgClass)
  - MUST match the payload variant.
  - MSG_CLASS_UNSPECIFIED MUST NOT be used.
- sender_id (string)
  - MUST be non-empty.
  - Included in signature binding.
- sender_instance (string)
  - MUST be non-empty.
  - Included in signature binding.
- counter (uint64)
  - MUST be non-zero.
  - Replay policy is defined in `replay-and-freshness.md`.
- ttl_ms (uint32)
  - MUST be non-zero.
  - Enforced by receiver using receiver-local time.
- transaction_id (string)
  - SHOULD be non-empty for state-changing messages.
  - Included in signature binding.
- idempotency_key (string)
  - Included in signature binding.
  - MAY be empty for non-idempotency-relevant messages, but empty weakens duplicate suppression.
- delivery_profile (enum DeliveryProfile)
  - Included in signature binding.
  - DELIVERY_PROFILE_UNSPECIFIED MUST NOT be used.

### Routing hints

These fields are security-relevant and MUST be included in signature binding:

- topic (string)
- target (string)
- scope (string)

### Payload

payload is a oneof:

- telemetry
- command
- audit
- config
- engineering

A valid envelope MUST contain a payload.

The payload is signed indirectly via a cryptographic hash over canonical payload bytes.

### Security fields

- sig_alg (enum SigAlg)
  - For v1, only ED25519 is permitted.
  - SIG_ALG_UNSPECIFIED MUST NOT be used.
- signature (bytes)
  - For v1 Ed25519, MUST be exactly 64 bytes.
- key_id (string)
  - MUST be non-empty.
  - Included in signature binding.
- auth_context (bytes)
  - Opaque authorization context evaluated by the receiver (for example, capabilities).
  - Included in signature binding as SHA-256(auth_context).
  - MAY be empty, but for state-changing operations, an empty auth_context will typically fail authorization.

## Message class consistency

msg_class MUST match the payload variant:

- TELEMETRY ↔ telemetry
- COMMAND ↔ command
- AUDIT ↔ audit
- CONFIG ↔ config
- ENGINEERING ↔ engineering

If msg_class does not match the payload variant, the receiver MUST reject the envelope.

## Signature model

For v1, envelopes are authenticated using Ed25519 signatures over a deterministic byte sequence called envelope
signing bytes.

The receiver MUST verify signature prior to any semantic processing.

### Domain separation

Signing bytes MUST begin with ASCII bytes of:

- HMFv1:envelope-signature

### Signing bytes composition

Signing bytes are constructed from:

1. Domain tag
2. Header fields
3. Routing hints
4. Security header subset (excluding signature)
5. auth_context_hash = SHA-256(auth_context)
6. payload_hash = SHA-256(canonical_payload_bytes(payload))

### Primitive encoding

All primitive values are encoded big-endian with explicit length prefixes for variable-length values:

- u32: 4 bytes big-endian
- u64: 8 bytes big-endian
- i32: 4 bytes big-endian
- bytes: u32 length prefix, then raw bytes
- string: bytes of UTF-8 encoded as bytes

### Canonical field order

Signing bytes MUST be formed in this order:

1. DOMAIN_TAG as bytes
2. proto_ver as u32
3. msg_class as i32
4. sender_id as string
5. sender_instance as string
6. counter as u64
7. ttl_ms as u32
8. transaction_id as string
9. idempotency_key as string
10. delivery_profile as i32
11. topic as string
12. target as string
13. scope as string
14. sig_alg as i32
15. key_id as string
16. auth_context_hash as 32 raw bytes
17. payload_hash as 32 raw bytes

signature MUST NOT be included in the signing bytes.

## Size limits

Implementations MUST enforce bounded sizes to mitigate resource exhaustion.
The reference implementation defines the default limits in code and exposes them as configuration, but compliant
implementations MUST, at minimum, enforce:

- Maximum envelope size (bytes)
- Maximum auth_context size (bytes)
- Maximum per-connection in-flight frames

## Delivery profile

Delivery profile expresses delivery expectations; it does not weaken validation.

A v1 implementation MUST define at least:

- ORDERED_RELIABLE: receiver processes in-order per sender stream when feasible.
- BEST_EFFORT: receiver may drop under load; validation remains fail-closed for accepted messages.

If a delivery profile cannot be satisfied, the receiver SHOULD reject with an explicit error code rather than
silently weakening semantics.
