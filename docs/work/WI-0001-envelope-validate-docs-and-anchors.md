# WI-0001 — Document `envelope_validate()` and Add Spec / Invariant Anchors

## Goal

Document the current `envelope_validate()` implementation in `hmf-core` and anchor it to the relevant requirements and invariants it already enforces.

This work item is documentation-only and must not change protocol behavior.

---

## Context

`crates/hmf-core/src/envelope.rs` contains `envelope_validate(env: &Envelope) -> Result<(), ValidateError>`.

This function currently performs envelope-level validation, including checks for:

- `proto_ver`
- `ttl_ms != 0`
- `counter != 0`
- `sig_alg == Ed25519`
- Ed25519 signature length
- non-empty `key_id`
- payload presence
- `msg_class` matching payload kind

This function does not yet implement the full receiver pipeline described by the architectural invariants.

The purpose of this work item is to document the current validation scope accurately and connect it to the relevant `REQ-*` and `INV-*` anchors without over-claiming conformance.

---

## Spec Anchors

- `REQ-ENVELOPE-001`
- `REQ-ENVELOPE-002`
- `REQ-ENVELOPE-003`
- `REQ-ENVELOPE-004`
- `REQ-ENVELOPE-005`
- `REQ-ENVELOPE-006`
- `REQ-ENVELOPE-007`

---

## Invariant Anchors

- `INV-ENVELOPE-001`
- `INV-ENVELOPE-002`
- `INV-ENVELOPE-003`
- `INV-SIGN-001`

---

## Scope

Allowed changes:

- Rustdoc comments on `envelope_validate()`
- inline comments referencing relevant `REQ-*` / `INV-*` anchors
- minor wording/documentation clarity improvements in the same file

Primary target:

- `crates/hmf-core/src/envelope.rs`

---

## Out of Scope

- changing validation behavior
- changing validation ordering
- adding replay protection
- adding idempotency checks
- adding authorization checks
- adding execution or audit behavior
- refactoring unrelated code

---

## Normative Impact

No normative change.

---

## Acceptance Criteria

This work item is complete when:

- `envelope_validate()` has Rustdoc explaining its current validation scope
- the implemented checks are anchored to relevant `REQ-*` / `INV-*` identifiers
- the documentation clearly states that this function is only part of the eventual full receiver validation model
- no behavior changes are introduced
- formatting, docs, and tests pass

---

## Verification

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo doc --workspace --no-deps
```

---

## Escalation Triggers

Create an escalation artifact if:

- the implemented checks cannot be mapped cleanly to existing `REQ-*` / `INV-*` anchors
- the current function behavior appears inconsistent with the spec
- documentation would require inventing semantics that are not explicitly defined
