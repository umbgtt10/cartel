// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::ballot_record::BallotRecord;
use crate::checkpoint::Checkpoint;
use crate::rejection::Rejection;
use alloc::vec::Vec;

// What happened: past participle, as every output is. An output never repeats
// what its command carried; the command is beside it in the log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Output {
    BallotOpened,
    VoteCounted {
        count: u64,
    },
    QuorumReached {
        count: u64,
        thresholds: Vec<u64>,
        voters: Vec<u64>,
    },
    BallotsClosed {
        ballots: Vec<Ballot>,
    },
    WatermarkMeasured {
        position: u64,
    },
    ClaimExamined {
        counted: Vec<u64>,
        outsiders: Vec<u64>,
        repeated: Vec<u64>,
        reached: bool,
    },
    BallotShown(BallotRecord),
    BallotsListed {
        ballots: Vec<Ballot>,
    },
    BooksShown(Checkpoint),
    CommandRejected(Rejection),
}
