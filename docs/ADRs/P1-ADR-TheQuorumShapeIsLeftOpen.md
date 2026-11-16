# P1-ADR-TheQuorumShapeIsLeftOpen

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

Three forms of quorum will be needed, and none from the start:

| form | what it is | who needs it |
|---|---|---|
| weighted votes | each member carries a weight, and a threshold is counted in weight | stake-based protocols, such as Tendermint's voting power |
| joint consensus | a ballot of several parts, each an electorate with its own threshold, reached when every part is | Raft's joint configurations, as etcd's |
| federated quorums | each voter brings its own quorum set, nested thresholds, and a quorum is the fixed point the voters' sets allow | Stellar's |

All three are data, not code: numbers, possibly nested. So vectors and replay survive them. What could shut the
door is the shape chosen now.

## Decision

None of the three is built until a protocol asks for it. Until then, four guards keep the door open:

1. **The quorum is its own type,** not a bare number in the command. Its first form is a set of flat thresholds
   over an electorate. Weights, parts and nested sets become new forms of it.
2. **The specification's JSON only grows.** The flat form's encoding never changes, and released vectors are never
   removed or edited. A log recorded on one version replays on every later one.
3. **The specification's wording is fixed:** "the highest position at least a threshold of members have reached",
   never "the k-th highest". The first stays true for weights and parts.
4. **Positions are given per member,** never as a bare list
   ([P1-ADR-TheWatermarkIsMeasuredNotKept](P1-ADR-TheWatermarkIsMeasuredNotKept.md)).

## Forcing constraints / Evidence

**What each form adds, later:**

| form | adds | the watermark becomes |
|---|---|---|
| weights | a weight per member; a refusal when the total overflows | add weights from the highest position down until the threshold is reached |
| joint consensus | a ballot of parts | the least of the parts' watermarks |
| federated quorums | voters' own quorum sets, the fixed point, and whether a set touches every slice of this node | defined over the quorum found |

Each is a new form of the quorum type, a new form in the JSON, and new vectors, with no change to the old.

**The answers do not change.** "This threshold reached, by these voters" is still the right answer under all
three, so a protocol still builds its certificate from the voters.

## Rejected alternatives

**Building one or more now.** Rejected: nothing asks for them.

**Marking federated quorums out of scope.** Rejected: their quorum sets are nested thresholds, data like the rest.

**An injected quorum rule.** Rejected: code cannot be a vector or replayed.

## Consequences

Order, when protocols ask: weights, then joint consensus, then federated quorums.

The specification lists the three in a "not yet" section, so a reader knows they are expected.

## Enforcement

- `spec/released.json` lists every released vector by id and digest. The conformance tier fails if one is missing
  or changed.
- The quorum type in the schema.
- The specification's wording.

## Related

[P1-ADR-TheProtocolSuppliesTheThresholds](P1-ADR-TheProtocolSuppliesTheThresholds.md),
[P1-ADR-TheElectorateTravelsWithTheBallot](P1-ADR-TheElectorateTravelsWithTheBallot.md).
