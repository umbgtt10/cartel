// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// What a vote is counted in: a question, a round and a phase, ordered in that
// order and never interpreted. The question is what is being decided, the
// round one attempt at deciding it, the phase a stage of a round, each named
// by the protocol that drives the instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ballot {
    pub question: u64,
    pub round: u64,
    pub phase: u64,
}
