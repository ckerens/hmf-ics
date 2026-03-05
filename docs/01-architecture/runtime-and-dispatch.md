# Runtime and dispatch model (v1)

Status: Informative (implementation guidance)

## Terminology

- Node: a runtime instance with a transport listener.
- Endpoint: a cryptographic identity within a node.
- Asset: functional or physical element controlled by an endpoint.

A single PLC host is typically one node with one endpoint controlling multiple assets.
A gateway may host multiple endpoints.

## High-level model

An HMF-enabled node is structured as:

- Transport adapters
- Router
- Shared ingress pipeline (validation and enforcement)
- Endpoint contexts
- Application handlers (asset logic)

The ingress pipeline is the security boundary and MUST be shared across handlers to prevent bypass.

## Transport adapter responsibilities

Transport adapters:

- Accept connections and frames.
- Enforce size limits and basic rate limiting.
- Decode wire envelopes into core domain types.
- Emit parsed envelopes upward.

Transport adapters MUST NOT:

- Make trust decisions.
- Evaluate authorization policy.
- Update replay state.
- Execute application logic.

## Router responsibilities

The router selects the destination endpoint context based on security-relevant routing hints:

- scope
- target
- topic
- msg_class

If routing resolves to zero or multiple endpoints, the message MUST be rejected.

## Ingress pipeline sequencing

All inbound envelopes MUST pass through these phases in order:

1. Structural validation
2. Freshness validation (TTL)
3. Signature validation
4. Replay validation
5. Authorization validation
6. Semantic execution
7. Audit emission

This sequencing is normative and defined in `docs/02-protocol/receiver-validation.md`.

## Endpoint context responsibilities

Each endpoint context owns:

- Identity (sender_id, sender_instance)
- Outbound counter state
- Endpoint policy configuration
- Handler bindings (assets)

## Persistence boundaries

Replay and trust state persistence requirements are defined in:

- `docs/02-protocol/replay-and-freshness.md`
- `docs/02-protocol/key-management.md`
