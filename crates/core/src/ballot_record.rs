// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::votes::Votes;
use alloc::vec::Vec;

// An open ballot's record: its electorate and its thresholds in ascending
// order, and its tally -- each choice voted for, in the choices' order, with
// its voters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BallotRecord {
    pub ballot: Ballot,
    pub electorate: Vec<u64>,
    pub thresholds: Vec<u64>,
    pub tally: Vec<Votes>,
}
