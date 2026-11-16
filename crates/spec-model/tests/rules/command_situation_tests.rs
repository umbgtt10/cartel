// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::rules::command_situation::CommandSituation;

fn held(situation: CommandSituation, cases: [(&[u64], &[u64]); 2]) -> [bool; 2] {
    cases.map(|(electorate, thresholds)| {
        situation.holds(&Commands::open(
            States::key(1, 0, 0),
            electorate,
            thresholds,
        ))
    })
}

#[test]
fn holds_of_empty_electorate_is_whether_no_member_is_given() {
    // Arrange & Act
    let held = held(
        CommandSituation::EmptyElectorate,
        [(&[], &[1]), (&[1], &[1])],
    );

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_no_thresholds_is_whether_no_threshold_is_given() {
    // Arrange & Act
    let held = held(CommandSituation::NoThresholds, [(&[1], &[]), (&[1], &[1])]);

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_repeated_member_for_a_measure_is_whether_its_positions_name_a_member_twice() {
    // Arrange & Act
    let held = [
        CommandSituation::RepeatedMember.holds(&Commands::measure(&[(1, 5), (2, 3), (1, 9)], 2)),
        CommandSituation::RepeatedMember.holds(&Commands::measure(&[(1, 5), (2, 3)], 2)),
    ];

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_repeated_member_is_whether_a_member_is_given_twice() {
    // Arrange & Act
    let held = held(
        CommandSituation::RepeatedMember,
        [(&[2, 1, 2], &[1]), (&[2, 1], &[1])],
    );

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_repeated_threshold_is_whether_a_threshold_is_given_twice() {
    // Arrange & Act
    let held = held(
        CommandSituation::RepeatedThreshold,
        [(&[1, 2], &[1, 1]), (&[1, 2], &[2, 1])],
    );

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn holds_of_threshold_out_of_range_for_a_measure_is_whether_it_is_zero_or_above_its_members() {
    // Arrange & Act
    let held = [0, 3, 2].map(|threshold| {
        CommandSituation::ThresholdOutOfRange
            .holds(&Commands::measure(&[(1, 5), (2, 3)], threshold))
    });

    // Assert
    assert_eq!(held, [true, true, false]);
}

#[test]
fn holds_of_threshold_out_of_range_is_whether_a_threshold_is_zero_or_above_the_electorate() {
    // Arrange & Act
    let held = held(
        CommandSituation::ThresholdOutOfRange,
        [(&[1, 2], &[3]), (&[1, 2], &[2])],
    );

    // Assert
    assert_eq!(held, [true, false]);
}
