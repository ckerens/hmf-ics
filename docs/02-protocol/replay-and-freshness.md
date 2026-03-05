# Replay and freshness model (v1)

Status: Normative
Applies to: All receivers and stateful senders

Freshness and uniqueness are enforced by the receiver.

## Scope of enforcement

Replay enforcement is scoped to:

- (sender_id, sender_instance) for counter tracking
- idempotency_key for duplicate suppression
- transaction_id for state transition traceability

These rules apply to all state-changing messages and SHOULD apply to all message classes unless explicitly exempted.

## TTL enforcement

ttl_ms is enforced by the receiver using receiver-local time and MUST NOT depend on sender wall-clock time.

A receiver MUST reject a message if:

(receive_time_ms - first_observed_time_ms) > ttl_ms

Where:

- receive_time_ms is receiver-local monotonic time in milliseconds.
- first_observed_time_ms is the time the message was first observed by the receiver or transport layer.

## Monotonic counter model

counter is scoped to (sender_id, sender_instance).

For a given (sender_id, sender_instance), the receiver MUST enforce:

counter_new > counter_last_seen

If counter_new <= counter_last_seen, the message MUST be rejected.

### Instance reset semantics

If a sender restarts and reuses the same sender_instance while resetting counter, receivers MUST treat this as a
replay condition and reject messages.

A legitimate reset requires a new sender_instance.

## Idempotency enforcement

idempotency_key prevents duplicate semantic execution under retransmission and at-least-once delivery.

Receivers SHOULD maintain an idempotency window per (sender_id, sender_instance). Within that window:

- If a message with identical idempotency_key has already been accepted, the receiver MUST NOT re-execute semantic
  effects.

On duplicate idempotency detection, the receiver MAY return a cached response or acknowledge without re-execution,
but MUST NOT apply state changes again.

## Persistence requirements

Receivers MUST persist replay state sufficient to enforce monotonicity and idempotency across restarts when the
deployment requires such enforcement.

If replay state is lost, the receiver MUST fail conservatively. Acceptable conservative strategies include:

- Require re-enrollment or explicit operator intervention.
- Refuse state-changing operations until replay state is re-established.
