// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// What a vote is counted in (§4): a question, a round and a phase, ordered in
// that order, and never interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BallotKey {
    pub question: u64,
    pub round: u64,
    pub phase: u64,
}
