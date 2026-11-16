// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel_core_validation::seeded_random::SeededRandom;

#[test]
fn below_of_a_bound_is_always_under_it() {
    // Arrange
    let mut random = SeededRandom::new(7);

    // Act
    let drawn: Vec<u64> = (0..1000).map(|_| random.below(6)).collect();

    // Assert
    assert!(drawn.iter().all(|value| *value < 6));
    assert_eq!((0..6).filter(|value| drawn.contains(value)).count(), 6);
}

#[test]
fn chance_of_none_never_happens_and_of_all_always_does() {
    // Arrange
    let mut random = SeededRandom::new(7);

    // Act
    let happened: Vec<(bool, bool)> = (0..100)
        .map(|_| (random.chance(0), random.chance(100)))
        .collect();

    // Assert
    assert!(happened.iter().all(|(never, always)| !never && *always));
}

#[test]
fn draw_of_one_seed_is_the_same_sequence_every_time() {
    // Arrange
    let mut first = SeededRandom::new(42);
    let mut second = SeededRandom::new(42);

    // Act
    let drawn: Vec<(u64, u64)> = (0..10).map(|_| (first.draw(), second.draw())).collect();

    // Assert
    assert!(drawn.iter().all(|(one, other)| one == other));
    assert_ne!(drawn[0].0, drawn[1].0);
}
