# Network zones and trust boundaries (v1)

Status: Informative

This document describes a pragmatic network zoning model and trust boundaries for HMF-ICS v1.
It is deployment guidance, not a compliance claim.

## Goals

- Keep control-plane authority (Warden) off the fast control loop.
- Allow direct HMI ↔ PLC communication inside OT boundaries.
- Minimize lateral movement and reconnaissance.
- Maintain auditability via signed receipts even under partition.
- Support a future DMZ or enterprise bridge without redesign.

## Practical zones

- Zone A: OT control zone (Levels 1 to 2)
  - PLCs, RTUs, device agents
  - HMIs and operator workstations
- Zone B: Site services zone (Level 3)
  - Warden (authority and registries)
  - Event log (audit and telemetry sink)
  - Read model (projection)
- Zone C: Enterprise or IT zone (Levels 4 to 5)
  - SIEM, analytics, ERP, MES
- Zone D: Industrial DMZ (optional, Level 3.5)
  - Export bridge, proxies, jump hosts

## Trust boundaries

- OT endpoints are not trusted by default. Devices must be enrolled and authorized.
- Warden is the site trust authority. Trust mutations flow through Warden.
- The HMI is not a trust authority. It does not issue keys and does not bypass validation.
- The data plane is independent of the control plane. HMI ↔ PLC does not require Warden availability.
- Audit integrity is anchored by signed receipts produced by the actor applying physical effects (typically the PLC).

## Allowed flows (default allow-list posture)

Unless explicitly allowed, deny by default.

- Zone A internal:
  - HMI ↔ PLC direct commands and limited telemetry.
- Zone A ↔ Zone B:
  - HMI → Warden for control-plane actions (admin and trust workflows).
  - PLC → Warden for enrollment, key rotation requests, and optional receipt mirroring.
  - PLC → Event log for telemetry, receipts, and security events (durable ACK).
  - HMI → Read model for queries and subscriptions.
- Zone B ↔ Zone D ↔ Zone C (optional export):
  - Event log and read model data flows are OT → IT, mediated by the DMZ.

## Partition expectations

- OT partition from site services:
  - HMI ↔ PLC continues functioning within Zone A.
  - Trust mutations are blocked until connectivity returns.
  - PLC buffers receipts and security events (bounded) until delivery is restored.
