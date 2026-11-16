// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::command::Command;
use crate::instance_state::InstanceState;
use crate::lookup::Lookup;

// A situation read from whether the ballot a command names is open (§5-§7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooksSituation {
    Open,
    NotOpen,
}

impl BooksSituation {
    #[must_use]
    pub fn holds(self, command: &Command, state: &InstanceState) -> bool {
        let open = Lookup::ballot(state, command.ballot).is_some();
        match self {
            BooksSituation::Open => open,
            BooksSituation::NotOpen => !open,
        }
    }
}
