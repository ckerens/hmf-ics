# Key management (v1)

Status: Normative
Applies to: All compliant deployments

## Trust model overview

1. Each endpoint possesses an Ed25519 signing keypair.
2. Endpoints are not trusted by default.
3. A site authority (Warden) explicitly approves endpoint identities.
4. Approved public keys are bound to a key_id.
5. Only approved keys are accepted for envelope validation.

Operational trust is rooted in the site authority.

Manufacturer identity, if present, does not grant operational authority by itself.

## Key types

### Endpoint signing key

- Algorithm: Ed25519
- Purpose: Sign envelopes
- Scope: Unique per endpoint identity
- MUST be unique per logical endpoint

### Site authority state

The site authority maintains:

- key_id → public key mappings
- approval metadata and status
- revocation status

## Enrollment

An unenrolled endpoint:

- MUST NOT be authorized to perform state-changing operations.
- MAY transmit enrollment requests.

Enrollment request MUST include:

- sender_id
- proposed sender_instance
- proposed key_id
- endpoint public key
- optional manufacturer attestation evidence

Enrollment requests MUST be signed by the endpoint private key.

Approval workflow is implementation-defined but MUST be explicit and auditable.

## key_id semantics

- key_id MUST uniquely identify a specific public key within a site.
- key_id MUST NOT refer to multiple distinct public keys.
- key_id MUST remain stable for the lifetime of the associated key.
- Key rotation MUST result in a new key_id.

If key_id is unknown, the message MUST be rejected and a security audit event SHOULD be emitted.

## Key rotation

A endpoint MAY rotate its signing key.

Rotation requires:

1. Generate a new keypair.
2. Submit a new enrollment request.
3. Explicit approval by site authority.

Until approved, messages signed with the new key MUST be rejected.

Overlap windows MAY be supported but MUST be deterministic and time-bounded.

## Revocation

The site authority MUST be able to revoke any approved key.

Revocation MUST:

- Immediately invalidate the key for signature verification.
- Prevent further message acceptance.

During partition:

- Revocation propagation may be delayed.
- Receivers MUST enforce revocation based on locally available state.
- Revocation state MUST NOT be weakened due to loss of connectivity.

## Optional manufacturer attestation (extension hook)

An endpoint MAY provide manufacturer evidence (TPM quote, secure element proof, certificate).
Verification is implementation-defined.

Manufacturer evidence MUST NOT grant operational authority by itself.

## Persistence requirements

Receivers MUST persist:

- Approved key_id → public key mappings
- Revocation status
- Approval metadata

If trust registry state is lost, unknown keys MUST be rejected.

## No runtime negotiation

HMF v1 does not support algorithm negotiation, downgrade, or automatic trust of new keys without approval.
All trust transitions MUST be explicit.
