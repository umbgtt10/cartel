# P1-ADR-TheProtocolSuppliesTheThresholds

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

How many votes decide is a protocol's arithmetic, and the protocols disagree:

| protocol | votes that decide |
|---|---|
| Raft, Multi-Paxos | a majority |
| IBFT | 2n/3+1 |
| PBFT with n > 3f+1 | ⌈(n+f+1)/2⌉ |
| Flexible Paxos | two sizes, one per phase |
| Fast Paxos, EPaxos | fast quorums |
| Avalanche | α of a sample of k |

IBFT also counts the same view changes against two numbers: f+1 to join a higher round, 2f+1 to send NewView.

## Decision

A ballot opens with one or more thresholds, plain numbers the protocol supplies. Cartel holds no formula and no
named rule. It reports each threshold the first time a choice's votes reach it, for that choice.

## Forcing constraints / Evidence

**A list of named rules keeps growing** and still misses half the protocols above. A number fits every one that
decides by counting.

**An injected rule would be code.** No vector can state it and no replay can rebuild it.

**Several thresholds on one ballot are real.** IBFT asks two questions of the same view changes, and the same
votes must answer both.

## Rejected alternatives

**Named rules in Cartel** (majority, two thirds). Rejected: the list grows, and each new protocol waits for a
release.

**A threshold given at construction and updated when the cluster changes**, as IBFT's `Resized` does. Rejected: it
clears every vote to stay safe. Votes cast under the old size would otherwise count against the new.

**Cartel computing from n and f.** Rejected: still a formula, and Flexible Paxos has no single f.

## Consequences

**The formula's edge cases are each protocol's tests.** For IBFT, `ValidatorSet::quorum_size` stays the one
source.

**Cartel's vectors pin counting at the threshold.** k−1 votes do not reach it, k do.

**A threshold of 0, or one above the electorate's size, is refused when the ballot opens.** Repeated thresholds
are refused too. Their order in the command does not matter.

**Quorum intersection is not Cartel's to check.** Flexible Paxos's phase-2 quorums are rightly smaller than a
majority. Whether thresholds intersect is the protocol's argument.

The thresholds are the first of the guards that keep the quorum's shape open
([P1-ADR-TheQuorumShapeIsLeftOpen](P1-ADR-TheQuorumShapeIsLeftOpen.md)).

## Enforcement

- The matrix's rows for thresholds out of range or repeated.
- Vectors at k−1 and k.
- The exhaustive matrix over small electorates with every threshold.

## Related

[P1-ADR-TheElectorateTravelsWithTheBallot](P1-ADR-TheElectorateTravelsWithTheBallot.md) — the number travels with
the electorate it counts.
