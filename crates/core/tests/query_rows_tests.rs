// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::fixtures::Fixtures;
use cartel::books::Books;
use cartel::output::Output;
use cartel::rejection::Rejection;

fn books() -> Books {
    let mut books = Books::new(Fixtures::instance());
    let _ = books.open(Fixtures::ballot(2, 0, 0), &[1], &[1]);
    let _ = books.open(Fixtures::ballot(1, 0, 1), &[1], &[1]);
    books
}

#[test]
fn ballot_of_a_ballot_not_open_is_refused() {
    // Arrange
    let books = books();

    // Act
    let output = books.query_rows().ballot(Fixtures::ballot(1, 0, 0));

    // Assert
    assert_eq!(output, Output::CommandRejected(Rejection::BallotNotOpen));
}

#[test]
fn ballot_of_an_open_ballot_shows_its_record() {
    // Arrange
    let books = books();

    // Act
    let output = books.query_rows().ballot(Fixtures::ballot(2, 0, 0));

    // Assert
    let Output::BallotShown(record) = output else {
        panic!("a record shown, not {output:?}");
    };
    assert_eq!(record.ballot, Fixtures::ballot(2, 0, 0));
}

#[test]
fn ballots_of_the_books_lists_the_open_ballots_in_canonical_order() {
    // Arrange
    let books = books();

    // Act
    let output = books.query_rows().ballots();

    // Assert
    assert_eq!(
        output,
        Output::BallotsListed {
            ballots: vec![Fixtures::ballot(1, 0, 1), Fixtures::ballot(2, 0, 0)],
        }
    );
}
