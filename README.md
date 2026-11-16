# cartel

A `no_std`, protocol-independent **consensus bookkeeper**.

**Under construction.** The design is settled: twenty-three ADRs in [`docs/ADRs/`](docs/ADRs/README.md), the
vocabulary and the commands in [`docs/DESIGN.md`](docs/DESIGN.md), the order of building in
[`docs/IMPLEMENTATION-PLAN.md`](docs/IMPLEMENTATION-PLAN.md). The first slice, ballots, is being built.

## What it does

**Cartel counts and never decides.** It keeps the books on votes:
- who may vote in a ballot;
- who voted for which choice;
- when a threshold is reached, and by whom.

A ballot is a question, a round and a phase. Its electorate and its thresholds are handed in when it opens, so a
membership change never touches a ballot already open. A choice is opaque bytes. A member voting two different
choices in one ballot is refused, the choice already counted named beside the one offered.

**What it leaves to the protocol that drives it:**
- verifying signatures;
- computing thresholds;
- casting its own vote;
- timing rounds;
- storing anything durably;
- deciding what a reached quorum means.

Raft or IBFT, crash- or Byzantine-tolerant: the protocol translates at one integration point, and Cartel answers
every call through one entry point, on a numbered receipt, reported to an observer.

## Siblings

Cartel is one of four bookkeepers a consensus protocol orchestrates, none aware of the others:

| bookkeeper | keeps |
|---|---|
| [faction](https://crates.io/crates/faction) | membership |
| [beggar](https://crates.io/crates/beggar) | recovery |
| notary | credentials |
| cartel | consensus |

## License

MIT.
