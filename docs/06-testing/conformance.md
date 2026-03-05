# Conformance and testing (v1)

Status: Normative (for reference implementation), Informative (for others)

## Goal

Provide a clear contract for validating HMF-ICS v1 implementations.

## Required test categories

A compliant reference implementation MUST include tests for:

- Canonical signing bytes determinism (including randomized map insertion order)
- Signature round-trip (sign then verify)
- Structural validation failures (each required field)
- TTL enforcement using receiver-local time
- Counter monotonic enforcement per (sender_id, sender_instance)
- Sender instance reset behavior
- Idempotency duplicate suppression window behavior
- Authorization success and failure for state-changing messages
- Rejection behavior is fail-closed (no partial semantic execution)
- Audit receipt generation on accept and on rejection (where feasible)

## Recommended fixtures

- Golden test vectors for signing bytes for at least:
  - TELEMETRY
  - COMMAND
  - AUDIT (receipt)
- Replay sequences with expected accept and reject points.
