// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

#![no_std]
#![forbid(unsafe_code)]

//! **A `no_std`, zero-`unsafe`, protocol-independent consensus bookkeeper.**
//!
//! Cartel keeps the books on votes and decides nothing. A ballot is a question,
//! a round and a phase; its electorate and its thresholds are handed in when it
//! opens; a choice is opaque bytes. Cartel counts who voted for what, and says
//! when a choice's count reaches a threshold, naming the quorum's voters. It
//! checks no signature, computes no threshold, casts no vote of its own, times
//! nothing and stores nothing durably: those are the protocol's.
//!
//! **Slices 1 to 5 are built** -- conformant to version 0.5.0 of the
//! specification in the repository's `spec/`.
//!
//! # What it does
//!
//! `OpenBallot`, `CastVote` and `CloseBallots`; `MeasureWatermark` and
//! `ExamineClaim`; the queries `ShowBallot`, `ListBallots` and `ShowBooks`, the
//! checkpoint.
//!
//! * **Counting** -- a member is a voter of at most one choice; a second choice is an equivocation, reported and never counted.
//! * **Closing** -- the protocol closes every ballot before a question and a round; the point only moves forward, so a late vote is refused as closed, not as unknown.
//! * **The watermark** -- the highest position at least a threshold of members have reached, from positions the protocol hands in and cartel never keeps.
//! * **Claims** -- a quorum the protocol did not count, examined against an electorate and a threshold and answered with facts: the members counted, the outsiders, the repeats, whether they reach it.
//! * **Bounds** -- the most ballots open, the most members an electorate may name, the most bytes a choice may have; a command that would exceed one is refused, so the books never hold more.
//!
//! # Why cartel
//!
//! * **Deterministic** -- every output is a function of the command and the record of the ballot it names
//! * **Specified** -- a normative specification with conformance vectors; this crate is one implementation of it
//! * **Replayable** -- every receipt is numbered by cartel, so a log rebuilds the books and a gap proves a lost line
//! * **Zero unsafe** -- `#![forbid(unsafe_code)]`
//! * **`no_std + alloc`** -- given bounds, its memory has a ceiling whatever faulty members send
//!
//! # Example
//!
//! ```rust
//! use cartel::ballot::Ballot;
//! use cartel::bounds::Bounds;
//! use cartel::cartel::Cartel;
//! use cartel::choice::Choice;
//! use cartel::command::Command;
//! use cartel::fresh::Fresh;
//! use cartel::instance_id::InstanceId;
//! use cartel::no_op_observer::NoOpObserver;
//! use cartel::origin::Origin;
//! use cartel::output::Output;
//! use cartel::rejection::Rejection;
//!
//! let mut cartel = Cartel::new(
//!     Origin::Fresh(Fresh {
//!         instance: InstanceId { node: 1, incarnation: 1 },
//!         bounds: Bounds::default(),
//!     }),
//!     Box::new(NoOpObserver),
//! );
//!
//! // A ballot of four members, reached at three votes for one choice.
//! let ballot = Ballot { question: 7, round: 0, phase: 0 };
//! let _ = cartel.process(Command::OpenBallot { ballot, electorate: vec![1, 2, 3, 4], thresholds: vec![3] });
//!
//! let proposal = Choice { bytes: vec![0xab; 32] };
//! for member in [1, 2] {
//!     let _ = cartel.process(Command::CastVote { ballot, member, choice: proposal.clone() });
//! }
//!
//! // The third vote reaches the threshold, and the receipt names the quorum.
//! let receipt = cartel.process(Command::CastVote { ballot, member: 3, choice: proposal.clone() });
//! assert_eq!(
//!     receipt.output,
//!     Output::QuorumReached { count: 3, thresholds: vec![3], voters: vec![1, 2, 3] }
//! );
//!
//! // A member voting a second choice is reported, never counted.
//! let receipt = cartel.process(Command::CastVote { ballot, member: 1, choice: Choice { bytes: vec![0xcd; 32] } });
//! assert_eq!(receipt.output, Output::CommandRejected(Rejection::Equivocation { counted: proposal }));
//!
//! // A certificate's verified signers, examined as a claim: facts, never a verdict.
//! let receipt = cartel.process(Command::ExamineClaim {
//!     electorate: vec![1, 2, 3, 4],
//!     threshold: 3,
//!     voters: vec![2, 9, 3, 3, 1],
//! });
//! assert_eq!(
//!     receipt.output,
//!     Output::ClaimExamined { counted: vec![1, 2, 3], outsiders: vec![9], repeated: vec![3], reached: true }
//! );
//! ```
//!
//! # Observer
//!
//! Every call is reported to the [`Observer`](observer::Observer) the instance
//! is constructed with: the command and its receipt, once, by the time
//! `process` returns. It answers nothing, so cartel's answers do not depend on
//! it. [`NoOpObserver`](no_op_observer::NoOpObserver) is provided for the "just
//! drive it" case.
//!
//! # Siblings
//!
//! Cartel is one of four bookkeepers a consensus protocol orchestrates, none
//! aware of the others: [`faction`](https://crates.io/crates/faction) keeps
//! membership, [`beggar`](https://crates.io/crates/beggar) recovery, `notary`
//! credentials (not yet built), and [`cartel`](https://crates.io/crates/cartel)
//! consensus. All are `no_std`, protocol-independent and specified, each with one
//! entry point and an observer.
//! Like beggar, cartel is a registry, not a state machine: every output is a
//! function of the command and the record of the ballot it addresses.
//!
//! # Further reading
//!
//! * [README](https://github.com/umbgtt10/cartel#readme) -- the overview: how it works, validation, project status
//! * [Specification](https://github.com/umbgtt10/cartel/blob/main/spec/SPEC.md) -- normative: this crate is one implementation of it
//! * [Design](https://github.com/umbgtt10/cartel/blob/main/docs/DESIGN.md) -- the design
//! * [ADRs](https://github.com/umbgtt10/cartel/tree/main/docs/ADRs) -- the decisions, one per file

extern crate alloc;

pub mod ballot;
pub mod ballot_record;
pub mod bounds;
pub mod cartel;
pub mod checkpoint;
pub mod choice;
pub mod close_point;
pub mod command;
pub mod fresh;
pub mod instance_id;
pub mod member_position;
pub mod no_op_observer;
pub mod observer;
pub mod origin;
pub mod output;
pub mod receipt;
pub mod registry;
pub mod rejection;
pub mod rows;
pub mod votes;

#[cfg(doctest)]
pub mod readme_doctests;
