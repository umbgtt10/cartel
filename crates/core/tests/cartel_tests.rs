// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::fixtures::Fixtures;
use crate::support::recording_observer::Calls;
use crate::support::recording_observer::RecordingObserver;
use cartel::bounds::Bounds;
use cartel::cartel::Cartel;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::no_op_observer::NoOpObserver;
use cartel::origin::Origin;
use cartel::output::Output;
use cartel::receipt::Receipt;
use cartel::rejection::Rejection;

fn fresh() -> Cartel {
    Cartel::new(
        Origin::Fresh(Fresh {
            instance: Fixtures::instance(),
            bounds: Bounds::default(),
        }),
        Box::new(NoOpObserver),
    )
}

fn open(ballot: (u64, u64, u64), electorate: &[u64], thresholds: &[u64]) -> Command {
    Command::OpenBallot {
        ballot: Fixtures::ballot(ballot.0, ballot.1, ballot.2),
        electorate: electorate.to_vec(),
        thresholds: thresholds.to_vec(),
    }
}

fn vote(member: u64, choice: &[u8]) -> Command {
    Command::CastVote {
        ballot: Fixtures::ballot(1, 0, 0),
        member,
        choice: Fixtures::choice(choice),
    }
}

#[test]
fn new_of_a_bounded_origin_answers_within_its_bounds() {
    // Arrange
    let bounds = Bounds {
        open_ballots: Some(1),
        electorate: Some(2),
        choice: Some(1),
    };
    let mut cartel = Cartel::new(
        Origin::Fresh(Fresh {
            instance: Fixtures::instance(),
            bounds,
        }),
        Box::new(NoOpObserver),
    );

    // Act
    let outputs: Vec<Output> = [
        open((1, 0, 0), &[1, 2, 3], &[1]),
        open((1, 0, 0), &[1, 2], &[2]),
        open((2, 0, 0), &[1], &[1]),
        vote(1, &[0xaa, 0xbb]),
        vote(1, &[0xaa]),
    ]
    .into_iter()
    .map(|command| cartel.process(command).output)
    .collect();

    // Assert
    assert_eq!(
        outputs,
        vec![
            Output::CommandRejected(Rejection::ElectorateTooLarge { bound: 2 }),
            Output::BallotOpened,
            Output::CommandRejected(Rejection::BooksFull { bound: 1 }),
            Output::CommandRejected(Rejection::ChoiceTooLong { bound: 1 }),
            Output::VoteCounted { count: 1 },
        ]
    );
}

#[test]
fn new_of_a_fresh_origin_holds_no_ballot() {
    // Arrange
    let mut cartel = fresh();

    // Act
    let receipt = cartel.process(Command::ListBallots);

    // Assert
    assert_eq!(
        receipt,
        Receipt {
            number: 1,
            output: Output::BallotsListed {
                ballots: Vec::new()
            },
        }
    );
}

#[test]
fn process_of_a_close_closes_the_ballots_below_its_point() {
    // Arrange
    let mut cartel = fresh();
    let _ = cartel.process(open((1, 0, 0), &[1], &[1]));

    // Act
    let receipt = cartel.process(Command::CloseBallots {
        before: Fixtures::point(1, 1),
    });

    // Assert
    assert_eq!(
        receipt.output,
        Output::BallotsClosed {
            ballots: vec![Fixtures::ballot(1, 0, 0)],
        }
    );
}

#[test]
fn process_of_a_measure_answers_the_watermark_and_changes_no_ballot() {
    // Arrange
    let mut cartel = fresh();
    let _ = cartel.process(open((1, 0, 0), &[1, 2], &[2]));
    let before = cartel.process(Command::ListBallots).output;

    // Act
    let receipt = cartel.process(Command::MeasureWatermark {
        positions: Fixtures::positions(&[(1, 9), (2, 4)]),
        threshold: 2,
    });

    // Assert
    assert_eq!(receipt.output, Output::WatermarkMeasured { position: 4 });
    assert_eq!(cartel.process(Command::ListBallots).output, before);
}

#[test]
fn process_of_a_refused_command_answers_its_rejection_and_changes_nothing() {
    // Arrange
    let mut cartel = fresh();
    let _ = cartel.process(open((1, 0, 0), &[1, 2], &[2]));
    let _ = cartel.process(vote(1, &[0xaa]));
    let before = cartel.process(Command::ShowBooks);

    // Act
    let receipt = cartel.process(vote(1, &[0xbb]));

    // Assert
    assert_eq!(
        receipt.output,
        Output::CommandRejected(Rejection::Equivocation {
            counted: Fixtures::choice(&[0xaa]),
        })
    );
    let Output::BooksShown(after) = cartel.process(Command::ShowBooks).output else {
        panic!("the books shown");
    };
    let Output::BooksShown(before) = before.output else {
        panic!("the books shown");
    };
    assert_eq!(after.ballots, before.ballots);
}

#[test]
fn process_of_a_vote_that_reaches_a_threshold_answers_the_quorum() {
    // Arrange
    let mut cartel = fresh();
    let _ = cartel.process(open((1, 0, 0), &[1, 2, 3], &[2]));
    let _ = cartel.process(vote(3, &[0xaa]));

    // Act
    let receipt = cartel.process(vote(1, &[0xaa]));

    // Assert
    assert_eq!(
        receipt.output,
        Output::QuorumReached {
            count: 2,
            thresholds: vec![2],
            voters: vec![1, 3],
        }
    );
}

#[test]
fn process_of_an_open_ballot_opens_it() {
    // Arrange
    let mut cartel = fresh();

    // Act
    let receipt = cartel.process(open((1, 0, 0), &[1], &[1]));

    // Assert
    assert_eq!(receipt.output, Output::BallotOpened);
}

#[test]
fn process_of_every_command_reports_it_with_its_receipt() {
    // Arrange
    let calls = Calls::default();
    let mut cartel = Cartel::new(
        Origin::Fresh(Fresh {
            instance: Fixtures::instance(),
            bounds: Bounds::default(),
        }),
        Box::new(RecordingObserver::new(calls.clone())),
    );

    // Act
    let receipts = [
        cartel.process(Command::ShowBallot {
            ballot: Fixtures::ballot(1, 0, 0),
        }),
        cartel.process(Command::ShowBooks),
    ];

    // Assert
    let reported: Vec<Receipt> = calls
        .borrow()
        .iter()
        .map(|(_, receipt)| receipt.clone())
        .collect();
    assert_eq!(reported, receipts.to_vec());
    assert_eq!(calls.borrow()[1].0, Command::ShowBooks);
}
