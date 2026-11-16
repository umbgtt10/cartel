# P1-ADR-AClaimIsExaminedAsFacts

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

A protocol is shown quorums it did not count:
- IBFT checks a commit certificate a recovering node imports;
- IBFT checks the view-change proofs a NewView forwards.

Once the protocol has verified the signatures, what remains is counting: are the signers distinct, are they
members, and are there enough of them?

IBFT's two checks treat an outsider differently. The commit certificate does not count an outsider's signature and
can still pass; a NewView with an unknown sender is rejected whole.

## Decision

Cartel examines a claim, a set of voters presented as a quorum, against an electorate and a threshold. It answers
with facts:
- the members counted;
- the outsiders;
- the voters repeated;
- whether the members counted reach the threshold.

Examining changes nothing in the books.

## Forcing constraints / Evidence

**A verdict would fix one policy.** Whether an outsider rejects the whole message is the protocol's choice, and
IBFT already makes it two ways.

**A claim need not match an open ballot.** A recovering node examines certificates for heights whose ballots it
never opened, or has closed.

**The protocol counting it itself would be a second copy of the counting.**

## Rejected alternatives

**A yes or no.** Rejected: it fixes one outsider policy.

**Examining against an open ballot's electorate.** Rejected: the ballot may be closed or never opened.

**The protocol counting.** Rejected: it duplicates Cartel's counting.

## Consequences

- Only distinct members count toward the threshold. Outsiders and repeats are listed, in ascending order.
- IBFT keeps its policies on top: a repeated signer rejects a commit certificate, an outsider rejects a NewView.
- Like measuring a watermark, examining is a pure question through the one entry point, and it is logged.

## Enforcement

- The matrix's rows.
- A property: examining changes no ballot.

## Related

[P1-ADR-NoEvidenceInTheBooks](P1-ADR-NoEvidenceInTheBooks.md).
