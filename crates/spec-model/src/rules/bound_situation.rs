// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::command::Command;
use crate::instance_state::InstanceState;

// A situation read from a bound the instance was given (§2, §5, §6): an
// opening's electorate naming more members than its bound, a vote's choice
// longer than its bound, or as many ballots open as their bound. A bound left
// out never holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundSituation {
    ElectorateTooLarge,
    ChoiceTooLong,
    BooksFull,
}

impl BoundSituation {
    #[must_use]
    pub fn holds(self, command: &Command, state: &InstanceState) -> bool {
        let bounds = state.bounds;
        match self {
            BoundSituation::ElectorateTooLarge => {
                let members = command.electorate.as_ref().expect("an electorate").len();
                Self::exceeds(members, bounds.electorate)
            }
            BoundSituation::ChoiceTooLong => {
                let bytes = command.choice.as_ref().expect("a choice").bytes.len();
                Self::exceeds(bytes, bounds.choice)
            }
            BoundSituation::BooksFull => bounds.open_ballots.is_some_and(|bound| {
                u64::try_from(state.ballots.len()).is_ok_and(|open| open == bound)
            }),
        }
    }

    fn exceeds(size: usize, bound: Option<u64>) -> bool {
        bound.is_some_and(|bound| u64::try_from(size).map_or(true, |size| size > bound))
    }
}
