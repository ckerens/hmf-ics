# Work Item Template

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

- Message type implemented in `hmf-core`
- Serialization support added to `hmf-wire-proto`
- Validation rules enforced
- Unit tests added
- Serialization round-trip tests added

---

## Out of Scope

Explicitly list what this work item does **not** include.

Example:

- Changes to canonical signing bytes
- New cryptographic algorithms
- Architectural refactors

---

## Implementation Notes

Optional notes for contributors or agents.

Example:

- Follow layering invariant between `hmf-core` and `hmf-wire-proto`
- Update documentation if semantics change
