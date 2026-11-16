// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::fixtures::Fixtures;
use cartel::output::Output;
use cartel::rejection::Rejection;
use cartel::rows::watermark_rows::WatermarkRows;

#[test]
fn answer_of_a_threshold_out_of_range_refuses_it() {
    // Arrange
    let positions = Fixtures::positions(&[(1, 5), (2, 3)]);

    // Act
    let answers = [
        WatermarkRows::new(&positions, 0).answer(),
        WatermarkRows::new(&positions, 3).answer(),
        WatermarkRows::new(&[], 1).answer(),
    ];

    // Assert
    assert_eq!(
        answers,
        [
            Output::CommandRejected(Rejection::ThresholdOutOfRange { threshold: 0 }),
            Output::CommandRejected(Rejection::ThresholdOutOfRange { threshold: 3 }),
            Output::CommandRejected(Rejection::ThresholdOutOfRange { threshold: 1 }),
        ]
    );
}

#[test]
fn answer_of_positions_is_the_highest_position_the_threshold_s_members_reached() {
    // Arrange
    let positions = Fixtures::positions(&[(1, 10), (2, 8), (3, 8), (4, 3), (5, 1)]);

    // Act
    let answers = [1, 3, 5].map(|threshold| WatermarkRows::new(&positions, threshold).answer());

    // Assert
    assert_eq!(
        answers,
        [
            Output::WatermarkMeasured { position: 10 },
            Output::WatermarkMeasured { position: 8 },
            Output::WatermarkMeasured { position: 1 },
        ]
    );
}

#[test]
fn answer_of_positions_naming_a_member_twice_refuses_the_lowest_such_member() {
    // Arrange
    let positions = Fixtures::positions(&[(5, 1), (3, 1), (5, 2), (3, 2)]);

    // Act
    let answer = WatermarkRows::new(&positions, 9).answer();

    // Assert
    assert_eq!(
        answer,
        Output::CommandRejected(Rejection::RepeatedMember { member: 3 })
    );
}
