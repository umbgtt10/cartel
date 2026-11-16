// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The bounds an instance is created with (§2), each given or left out: the
// most ballots open at once, the most members an electorate may name, the
// most bytes a choice may have. One left out is no bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bounds {
    pub open_ballots: Option<u64>,
    pub electorate: Option<u64>,
    pub choice: Option<u64>,
}
