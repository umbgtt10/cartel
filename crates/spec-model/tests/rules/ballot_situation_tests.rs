// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::rules::ballot_situation::BallotSituation;

fn held(situation: BallotSituation, votes: [(u64, &[u8]); 2]) -> [bool; 2] {
    votes.map(|(member, choice)| {
        situation.holds(
            &Commands::cast(States::key(1, 0, 0), member, choice),
            &state(),
        )
    })
}

// Members 1 to 4, thresholds 2 and 3; member 1 a voter of 0xaa, member 2 of 0xbb.
fn state() -> InstanceState {
    States::holding(vec![States::ballot(
        States::key(1, 0, 0),
        &[1, 2, 3, 4],
        &[2, 3],
        &[(1, &[0xaa]), (2, &[0xbb])],
    )])
}

#[test]
fn holds_of_already_counted_is_whether_the_member_is_a_voter_of_the_choice() {
    // Arrange & Act
    let held = held(
        BallotSituation::AlreadyCounted,
        [(1, &[0xaa]), (2, &[0xaa])],
    );

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_equivocates_is_whether_the_member_is_a_voter_of_another_choice() {
    // Arrange & Act
    let held = held(BallotSituation::Equivocates, [(1, &[0xbb]), (1, &[0xaa])]);

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_not_a_member_is_whether_the_member_is_outside_the_electorate() {
    // Arrange & Act
    let held = held(BallotSituation::NotAMember, [(5, &[0xaa]), (4, &[0xaa])]);

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_reaches_threshold_is_whether_the_count_with_the_vote_is_a_threshold() {
    // Arrange & Act
    let held = held(
        BallotSituation::ReachesThreshold,
        [(3, &[0xaa]), (3, &[0xcc])],
    );

    // Assert
    assert_eq!(held, [true, false]);
}
