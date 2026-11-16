// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// What a replay of a log made of it: how many of its lines it rebuilt as
// logged, out of how many the log held, the numbers missing between its
// first line and its last -- each a line lost -- and every line a replay
// answered otherwise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayReport {
    pub reconstructed: usize,
    pub total: usize,
    pub gaps: Vec<u64>,
    pub departures: Vec<String>,
}
