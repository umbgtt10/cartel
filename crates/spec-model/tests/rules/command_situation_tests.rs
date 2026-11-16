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
fn holds_of_threshold_out_of_range_is_whether_a_threshold_is_zero_or_above_the_electorate() {
    // Arrange & Act
    let held = held(
        CommandSituation::ThresholdOutOfRange,
        [(&[1, 2], &[3]), (&[1, 2], &[2])],
    );

    // Assert
    assert_eq!(held, [true, false]);
}
