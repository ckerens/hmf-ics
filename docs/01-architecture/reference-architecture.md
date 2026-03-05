# Reference architecture (v1)

Status: Informative (guardrails)

## Mission and shape

HMF-ICS v1 assumes a single site with a sovereign local authority and supports optional export-only federation.

Default logical components:

- OT control zone:
  - HMI (endpoint)
  - PLC / device node (endpoint)
- Site services zone:
  - Warden (authority, registries, policy distribution)
  - Event log (durable audit and telemetry sink)
  - Read model (projection for UI and tools)

Optional components:

- Federation bridge and upstream ingest (observability only)
- DMZ segmentation between OT and IT

## Architectural planes

1. Data plane (direct P2P)
2. Control plane (authority and policy)
3. Event and audit plane (durable fanout)
4. Read plane (projection for UI)

## Data plane requirements

- HMI ↔ PLC direct communication MUST be supported.
- Command delivery MUST NOT require Warden availability.
- Both HMI and PLC MUST perform receiver validation as specified in `docs/02-protocol/receiver-validation.md`.

## Control plane requirements

- Trust mutations (enrollment, key rotation approval, capability issuance, revocation) flow through Warden.
- Loss of Warden availability MUST prevent trust mutation but MUST NOT prevent routine data plane operation.

## Event and audit plane requirements

- The PLC SHOULD emit signed receipts for state-changing commands.
- Audit and security events MUST NOT require Warden availability to generate.
- The event log provides durable acknowledgment semantics to reduce edge buffering requirements.

## Failure expectations

- Warden unavailable:
  - Enrollment and policy changes are blocked.
  - Existing authority remains usable until expiration (or local revocation).
  - HMI ↔ PLC operations continue in the OT zone.
- Read model unavailable:
  - Operator visibility degrades.
  - Command path remains intact (direct).
- Federation unavailable:
  - Local event capture continues.
  - Export resumes when the link returns.

## Scope guardrails (must not drift)

- Warden is not required in the data plane for command delivery.
- Warden is not a telemetry broker.
- Federation is export-only and MUST NOT introduce cross-site control dependency.
- Quorum and multi-warden authority are out of scope for v1.
