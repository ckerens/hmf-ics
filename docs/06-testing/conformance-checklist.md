# HMF-ICS v1 Conformance Checklist

Status: Normative (Reference Implementation), Informative (Other Implementations)

This checklist provides a high-level, engineer-friendly summary of required behaviors.
An implementation claiming HMF-ICS v1 compliance MUST satisfy all MUST items.

## 1. Cryptography

- [ ] Uses Ed25519 for envelope signatures (no alternative algorithms).
- [ ] Implements canonical signing bytes exactly as specified.
- [ ] Includes domain separation tag: `HMFv1:envelope-signature`.
- [ ] Includes SHA-256(auth_context) in signing bytes.
- [ ] Includes SHA-256(canonical_payload_bytes(payload)) in signing bytes.
- [ ] Rejects any signature verification failure.
- [ ] Rejects unknown key_id values.

## 2. Validation Sequencing

- [ ] Structural validation occurs before signature verification.
- [ ] TTL validation occurs before signature verification.
- [ ] Signature verification occurs before replay validation.
- [ ] Replay validation occurs before authorization.
- [ ] No semantic execution occurs before full validation succeeds.
- [ ] Rejections are fail-closed.

## 3. Replay and Freshness

- [ ] Enforces ttl_ms using receiver-local monotonic time.
- [ ] Enforces counter monotonicity per (sender_id, sender_instance).
- [ ] Rejects counter regression.
- [ ] Requires new sender_instance after restart if counter resets.
- [ ] Implements idempotency window per sender stream.
- [ ] Does not update replay state for unsigned messages.

## 4. Message Classes

- [ ] TELEMETRY implemented.
- [ ] COMMAND implemented.
- [ ] AUDIT implemented (including receipts).
- [ ] CONFIG and ENGINEERING implemented if claimed.
- [ ] msg_class matches payload variant (mismatch rejected).

## 5. Audit Requirements

- [ ] Emits AUDIT receipt for accepted state-changing command (recommended).
- [ ] Emits AUDIT event for rejected state-changing command (recommended).
- [ ] Audit messages are signed and validated like other messages.
- [ ] Audit generation does not require Warden availability.
- [ ] Audit buffering is bounded.

## 6. Key Management

- [ ] No Trust On First Use (TOFU) for production profile.
- [ ] Enrollment requires explicit approval.
- [ ] key_id uniquely identifies a public key.
- [ ] Key rotation requires new enrollment approval.
- [ ] Revocation immediately invalidates key locally.
- [ ] Loss of trust registry state results in conservative rejection.

## 7. Transport

- [ ] Supports TLS 1.3 over TCP or QUIC with TLS 1.3.
- [ ] No plaintext fallback.
- [ ] No algorithm downgrade negotiation.
- [ ] Enforces size limits and connection limits.

## 8. Architectural Guardrails

- [ ] HMI is a first-class endpoint with its own signing key.
- [ ] Direct HMI ↔ PLC data plane supported.
- [ ] Warden is not required for command delivery.
- [ ] Event log and read model are logically separate from Warden.
- [ ] Federation is export-only (if implemented).

## 9. Testing

- [ ] Includes golden test vectors for signing bytes.
- [ ] Includes replay regression tests.
- [ ] Includes idempotency duplicate tests.
- [ ] Includes validation failure path tests.
- [ ] Includes degraded mode tests (Warden unavailable).

If any MUST requirement is not met, the implementation MUST NOT claim HMF-ICS v1 compliance.
