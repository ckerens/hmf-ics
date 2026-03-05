# Federation (export-only pattern A)

Status: Informative (guardrails)

HMF-ICS v1 supports export-only federation for multi-site observability.
Each site remains sovereign for trust and operations.

## Goals

- Capture audit and security events locally even when WAN is down.
- Provide local durable acknowledgment semantics so devices do not rely on large local buffers.
- Aggregate and batch events to reduce inter-site chatter.
- Preserve end-to-end verifiability (original signatures remain verifiable).
- Keep control-plane authority local.

## Non-goals (v1)

- No global trust root spanning sites.
- No device roaming or automatic trust portability across sites.
- No cross-site capability issuance or revocation.
- No consensus or quorum across sites.

## Components

Per site:

- Warden (local authority)
- Event log (local durable append-only sink)
- Read model (local projection)
- Federation bridge (tails event log and forwards upstream)

Upstream:

- Central event ingest
- SIEM and analytics consumers

## Integrity and verification

Upstream ingest SHOULD verify:

- Envelope signatures using site-provided trust metadata (for example, exported site key registry snapshots).
- Event identity for deduplication and replay suppression (event_id or transaction_id and idempotency_key).

## Operational constraint

Federation MUST NOT introduce any dependency that prevents local operation.
If WAN is unavailable, the site continues operating safely and records events locally.
