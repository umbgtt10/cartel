// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::bounds::Bounds;
use crate::choice::Choice;
use crate::rejection::Rejection;

// The rows the bounds add, each naming the bound it refuses by: an
// electorate naming more members than its bound, a choice of more bytes than
// its bound, books holding as many open ballots as their bound. A bound left
// out refuses nothing.
#[derive(Debug, Clone, Copy)]
pub struct BoundRows {
    bounds: Bounds,
}

impl BoundRows {
    pub fn new(bounds: Bounds) -> Self {
        Self { bounds }
    }

    pub fn books(&self, open: usize) -> Option<Rejection> {
        self.bounds
            .open_ballots
            .filter(|bound| open as u64 >= *bound)
            .map(|bound| Rejection::BooksFull { bound })
    }

    pub fn choice(&self, choice: &Choice) -> Option<Rejection> {
        Self::exceeded(self.bounds.choice, choice.bytes.len())
            .map(|bound| Rejection::ChoiceTooLong { bound })
    }

    pub fn electorate(&self, electorate: &[u64]) -> Option<Rejection> {
        Self::exceeded(self.bounds.electorate, electorate.len())
            .map(|bound| Rejection::ElectorateTooLarge { bound })
    }

    // The bound a size is beyond, or none: within it, or no bound given.
    fn exceeded(bound: Option<u64>, size: usize) -> Option<u64> {
        bound.filter(|bound| size as u64 > *bound)
    }
}
