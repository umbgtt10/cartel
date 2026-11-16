// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::spec_files::SpecFiles;
use crate::support::states::States;
use cartel_spec_model::bounds::Bounds;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::spec_reader::SpecReader;
use cartel_spec_model::vocabulary::command_name::CommandName;
use serde_json::json;

#[test]
fn command_of_a_close_reads_its_point() {
    // Arrange & Act
    let command =
        SpecReader::command(&json!({"CloseBallots": {"before": {"question": "7", "round": "1"}}}));

    // Assert
    assert_eq!(command, Commands::close(7, 1));
}

#[test]
fn command_of_a_measure_reads_its_positions_and_threshold() {
    // Arrange
    let value = json!({"MeasureWatermark": {
        "positions": [{"member": "3", "position": "7"}, {"member": "1", "position": "5"}],
        "threshold": "2",
    }});

    // Act
    let command = SpecReader::command(&value);

    // Assert
    assert_eq!(command, Commands::measure(&[(3, 7), (1, 5)], 2));
}

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
fn origin_of_a_fresh_instance_with_bounds_holds_the_bounds_given() {
    // Arrange & Act
    let state = SpecReader::origin(&json!({"Fresh": {
        "instance": {"node": "2", "incarnation": "3"},
        "bounds": {"open_ballots": "4", "choice": "0"},
    }}));

    // Assert
    assert_eq!(
        state.bounds,
        Bounds {
            open_ballots: Some(4),
            electorate: None,
            choice: Some(0),
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
