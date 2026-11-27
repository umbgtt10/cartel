# cartel

**A `no_std`, zero-`unsafe`, protocol-independent consensus bookkeeper.**

Every consensus protocol counts votes. IBFT counts prepares, commits and view changes against more than two thirds
of its validators; crash-tolerant protocols count votes and match indices against a majority; PBFT, Tendermint and
Paxos count their own. Each writes the counting inline, tangled with the rules that give the counts their meaning,
so it is tested only through the protocol, and every protocol writes it again. The bugs it hides -- a member counted
twice, an outsider's vote let in, a quorum reported on votes a crash had lost -- surface under exactly the faults
consensus exists to survive.

`cartel` takes the counting out: a **formally specified, fully tested registry** that keeps the books on votes and
decides nothing.

**Slices 1 to 5 of five are built** (0.6.0, 2026-11-16): ballots, closing, the watermark, the bounds and claims,
conformant to version 0.5.0 of the specification in [`spec/`](spec/SPEC.md). Version 0.6.1 is on
[crates.io](https://crates.io/crates/cartel).

---

## What cartel does

`cartel` keeps the books on votes:

- who may vote in a ballot;
- who voted for which choice;
- when a threshold is reached, and by whom.

A ballot is a question, a round and a phase. Its electorate and its thresholds are handed in when it opens, so a
membership change never touches a ballot already open. A choice is opaque bytes. Beside the ballots, `cartel`
answers two pure questions the protocols ask of their own numbers: the highest position enough members have
reached, and whether a set of signers it never counted makes a quorum.

It does exactly this. Nothing more.

No signature checked. No threshold computed. No vote of its own, no clock, no storage, no opinion on what a reached
quorum means. The caller owns all of that. `cartel` owns the books.

---

## Why cartel?

- **Deterministic** — every output is a function of the command and the books it meets. The same calls always
  give the same receipts.
- **Specified** — a normative, language-independent specification: prose, a JSON Schema, a decision table and 109
  conformance vectors. The crate is one implementation; an independent model of the specification is another.
- **Verifiable** — every command in every situation is checked against the model over small electorates, and the
  invariants after every call over random sequences. No untested paths exist.
- **Embeddable** — `no_std + alloc`, zero `unsafe`. Given bounds, its memory has a ceiling whatever faulty members
  send, so it runs on a microcontroller as on a server.
- **Observable** — every call reaches a trait-based `Observer`, once, with its receipt: an accepted command, a
  refused one, a query.
- **Replayable** — every receipt is numbered by `cartel`, one more than the last. A log rebuilds the books receipt
  by receipt, and a gap in the numbers proves a line lost.
- **Protocol-independent** — its vocabulary is no protocol's: ballot, question, round, phase, electorate,
  threshold, choice. No term, height, view or certificate appears in its types.

---

## How it works

### A registry, not a state machine

`cartel` has no states. Every answer is a function of the command and the record of the ballot it names, so
ballots do not interfere: a vote in one changes nothing in another. That decides how it is tested: invariants and a
decision table, not transition coverage.

### Commands and outputs

Eight commands, one entry point, `process(command) -> receipt`:

| Command | Answered with | Meaning |
|---|---|---|
| `OpenBallot { ballot, electorate, thresholds }` | `BallotOpened` | Open a ballot over its electorate, reached at each threshold |
| `CastVote { ballot, member, choice }` | `VoteCounted { count }`, or `QuorumReached { count, thresholds, voters }` | Count a member's vote for a choice |
| `CloseBallots { before }` | `BallotsClosed { ballots }` | Close every ballot before a question and round |
| `MeasureWatermark { positions, threshold }` | `WatermarkMeasured { position }` | The highest position at least `threshold` members have reached |
| `ExamineClaim { electorate, threshold, voters }` | `ClaimExamined { counted, outsiders, repeated, reached }` | Examine a quorum the protocol did not count |
| `ShowBallot { ballot }` | `BallotShown` | One ballot's record |
| `ListBallots` | `BallotsListed` | The open ballots |
| `ShowBooks` | `BooksShown` | The checkpoint: the whole books |

Any command may instead be refused, `CommandRejected(rejection)`, with one of fifteen rejections saying why and
carrying what the log needs beside the command. A refusal changes nothing. `cartel` never panics and never returns
an opaque error.

### Counting

A member is a voter of at most one choice in a ballot. A vote reaching one of the ballot's thresholds answers
`QuorumReached`, naming the quorum's voters, once, for that choice. A member voting a second choice is an
**equivocation**: refused, the choice already counted named beside the one offered. What to do with an equivocator
is the protocol's.

### Closing

The protocol closes what it is done with: every ballot before a question and a round, at once. The books remember
the point they closed to, which only moves forward, so a late vote for a closed round is refused as closed, not as
unknown, and a protocol that closes as it goes keeps its books as small as the ballots it still counts.

### The watermark

Given each member's position -- how far its log is matched, which round it has acknowledged -- and a threshold,
`cartel` answers the highest position at least that many members have reached. It keeps no position: the protocol
owns them and hands them in each time, so the books are never kept twice.

### Claims

A quorum the protocol did not count -- a certificate's signers, once it has verified their signatures -- is
examined against an electorate and a threshold and answered with facts: the members counted, the outsiders, the
voters named twice, and whether the members counted reach the threshold. What an outsider or a repeat costs is the
protocol's rule.

### Bounds

An instance may be given three bounds: the most ballots open at once, the most members an electorate may name, the
most bytes a choice may have. A command that would exceed one is refused, naming it, so the books never hold more
than the bounds allow. A bound left out is no bound.

### Crash and restart

A restarted node creates a fresh instance under a new incarnation, with empty books. Forgetting is safe: given any
subset of the votes counted, the rules reach no threshold the whole set did not, so a crash can make an instance
miss a quorum and never report one it did not have.

---

## Quick start

```toml
[dependencies]
cartel = "0.6"
```

```rust
use cartel::ballot::Ballot;
use cartel::bounds::Bounds;
use cartel::cartel::Cartel;
use cartel::choice::Choice;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::instance_id::InstanceId;
use cartel::no_op_observer::NoOpObserver;
use cartel::origin::Origin;
use cartel::output::Output;
use cartel::rejection::Rejection;

let mut cartel = Cartel::new(
    Origin::Fresh(Fresh {
        instance: InstanceId { node: 1, incarnation: 1 },
        bounds: Bounds::default(),
    }),
    Box::new(NoOpObserver),
);

// A ballot of four members, reached at three votes for one choice.
let ballot = Ballot { question: 7, round: 0, phase: 0 };
let _ = cartel.process(Command::OpenBallot { ballot, electorate: vec![1, 2, 3, 4], thresholds: vec![3] });

let proposal = Choice { bytes: vec![0xab; 32] };
for member in [1, 2] {
    let _ = cartel.process(Command::CastVote { ballot, member, choice: proposal.clone() });
}

// The third vote reaches the threshold, and the receipt names the quorum.
let receipt = cartel.process(Command::CastVote { ballot, member: 3, choice: proposal.clone() });
assert_eq!(
    receipt.output,
    Output::QuorumReached { count: 3, thresholds: vec![3], voters: vec![1, 2, 3] }
);

// A member voting a second choice is reported, never counted.
let receipt = cartel.process(Command::CastVote { ballot, member: 1, choice: Choice { bytes: vec![0xcd; 32] } });
assert_eq!(receipt.output, Output::CommandRejected(Rejection::Equivocation { counted: proposal }));

// A certificate's verified signers, examined as a claim: facts, never a verdict.
let receipt = cartel.process(Command::ExamineClaim {
    electorate: vec![1, 2, 3, 4],
    threshold: 3,
    voters: vec![2, 9, 3, 3, 1],
});
assert_eq!(
    receipt.output,
    Output::ClaimExamined { counted: vec![1, 2, 3], outsiders: vec![9], repeated: vec![3], reached: true }
);
```

The caller owns the protocol. `cartel` owns the books.

---

## Validation

| Package | What it tests | Tests |
|---|---|---|
| [`spec/`](./spec/SPEC.md) | The normative specification: a decision table of 24 rows and 109 conformance vectors, each receipt written by hand | — |
| `crates/spec-model/` | The specification's independent model: its decision table read and interpreted, depending on nothing of the crate | 101 |
| `crates/core/` unit tests | Each type and each row in isolation, and the front page's and the README's examples | 67 |
| `validation/ladder/` | Conformance to the specification, of the model and then of the crate; every command in every situation over small electorates and bounds, against the model; properties over random command sequences, the crate against the model among them; logs replayed | 35 |
| `validation/core-validation/` | A seeded cluster simulation: honest, crashed and equivocating members, votes lost, duplicated and reordered, nodes restarting, and quorum intersection checked across them | 17 |
| `validation/memory/` | A counting allocator: the books holding what their votes need and nothing for refusals and queries, nothing after a drop, and no more than they hold full when flooded beyond their bounds | 12 |

---

## Project status

| Metric | Value |
|---|---|
| Core productive LOC | ~690 |
| Specification | [`spec/`](./spec/SPEC.md) 0.5.0 — its independent model and the crate answer all 109 vectors as written |
| Total tests | 353 |
| Code coverage (productive) | 100%, over every tier and over the conformance tier alone |
| Crappy functions (CRAP score) | 0 |
| Unsafe code | 0 — enforced by `#![forbid(unsafe_code)]` |
| `no_std` | Verified on `thumbv7em-none-eabihf` |
| Published | [crates.io](https://crates.io/crates/cartel) 0.6.1 |

---

## Design principles

- **Counts, never decides** — `cartel` says what the votes are. What they mean is the protocol's.
- **A registry, not a state machine** — every answer is a function of the command and the record it names.
- **The specification is normative** — the crate is a reference implementation; where they disagree, the
  specification is right.
- **One entry point, one output** — every call is a command, every command answered by one receipt.
- **Observer, not logger** — the `Observer` receives every call with its receipt. Wire it to anything.
- **The log is the record** — numbered receipts rebuild the books; the checkpoint is a query.
- **No evidence in the books** — signatures are verified by the protocol, before a vote reaches `cartel`.
- **The protocol supplies the thresholds** — a number per ballot, no formula inside.
- **The quorum's shape is left open** — weights, joint consensus and federated quorums can arrive as new forms,
  never as changed answers.
- **One struct per file**, and **no `&mut` parameters** — prefer return values over in-place mutation.

Each principle is recorded as an Architecture Decision Record — twenty-three, in [docs/ADRs/](./docs/ADRs/README.md).

---

## Non-goals

`cartel` deliberately does not:

- Verify signatures, or keep evidence of a vote
- Compute a threshold from a cluster's size
- Cast a vote of its own, or decide what a reached quorum means
- Judge an equivocator — it reports one
- Time rounds, or own a clock
- Store anything durably — a restart begins with fresh books
- Know any protocol — protocols build on top of `cartel`, through one integration point each

---

## Quality gates

```sh
just stage1   # format, clippy, the no_std check and the tests
just stage2   # house rules, duplication, CRAP, coverage, mirrored tests, file risk, dependencies
```

Both gates must pass before any commit lands. Stage 2's gates are
[`serge4rust`](https://github.com/umbgtt10/serge4rust)'s, the house's quality gates as a library; `xtask/src/main.rs`
holds only which of them run, in what order, and with what settings, and takes serge4rust from crates.io. They
drive these tools, each installed once from crates.io:

| Tool | Used by | Install |
|---|---|---|
| [`just`](https://crates.io/crates/just) | Runs the two stages | `cargo install just` |
| [`cargo-stern4rust`](https://crates.io/crates/cargo-stern4rust) | Stage 2 checks the house coding rules in every package | `cargo install cargo-stern4rust` |
| [`cargo-dry4rust`](https://crates.io/crates/cargo-dry4rust) | Stage 2 refuses duplicated code in `crates/` | `cargo install cargo-dry4rust` |
| [`cargo-crap4rust`](https://crates.io/crates/cargo-crap4rust) | Stage 2 scores CRAP (Change Risk Anti-Patterns) complexity over `crates/` | `cargo install cargo-crap4rust` |
| [`cargo-llvm-cov`](https://crates.io/crates/cargo-llvm-cov) | Stage 2 requires every line, region and function of `crates/` covered, by every tier and by conformance alone | `cargo install cargo-llvm-cov` |
| [`cargo-twin4rust`](https://crates.io/crates/cargo-twin4rust) | Stage 2 requires a mirrored test file for every source file | `cargo install cargo-twin4rust` |
| [`cargo-iceberg4rust`](https://crates.io/crates/cargo-iceberg4rust) | Stage 2 bounds each file's private implementation risk | `cargo install cargo-iceberg4rust` |
| [`cargo-deny`](https://crates.io/crates/cargo-deny) | Stage 2 checks advisories, licences and sources | `cargo install --locked cargo-deny` |

---

## Roadmap

`cartel` was built in five additive slices -- ballots, closing, the watermark, the bounds, claims -- each a release,
each answering every vector of the ones before it unchanged. All five are built. A first protocol's integration is
built, a plug-in of its own keeping its consensus books in every one of its deployments; IBFT's follows.

Nothing more is built ahead of need. When a protocol asks: restoring an instance from a checkpoint, withdrawing a
vote, closing one ballot at a time, an answer that a threshold can no longer be reached, and weights, joint
consensus and federated quorums as new forms of the quorum.

See [IMPLEMENTATION-PLAN.md](./docs/IMPLEMENTATION-PLAN.md) for the plan and [DESIGN.md](./docs/DESIGN.md) for the
design.

---

## Workspace

The workspace is laid out by role: [`spec/`](./spec/SPEC.md) holds the normative specification, `crates/` the
library, `validation/` everything that checks it, and `xtask/` the gates.

| Package | Role |
|---|---|
| `crates/core/` | The registry: `cartel`, the crate on crates.io |
| `crates/spec-model/` | The specification's independent model, `no_std` |
| `validation/ladder/` | Conformance, the exhaustive matrix, property and replay tiers |
| `validation/core-validation/` | The seeded cluster simulation |
| `validation/memory/` | The memory tier, a counting allocator in a binary of its own |
| `xtask/` | The gates, taken from [`serge4rust`](https://github.com/umbgtt10/serge4rust): `cargo xtask stage2` and the commit scores |

---

## Siblings

`cartel` is one of four bookkeepers a consensus protocol orchestrates, none aware of the others:

| Bookkeeper | Keeps |
|---|---|
| [`faction`](https://crates.io/crates/faction) | Membership: whether the cluster is ready, who joins and leaves, who still answers |
| [`beggar`](https://crates.io/crates/beggar) | Recovery: which chunks of a transfer have moved, at both ends |
| `notary` | Credentials — not yet built |
| [`cartel`](https://crates.io/crates/cartel) | Consensus: who voted for what, and when a threshold is reached |

All are `no_std`, protocol-independent and specified, each with one entry point and an observer; the protocol
translates for each at one integration point.

---

## License

MIT. See [LICENSE](./LICENSE).

---

## Links

- [SPEC.md](./spec/SPEC.md) — the normative specification: data model, decision table, prose and conformance vectors
- [DESIGN.md](./docs/DESIGN.md) — the design: the vocabulary, the commands, where `cartel` sits
- [ADRs](./docs/ADRs/README.md) — architecture decision records: the design rationale, one decision per file
- [IMPLEMENTATION-PLAN.md](./docs/IMPLEMENTATION-PLAN.md) — the slices, and what waits until a protocol asks
- [INVENTORY.md](./docs/INVENTORY.md) — every book the protocols in view keep, and whose each becomes
- [CHANGELOG.md](./CHANGELOG.md) — version history
