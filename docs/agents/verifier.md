# Role Card: Verifier

## Purpose

Confirm that the implementation passes required verification checks.

## Inputs

- Repository state after implementation

## Outputs

Verification report.

## Responsibilities

Run required checks such as:

- cargo fmt
- cargo clippy --all-targets -- -D warnings
- cargo test --all
- cargo doc --workspace --no-deps

Ensure acceptance criteria for the slice are satisfied.
