# Escalation Artifact Template
Created: 2026-03-07T22:54:45.067684+00:00

## Purpose

This artifact records situations where an agent must halt work and request human direction.
It ensures architectural, specification, or security ambiguities are captured in a consistent,
reviewable format.

Escalations occur when:

- a work item conflicts with `REQ-*` requirements
- a change may violate an `INV-*` invariant
- specification behavior is ambiguous or missing
- the implementation scope is insufficient to complete the task safely
- a security-sensitive decision requires human judgment

Escalations are **not failures**.  
They are the designed mechanism that prevents architectural drift.

---

# Escalation Record

## Escalation ID

Format:

ESC-YYYYMMDD-###

Example:

ESC-20260307-001

---

## Status

One of:

- Open
- Under Review
- Resolved
- Rejected

---

## Triggering Role

Which agent role triggered the escalation.

Examples:

- Implementer
- Planner
- Guardian
- Spec Sync
- Reviewer
- Verifier

---

## Related Work Item

Example:

WI-0007

---

## Slice (if applicable)

Example:

WI-0007-S2

---

## Specification Anchors

Relevant `REQ-*` identifiers.

Example:

- REQ-SIG-003
- REQ-SIG-004

---

## Invariant Anchors

Relevant `INV-*` identifiers.

Example:

- INV-PIPE-001
- INV-SIGN-001

---

## Affected Files or Modules

Example:

- hmf-core/src/security/validation.rs
- hmf-wire-proto/src/envelope.rs

---

## Problem Description

Clear explanation of why the agent cannot safely proceed.

Describe:

- what the agent attempted
- what constraint prevented progress
- what ambiguity or conflict was detected

---

## Why This Requires Human Judgment

Explain why the decision cannot be resolved automatically.

Examples:

- architectural decision required
- specification gap
- conflicting invariants
- security implications unclear

---

## Options Considered

List possible approaches the agent identified.

Example:

Option A — interpret spec behavior literally  
Option B — adjust validation ordering  
Option C — require ADR

---

## Recommended Resolution

The agent's recommended path forward.

This is advisory only.

---

## Human Decision

To be filled by a maintainer.

Decision:

Rationale:

---

## Resolution Actions

If resolved, describe required follow-up actions.

Examples:

- update protocol documentation
- add ADR
- modify work item
- adjust invariant catalog
- implement change

---

## Resolution Commit / PR

Example:

PR #42
