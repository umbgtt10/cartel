// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::rules::change::Change;
use cartel_spec_model::vocabulary::command_name::CommandName;

#[test]
fn applied_of_ballot_opened_opens_the_ballot_with_an_empty_tally() {
    // Arrange
    let command = Commands::open(States::key(1, 0, 0), &[2, 1], &[2]);

    // Act
    let after = Change::BallotOpened.applied(&command, &States::instance());

    // Assert
    assert_eq!(
        after,
        States::holding(vec![States::ballot(
            States::key(1, 0, 0),
            &[2, 1],
            &[2],
            &[]
        )])
    );
}

#[test]
fn applied_of_nothing_leaves_the_state_as_it_was() {
    // Arrange
    let state = States::holding(vec![States::ballot(States::key(1, 0, 0), &[1], &[1], &[])]);

    // Act
    let after = Change::Nothing.applied(&Commands::of(CommandName::ListBallots), &state);

    // Assert
    assert_eq!(after, state);
}

#[test]
fn applied_of_vote_cast_makes_the_member_a_voter_of_the_choice_in_that_ballot_alone() {
    // Arrange
    let state = States::holding(vec![
        States::ballot(States::key(1, 0, 0), &[1, 2], &[2], &[(1, &[0xaa])]),
        States::ballot(States::key(2, 0, 0), &[1, 2], &[2], &[]),
    ]);

    // Act
    let after = Change::VoteCast.applied(&Commands::cast(States::key(1, 0, 0), 2, &[0xaa]), &state);

    // Assert
    assert_eq!(
        after,
        States::holding(vec![
            States::ballot(
                States::key(1, 0, 0),
                &[1, 2],
                &[2],
                &[(1, &[0xaa]), (2, &[0xaa])]
            ),
            States::ballot(States::key(2, 0, 0), &[1, 2], &[2], &[]),
        ])
    );
}
