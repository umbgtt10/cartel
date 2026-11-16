// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::choice::Choice;
use alloc::vec::Vec;

// One choice of a tally and its voters, in ascending order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Votes {
    pub choice: Choice,
    pub voters: Vec<u64>,
}
