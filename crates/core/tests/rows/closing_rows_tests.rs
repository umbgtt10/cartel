// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::fixtures::Fixtures;
use cartel::rejection::Rejection;
use cartel::rows::closing_rows::ClosingRows;

#[test]
fn covers_of_a_point_is_whether_a_ballot_s_question_and_round_come_before_it() {
    // Arrange
    let rows = ClosingRows::new(Some(Fixtures::point(2, 1)));

    // Act
    let covered = [
        Fixtures::ballot(2, 0, 9),
        Fixtures::ballot(2, 1, 0),
        Fixtures::ballot(1, 9, 9),
        Fixtures::ballot(3, 0, 0),
    ]
    .map(|ballot| rows.covers(ballot));

    // Assert
    assert_eq!(covered, [true, false, true, false]);
}

#[test]
fn covers_of_books_never_closed_is_false_for_every_ballot() {
    // Arrange
    let rows = ClosingRows::new(None);

    // Act & Assert
    assert!(!rows.covers(Fixtures::ballot(0, 0, 0)));
}

#[test]
fn refusal_of_a_close_at_or_behind_the_point_names_it_and_of_one_ahead_is_none() {
    // Arrange
    let rows = ClosingRows::new(Some(Fixtures::point(5, 2)));

    // Act
    let refused = [
        rows.refusal(Fixtures::point(5, 2)),
        rows.refusal(Fixtures::point(4, 9)),
        rows.refusal(Fixtures::point(5, 3)),
    ];

    // Assert
    let behind = Some(Rejection::CloseBehind {
        close_point: Fixtures::point(5, 2),
    });
    assert_eq!(refused, [behind.clone(), behind, None]);
}

#[test]
fn refusal_of_any_close_on_books_never_closed_is_none() {
    // Arrange & Act
    let refused = ClosingRows::new(None).refusal(Fixtures::point(0, 0));

    // Assert
    assert_eq!(refused, None);
}
