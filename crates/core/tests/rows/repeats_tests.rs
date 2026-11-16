// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::rows::repeats::Repeats;
use std::collections::BTreeSet;

#[test]
fn all_of_numbers_is_every_one_given_more_than_once_in_ascending_order() {
    // Arrange & Act
    let all = [
        Repeats::all(&[9, 5, 3, 5, 3, 9, 1]),
        Repeats::all(&[3, 1, 2]),
    ];

    // Assert
    assert_eq!(all, [BTreeSet::from([3, 5, 9]), BTreeSet::new()]);
}

#[test]
fn lowest_of_distinct_numbers_is_none() {
    // Arrange & Act
    let lowest = Repeats::lowest(&[3, 1, 2]);

    // Assert
    assert_eq!(lowest, None);
}

#[test]
fn lowest_of_numbers_given_more_than_once_is_the_lowest_of_them() {
    // Arrange & Act
    let lowest = Repeats::lowest(&[5, 3, 5, 3, 9]);

    // Assert
    assert_eq!(lowest, Some(3));
}
