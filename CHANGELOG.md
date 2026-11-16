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
