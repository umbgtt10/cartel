// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::fixtures::Fixtures;
use cartel::open_ballot::OpenBallot;
use cartel::rejection::Rejection;
use cartel::rows::closing_rows::ClosingRows;
use cartel::rows::opening_rows::OpeningRows;
use std::collections::BTreeMap;

// Ballot (1, 0, 0) open; the opening asked for is of that ballot.
fn refusal(electorate: &[u64], thresholds: &[u64]) -> Option<Rejection> {
    let mut ballots = BTreeMap::new();
    ballots.insert(
        Fixtures::ballot(1, 0, 0),
        OpenBallot::new(Fixtures::set(&[1]), Fixtures::set(&[1])),
    );
    OpeningRows::new(&ballots, ClosingRows::new(None)).refusal(
        Fixtures::ballot(1, 0, 0),
        electorate,
        thresholds,
    )
}

#[test]
fn refusal_of_a_ballot_below_the_close_point_is_closed_once_its_form_is_checked() {
    // Arrange
    let ballots = BTreeMap::new();
    let rows = OpeningRows::new(&ballots, ClosingRows::new(Some(Fixtures::point(3, 0))));

    // Act
    let refused = [
        rows.refusal(Fixtures::ballot(2, 5, 0), &[1], &[1]),
        rows.refusal(Fixtures::ballot(2, 5, 0), &[], &[1]),
        rows.refusal(Fixtures::ballot(3, 0, 0), &[1], &[1]),
    ];

    // Assert
    assert_eq!(
        refused,
        [
            Some(Rejection::BallotClosed),
            Some(Rejection::EmptyElectorate),
            None
        ]
    );
}

#[test]
fn refusal_of_a_ballot_not_open_and_well_formed_is_none() {
    // Arrange
    let ballots = BTreeMap::new();

    // Act
    let refused = OpeningRows::new(&ballots, ClosingRows::new(None)).refusal(
        Fixtures::ballot(2, 0, 0),
        &[3, 1, 2],
        &[3, 2],
    );

    // Assert
    assert_eq!(refused, None);
}

#[test]
fn refusal_of_an_electorate_naming_members_twice_names_the_lowest() {
    // Arrange & Act
    let refused = refusal(&[5, 3, 5, 3, 9], &[]);

    // Assert
    assert_eq!(refused, Some(Rejection::RepeatedMember { member: 3 }));
}

#[test]
fn refusal_of_an_empty_electorate_comes_first() {
    // Arrange & Act
    let refused = refusal(&[], &[]);

    // Assert
    assert_eq!(refused, Some(Rejection::EmptyElectorate));
}

#[test]
fn refusal_of_an_open_ballot_well_formed_is_already_open() {
    // Arrange & Act
    let refused = refusal(&[1, 2], &[2]);

    // Assert
    assert_eq!(refused, Some(Rejection::BallotAlreadyOpen));
}

#[test]
fn refusal_of_no_thresholds_comes_before_the_ballot() {
    // Arrange & Act
    let refused = refusal(&[1], &[]);

    // Assert
    assert_eq!(refused, Some(Rejection::NoThresholds));
}

#[test]
fn refusal_of_thresholds_given_twice_names_the_lowest() {
    // Arrange & Act
    let refused = refusal(&[1, 2, 3, 4], &[3, 1, 3, 1]);

    // Assert
    assert_eq!(refused, Some(Rejection::RepeatedThreshold { threshold: 1 }));
}

#[test]
fn refusal_of_thresholds_out_of_range_names_the_lowest_before_any_repeat() {
    // Arrange & Act
    let refused = [
        refusal(&[1, 2, 3], &[6, 2, 4]),
        refusal(&[1, 2], &[0]),
        refusal(&[1, 2, 3], &[5, 5]),
    ];

    // Assert
    assert_eq!(
        refused,
        [
            Some(Rejection::ThresholdOutOfRange { threshold: 4 }),
            Some(Rejection::ThresholdOutOfRange { threshold: 0 }),
            Some(Rejection::ThresholdOutOfRange { threshold: 5 }),
        ]
    );
}
