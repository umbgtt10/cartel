# Architecture Decision Records

Each ADR documents one load-bearing property of `cartel` — succinct, self-contained, citable on its own. Priority
marks the dependency tier, not importance:

- **P0** — foundational axiom; Cartel's identity rests on it.
- **P1** — derived or shape decision; follows from a P0.
- **P2** — structural; real, but below the core.

The ADRs say *why*; [`../DESIGN.md`](../DESIGN.md) says *how*: the vocabulary, the commands and the open
questions. [`../INVENTORY.md`](../INVENTORY.md) is what Raft and IBFT keep today, and
[`../IMPLEMENTATION-PLAN.md`](../IMPLEMENTATION-PLAN.md) the order of building.

## Index

| ADR | Status | Property |
|---|---|---|
| [P0-ADR-ConsensusIsKeptOnBallots](P0-ADR-ConsensusIsKeptOnBallots.md) | Accepted | Raft or IBFT, crash- or Byzantine-tolerant: Cartel knows ballots, members, opaque choices and numbers, and nothing else. |
| [P0-ADR-ARegistryNotAStateMachine](P0-ADR-ARegistryNotAStateMachine.md) | Accepted | Every output is a function of the command and the addressed ballot's record; no stage label does any work. |
| [P0-ADR-CartelCountsNeverDecides](P0-ADR-CartelCountsNeverDecides.md) | Accepted | No signature, threshold, own vote, clock, storage or protocol rule; one injected dependency, an observer that answers nothing. |
| [P0-ADR-SpecificationIsNormativeCrateIsReference](P0-ADR-SpecificationIsNormativeCrateIsReference.md) | Accepted | Specified first and by hand in `spec/` — schema, decision table, prose, vectors; the crate is one implementation. |
| [P0-ADR-NoStdZeroUnsafe](P0-ADR-NoStdZeroUnsafe.md) | Accepted | `#![no_std]` with `alloc`, `#![forbid(unsafe_code)]`, BTree collections only. |
| [P1-ADR-OneEntryPointOneOutput](P1-ADR-OneEntryPointOneOutput.md) | Accepted | One `process(command)`, one output per command on a numbered receipt; a counted vote answers `VoteCounted`, or `QuorumReached` with the thresholds and voters. |
| [P1-ADR-TheLogIsTheRecord](P1-ADR-TheLogIsTheRecord.md) | Accepted | Every call logged as `(number, command, output)`; state rebuilt by replay; the checkpoint a query; a restart restores into a new incarnation; forgetting votes is safe. |
| [P1-ADR-EveryCallIsObserved](P1-ADR-EveryCallIsObserved.md) | Accepted | Every call reported once to an observer given at construction, which answers nothing. |
| [P1-ADR-BoundsGivenAtConstruction](P1-ADR-BoundsGivenAtConstruction.md) | Accepted | Most open ballots, largest electorate, longest choice — each optional; past one given, refused. They protect memory, not liveness. |
| [P1-ADR-IndependentModelOfTheSpecification](P1-ADR-IndependentModelOfTheSpecification.md) | Accepted | `cartel-spec-model`, a second implementation independent of the crate; vectors and random sequences checked against it. |
| [P1-ADR-NoLineTheSpecificationDoesNotAskFor](P1-ADR-NoLineTheSpecificationDoesNotAskFor.md) | Accepted | 100% coverage from the first function, under every tier and under the vectors alone. |
| [P1-ADR-ABallotIsAQuestionARoundAndAPhase](P1-ADR-ABallotIsAQuestionARoundAndAPhase.md) | Accepted | A ballot is three u64, ordered question, round, phase; the phase a number Cartel never interprets; one close point fits protocols that decide in order. |
| [P1-ADR-AChoiceIsOpaqueBytes](P1-ADR-AChoiceIsOpaqueBytes.md) | Accepted | Bytes, possibly empty, compared byte by byte, limited by the longest-choice bound. |
| [P1-ADR-TheProtocolSuppliesTheThresholds](P1-ADR-TheProtocolSuppliesTheThresholds.md) | Accepted | One or more plain numbers per ballot, from the protocol; no formula, no named rule; each reported once when reached. |
| [P1-ADR-TheElectorateTravelsWithTheBallot](P1-ADR-TheElectorateTravelsWithTheBallot.md) | Accepted | Each ballot is counted against the electorate it opened with; Cartel never learns of a membership change. |
| [P1-ADR-NoEvidenceInTheBooks](P1-ADR-NoEvidenceInTheBooks.md) | Accepted | Voters, never signatures; a quorum's voters named in order; certificates are the protocol's. |
| [P1-ADR-EquivocationIsReportedNotJudged](P1-ADR-EquivocationIsReportedNotJudged.md) | Accepted | The first choice stays counted, the second is refused with both reported; the consequence is the protocol's. |
| [P1-ADR-AClaimIsExaminedAsFacts](P1-ADR-AClaimIsExaminedAsFacts.md) | Accepted | Counted, outsiders, repeated, reached — facts, never a verdict; the books unchanged. |
| [P1-ADR-TheWatermarkIsMeasuredNotKept](P1-ADR-TheWatermarkIsMeasuredNotKept.md) | Accepted | The highest position at least a threshold of members have reached, from positions given per member; none kept. |
| [P1-ADR-ClosingIsExplicitAndRemembered](P1-ADR-ClosingIsExplicitAndRemembered.md) | Accepted | The protocol closes every ballot before a point; below it, refused as closed, not as unknown. |
| [P1-ADR-TheQuorumShapeIsLeftOpen](P1-ADR-TheQuorumShapeIsLeftOpen.md) | Accepted | Weights, joint consensus and federated quorums later; four guards keep the door open. |
| [P2-ADR-TestingLadder](P2-ADR-TestingLadder.md) | Accepted | Seven tiers — unit, conformance, exhaustive matrix, property, cluster simulation, replay, memory. |
| [P2-ADR-WorkspaceLaidOutByRole](P2-ADR-WorkspaceLaidOutByRole.md) | Accepted | `spec/`, `crates/`, `validation/`, `xtask/`, as beggar and faction. |

## Template

```markdown
# <Priority>-ADR-<Name>

- **Status:** Accepted | Proposed | Superseded by <ADR>
- **Date:** YYYY-MM-DD
- **Priority:** P0 (axiom) · P1 (derived) · P2 (structural)

## Context
The forces and tension this resolves.

## Decision
The choice, in one quotable sentence.

## Forcing constraints / Evidence
Why this was forced, not freely chosen — the real evidence. `N/A` if none.

## Rejected alternatives
What we did not do, and why.

## Consequences
What it commits us to; what it costs; obligations pushed onto consumers.

## Enforcement
The specific test, gate, or structural mechanism that keeps it true.

## Related
Links to other ADRs (same or sibling repos) and architecture docs.
```

Fields that do not apply are marked `N/A` rather than padded. Each ADR is a snapshot of the decision as it stands
today, not a changelog.
