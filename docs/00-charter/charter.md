# Charter

## Mission

HMF-ICS is a security-first OT/ICS protocol that enables authenticated, authorized, and auditable communication
between OT endpoints (for example, an HMI and a PLC) while remaining resilient to common industrial failure modes
such as partitions and partial outages.

## One-paragraph category definition

HMF-ICS is a security-first OT/ICS communication protocol and reference implementation scope that separates the
data plane (direct HMI ↔ PLC operational messages) from the control plane (site-local authority, enrollment, and
policy distribution) and from the event and audit plane (durable, signed receipts and security events). Unlike a
traditional telemetry broker or a SCADA-centric message hub, HMF-ICS requires receiver-enforced validation
(signature, freshness, replay resistance, and authorization) for all state-changing actions, continues safe local
operation during partial outages, and produces cryptographically attributable audit evidence at the point of
physical effect.

## Priorities (v1)

In order:

1. Security correctness
2. Operational resilience
3. Demonstrability
4. Regulatory alignment (design consideration)
5. Future productization

## Non-goals (v1)

HMF-ICS v1 is not:

- A universal telemetry broker.
- A cross-site control-plane trust mesh.
- A distributed consensus system.
- A replacement for every OT protocol in one step.
- A cloud-dependent architecture.

Quorum is out of scope for v1.

## Normative language

The key words MUST, MUST NOT, SHOULD, SHOULD NOT, and MAY are to be interpreted as described in RFC 2119.

## Spec hierarchy

If any document conflicts:

1. `docs/02-protocol/*` (normative protocol) wins.
2. `docs/03-security/*` (normative security properties) wins over architecture guidance.
3. `docs/01-architecture/*` provides system shape and guardrails.
4. Everything else is informative.

The reference implementation MUST conform to normative documents.
