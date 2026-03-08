# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

HMF-ICS is a security-first OT/ICS protocol and Rust reference implementation. This is a **specification-driven** repository where normative documentation defines behavior and code implements the specification.

**Critical principle:** The protocol specification is authoritative. If code and specification disagree, the code is incorrect.

## Required Reading Before Making Changes

1. **AGENTS.md** - Operating rules for AI agents (MUST follow)
2. **docs/invariants.md** - Non-negotiable architectural invariants (all `INV-*` constraints)
3. **CONTRIBUTING.md** - Development workflow and documentation standards
4. **docs/adr/** - Architecture Decision Records (ADR-0001, ADR-0002 are foundational)

## Commands

### Build and Test
```bash
# Build all crates
cargo build --all

# Run all tests
cargo test --all

# Build documentation
cargo doc --workspace --no-deps

# Open generated documentation in browser
cargo doc --workspace --no-deps --open
```

### Required Pre-PR Checks
Run these locally before opening a pull request:
```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo doc --workspace --no-deps
```

### Running Specific Tests
```bash
# Run tests for a specific crate
cargo test -p hmf-core
cargo test -p hmf-wire-proto

# Run a specific test by name
cargo test --all <test_name>

# Run tests with output
cargo test --all -- --nocapture
```

### Linting and Security
```bash
# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt --check

# Lint with clippy
cargo clippy --all-targets -- -D warnings

# Check dependencies for security advisories
cargo deny check advisories

# Check license compliance
cargo deny check licenses
```

## Architecture

### Crate Structure

```text
crates/
  hmf-core/         Protocol semantics, validation, cryptography (NO protobuf dependencies)
  hmf-wire-proto/   Protobuf schema and wire encoding/decoding
  hmf-transport/    TLS/QUIC transport adapters

bins/
  hmf-device/       Reference PLC-style endpoint
  hmf-warden/       Reference site authority (enrollment, policy, revocation)
```

### Layering Model (ADR-0001)

**Strict separation enforced:**

- **hmf-core**: Protocol types, canonical signing, validation pipeline, replay protection
  - MUST NOT depend on `prost` or protobuf-generated types
  - Defines the security boundary

- **hmf-wire-proto**: Protobuf schemas and conversion to/from `hmf-core` types
  - Owns `build.rs` that generates protobuf bindings
  - Conversion layer between wire format and protocol semantics

- **hmf-transport**: Connection handling, TLS/QUIC, frame serialization
  - Uses `hmf-wire-proto` for encoding/decoding
  - Uses `hmf-core` types as API surface

**Application code** (bins, future clients) depends on `hmf-core` types, not generated protobuf structs.

### Security Model

#### Canonical Signing (ADR-0002)
Signatures are computed over **canonical signing bytes**, not raw protobuf:
- Domain separation: `HMFv1:envelope-signature`
- Deterministic field ordering and big-endian numeric encoding
- Maps sorted by key for determinism
- `auth_context` included as SHA-256 hash
- Payload signed via `payload_hash = SHA-256(canonical_payload_bytes(payload))`

Changes to canonicalization require protocol version increment, ADR update, and new test vectors.

#### Receiver Validation Pipeline (INV-PIPE-001)
Message processing follows strict ordering. **Side effects MUST NOT occur before validation completes.**

1. Structural validation
2. Freshness (TTL check using receiver-local time)
3. Signature verification (Ed25519)
4. Replay protection (monotonic counters per `(sender_id, sender_instance)`)
5. Idempotency enforcement
6. Authorization check
7. Semantic execution
8. Audit emission

**Never reorder these phases.**

### Architectural Planes

- **Data plane**: Direct HMI ↔ PLC operational communication
- **Control plane**: Warden authority (enrollment, policy, revocation)
- **Event & audit plane**: Durable receipts and security events
- **Read plane**: Queryable projections and analytics

**Critical constraint (INV-MODEL-001):** Data plane operations MUST continue during Warden outages.

## Documentation Standards

### Rustdoc Requirements
- All `pub` items MUST include `///` Rustdoc comments
- Modules should have `//!` module-level documentation
- Security-sensitive code requires explanatory comments

### Comment Tags
Use consistently:
- `INVARIANT:` - Property that must always hold
- `SECURITY:` - Security-critical design decision
- `DEV-ONLY:` - Temporary development implementation
- `TODO(name):` - Future improvement
- `FIXME(name):` - Known incorrect behavior
- `SAFETY:` - Required for unsafe code

### Comment Philosophy
Explain **why**, not **what**. Document invariants, protocol requirements, and security reasoning.

Good:
```rust
// SECURITY: counter must remain monotonic per sender_instance to prevent replay.
counter += 1;
```

Bad:
```rust
// increment the counter
counter += 1;
```

## Development Workflow

### Branch Workflow
1. Update `main`: `git checkout main && git fetch origin && git pull --ff-only`
2. Create branch: `git checkout -b feature/<description>` (or `fix/`, `docs/`, `refactor/`)
3. Implement changes following invariants and specifications
4. Run pre-PR checks locally
5. Push: `git push -u origin <branch-name>`
6. Open PR using template: `gh pr create --web` (applies template automatically)

### Branch Naming
```text
feature/<short-description>
fix/<short-description>
refactor/<short-description>
docs/<short-description>
```

### Commit Messages
```text
type(scope): short description

Longer explanation if needed.
References WI-XXXX or relevant invariants.
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

### Keeping Branches Updated
Merge `origin/main` into feature branches (do NOT force-push or rebase without explicit human approval):
```bash
git fetch origin
git merge origin/main
```

## Specification Anchors

Use stable identifiers when implementing or reviewing:

- `INV-*` - Architectural invariants (see docs/invariants.md)
- `REQ-*` - Normative requirements (see docs/02-protocol/REQ-index.md)

Reference these in:
- Work items
- Code comments where appropriate
- Tests (which requirement/invariant is being validated)
- Pull requests (list implemented/preserved identifiers)

## Work Items

Before starting implementation, ensure a work item exists in `docs/work/`.

Work items include:
- Goal and context
- Normative impact
- Acceptance criteria
- Out of scope

Implement ONLY what the work item describes. If unclear, stop and ask.

## Agent Constraints

**STOP and request human direction if:**
- Protocol semantics need modification
- Canonical signing rules would change
- Validation pipeline ordering would change
- Architectural layering would change (e.g., `hmf-core` depends on protobuf)
- A new security primitive is introduced
- Any `INV-*` invariant would be violated

These require Architecture Decision Records (ADRs).

## Protocol Version

Current protocol version: **v1** (`proto_ver = 1`)

Version changes require:
- Protocol version increment
- ADR documenting the change
- Updates to all normative documentation
- New conformance tests and test vectors

## Testing Philosophy

All new functionality requires tests:
- Unit tests for new logic
- Serialization round-trip tests
- Validation failure tests (fail-closed behavior)
- Security-relevant negative tests (replay attacks, expired TTL, invalid signatures, etc.)

Golden test vectors should exist for canonical signing of:
- TELEMETRY messages
- COMMAND messages
- AUDIT messages

## Dependencies

Core cryptographic dependencies:
- `ed25519-dalek` - Ed25519 signatures (v1 uses Ed25519 exclusively, no algorithm negotiation)
- `sha2` - SHA-256 for hashing
- `zeroize` - Secure key material handling

Wire protocol:
- `prost` - Protobuf encoding/decoding
- `prost-build` - Build-time protobuf code generation

## Environment

- Rust edition: 2024
- License: Apache-2.0
- Requires `protoc` (Protocol Buffer compiler) for building `hmf-wire-proto`

## Security Invariants Summary

Key non-negotiable constraints:
- Ed25519 signatures only (no algorithm negotiation)
- Canonical signing bytes (not raw protobuf)
- Receiver-enforced validation in strict order
- Monotonic counters prevent replay
- TTL enforced using receiver-local time
- `hmf-core` independent of protobuf types
- Data plane operational during Warden outages
- Network position does NOT grant privilege
- Unknown keys are rejected (fail conservatively)

See `docs/invariants.md` for the complete catalog.
