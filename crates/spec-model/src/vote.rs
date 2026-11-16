// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::choice::Choice;

// One vote counted in a ballot: the member that cast it and the choice it is
// for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub member: u64,
    pub choice: Choice,
}
