// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::open_ballot::OpenBallot;

// What an instance is (§2, §9): its id, the ballots open in no particular
// order, and the number its next receipt takes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceState {
    pub node: u64,
    pub incarnation: u64,
    pub ballots: Vec<OpenBallot>,
    pub next_receipt: u64,
}
