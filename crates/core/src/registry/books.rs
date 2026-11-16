// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::checkpoint::Checkpoint;
use crate::choice::Choice;
use crate::close_point::ClosePoint;
use crate::instance_id::InstanceId;
use crate::output::Output;
use crate::registry::open_ballot::OpenBallot;
use crate::rejection::Rejection;
use crate::rows::closing_rows::ClosingRows;
use crate::rows::opening_rows::OpeningRows;
use crate::rows::query_rows::QueryRows;
use alloc::collections::BTreeMap;
use core::mem::replace;

// The books: the open ballots in canonical order, the close point, and the
// calls counted. A command meets the rows before it changes anything, and
// every change is made inside a call already counted.
pub struct Books {
    instance: InstanceId,
    close_point: Option<ClosePoint>,
    ballots: BTreeMap<Ballot, OpenBallot>,
    next_receipt: u64,
}

impl Books {
    pub fn new(instance: InstanceId) -> Self {
        Self {
            instance,
            close_point: None,
            ballots: BTreeMap::new(),
            next_receipt: 1,
        }
    }

    // A vote met by the rows in the matrix's order -- the close point, the
    // ballot, then those of the ballot itself -- and counted if none refuses it.
    pub fn cast(&mut self, ballot: Ballot, member: u64, choice: Choice) -> Output {
        if ClosingRows::new(self.close_point).covers(ballot) {
            return Output::CommandRejected(Rejection::BallotClosed);
        }
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
            close_point: self.close_point,
            ballots: self
                .ballots
                .iter()
                .map(|(ballot, open)| open.record(*ballot))
                .collect(),
            next_receipt: self.next_receipt,
        }
    }

    // A close: refused at or behind the close point; otherwise every ballot
    // below the new point closed and named, and the point moved to it. The
    // ballots below it are exactly those before its question and round's
    // first phase, so the map splits there.
    pub fn close(&mut self, before: ClosePoint) -> Output {
        if let Some(rejection) = ClosingRows::new(self.close_point).refusal(before) {
            return Output::CommandRejected(rejection);
        }
        let kept = self.ballots.split_off(&Ballot {
            question: before.question,
            round: before.round,
            phase: 0,
        });
        let closed = replace(&mut self.ballots, kept);
        self.close_point = Some(before);
        Output::BallotsClosed {
            ballots: closed.into_keys().collect(),
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
        let rows = OpeningRows::new(&self.ballots, ClosingRows::new(self.close_point));
        if let Some(rejection) = rows.refusal(ballot, electorate, thresholds) {
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
        QueryRows::new(&self.ballots, ClosingRows::new(self.close_point))
    }
}
