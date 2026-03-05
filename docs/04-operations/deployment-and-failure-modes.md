# Deployment and failure modes (v1)

Status: Informative

## Default placement

- Zone A (OT):
  - HMI endpoint
  - PLC endpoint(s)
- Zone B (site services):
  - Warden
  - Event log
  - Read model

## Degraded modes

### Warden unavailable

- Enrollment and trust mutation are blocked.
- Existing authority remains usable until expiration (or local revocation).
- HMI ↔ PLC operations continue.

### Event log unavailable

- PLC continues safe operation.
- PLC buffers receipts and security events (bounded).
- Buffers may drop when full, but rejection behavior remains fail-closed.

### Read model unavailable

- Operator visibility degrades.
- HMI remains able to command PLCs directly.

### WAN unavailable (if federation is enabled)

- Local event capture continues.
- Federation resumes when link returns.

## Upgrade notes

- Protocol version changes require explicit deployment decision.
- Mixed-version behavior is undefined unless explicitly specified in versioning docs.
