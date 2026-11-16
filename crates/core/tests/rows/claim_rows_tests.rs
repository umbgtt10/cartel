// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::output::Output;
use cartel::rejection::Rejection;
use cartel::rows::claim_rows::ClaimRows;

#[test]
fn answer_of_a_claim_is_the_members_counted_the_outsiders_the_repeats_and_whether_they_reach() {
    // Arrange & Act
    let answers = [
        ClaimRows::new(&[4, 1, 2, 3], 3, &[9, 2, 1, 2, 9, 3]).answer(),
        ClaimRows::new(&[1, 2, 3, 4], 3, &[2, 2, 1]).answer(),
    ];

    // Assert
    assert_eq!(
        answers,
        [
            Output::ClaimExamined {
                counted: vec![1, 2, 3],
                outsiders: vec![9],
                repeated: vec![2, 9],
                reached: true,
            },
            Output::ClaimExamined {
                counted: vec![1, 2],
                outsiders: Vec::new(),
                repeated: vec![2],
                reached: false,
            },
        ]
    );
}

#[test]
fn answer_of_a_claim_of_no_voters_counts_none_and_reaches_nothing() {
    // Arrange & Act
    let answer = ClaimRows::new(&[1, 2, 3], 1, &[]).answer();

    // Assert
    assert_eq!(
        answer,
        Output::ClaimExamined {
            counted: Vec::new(),
            outsiders: Vec::new(),
            repeated: Vec::new(),
            reached: false,
        }
    );
}

#[test]
fn answer_of_a_malformed_electorate_or_threshold_refuses_the_first_fault() {
    // Arrange & Act
    let answers = [
        ClaimRows::new(&[], 0, &[1]).answer(),
        ClaimRows::new(&[5, 3, 5, 3], 9, &[3, 3]).answer(),
        ClaimRows::new(&[1, 2], 0, &[1]).answer(),
        ClaimRows::new(&[1, 2], 3, &[1, 2, 3]).answer(),
    ];

    // Assert
    assert_eq!(
        answers,
        [
            Output::CommandRejected(Rejection::EmptyElectorate),
            Output::CommandRejected(Rejection::RepeatedMember { member: 3 }),
            Output::CommandRejected(Rejection::ThresholdOutOfRange { threshold: 0 }),
            Output::CommandRejected(Rejection::ThresholdOutOfRange { threshold: 3 }),
        ]
    );
}
