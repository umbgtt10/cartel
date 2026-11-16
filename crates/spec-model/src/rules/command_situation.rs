// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::command::Command;
use crate::figures::Figures;
use crate::numbers::Numbers;

// A situation read from what a command carries alone (§5, §11): an opening's
// electorate and thresholds, or a measure's positions and threshold, the
// rows they share reading both through their figures.
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
        let members = Figures::members(command);
        let thresholds = Figures::thresholds(command);
        match self {
            CommandSituation::EmptyElectorate => members.is_empty(),
            CommandSituation::RepeatedMember => Numbers::lowest_repeated(&members).is_some(),
            CommandSituation::NoThresholds => thresholds.is_empty(),
            CommandSituation::ThresholdOutOfRange => {
                Numbers::lowest_out_of_range(&thresholds, members.len()).is_some()
            }
            CommandSituation::RepeatedThreshold => Numbers::lowest_repeated(&thresholds).is_some(),
        }
    }
}
