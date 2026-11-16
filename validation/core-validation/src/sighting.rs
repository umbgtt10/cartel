// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;
use cartel::choice::Choice;

// A quorum a node's books reported: the node and the incarnation it was in,
// the choice, the threshold reached and the voters that reached it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sighting {
    pub node: u64,
    pub incarnation: u64,
    pub choice: Choice,
    pub threshold: u64,
    pub voters: Vec<u64>,
}
