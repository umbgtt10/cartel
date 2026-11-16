# P1-ADR-EquivocationIsReportedNotJudged

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

A faulty member may vote two different choices in one ballot. IBFT does two different things with it today:
- it detects it for proposals only, rejecting both blocks and marking the sender for the height;
- it counts a voter's prepares or commits for two block hashes in one round for both, and nobody notices.

Safety holds anyway, because two of IBFT's quorums share at least f+1 voters. Evidence is lost, though.

What to do about an equivocating member differs by protocol: ignore it, exclude it, slash its stake.

## Decision

When a member votes a choice different from the one already counted for it in a ballot:
- the first vote stays counted;
- the second is not counted;
- the command is refused with an equivocation, carrying the ballot, the member, the choice counted and the choice
  offered.

Cartel takes no further action.

## Forcing constraints / Evidence

**Counting both weakens every quorum.** One member would count twice, once on each side of a split.

**Discarding both is a policy.** That is IBFT's consequence for proposals; another protocol may choose otherwise.

**Excluding the member is a judgment,** and state that outlives the ballot.

## Rejected alternatives

**Counting both, as IBFT does today.** Rejected for the reason above.

**Discarding both.** Rejected: it is a policy.

**Marking the member in the books.** Rejected: it is a judgment, and the protocol's.

## Consequences

- The same choice voted again is refused as already counted, not as an equivocation.
- Every further different choice from the member is refused as an equivocation again.
- In Raft every choice is empty, so equivocation never arises.
- What follows is the protocol's: IBFT can mark the member for the height as it does for proposals.

## Enforcement

- The matrix's rows for a repeated vote and for an equivocation.
- A property: no member is counted for two choices in one ballot.
- The cluster simulation with equivocating members.

## Related

[P0-ADR-CartelCountsNeverDecides](P0-ADR-CartelCountsNeverDecides.md).
