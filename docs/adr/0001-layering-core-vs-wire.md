# ADR 0001: Separate `hmf-core` semantics from protobuf wire types

Status: Accepted
Date: 2026-02-20

## Context

HMF uses protobuf today. Signing raw protobuf bytes is unsafe due to nondeterminism (field ordering and map order)
and couples security semantics to incidental encoding behavior.

## Decision

Enforce strict layering:

- hmf-core: protocol semantics, canonical signing bytes, validation, replay, authorization boundary.
  - MUST NOT depend on prost or generated protobuf.
- hmf-wire-proto: protobuf schema and encode/decode; conversion to hmf-core types.
- hmf-transport: IO, framing, connection handling; uses hmf-wire-proto for serialization and hmf-core as API.

Application crates use hmf-core types and do not import generated protobuf types directly.

## Consequences

- Security semantics are explicit and auditable.
- Requires conversion boilerplate, but prevents architectural drift.
