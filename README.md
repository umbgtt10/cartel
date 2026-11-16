# cartel

A `no_std`, protocol-independent **consensus bookkeeper**.

**Slices 1 to 3 of six are built** (0.4.0, 2026-11-16): ballots, closing and the watermark -- an instance, created
fresh, with an observer told of every call; `OpenBallot`, `CastVote` and `CloseBallots`; `MeasureWatermark`; the
queries `ShowBallot`, `ListBallots` and `ShowBooks`, the checkpoint -- conformant to version 0.3.0 of the
specification in [`spec/`](spec/SPEC.md). Version 0.4.0 is on [crates.io](https://crates.io/crates/cartel).

The design is twenty-three ADRs in [`docs/ADRs/`](docs/ADRs/README.md), the vocabulary and the commands in
[`docs/DESIGN.md`](docs/DESIGN.md), the order of building in [`docs/IMPLEMENTATION-PLAN.md`](docs/IMPLEMENTATION-PLAN.md):
restore, bounds and claims come next.

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

Any consensus protocol, crash- or Byzantine-tolerant: the protocol translates at one integration point, and Cartel answers
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
