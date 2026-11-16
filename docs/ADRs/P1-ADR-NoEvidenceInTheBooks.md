# P1-ADR-NoEvidenceInTheBooks

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

IBFT keeps each prepare's and commit's signature beside the vote, so that it can build the prepared and commit
certificates from the quorum's voters. Other protocols carry evidence in other forms:

| protocol | evidence |
|---|---|
| Raft | none |
| PBFT, IBFT | one signature per voter |
| Tendermint, HotStuff with BLS | one aggregate signature and a bitmap |
| threshold schemes | shares combined by interpolation |
| post-quantum schemes | kilobytes per signature |

## Decision

Cartel keeps voters, never evidence: no signature, proof or certificate. When a choice reaches a threshold, Cartel
names the quorum's voters in ascending order. From them the protocol assembles whatever certificate its scheme
needs, and it verifies the certificates it is shown.

## Forcing constraints / Evidence

**Carrying evidence fits only one family.** Per-voter signatures could ride along as opaque bytes. An aggregate or
a threshold signature is not the concatenation of the votes' evidence: building it is cryptography
([P0-ADR-CartelCountsNeverDecides](P0-ADR-CartelCountsNeverDecides.md)).

**Evidence is memory a bounded bookkeeper cannot afford.** Post-quantum signatures would make the books kilobytes
per vote on a microcontroller.

**The voters are enough.** A list of signatures, a bitmap for an aggregate and share indices for a threshold scheme
are all built from who voted.

## Rejected alternatives

**Opaque evidence bytes per vote.** Rejected for the reasons above. Cartel would also hand back bytes it cannot
vouch for.

**A handle per vote, pointing into the protocol's store.** Rejected: the ballot and the member already identify a
vote.

## Consequences

- The protocol keeps its evidence keyed by ballot and member, and lets it go when it closes the ballots.
- A vote is cast only after the protocol has verified it.
- A certificate presented to the protocol is counted as a claim
  ([P1-ADR-AClaimIsExaminedAsFacts](P1-ADR-AClaimIsExaminedAsFacts.md)) after its signatures are verified.

## Enforcement

The schema has no evidence field; the crate's types carry none.

## Related

[P1-ADR-AClaimIsExaminedAsFacts](P1-ADR-AClaimIsExaminedAsFacts.md).
