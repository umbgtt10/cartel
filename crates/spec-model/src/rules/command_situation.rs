// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::command::Command;
use crate::numbers::Numbers;

// A situation read from what an OpenBallot carries alone (§5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandSituation {
    EmptyElectorate,
    RepeatedMember,
    NoThresholds,
    ThresholdOutOfRange,
    RepeatedThreshold,
}

impl CommandSituation {
    #[must_use]
    pub fn holds(self, command: &Command) -> bool {
        let electorate = command.electorate.as_deref().expect("an electorate");
        let thresholds = command.thresholds.as_deref().expect("thresholds");
        match self {
            CommandSituation::EmptyElectorate => electorate.is_empty(),
            CommandSituation::RepeatedMember => Numbers::lowest_repeated(electorate).is_some(),
            CommandSituation::NoThresholds => thresholds.is_empty(),
            CommandSituation::ThresholdOutOfRange => {
                Numbers::lowest_out_of_range(thresholds, electorate.len()).is_some()
            }
            CommandSituation::RepeatedThreshold => Numbers::lowest_repeated(thresholds).is_some(),
        }
    }
}
