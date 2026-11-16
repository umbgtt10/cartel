// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel_spec_model::numbers::Numbers;

#[test]
fn lowest_out_of_range_of_numbers_all_in_range_is_none() {
    // Arrange & Act
    let lowest = Numbers::lowest_out_of_range(&[1, 3, 2], 3);

    // Assert
    assert_eq!(lowest, None);
}

#[test]
fn lowest_out_of_range_of_numbers_is_the_lowest_zero_or_above_the_size() {
    // Arrange & Act
    let lowest = [
        Numbers::lowest_out_of_range(&[6, 2, 4], 3),
        Numbers::lowest_out_of_range(&[5, 0], 3),
    ];

    // Assert
    assert_eq!(lowest, [Some(4), Some(0)]);
}

#[test]
fn lowest_repeated_of_distinct_numbers_is_none() {
    // Arrange & Act
    let lowest = Numbers::lowest_repeated(&[3, 1, 2]);

    // Assert
    assert_eq!(lowest, None);
}

#[test]
fn lowest_repeated_of_numbers_is_the_lowest_named_more_than_once() {
    // Arrange & Act
    let lowest = Numbers::lowest_repeated(&[5, 3, 5, 3, 9]);

    // Assert
    assert_eq!(lowest, Some(3));
}
