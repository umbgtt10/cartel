// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::bounds::Bounds;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::rules::ballot_situation::BallotSituation;
use cartel_spec_model::rules::books_situation::BooksSituation;
use cartel_spec_model::rules::bound_situation::BoundSituation;
use cartel_spec_model::rules::command_situation::CommandSituation;
use cartel_spec_model::rules::situation::Situation;
use cartel_spec_model::vocabulary::command_name::CommandName;

#[test]
fn holds_of_a_ballot_situation_is_that_situation_s() {
    // Arrange
    let situation = Situation::Ballot(BallotSituation::NotAMember);
    let state = States::holding(vec![States::ballot(States::key(1, 0, 0), &[1], &[1], &[])]);

    // Act
    let held = [2, 1]
        .map(|member| situation.holds(&Commands::cast(States::key(1, 0, 0), member, &[]), &state));

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_a_books_situation_is_that_situation_s() {
    // Arrange
    let situation = Situation::Books(BooksSituation::Open);
    let state = States::holding(vec![States::ballot(States::key(1, 0, 0), &[1], &[1], &[])]);

    // Act
    let held = [States::key(1, 0, 0), States::key(2, 0, 0)]
        .map(|key| situation.holds(&Commands::show(key), &state));

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_a_bound_situation_is_that_situation_s() {
    // Arrange
    let situation = Situation::Bound(BoundSituation::ElectorateTooLarge);
    let state = InstanceState {
        bounds: Bounds {
            electorate: Some(1),
            ..Bounds::default()
        },
        ..States::instance()
    };

    // Act
    let held = [&[1, 2][..], &[1]].map(|electorate| {
        situation.holds(
            &Commands::open(States::key(1, 0, 0), electorate, &[1]),
            &state,
        )
    });

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_a_command_situation_is_that_situation_s() {
    // Arrange
    let situation = Situation::Command(CommandSituation::NoThresholds);

    // Act
    let held = [
        Commands::open(States::key(1, 0, 0), &[1], &[]),
        Commands::open(States::key(1, 0, 0), &[1], &[1]),
    ]
    .map(|command| situation.holds(&command, &States::instance()));

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_always_is_true() {
    // Arrange
    let command = Commands::of(CommandName::ListBallots);

    // Act & Assert
    assert!(Situation::Always.holds(&command, &States::instance()));
}
