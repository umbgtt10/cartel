# P0-ADR-CartelCountsNeverDecides

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P0 (axiom)

## Context

Consensus needs decisions and it needs counting.

- **Decisions:** whether a vote is genuine, what threshold a ballot needs, which ballot to open next, when a round
  times out, what a reached quorum means, what to do with a member who voted twice.
- **Counting:** who voted for what, and whether a threshold is reached.

Kept in one crate, the counting inherits everything the decisions need: a clock, cryptography, a transport,
storage, membership and one protocol's rules.

## Decision

Cartel counts and never decides. It:
- checks no signature;
- computes no threshold;
- casts no vote of its own accord;
- times nothing;
- stores nothing durably;
- applies no protocol's rule.

It has one injected dependency, an observer it tells of every call and asks nothing of.

## Forcing constraints / Evidence

**A signature is technology.** Per-voter signatures, BLS aggregates, threshold shares and post-quantum schemes
differ in every respect a bookkeeper would have to know. The protocol verifies a vote and only then casts it, so a
vote Cartel counts is a vote the protocol vouches for.

**A threshold is a rule.** Majorities, 2f+1, Flexible Paxos's two phase quorums and fast quorums are each a
protocol's arithmetic ([P1-ADR-TheProtocolSuppliesTheThresholds](P1-ADR-TheProtocolSuppliesTheThresholds.md)).

**The voter's side is the protocol's.** Whether this node votes, and for what, is Raft's up-to-date check or
IBFT's lock. Its own vote, once given, is cast like anyone's, and the record that it voted is durable protocol
state.

**Determinism needs no clock and no randomness.** A sample to poll, a timeout and a retry are the caller's; what
reaches Cartel is the command that follows from them.

## Rejected alternatives

**An injected verifier.** Rejected: Cartel would carry evidence it cannot interpret, and the conformance suite
would need stubs.

**Cartel tracking the node's own vote.** Rejected: it is durable state with a protocol's rules, the term's
`votedFor` and IBFT's `prepare_sent`.

**Cartel acting on equivocation**, excluding or marking the member. Rejected: that is a judgment, and the
protocol's ([P1-ADR-EquivocationIsReportedNotJudged](P1-ADR-EquivocationIsReportedNotJudged.md)).

## Consequences

Three layers:
1. Cartel counts.
2. Each protocol's integration point translates.
3. The protocol decides.

The dependency runs one way: a protocol depends on Cartel, never the reverse.

Numbers the caller fixes at construction are configuration, not policy: the bounds
([P1-ADR-BoundsGivenAtConstruction](P1-ADR-BoundsGivenAtConstruction.md)).

What would need storage, the books across a restart, is the caller's to keep: it asks for the checkpoint and
restores from it ([P1-ADR-TheLogIsTheRecord](P1-ADR-TheLogIsTheRecord.md)).

With nothing injected that answers, any implementation in any language is tested with nothing to stub.

## Enforcement

The crate's manifest has no dependencies. Cartel calls out through one trait, `Observer`, whose one method returns
nothing.

## Related

beggar's `P0-ADR-AccountsNeverDecides` — the same stance for recovery.
