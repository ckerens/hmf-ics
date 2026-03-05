# Work Items

This directory contains implementation work items for HMF-ICS.

Work items are small, focused tasks used to coordinate development work.
They provide implementation guidance without replacing the normative
protocol specification.

---

## Purpose

Work items exist to:

- define implementation tasks
- document development intent
- provide context for pull requests
- enable structured AI-assisted development

They are **not architectural design documents**.

Architectural changes require an **ADR** in:

```text
docs/adr/
```

---

## Relationship to Other Documents

| Artifact | Purpose |
|--------|--------|
| `docs/02-protocol` | Normative protocol specification |
| `docs/adr` | Architecture decision records |
| `docs/invariants.md` | Non-negotiable system constraints |
| `AGENTS.md` | Rules for AI agents |
| `CONTRIBUTING.md` | Human development workflow |

Work items should reference these artifacts when relevant.

---

## Naming Convention

Work item files should follow this format:

```text
WI-0001-short-description.md
```

Examples:

```text
WI-0001-audit-message.md
WI-0002-authorization-validation.md
WI-0003-transport-adapter.md
```

Numbers should increment sequentially.

---

## Lifecycle

Typical lifecycle:

1. Work item created
2. Implementation branch created
3. Code implemented
4. Pull request opened
5. CI validation
6. Merge to `main`

After merge, the work item remains as historical context.

---

## Writing Guidelines

Work items should:

- remain small and focused
- avoid architectural speculation
- clearly define acceptance criteria
- reference ADRs where relevant

If the work item requires architectural change,
create an ADR instead.
