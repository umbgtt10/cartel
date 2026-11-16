// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// A node and an incarnation: what tells one instance's receipts from
// another's. An incarnation never repeats for its node, so a node that
// restarts is a new instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstanceId {
    pub node: u64,
    pub incarnation: u64,
}
