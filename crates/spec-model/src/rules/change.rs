// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::command::Command;
use crate::instance_state::InstanceState;
use crate::open_ballot::OpenBallot;
use crate::vote::Vote;

// What a row does to the instance, as its prose states it. A refusal and a
// query change nothing; the receipt's number is the model's to move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    Nothing,
    BallotOpened,
    VoteCast,
}

impl Change {
    #[must_use]
    pub fn applied(self, command: &Command, state: &InstanceState) -> InstanceState {
        let mut after = state.clone();
        match self {
            Change::Nothing => {}
            Change::BallotOpened => after.ballots.push(OpenBallot {
                key: command.ballot.expect("a ballot"),
                electorate: command.electorate.clone().expect("an electorate"),
                thresholds: command.thresholds.clone().expect("thresholds"),
                votes: Vec::new(),
            }),
            Change::VoteCast => after
                .ballots
                .iter_mut()
                .filter(|ballot| Some(ballot.key) == command.ballot)
                .for_each(|ballot| {
                    ballot.votes.push(Vote {
                        member: command.member.expect("a member"),
                        choice: command.choice.clone().expect("a choice"),
                    });
                }),
        }
        after
    }
}
