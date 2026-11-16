// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// Quorum intersection across a cluster: over random clusters of four to eight
// members, up to a third of them equivocating, some silent, and a transport
// that loses, duplicates and reorders, no two nodes ever see different choices
// reach a threshold at which two quorums must share an honest member -- the
// threshold a protocol tolerating that many equivocators would supply.

use cartel_core_validation::cluster_invariants::ClusterInvariants;
use cartel_core_validation::cluster_setup::ClusterSetup;
use cartel_core_validation::cluster_simulation::ClusterSimulation;
use cartel_core_validation::fault_policy::FaultPolicy;
use proptest::prelude::Strategy;
use proptest::prelude::any;
use proptest::proptest;

proptest! {
    #[test]
    fn run_of_any_cluster_keeps_every_invariant(setup in setup()) {
        // Arrange
        let mut simulation = ClusterSimulation::new(setup);
        let cast = ClusterSimulation::cast();

        // Act
        let sightings = simulation.run();

        // Assert
        let invariants = ClusterInvariants::new(setup.members, setup.equivocating, &cast);
        assert_eq!(invariants.check(&sightings), None, "{setup:?}: {sightings:#?}");
    }
}

fn setup() -> impl Strategy<Value = ClusterSetup> {
    (4..=8_u64, any::<u64>(), 0..=40_u64, 0..=40_u64).prop_flat_map(
        |(members, seed, loss, duplication)| {
            let tolerated = (members - 1) / 3;
            (0..=tolerated, 0..=1_u64).prop_map(move |(equivocating, silent)| ClusterSetup {
                members,
                equivocating,
                silent,
                threshold: (members + equivocating) / 2 + 1,
                faults: FaultPolicy {
                    loss,
                    duplication,
                    crash: 0,
                },
                seed,
            })
        },
    )
}
