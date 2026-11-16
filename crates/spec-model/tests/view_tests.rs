// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::states::States;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::view::View;
use serde_json::json;

#[test]
fn ballot_of_a_key_is_its_three_numbers() {
    // Arrange & Act
    let written = View::ballot(States::key(1, 2, 3));

    // Assert
    assert_eq!(
        written,
        json!({"question": "1", "round": "2", "phase": "3"})
    );
}

#[test]
fn ballots_of_a_state_are_its_open_ballots_in_canonical_order() {
    // Arrange
    let state = States::holding(vec![
        States::ballot(States::key(2, 0, 0), &[1], &[1], &[]),
        States::ballot(States::key(1, 1, 0), &[1], &[1], &[]),
        States::ballot(States::key(1, 0, 1), &[1], &[1], &[]),
    ]);

    // Act
    let ballots = View::ballots(&state);

    // Assert
    assert_eq!(
        ballots,
        json!([
            {"question": "1", "round": "0", "phase": "1"},
            {"question": "1", "round": "1", "phase": "0"},
            {"question": "2", "round": "0", "phase": "0"},
        ])
    );
}

#[test]
fn checkpoint_of_a_state_is_its_instance_ballots_and_next_receipt() {
    // Arrange
    let state = States::holding(vec![
        States::ballot(States::key(3, 0, 0), &[1], &[1], &[]),
        States::ballot(States::key(1, 0, 0), &[2], &[1], &[]),
    ]);

    // Act
    let checkpoint = View::checkpoint(&state);

    // Assert
    let record = |question: &str, member: &str| {
        json!({
            "ballot": {"question": question, "round": "0", "phase": "0"},
            "electorate": [member],
            "thresholds": ["1"],
            "tally": [],
        })
    };
    assert_eq!(
        checkpoint,
        json!({
            "instance": {"node": "2", "incarnation": "3"},
            "ballots": [record("1", "2"), record("3", "1")],
            "next_receipt": "5",
        })
    );
}

#[test]
fn checkpoint_of_closed_books_holds_the_close_point() {
    // Arrange
    let state = InstanceState {
        close_point: Some(States::point(3, 1)),
        ..States::instance()
    };

    // Act
    let checkpoint = View::checkpoint(&state);

    // Assert
    assert_eq!(
        checkpoint,
        json!({
            "instance": {"node": "2", "incarnation": "3"},
            "close_point": {"question": "3", "round": "1"},
            "ballots": [],
            "next_receipt": "5",
        })
    );
}

#[test]
fn choice_of_bytes_is_their_lowercase_hexadecimal_digits() {
    // Arrange & Act
    let written =
        [States::choice(&[0x0a, 0xff]), States::choice(&[])].map(|choice| View::choice(&choice));

    // Assert
    assert_eq!(written, [json!("0aff"), json!("")]);
}

#[test]
fn number_of_a_u64_is_its_decimal_digits() {
    // Arrange & Act
    let written = View::number(u64::MAX);

    // Assert
    assert_eq!(written, json!("18446744073709551615"));
}

#[test]
fn point_of_a_question_and_a_round_is_its_two_numbers() {
    // Arrange & Act
    let written = View::point(States::point(4, 0));

    // Assert
    assert_eq!(written, json!({"question": "4", "round": "0"}));
}

#[test]
fn record_of_a_ballot_orders_its_electorate_thresholds_and_tally() {
    // Arrange
    let ballot = States::ballot(
        States::key(1, 2, 3),
        &[3, 1, 2],
        &[3, 2],
        &[(3, &[0xff]), (2, &[0x01, 0x00]), (1, &[0xff])],
    );

    // Act
    let record = View::record(&ballot);

    // Assert
    assert_eq!(
        record,
        json!({
            "ballot": {"question": "1", "round": "2", "phase": "3"},
            "electorate": ["1", "2", "3"],
            "thresholds": ["2", "3"],
            "tally": [
                {"choice": "0100", "voters": ["2"]},
                {"choice": "ff", "voters": ["1", "3"]},
            ],
        })
    );
}
