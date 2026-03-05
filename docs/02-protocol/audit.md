# Audit payload (v1)

Status: Normative
Applies to: All compliant endpoints

## Purpose

The AUDIT message class provides protocol-native audit and security events, including command receipts, for direct
data plane communication (HMI ↔ PLC) and for delivery to the event log.

AUDIT messages are first-class protocol messages and MUST be validated like any other message class.

## Audit events

An audit payload represents one of:

- Command receipt (accept, reject, execute outcome)
- Security event (signature fail, replay reject, authz deny, key lookup failure)
- Lifecycle event (boot, shutdown, enrollment state transitions)

## Required fields

An audit payload MUST include:

- event_type (enum AuditEventType)
- receiver_id (string)
  - Identity of the emitter of the audit record (typically the PLC for receipts).
- receiver_instance (string)
- receiver_time_ms (uint64)
  - Receiver-local monotonic time in milliseconds since process start (or equivalent monotonic basis).
- transaction_id (string)
  - Correlates with the triggering command when applicable.
- related_sender_id (string)
  - Identity of the triggering sender when applicable.
- related_sender_instance (string)
- related_counter (uint64)
  - Counter value of the triggering message when applicable.
- result_code (enum AuditResultCode)
- summary (bytes, optional, bounded)
  - A bounded, implementation-defined summary. MUST NOT include unbounded payload copies.

The audit payload SHOULD include:

- idempotency_key (string)
- correlation_id (string)
- event_id (bytes)
  - Recommended: SHA-256 of canonical audit payload bytes (excluding signature) to support deduplication.

## Audit event types

A compliant v1 implementation MUST support these event types:

- COMMAND_ACCEPTED
- COMMAND_REJECTED_VALIDATION
- COMMAND_REJECTED_REPLAY
- COMMAND_REJECTED_AUTHZ
- COMMAND_EXECUTED
- COMMAND_EXECUTION_FAILED
- SECURITY_SIGNATURE_INVALID
- SECURITY_KEY_ID_UNKNOWN
- SECURITY_TTL_EXPIRED
- SECURITY_COUNTER_REGRESSION
- SECURITY_IDEMPOTENCY_DUPLICATE
- LIFECYCLE_BOOT
- LIFECYCLE_SHUTDOWN
- LIFECYCLE_ENROLLMENT_REQUESTED
- LIFECYCLE_ENROLLMENT_APPROVED
- LIFECYCLE_ENROLLMENT_REVOKED
- LIFECYCLE_KEY_ROTATION_REQUESTED
- LIFECYCLE_KEY_ROTATION_APPROVED
- LIFECYCLE_KEY_REVOKED

Deployments MAY add additional event types, but implementations MUST NOT treat unknown event types as equivalent to
any known type.

## Command receipts (normative)

For every accepted state-changing command, the receiver applying physical effect (typically the PLC) SHOULD emit an
AUDIT receipt.

The PLC SHOULD emit one or more receipts that cover:

- Acceptance or rejection
- If accepted, execution outcome (executed or failed)

Receipts MUST NOT require Warden availability to generate.

## Delivery expectations

Audit delivery is best-effort and partition tolerant.

- Under partition, an endpoint SHOULD buffer audit events (bounded) for later delivery.
- If buffers are exhausted, endpoints MAY drop audit events, but MUST continue fail-closed validation and safe
  operation.

## Authorization

AUDIT messages MUST be subject to authorization policy.

The default posture is:

- Receipts MAY be sent from PLC to the originating HMI without additional capability beyond the command context.
- Audit events destined for the event log MUST be capability-gated by topic, scope, and target.
