// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::ballot_key::BallotKey;
use crate::vote::Vote;

// An open ballot (§4): its key, its electorate and its thresholds as the
// command that opened it gave them, and the votes counted in it, in the order
// they were cast.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenBallot {
    pub key: BallotKey,
    pub electorate: Vec<u64>,
    pub thresholds: Vec<u64>,
    pub votes: Vec<Vote>,
}
