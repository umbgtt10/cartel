// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::ballot_key::BallotKey;
use crate::choice::Choice;
use crate::member_position::MemberPosition;
use crate::point::Point;
use crate::vocabulary::command_name::CommandName;

// A command as the specification encodes it, each of its members read where
// it carries one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub name: CommandName,
    pub before: Option<Point>,
    pub positions: Option<Vec<MemberPosition>>,
    pub threshold: Option<u64>,
    pub ballot: Option<BallotKey>,
    pub electorate: Option<Vec<u64>>,
    pub thresholds: Option<Vec<u64>>,
    pub member: Option<u64>,
    pub choice: Option<Choice>,
}
