# P1-ADR-ClosingIsExplicitAndRemembered

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

Ballots must be let go of, and the protocols do it each in their own shape:
- Raft supersedes: a new ballot of a kind replaces the last.
- IBFT clears its tallies at every new round and after every commit, discards view changes below its current round,
  and keeps only each sender's latest view change for rounds ahead.

## Decision

The protocol closes ballots explicitly: `CloseBallots` names a point, a question and a round, and every open ballot
before it is closed and forgotten.

Cartel remembers the highest point it has closed to. A later open of, or vote in, a ballot before it is refused as
closed, not as not open.

The point only moves forward: a close to a point at or behind it is refused, changing nothing, and the refusal
names the close point, so a protocol closing backwards -- a bug, or a replay after a restart -- shows in the log.

## Forcing constraints / Evidence

**When to let go is the protocol's.** An explicit close keeps Cartel free of either protocol's shape
([P0-ADR-ARegistryNotAStateMachine](P0-ADR-ARegistryNotAStateMachine.md)).

**Late is not unknown.** A vote for a round already closed is late, and IBFT's stale messages are exactly that. An
observer should see it as late, not as a ballot never heard of.

**One point costs one pair of numbers.** A tombstone per closed ballot would grow without bound.

## Rejected alternatives

**Superseding by phase.** Rejected: it is Raft's shape.

**Cartel inferring closure** from a new question or round. Rejected: when a round ends is a protocol rule.

**Tombstones per ballot.** Rejected: unbounded.

**Closing one ballot at a time.** Rejected: protocols let go by range, and one command per ballot would multiply
the calls.

## Consequences

- **The close point is part of the books,** carried in the checkpoint.
- **IBFT's rule for rounds ahead is not a close.** Keeping only a sender's latest view change for a round ahead of
  its own is a different rule. Whether it stays IBFT's policy, or Cartel withdraws a vote, is an open question in
  [`../DESIGN.md`](../DESIGN.md).
- **The bound on open ballots keeps ballots ahead finite either way.**
- **One close point fits protocols that decide in order.** A protocol that decides out of order needs to close one
  ballot at a time, a command a later slice can add
  ([P1-ADR-ABallotIsAQuestionARoundAndAPhase](P1-ADR-ABallotIsAQuestionARoundAndAPhase.md)).

## Enforcement

- The matrix's rows for a ballot below the close point, and for a close at or behind it.
- A property: no open ballot is below the close point.

## Related

[P1-ADR-ABallotIsAQuestionARoundAndAPhase](P1-ADR-ABallotIsAQuestionARoundAndAPhase.md) — the order closing relies
on.
