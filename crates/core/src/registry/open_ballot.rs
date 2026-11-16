// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::ballot_record::BallotRecord;
use crate::choice::Choice;
use crate::output::Output;
use crate::rejection::Rejection;
use crate::votes::Votes;
use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::vec;

// One open ballot's books: who may vote, the thresholds, and each choice's
// voters. A member is a voter of at most one choice: the rows a vote meets,
// checked before it is counted, keep it so.
pub struct OpenBallot {
    electorate: BTreeSet<u64>,
    thresholds: BTreeSet<u64>,
    tally: BTreeMap<Choice, BTreeSet<u64>>,
}

impl OpenBallot {
    pub fn new(electorate: BTreeSet<u64>, thresholds: BTreeSet<u64>) -> Self {
        Self {
            electorate,
            thresholds,
            tally: BTreeMap::new(),
        }
    }

    // A vote counted: the choice's count with it, and the quorum when the
    // count is one of the thresholds. A vote raises the count by one, so it
    // reaches at most one.
    pub fn count(&mut self, member: u64, choice: Choice) -> Output {
        let voters = self.tally.entry(choice).or_default();
        voters.insert(member);
        let count = voters.len() as u64;
        if self.thresholds.contains(&count) {
            Output::QuorumReached {
                count,
                thresholds: vec![count],
                voters: voters.iter().copied().collect(),
            }
        } else {
            Output::VoteCounted { count }
        }
    }

    pub fn counted(&self, member: u64) -> Option<&Choice> {
        self.tally
            .iter()
            .find(|(_, voters)| voters.contains(&member))
            .map(|(choice, _)| choice)
    }

    pub fn is_member(&self, member: u64) -> bool {
        self.electorate.contains(&member)
    }

    pub fn record(&self, ballot: Ballot) -> BallotRecord {
        BallotRecord {
            ballot,
            electorate: self.electorate.iter().copied().collect(),
            thresholds: self.thresholds.iter().copied().collect(),
            tally: self
                .tally
                .iter()
                .map(|(choice, voters)| Votes {
                    choice: choice.clone(),
                    voters: voters.iter().copied().collect(),
                })
                .collect(),
        }
    }

    // The first row that refuses a vote in this ballot, in the matrix's order --
    // the member, then the choice it is already a voter of -- or none: then it
    // is counted.
    pub fn refusal(&self, member: u64, choice: &Choice) -> Option<Rejection> {
        if !self.is_member(member) {
            return Some(Rejection::NotAMember);
        }
        match self.counted(member) {
            Some(counted) if counted == choice => Some(Rejection::AlreadyCounted),
            Some(counted) => Some(Rejection::Equivocation {
                counted: counted.clone(),
            }),
            None => None,
        }
    }
}
