// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::sighting::Sighting;

// A cross-node invariant broken, with the sightings that break it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Violation {
    ConflictingQuorums { first: Sighting, second: Sighting },
    UncastChoice { sighting: Sighting },
    VoterOutsideTheCluster { sighting: Sighting },
}
