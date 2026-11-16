// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::spec_files::SpecFiles;
use crate::support::states::States;
use cartel_spec_model::spec_reader::SpecReader;
use cartel_spec_model::table::decision_table::DecisionTable;
use cartel_spec_model::table::row::Row;
use cartel_spec_model::vocabulary::command_name::CommandName;
use cartel_spec_model::vocabulary::output_name::OutputName;

#[test]
fn new_of_rows_answers_by_those_rows_alone() {
    // Arrange
    let table = DecisionTable::new(vec![Row {
        id: String::from("ballots-listed"),
        commands: vec![CommandName::ListBallots],
        output: OutputName::BallotsListed,
        rejection: None,
    }]);

    // Act
    let row = table.row(&Commands::of(CommandName::ListBallots), &States::instance());

    // Assert
    assert_eq!(row.id, "ballots-listed");
}

#[test]
fn row_of_a_command_is_the_first_row_for_it_that_applies() {
    // Arrange
    let table = SpecReader::table(&SpecFiles::matrix());

    // Act
    let rows = [
        Commands::open(States::key(1, 0, 0), &[], &[]),
        Commands::show(States::key(1, 0, 0)),
    ]
    .map(|command| table.row(&command, &States::instance()).id.clone());

    // Assert
    assert_eq!(rows, ["empty-electorate", "ballot-not-open"]);
}
