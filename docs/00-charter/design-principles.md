# Design principles (v1)

This document is informative but boundary-setting.

## Security-first

Security is enforced at the protocol boundary, not retrofitted behind infrastructure.
Transport security is necessary for confidentiality and channel hardening, but it is not sufficient for trust.

## Explicit authority only

No authority may be implied from network position, topology, or infrastructure. Authority MUST be explicit,
cryptographically bound, verifiable by the receiver, scope-limited, and time-bounded.

## Receiver-enforced validation

Receivers enforce:

- Structural validity
- Freshness (TTL)
- Signature verification
- Replay resistance
- Authorization

No semantic execution may occur before these checks complete successfully.

## Partition-safe degradation

Loss of Warden availability MUST reduce privilege, not expand it.
Routine operations in the OT zone MUST remain functional given existing authority and local policy.

## Plane separation

HMF-ICS separates concerns:

- Data plane: direct operational messages (HMI ↔ PLC)
- Control plane: authority (enrollment, policy, revocation)
- Event and audit plane: durable audit and security events
- Read plane: queryable projections for UI and tools

Co-hosting components is an implementation detail.
The logical boundaries and APIs MUST remain distinct.

## Determinism and auditability

Protocol behavior MUST be deterministic.
Audit evidence MUST be attributable and verifiable end-to-end.
Receipts are authored by the component applying physical effect (typically the PLC).

## Minimal coupling

Transport adapters are byte carriers. They MUST NOT become the policy engine.
Authorization and replay enforcement live in the core runtime boundary.
