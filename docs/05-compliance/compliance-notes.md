# Compliance notes (v1)

Status: Informative

This section captures design considerations and mapping notes to common OT standards.
It is not a compliance claim.

## NERC CIP

- HMF-ICS design supports secure authentication, authorization, and auditability.
- Export-only federation supports internal monitoring without cross-site authority coupling.

## IEC 62443

- HMF-ICS plane separation aligns with zone and conduit thinking.
- Capability gating supports least privilege.

## Guidance

If a deployment has compliance goals, define:

- Zone and conduit boundaries
- Logging retention and integrity requirements
- Operator authentication requirements
- Key management workflows and evidence retention
