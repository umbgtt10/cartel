# P0-ADR-ARegistryNotAStateMachine

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P0 (axiom)

## Context

`faction`, a sibling bookkeeper, is a state machine: one subject, a node's life in the cluster, whose stage decides
what every command means. `beggar`, another, is a registry of independent transfers.

Consensus has states too: follower, candidate and leader; a new round, prepared and committed. Faction's shape is
therefore the obvious default for its bookkeeper.

## Decision

Cartel is a registry of ballots, not a state machine. Every output is a function of the command and the record of
the ballot it addresses; no stage label does any work.

## Forcing constraints / Evidence

**The consensus state machine is the protocol.** Raft's states are not IBFT's. A stage in Cartel would either copy
one protocol's graph, and stop being protocol-independent, or be a common denominator each protocol translates
into. Either way there would be two machines to keep in step, and their divergence is where bugs breed.

**Many ballots are open at once.** IBFT holds a round's prepares and commits, view changes for several rounds, and
some for rounds ahead of its own. One stage could express that only as a product of states.

**A ballot has no stages of its own.** Reaching a threshold changes nothing about what the next vote means: it is
counted, and may reach the next threshold, as IBFT's view changes reach f+1 and then 2f+1. What differs between
ballots is data — the tally and the thresholds reached — never a mode that admits or refuses a command.

**The test that settles it:** in a state machine, the same command with the same arguments yields different
results in different states. Here it differs only with the record of the ballot it addresses.

## Rejected alternatives

**A state machine, faction's shape.** Rejected for the reasons above.

**A small machine per ballot**, a state type per stage as faction has. Rejected: no command is admitted in one
stage and refused in another, so the types would be ceremony.

**One open ballot per phase, a new one superseding the last.** Rejected: that is Raft's shape. Closing is the
protocol's explicit command ([P1-ADR-ClosingIsExplicitAndRemembered](P1-ADR-ClosingIsExplicitAndRemembered.md)).

## Consequences

The matrix is command × situation → output, with no next-stage column, as beggar's is.

Correctness is invariants and properties, not transition coverage. One property is stated in the specification:
**non-interference**. A command addressed to one ballot leaves every other ballot as it was. There are exactly two
couplings, both explicit:
- **the bound on open ballots:** an open can be refused because others fill the books;
- **the close point:** a ballot below it is refused as closed.

Two commands address no ballot at all, measuring a watermark and examining a claim. They are pure questions,
answered through the same entry point.

The protocol keeps the one source of truth about where consensus stands.

## Enforcement

- The matrix: every row's output follows from the command and the addressed ballot's record, the bound and the
  close point.
- The conformance vectors check every row.
- A property test checks non-interference over random sequences.

## Related

beggar's ADR of the same name — the registry this one follows.
