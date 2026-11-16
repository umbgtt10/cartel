// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::fixtures::Fixtures;
use cartel::bounds::Bounds;
use cartel::rejection::Rejection;
use cartel::rows::bound_rows::BoundRows;

fn every(open_ballots: u64, electorate: u64, choice: u64) -> BoundRows {
    BoundRows::new(Bounds {
        open_ballots: Some(open_ballots),
        electorate: Some(electorate),
        choice: Some(choice),
    })
}

#[test]
fn books_of_as_many_open_ballots_as_the_bound_are_full_naming_it() {
    // Arrange
    let rows = every(2, 9, 9);

    // Act
    let refused = [rows.books(2), rows.books(1), every(0, 9, 9).books(0)];

    // Assert
    assert_eq!(
        refused,
        [
            Some(Rejection::BooksFull { bound: 2 }),
            None,
            Some(Rejection::BooksFull { bound: 0 }),
        ]
    );
}

#[test]
fn books_without_the_bound_are_never_full() {
    // Arrange & Act
    let refused = BoundRows::new(Bounds::default()).books(usize::MAX);

    // Assert
    assert_eq!(refused, None);
}

#[test]
fn choice_of_more_bytes_than_the_bound_is_too_long_naming_it() {
    // Arrange
    let rows = every(9, 9, 2);

    // Act
    let refused = [&[0xaa, 0xbb, 0xcc][..], &[0xaa, 0xbb], &[]]
        .map(|bytes| rows.choice(&Fixtures::choice(bytes)));

    // Assert
    assert_eq!(
        refused,
        [Some(Rejection::ChoiceTooLong { bound: 2 }), None, None]
    );
}

#[test]
fn choice_without_the_bound_is_never_too_long() {
    // Arrange & Act
    let refused = BoundRows::new(Bounds::default()).choice(&Fixtures::choice(&[0xaa; 1024]));

    // Assert
    assert_eq!(refused, None);
}

#[test]
fn electorate_of_more_members_than_the_bound_is_too_large_naming_it() {
    // Arrange
    let rows = every(9, 3, 9);

    // Act
    let refused = [
        rows.electorate(&[1, 2, 3, 4]),
        rows.electorate(&[1, 2, 3]),
        every(9, 0, 9).electorate(&[1]),
    ];

    // Assert
    assert_eq!(
        refused,
        [
            Some(Rejection::ElectorateTooLarge { bound: 3 }),
            None,
            Some(Rejection::ElectorateTooLarge { bound: 0 }),
        ]
    );
}

#[test]
fn electorate_without_the_bound_is_never_too_large() {
    // Arrange & Act
    let refused = BoundRows::new(Bounds::default()).electorate(&(0..1024).collect::<Vec<u64>>());

    // Assert
    assert_eq!(refused, None);
}
