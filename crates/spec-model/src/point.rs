// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// A question and a round (§4, §10): where a close lets go before, and the
// close point. Points are ordered by question, then round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub question: u64,
    pub round: u64,
}
