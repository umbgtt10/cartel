// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::vocabulary::spelling::Spelling;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandName {
    OpenBallot,
    CastVote,
    ShowBallot,
    ListBallots,
    ShowBooks,
}

impl CommandName {
    pub const SPELLING: Spelling<CommandName> = Spelling {
        kind: "a command",
        words: &[
            (CommandName::OpenBallot, "OpenBallot"),
            (CommandName::CastVote, "CastVote"),
            (CommandName::ShowBallot, "ShowBallot"),
            (CommandName::ListBallots, "ListBallots"),
            (CommandName::ShowBooks, "ShowBooks"),
        ],
    };
}
