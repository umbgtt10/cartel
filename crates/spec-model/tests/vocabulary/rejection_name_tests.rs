// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::bounds::Bounds;
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
fn encoded_of_a_bound_s_refusal_names_the_bound() {
    // Arrange
    let bounded = InstanceState {
        bounds: Bounds {
            open_ballots: Some(4),
            electorate: Some(3),
            choice: Some(0),
        },
        ..state()
    };
    let opening = Commands::open(States::key(2, 0, 0), &[1, 2, 3, 4], &[1]);
    let vote = Commands::cast(States::key(1, 0, 0), 2, &[0xbb]);

    // Act
    let written = [
        (RejectionName::ElectorateTooLarge, &opening),
        (RejectionName::BooksFull, &opening),
        (RejectionName::ChoiceTooLong, &vote),
    ]
    .map(|(rejection, command)| {
        rejection.encoded(&Call {
            command,
            before: &bounded,
            after: &bounded,
        })
    });

    // Assert
    assert_eq!(
        written,
        [
            json!({"ElectorateTooLarge": {"bound": "3"}}),
            json!({"BooksFull": {"bound": "4"}}),
            json!({"ChoiceTooLong": {"bound": "0"}}),
        ]
    );
}

#[test]
fn encoded_of_a_measure_s_refusals_names_its_member_or_threshold() {
    // Arrange & Act
    let written = [
        encoded(
            RejectionName::RepeatedMember,
            &Commands::measure(&[(5, 1), (3, 1), (5, 2), (3, 2)], 1),
        ),
        encoded(
            RejectionName::ThresholdOutOfRange,
            &Commands::measure(&[(1, 5), (2, 3)], 3),
        ),
    ];

    // Assert
    assert_eq!(
        written,
        [
            json!({"RepeatedMember": {"member": "3"}}),
            json!({"ThresholdOutOfRange": {"threshold": "3"}}),
        ]
    );
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
        RejectionName::BallotClosed,
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
            json!({"BallotClosed": {}}),
        ]
    );
}

#[test]
fn encoded_of_close_behind_names_the_close_point() {
    // Arrange
    let state = InstanceState {
        close_point: Some(States::point(5, 2)),
        ..state()
    };
    let command = Commands::close(5, 1);

    // Act
    let written = RejectionName::CloseBehind.encoded(&Call {
        command: &command,
        before: &state,
        after: &state,
    });

    // Assert
    assert_eq!(
        written,
        json!({"CloseBehind": {"close_point": {"question": "5", "round": "2"}}})
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
