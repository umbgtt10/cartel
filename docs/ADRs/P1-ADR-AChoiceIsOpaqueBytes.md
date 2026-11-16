# P1-ADR-AChoiceIsOpaqueBytes

- **Status:** Accepted
- **Date:** 2026-11-16
- **Priority:** P1 (derived)

## Context

What a vote is for differs by protocol:
- **Raft:** nothing. A vote is a yes for the candidate who opened the ballot.
- **IBFT:** a block hash.
- **Tendermint:** a block id, or nil for no block.
- **Paxos:** a value.
- **Others:** a digest of a size their own hash algorithm sets.

## Decision

A choice is a sequence of bytes, possibly empty, compared byte by byte. Cartel never interprets it. Its length is
limited by the longest-choice bound, given at construction or left out
([P1-ADR-BoundsGivenAtConstruction](P1-ADR-BoundsGivenAtConstruction.md)).

## Forcing constraints / Evidence

**Technology-independent means no hash algorithm.** A fixed 32 bytes is SHA-256's or Keccak's size, and not every
protocol's.

**A Byzantine member supplies the bytes.** Without a bound, a choice is an allocation an attacker sizes. An
integration that tolerates Byzantine faults gives the bound.

**Choices of different lengths share a ballot.** Tendermint's nil prevote, naturally the empty choice, stands beside
32-byte block ids in one ballot.

## Rejected alternatives

**A fixed limit in the specification,** 64 bytes for instance. Rejected: it could never be raised. The vector that
proves a longer choice is refused would have to change, and released vectors are never edited.

**An exact width set at construction.** Rejected: it refuses Tendermint's nil vote beside a block id.

**No limit, relying on the transport's message size.** Rejected: an in-memory or test deployment has no transport,
and the memory ceiling would be unprovable.

**`[u8; 32]`.** Rejected: one hash family's size.

**A u64.** Rejected: too small for a digest, and a protocol mapping a digest into it would risk collisions.

**A type parameter.** Rejected: no wire form for the vectors, and the observer could not log it.

## Consequences

- In the specification's JSON, a choice is a string of lowercase hexadecimal digits; the empty choice is the empty
  string.
- A ballot without choices, Raft's, is a ballot whose every vote has the empty choice.
- IBFT gives a longest choice of 32 bytes. Raft can give 0, or none.
- The empty choice is valid beside any other.

## Enforcement

The schema's choice type; the matrix's row for a choice too long; vectors with empty choices, choices of mixed
lengths in one ballot, and choices at the bound.

## Related

[P0-ADR-ConsensusIsKeptOnBallots](P0-ADR-ConsensusIsKeptOnBallots.md).
