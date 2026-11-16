// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// What a flood at the bounds left the counter showing, in bytes counted from
// before the instance was created: what the books held full, the most they
// held between any two calls of the flood, and what they held after it.
#[derive(Debug)]
pub struct FloodProfile {
    pub full: isize,
    pub most_while_flooded: isize,
    pub after_flood: isize,
}
