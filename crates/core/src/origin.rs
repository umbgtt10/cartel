// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::fresh::Fresh;

// Where an instance comes from: in 0.1.0 of the specification, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Fresh(Fresh),
}
