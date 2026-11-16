# Changelog

The crate's versions. The specification in [`spec/`](spec/SPEC.md) is versioned on its own; each release names the
version it conforms to. The first release is 0.2.0: 0.1.0 on crates.io is the name's placeholder.

## [Unreleased]

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
