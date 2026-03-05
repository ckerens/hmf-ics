# Pull Request

## PR Title Format

Use:

type: short description

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

- ci: add fmt, clippy, and test checks
- docs: add protocol envelope documentation
- core: enforce receiver validation ordering
- security: add replay protection checks

## Summary

Provide a clear description of the change.

---

## Work Item

Link the related work item or issue.

Example:

```text
docs/work/WI-0001-audit-message.md
```

---

## Architectural Impact

Indicate whether this change affects architecture.

- [ ] No architectural impact
- [ ] Requires ADR update
- [ ] New ADR included

---

## Protocol Impact

Does this change modify protocol behavior?

- [ ] No protocol change
- [ ] Protocol documentation updated

---

## Tests

Describe tests added or updated.

Example:

```text
- unit tests for new logic
- serialization round-trip tests
- validation failure tests
```

---

## Security Considerations

Describe any security implications of the change.

If none, state:

```text
No security impact.
```

---

## Checklist

- [ ] Work item referenced or intentionally omitted
- [ ] Tests added or updated
- [ ] Documentation updated if needed
- [ ] CI checks passing
