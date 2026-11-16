# P1-ADR-TheLogIsTheRecord

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

Why a quorum formed, or did not, has to be reconstructible after the fact. That includes a vote that arrived too
late, a member that voted twice, and a node that committed what another did not. It must work from a log Cartel
assumes nothing about: a line may be lost.

A full-state dump on every call would cost more than the lines it is read against.

## Decision

Every call is answered with a receipt, logged as `(number, command, output)`. The receipt holds the output and a
number the instance counts itself, one more than the last.

- The state is rebuilt by replaying the receipts.
- It is resynchronised from a checkpoint the caller asks for.
- A gap in the numbers proves a lost line.

## Forcing constraints / Evidence

**The state is derivable.** `process` is total and deterministic
([P1-ADR-OneEntryPointOneOutput](P1-ADR-OneEntryPointOneOutput.md)), so the state after a call follows from the
state before it and the command.

**A lossy log looks complete without numbers.** With a number on every receipt, a gap is detectable and the replay
knows when to resynchronise.

**Cartel can count its calls itself.** A number is a count, not a time: it needs no clock, is deterministic, and is
restored with the checkpoint.

## Rejected alternatives

**The full state on every call.** Rejected: it is derivable.

**An ordinal the caller supplies.** Rejected for beggar's reasons: a gap would prove loss only where the caller
numbered every call.

**A checkpoint cadence held by Cartel.** Rejected: when to checkpoint is a policy.

## Consequences

**Receipts.** `process` returns a receipt carrying the `number` and the `output`; every call gets one. An instance's
first receipt is number 1. The instance id, a node and an incarnation, tells two instances' receipts apart.

**The checkpoint is a query: the whole books.** It holds:
- the instance id and the bounds;
- every open ballot with its electorate, thresholds and tally;
- the close point;
- the next receipt's number.

**An instance has one of two origins:** fresh, or a checkpoint.

**Restoring a checkpoint may serve, unlike in beggar.** Beggar forbids a live restore because a restored instance
would mint names again. Cartel mints no names, so a restored instance can serve: a protocol that keeps its books
across a restart, rather than lose a round, restores them. Neither Raft nor IBFT does today; IBFT gave its
volatile snapshot up (D20, D21) and starts from its hard state alone.
- **A restart** restores into a new incarnation, so its receipts are told apart from those before the crash.
- **A replay** restores under the same one.

**Storing the checkpoint is the caller's:** a WAL, a file, nothing.

**Forgetting is safe.** A fresh or restored instance holds a subset of the votes the instance it replaces counted.
The specification states the property this rests on: given any subset of the votes counted, Cartel reports no
threshold reached that the whole set did not reach.
- Counting only adds, so a crash can make Cartel miss a quorum and never report one its counted votes did not reach.
- A faulty member that voted two choices may have its second counted once its first is forgotten. Quorum
  intersection holds against it, not the books.
- Safety across a crash rests on the protocol's own durable promises, its own vote above all, never on Cartel's
  books ([`../DESIGN.md`](../DESIGN.md) §8).

## Enforcement

- The property tests check that every call's number is one more than the last, refused calls included.
- A property test keeps any subset of a run's counted votes and replays it: quorums reached may vanish, never
  appear.
- The conformance vectors give every expected receipt.
- The replay tier ([P2-ADR-TestingLadder](P2-ADR-TestingLadder.md)):
  - rebuilds every receipt from a log;
  - detects a dropped line;
  - from 0.4.0, resynchronises from a checkpoint;
  - from 0.4.0, restores into a new incarnation.

## Related

beggar's ADR of the same name.
