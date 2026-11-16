// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The memory tier: ballots opened and voted one after another on one instance,
// measured by a counting allocator where it sees every layout -- the books
// holding what their votes need and nothing for the refusals and queries met
// between them, more for more members, no more after a hundred ballots than
// after one when each is closed once counted, and nothing once the instance is gone.

use cartel_memory::ballot_sequence::BallotSequence;
use cartel_memory::counting_allocator::CountingAllocator;

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;
const BALLOTS: u64 = 100;

// A close splits the books' map, and the part it keeps may hold an empty node,
// so closing everything is not an empty instance; what closing promises is
// that the books stop growing.
#[test]
fn run_of_ballots_closed_as_they_go_holds_as_much_after_a_hundred_as_after_one() {
    // Arrange
    let sequence = BallotSequence::new(&ALLOCATOR, 24, true, true);

    // Act
    let profiles = [sequence.run(1), sequence.run(BALLOTS)];

    // Assert
    let [one, hundred] = profiles;
    assert_eq!(
        hundred.held_after_all, one.held_after_all,
        "{hundred:?}, {one:?}"
    );
}

// A B-tree holds its entries eleven to a node, so four members and seven hold
// alike; twenty-four need more nodes.
#[test]
fn run_of_ballots_holds_more_for_more_members() {
    // Arrange
    let fewer = BallotSequence::new(&ALLOCATOR, 4, false, false);
    let more = BallotSequence::new(&ALLOCATOR, 24, false, false);

    // Act
    let profiles = [fewer.run(BALLOTS), more.run(BALLOTS)];

    // Assert
    let [fewer, more] = profiles;
    assert!(
        fewer.held_after_all > fewer.skeleton,
        "the counter never moved: {fewer:?}"
    );
    assert!(
        more.held_after_all > fewer.held_after_all,
        "{more:?}, {fewer:?}"
    );
}

#[test]
fn run_of_ballots_leaves_nothing_once_the_instance_is_gone() {
    // Arrange
    let sequence = BallotSequence::new(&ALLOCATOR, 7, true, false);

    // Act
    let profile = sequence.run(BALLOTS);

    // Assert
    assert_eq!(profile.after_drop, 0, "{profile:?}");
}

#[test]
fn run_with_refusals_and_queries_between_the_votes_holds_what_the_votes_alone_hold() {
    // Arrange
    let quiet = BallotSequence::new(&ALLOCATOR, 7, false, false);
    let busy = BallotSequence::new(&ALLOCATOR, 7, true, false);

    // Act
    let profiles = [quiet.run(BALLOTS), busy.run(BALLOTS)];

    // Assert
    let [quiet, busy] = profiles;
    assert_eq!(
        busy.held_after_all, quiet.held_after_all,
        "{busy:?}, {quiet:?}"
    );
}
