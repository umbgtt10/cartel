# P1-ADR-NoLineTheSpecificationDoesNotAskFor

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

Coverage is usually a target approached over time, because sockets, files, clocks and processes make some lines
hard to reach from a test. Cartel has none of them: every line is reachable from `process`. A shortfall in
coverage here is never a hard-to-test line; it is one of:
- untested behaviour;
- dead code;
- behaviour the specification does not ask for.

## Decision

Cartel has 100% coverage from its first function: no uncovered line, region or function. That holds under all the
test tiers together, and under the conformance vectors alone.

## Forcing constraints / Evidence

**Nothing is out of reach.** With nothing injected but an observer that answers nothing, and one entry point, there
is no line a test cannot drive.

**The vectors alone are the stronger measure.** A line the vectors never reach is behaviour the specification does
not state.

## Rejected alternatives

**A percentage threshold.** Rejected: a rounded percentage hides the one line that matters.

**Branch or MC/DC coverage.** Unavailable on a pinned stable compiler. Region coverage, which counts every arm of a
`match` and an `if`, is the finest stable measure.

**Excluding code from measurement.** Rejected: an exclusion is where an untested line would hide.

## Consequences

A type derives only the traits something uses.

A new line arrives with the vector that reaches it.

Coverage says every line ran, not that its effect is asserted. Mutation testing answers that second question; the
house's sentinel measures it on its own server.

## Enforcement

A coverage gate in stage 2, built with the first slice:
- `cargo llvm-cov` with `--fail-uncovered-lines 0`, `--fail-uncovered-regions 0` and
  `--fail-uncovered-functions 0`;
- run twice: over every tier, and over the conformance tests alone;
- measuring the specification's model too.

## Related

beggar's and faction's ADRs of the same name.
