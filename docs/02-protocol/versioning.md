# Versioning (v1)

Status: Normative

## Protocol version

Protocol version is defined by proto_ver in the envelope.

- HMF-ICS v1 uses proto_ver == 1.

## Breaking changes

Breaking changes require a version increment.

Examples of breaking changes:

- Signature algorithm changes
- Canonical signing bytes changes
- Validation sequencing changes
- Replay semantics changes
- Required fields or required invariants changes

## Backward compatibility

v1 implementations MUST reject proto_ver values they do not support.
