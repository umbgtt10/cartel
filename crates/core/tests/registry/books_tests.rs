// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::fixtures::Fixtures;
use cartel::bounds::Bounds;
use cartel::checkpoint::Checkpoint;
use cartel::output::Output;
use cartel::registry::books::Books;
use cartel::rejection::Rejection;

#[test]
fn cast_of_a_choice_longer_than_its_bound_is_refused_before_the_ballot() {
    // Arrange
    let mut books = Books::new(
        Fixtures::instance(),
        Bounds {
            choice: Some(1),
            ..Bounds::default()
        },
    );
    let _ = books.open(Fixtures::ballot(1, 0, 0), &[1], &[1]);

    // Act
    let outputs = [
        books.cast(
            Fixtures::ballot(2, 0, 0),
            1,
            Fixtures::choice(&[0xaa, 0xbb]),
        ),
        books.cast(
            Fixtures::ballot(1, 0, 0),
            9,
            Fixtures::choice(&[0xaa, 0xbb]),
        ),
        books.cast(Fixtures::ballot(1, 0, 0), 1, Fixtures::choice(&[0xaa])),
    ];

    // Assert
    assert_eq!(
        outputs,
        [
            Output::CommandRejected(Rejection::ChoiceTooLong { bound: 1 }),
            Output::CommandRejected(Rejection::ChoiceTooLong { bound: 1 }),
            Output::QuorumReached {
                count: 1,
                thresholds: vec![1],
                voters: vec![1],
            },
        ]
    );
}

#[test]
fn cast_of_a_vote_below_the_close_point_is_refused_as_closed() {
    // Arrange
    let mut books = Books::new(Fixtures::instance(), Bounds::default());
    let _ = books.open(Fixtures::ballot(1, 0, 0), &[1], &[1]);
    let _ = books.close(Fixtures::point(1, 1));

    // Act
    let outputs = [
        books.cast(Fixtures::ballot(1, 0, 0), 1, Fixtures::choice(&[0xaa])),
        books.cast(Fixtures::ballot(1, 1, 0), 1, Fixtures::choice(&[0xaa])),
    ];

    // Assert
    assert_eq!(
        outputs,
        [
            Output::CommandRejected(Rejection::BallotClosed),
            Output::CommandRejected(Rejection::BallotNotOpen),
        ]
    );
}

#[test]
fn cast_of_a_vote_counts_it_in_that_ballot_alone() {
    // Arrange
    let mut books = Books::new(Fixtures::instance(), Bounds::default());
    let _ = books.open(Fixtures::ballot(1, 0, 0), &[1, 2], &[2]);
    let _ = books.open(Fixtures::ballot(2, 0, 0), &[1, 2], &[2]);

    // Act
    let output = books.cast(Fixtures::ballot(1, 0, 0), 2, Fixtures::choice(&[0xaa]));

    // Assert
    assert_eq!(output, Output::VoteCounted { count: 1 });
    assert!(books.checkpoint().ballots[1].tally.is_empty());
}

#[test]
fn cast_of_a_vote_in_a_ballot_not_open_is_refused() {
    // Arrange
    let mut books = Books::new(Fixtures::instance(), Bounds::default());
    let _ = books.open(Fixtures::ballot(1, 0, 0), &[1], &[1]);

    // Act
    let output = books.cast(Fixtures::ballot(1, 0, 1), 9, Fixtures::choice(&[0xaa]));

    // Assert
    assert_eq!(output, Output::CommandRejected(Rejection::BallotNotOpen));
}

#[test]
fn cast_of_a_vote_the_ballot_refuses_changes_nothing() {
    // Arrange
    let mut books = Books::new(Fixtures::instance(), Bounds::default());
    let _ = books.open(Fixtures::ballot(1, 0, 0), &[1, 2], &[2]);
    let _ = books.cast(Fixtures::ballot(1, 0, 0), 1, Fixtures::choice(&[0xaa]));

    // Act
    let output = books.cast(Fixtures::ballot(1, 0, 0), 1, Fixtures::choice(&[0xaa]));

    // Assert
    assert_eq!(output, Output::CommandRejected(Rejection::AlreadyCounted));
    assert_eq!(books.checkpoint().ballots[0].tally[0].voters, vec![1]);
}

#[test]
fn checkpoint_of_bounded_books_holds_their_bounds() {
    // Arrange
    let bounds = Bounds {
        open_ballots: Some(4),
        electorate: None,
        choice: Some(0),
    };
    let books = Books::new(Fixtures::instance(), bounds);

    // Act
    let checkpoint = books.checkpoint();

    // Assert
    assert_eq!(checkpoint.bounds, bounds);
}

