# cartel — design

**Status: design settled; no implementation.**
- The decisions are twenty-three ADRs in [`ADRs/`](ADRs/README.md), five P0, sixteen P1 and two P2, and say why.
  All are accepted.
- What Raft and IBFT keep today is [`INVENTORY.md`](INVENTORY.md).
- The specification will be `spec/`, at the repository root.
- The order of building, slice by slice, is [`IMPLEMENTATION-PLAN.md`](IMPLEMENTATION-PLAN.md).

Settled 2026-11-16.

## 1. What Cartel is

Cartel is the consensus bookkeeper: one of four that a consensus protocol orchestrates. The four do not know one
another, and none knows which protocol drives it:

| bookkeeper | keeps |
|---|---|
| faction | membership |
| beggar | recovery |
| notary | credentials |
| cartel | consensus |

**Cartel keeps the books on votes:**
- who may vote in a ballot;
- who voted for which choice;
- when a threshold is reached;
- the highest position a threshold of members has reached;
- whether a set of voters presented as a quorum reaches a threshold;
- who voted two different choices in one ballot.

**It never:**
- checks a signature;
- computes a threshold;
- casts a vote of its own accord;
- times anything;
- stores anything durably;
- applies a protocol's rule.

See [`P0-ADR-CartelCountsNeverDecides`](ADRs/P0-ADR-CartelCountsNeverDecides.md).

## 2. Properties

The requirements, each held by the ADRs that answer it:

| requirement | held by |
|---|---|
| technology-independent | `P0-ADR-CartelCountsNeverDecides`, `P1-ADR-AChoiceIsOpaqueBytes`, `P1-ADR-NoEvidenceInTheBooks` |
| protocol-independent | `P0-ADR-ConsensusIsKeptOnBallots`, `P1-ADR-TheProtocolSuppliesTheThresholds`, the vocabulary (§4) |
| fully observable | `P1-ADR-EveryCallIsObserved`, `P1-ADR-TheLogIsTheRecord` |
| `no_std` + `alloc` | `P0-ADR-NoStdZeroUnsafe`, `P1-ADR-BoundsGivenAtConstruction` |
| no unsafe code | `P0-ADR-NoStdZeroUnsafe` |
| one entry point | `P1-ADR-OneEntryPointOneOutput` |
| fully specified | `P0-ADR-SpecificationIsNormativeCrateIsReference`, `P1-ADR-IndependentModelOfTheSpecification` |
| tested as the other bookkeepers | `P1-ADR-NoLineTheSpecificationDoesNotAskFor`, `P2-ADR-TestingLadder`, `P2-ADR-WorkspaceLaidOutByRole` |
| test vectors | `P0-ADR-SpecificationIsNormativeCrateIsReference`, `P1-ADR-TheQuorumShapeIsLeftOpen` |
| replayable | `P1-ADR-TheLogIsTheRecord` |

## 3. A registry, not a state machine

Cartel's state is:
- the open ballots, keyed by question, round and phase;
- the close point;
- the bounds.

It has no stage. Every output follows from the command and the record of the ballot it addresses. A command for
one ballot leaves every other ballot as it was. The only couplings are the bound on open ballots and the close
point.

See [`P0-ADR-ARegistryNotAStateMachine`](ADRs/P0-ADR-ARegistryNotAStateMachine.md).

## 4. Vocabulary

The words come from the language of assemblies and elections, which reads the same whatever the protocol. Two
come from systems work (position, watermark) and one from Byzantine fault tolerance (equivocation).

Ballot, question and watermark were chosen over poll, matter and level on 2026-11-16.

### Who

| word | meaning | Raft | IBFT |
|---|---|---|---|
| **member** | one id in an electorate | a voter of the configuration | a validator |
| **electorate** | the members who may vote in a ballot, handed in when it opens | the configuration's voters | the validator set in force at the height |
| **voter** | a member who has voted in a ballot | | |

### What is voted on

| word | meaning | Raft | IBFT |
|---|---|---|---|
| **question** | what is being decided | the term | the height |
| **round** | one attempt at a question; an unsettled question goes to another round | 0, a new term is a new question | the round |
| **phase** | a stage of a round; a label Cartel never interprets | pre-vote, vote | prepare, commit, view change |
| **ballot** | one phase of one round of one question; what Cartel counts | a term's pre-vote or vote | the prepares at a height and round |
| **choice** | what a vote is for; opaque bytes, possibly empty | empty | the block hash; empty for a view change |
| **vote** | a member's choice in a ballot, *cast* | | |

### Counting

