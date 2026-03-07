# AGENTS.md

Agents must follow this document in order of precedence from top to bottom.

## Purpose

This file defines the operating rules for AI agents and automated tools interacting with the HMF-ICS repository.

HMF-ICS is a protocol specification and reference implementation for a security-first OT/ICS communication fabric. The documentation in this repository is the authoritative engineering specification, and the Rust implementation serves as a conformance reference implementation.

AI agents are expected to operate within the constraints defined by the specification, ADRs, and architectural invariants.

Agents implement work.
Humans make architectural decisions.

If a task requires architectural judgment or changes to protocol semantics, the agent must stop and request human direction.

---

## Repository Overview

The repository contains two primary components:

1. Normative protocol documentation
2. Rust reference implementation

The documentation defines the behavior of compliant implementations.
The code exists to demonstrate and validate the specification.

Normative documentation uses RFC-style language:

- MUST
- MUST NOT
- SHOULD
- SHOULD NOT
- MAY

If behavior is not explicitly defined in a normative document, it is undefined and MUST NOT be assumed.

Documentation structure:

```text
docs/
  00-charter       Vision, principles, glossary
  01-architecture  System and component architecture
  02-protocol      Normative protocol specification
  03-security      Security properties and threat model
  04-operations    Deployment and failure modes
  05-compliance    Standards notes
  06-testing       Conformance guidance
  adr              Architecture Decision Records
```

---

## Core Architectural Invariants

The canonical invariant catalog is defined in:

docs/invariants.md

Agents MUST treat this document as the authoritative source of architectural invariants and MUST preserve all `INV-*` constraints defined there.

If a requested change would violate any invariant, the agent must halt and request clarification.

### 1. Protocol Specification Is the Source of Truth

The protocol specification in `docs/02-protocol` defines behavior.

The Rust code must implement the specification.
The specification does not adapt to the code.

If code and specification disagree, the code is incorrect.

### 2. Canonical Signing Bytes

Signatures MUST be computed using canonical signing bytes defined in ADR-0002.

Requirements include:

- domain separation tag `HMFv1:envelope-signature`
- deterministic field ordering
- stable numeric encoding
- deterministic ordering of map entries
- `auth_context` hashed before inclusion
- payload signed via `payload_hash`

Agents MUST NOT modify signature semantics without updating ADR-0002 and associated documentation.

Any change to canonicalization requires new test vectors.

### 3. Layering: Core vs Wire

The layering model defined in ADR-0001 is mandatory.

High-level semantics live in:

```text
hmf-core
```

Wire serialization lives in:

```text
hmf-wire-proto
```

Rules:

- `hmf-core` MUST NOT depend on protobuf or prost types.
- Applications MUST NOT depend directly on generated protobuf structures.
- Conversion between wire types and core types must occur in the wire layer.

This separation ensures stable protocol semantics independent of serialization format.

### 4. Validation Pipeline Ordering

Message processing follows a strict validation pipeline.

The following order MUST be preserved:

1. structural validation
2. TTL validation
3. signature verification
4. replay protection
5. idempotency check
6. authorization
7. execution
8. audit emission

Side effects MUST NOT occur before full validation.

Agents MUST NOT reorder these steps.

### 5. Operational Model

HMF-ICS assumes the following operational constraints:

- Data plane communication between endpoints is direct.
- The Warden is an authority and registry, not a telemetry broker.
- Endpoints MUST remain operational during Warden outages.
- Federation (if implemented) is export-only and MUST NOT create cross-site control dependencies.
- Consensus or quorum mechanisms are out of scope for v1.

Agents must not introduce features that violate these guardrails.

---

## Specification Anchors (`REQ-*` and `INV-*`)

The repository uses stable identifiers for requirements and invariants.

Agents MUST reference these identifiers when implementing, reviewing, or modifying behavior.

Identifier types:

- `REQ-*` — normative implementation requirements
- `INV-*` — architectural invariants

Usage rules:

- Work items MUST reference relevant `REQ-*` and `INV-*` anchors.
- Code changes SHOULD reference relevant anchors in comments where appropriate.
- Tests SHOULD reference the requirement or invariant they validate.
- Pull requests SHOULD list implemented or preserved identifiers.

Example:

Implements:

- REQ-SIG-003
- REQ-SIG-004

Preserves:

- INV-PIPE-001
- INV-SIGN-001

If an agent cannot identify a valid requirement or invariant anchor for a change, the agent MUST stop and request clarification.

---

## Repository Safety Rules

Agents MUST treat repository history and branches as durable artifacts.

