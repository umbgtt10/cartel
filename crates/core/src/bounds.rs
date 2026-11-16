// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The bounds an instance is created with, each given or left out: the most
// ballots open at once, the most members a ballot's electorate may name, the
// most bytes a counted choice may have. One left out is no bound, so the
// default binds nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bounds {
    pub open_ballots: Option<u64>,
    pub electorate: Option<u64>,
    pub choice: Option<u64>,
}
