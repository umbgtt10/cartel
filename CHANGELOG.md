# Changelog

The crate's versions. The specification in [`spec/`](spec/SPEC.md) is versioned on its own; each release names the
version it conforms to. The first release is 0.2.0: 0.1.0 on crates.io is the name's placeholder.

## [Unreleased]

#### Added
- The watermark (slice 3, the specification's 0.3.0): `MeasureWatermark` takes each member's position and a threshold
  and answers `WatermarkMeasured`, the highest position at least that many members have reached. It keeps no
  position and touches no ballot. A member named twice, or a threshold of 0 or above the members given, is refused
  by the same rows an opening meets: `RepeatedMember` and `ThresholdOutOfRange`.
- The ladder: 14 vectors; a matrix tier for the measure; properties that a higher threshold never raises the
  watermark and a member further on never lowers it. A reversed sort planted in the measure is caught by four tests.

## [0.3.0] — 2026-11-16

### Published on crates.io

Slice 2, closing. Conforms to the specification's version 0.2.0; its 18 new vectors are released beside the 42 of
0.1.0, which it answers alike.

#### Added
- Closing (slice 2, the specification's 0.2.0): `CloseBallots` closes every ballot before a question and a round and
  names them; the books remember the close point, which only moves forward. A close at or behind it is refused with
  `CloseBehind`, naming it; an opening, a vote or a query for a ballot below it is refused with `BallotClosed`. The
  checkpoint holds the close point once a close has moved it, and nothing for it before, so every released vector
  answers alike.
- The ladder: 18 vectors, a matrix tier for closing, and the property tier's new invariants -- a close leaves every
  ballot at or above its point as it was, and no open ballot is ever below the close point. The memory tier: books
  closed as they go hold as much after a hundred ballots as after one.

#### Changed
- The rows are grouped under `rows/`: `cartel::rows::opening_rows`, `cartel::rows::query_rows` and the new
  `cartel::rows::closing_rows`. `src/` would otherwise hold more files than the house allows. `Checkpoint` gains
  `close_point`; `Command`, `Output` and `Rejection` gain the closing variants.

## [0.2.0] — 2026-11-16

### Published on crates.io

Slice 1, ballots. Conforms to the specification's version 0.1.0. The first version of the crate on
[crates.io](https://crates.io/crates/cartel/0.2.0); 0.1.0 there is the name's placeholder.

#### Added
- The workspace, laid out by role: the crate in `crates/core/`, the specification's model in `crates/spec-model/`,
  the packages that check them in `validation/`, the gates in `xtask/`, ported from beggar's.
- The specification, version 0.1.0, ballots: the prose, the schema, a decision table of 16 rows and 42 conformance
  vectors, every expected receipt written by hand. `released.json` lists the released vectors, none yet.
- The specification's independent model, `cartel-spec-model`, never published: every vector answered as written on
  the first run, each deciding by exactly the rows it names.
- The crate's first slice, ballots: `Cartel::new` from a fresh origin and an observer, and `process`, one entry point
  answering `OpenBallot`, `CastVote`, `ShowBallot`, `ListBallots` and `ShowBooks` on numbered receipts. Every vector
  answered and every call reported as answered. `NoOpObserver` for a caller with no use for one.
- The rest of the testing ladder: the exhaustive matrix over small electorates, every threshold and two choices; the
  property tier, crate against model, receipts numbered, ballots not interfering, no member counted twice, and any
  subset of the votes counted reaching no quorum the whole set did not; the replay tier, logs rebuilt and a lost line
  found; the cluster simulation, quorum intersection kept with equivocating members, lost, duplicated and reordered
  votes and fresh restarts; the memory tier, refusals and queries holding nothing and nothing left after a drop.
