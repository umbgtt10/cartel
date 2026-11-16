// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::command::Command;
use crate::instance_state::InstanceState;
use crate::lookup::Lookup;

// A situation read from the open ballot a vote names (§6), where an earlier row
// has already found it open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallotSituation {
    NotAMember,
    AlreadyCounted,
    Equivocates,
    ReachesThreshold,
}

impl BallotSituation {
    #[must_use]
    pub fn holds(self, command: &Command, state: &InstanceState) -> bool {
        let ballot = Lookup::named(state, command);
        let member = command.member.expect("a member");
        let choice = command.choice.as_ref().expect("a choice");
        let counted = Lookup::choice(ballot, member);
        match self {
            BallotSituation::NotAMember => !ballot.electorate.contains(&member),
            BallotSituation::AlreadyCounted => counted == Some(choice),
            BallotSituation::Equivocates => counted.is_some_and(|counted| counted != choice),
            BallotSituation::ReachesThreshold => {
                let count = Lookup::voters(ballot, choice).len() + 1;
                ballot
                    .thresholds
                    .iter()
                    .any(|threshold| usize::try_from(*threshold).is_ok_and(|at| at == count))
            }
        }
    }
}
