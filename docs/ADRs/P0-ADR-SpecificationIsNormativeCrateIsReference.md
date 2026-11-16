# P0-ADR-SpecificationIsNormativeCrateIsReference

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P0 (axiom)

## Context

Cartel's consumers run on boards and on hosts, inside more than one protocol, and may one day be written in more
than one language. A definition that lives only in Rust is one implementation's behaviour, its bugs included, and
a second implementation has nothing to be checked against.

## Decision

Cartel is specified as a standard component, in a versioned `spec/` written first and by hand:
- a JSON Schema data model;
- a JSON decision table;
- normative prose;
- JSON conformance vectors.

This crate is one implementation of it. Where the two disagree, the specification is right and the crate has a
bug.

## Forcing constraints / Evidence

It is practical here for reasons specific to the design:

| because | the specification needs |
|---|---|
| no wire types | no encoding |
| no clock | no temporal logic |
| a registry, not a state machine | only a data model and one total function over a decision table |
| an observer that answers nothing as the one dependency | no stubs: an implementation runs the vectors as they are |

Nothing of Cartel is built yet, so the specification can come first.

## Rejected alternatives

**The Rust crate as the definition.** Rejected: tests in one language are not consumable by another, and prose
without vectors is not checkable.

**Vectors exported from the crate's tests.** Rejected: the crate becomes the source the specification is drawn
from, and a refactor redefines what conforming means.

**`serde` derives on the crate's types, their JSON standing as the model.** Rejected: it ties the format to one
implementation's names and nesting, and gives the crate a dependency.

**u64 as JSON numbers.** Rejected: many JSON parsers read numbers as 64-bit floats, exact only to 2⁵³.

## Consequences

The specification has four parts, versioned together:

| part | form | says |
|---|---|---|
| the data model | JSON Schema | every command, receipt, output, rejection and the checkpoint |
| the matrix | `matrix.json`, a decision table | the rows in check order, each a named situation and its output |
| the semantics | `SPEC.md`, normative prose | what JSON cannot say: each situation's meaning, totality, determinism, the watermark |
| conformance | `vectors/*.json` | `(initial state, commands) → (receipts, final state)`, each vector naming its rows |

Encodings:
- every u64 is a string of decimal digits;
- a choice is a string of lowercase hexadecimal digits, the empty string for an empty choice.

Behaviour is in the specification before it is code.

The crate never reads the specification; its conformance tests map the JSON to its types through a model of their
own. The specification's model is a second implementation
([P1-ADR-IndependentModelOfTheSpecification](P1-ADR-IndependentModelOfTheSpecification.md)).

Released vectors are never removed or edited ([P1-ADR-TheQuorumShapeIsLeftOpen](P1-ADR-TheQuorumShapeIsLeftOpen.md)).

## Enforcement

- The conformance tests run every vector.
- Every vector validates against the schema.
- Every row of `matrix.json` is exercised by at least one vector.

They arrive with the first slice, before its first function.

## Related

beggar's and faction's ADRs of the same name.
