// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::open_ballot::OpenBallot;
use crate::output::Output;
use crate::rejection::Rejection;
use crate::rows::closing_rows::ClosingRows;
use alloc::collections::BTreeMap;

// What the queries answer from the books: one ballot's record -- refused as
// closed below the close point, as not open above it -- or the open ballots in
// canonical order.
pub struct QueryRows<'a> {
    ballots: &'a BTreeMap<Ballot, OpenBallot>,
    closing: ClosingRows,
}

impl<'a> QueryRows<'a> {
    pub fn new(ballots: &'a BTreeMap<Ballot, OpenBallot>, closing: ClosingRows) -> Self {
        Self { ballots, closing }
    }

    pub fn ballot(&self, ballot: Ballot) -> Output {
        if self.closing.covers(ballot) {
            return Output::CommandRejected(Rejection::BallotClosed);
        }
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
