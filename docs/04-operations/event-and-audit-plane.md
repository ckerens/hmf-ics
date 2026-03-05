# Event and audit plane (v1)

Status: Informative (implementation guidance)

## Intent

HMF-ICS provides strong auditability without requiring Warden to be a broker or to sit in the control loop.

The event and audit plane is a consumer path, not a mediation path.

## Event categories

- Operational telemetry
- Command receipts (audit)
- Security events (audit)
- Lifecycle events (audit)

Receipts are the core audit primitive and are produced by the receiver applying physical effect.

## Minimal requirements

- Receipts MUST NOT require Warden availability to generate.
- Receipt delivery is best-effort under partition.
- Endpoints SHOULD buffer receipts and security events locally (bounded).
- Failure to emit an audit event MUST NOT weaken rejection behavior.

## Durable acknowledgment

The event log provides a local durable acknowledgment point to reduce edge buffering requirements.
Devices may retry until the event log ACKs a durable write.

## Broker considerations

A broker may be used as an implementation of the event and audit plane, but HMF-ICS v1 does not require a broker.
If a broker is used, it SHOULD NOT be required for direct HMI ↔ PLC command delivery.
