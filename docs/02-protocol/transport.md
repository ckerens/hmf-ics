# Transport profile (v1)

Status: Normative
Applies to: All compliant transports

## Purpose

Transport provides confidentiality, integrity, and channel hardening for HMF traffic.
Transport does not establish protocol authority. Envelope signatures remain authoritative.

## Required transport modes

A compliant implementation MUST support at least one of the following:

- TLS 1.3 over TCP
- QUIC using TLS 1.3

An implementation MAY support both.

## Prohibited behavior

Transports MUST NOT:

- Allow plaintext fallback for protocol traffic.
- Negotiate TLS 1.2 or older for protocol traffic.
- Perform cryptographic downgrade negotiation within HMF.

## Mutual authentication

Mutual authentication is deployment-defined:

- A deployment MAY use mTLS with a site CA.
- A deployment MAY use pinned keys or pinned certs.

Regardless of mutual authentication:

- Envelopes MUST be signed and verified as defined in the protocol.
- Loss or compromise of transport infrastructure MUST NOT confer HMF authority.

## Transport hardening requirements

Transports SHOULD implement:

- Connection limits and backpressure.
- Frame size caps.
- Per-peer rate limiting.
- Idle timeouts.
- Resource accounting (to mitigate slow-read and slow-loris style attacks).
