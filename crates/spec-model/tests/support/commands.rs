// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::states::States;
use cartel_spec_model::ballot_key::BallotKey;
use cartel_spec_model::command::Command;
use cartel_spec_model::vocabulary::command_name::CommandName;

// Commands as the reader gives them, each carrying what its kind carries.
pub struct Commands;

impl Commands {
    pub fn cast(key: BallotKey, member: u64, choice: &[u8]) -> Command {
        Command {
            ballot: Some(key),
            member: Some(member),
            choice: Some(States::choice(choice)),
            ..Self::of(CommandName::CastVote)
        }
    }

    pub fn of(name: CommandName) -> Command {
        Command {
            name,
            ballot: None,
            electorate: None,
            thresholds: None,
            member: None,
            choice: None,
        }
    }

    pub fn open(key: BallotKey, electorate: &[u64], thresholds: &[u64]) -> Command {
        Command {
            ballot: Some(key),
            electorate: Some(electorate.to_vec()),
            thresholds: Some(thresholds.to_vec()),
            ..Self::of(CommandName::OpenBallot)
        }
    }

    pub fn show(key: BallotKey) -> Command {
        Command {
            ballot: Some(key),
            ..Self::of(CommandName::ShowBallot)
        }
    }
}
