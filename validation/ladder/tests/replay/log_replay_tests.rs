// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The replay tier: logs of random sequences written in the specification's
// step form and replayed into a fresh instance of the same origin, its bounds
// included -- every receipt rebuilt -- and a log with a line lost, the gap
// found where it is.

use crate::support::log_replay::LogReplay;
use crate::support::random_commands::RandomCommands;
use crate::support::spec_files::SpecFiles;
use crate::support::spec_schema::SpecSchema;
use crate::support::spec_writer::SpecWriter;
use cartel::ballot::Ballot;
use cartel::bounds::Bounds;
use cartel::cartel::Cartel;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::instance_id::InstanceId;
use cartel::no_op_observer::NoOpObserver;
use cartel::origin::Origin;
use proptest::proptest;
use serde_json::Value;
use serde_json::json;

const ORIGIN: Origin = Origin::Fresh(Fresh {
    instance: InstanceId {
        node: 3,
        incarnation: 9,
    },
    bounds: Bounds {
        open_ballots: None,
        electorate: None,
        choice: None,
    },
});

// The log a run of commands leaves on an instance of the origin given: each
// command beside its receipt.
fn logged(origin: Origin, commands: &[Command]) -> Vec<Value> {
    let mut cartel = Cartel::new(origin, Box::new(NoOpObserver));
    commands
        .iter()
        .map(|command| {
            let receipt = cartel.process(command.clone());
            json!({"command": SpecWriter::command(command), "receipt": SpecWriter::receipt(&receipt)})
        })
        .collect()
}

proptest! {
    #[test]
    fn replay_of_a_log_of_any_sequence_rebuilds_every_receipt(commands in RandomCommands::sequence()) {
        // Arrange
        let lines = logged(ORIGIN, &commands);
        let schema = SpecSchema::new(SpecFiles::schema());

        // Act
        let report = LogReplay::replay(&SpecWriter::origin(&ORIGIN), &lines);

        // Assert
        assert!(lines.iter().all(|line| schema.admits_step(line)));
        assert_eq!((report.reconstructed, report.total), (lines.len(), lines.len()), "{:?}", report.departures);
        assert!(report.gaps.is_empty());
    }

    #[test]
    fn replay_of_a_log_of_any_sequence_under_any_bounds_rebuilds_every_receipt(
        bounds in RandomCommands::bounds(),
        commands in RandomCommands::sequence(),
    ) {
        // Arrange
        let origin = Origin::Fresh(Fresh {
            instance: InstanceId {
                node: 3,
                incarnation: 9,
            },
            bounds,
        });
        let lines = logged(origin, &commands);

        // Act
        let report = LogReplay::replay(&SpecWriter::origin(&origin), &lines);

        // Assert
        assert_eq!((report.reconstructed, report.total), (lines.len(), lines.len()), "{:?}", report.departures);
        assert!(report.gaps.is_empty());
    }
}

#[test]
fn replay_of_a_log_with_a_line_lost_finds_the_gap_and_rebuilds_up_to_it() {
    // Arrange
    let commands = vec![
        Command::ListBallots,
        Command::OpenBallot {
            ballot: Ballot {
                question: 1,
                round: 0,
                phase: 0,
            },
            electorate: vec![1, 2],
            thresholds: vec![2],
        },
        Command::ShowBooks,
        Command::ListBallots,
    ];
    let mut lines = logged(ORIGIN, &commands);
    lines.remove(2);

    // Act
    let report = LogReplay::replay(&SpecWriter::origin(&ORIGIN), &lines);

    // Assert
    assert_eq!(report.gaps, vec![3]);
    assert_eq!((report.reconstructed, report.total), (2, 3));
}
