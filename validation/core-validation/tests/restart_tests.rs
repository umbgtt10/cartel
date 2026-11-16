// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// Fresh restarts: nodes that crash before deliveries lose the votes they had
// counted and open the ballot again under their next incarnation. Forgetting
// is safe: over random clusters with crashes, the invariants hold through
// every restart -- and restarts do happen in them.

use cartel_core_validation::cluster_invariants::ClusterInvariants;
use cartel_core_validation::cluster_setup::ClusterSetup;
use cartel_core_validation::cluster_simulation::ClusterSimulation;
use cartel_core_validation::fault_policy::FaultPolicy;
use proptest::prelude::any;
use proptest::proptest;

proptest! {
    #[test]
    fn run_of_any_cluster_with_crashes_keeps_every_invariant(
        members in 4..=8_u64,
        seed in any::<u64>(),
        crash in 1..=40_u64,
    ) {
        // Arrange
        let equivocating = (members - 1) / 3;
        let setup = ClusterSetup {
            members,
            equivocating,
            silent: 0,
            threshold: (members + equivocating) / 2 + 1,
            faults: FaultPolicy {
                loss: 10,
                duplication: 10,
                crash,
            },
            seed,
        };
        let cast = ClusterSimulation::cast();

        // Act
        let sightings = ClusterSimulation::new(setup).run();

        // Assert
        let invariants = ClusterInvariants::new(members, equivocating, &cast);
        assert_eq!(invariants.check(&sightings), None, "{setup:?}: {sightings:#?}");
    }
}

#[test]
fn run_with_crashes_sees_quorums_from_later_incarnations() {
    // Arrange
    let incarnations: Vec<u64> = (0..50)
        .flat_map(|seed| {
            ClusterSimulation::new(ClusterSetup {
                members: 4,
                equivocating: 0,
                silent: 0,
                threshold: 1,
                faults: FaultPolicy {
                    loss: 0,
                    duplication: 0,
                    crash: 30,
                },
                seed,
            })
            .run()
        })
        .map(|sighting| sighting.incarnation)
        .collect();

    // Act & Assert
    assert!(incarnations.iter().any(|incarnation| *incarnation > 1));
}
