# P1-ADR-TheElectorateTravelsWithTheBallot

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

Clusters grow and shrink. Raft changes its configuration one server at a time, through Faction. IBFT schedules
validator-set changes at heights every node knows. A ballot opened before a change may still be counted after it.

The four bookkeepers do not know one another, so Faction cannot tell Cartel that the cluster changed.

## Decision

Each ballot opens with its electorate and is counted against that electorate for as long as it is open. Cartel
never learns of a membership change.

## Forcing constraints / Evidence

**IBFT's `Resized` shows the alternative's cost.** An electorate held by the instance and updated on a change has to
clear every vote to stay safe.

**Raft does this already.** Its `Opened { ballot, electorate }` carries the configuration's voters.

**Ballots on either side of a change coexist.** IBFT's view changes for height h and its prepares for h+1 can each
be counted against the set in force at its own height.

**It is how the bookkeepers stay unaware of one another.** The protocol reads its configuration, Faction's answer
or its own log, and hands it to Cartel inside each ballot.

## Rejected alternatives

**An electorate held by the instance and updated on a change.** Rejected for the reason above.

**Cartel learning of changes from Faction.** Rejected: the bookkeepers do not know one another.

## Consequences

- An electorate is a set of distinct members. An empty one, or one naming a member twice, is refused.
- A vote from a member outside the ballot's electorate is refused.
- Each open ballot stores its electorate, within the bound on the largest electorate.
- Joint consensus, two electorates both reaching their quorums, is a later form of the ballot
  ([P1-ADR-TheQuorumShapeIsLeftOpen](P1-ADR-TheQuorumShapeIsLeftOpen.md)).

## Enforcement

- The matrix's rows for an empty electorate, a repeated member and a vote from a non-member.
- A property: a ballot's electorate never changes while it is open.

## Related

[P1-ADR-TheProtocolSuppliesTheThresholds](P1-ADR-TheProtocolSuppliesTheThresholds.md).
