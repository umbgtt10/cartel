# P1-ADR-EveryCallIsObserved

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

The log is the record ([P1-ADR-TheLogIsTheRecord](P1-ADR-TheLogIsTheRecord.md)). With the receipt only returned,
the log holds every line only if every call site in every caller remembers to write it.

IBFT's consensus today reports to its observer from inside each handler, with a count and a quorum size it
computes there. Raft's consensus port reports nothing of its own.

## Decision

Cartel is constructed with an `Observer`, through its one constructor, and reports every call to it exactly once,
by the time `process` returns. It reports the command and its receipt, the line the log holds.

- The receipt is still returned.
- The observer is told, and answers nothing.

## Forcing constraints / Evidence

**Reporting at the one exit makes it total by construction.** One call where the receipt is made reports every
command, a query and a refusal included. A command added later is reported without anyone remembering to.

**One method.** Cartel has one exit and one form of answer, so its observer has one method, as beggar's does.

**Answering nothing keeps Cartel a counter.** An observer that returned something would be a dependency Cartel
decides with ([P0-ADR-CartelCountsNeverDecides](P0-ADR-CartelCountsNeverDecides.md)).

**Told by reference.** The observer is lent the command and the receipt. Nothing is copied for an observer that
discards it.

## Rejected alternatives

**The receipt only returned.** Rejected: the record then rests on every call site.

**A method per kind of output.** Rejected: it would be a second matrix to keep in step with the first.

**Observing the state.** Rejected: the state is derivable.

## Consequences

The observer is boxed, `Box<dyn Observer>`. `NoOpObserver` is provided for a caller with no use for one.

A protocol's own observer is fed from what Cartel reports, not recounted beside it. That covers IBFT's
`PrepareVoteAccepted` and `CommitQuorumReached`.

The dashboard's view of a node's consensus books is a `ShowBooks` answer, logged like any other.

The conformance suite runs each vector twice:
- with an observer that does nothing, checking the receipts;
- with one that records, checking it was told exactly what was answered.

## Enforcement

`process` calls the observer once, after the output is decided and before the receipt is returned. The coverage
gate over the conformance tier alone reaches the call and `NoOpObserver`.

## Related

beggar's ADR of the same name; faction's `P2-ADR-TotalObservability`.
