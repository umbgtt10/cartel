// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::output::Output;
use crate::rejection::Rejection;
use crate::rows::repeats::Repeats;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

// A claim, answered from what it carries and nothing the books keep: refused
// if its electorate is empty or names a member twice, or its threshold is out
// of range; otherwise the facts -- the members among the voters, the voters
// outside the electorate, the voters named more than once, each once and in
// ascending order, and whether the members counted reach the threshold.
pub struct ClaimRows<'a> {
    electorate: &'a [u64],
    threshold: u64,
    voters: &'a [u64],
}

impl<'a> ClaimRows<'a> {
    pub fn new(electorate: &'a [u64], threshold: u64, voters: &'a [u64]) -> Self {
        Self {
            electorate,
            threshold,
            voters,
        }
    }

    pub fn answer(&self) -> Output {
        match self.refusal() {
            Some(rejection) => Output::CommandRejected(rejection),
            None => self.facts(),
        }
    }

    fn facts(&self) -> Output {
        let distinct: BTreeSet<u64> = self.voters.iter().copied().collect();
        let (counted, outsiders): (Vec<u64>, Vec<u64>) = distinct
            .into_iter()
            .partition(|voter| self.electorate.contains(voter));
        Output::ClaimExamined {
            reached: counted.len() as u64 >= self.threshold,
            counted,
            outsiders,
            repeated: Repeats::all(self.voters).into_iter().collect(),
        }
    }

    fn refusal(&self) -> Option<Rejection> {
        if self.electorate.is_empty() {
            return Some(Rejection::EmptyElectorate);
        }
        if let Some(member) = Repeats::lowest(self.electorate) {
            return Some(Rejection::RepeatedMember { member });
        }
        (!(1..=self.electorate.len() as u64).contains(&self.threshold)).then_some(
            Rejection::ThresholdOutOfRange {
                threshold: self.threshold,
            },
        )
    }
}
