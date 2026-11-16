// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::sighting::Sighting;
use crate::violation::Violation;
use cartel::choice::Choice;

// What a cluster's sightings must keep, whatever the faults:
// - quorum intersection: where twice the threshold exceeds the members and the
//   equivocators together, two quorums share an honest member, so no two
//   sightings, on any node or incarnation, are of different choices;
// - validity: every choice sighted is one a member cast;
// - every voter of a quorum is a member.
pub struct ClusterInvariants<'a> {
    members: u64,
    equivocating: u64,
    cast: &'a [Choice],
}

impl<'a> ClusterInvariants<'a> {
    pub fn new(members: u64, equivocating: u64, cast: &'a [Choice]) -> Self {
        Self {
            members,
            equivocating,
            cast,
        }
    }

    // The first invariant the sightings break, or none.
    pub fn check(&self, sightings: &[Sighting]) -> Option<Violation> {
        for sighting in sightings {
            if !self.cast.contains(&sighting.choice) {
                return Some(Violation::UncastChoice {
                    sighting: sighting.clone(),
                });
            }
            if sighting
                .voters
                .iter()
                .any(|voter| *voter == 0 || *voter > self.members)
            {
                return Some(Violation::VoterOutsideTheCluster {
                    sighting: sighting.clone(),
                });
            }
        }
        let intersecting = |threshold: u64| 2 * threshold > self.members + self.equivocating;
        for (at, first) in sightings.iter().enumerate() {
            let conflict = sightings[at + 1..].iter().find(|second| {
                second.threshold == first.threshold
                    && intersecting(first.threshold)
                    && second.choice != first.choice
            });
            if let Some(second) = conflict {
                return Some(Violation::ConflictingQuorums {
                    first: first.clone(),
                    second: second.clone(),
                });
            }
        }
        None
    }
}
