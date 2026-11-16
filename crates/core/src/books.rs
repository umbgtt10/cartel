// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::checkpoint::Checkpoint;
use crate::choice::Choice;
use crate::instance_id::InstanceId;
use crate::open_ballot::OpenBallot;
use crate::opening_rows::OpeningRows;
use crate::output::Output;
use crate::query_rows::QueryRows;
use crate::rejection::Rejection;
use alloc::collections::BTreeMap;

// The books: the open ballots in canonical order, and the calls counted. A
// command meets the rows before it changes anything, and every change is made
// inside a call already counted.
pub struct Books {
    instance: InstanceId,
    ballots: BTreeMap<Ballot, OpenBallot>,
    next_receipt: u64,
}

impl Books {
    pub fn new(instance: InstanceId) -> Self {
        Self {
            instance,
            ballots: BTreeMap::new(),
            next_receipt: 1,
        }
    }

    // A vote met by the rows in the matrix's order -- the ballot, then those
    // of the ballot itself -- and counted if none refuses it.
    pub fn cast(&mut self, ballot: Ballot, member: u64, choice: Choice) -> Output {
        let Some(open) = self.ballots.get_mut(&ballot) else {
            return Output::CommandRejected(Rejection::BallotNotOpen);
        };
        match open.refusal(member, &choice) {
            Some(rejection) => Output::CommandRejected(rejection),
            None => open.count(member, choice),
        }
    }

    // The books as they stand: what ShowBooks answers.
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            instance: self.instance,
            ballots: self
                .ballots
                .iter()
                .map(|(ballot, open)| open.record(*ballot))
                .collect(),
            next_receipt: self.next_receipt,
        }
    }

    // A call counted: its receipt's number, one more than the last.
    pub fn count_call(&mut self) -> u64 {
        let number = self.next_receipt;
        self.next_receipt = number + 1;
        number
    }

    // An opening met by the rows in the matrix's order, and the ballot opened
    // with an empty tally if none refuses it.
    pub fn open(&mut self, ballot: Ballot, electorate: &[u64], thresholds: &[u64]) -> Output {
        if let Some(rejection) =
            OpeningRows::new(&self.ballots).refusal(ballot, electorate, thresholds)
        {
            return Output::CommandRejected(rejection);
        }
        self.ballots.insert(
            ballot,
            OpenBallot::new(
                electorate.iter().copied().collect(),
                thresholds.iter().copied().collect(),
            ),
        );
        Output::BallotOpened
    }

    pub fn query_rows(&self) -> QueryRows<'_> {
        QueryRows::new(&self.ballots)
    }
}