| word | meaning | Raft | IBFT |
|---|---|---|---|
| **tally** | the voters of each choice in a ballot | | |
| **threshold** | a number of votes for one choice; one or more per ballot, from the protocol | a majority | 2f+1; and f+1 on view changes |
| **quorum** | the voters of one choice once they reach a threshold; reported once per threshold | elected | prepared, committed, round joined, NewView due |
| **equivocation** | a member voting two different choices in one ballot; the first stays counted, the second is refused | cannot arise | a voter's two hashes in one round |

### Progress

| word | meaning | Raft | IBFT |
|---|---|---|---|
| **position** | how far a member has got; a number the protocol gives per member | match index; acknowledged heartbeat round | not used yet |
| **watermark** | the highest position at least a threshold of members have reached | the commit candidate; the confirmed read round; check-quorum | |

### Claims

| word | meaning | IBFT |
|---|---|---|
| **claim** | a set of voters presented as a quorum: a certificate's signers, once the protocol has verified the signatures; *examined* against an electorate and a threshold | the commit certificate; NewView's view-change proofs |

### Keeping

| word | meaning |
|---|---|
| **close** | stop counting, and forget, every ballot before a question and round |
| **close point** | the highest point closed to; below it, a ballot is refused as closed |
| **books** | everything Cartel holds: the open ballots, their electorates, thresholds and tallies, the close point |
| **checkpoint** | the books written out; an instance can be *restored* from one |
| **bounds** | the most open ballots, the largest electorate and the longest choice, fixed at construction |
| **instance**, **receipt**, **observer** | as in beggar |

### Words left out

| left out | why | Cartel says |
|---|---|---|
| term, height, slot, view, epoch | each is one protocol's | question, round |
| leader, candidate, proposer, validator | roles | member |
| prepare, commit, pre-vote, view change | phases' names | a phase number |
| block, log, entry, index | what is decided | choice, position |
| majority, supermajority, 2f+1 | rules | threshold, a number |
| certificate, signature, proof | evidence, the protocol's | claim |
| elected, prepared, committed, decided | what a quorum means to one protocol | quorum reached |
| instance, for what is decided | beggar's word for the object | question |
| value | Paxos's word, and it reads as any number | choice |

## 5. Commands and outputs — a draft

Commands are imperative and outputs past participle, as in beggar. The specification fixes the names, the fields
and the check order; this is the shape.

| command | output |
|---|---|
| `OpenBallot { ballot, electorate, thresholds }` | `BallotOpened` |
| `CastVote { ballot, member, choice }` | `VoteCounted { count }`, or `QuorumReached { count, thresholds, voters }` when the vote reaches a threshold |
| `CloseBallots { before }` | `BallotsClosed { ballots }` |
| `MeasureWatermark { positions, threshold }` | `WatermarkMeasured { position }` |
| `ExamineClaim { electorate, threshold, voters }` | `ClaimExamined { counted, outsiders, repeated, reached }` |
| `ShowBallot { ballot }` | `BallotShown` |
| `ListBallots` | `BallotsListed` |
| `ShowBooks` | `BooksShown`, the checkpoint |

Every refusal is one output carrying a specific rejection:

