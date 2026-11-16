// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The memory tier at the bounds: an instance filled to every bound it is
// given -- the most ballots, each over the largest electorate, every member
// voting a choice of its own of the longest length -- and then flooded with
// every command that would exceed a bound, measured between the calls: the
// books never hold more than they did full, hold exactly that after any
// flood, and hold more full under larger bounds.

use crate::support::installed::ALLOCATOR;
use cartel_memory::bounded_flood::BoundedFlood;

const ROUNDS: u64 = 1000;

#[test]
fn run_at_larger_bounds_holds_more_when_full() {
    // Arrange
    let smaller = BoundedFlood::new(&ALLOCATOR, 4, 7, 32);
    let larger = BoundedFlood::new(&ALLOCATOR, 8, 24, 64);

    // Act
    let profiles = [smaller.run(1), larger.run(1)];

    // Assert
    let [smaller, larger] = profiles;
    assert!(smaller.full > 0, "the counter never moved: {smaller:?}");
    assert!(larger.full > smaller.full, "{larger:?}, {smaller:?}");
}

#[test]
fn run_of_a_flood_holds_after_it_what_the_books_held_full() {
    // Arrange
    let flood = BoundedFlood::new(&ALLOCATOR, 4, 7, 32);

    // Act
    let profile = flood.run(ROUNDS);

    // Assert
    assert_eq!(profile.after_flood, profile.full, "{profile:?}");
}

#[test]
fn run_of_a_flood_never_holds_more_than_the_books_full() {
    // Arrange
    let flood = BoundedFlood::new(&ALLOCATOR, 4, 7, 32);

    // Act
    let profile = flood.run(ROUNDS);

    // Assert
    assert!(profile.most_while_flooded <= profile.full, "{profile:?}");
}
