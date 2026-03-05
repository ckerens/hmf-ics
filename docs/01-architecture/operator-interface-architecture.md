# Operator interface architecture (v1)

Status: Informative (boundary-setting)

## Summary

In HMF-ICS v1, the Operator Interface (HMI):

- IS a first-class HMF endpoint with its own signing key.
- Communicates directly with PLCs over the data plane for routine operations.
- Uses the read model for operator display and subscriptions when available.
- Uses Warden for control-plane workflows (enrollment approvals, policy, capabilities), not for command mediation.

The HMI is not a trust authority. It does not issue keys and does not bypass receiver validation.

## Data plane flows

- HMI → PLC:
  - Signed `COMMAND` envelopes.
- PLC → HMI:
  - Signed `TELEMETRY` envelopes (when applicable).
  - Signed `AUDIT` envelopes (receipts and outcomes).

The PLC is the authority for physical effect and MUST generate receipts for state-changing commands.

## Control plane flows

- HMI ↔ Warden:
  - Operator-driven enrollment approvals and administrative workflows.
  - Viewing trust state and policy state.
  - Requesting or updating capabilities for endpoints (subject to workflow).

## Read plane flows

- HMI ↔ Read model:
  - Queries and subscriptions for projected state.
  - Read access MUST be capability-gated.

## Degraded operation

- If Warden is unavailable:
  - HMI continues data plane operations with PLCs using existing authority.
  - New approvals and trust mutations are blocked.
- If read model is unavailable:
  - HMI enters degraded visibility mode.
  - Command path remains intact (direct).

## Security and UX constraints

- HMI MUST present clear operator feedback for:
  - Command acceptance vs rejection.
  - Authorization denial vs replay denial vs validation failure.
  - Degraded visibility and dependency outages.