#[test]
fn checkpoint_of_fresh_books_holds_no_ballot_and_numbers_from_one() {
    // Arrange
    let books = Books::new(Fixtures::instance(), Bounds::default());

    // Act
    let checkpoint = books.checkpoint();

    // Assert
    assert_eq!(
        checkpoint,
        Checkpoint {
            instance: Fixtures::instance(),
            bounds: Bounds::default(),
            close_point: None,
            ballots: Vec::new(),
            next_receipt: 1,
        }
    );
}

#[test]
fn close_at_or_behind_the_close_point_is_refused_naming_it() {
    // Arrange
    let mut books = Books::new(Fixtures::instance(), Bounds::default());
    let _ = books.close(Fixtures::point(5, 2));

    // Act
    let outputs = [
        books.close(Fixtures::point(5, 2)),
        books.close(Fixtures::point(4, 9)),
    ];

    // Assert
    let refused = Output::CommandRejected(Rejection::CloseBehind {
        close_point: Fixtures::point(5, 2),
    });
    assert_eq!(outputs, [refused.clone(), refused]);
    assert_eq!(books.checkpoint().close_point, Some(Fixtures::point(5, 2)));
}

#[test]
fn close_of_a_point_closes_every_ballot_below_it_and_moves_the_point() {
    // Arrange
    let mut books = Books::new(Fixtures::instance(), Bounds::default());
    for ballot in [
        Fixtures::ballot(2, 1, 0),
        Fixtures::ballot(2, 0, 9),
        Fixtures::ballot(1, 7, 0),
    ] {
        let _ = books.open(ballot, &[1], &[1]);
    }

    // Act
    let output = books.close(Fixtures::point(2, 1));

    // Assert
    assert_eq!(
        output,
        Output::BallotsClosed {
            ballots: vec![Fixtures::ballot(1, 7, 0), Fixtures::ballot(2, 0, 9)],
        }
    );
    let checkpoint = books.checkpoint();
    assert_eq!(checkpoint.close_point, Some(Fixtures::point(2, 1)));
    let open: Vec<_> = checkpoint
        .ballots
        .into_iter()
        .map(|record| record.ballot)
        .collect();
    assert_eq!(open, vec![Fixtures::ballot(2, 1, 0)]);
}

#[test]
fn count_call_of_each_call_is_one_more_than_the_last() {
    // Arrange
    let mut books = Books::new(Fixtures::instance(), Bounds::default());

    // Act
    let numbers = [books.count_call(), books.count_call()];

    // Assert
    assert_eq!(numbers, [1, 2]);
    assert_eq!(books.checkpoint().next_receipt, 3);
}

#[test]
fn open_of_a_ballot_at_full_books_is_refused_until_a_close_frees_one() {
    // Arrange
    let mut books = Books::new(
        Fixtures::instance(),
        Bounds {
            open_ballots: Some(1),
            ..Bounds::default()
        },
    );
    let _ = books.open(Fixtures::ballot(1, 0, 0), &[1], &[1]);

    // Act
    let refused = books.open(Fixtures::ballot(2, 0, 0), &[1], &[1]);
    let _ = books.close(Fixtures::point(2, 0));
    let opened = books.open(Fixtures::ballot(2, 0, 0), &[1], &[1]);

    // Assert
    assert_eq!(
        [refused, opened],
        [
            Output::CommandRejected(Rejection::BooksFull { bound: 1 }),
            Output::BallotOpened,
        ]
    );
}

#[test]
fn open_of_a_ballot_lists_it_in_canonical_order() {
    // Arrange
    let mut books = Books::new(Fixtures::instance(), Bounds::default());

    // Act
    let _ = books.open(Fixtures::ballot(2, 0, 0), &[1], &[1]);
    let _ = books.open(Fixtures::ballot(1, 1, 0), &[1], &[1]);

    // Assert
    let ballots: Vec<_> = books
        .checkpoint()
        .ballots
        .into_iter()
        .map(|record| record.ballot)
        .collect();
    assert_eq!(
        ballots,
        vec![Fixtures::ballot(1, 1, 0), Fixtures::ballot(2, 0, 0)]
    );
}

#[test]
fn open_of_a_ballot_the_rows_refuse_opens_nothing() {
    // Arrange
    let mut books = Books::new(Fixtures::instance(), Bounds::default());

    // Act
    let output = books.open(Fixtures::ballot(1, 0, 0), &[], &[1]);

    // Assert
    assert_eq!(output, Output::CommandRejected(Rejection::EmptyElectorate));
    assert!(books.checkpoint().ballots.is_empty());
}
