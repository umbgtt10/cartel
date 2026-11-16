// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::registry::open_ballot::OpenBallot;
use crate::rejection::Rejection;
use crate::rows::bound_rows::BoundRows;
use crate::rows::closing_rows::ClosingRows;
use crate::rows::repeats::Repeats;
use alloc::collections::BTreeMap;

// The rows a ballot meets before it is opened, in the matrix's order: what the
// command carries -- its electorate, then its thresholds, then the
// electorate's size against its bound -- then whether the ballot is below the
// close point, whether it is open already, and whether the books are full.
pub struct OpeningRows<'a> {
    ballots: &'a BTreeMap<Ballot, OpenBallot>,
    closing: ClosingRows,
    bounding: BoundRows,
}

impl<'a> OpeningRows<'a> {
    pub fn new(
        ballots: &'a BTreeMap<Ballot, OpenBallot>,
        closing: ClosingRows,
        bounding: BoundRows,
    ) -> Self {
        Self {
            ballots,
            closing,
            bounding,
        }
    }

    // The first row that refuses the opening, or none: then it is opened.
    pub fn refusal(
        &self,
        ballot: Ballot,
        electorate: &[u64],
        thresholds: &[u64],
    ) -> Option<Rejection> {
        self.form(electorate, thresholds)
            .or_else(|| self.bounding.electorate(electorate))
            .or_else(|| self.place(ballot))
    }

    // What the command carries: its electorate, then its thresholds.
    fn form(&self, electorate: &[u64], thresholds: &[u64]) -> Option<Rejection> {
        if electorate.is_empty() {
            return Some(Rejection::EmptyElectorate);
        }
        if let Some(member) = Repeats::lowest(electorate) {
            return Some(Rejection::RepeatedMember { member });
        }
        if thresholds.is_empty() {
            return Some(Rejection::NoThresholds);
        }
        let size = electorate.len() as u64;
        if let Some(threshold) = thresholds
            .iter()
            .copied()
            .filter(|at| !(1..=size).contains(at))
            .min()
        {
            return Some(Rejection::ThresholdOutOfRange { threshold });
        }
        Repeats::lowest(thresholds).map(|threshold| Rejection::RepeatedThreshold { threshold })
    }

    // Where the ballot stands: below the close point, open already, or one
    // more than the books may hold.
    fn place(&self, ballot: Ballot) -> Option<Rejection> {
        if self.closing.covers(ballot) {
            return Some(Rejection::BallotClosed);
        }
        if self.ballots.contains_key(&ballot) {
            return Some(Rejection::BallotAlreadyOpen);
        }
        self.bounding.books(self.ballots.len())
    }
}
