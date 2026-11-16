// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::bounds::Bounds;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::rules::bound_situation::BoundSituation;

fn bounded(bounds: Bounds, open: u64) -> InstanceState {
    InstanceState {
        bounds,
        ..States::holding(
            (0..open)
                .map(|question| States::ballot(States::key(question, 0, 0), &[1], &[1], &[]))
                .collect(),
        )
    }
}

#[test]
fn holds_of_books_full_is_whether_the_open_ballots_number_the_bound() {
    // Arrange
    let opening = Commands::open(States::key(9, 0, 0), &[1], &[1]);
    let at = |bound: u64| Bounds {
        open_ballots: Some(bound),
        ..Bounds::default()
    };

    // Act
    let held = [
        BoundSituation::BooksFull.holds(&opening, &bounded(at(2), 2)),
        BoundSituation::BooksFull.holds(&opening, &bounded(at(2), 1)),
        BoundSituation::BooksFull.holds(&opening, &bounded(at(0), 0)),
    ];

    // Assert
    assert_eq!(held, [true, false, true]);
}

#[test]
fn holds_of_books_full_without_the_bound_is_false() {
    // Arrange
    let opening = Commands::open(States::key(9, 0, 0), &[1], &[1]);

    // Act
    let held = BoundSituation::BooksFull.holds(&opening, &bounded(Bounds::default(), 3));

    // Assert
    assert!(!held);
}

#[test]
fn holds_of_choice_too_long_is_whether_the_choice_has_more_bytes_than_the_bound() {
    // Arrange
    let state = bounded(
        Bounds {
            choice: Some(2),
            ..Bounds::default()
        },
        1,
    );

    // Act
    let held = [&[0xaa, 0xbb, 0xcc][..], &[0xaa, 0xbb], &[]].map(|choice| {
        BoundSituation::ChoiceTooLong
            .holds(&Commands::cast(States::key(0, 0, 0), 1, choice), &state)
    });

    // Assert
    assert_eq!(held, [true, false, false]);
}

#[test]
fn holds_of_choice_too_long_without_the_bound_is_false() {
    // Arrange
    let vote = Commands::cast(States::key(0, 0, 0), 1, &[0xaa; 64]);

    // Act
    let held = BoundSituation::ChoiceTooLong.holds(&vote, &bounded(Bounds::default(), 1));

    // Assert
    assert!(!held);
}

#[test]
fn holds_of_electorate_too_large_is_whether_the_electorate_names_more_members_than_the_bound() {
    // Arrange
    let state = bounded(
        Bounds {
            electorate: Some(2),
            ..Bounds::default()
        },
        0,
    );

    // Act
    let held = [&[1, 2, 3][..], &[1, 2], &[1]].map(|electorate| {
        BoundSituation::ElectorateTooLarge.holds(
            &Commands::open(States::key(1, 0, 0), electorate, &[1]),
            &state,
        )
    });

    // Assert
    assert_eq!(held, [true, false, false]);
}

#[test]
fn holds_of_electorate_too_large_without_the_bound_is_false() {
    // Arrange
    let opening = Commands::open(States::key(1, 0, 0), &[1, 2, 3, 4, 5], &[1]);

    // Act
    let held = BoundSituation::ElectorateTooLarge.holds(&opening, &bounded(Bounds::default(), 0));

    // Assert
    assert!(!held);
}
