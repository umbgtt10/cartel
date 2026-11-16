// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::open_ballot::OpenBallot;
use crate::rejection::Rejection;
use crate::rows::closing_rows::ClosingRows;
use crate::rows::repeats::Repeats;
use alloc::collections::BTreeMap;

// The rows a ballot meets before it is opened, in the matrix's order: what the
// command carries -- its electorate, then its thresholds -- then whether the
// ballot is below the close point, and whether it is open already.
pub struct OpeningRows<'a> {
    ballots: &'a BTreeMap<Ballot, OpenBallot>,
    closing: ClosingRows,
}

impl<'a> OpeningRows<'a> {
    pub fn new(ballots: &'a BTreeMap<Ballot, OpenBallot>, closing: ClosingRows) -> Self {
        Self { ballots, closing }
    }

    // The first row that refuses the opening, or none: then it is opened.
    pub fn refusal(
        &self,
        ballot: Ballot,
        electorate: &[u64],
        thresholds: &[u64],
    ) -> Option<Rejection> {
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
        if let Some(threshold) = Repeats::lowest(thresholds) {
            return Some(Rejection::RepeatedThreshold { threshold });
        }
        if self.closing.covers(ballot) {
            return Some(Rejection::BallotClosed);
        }
        self.ballots
            .contains_key(&ballot)
            .then_some(Rejection::BallotAlreadyOpen)
    }
}