| command | rejections |
|---|---|
| opening a ballot | empty electorate, repeated member, threshold out of range, repeated threshold, electorate too large, below the close point, already open, books full |
| casting a vote | choice too long, below the close point, not open, not a member, already counted, an equivocation carrying the choice counted (the choice offered is the command's) |
| showing a ballot | below the close point, not open |
| closing | at or behind the close point, carrying it |
| measuring a watermark | repeated member, threshold out of range |

A ballot below the close point is refused as closed, not as not open: late is not unknown.

## 6. Where Cartel sits

The protocol orchestrates. Each protocol reaches Cartel through one integration point, the only place its words
meet Cartel's:

| protocol | integration point | status |
|---|---|---|
| Raft | a `raft-cartel` plug-in behind its `ConsensusBookkeeping` port, in place of `MajorityConsensusBookkeeping` in every deployment, with no change of behaviour: one instance per kind of ballot, progress a watermark | built 2026-11-16 |
| IBFT | its `VoteTally` port first, then the books still in its consensus code (inventory I1–I17) | after Raft |

## 7. The quorum's shape, left open

Weights, joint consensus and federated quorums will be needed; none from the start. Four guards keep the door open:
1. the quorum is its own type;
2. the JSON only grows, and released vectors are kept;
3. the wording is fixed;
4. positions are given per member.

See [`P1-ADR-TheQuorumShapeIsLeftOpen`](ADRs/P1-ADR-TheQuorumShapeIsLeftOpen.md).

## 8. Crash and restart

**A crash can make Cartel forget, never invent.** Safety across a crash never rests on Cartel's books. It rests on
the protocol's own durable promises. Losing the books costs liveness only: a quorum goes unnoticed, a timeout fires,
and a new term or round begins.

### Why forgetting is safe

1. **Counting only adds.** A vote is a fact another node sent. After a restart Cartel holds nothing, or an older
   checkpoint, which is a subset of the votes it had counted. This is a property of the specification: given any
   subset of the votes counted, Cartel reports no threshold reached that the whole set did not reach
   ([`P1-ADR-TheLogIsTheRecord`](ADRs/P1-ADR-TheLogIsTheRecord.md)).
   - **One exception:** a member that voted two choices may have its second counted once a restart forgot its
     first. Only a faulty member votes two choices, and point 3 holds against it.
2. **The danger is the node's own vote, and it is the protocol's.** What breaks safety across a crash is a node
   forgetting that it voted and voting again, differently. The node's own vote is durable protocol state
   ([`P0-ADR-CartelCountsNeverDecides`](ADRs/P0-ADR-CartelCountsNeverDecides.md)).
   - Raft writes its term and `votedFor` before it answers.
   - IBFT keeps its round, pending block and prepared certificate in its hard state, and restarts from it alone
     (D21): it begins with empty tallies and infers from that hard state whether it already sent its prepare.
3. **Across the cluster nothing changes.** Two quorums for different choices need, in their intersection, an honest
   member who voted both. Whatever any node's Cartel remembers, that cannot happen while honest members keep their
   own votes durable.
4. **What is lost is evidence.** An equivocation reported before the crash is forgotten, unless the protocol kept
   the report. IBFT's malicious senders are volatile: a restart forgets them.

### How a protocol recovers

| step | what | whose |
|---|---|---|
| 1. its own promises | the hard state: Raft's term, vote and log; IBFT's round, lock and pending block | the protocol's storage |
| 2. what was decided while it was down | never recounted. Raft's leader sends the entries and its commit index. IBFT imports blocks with their commit certificates through recovery, and examines each certificate's verified signers as a claim, which needs no books, so a fresh instance does it. | the protocol, beggar, and Cartel's claims |
| 3. the question now open | the ballots it now counts, counted from there, on a fresh or restored instance | the protocol, with Cartel |

For step 3 the protocol chooses one of two ways:

- **Fresh.** Raft and IBFT both start fresh. A new instance, under a new incarnation, with empty books.
  - A Raft node restarts as a follower; a candidate that crashed loses that term's election. A new leader rebuilds
    its followers' positions from their replies, as Raft always does: they are volatile by design.
  - In IBFT, a round that cannot reach its quorum without the votes lost times out into a view change.
- **Restored.** No protocol in view does this today; IBFT did, until it gave its volatile snapshot up (D20, D21).
  The protocol stores the checkpoint, `ShowBooks`'s answer, in its own WAL, and restores it into a new incarnation.
  The argument is the same, since an older checkpoint holds a subset of the votes counted. What it gains is the
  round not lost. Cartel builds it when a protocol asks ([`IMPLEMENTATION-PLAN.md`](IMPLEMENTATION-PLAN.md) §4).

The restart sequence for a protocol:
1. Create the instance, fresh or restored.
2. Close to the point its hard state gives (IBFT: before its height and round). A fresh instance has no close
   point, and without this a late vote for an old round would be refused as not open rather than as closed.
3. Open the ballots it now counts.

The testing ladder holds this two ways ([`P2-ADR-TestingLadder`](ADRs/P2-ADR-TestingLadder.md)):
- **a property test:** any subset of a run's counted votes loses quorums and never gains one;
- **the cluster simulation:** nodes crash and restart fresh -- and from older checkpoints, once restoring is
  built -- and the cross-node invariant must hold through every restart.

## 9. Open questions

1. **IBFT's view changes for rounds ahead.** IBFT keeps only each sender's latest one. Does that stay IBFT's policy,
   with IBFT casting only what it keeps, or does Cartel learn to withdraw a vote?
2. **A proposal as a ballot of one.** IBFT detects a proposer's two blocks at one height and round (inventory I14).
   As a ballot whose electorate is the proposer and whose threshold is 1, Cartel would detect it as an
   equivocation. Is that a fit, or a stretch?
3. **A "lost" answer.** A ballot that can no longer reach a threshold, as etcd's Raft reports. Raft ignores denied
   votes today (inventory R10). Later, if a protocol asks.
