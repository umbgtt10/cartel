// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

// What a vote is for: bytes, possibly none, ordered byte by byte -- the first
// byte that differs decides, and a choice that starts another comes before
// it. Cartel never interprets them.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Choice {
    pub bytes: Vec<u8>,
}
