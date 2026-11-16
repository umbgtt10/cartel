// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::command::Command;
use crate::instance_state::InstanceState;
use crate::lookup::Lookup;

// A situation read from the books as a whole (§5-§7, §10): whether the
// ballot a command names is open, or below the close point, and whether a
// close is at or behind it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooksSituation {
    Open,
    NotOpen,
    Closed,
    CloseBehind,
}

impl BooksSituation {
    #[must_use]
    pub fn holds(self, command: &Command, state: &InstanceState) -> bool {
        match self {
            BooksSituation::Open => Lookup::ballot(state, command.ballot).is_some(),
            BooksSituation::NotOpen => Lookup::ballot(state, command.ballot).is_none(),
            BooksSituation::Closed => {
                let key = command.ballot.expect("a ballot");
                state
                    .close_point
                    .is_some_and(|point| Lookup::is_below(key, point))
            }
            BooksSituation::CloseBehind => {
                let before = command.before.expect("a point");
                state.close_point.is_some_and(|point| before <= point)
            }
        }
    }
}
