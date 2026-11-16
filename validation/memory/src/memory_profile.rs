// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// What a run of ballots left the counter showing, in bytes counted from before
// the instance was created: what the instance held empty, what it held after
// the last ballot, and what was left once it was dropped.
#[derive(Debug)]
pub struct MemoryProfile {
    pub skeleton: isize,
    pub held_after_all: isize,
    pub after_drop: isize,
}
