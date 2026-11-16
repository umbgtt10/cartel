// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::choice::Choice;

// A vote in flight: the node it is delivered to, the member that cast it and
// its choice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    pub node: u64,
    pub member: u64,
    pub choice: Choice,
}
