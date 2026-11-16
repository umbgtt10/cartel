// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

#![no_std]
#![forbid(unsafe_code)]

//! A `no_std`, protocol-independent consensus bookkeeper.
//!
//! **Slices 1 to 3, ballots, closing and the watermark:** an instance,
//! created fresh, with an observer told of every call; `OpenBallot`,
//! `CastVote` and `CloseBallots`; `MeasureWatermark`; the queries
//! `ShowBallot`, `ListBallots` and `ShowBooks`, the checkpoint -- conformant to
//! version 0.3.0 of the specification in the repository's `spec/`.
//!
//! # The watermark
//!
//! Given each member's position -- how far its log is matched, which round
//! it has acknowledged -- and a threshold, cartel answers the highest position
//! at least that many members have reached. It keeps no position: the
//! protocol owns them and hands them in each time, so the books are never
//! kept twice.
//!
//! # Closing
//!
//! The protocol closes what it is done with: every ballot before a question
//! and a round, at once. The books remember the point they closed to, which
//! only moves forward, so a late vote for a closed round is refused as
//! closed, not as unknown -- and a protocol that closes as it goes keeps its
//! books as small as the ballots it still counts.
//!
//! # What it does
//!
//! Cartel keeps the books on votes and decides nothing. A ballot is a
//! question, a round and a phase; its electorate and its thresholds are
//! handed in when it opens; a choice is opaque bytes. Cartel counts who voted
//! for what, and says when a choice's count reaches a threshold, naming the
//! quorum's voters. It checks no signature, computes no threshold, casts no
//! vote of its own, times nothing and stores nothing durably: those are the
//! protocol's, whatever the protocol is.
//!
//! # Sibling to `faction` and `beggar`
//!
//! Cartel is one of four bookkeepers a consensus protocol orchestrates, none
//! aware of the others: [`faction`](https://crates.io/crates/faction) keeps
//! membership, [`beggar`](https://crates.io/crates/beggar) recovery, and
//! cartel consensus. Like beggar it is a registry, not a state machine: every
//! output is a function of the command and the record of the ballot it
//! addresses.
//!
//! # Observer
//!
//! Every call is reported to the [`Observer`](observer::Observer) the instance
//! is constructed with: the command and its receipt, once, by the time
//! `process` returns. It answers nothing, so cartel's answers do not depend on
//! it. [`NoOpObserver`](no_op_observer::NoOpObserver) is provided for the
//! "just drive it" case.

extern crate alloc;

pub mod ballot;
pub mod ballot_record;
pub mod books;
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
pub mod open_ballot;
pub mod origin;
pub mod output;
pub mod receipt;
pub mod rejection;
pub mod rows;
pub mod votes;
