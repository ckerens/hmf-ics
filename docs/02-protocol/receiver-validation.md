# Receiver validation requirements (v1)

Status: Normative
Applies to: All compliant receivers

No semantic execution may occur until required validation steps complete successfully.

## Validation phases

Receiver processing MUST occur in distinct phases:

1. Structural validation
2. Freshness validation
3. Signature validation
4. Replay validation
5. Authorization validation
6. Semantic execution
7. Audit emission

Failure in any phase MUST abort processing immediately.

## Structural validation

Receiver MUST validate:

- proto_ver == 1
- ttl_ms != 0
- counter != 0
- sig_alg == ED25519
- signature length == 64
- key_id is non-empty
- payload is present
- msg_class matches payload variant
- enums are not UNSPECIFIED
- routing hints (topic, target, scope) are present (non-empty) for message classes that require routing

Structural validation MUST occur before signature verification to reduce unnecessary cryptographic work.

## Freshness validation

Receiver MUST enforce TTL expiration using receiver-local time as defined in `replay-and-freshness.md`.

Expired messages MUST NOT proceed to signature verification.

## Signature validation

Receiver MUST:

1. Reconstruct canonical signing bytes as defined in `envelope.md`.
2. Resolve verification key via key_id using local trust registry state.
3. Verify Ed25519 signature.

If verification fails, message MUST be rejected.
A security audit event SHOULD be emitted where feasible.

No partial semantic evaluation is permitted prior to successful signature verification.

## Replay validation

Receiver MUST enforce replay resistance as defined in `replay-and-freshness.md`.

Unsigned messages MUST never update replay state.

## Authorization validation

Receiver MUST evaluate auth_context and apply local authorization policy.

Authorization failure MUST result in rejection without semantic execution.

## Semantic execution

Only after successful completion of:

- Structural validation
- Freshness validation
- Signature validation
- Replay validation
- Authorization validation

may semantic logic execute.

## Side-effect isolation

No phase prior to semantic execution may:

- Mutate durable state (except replay tracking after signature validation)
- Emit actuator commands
- Trigger control logic

Logging and security telemetry are permitted.

## Audit requirements

For every accepted state-changing message, the receiver SHOULD emit an AUDIT receipt.

For every rejection of a state-changing message, the receiver SHOULD emit an AUDIT security event with the rejection
reason when feasible.

Audit emission MUST NOT weaken rejection behavior if emission fails.
