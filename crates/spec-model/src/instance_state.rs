// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::bounds::Bounds;
use crate::open_ballot::OpenBallot;
use crate::point::Point;

// What an instance is (§2, §9): its id, the bounds it was given, its close
// point if a close has moved it, the ballots open in no particular order, and
// the number its next receipt takes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceState {
    pub node: u64,
    pub incarnation: u64,
    pub bounds: Bounds,
    pub close_point: Option<Point>,
    pub ballots: Vec<OpenBallot>,
    pub next_receipt: u64,
}
