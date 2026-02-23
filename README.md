# Hardened Management Fabric for Industrial Control Systems (HMF-ICS)

**HMF-ICS** is an open, security-first protocol and reference
implementation for industrial control environments.

This repository defines a protobuf-first wire contract and a Rust
implementation of a management and enforcement fabric designed
specifically for OT/ICS systems.

------------------------------------------------------------------------

## Why This Exists

This is an academic engineering project with the purpose of better
understanding the OT/ICS security landscape.

My learning process strongly favors authentic problems --- So, I did
what any engineer would do — I started writing a protocol.

OT/ICS environments impose constraints that are structurally different
from typical IT systems:

-   Long-lived devices
-   Deterministic operational requirements
-   Intermittent or segmented connectivity
-   Partition risk
-   Slow DoS susceptibility
-   High audit and traceability requirements
-   Safety implications

Rather than layering security onto an existing messaging model, this
project explores what a protocol would look like if operational security
and reliability were first-order design constraints.

------------------------------------------------------------------------

## What This Is

HMF-ICS is:

-   A protocol specification (protobuf-first wire contract)
-   A reference architecture
-   A Rust workspace implementing core roles
-   A structured experiment in secure authority and degradation
    semantics

It is not:

-   A production-ready ICS platform
-   A monitoring product
-   A wrapper around MQTT or another broker
-   A vendor positioning exercise

This is a ground-up design exploration.

------------------------------------------------------------------------

## Core Design Principles

### 1. Security Is Structural

Security properties are embedded into the envelope and protocol
semantics.

Every message is structured to support:

-   Protocol versioning
-   Scoped authority boundaries
-   Monotonic counters
-   TTL enforcement
-   Transaction identifiers
-   Delivery profiles
-   Signed security fields

Some of these mechanisms are implemented; others are defined in the wire
contract and under active development.

------------------------------------------------------------------------

### 2. Authority Is Explicit and Time-Bounded

Authority is scoped and intentionally constrained.

Devices operate within:

-   A defined **Authority Domain**
-   Explicit scope validation
-   Deterministic fallback behavior when authority becomes stale or
    invalid

Secure degradation is treated as a required behavior, not an edge case.

------------------------------------------------------------------------

### 3. Devices Participate in Enforcement

Devices are not passive endpoints.

They:

-   Validate scope and authority
-   Enforce policy locally
-   Maintain monotonic counters
-   Emit lifecycle telemetry
-   Reject stale or invalid commands

Control integrity is not assumed to exist at a single central node.

------------------------------------------------------------------------

### 4. Auditability Is First-Class

HMF-ICS supports:

-   Transaction IDs
-   Idempotency keys
-   Ordered delivery profiles
-   Correlatable request/ack/result flows

The protocol is designed to make reconstruction and analysis
straightforward.

------------------------------------------------------------------------

### 5. Reliability Before Feature Density

The architecture prioritizes:

-   Deterministic behavior
-   Partition tolerance
-   Explicit TTL handling
-   Clear failure semantics

Feature growth is secondary to correctness.

------------------------------------------------------------------------

## Roles and Terminology

### Warden

The authority node responsible for:

-   Policy issuance
-   Command authorization
-   Domain enforcement
-   Security validation

A Warden governs a defined **Authority Domain**.

------------------------------------------------------------------------

### Authority Domain

A scoped control boundary, such as:

-   `zone:cellA`
-   `site:plant7`
-   `control_center:east`

Messages are validated against domain scope.

------------------------------------------------------------------------

### Device

An enforcement participant that:

-   Emits telemetry
-   Validates commands
-   Enforces policy locally
-   Maintains monotonic counters
-   Degrades deterministically if authority becomes invalid

------------------------------------------------------------------------

### Operator

A human principal interacting via authenticated tooling.

Operators operate through protocol guarantees --- not around them.

------------------------------------------------------------------------

## Protocol Architecture

HMF-ICS is envelope-centric.

Each message is wrapped in an `Envelope` that includes:

-   `proto_ver`
-   `msg_class`
-   `sender_id`
-   `sender_instance`
-   `counter`
-   `ttl_ms`
-   `transaction_id`
-   `idempotency_key`
-   `delivery_profile`
-   `scope`
-   `target`
-   `topic`
-   Security metadata

Payloads are defined using protobuf `oneof` structures.

Current payload categories include:

-   Telemetry
-   Command
-   Configuration
-   Engineering actions

The protobuf schema is the canonical contract.

------------------------------------------------------------------------

## Repository Structure

This repository is a Rust workspace:

    crates/
      hmf-core/         → Core validation, signing, shared logic
      hmf-transport/    → Transport abstractions and implementations
      hmf-wire-proto/   → Protobuf schemas + generated Rust types

    bins/
      hmf-device/       → Demo device implementation
      hmf-operator/     → Operator simulator
      hmf-warden/       → Warden reference implementation

The binaries are minimal reference implementations used to exercise and
validate the protocol. They demonstrate current implementation status
rather than representing production-ready components.

------------------------------------------------------------------------

## Current Status

Early-stage, functional in limited paths.

Implemented:

-   Protobuf wire contract
-   Envelope structure and parsing
-   Telemetry message flow
-   Transport abstraction layer
-   Reference binaries for exercising protocol paths

Defined in schema but not yet fully enforced:

-   Monotonic counter validation
-   Lease / authority expiration semantics
-   Full Warden-side authority enforcement
-   Complete command lifecycle validation

This project is actively evolving. Expect iteration and structural
refinement as security semantics are hardened.

------------------------------------------------------------------------

## Build Requirements

Ubuntu / WSL example:

``` bash
sudo apt update
sudo apt install -y build-essential protobuf-compiler
```

Install Rust (via rustup), then:

``` bash
cargo check
```

------------------------------------------------------------------------

## License

Licensed under the Apache License, Version 2.0.