// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec;
use alloc::vec::Vec;

use crate::command::Command;

// The figures a command gives, whichever form it carries them in (§5, §11):
// the members -- an opening's electorate, or the members of a measure's
// positions -- and the thresholds, an opening's several or a measure's one.
// The rows on repeated members and on thresholds out of range read them so,
// one rule for both commands.
pub struct Figures;

impl Figures {
    #[must_use]
    pub fn members(command: &Command) -> Vec<u64> {
        command.electorate.clone().unwrap_or_else(|| {
            command
                .positions
                .as_ref()
                .expect("an electorate or positions")
                .iter()
                .map(|given| given.member)
                .collect()
        })
    }

    #[must_use]
    pub fn thresholds(command: &Command) -> Vec<u64> {
        command
            .thresholds
            .clone()
            .unwrap_or_else(|| vec![command.threshold.expect("thresholds or a threshold")])
    }
}
