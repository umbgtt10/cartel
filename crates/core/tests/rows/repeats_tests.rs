// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::rows::repeats::Repeats;

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
