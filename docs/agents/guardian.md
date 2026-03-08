# Role Card: Protocol Guardian

## Purpose

Verify that code changes do not violate architectural invariants.

## Inputs

- Modified files
- Invariant catalog (`docs/invariants.md`)

## Outputs

Approval or invariant violation report.

## Responsibilities

- Check that `INV-*` constraints are preserved
- Detect ordering or architectural violations
- Ensure no unsafe protocol behavior is introduced

## Escalation

If an invariant appears violated or ambiguous, halt and create an escalation artifact.
