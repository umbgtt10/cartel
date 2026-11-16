// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

#![no_std]
#![forbid(unsafe_code)]

//! The testing ladder's cluster simulation tier: several nodes' books, one
//! cartel instance each, fed the votes of honest, silent and equivocating
//! members over a seeded transport that loses, duplicates and reorders them,
//! with nodes that crash and restart fresh -- and the cross-node invariants
//! consensus counting must keep, each shown firing on a crafted violation.

extern crate alloc;

pub mod cluster_invariants;
pub mod cluster_setup;
pub mod cluster_simulation;
pub mod delivery;
pub mod fault_policy;
pub mod member_conduct;
pub mod seeded_random;
pub mod sighting;
pub mod violation;
