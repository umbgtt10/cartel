// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::call::Call;
use cartel_spec_model::command::Command;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::vocabulary::rejection_name::RejectionName;
use serde_json::json;
use serde_json::Value;

fn encoded(rejection: RejectionName, command: &Command) -> Value {
    let state = state();
    rejection.encoded(&Call {
        command,
        before: &state,
        after: &state,
    })
}

// Member 1 a voter of 0xaa in ballot (1, 0, 0).
fn state() -> InstanceState {
    States::holding(vec![States::ballot(
        States::key(1, 0, 0),
        &[1, 2],
        &[1],
        &[(1, &[0xaa])],
    )])
}

#[test]
fn encoded_of_a_rejection_that_carries_nothing_is_its_name_alone() {
    // Arrange & Act
    let written = [
        RejectionName::EmptyElectorate,
        RejectionName::NoThresholds,
        RejectionName::BallotAlreadyOpen,
        RejectionName::BallotNotOpen,
        RejectionName::NotAMember,
        RejectionName::AlreadyCounted,
    ]
    .map(|rejection| encoded(rejection, &Commands::open(States::key(1, 0, 0), &[1], &[1])));

    // Assert
    assert_eq!(
        written,
        [
            json!({"EmptyElectorate": {}}),
            json!({"NoThresholds": {}}),
            json!({"BallotAlreadyOpen": {}}),
            json!({"BallotNotOpen": {}}),
            json!({"NotAMember": {}}),
            json!({"AlreadyCounted": {}}),
        ]
    );
}

#[test]
fn encoded_of_equivocation_names_the_choice_counted() {
    // Arrange & Act
    let written = encoded(
        RejectionName::Equivocation,
        &Commands::cast(States::key(1, 0, 0), 1, &[0xbb]),
    );

    // Assert
    assert_eq!(written, json!({"Equivocation": {"counted": "aa"}}));
}

#[test]
fn encoded_of_repeated_member_names_the_lowest_member_given_twice() {
    // Arrange & Act
    let written = encoded(
        RejectionName::RepeatedMember,
        &Commands::open(States::key(2, 0, 0), &[5, 3, 5, 3], &[1]),
    );

    // Assert
    assert_eq!(written, json!({"RepeatedMember": {"member": "3"}}));
}

#[test]
fn encoded_of_repeated_threshold_names_the_lowest_threshold_given_twice() {
    // Arrange & Act
    let written = encoded(
        RejectionName::RepeatedThreshold,
        &Commands::open(States::key(2, 0, 0), &[1, 2, 3], &[3, 1, 3, 1]),
    );

    // Assert
    assert_eq!(written, json!({"RepeatedThreshold": {"threshold": "1"}}));
}

#[test]
fn encoded_of_threshold_out_of_range_names_the_lowest_threshold_out_of_range() {
    // Arrange & Act
    let written = encoded(
        RejectionName::ThresholdOutOfRange,
        &Commands::open(States::key(2, 0, 0), &[1, 2, 3], &[6, 2, 4]),
    );

    // Assert
    assert_eq!(written, json!({"ThresholdOutOfRange": {"threshold": "4"}}));
}
