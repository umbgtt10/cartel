// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::choice::Choice;
use alloc::vec::Vec;

// What a caller asks of an instance: imperative, as every command is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    OpenBallot {
        ballot: Ballot,
        electorate: Vec<u64>,
        thresholds: Vec<u64>,
    },
    CastVote {
        ballot: Ballot,
        member: u64,
        choice: Choice,
    },
    ShowBallot {
        ballot: Ballot,
    },
    ListBallots,
    ShowBooks,
}
