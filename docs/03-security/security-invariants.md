# Security invariants (v1)

Status: Normative
Scope: All protocol-compliant implementations

If any invariant is violated, the system is considered insecure even if functionality appears correct.

## Explicit authority only

Authority MUST be explicit, cryptographically bound, verifiable by the receiver, scope-limited, and time-bounded.
Network position and infrastructure position do not grant implicit privilege.

## Message-level authentication

Every state-changing message MUST be cryptographically signed and verified prior to semantic processing.
Transport security is insufficient to establish trust.

## Receiver-enforced freshness

Receivers MUST reject messages that exceed TTL, regress counters, or duplicate idempotency identity within the
enforcement window.

Freshness enforcement MUST NOT depend on sender wall-clock time.

## Deterministic validation order

Security validation MUST occur before semantic execution.
No side effects may occur before full validation.

## No algorithm downgrade

Implementations MUST NOT negotiate weaker algorithms or silently fallback.
Cryptographic changes require a protocol version increment.

## Discovery is sensitive

Enumeration of global topology and authority metadata MUST require explicit authorization.

## Partition-safe degradation

Loss of dependencies MUST reduce privilege, not expand it.
Instability MUST NOT result in relaxed authorization.

## No implicit trust from infrastructure

Compromise of VPN, gateways, firewalls, proxies, or cloud ingress MUST NOT confer protocol authority.
All protocol-level decisions MUST be independently verifiable.

## Identity integrity

Device identity MUST be unique, cryptographically bound, and resistant to cloning within the deployment's threat
model.
Sender instance reuse without appropriate reset semantics is prohibited for state-changing operations.
