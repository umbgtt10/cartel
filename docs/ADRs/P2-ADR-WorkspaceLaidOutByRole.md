# P2-ADR-WorkspaceLaidOutByRole

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P2 (structural)

## Context

The workspace will hold the specification, the library, its model, the packages that check them, and the crate
that runs the gates. Each role is held to different gates. faction and beggar lay their workspaces out by role.

## Decision

The workspace is laid out by role:

| directory | holds | packages |
|---|---|---|
| `spec/` | the specification | — |
| `crates/core/` | the library | `cartel`, published |
| `crates/spec-model/` | the specification's model | `cartel-spec-model` |
| `validation/ladder/` | the testing ladder's scenario tiers | `cartel-ladder` |
| `validation/core-validation/` | the cluster simulation | `cartel-core-validation` |
| `validation/memory/` | the memory tier | `cartel-memory` |
| `xtask/` | the gates | `xtask` |

## Forcing constraints / Evidence

The gates name the library already: CRAP, coverage, mirrored tests and file risk measure the library's crates, and
the duplication gate scans their source alone. The layout states what the gates enforce.

## Rejected alternatives

**Every package at the root.** Rejected: the roles would live only in `CLAUDE.md`.

**Directories named for their packages.** Rejected: the siblings name their core `core/`.

## Consequences

Packages reach the specification and the README two directories up.

`cartel-core` on crates.io belongs to another project, which matters to nothing here:
- the published package is `cartel`;
- the validation packages are never published.

## Enforcement

The workspace manifest's members; `dry4rust.toml` scans `crates/` only.

## Related

beggar's and faction's ADRs of the same name.
