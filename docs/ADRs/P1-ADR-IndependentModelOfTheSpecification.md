# P1-ADR-IndependentModelOfTheSpecification

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

Every vector's expected receipts are written by hand, and a hand-written receipt can be wrong. The crate as the
judge of its own vectors is the alternative
[P0-ADR-SpecificationIsNormativeCrateIsReference](P0-ADR-SpecificationIsNormativeCrateIsReference.md) rejects.

## Decision

The specification has a second implementation in the repository: the package `cartel-spec-model` in
`crates/spec-model/`. It is:
- written from the prose and the decision table;
- independent of the crate;
- `no_std` and free of unsafe code;
- gated like the library.

The vectors are checked against it before the crate runs them. The crate is checked against it over command
sequences no vector holds.

## Forcing constraints / Evidence

**Independent of the crate, or worthless.** A model that shares the crate's types shares its mistakes. So the
model depends on nothing of `cartel`, and takes and answers commands in the specification's own encoding.

**The specification's shape, not the crate's.** The model reads the rows from `spec/matrix.json` and owns only what
the prose gives each:
- the situation in which it applies;
- the change it makes;
- what its output carries.

## Rejected alternatives

**A naive registry in the ladder as the judge.** Rejected: built on the crate's types, it would agree with the
crate's mistakes.

**The decision table typed into the model.** Rejected: the table is normative data; read, it is the table the
specification states.

## Consequences

The model is independent of the crate's code, not of its author. The vectors are the check on both.

A change to the specification changes the model with it.

The exhaustive matrix and property tiers compare the crate with the model, every receipt and checkpoint compared
whole in the specification's encoding.

## Enforcement

- `cargo deny`'s bans refuse `cartel` to the model.
- The model's conformance tier.
- The crate against the model, in the exhaustive matrix and property tiers.
- Stage 2 holds the model to every gate the library is held to.

## Related

beggar's and faction's ADRs of the same name.
