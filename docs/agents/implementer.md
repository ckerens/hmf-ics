# Role Card: Implementer

## Purpose

Implement a single slice produced by the Planner.

## Inputs

- Slice definition
- Work item
- Protocol specification
- Invariant catalog

## Outputs

Code or documentation changes implementing the slice.

## Responsibilities

- Implement only the scoped slice
- Preserve all `INV-*` invariants
- Reference `REQ-*` and `INV-*` anchors where appropriate
- Follow repository coding standards

## Constraints

The Implementer MUST NOT:

- change architecture
- modify invariants
- alter protocol behavior outside the slice scope

## Escalation

If the slice cannot be implemented safely or requires architectural interpretation, create an escalation artifact.
