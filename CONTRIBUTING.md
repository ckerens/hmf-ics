# CONTRIBUTING.md

## Purpose

This document defines the contribution workflow for the HMF-ICS repository.

All contributors (human or automated) must follow the workflow defined here to maintain architectural consistency, security guarantees, and protocol correctness.

---

## Development Workflow

### 1. Work Item

Before starting development, ensure that a work item exists.

Work items should live in:

```text
docs/work/
```

A work item should include:

- Goal
- Context
- Normative impact
- Acceptance criteria
- Out of scope

---

### 2. Create a Branch

Create a branch from `main` before beginning work.

Branch naming conventions:

```text
feature/<short-description>
fix/<short-description>
refactor/<short-description>
docs/<short-description>
```

Example:

```text
feature/audit-message
```

Branches should remain short-lived.

---

### 3. Implement the Change

Implementation must follow architectural constraints defined in:

- `AGENTS.md`
- `docs/adr/`
- protocol specifications in `docs/02-protocol`

Agents and contributors must not modify protocol semantics without updating the specification.

---

### 4. Commit Changes

Commit messages should follow a structured format.

Example:

```text
feat(protocol): add audit message type

Implements audit message support as defined in WI-0001.
```

Recommended prefixes:

```text
feat
fix
docs
refactor
test
chore
```

---

### 5. Open a Pull Request

All changes must be submitted through a pull request.

The pull request must:

- reference the related work item
- explain the change
- include tests
- update documentation if behavior changed

---

## Testing Requirements

All new functionality must include tests.

Testing expectations include:

- unit tests for new logic
- serialization round-trip tests
- validation failure tests
- security-relevant negative tests

---

## Documentation Requirements

Documentation must be updated if:

- protocol semantics change
- message formats change
- security assumptions change
- operational behavior changes

Normative documentation must remain synchronized with the implementation.

---

## Architectural Changes

Architectural changes require an Architecture Decision Record (ADR).

ADRs are stored in:

```text
docs/adr/
```

Changes affecting:

- protocol semantics
- cryptographic primitives
- validation pipeline
- crate boundaries

must include a new ADR.

## Rust Documentation and Commenting Standards

This repository follows standard Rust documentation conventions with additional rules appropriate for a security-sensitive protocol implementation.

### Rustdoc usage

Rust documentation uses Rustdoc comments.

- `///` documents items (functions, structs, enums, traits)
- `//!` documents modules and crates

All **public items must include Rustdoc documentation**.

Examples of public items requiring documentation:

- `pub struct`
- `pub enum`
- `pub fn`
- `pub trait`
- `pub mod`

Public documentation should describe:

- purpose
- protocol semantics when relevant
- security considerations
- error behavior

### Module documentation

Modules should begin with a module-level Rustdoc comment explaining the purpose of the module.

Example:

```rust
//! Envelope validation and structural integrity checks.
//!
//! This module implements receiver-side validation of HMF envelopes,
//! including signature verification, freshness checks, and replay protection.
```

### Comment philosophy

Comments should explain **why something exists**, not restate what the code does.

Good comments explain:

- invariants
- protocol requirements
- security reasoning
- unusual implementation choices

Example:

```rust
// INVARIANT: replay state must only be updated after signature validation.
// Updating earlier would allow unsigned messages to poison replay tracking.
```

Avoid narrating obvious code behavior.

Bad:

```rust
// increment the counter
counter += 1;
```

Good:

```rust
// SECURITY: counter must remain monotonic per sender_instance to prevent replay.
counter += 1;
```

### Standard comment tags

Use the following comment prefixes consistently:

| Tag | Purpose |
| ----- | -------- |
| `INVARIANT:` | Describes a property that must always hold |
| `SECURITY:` | Explains a security-critical design decision |
| `DEV-ONLY:` | Temporary development implementation |
| `TODO(name):` | Future improvement |
| `FIXME(name):` | Known incorrect or incomplete behavior |
| `SAFETY:` | Required explanation for unsafe code |

Example:

```rust
// DEV-ONLY: placeholder signing key used for local demo environment.
```

Example:

```rust
// INVARIANT: canonical signing bytes must match ADR-0002 ordering rules.
```

### Unsafe code

Unsafe code should be avoided in protocol logic whenever possible.

If unsafe code is required, it **must include a Rustdoc `# Safety` section** explaining the constraints.

Example:

```rust
/// # Safety
///
/// Caller must guarantee that the provided buffer contains valid
/// canonical signing bytes and is not mutated during verification.
```

### Documentation scope

| Visibility | Requirement |
| ------------ | ------------- |
| `pub` | MUST include Rustdoc |
| `pub(crate)` | SHOULD include Rustdoc if non-trivial |
| private | comments optional |

### Code structure over comments

Prefer improving readability through:

- descriptive naming
- small functions
- explicit types
- clear module boundaries

Refactor complex logic instead of documenting confusing code.

### Protocol and security comments

Security-sensitive code should include comments describing the reasoning behind decisions.

Typical areas requiring explanation include:

- signature verification
- replay protection
- canonical encoding
- authorization logic
- key handling

Example:

```rust
// SECURITY: signatures are computed over canonical signing bytes rather than
// raw protobuf encoding to prevent field ordering ambiguity across encoders.
```

### AI agent requirements

AI agents contributing code must follow the same standards:

- All `pub` items must include Rustdoc
- Security-sensitive logic must include explanatory comments
- Comments must explain invariants and rationale
- Comment tags must be used consistently

Agents must not introduce undocumented public APIs.
