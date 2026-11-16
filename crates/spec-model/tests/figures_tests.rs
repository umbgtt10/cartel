// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::figures::Figures;

#[test]
fn members_of_a_measure_are_the_members_of_its_positions() {
    // Arrange & Act
    let members = Figures::members(&Commands::measure(&[(3, 9), (1, 0)], 2));

    // Assert
    assert_eq!(members, vec![3, 1]);
}

#[test]
fn members_of_an_opening_are_its_electorate() {
    // Arrange & Act
    let members = Figures::members(&Commands::open(States::key(1, 0, 0), &[2, 1], &[1]));

    // Assert
    assert_eq!(members, vec![2, 1]);
}

#[test]
fn thresholds_of_a_measure_are_its_one_threshold() {
    // Arrange & Act
    let thresholds = Figures::thresholds(&Commands::measure(&[(3, 9)], 4));

    // Assert
    assert_eq!(thresholds, vec![4]);
}

#[test]
fn thresholds_of_an_opening_are_those_it_gives() {
    // Arrange & Act
    let thresholds = Figures::thresholds(&Commands::open(States::key(1, 0, 0), &[1, 2], &[2, 1]));

    // Assert
    assert_eq!(thresholds, vec![2, 1]);
}
