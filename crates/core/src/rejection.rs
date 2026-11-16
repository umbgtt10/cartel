// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::choice::Choice;
use crate::close_point::ClosePoint;

// Why a command was refused, with what the log needs beside the command: the
// lowest member or threshold at fault, the choice a member is already a voter
// of, the close point a close was at or behind, or the bound a command would
// exceed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejection {
    EmptyElectorate,
    RepeatedMember { member: u64 },
    NoThresholds,
    ThresholdOutOfRange { threshold: u64 },
    RepeatedThreshold { threshold: u64 },
    BallotAlreadyOpen,
    BallotNotOpen,
    NotAMember,
    AlreadyCounted,
    Equivocation { counted: Choice },
    BallotClosed,
    CloseBehind { close_point: ClosePoint },
    ElectorateTooLarge { bound: u64 },
    BooksFull { bound: u64 },
    ChoiceTooLong { bound: u64 },
}
