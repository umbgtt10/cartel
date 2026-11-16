# P0-ADR-NoStdZeroUnsafe

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P0 (axiom)

## Context

Cartel runs inside Raft and IBFT on every deployment they have, from a microcontroller to processes in a cluster:
ideally the same source, the same crate.

## Decision

`#![no_std]` with `alloc`, and `#![forbid(unsafe_code)]`.

## Forcing constraints / Evidence

The target set makes `std` unavailable on one end. The security posture makes `unsafe` undesirable throughout: a
consensus bookkeeper whose correctness rests on ownership can be reviewed by inspection.

## Rejected alternatives

**`std`.** Rejected: it excludes bare metal.

**`unsafe` for speed.** Rejected: counting votes needs none, and one block forfeits the guarantee.

**`#![deny(unsafe_code)]`.** Rejected: `deny` can be lifted locally with an `#[allow]`; `forbid` cannot.

## Consequences

- State is `alloc` collections, BTree only: their order is deterministic, a hash map's is not.
- No threads.
- No `std`-only types anywhere.
- Memory is bounded by the caller's bounds ([P1-ADR-BoundsGivenAtConstruction](P1-ADR-BoundsGivenAtConstruction.md)).

## Enforcement

- `#![no_std]` and `#![forbid(unsafe_code)]` at the crate root are enforced by the compiler.
- Stage 1 builds the crate and the specification's model for a target with no `std`.

## Related

faction's ADR of the same name.
