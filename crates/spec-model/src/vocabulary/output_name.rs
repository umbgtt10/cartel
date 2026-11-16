// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;

use crate::call::Call;
use crate::lookup::Lookup;
use crate::view::View;
use crate::vocabulary::rejection_name::RejectionName;
use crate::vocabulary::speller::Speller;
use crate::vocabulary::spelling::Spelling;
use crate::vocabulary::variant::Variant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputName {
    BallotOpened,
    VoteCounted,
    QuorumReached,
    BallotShown,
    BallotsListed,
    BooksShown,
    CommandRejected,
}

impl OutputName {
    pub const SPELLING: Spelling<OutputName> = Spelling {
        kind: "an output",
        words: &[
            (OutputName::BallotOpened, "BallotOpened"),
            (OutputName::VoteCounted, "VoteCounted"),
            (OutputName::QuorumReached, "QuorumReached"),
            (OutputName::BallotShown, "BallotShown"),
            (OutputName::BallotsListed, "BallotsListed"),
            (OutputName::BooksShown, "BooksShown"),
            (OutputName::CommandRejected, "CommandRejected"),
        ],
    };

    // What the output carries (§5-§9), read from the state the command met, or
    // from the one it left where the output says what the call made: a vote's
    // count and quorum, the checkpoint after it.
    #[must_use]
    pub fn encoded(self, rejection: Option<RejectionName>, call: &Call) -> Value {
        let payload = match self {
            OutputName::BallotOpened => Value::Object(Map::new()),
            OutputName::VoteCounted => {
                let voters = Self::voters_after(call);
                json!({"count": View::number(Self::count(&voters))})
            }
            OutputName::QuorumReached => {
                let voters = Self::voters_after(call);
                let count = Self::count(&voters);
                let ballot = Lookup::named(call.after, call.command);
                let reached: Vec<u64> = ballot
                    .thresholds
                    .iter()
                    .copied()
                    .filter(|threshold| *threshold == count)
                    .collect();
                json!({
                    "count": View::number(count),
                    "thresholds": View::numbers(&reached),
                    "voters": View::numbers(&voters),
                })
            }
            OutputName::BallotShown => {
                json!({"record": View::record(Lookup::named(call.before, call.command))})
            }
            OutputName::BallotsListed => json!({"ballots": View::ballots(call.before)}),
            OutputName::BooksShown => json!({"checkpoint": View::checkpoint(call.after)}),
            OutputName::CommandRejected => rejection.expect("a rejection").encoded(call),
        };
        Variant::of(Speller::name(&Self::SPELLING, self), payload)
    }

    fn count(voters: &[u64]) -> u64 {
        u64::try_from(voters.len()).expect("a count within u64")
    }

    fn voters_after(call: &Call) -> Vec<u64> {
        let ballot = Lookup::named(call.after, call.command);
        Lookup::voters(ballot, call.command.choice.as_ref().expect("a choice"))
    }
}
