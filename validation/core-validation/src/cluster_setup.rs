// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::fault_policy::FaultPolicy;

// A cluster of `members` nodes, each a member: the first `equivocating` of them
// vote two choices, the next `silent` none, the rest one; every node opens the
// same ballot with the one threshold given; the transport's faults; the seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClusterSetup {
    pub members: u64,
    pub equivocating: u64,
    pub silent: u64,
    pub threshold: u64,
    pub faults: FaultPolicy,
    pub seed: u64,
}
