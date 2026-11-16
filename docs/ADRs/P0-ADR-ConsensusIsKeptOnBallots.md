# P0-ADR-ConsensusIsKeptOnBallots

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P0 (axiom)

## Context

Raft and IBFT both keep books on consensus, and keep them differently.

- **Raft** counts pre-votes and votes keyed by a term, and asks what a majority of its voters hold.
- **IBFT** counts prepares and commits keyed by a height, a round and a block hash. It counts view changes twice, at f+1 and at 2f+1, and checks the members of the certificates it is shown.

The [inventory](../INVENTORY.md) lists every such book.

A bookkeeper both can use, and the protocols after them, must hold only what they share, in words that belong to
neither.

## Decision

Cartel keeps consensus as ballots, and knows nothing else:
- ballots, each a question, a round and a phase;
- the members of an electorate;
- opaque choices;
- numbers: thresholds and positions.

It knows no term, height, slot, view, block, log, leader, validator, signature or hash.

## Forcing constraints / Evidence

**Every book marked Cartel's in the inventory reduces to these.**
- An election, a prepare, a commit and a view change are each a ballot.
- A commit index and a confirmed read are both a watermark.
- A certificate's signers are a claim.

**The same words fit protocols beyond the two.** Each one names its own unit in Cartel's words:

| protocol | its unit, in Cartel's words |
|---|---|
| Multi-Paxos | a slot is a question |
| Flexible Paxos | two phases with two thresholds |
| Tendermint | a height and a round |
| Avalanche | a poll is a ballot over its sample, with α as its threshold |

None needs a word Cartel does not have.

## Rejected alternatives

**Raft's port, extended.** Its `BallotKind` names Raft's phases, pre-vote and vote; every new protocol would add
variants to a shared enum.

**IBFT's `VoteKey`**, a height, a round and a block hash. It fixes the shape of one protocol's question and the
size of its hash.

**Generic keys and choices.** A type parameter has no wire form, so the specification could not state a vector
and the observer could not log one; and each consumer's types would leak into the books.

## Consequences

Each protocol translates at its single integration point, the one place its own words meet Cartel's: a
`raft-cartel` plug-in, IBFT's tally port.

The vocabulary in [`../DESIGN.md`](../DESIGN.md) §4 is normative, together with the words it leaves out and why.

What is protocol-specific stays in the protocol:
- the current-term commit rule;
- locking and round justification;
- which ballot to open next;
- what a reached quorum means.

## Enforcement

A test in the ladder checks the specification's schema and the crate's public identifiers against the words left
out. It lands with the first slice.

## Related

[P0-ADR-CartelCountsNeverDecides](P0-ADR-CartelCountsNeverDecides.md) — what Cartel does with the ballots: it counts
them.
