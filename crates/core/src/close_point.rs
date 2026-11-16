// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// A question and a round: where a close lets go before, and the close point
// the books remember. Points are ordered by question, then round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClosePoint {
    pub question: u64,
    pub round: u64,
}
