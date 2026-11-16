# P1-ADR-TheWatermarkIsMeasuredNotKept

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

Raft asks the same question three times: what is the highest position a majority of its voters have reached?
- Its commit index asks it of match indices.
- ReadIndex asks it of acknowledged heartbeat rounds.
- Check-quorum asks whether that answer moved within an election timeout.

The positions are Raft's own, kept in its context and updated on its own events. Raft passes them as a bare list,
`held: Vec<u64>`, in which no position names its member.

## Decision

Cartel measures a watermark: given each member's position and a threshold, it answers the highest position at
least that many members have reached. It keeps no position.

The positions are given per member, each member once.

## Forcing constraints / Evidence

**The books are not kept twice.** The match indices are Raft's, and Raft updates them; a copy in Cartel would be a
second record to keep in step.

**Per member, or weights can never come.** A weighted watermark needs to know whose position is whose
([P1-ADR-TheQuorumShapeIsLeftOpen](P1-ADR-TheQuorumShapeIsLeftOpen.md)).

## Rejected alternatives

**Cartel keeping the positions**, told of each advance. Rejected: it duplicates the protocol's record.

**A bare list of positions, Raft's form.** Rejected: it shuts the door on weights.

**A majority built in.** Rejected: the protocol supplies the threshold.

## Consequences

**The answer for threshold t is the t-th highest position.** The specification states it as "the highest position
at least t members have reached", the wording that stays true for weights and for parts.

**Refused:**
- a member named twice;
- a threshold of 0;
- a threshold above the number of members given.

**A pure question.** Measuring touches no ballot, and is logged like any other.

**Raft's current-term rule stays Raft's.** It caps the watermark at what Raft may commit.

## Enforcement

- The matrix's rows.
- The exhaustive matrix over small member counts.
- A property: measuring changes no ballot.

## Related

[P1-ADR-TheProtocolSuppliesTheThresholds](P1-ADR-TheProtocolSuppliesTheThresholds.md).
