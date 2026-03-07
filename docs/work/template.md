# Work Item Template

Work Item ID: WI-XXXX
Status: Draft

This template defines the structure for implementation work items in HMF-ICS.

Work items describe **implementation tasks**, not architectural decisions.
Architectural changes require an ADR.

---

## Goal

Describe the outcome this work item must achieve.

Example:

- Add support for the AUDIT message class in the reference implementation.

---

## Context

Explain why this work item exists.

Include:

- relevant protocol sections
- related ADRs
- operational motivation

---

## Spec Anchors

Relevant normative requirements.

Examples:

- REQ-TTL-001
- REQ-SIG-004

---

## Invariant Anchors

Relevant system invariants.

Examples:

- INV-PIPE-001
- INV-LAYER-002

---

## Scope

Files/modules that may be modified.

Example:

- hmf-core/src/security/validation.rs
- tests/security/

---

## Normative Impact

Indicate whether the protocol specification changes.

Options:

- No normative change
- Normative documentation update required
- ADR required

---

## Acceptance Criteria

Define conditions that must be true for the work to be considered complete.

Example:

- Validation rejects ttl_ms == 0
- Unit tests added
- Round-trip tests added

---

## Out of Scope

Explicitly list what this work item does **not** include.

Example:

- Changes to canonical signing bytes
- New cryptographic algorithms
- Architectural refactors

---

## Escalation Triggers

Conditions that require halting work.

Examples:

- specification ambiguity
- invariant conflict
- acceptance criteria insufficient
- scope insufficient

---

## Implementation Notes

Optional notes for contributors or agents.

Example:

- Follow layering invariant between `hmf-core` and `hmf-wire-proto`
- Update documentation if semantics change
