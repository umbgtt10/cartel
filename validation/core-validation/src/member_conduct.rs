// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// How a member votes: one choice to every node, nothing at all, or one choice
// to some nodes and another to the rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberConduct {
    Honest,
    Silent,
    Equivocating,
}
