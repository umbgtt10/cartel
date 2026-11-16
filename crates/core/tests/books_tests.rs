// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::fixtures::Fixtures;
use cartel::books::Books;
use cartel::checkpoint::Checkpoint;
use cartel::output::Output;
use cartel::rejection::Rejection;

#[test]
fn cast_of_a_vote_counts_it_in_that_ballot_alone() {
    // Arrange
    let mut books = Books::new(Fixtures::instance());
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
    let mut books = Books::new(Fixtures::instance());
    let _ = books.open(Fixtures::ballot(1, 0, 0), &[1], &[1]);

    // Act
    let output = books.cast(Fixtures::ballot(1, 0, 1), 9, Fixtures::choice(&[0xaa]));

    // Assert
    assert_eq!(output, Output::CommandRejected(Rejection::BallotNotOpen));
}

#[test]
fn cast_of_a_vote_the_ballot_refuses_changes_nothing() {
    // Arrange
    let mut books = Books::new(Fixtures::instance());
    let _ = books.open(Fixtures::ballot(1, 0, 0), &[1, 2], &[2]);
    let _ = books.cast(Fixtures::ballot(1, 0, 0), 1, Fixtures::choice(&[0xaa]));

    // Act
    let output = books.cast(Fixtures::ballot(1, 0, 0), 1, Fixtures::choice(&[0xaa]));

    // Assert
    assert_eq!(output, Output::CommandRejected(Rejection::AlreadyCounted));
    assert_eq!(books.checkpoint().ballots[0].tally[0].voters, vec![1]);
}

#[test]
fn checkpoint_of_fresh_books_holds_no_ballot_and_numbers_from_one() {
    // Arrange
    let books = Books::new(Fixtures::instance());

    // Act
    let checkpoint = books.checkpoint();

    // Assert
    assert_eq!(
        checkpoint,
        Checkpoint {
            instance: Fixtures::instance(),
            ballots: Vec::new(),
            next_receipt: 1,
        }
    );
}

#[test]
fn count_call_of_each_call_is_one_more_than_the_last() {
    // Arrange
    let mut books = Books::new(Fixtures::instance());

    // Act
    let numbers = [books.count_call(), books.count_call()];

    // Assert
    assert_eq!(numbers, [1, 2]);
    assert_eq!(books.checkpoint().next_receipt, 3);
}

#[test]
fn open_of_a_ballot_lists_it_in_canonical_order() {
    // Arrange
    let mut books = Books::new(Fixtures::instance());

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
    let mut books = Books::new(Fixtures::instance());

    // Act
    let output = books.open(Fixtures::ballot(1, 0, 0), &[], &[1]);

    // Assert
    assert_eq!(output, Output::CommandRejected(Rejection::EmptyElectorate));
    assert!(books.checkpoint().ballots.is_empty());
}
