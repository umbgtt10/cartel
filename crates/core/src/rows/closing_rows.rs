// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::ballot::Ballot;
use crate::close_point::ClosePoint;
use crate::rejection::Rejection;

// The rows closing adds, read from the close point the books remember: whether
// a ballot is below it -- its question and round before the point's, whatever
// its phase -- and whether a close is at or behind it.
#[derive(Debug, Clone, Copy)]
pub struct ClosingRows {
    close_point: Option<ClosePoint>,
}

impl ClosingRows {
    pub fn new(close_point: Option<ClosePoint>) -> Self {
        Self { close_point }
    }

    pub fn covers(&self, ballot: Ballot) -> bool {
        self.close_point
            .is_some_and(|point| (ballot.question, ballot.round) < (point.question, point.round))
    }

    // The refusal of a close at or behind the close point, naming it, or none:
    // then the point moves.
    pub fn refusal(&self, before: ClosePoint) -> Option<Rejection> {
        self.close_point
            .filter(|point| before <= *point)
            .map(|close_point| Rejection::CloseBehind { close_point })
    }
}
