// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::open_ballot::OpenBallot;
use crate::rejection::Rejection;
use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;

// The rows a ballot meets before it is opened, in the matrix's order: what the
// command carries -- its electorate, then its thresholds -- and then whether
// the ballot is open already.
pub struct OpeningRows<'a> {
    ballots: &'a BTreeMap<Ballot, OpenBallot>,
}

impl<'a> OpeningRows<'a> {
    pub fn new(ballots: &'a BTreeMap<Ballot, OpenBallot>) -> Self {
        Self { ballots }
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
        if let Some(member) = Self::first_repeat(electorate) {
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
        if let Some(threshold) = Self::first_repeat(thresholds) {
            return Some(Rejection::RepeatedThreshold { threshold });
        }
        self.ballots
            .contains_key(&ballot)
            .then_some(Rejection::BallotAlreadyOpen)
    }

    // The lowest number given more than once, or none.
    fn first_repeat(numbers: &[u64]) -> Option<u64> {
        let mut seen = BTreeSet::new();
        let mut repeated = BTreeSet::new();
        for number in numbers {
            if !seen.insert(*number) {
                repeated.insert(*number);
            }
        }
        repeated.first().copied()
    }
}
