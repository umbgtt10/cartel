// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::choice::Choice;
use cartel_core_validation::cluster_setup::ClusterSetup;
use cartel_core_validation::cluster_simulation::ClusterSimulation;
use cartel_core_validation::fault_policy::FaultPolicy;
use cartel_core_validation::member_conduct::MemberConduct;

fn setup(faults: FaultPolicy, seed: u64) -> ClusterSetup {
    ClusterSetup {
        members: 4,
        equivocating: 0,
        silent: 0,
        threshold: 3,
        faults,
        seed,
    }
}

#[test]
fn cast_of_the_simulation_is_the_honest_choice_then_the_other() {
    // Arrange & Act
    let cast = ClusterSimulation::cast();

    // Assert
    assert_eq!(
        cast,
        [Choice { bytes: vec![0xaa] }, Choice { bytes: vec![0xbb] }]
    );
}

#[test]
fn conduct_of_a_member_is_by_its_place_equivocating_then_silent_then_honest() {
    // Arrange
    let simulation = ClusterSimulation::new(ClusterSetup {
        equivocating: 1,
        silent: 1,
        ..setup(FaultPolicy::default(), 1)
    });

    // Act
    let conducts = [1, 2, 3, 4].map(|member| simulation.conduct(member));

    // Assert
    assert_eq!(
        conducts,
        [
            MemberConduct::Equivocating,
            MemberConduct::Silent,
            MemberConduct::Honest,
            MemberConduct::Honest,
        ]
    );
}

#[test]
fn deliveries_with_every_vote_duplicated_are_two_of_each() {
    // Arrange
    let mut simulation = ClusterSimulation::new(setup(
        FaultPolicy {
            duplication: 100,
            ..FaultPolicy::default()
        },
        3,
    ));

    // Act
    let deliveries = simulation.deliveries();

    // Assert
    assert_eq!(deliveries.len(), 32);
}

#[test]
fn run_of_honest_members_over_a_perfect_transport_sees_every_node_reach_the_quorum() {
    // Arrange
    let mut simulation = ClusterSimulation::new(setup(FaultPolicy::default(), 5));

    // Act
    let sightings = simulation.run();

    // Assert
    let mut nodes: Vec<u64> = sightings.iter().map(|sighting| sighting.node).collect();
    nodes.sort_unstable();
    assert_eq!(nodes, vec![1, 2, 3, 4]);
    assert!(sightings
        .iter()
        .all(|sighting| sighting.choice == ClusterSimulation::cast()[0]));
}

#[test]
fn run_of_one_seed_twice_sees_the_same_quorums() {
    // Arrange
    let faults = FaultPolicy {
        loss: 20,
        duplication: 20,
        crash: 10,
    };

    // Act
    let runs = [11, 11].map(|seed| ClusterSimulation::new(setup(faults, seed)).run());

    // Assert
    assert_eq!(runs[0], runs[1]);
}

#[test]
fn run_with_every_vote_lost_sees_no_quorum() {
    // Arrange
    let mut simulation = ClusterSimulation::new(setup(
        FaultPolicy {
            loss: 100,
            ..FaultPolicy::default()
        },
        9,
    ));

    // Act
    let sightings = simulation.run();

    // Assert
    assert!(sightings.is_empty());
}
