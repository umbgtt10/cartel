// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// A member and the position it has reached, as a measure gives them (§11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemberPosition {
    pub member: u64,
    pub position: u64,
}
