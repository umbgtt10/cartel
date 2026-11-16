// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// A member and the position it has reached -- how far its log is matched,
// which round of the protocol's messages it has acknowledged -- as the
// protocol gives them to a measure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemberPosition {
    pub member: u64,
    pub position: u64,
}
