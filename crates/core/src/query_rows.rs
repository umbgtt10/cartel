// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::open_ballot::OpenBallot;
use crate::output::Output;
use crate::rejection::Rejection;
use alloc::collections::BTreeMap;

// What the queries answer from the books: one ballot's record, or the open
// ballots in canonical order.
pub struct QueryRows<'a> {
    ballots: &'a BTreeMap<Ballot, OpenBallot>,
}

impl<'a> QueryRows<'a> {
    pub fn new(ballots: &'a BTreeMap<Ballot, OpenBallot>) -> Self {
        Self { ballots }
    }

    pub fn ballot(&self, ballot: Ballot) -> Output {
        self.ballots
            .get(&ballot)
            .map_or(Output::CommandRejected(Rejection::BallotNotOpen), |open| {
                Output::BallotShown(open.record(ballot))
            })
    }

    pub fn ballots(&self) -> Output {
        Output::BallotsListed {
            ballots: self.ballots.keys().copied().collect(),
        }
    }
}
