// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot_record::BallotRecord;
use crate::bounds::Bounds;
use crate::close_point::ClosePoint;
use crate::instance_id::InstanceId;
use alloc::vec::Vec;

// The books, as ShowBooks answers them: the instance id, its bounds, the close
// point once a close has moved it, every open ballot's record in canonical
// order, and the next receipt's number -- the state after the call that shows
// it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub instance: InstanceId,
    pub bounds: Bounds,
    pub close_point: Option<ClosePoint>,
    pub ballots: Vec<BallotRecord>,
    pub next_receipt: u64,
}
