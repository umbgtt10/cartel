# P1-ADR-OneEntryPointOneOutput

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

Some bookkeepers cannot be enumerated, replayed or written down as one function:
- one reached through several entrances;
- one that throws on bad input;
- one whose queries sit beside its commands.

Raft's consensus port has one entry point already. IBFT's tally has two, `step` and `read`, and the rest of IBFT's
consensus books are reached through a dozen methods of its volatile state.

## Decision

Cartel has one entry point, `process(command)`, which takes `&mut self` and returns exactly one output for every
command, a query and a refused command included, on a numbered receipt, and never an exception.

A counted vote answers one of two outputs:
- `VoteCounted`, when it reaches no new threshold;
- `QuorumReached`, when it does, naming the thresholds reached and the quorum's voters.

## Forcing constraints / Evidence

Totality is what the other promises rest on:
- The matrix is the specification only if every command in every situation has a row
  ([P0-ADR-SpecificationIsNormativeCrateIsReference](P0-ADR-SpecificationIsNormativeCrateIsReference.md)).
- The log is the record only if every call leaves a line
  ([P1-ADR-TheLogIsTheRecord](P1-ADR-TheLogIsTheRecord.md)).

A call that throws, returns nothing or returns a sequence of unknown length is one neither can hold as a row.

**A quorum reached is what a protocol waits for**, so it has a name of its own: in the log, to the observer, on the
dashboard. IBFT's observer names it so today, with `CommitQuorumReached`. Beggar answers the same way: a partial
that completes a transfer answers `RecoveryCompleted`, not `RecoveryAdvanced`.

**With flat thresholds one vote reaches at most one.** A counted vote raises its choice's count by exactly one, and
a ballot's thresholds are distinct. `QuorumReached` still lists the thresholds: under weights, one vote can reach
several ([P1-ADR-TheQuorumShapeIsLeftOpen](P1-ADR-TheQuorumShapeIsLeftOpen.md)), and the output's shape must not
change when they come.

## Rejected alternatives

**A sequence of outputs per command:** a vote counted, then a quorum reached. Rejected: the matrix would need rows
for sequences, and a replay would have to know where one command's outputs end.

**One output with a field,** `VoteCounted { reached }`, mostly empty. Rejected: every caller would check a field for
the one thing it waits for, and in the log a quorum would read like any other vote.

**Errors as a `Result`.** Rejected: a refused call drops out of the record, or the function stops being total.

**Queries as methods of their own**, as IBFT's tally `read` is. Rejected: they would escape the numbering and the
log, and the checkpoint is a query that has to be in the log.

**An output per kind of refusal.** Rejected: it widens the matrix for no gain.

## Consequences

**Outputs carry what happened.** The command is in the logged tuple, so the ballot, the member and the choice are
not repeated in the output.
- `VoteCounted` carries the choice's count.
- `QuorumReached` carries the count, the thresholds reached, and the choice's voters in ascending order.

**Refusals:**
- Every refusal is one output carrying a specific rejection, and what the log needs to tell a degenerate caller
  from a malicious member.
- An equivocation is such a rejection: it carries the choice counted, and the command beside it the choice
  offered.
- A refusal changes nothing.

**The matrix is checked in a fixed order:**
1. whether the command is well formed;
2. then the command's own sizes against the bounds: an electorate's members, a choice's bytes;
3. then the ballot;
4. then the member;
5. then the effect, which the bound on open ballots can refuse: only an opening that would add a ballot exceeds
   it, so one below the close point or already open is refused for that first.

The first row that applies is the output.

**Queries are commands.** So are one ballot's record, the open ballots and the whole books, and the two pure
questions, measuring a watermark and examining a claim. Each gets a receipt and is logged.

## Enforcement

- The matrix: one row for every command in every situation. Casting a vote has two rows that count, reaching no
  new threshold and reaching one.
- The conformance vectors.
- An exhaustive property test over small electorates.

## Related

[P0-ADR-ARegistryNotAStateMachine](P0-ADR-ARegistryNotAStateMachine.md) — why there is no set of admissible
commands to return beside the output.
