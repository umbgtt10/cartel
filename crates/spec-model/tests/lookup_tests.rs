// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::lookup::Lookup;

#[test]
fn ballot_of_a_key_is_the_open_ballot_of_that_key() {
    // Arrange
    let state = States::holding(vec![
        States::ballot(States::key(1, 0, 0), &[1], &[1], &[]),
        States::ballot(States::key(1, 0, 1), &[2], &[1], &[]),
    ]);

    // Act
    let found = [States::key(1, 0, 1), States::key(1, 1, 0)]
        .map(|key| Lookup::ballot(&state, Some(key)).map(|ballot| ballot.electorate.clone()));

    // Assert
    assert_eq!(found, [Some(vec![2]), None]);
}

#[test]
fn choice_of_a_member_is_the_choice_it_is_a_voter_of() {
    // Arrange
    let ballot = States::ballot(States::key(1, 0, 0), &[1, 2], &[1], &[(1, &[0xaa])]);

    // Act
    let choices = [1, 2].map(|member| Lookup::choice(&ballot, member).cloned());

    // Assert
    assert_eq!(choices, [Some(States::choice(&[0xaa])), None]);
}

#[test]
fn is_below_of_a_key_is_whether_its_question_and_round_come_before_the_point() {
    // Arrange
    let point = States::point(2, 1);

    // Act
    let below = [
        States::key(2, 0, 9),
        States::key(2, 1, 0),
        States::key(1, 9, 9),
        States::key(3, 0, 0),
    ]
    .map(|key| Lookup::is_below(key, point));

    // Assert
    assert_eq!(below, [true, false, true, false]);
}

#[test]
fn named_of_a_command_is_the_open_ballot_it_names() {
    // Arrange
    let state = States::holding(vec![States::ballot(States::key(4, 1, 2), &[7], &[1], &[])]);

    // Act
    let named = Lookup::named(&state, &Commands::show(States::key(4, 1, 2)));

    // Assert
    assert_eq!(named.electorate, vec![7]);
}

#[test]
fn voters_of_a_choice_are_its_voters_in_ascending_order() {
    // Arrange
    let ballot = States::ballot(
        States::key(1, 0, 0),
        &[1, 2, 3, 4],
        &[2],
        &[(3, &[0xaa]), (2, &[0xbb]), (1, &[0xaa])],
    );

    // Act
    let voters = Lookup::voters(&ballot, &States::choice(&[0xaa]));

    // Assert
    assert_eq!(voters, vec![1, 3]);
}
