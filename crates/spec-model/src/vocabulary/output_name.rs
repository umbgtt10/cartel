// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeSet;
use alloc::vec::Vec;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use crate::ballot_key::BallotKey;
use crate::call::Call;
use crate::command::Command;
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
    BallotsClosed,
    WatermarkMeasured,
    ClaimExamined,
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
            (OutputName::BallotsClosed, "BallotsClosed"),
            (OutputName::WatermarkMeasured, "WatermarkMeasured"),
            (OutputName::ClaimExamined, "ClaimExamined"),
            (OutputName::BallotShown, "BallotShown"),
            (OutputName::BallotsListed, "BallotsListed"),
            (OutputName::BooksShown, "BooksShown"),
            (OutputName::CommandRejected, "CommandRejected"),
        ],
    };

    // What the output carries (§5-§10), read from the state the command met,
    // or from the one it left where the output says what the call made: a
    // vote's count and quorum, the checkpoint after it. A close names the
    // ballots it closed: those below its point in the state it met.
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
            OutputName::BallotsClosed => {
                let point = call.command.before.expect("a point");
                let mut closed: Vec<BallotKey> = call
                    .before
                    .ballots
                    .iter()
                    .map(|ballot| ballot.key)
                    .filter(|key| Lookup::is_below(*key, point))
                    .collect();
                closed.sort_unstable();
                json!({"ballots": closed.into_iter().map(View::ballot).collect::<Vec<Value>>()})
            }
            OutputName::WatermarkMeasured => {
                json!({"position": View::number(Self::watermark(call.command))})
            }
            OutputName::ClaimExamined => Self::claim(call.command),
            OutputName::BallotShown => {
                json!({"record": View::record(Lookup::named(call.before, call.command))})
            }
            OutputName::BallotsListed => json!({"ballots": View::ballots(call.before)}),
            OutputName::BooksShown => json!({"checkpoint": View::checkpoint(call.after)}),
            OutputName::CommandRejected => rejection.expect("a rejection").encoded(call),
        };
        Variant::of(Speller::name(&Self::SPELLING, self), payload)
    }

    // A claim's facts (§12): the electorate's members among the voters, the
    // voters outside it, the voters named more than once -- each once, in
    // ascending order -- and whether the members counted number at least the
    // threshold.
    fn claim(command: &Command) -> Value {
        let electorate = command.electorate.as_ref().expect("an electorate");
        let voters = command.voters.as_ref().expect("voters");
        let named = |voter: &u64| voters.iter().filter(|other| *other == voter).count();
        let listed = |keep: &dyn Fn(&u64) -> bool| {
            let found: BTreeSet<u64> = voters.iter().copied().filter(|voter| keep(voter)).collect();
            found.into_iter().collect::<Vec<u64>>()
        };
        let counted = listed(&|voter| electorate.contains(voter));
        let reached = u64::try_from(counted.len())
            .is_ok_and(|count| count >= command.threshold.expect("a threshold"));
        json!({
            "counted": View::numbers(&counted),
            "outsiders": View::numbers(&listed(&|voter| !electorate.contains(voter))),
            "repeated": View::numbers(&listed(&|voter| named(voter) > 1)),
            "reached": reached,
        })
    }

    fn count(voters: &[u64]) -> u64 {
        u64::try_from(voters.len()).expect("a count within u64")
    }

    fn voters_after(call: &Call) -> Vec<u64> {
        let ballot = Lookup::named(call.after, call.command);
        Lookup::voters(ballot, call.command.choice.as_ref().expect("a choice"))
    }

    // The highest position at least the threshold's number of members have
    // reached, a member having reached every position up to its own (§11): of
    // the positions given, the highest that many members are at or beyond.
    fn watermark(command: &Command) -> u64 {
        let given = command.positions.as_ref().expect("positions");
        let threshold = command.threshold.expect("a threshold");
        let reached_by = |position: u64| {
            given
                .iter()
                .filter(|other| other.position >= position)
                .count()
        };
        given
            .iter()
            .map(|candidate| candidate.position)
            .filter(|position| {
                u64::try_from(reached_by(*position)).is_ok_and(|count| count >= threshold)
            })
            .max()
            .expect("a position in range")
    }
}
