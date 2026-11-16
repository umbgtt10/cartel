# P1-ADR-ABallotIsAQuestionARoundAndAPhase

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

The protocols name the thing a vote is counted in differently:
- Raft: a term and a kind (pre-vote or vote);
- IBFT: a height, a round and a phase (prepare, commit or view change);
- Multi-Paxos: a slot and a ballot number;
- Tendermint: a height, a round and a step.

Cartel needs one key for all of them, and an order on it: closing lets go of every ballot before a point
([P1-ADR-ClosingIsExplicitAndRemembered](P1-ADR-ClosingIsExplicitAndRemembered.md)).

## Decision

A ballot is identified by a **question**, a **round** and a **phase**, three u64.

- **The question** is what is being decided.
- **The round** is one attempt at deciding it.
- **The phase** is a stage of a round: a number Cartel never interprets.

Ballots are ordered by question, then round, then phase.

## Forcing constraints / Evidence

**Two ordered levels are what closing needs.** IBFT lets go of a round's ballots at the next round and of a
height's at its commit. Raft lets go of a term's at the next term. A single ordered key could not tell a round from
a question.

**The phase must be opaque.** A named phase is a protocol word ([P0-ADR-ConsensusIsKeptOnBallots](P0-ADR-ConsensusIsKeptOnBallots.md)).

**The phase is in the order** only so that every listing, and the checkpoint, has one canonical form.

## Rejected alternatives

**An opaque key with no order.** Rejected: closing would need one command per ballot, and listings no canonical
order.

**One ordered u64.** Rejected: IBFT would pack its height and round into one number.

**A tuple of any length.** Rejected:
- No protocol in view needs a third level.
- Closing by prefix complicates the specification and every vector.
- A key's shape can never change additively, so a general one would be a bet made forever.

**A phase as a string.** Rejected: unbounded in size, and a protocol's words would enter the books. The
protocol's observer names the phase.

## Consequences

| protocol | question | round | phase |
|---|---|---|---|
| Raft | the term | 0 | 0 for pre-vote, 1 for vote, by Raft's choice |
| IBFT | the height | the round | one number each for prepare, commit and view change |

A protocol's words for these numbers are its own, at its integration point.

Listings and the checkpoint come in ascending order of the key. Every vector carries the order from the first
slice.

**One close point fits protocols that decide in order:** Raft, IBFT, Tendermint, chained HotStuff. A protocol that
decides out of order does not fit it: pipelined Multi-Paxos, EPaxos. Slot 7 can be decided while slot 5 is not, and
closing before slot 6 would end slot 5. Such a protocol needs to close one ballot at a time. That is a command a
later slice can add, `CloseBallot`, without changing any answer.

**Independent consensus groups, shards for instance, are separate instances,** not a fourth level of the key.

## Enforcement

The schema's ballot type; the matrix's rows for closing; the canonical order in every listing's and every
checkpoint's vectors.

## Related

[P1-ADR-ClosingIsExplicitAndRemembered](P1-ADR-ClosingIsExplicitAndRemembered.md).
