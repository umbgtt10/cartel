// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::spec_files::SpecFiles;
use crate::support::states::States;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::spec_reader::SpecReader;
use cartel_spec_model::vocabulary::command_name::CommandName;
use serde_json::json;

#[test]
fn command_of_a_query_carries_nothing_else() {
    // Arrange & Act
    let command = SpecReader::command(&json!({"ListBallots": {}}));

    // Assert
    assert_eq!(command, Commands::of(CommandName::ListBallots));
}

#[test]
fn command_of_a_vote_reads_its_ballot_member_and_choice() {
    // Arrange
    let value = json!({"CastVote": {
        "ballot": {"question": "1", "round": "0", "phase": "0"},
        "member": "7",
        "choice": "0aff",
    }});

    // Act
    let command = SpecReader::command(&value);

    // Assert
    assert_eq!(
        command,
        Commands::cast(States::key(1, 0, 0), 7, &[0x0a, 0xff])
    );
}

#[test]
fn command_of_an_open_ballot_reads_its_ballot_electorate_and_thresholds() {
    // Arrange
    let value = json!({"OpenBallot": {
        "ballot": {"question": "4", "round": "1", "phase": "2"},
        "electorate": ["3", "1"],
        "thresholds": ["2"],
    }});

    // Act
    let command = SpecReader::command(&value);

    // Assert
    assert_eq!(command, Commands::open(States::key(4, 1, 2), &[3, 1], &[2]));
}

#[test]
fn origin_of_a_fresh_instance_holds_no_ballot_and_numbers_from_one() {
    // Arrange & Act
    let state =
        SpecReader::origin(&json!({"Fresh": {"instance": {"node": "2", "incarnation": "3"}}}));

    // Assert
    assert_eq!(
        state,
        InstanceState {
            next_receipt: 1,
            ..States::instance()
        }
    );
}

#[test]
fn table_of_the_matrix_answers_by_its_rows() {
    // Arrange
    let table = SpecReader::table(&SpecFiles::matrix());

    // Act
    let row = table.row(&Commands::of(CommandName::ShowBooks), &States::instance());

    // Assert
    assert_eq!(row.id, "books-shown");
}
