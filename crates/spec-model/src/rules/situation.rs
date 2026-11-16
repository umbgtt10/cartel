// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::command::Command;
use crate::instance_state::InstanceState;
use crate::rules::ballot_situation::BallotSituation;
use crate::rules::books_situation::BooksSituation;
use crate::rules::bound_situation::BoundSituation;
use crate::rules::command_situation::CommandSituation;

// When a row applies, as its prose states it: always, or by what the command
// carries, by a bound the instance was given, by whether its ballot is open,
// or by the open ballot it names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Situation {
    Always,
    Command(CommandSituation),
    Bound(BoundSituation),
    Books(BooksSituation),
    Ballot(BallotSituation),
}

impl Situation {
    #[must_use]
    pub fn holds(self, command: &Command, state: &InstanceState) -> bool {
        match self {
            Situation::Always => true,
            Situation::Command(situation) => situation.holds(command),
            Situation::Bound(situation) => situation.holds(command, state),
            Situation::Books(situation) => situation.holds(command, state),
            Situation::Ballot(situation) => situation.holds(command, state),
        }
    }
}
