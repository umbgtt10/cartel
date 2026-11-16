# P1-ADR-BoundsGivenAtConstruction

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

Cartel's memory grows with what the protocol asks it to hold:
- the open ballots;
- each ballot's electorate;
- the distinct choices voted in each.

In a protocol tolerating Byzantine faults, what the protocol asks it to hold follows partly from what faulty
members send. IBFT opens ballots for rounds ahead of its own on a view change, and a choice is bytes a member
supplies. On a microcontroller, the node that runs out of memory is lost to the cluster.

## Decision

An instance is created with three optional bounds, as beggar's bound on open sessions is:
- the most open ballots;
- the largest electorate;
- the longest choice.

A bound left out is no bound. A command that would exceed a bound given is refused, changing nothing.

## Forcing constraints / Evidence

**A number fixed at construction is configuration, not policy.** Cartel does not choose it, change it, or decide
which ballot matters more.

**Three numbers give memory a ceiling.** Within a ballot, each member is counted for at most one choice, so a
ballot holds at most as many choices as members. The books are therefore bounded by:

open ballots × largest electorate × (longest choice + a member's record).

The memory tier measures against that.

**Optional is additive.** An instance without bounds behaves as one built before bounds existed. The bounds can
therefore arrive in a later slice without changing an answer a released vector gives.

**A refusal belongs in the record.** A caller could police its own calls, but a refused open would then leave no
line.

## Rejected alternatives

**No bounds.** Rejected: the caller's only defence would be its own count beside Cartel's.

**Mandatory bounds.** Rejected:
- They would have to arrive in the first slice.
- Every caller would choose numbers, Raft and every test included.
- What they guarantee, a ceiling on every instance, is the integration's to guarantee: an integration that must
  bound memory sets the bounds.

**Mandatory, with "unbounded" stated explicitly.** Rejected: the cost of mandatory bounds, for an optional bound
spelled longer.

**A default inside the crate.** Rejected: a default is a number Cartel would be choosing.

**Evicting the oldest ballot to make room.** Rejected: choosing which ballot to give up is a policy.

## Consequences

**Integrations that tolerate Byzantine faults, or run on small hosts, set all three.** IBFT takes them from its
protocol limits. A microcontroller's deployment sets them from its memory budget. Raft may leave them out.

**Bounds protect memory, not liveness.** A full instance refuses honest ballots as readily as faulty ones. Which
ballots to open, and what to do when an open is refused, is the protocol's. IBFT's choice of which view changes for
rounds ahead it opens ballots for matters for that reason.

**The bounds are part of the instance:** given at construction, carried in the checkpoint when given, and changed
by no command.

**A full instance opens again** as soon as a close frees a ballot.

**Their refusals arrive with them.** Books full, electorate too large and choice too long can arise only with a
bound given, so they add rows without changing any earlier answer.

## Enforcement

- The invariant that the books never exceed a bound given.
- The matrix's rows at each bound.
- The exhaustive matrix over small bounds, none included.
- The memory tier at the bounds.

## Related

beggar's `P1-ADR-AnOptionalBoundOnOpenSessions`;
[P1-ADR-AChoiceIsOpaqueBytes](P1-ADR-AChoiceIsOpaqueBytes.md) — the longest choice.
