# HMF-ICS v1 Documentation

This repository contains the authoritative specification for HMF-ICS v1.

HMF-ICS v1 is a security-first OT/ICS protocol and reference implementation scope that enables authenticated,
authorized, and auditable communication between OT endpoints (for example, an HMI and a PLC) while remaining
resilient to common ICS failure modes such as partitions and partial outages.

This documentation is not marketing material. It is an engineering specification.

## Document classes

Documents are classified as:

- **Normative**: Required behavior for compliant implementations.
- **Informative**: Explanatory or architectural guidance.

Normative documents use RFC-style language: MUST, MUST NOT, SHOULD, SHOULD NOT, MAY.

If behavior is not explicitly defined in a normative document, it is undefined and MUST NOT be assumed.

## Structure

- `docs/00-charter`: Vision, positioning, principles, glossary
- `docs/01-architecture`: System and component architecture (informative, but boundary-setting)
- `docs/02-protocol`: Normative protocol specification
- `docs/03-security`: Security properties and threat model
- `docs/04-operations`: Deployment, observability, failure modes
- `docs/05-compliance`: Standards notes (informative)
- `docs/06-testing`: Conformance guidance and test requirements
- `docs/adr`: Architecture decisions

## v1 scope guardrails

- Data plane is direct (HMI ↔ PLC) and MUST remain operational during Warden unavailability.
- Warden is an authority and registry, not a telemetry broker.
- Federation (if enabled) is export-only and MUST NOT introduce cross-site control dependency.
- Quorum / consensus is out of scope for v1.

## Markdown

Docs are formatted to be compatible with `markdownlint`.
