// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

// What a vote is for (§1): bytes, possibly none, ordered byte by byte, a
// choice that starts another coming before it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Choice {
    pub bytes: Vec<u8>,
}