The following destructive operations are prohibited unless explicitly requested by a human:

- deleting local or remote branches
- force pushing (`git push --force` or `--force-with-lease`)
- rewriting history on shared branches
- deleting tags
- deleting GitHub releases

Agents MUST NOT commit directly to `main`.

All changes must occur in feature branches and be submitted through pull requests.

Agents may:

- create feature branches
- push commits to feature branches
- open pull requests
- update pull requests

Branch deletion should be performed by a human after a pull request is merged unless explicitly requested otherwise.

## Development Workflow

All implementation work should follow the repository workflow.

### Work Items

Work items are stored in:

```text
docs/work/
```

Each work item should include:

- Goal
- Context
- Normative impact
- Acceptance criteria
- Out of scope

Agents should implement only the work described in the work item.

If the work item conflicts with specification constraints, the agent must stop and report the conflict.

### Branching

Before beginning work:

1. Create a branch from `main`.

Branch naming conventions:

```text
feature/<short-description>
fix/<short-description>
refactor/<short-description>
docs/<short-description>
```

Branches should remain short-lived.

### Commits

Commit messages should follow a structured format.

Example:

```text
feat(protocol): add audit message type

Implements audit message support as defined in WI-0001.
Adds serialization support and validation tests.
```

Common prefixes:

```text
feat
fix
docs
refactor
test
chore
```

### Pull Requests

### PR Title Format

Pull request titles MUST use:

```text
type: short description
```

Recommended types:

- ci
- docs
- core
- protocol
- security
- test
- refactor
- build

Examples:

```text
ci: add baseline checks (fmt, clippy, test)
docs: add protocol envelope documentation
core: enforce receiver validation ordering
security: add replay protection checks
```

### Creating Pull Requests with GitHub CLI

When creating pull requests using the GitHub CLI (`gh`), agents and contributors SHOULD use the repository PR template.

Preferred:

```bash
gh pr create --template .github/pull_request_template.md
```

Alternative (opens the browser UI, which applies the template automatically):

```bash
gh pr create --web
```

### Keeping Feature Branches Up to Date

Agents and contributors SHOULD ensure their work branches remain compatible with `main`.

Workflow:

1. Update local `main` before creating a branch:

   ```bash
   git checkout main
   git fetch origin
   git pull --ff-only
   ```

2. Create the work branch from updated `main`:

   ```bash
   git checkout -b feature/<short-description>
   ```

3. Before pushing and opening a PR (or before merging), bring `main` changes into the branch.

Agents MUST NOT rewrite history on shared branches and MUST NOT force-push. Prefer merging `origin/main` into the work branch:

```bash
git fetch origin
git merge origin/main
```

If conflicts occur, resolve them and commit the merge. This keeps history auditable and is compatible with branch protection rules.

If a rebase is desired, it MUST be explicitly requested by a human, and MUST NOT be used on shared branches.

All code changes must be submitted through a pull request.

Pull requests must include:

- summary of changes
- reference to work item or issue
- tests demonstrating new behavior
- documentation updates if semantics changed
- note if an ADR is required

Pull requests must pass CI checks before merging.

---

## Testing Expectations

Agents must add tests for:

- new protocol behavior
- serialization round-trip
- signature validation
- negative cases

Security-relevant changes require both positive and negative tests.

### Required local checks before opening a PR

Agents SHOULD keep the feedback loop local. CI is the final gate, but it should rarely fail for avoidable reasons.

For changes that affect Rust code (including build scripts and protobufs), agents MUST run the following checks locally and fix failures before opening a PR:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo doc --workspace --no-deps
```

For documentation-only changes, agents SHOULD still run at least:

```bash
cargo fmt
cargo doc --workspace --no-deps
```

If the change modifies protobuf definitions or `build.rs` behavior, ensure `protoc` is available in the environment (CI installs it). If local tooling differs, note it in the PR description.

---

## Documentation Requirements

Documentation must be updated when:

- protocol semantics change
- message formats change
- security assumptions change
- operational behavior changes

Normative documentation must remain consistent with the implementation.

---

## When the Agent Must Stop

The agent must stop and request human direction when:

- protocol semantics need modification
- canonical signing rules would change
- validation pipeline ordering would change
- architectural layering would change
- a new security primitive is introduced

These changes require a new Architecture Decision Record (ADR).

---

## Guiding Principle

AI agents accelerate implementation.

They do not replace engineering judgment.

All architectural decisions must remain explicit, documented, and reviewable.

The goal of this repository is to produce a clear, secure, and verifiable protocol specification with a reference implementation, not to maximize implementation speed.
