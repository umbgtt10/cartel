// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// How often, in percent, the transport loses a vote, delivers it twice, and a
// node crashes before a delivery -- restarting fresh, its books lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FaultPolicy {
    pub loss: u64,
    pub duplication: u64,
    pub crash: u64,
}
