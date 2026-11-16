// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::rules::books_situation::BooksSituation;

#[test]
fn holds_of_close_behind_is_whether_a_close_is_at_or_before_the_close_point() {
    // Arrange
    let state = InstanceState {
        close_point: Some(States::point(5, 2)),
        ..States::instance()
    };

    // Act
    let held = [(5, 1), (5, 2), (5, 3)].map(|(question, round)| {
        BooksSituation::CloseBehind.holds(&Commands::close(question, round), &state)
    });

    // Assert
    assert_eq!(held, [true, true, false]);
}

#[test]
fn holds_of_close_behind_on_books_never_closed_is_false() {
    // Arrange & Act
    let held = BooksSituation::CloseBehind.holds(&Commands::close(0, 0), &States::instance());

    // Assert
    assert!(!held);
}

#[test]
fn holds_of_closed_is_whether_the_command_s_ballot_is_below_the_close_point() {
    // Arrange
    let state = InstanceState {
        close_point: Some(States::point(2, 1)),
        ..States::instance()
    };

    // Act
    let held = [
        States::key(2, 0, 9),
        States::key(2, 1, 0),
        States::key(1, 5, 5),
    ]
    .map(|key| BooksSituation::Closed.holds(&Commands::show(key), &state));

    // Assert
    assert_eq!(held, [true, false, true]);
}

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
