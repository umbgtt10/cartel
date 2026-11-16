// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::rules::books_situation::BooksSituation;

#[test]
fn holds_of_not_open_is_whether_no_ballot_of_the_command_s_key_is_open() {
    // Arrange
    let state = States::holding(vec![States::ballot(States::key(1, 0, 0), &[1], &[1], &[])]);

    // Act
    let held = [States::key(1, 0, 1), States::key(1, 0, 0)]
        .map(|key| BooksSituation::NotOpen.holds(&Commands::show(key), &state));

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_open_is_whether_a_ballot_of_the_command_s_key_is_open() {
    // Arrange
    let state = States::holding(vec![States::ballot(States::key(1, 0, 0), &[1], &[1], &[])]);

    // Act
    let held = [States::key(1, 0, 0), States::key(1, 0, 1)]
        .map(|key| BooksSituation::Open.holds(&Commands::show(key), &state));

    // Assert
    assert_eq!(held, [true, false]);
}
