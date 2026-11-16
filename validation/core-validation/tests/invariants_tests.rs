// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The cross-node invariants, each shown holding on sightings that keep it and
// firing on a crafted violation, so a check that never fires cannot pass for
// one that holds.

use cartel::choice::Choice;
use cartel_core_validation::cluster_invariants::ClusterInvariants;
use cartel_core_validation::sighting::Sighting;
use cartel_core_validation::violation::Violation;

fn choice(byte: u8) -> Choice {
    Choice { bytes: vec![byte] }
}

fn sighting(node: u64, byte: u8, threshold: u64, voters: &[u64]) -> Sighting {
    Sighting {
        node,
        incarnation: 1,
        choice: choice(byte),
        threshold,
        voters: voters.to_vec(),
    }
}

#[test]
fn check_of_a_choice_no_member_cast_is_an_uncast_choice() {
    // Arrange
    let cast = [choice(0xaa)];
    let invariants = ClusterInvariants::new(4, 1, &cast);
    let sightings = [sighting(1, 0xcc, 3, &[1, 2, 3])];

    // Act
    let checked = invariants.check(&sightings);

    // Assert
    assert_eq!(
        checked,
        Some(Violation::UncastChoice {
            sighting: sightings[0].clone(),
        })
    );
}

#[test]
fn check_of_a_voter_outside_the_cluster_is_reported() {
    // Arrange
    let cast = [choice(0xaa)];
    let invariants = ClusterInvariants::new(4, 1, &cast);
    let sightings = [sighting(1, 0xaa, 3, &[1, 2, 5])];

    // Act
    let checked = invariants.check(&sightings);

    // Assert
    assert_eq!(
        checked,
        Some(Violation::VoterOutsideTheCluster {
            sighting: sightings[0].clone(),
        })
    );
}

#[test]
fn check_of_one_choice_seen_everywhere_holds() {
    // Arrange
    let cast = [choice(0xaa), choice(0xbb)];
    let invariants = ClusterInvariants::new(4, 1, &cast);
    let sightings = [
        sighting(1, 0xaa, 3, &[1, 2, 3]),
        sighting(2, 0xaa, 3, &[2, 3, 4]),
    ];

    // Act & Assert
    assert_eq!(invariants.check(&sightings), None);
}

#[test]
fn check_of_two_choices_at_a_threshold_that_intersects_is_conflicting_quorums() {
    // Arrange
    let cast = [choice(0xaa), choice(0xbb)];
    let invariants = ClusterInvariants::new(4, 1, &cast);
    let sightings = [
        sighting(1, 0xaa, 3, &[1, 2, 3]),
        sighting(2, 0xbb, 3, &[1, 2, 4]),
    ];

    // Act
    let checked = invariants.check(&sightings);

    // Assert
    assert_eq!(
        checked,
        Some(Violation::ConflictingQuorums {
            first: sightings[0].clone(),
            second: sightings[1].clone(),
        })
    );
}

#[test]
fn check_of_two_choices_at_a_threshold_too_low_to_intersect_holds() {
    // Arrange
    let cast = [choice(0xaa), choice(0xbb)];
    let invariants = ClusterInvariants::new(4, 1, &cast);
    let sightings = [sighting(1, 0xaa, 2, &[1, 2]), sighting(2, 0xbb, 2, &[3, 4])];

    // Act & Assert
    assert_eq!(invariants.check(&sightings), None);
}
