# cartel — implementation plan

**Status: agreed 2026-11-16; nothing built.** The decisions are the ADRs in [`ADRs/`](ADRs/README.md); the
vocabulary and the commands are [`DESIGN.md`](DESIGN.md). This document says in what order Cartel is built, and the
rules that keep each step additive.

## 1. Additivity

Released vectors are never removed or edited
([`P1-ADR-TheQuorumShapeIsLeftOpen`](ADRs/P1-ADR-TheQuorumShapeIsLeftOpen.md)). A later slice therefore never
changes what an earlier one answers. Five rules hold every slice to that:

1. **A slice only adds:**
   - commands, outputs and rejections;
   - origins, construction parameters and checkpoint fields.

   It never changes one that exists.
2. **A slice adds rows to an existing command only for situations its own additions create.** For example, a ballot
   below the close point exists only once closing does.
3. **A field added later is optional, and its absence reproduces the earlier behaviour:** no close point, no bound.
4. **Every released vector passes unchanged on every later version.** `spec/released.json` lists them by id and
   digest, and the conformance tier fails on one missing or changed.
5. **A command lands complete,** every situation it has, in one slice. That is why choices, equivocation and
   several thresholds are in slice 1: added later, they would change what casting a vote answers.

In the crate, each slice adds a command's rows in a file of its own, as beggar's `opening_rows.rs` and
`partial_rows.rs`, so the dispatch gains an arm and nothing else is edited.

## 2. The slices

Each slice is a release: a specification version, a tag, a changelog entry. Publishing on crates.io is the owner's
call, never a step of the plan.

| slice | adds | needed by |
|---|---|---|
| **1. Ballots** | Everything listed below. | Raft's elections |
| **2. Closing** | `CloseBallots` and the close point. Opening or casting below it is refused as closed; a close at or behind it is answered, closing nothing. The close point is in the checkpoint, optional. | Raft, IBFT |
| **3. Watermark** | `MeasureWatermark`: positions per member, a threshold, the highest position at least that many members have reached. Refused: a repeated member, a threshold out of range. | Raft's commit, ReadIndex and check-quorum |
| *Raft integrates* | §3 | |
| **4. Restore** | An instance from a checkpoint; a restart under a new incarnation; the replay tier resynchronising from a checkpoint; restarts from older checkpoints in the cluster simulation. | a replay's resynchronisation; a protocol that keeps its books across a restart |
| **5. Bounds** | The three optional bounds and their refusals: books full, electorate too large, choice too long. The bounds are in the checkpoint, optional. The memory tier is measured at the bounds. | small hosts, Byzantine-tolerant protocols |
| **6. Claims** | `ExamineClaim`: the members counted, the outsiders, the repeats, whether the threshold is reached. | IBFT's certificate and NewView checks |
| *IBFT integrates* | §3 | |

**Slice 1, Ballots, holds:**
- A fresh instance (the instance id), the observer and receipts.
- `OpenBallot`: an electorate and one or more thresholds. Refused: already open, empty electorate, repeated member,
  threshold out of range, repeated threshold.
- `CastVote`: choices, the tally, and `VoteCounted`, or `QuorumReached` with the thresholds and voters. Refused:
  not open, not a member, already counted, an equivocation.
- The queries `ShowBallot`, `ListBallots` and `ShowBooks`, the checkpoint and every vector's final state.

**Slice 1 also lays the ground, as beggar's first slice did:**
- the workspace by role ([`P2-ADR-WorkspaceLaidOutByRole`](ADRs/P2-ADR-WorkspaceLaidOutByRole.md));
- the gates, `just stage1` and `just stage2` with the xtask;
- the pinned toolchain;
- the commit-score hook;
- MIT, as the other bookkeepers;
- the specification's conventions, instance, entry point, names and `released.json`;
- the specification's model;
- every tier of the ladder;
- the vocabulary test ([`P0-ADR-ConsensusIsKeptOnBallots`](ADRs/P0-ADR-ConsensusIsKeptOnBallots.md)).

It is the largest slice by far. The five after it are small.

## 3. Integration

**Raft, after slice 3 -- built 2026-11-16** (`etheram-raft`, branch `cartel-integration`). A `raft-cartel`
plug-in behind Raft's `ConsensusBookkeeping` port, over Cartel 0.4.0 from crates.io, keeps the consensus books in
every deployment there, with no change of behaviour. What was built differs from what this section first planned,
for reasons the port's contract and a property test gave:
- **One Cartel instance per kind of ballot.** Raft opens and supersedes a pre-vote and a vote apart, and its
  contract asks that opening one leave the other's count as it was. A close in one instance takes every phase below
  its point, so one instance with a phase per kind could not drop a kind's superseded attempts without the other's
  live ballot; two instances can.
- **A ballot's question is its opening's place in the kind's sequence,** not the term. Each opening is a ballot of
  its own, the one it supersedes closed before it, so a re-opening counts over and the books hold only the ballot
  open now. With the term as the question, a key lower than the last -- which the port allows, though Raft never
  sends one -- met a close point above it: the property test against the default found it.
- **The watermark,** from what each member holds, now named beside it (`HeldIndex`), at a majority's threshold.
- `MajorityConsensusBookkeeping` stays Raft's default in `node-infra`, and the consumer outside that repository
  that constructs it is untouched until its own step.

Raft's stages 1, 2, 5 and 6 and its soak stay green.

**IBFT, after slice 6.**
1. Its `VoteTally` port first.
2. Then the books still in its consensus code, row by row as [`INVENTORY.md`](INVENTORY.md) marks them:
   - view changes and their two thresholds;
   - closing at a new round and at a commit;
   - equivocation;
   - certificates examined as claims.

The bounds come from its protocol limits. Its stages run on hardware as well.

## 4. Later, when a protocol asks

None is built ahead of need. Each arrives as a new command or a new quorum form, never a change to an existing
answer:
- withdrawing a vote, if IBFT's rule for view changes ahead moves into Cartel;
- closing one ballot at a time, for protocols that decide out of order;
- a "can no longer reach" answer;
- weights, then joint consensus, then federated quorums.

## 5. Each slice's routine

1. **The specification first.** The prose, the matrix's rows, the schema, the vectors, and `released.json` at the
   release.
2. **The specification's model,** answering the new vectors.
3. **The crate, test first.** Each test is seen failing before its code exists.
4. **Every tier the slice touches:** unit, conformance, exhaustive matrix, property, cluster simulation, replay,
   memory.
5. **Stages 1 and 2 green,** 100% coverage under every tier and under the vectors alone.
6. **The release:** the version, the changelog, the tag.

Mutation testing is the house's sentinel, run on its own server against what is pushed; its survivors are answered
in a round of their own.
