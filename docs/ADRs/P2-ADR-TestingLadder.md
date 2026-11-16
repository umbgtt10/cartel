# P2-ADR-TestingLadder

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P2 (structural)

## Context

One style of test catches one class of failure and misses the others.
- The vectors prove the specification's samples.
- A property test explores sequences nobody wrote down, but cannot enumerate every situation.
- Neither drives several nodes' books through a cluster where votes are lost, duplicated, reordered or forged.

## Decision

Cartel's correctness rests on a fixed ladder of seven tiers, each with an obligation of its own, all run by the
gates: unit, conformance, exhaustive matrix, property, cluster simulation, replay and memory.

## Forcing constraints / Evidence

Each tier is the only one that catches its class of failure:

| tier | proves | catches |
|---|---|---|
| 1. unit | each type alone, one test file per source file, the crate's and the specification's model's | a type wrong in isolation |
| 2. conformance | every vector against the crate and the model, valid against the schema, every row exercised | a departure from the specification |
| 3. exhaustive matrix | every command in every situation over small bounds — electorates up to 4, every threshold, two choices — against the model | a situation no vector names |
| 4. property | the invariants after every call: receipts one more each call, non-interference between ballots, no member counted twice, the crate against the model over random sequences; and forgetting is safe — any subset of a run's counted votes loses quorums and never gains one | a sequence nobody wrote down |
| 5. cluster simulation | several nodes' books fed by a seeded harness, with members that are honest, crashed or equivocating, and votes lost, duplicated and reordered; nodes that crash and restart fresh under a new incarnation, and from 0.4.0 from an older checkpoint; checked by cross-node invariants through every restart | what happens between nodes, and across a crash |
| 6. replay | logs replayed into a fresh instance: every receipt matches, a dropped line is detected; from 0.4.0, a checkpoint resynchronises and a restart restores into a new incarnation | a log that cannot rebuild the books |
| 7. memory | a counting allocator in a test target of its own: a positive control, the books holding what their votes need and nothing for refusals and queries, nothing left after a drop; from 0.5.0, the peak at the bounds | growth no other tier sees |

The cluster simulation's cross-node invariant is quorum intersection, checked against Cartel's counting:
- With f equivocating members and a threshold t where 2t > n + f, no two nodes see different choices reach t in
  the same ballot.
- With none and 2t > n, the same holds for crash-tolerant quorums.

The invariant holds through every restart, because a restarted node holds a subset of the votes. In the simulation,
an honest member's own vote is durable, as its protocol keeps it. A counting bug, such as one member counted twice,
breaks the invariant.

## Rejected alternatives

**The vectors alone.** Rejected: they are samples.

**A system tier.** Not applicable: Cartel has no transport. Its system tier is its consumers', Raft's and IBFT's
system tests once it is integrated.

## Consequences

A command lands in every tier that applies to it, in the slice that brings it.

Tier 3's bounds keep it exhaustive and fast. Tier 5 is seeded, so a failure replays.

## Enforcement

Stage 1 runs every tier:
- the unit tier in `crates/*/tests/`;
- the scenario tiers in `validation/ladder/`;
- the cluster simulation in `validation/core-validation/`, its cross-node invariants a type of their own, each
  check shown firing on a crafted violation;
- the memory tier in `validation/memory/`.

Stage 2's coverage gate measures every tier together, and the conformance tier alone.

## Related

beggar's and faction's ADRs of the same name.
