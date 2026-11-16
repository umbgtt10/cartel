// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

//! # cartel-spec-model
//!
//! **An independent model of cartel's specification.**
//!
//! The specification in `spec/` is normative, and the `cartel` crate is one
//! implementation of it. This is a second one, written from the
//! specification's prose and decision table alone, and kept apart from the
//! crate: it depends on nothing of `cartel`, and the two meet only in the
//! specification's own encoding, in the ladder. Its vectors are checked
//! against it before the crate runs them, and the crate is checked against it
//! over command sequences no vector holds.
//!
//! Its shape is the specification's rule, not the crate's books: a command is
//! answered by the first row of the decision table that applies, in the
//! table's order. The model reads the rows, and owns only what the prose gives
//! each one -- the situation in which it applies and the change it makes --
//! what its output carries, and the checkpoint. It keeps a ballot's votes as
//! they came, and reads the tally from them when it is shown.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod answer;
pub mod ballot_key;
pub mod bounds;
pub mod call;
pub mod choice;
pub mod command;
pub mod figures;
pub mod instance_state;
pub mod lookup;
pub mod member_position;
pub mod model;
pub mod numbers;
pub mod open_ballot;
pub mod point;
pub mod rules;
pub mod spec_reader;
pub mod table;
pub mod value_reader;
pub mod view;
pub mod vocabulary;
pub mod vote;
