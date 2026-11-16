// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The property tier: random sequences of commands over a small world, the
// crate against the specification's model receipt by receipt, and the
// invariants after every call -- receipts one more on every call, no ballot
// touched by a command naming another, no member a voter of two choices in a
// ballot -- and forgetting safe: of the votes counted, any subset reaches no
// quorum the whole set did not.

use crate::support::both_sides::BothSides;
use crate::support::random_commands::RandomCommands;
use cartel::ballot::Ballot;
use cartel::cartel::Cartel;
use cartel::checkpoint::Checkpoint;
use cartel::choice::Choice;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::instance_id::InstanceId;
use cartel::no_op_observer::NoOpObserver;
use cartel::origin::Origin;
use cartel::output::Output;
use cartel::rows::closing_rows::ClosingRows;
use proptest::collection::vec;
use proptest::prelude::any;
use proptest::proptest;
use std::collections::BTreeSet;

fn books(cartel: &mut Cartel) -> Checkpoint {
    match cartel.process(Command::ShowBooks).output {
        Output::BooksShown(checkpoint) => checkpoint,
        other => panic!("the books shown, not {other:?}"),
    }
}

fn fresh() -> Cartel {
    Cartel::new(
        Origin::Fresh(Fresh {
            instance: InstanceId {
                node: 1,
                incarnation: 1,
            },
        }),
        Box::new(NoOpObserver),
    )
}

fn named(command: &Command) -> Option<Ballot> {
    match command {
        Command::OpenBallot { ballot, .. }
        | Command::CastVote { ballot, .. }
        | Command::ShowBallot { ballot } => Some(*ballot),
        Command::CloseBallots { .. } | Command::ListBallots | Command::ShowBooks => None,
    }
}

proptest! {
    #[test]
    fn process_of_any_sequence_answers_as_the_model_does(commands in RandomCommands::sequence()) {
        // Arrange
        let mut sides = BothSides::new();

        // Act
        let departures: Vec<String> = commands
            .iter()
            .chain([&Command::ShowBooks])
            .filter_map(|command| {
                let (from_crate, from_model) = sides.process(command);
                (from_crate != from_model).then(|| format!("{command:?}: crate {from_crate}, model {from_model}"))
            })
            .collect();

        // Assert
        assert!(departures.is_empty(), "{departures:#?}");
    }

    #[test]
    fn process_of_any_sequence_counts_no_member_twice_in_a_ballot(commands in RandomCommands::sequence()) {
        // Arrange
        let mut cartel = fresh();

        // Act
        for command in commands {
            let _ = cartel.process(command);
        }

        // Assert
        for record in books(&mut cartel).ballots {
            let voters: Vec<u64> = record.tally.iter().flat_map(|votes| votes.voters.clone()).collect();
            let distinct: BTreeSet<u64> = voters.iter().copied().collect();
            assert_eq!(voters.len(), distinct.len(), "{record:?}");
        }
    }

    #[test]
    fn process_of_any_sequence_numbers_every_receipt_one_more_than_the_last(commands in RandomCommands::sequence()) {
        // Arrange
        let mut cartel = fresh();

        // Act
        let numbers: Vec<u64> = commands.into_iter().map(|command| cartel.process(command).number).collect();

        // Assert
        let expected: Vec<u64> = (1..=numbers.len() as u64).collect();
        assert_eq!(numbers, expected);
    }

    #[test]
    fn process_of_a_command_naming_a_ballot_leaves_every_other_ballot_as_it_was(
        commands in RandomCommands::sequence(),
    ) {
        // Arrange
        let mut cartel = fresh();

        // Act & Assert
        for command in commands {
            let before = books(&mut cartel);
            let _ = cartel.process(command.clone());
            let after = books(&mut cartel);
            let kept: Vec<_> = match (&command, after.close_point) {
                (Command::CloseBallots { .. }, Some(point)) => {
                    before.ballots.iter().filter(|record| !ClosingRows::new(Some(point)).covers(record.ballot)).cloned().collect()
                }
                _ => before.ballots.iter().filter(|record| Some(record.ballot) != named(&command)).cloned().collect(),
            };
            let left: Vec<_> = after
                .ballots
                .iter()
                .filter(|record| matches!(command, Command::CloseBallots { .. }) || Some(record.ballot) != named(&command))
                .cloned()
                .collect();
            assert_eq!(kept, left, "{command:?}");
        }
    }

    #[test]
    fn process_of_any_sequence_leaves_no_open_ballot_below_the_close_point(commands in RandomCommands::sequence()) {
        // Arrange
        let mut cartel = fresh();

        // Act
        for command in commands {
            let _ = cartel.process(command);
        }

        // Assert
        let checkpoint = books(&mut cartel);
        if let Some(point) = checkpoint.close_point {
            assert!(checkpoint.ballots.iter().all(|record| !ClosingRows::new(Some(point)).covers(record.ballot)), "{checkpoint:?}");
        }
    }

    #[test]
    fn process_of_a_subset_of_the_votes_counted_reaches_no_quorum_the_whole_set_did_not(
        commands in RandomCommands::sequence(),
        kept in vec(any::<bool>(), 60),
    ) {
        // Arrange
        let mut cartel = fresh();
        let counted: Vec<Command> = commands
            .iter()
            .filter(|command| {
                let output = cartel.process((*command).clone()).output;
                matches!(command, Command::OpenBallot { .. } | Command::CloseBallots { .. })
                    || matches!(output, Output::VoteCounted { .. } | Output::QuorumReached { .. })
            })
            .cloned()
            .collect();
        let subset: Vec<Command> = counted
            .iter()
            .zip(kept.iter().cycle())
            .filter(|(command, keep)| {
                matches!(command, Command::OpenBallot { .. } | Command::CloseBallots { .. }) || **keep
            })
            .map(|(command, _)| command.clone())
            .collect();

        // Act
        let reached_by_subset = quorums(&subset);

        // Assert
        let reached_by_all = quorums(&counted);
        assert!(reached_by_subset.is_subset(&reached_by_all), "{reached_by_subset:?} beyond {reached_by_all:?}");
    }
}

// The quorums a run of commands reached: each ballot, choice and threshold.
fn quorums(commands: &[Command]) -> BTreeSet<(Ballot, Choice, u64)> {
    let mut cartel = fresh();
    commands
        .iter()
        .filter_map(
            |command| match (command, cartel.process(command.clone()).output) {
                (
                    Command::CastVote { ballot, choice, .. },
                    Output::QuorumReached { thresholds, .. },
                ) => Some(
                    thresholds
                        .into_iter()
                        .map(|threshold| (*ballot, choice.clone(), threshold))
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            },
        )
        .flatten()
        .collect()
}
