// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The memory tier: ballots opened and voted one after another on one instance,
// measured by a counting allocator where it sees every layout -- the books
// holding what their votes need and nothing for the refusals and queries met
// between them, more for more members, and nothing once the instance is gone.

use cartel_memory::ballot_sequence::BallotSequence;
use cartel_memory::counting_allocator::CountingAllocator;

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;
const BALLOTS: u64 = 100;

// A B-tree holds its entries eleven to a node, so four members and seven hold
// alike; twenty-four need more nodes.
#[test]
fn run_of_ballots_holds_more_for_more_members() {
    // Arrange
    let fewer = BallotSequence::new(&ALLOCATOR, 4, false);
    let more = BallotSequence::new(&ALLOCATOR, 24, false);

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
    let sequence = BallotSequence::new(&ALLOCATOR, 7, true);

    // Act
    let profile = sequence.run(BALLOTS);

    // Assert
    assert_eq!(profile.after_drop, 0, "{profile:?}");
}

#[test]
fn run_with_refusals_and_queries_between_the_votes_holds_what_the_votes_alone_hold() {
    // Arrange
    let quiet = BallotSequence::new(&ALLOCATOR, 7, false);
    let busy = BallotSequence::new(&ALLOCATOR, 7, true);

    // Act
    let profiles = [quiet.run(BALLOTS), busy.run(BALLOTS)];

    // Assert
    let [quiet, busy] = profiles;
    assert_eq!(
        busy.held_after_all, quiet.held_after_all,
        "{busy:?}, {quiet:?}"
    );
}
