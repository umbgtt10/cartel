// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::ballot_key::BallotKey;
use crate::choice::Choice;
use crate::command::Command;
use crate::instance_state::InstanceState;
use crate::open_ballot::OpenBallot;
use crate::point::Point;

// The books looked up: a ballot by its key, the choice a member is a voter
// of, a choice's voters.
pub struct Lookup;

impl Lookup {
    #[must_use]
    pub fn ballot(state: &InstanceState, key: Option<BallotKey>) -> Option<&OpenBallot> {
        state.ballots.iter().find(|ballot| Some(ballot.key) == key)
    }

    #[must_use]
    pub fn choice(ballot: &OpenBallot, member: u64) -> Option<&Choice> {
        ballot
            .votes
            .iter()
            .find(|vote| vote.member == member)
            .map(|vote| &vote.choice)
    }

    // The open ballot a command names, where an earlier row of the table has
    // already found it open.
    // A ballot is below a point when its question and round come before the
    // point's, whatever its phase (§4).
    #[must_use]
    pub fn is_below(key: BallotKey, point: Point) -> bool {
        (key.question, key.round) < (point.question, point.round)
    }

    #[must_use]
    pub fn named<'a>(state: &'a InstanceState, command: &Command) -> &'a OpenBallot {
        Self::ballot(state, command.ballot).expect("the open ballot the command names")
    }

    #[must_use]
    pub fn voters(ballot: &OpenBallot, choice: &Choice) -> Vec<u64> {
        let mut voters: Vec<u64> = ballot
            .votes
            .iter()
            .filter(|vote| vote.choice == *choice)
            .map(|vote| vote.member)
            .collect();
        voters.sort_unstable();
        voters
    }
}
