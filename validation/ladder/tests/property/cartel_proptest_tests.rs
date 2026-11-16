// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The property tier: random sequences of commands over a small world, the
// crate against the specification's model receipt by receipt, without bounds
// and under any, and the invariants after every call -- receipts one more on
// every call, no ballot touched by a command naming another, no member a
// voter of two choices in a ballot, the books within every bound given, a
// claim changing nothing -- and forgetting safe: of the votes counted, any
// subset reaches no quorum the whole set did not, and of a claim's voters,
// any subset reaches no threshold the whole list did not.

use crate::support::both_sides::BothSides;
use crate::support::random_commands::RandomCommands;
use cartel::ballot::Ballot;
use cartel::bounds::Bounds;
use cartel::cartel::Cartel;
use cartel::checkpoint::Checkpoint;
use cartel::choice::Choice;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::instance_id::InstanceId;
use cartel::member_position::MemberPosition;
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

fn bounded(bounds: Bounds) -> Cartel {
    Cartel::new(
        Origin::Fresh(Fresh {
            instance: InstanceId {
                node: 1,
                incarnation: 1,
            },
            bounds,
        }),
        Box::new(NoOpObserver),
    )
}

fn fresh() -> Cartel {
    bounded(Bounds::default())
}

fn named(command: &Command) -> Option<Ballot> {
    match command {
        Command::OpenBallot { ballot, .. }
        | Command::CastVote { ballot, .. }
        | Command::ShowBallot { ballot } => Some(*ballot),
        Command::CloseBallots { .. }
        | Command::MeasureWatermark { .. }
        | Command::ExamineClaim { .. }
        | Command::ListBallots
        | Command::ShowBooks => None,
    }
}

proptest! {
    #[test]
    fn process_of_any_sequence_answers_as_the_model_does(commands in RandomCommands::sequence()) {
        // Arrange
        let mut sides = BothSides::new(Bounds::default());

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
    fn process_of_a_measure_reaches_no_higher_at_a_higher_threshold_nor_lower_for_a_member_further_on(
        pairs in vec((0..6_u64, 0..10_u64), 1..6),
        raised in 0..6_usize,
    ) {
        // Arrange
        let mut members = BTreeSet::new();
        let positions: Vec<MemberPosition> = pairs
            .into_iter()
            .filter(|(member, _)| members.insert(*member))
            .map(|(member, position)| MemberPosition { member, position })
            .collect();
        let mut further = positions.clone();
        let at = raised % further.len();
        further[at].position += 3;
        let mut cartel = fresh();
        let mut measure = |positions: &[MemberPosition], threshold: u64| match cartel
            .process(Command::MeasureWatermark { positions: positions.to_vec(), threshold })
            .output
        {
            Output::WatermarkMeasured { position } => position,
            other => panic!("a watermark, not {other:?}"),
        };

        // Act
        let size = positions.len() as u64;
        let by_threshold: Vec<u64> = (1..=size).map(|threshold| measure(&positions, threshold)).collect();
        let raised_by_threshold: Vec<u64> = (1..=size).map(|threshold| measure(&further, threshold)).collect();

        // Assert
        assert!(by_threshold.windows(2).all(|pair| pair[0] >= pair[1]), "{by_threshold:?}");
        assert!(by_threshold.iter().zip(&raised_by_threshold).all(|(before, after)| after >= before));
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

    #[test]
    fn process_of_any_sequence_under_any_bounds_answers_as_the_model_does(
        bounds in RandomCommands::bounds(),
        commands in RandomCommands::sequence(),
    ) {
        // Arrange
        let mut sides = BothSides::new(bounds);

        // Act
        let departures: Vec<String> = commands
            .iter()
            .chain([&Command::ShowBooks])
            .filter_map(|command| {
                let (from_crate, from_model) = sides.process(command);
                (from_crate != from_model).then(|| format!("{command:?} under {bounds:?}: crate {from_crate}, model {from_model}"))
            })
            .collect();

        // Assert
        assert!(departures.is_empty(), "{departures:#?}");
    }

    #[test]
    fn process_of_any_sequence_under_any_bounds_keeps_the_books_within_them(
        bounds in RandomCommands::bounds(),
        commands in RandomCommands::sequence(),
    ) {
        // Arrange
        let mut cartel = bounded(bounds);

        // Act
        let overflows: Vec<String> = commands
            .into_iter()
            .filter_map(|command| {
                let _ = cartel.process(command.clone());
                let checkpoint = books(&mut cartel);
                let held = within(bounds.open_ballots, checkpoint.ballots.len())
                    && checkpoint.ballots.iter().all(|record| {
                        within(bounds.electorate, record.electorate.len())
                            && record.tally.iter().all(|votes| within(bounds.choice, votes.choice.bytes.len()))
                    });
                (!held).then(|| format!("{command:?} under {bounds:?} left {checkpoint:?}"))
            })
            .collect();

        // Assert
        assert!(overflows.is_empty(), "{overflows:#?}");
    }

    #[test]
    fn process_of_a_claim_after_any_sequence_changes_nothing_but_the_receipt_number(
        commands in RandomCommands::sequence(),
        electorate in vec(0..5_u64, 0..5),
        threshold in 0..6_u64,
        voters in vec(0..6_u64, 0..6),
    ) {
        // Arrange
        let mut cartel = fresh();
        for command in commands {
            let _ = cartel.process(command);
        }
        let before = books(&mut cartel);

        // Act
        let _ = cartel.process(Command::ExamineClaim { electorate, threshold, voters });

        // Assert
        let after = books(&mut cartel);
        assert_eq!(after, Checkpoint { next_receipt: before.next_receipt + 2, ..before });
    }

    #[test]
    fn process_of_a_claim_of_fewer_voters_reaches_no_threshold_the_whole_list_did_not(
        electorate in vec(0..6_u64, 1..6),
        threshold in 1..6_u64,
        voters in vec(0..8_u64, 0..8),
        kept in vec(any::<bool>(), 8),
    ) {
        // Arrange
        let distinct: Vec<u64> = electorate.iter().copied().collect::<BTreeSet<u64>>().into_iter().collect();
        let threshold = threshold.min(distinct.len() as u64);
        let fewer: Vec<u64> = voters.iter().zip(&kept).filter(|(_, keep)| **keep).map(|(voter, _)| *voter).collect();
        let mut cartel = fresh();
        let mut reached = |voters: &[u64]| match cartel
            .process(Command::ExamineClaim { electorate: distinct.clone(), threshold, voters: voters.to_vec() })
            .output
        {
            Output::ClaimExamined { reached, .. } => reached,
            other => panic!("a claim examined, not {other:?}"),
        };

        // Act
        let whole = reached(&voters);
        let subset = reached(&fewer);

        // Assert
        assert!(whole || !subset, "{fewer:?} of {voters:?} reached where the whole did not");
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

// Whether a size is within its bound, a bound left out holding every size.
fn within(bound: Option<u64>, size: usize) -> bool {
    bound.is_none_or(|bound| size as u64 <= bound)
}
