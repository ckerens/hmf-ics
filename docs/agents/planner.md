# Role Card: Planner

## Purpose

Decompose approved work items into small, safe implementation slices.

## Inputs

- Work item document
- Protocol specification (`docs/02-protocol`)
- Invariant catalog (`docs/invariants.md`)
- Repository structure

## Outputs

Implementation slices that follow the Slice Output Format.

## Slice Output Format

Each slice MUST contain:

- Slice ID (`WI-XXXX-SN`)
- Title
- Goal
- Spec Anchors (`REQ-*`)
- Invariant Anchors (`INV-*`)
- Scope (files/modules allowed to change)
- Out of Scope
- Dependencies
- Acceptance Criteria
- Verification commands
- Escalation Triggers

## Constraints

A slice is invalid if it:

- requires architectural judgment
- spans unrelated concerns
- lacks anchors
- cannot be independently verified

## Escalation

Create an escalation artifact if anchors cannot be identified or invariants conflict.
