// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::spec_files::SpecFiles;
use cartel_spec_model::model::Model;
use cartel_spec_model::spec_reader::SpecReader;
use serde_json::json;

fn fresh() -> Model {
    Model::new(
        SpecReader::table(&SpecFiles::matrix()),
        SpecReader::origin(&json!({"Fresh": {"instance": {"node": "2", "incarnation": "3"}}})),
    )
}

#[test]
fn process_of_a_command_answers_the_first_row_that_applies() {
    // Arrange
    let mut model = fresh();

    // Act
    let answer = model.process(&json!({"OpenBallot": {
        "ballot": {"question": "1", "round": "0", "phase": "0"},
        "electorate": [],
        "thresholds": [],
    }}));

    // Assert
    assert_eq!(answer.row, "empty-electorate");
    assert_eq!(
        answer.receipt,
        json!({"number": "1", "output": {"CommandRejected": {"EmptyElectorate": {}}}})
    );
}

#[test]
fn process_of_a_vote_that_reaches_a_threshold_answers_the_quorum() {
    // Arrange
    let mut model = fresh();
    let ballot = json!({"question": "1", "round": "0", "phase": "0"});
    model.process(
        &json!({"OpenBallot": {"ballot": ballot, "electorate": ["1", "2"], "thresholds": ["1"]}}),
    );

    // Act
    let answer =
        model.process(&json!({"CastVote": {"ballot": ballot, "member": "2", "choice": "aa"}}));

    // Assert
    assert_eq!(answer.row, "quorum-reached");
    assert_eq!(
        answer.receipt["output"],
        json!({"QuorumReached": {"count": "1", "thresholds": ["1"], "voters": ["2"]}})
    );
}

#[test]
fn process_of_each_command_numbers_its_receipt_one_more_than_the_last() {
    // Arrange
    let mut model = fresh();

    // Act
    let numbers = [
        model.process(&json!({"ListBallots": {}})),
        model.process(&json!({"ListBallots": {}})),
    ]
    .map(|answer| answer.receipt["number"].clone());

    // Assert
    assert_eq!(numbers, [json!("1"), json!("2")]);
}

#[test]
fn process_of_show_books_answers_the_state_after_its_own_call() {
    // Arrange
    let mut model = fresh();

    // Act
    let answer = model.process(&json!({"ShowBooks": {}}));

    // Assert
    assert_eq!(
        answer.receipt["output"],
        json!({"BooksShown": {"checkpoint": {
            "instance": {"node": "2", "incarnation": "3"},
            "ballots": [],
            "next_receipt": "2",
        }}})
    );
}
