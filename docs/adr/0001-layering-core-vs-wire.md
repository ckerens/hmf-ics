# ADR 0001 — Separate `hmf-core` semantics from protobuf wire types

**Status:** Accepted  
**Date:** 2026-02-20

## Context

HMF uses protobuf (`prost`) for a wire format today. As protocol security is implemented (hashing, canonical signing bytes, Ed25519 signatures), direct use of generated protobuf types inside the “core” crate creates risks:

- Security semantics become coupled to protobuf encoding behavior (especially `map<>`, which is nondeterministic with typical `HashMap` backing).
- Core logic becomes harder to audit and reason about.
- Future wire changes (schema evolution, unknown fields, versioning) bleed into domain logic.
- Adding another wire encoding later becomes expensive.

## Decision

We will enforce a strict layering with separate crates:

1. **`hmf-core`** defines protocol semantics and security rules.  
   - Contains domain types, validation, canonical signing bytes, hashing, and signature algorithms.  
   - Must not depend on `prost`, protobuf, or generated schema.

2. **`hmf-wire-proto`** owns protobuf details.  
   - Contains prost-generated types and encode/decode.  
   - Contains conversions between protobuf types and `hmf-core` domain types.

3. **`hmf-transport`** owns IO + framing.  
   - Uses `hmf-wire-proto` for serialization.  
   - Uses `hmf-core` types as its API surface.

Application crates (`hmf-device`, `hmf-warden`) will use `hmf-core` domain types and will not import generated protobuf types directly.

## Consequences

### Positive
- Canonical signing bytes can be defined purely in terms of protocol semantics.
- `map<>` nondeterminism is solved in one place (canonicalization in `hmf-core`).
- The crate graph enforces separation — preventing drift over time.
- Encoding changes and schema evolution are isolated to `hmf-wire-proto`.

### Negative
- Requires writing and maintaining conversion code between core and proto types.
- Slightly more boilerplate up front.
- Changes to the schema may require updating conversions.

## Alternatives considered

1. **Keep protobuf types in `hmf-core` and sign serialized bytes.**  
   Rejected: breaks with `map<>` nondeterminism and couples security to wire encoding details.

2. **Keep protobuf types in `hmf-core`, but re-export behind a facade.**  
   Rejected: reduces import churn but does not solve semantic coupling.

3. **Use deterministic protobuf serialization everywhere.**  
   Rejected for now: library support varies, and it still couples security semantics to a specific wire encoding approach.

## Implementation notes

- Add an `ARCHITECTURE.md` describing crate responsibilities and the dependency graph.
- Add round-trip tests in `hmf-wire-proto` to ensure conversions preserve meaning.
- Add stability tests in `hmf-core` for signing bytes (same logical message → same bytes).
