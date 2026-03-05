# Glossary

- **Node**: A runtime instance with a transport listener (PLC process, gateway daemon, Warden service).
- **Endpoint**: A cryptographic HMF identity that signs and receives envelopes.
- **Asset**: A physical or functional element monitored or controlled (pump, valve, tank).
- **Data plane**: Direct operational communication between OT endpoints (for example, HMI ↔ PLC).
- **Control plane**: Authority issuance, enrollment, policy distribution, revocation.
- **Event and audit plane**: Durable capture of telemetry, receipts, security events, lifecycle events.
- **Read plane**: Projections derived from the event log, optimized for queries and UI subscriptions.
- **Warden**: Site-local trust authority and registry service (control plane).
- **Event log**: Durable append-only sink for events and audit records.
- **Read model**: Queryable projection built from the event log.
- **Federation bridge**: Export-only batching and forwarding component for multi-site observability.
- **Receipt**: An audit event produced by the receiver applying physical effect, attesting to acceptance,
  rejection, and (when applicable) execution outcome.
- **Capability**: Cryptographically bound authorization artifact evaluated by the receiver.
- **Auth context**: Opaque bytes carried in the envelope and evaluated by the receiver as authorization input.
- **Sender instance**: A boot or instantiation identifier used to scope counter monotonicity.
