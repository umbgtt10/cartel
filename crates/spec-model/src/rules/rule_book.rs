// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::rules::ballot_situation::BallotSituation;
use crate::rules::books_situation::BooksSituation;
use crate::rules::change::Change;
use crate::rules::command_situation::CommandSituation;
use crate::rules::rule::Rule;
use crate::rules::situation::Situation;

const RULES: [(&str, Rule); 19] = [
    (
        "empty-electorate",
        Rule {
            situation: Situation::Command(CommandSituation::EmptyElectorate),
            change: Change::Nothing,
        },
    ),
    (
        "repeated-member",
        Rule {
            situation: Situation::Command(CommandSituation::RepeatedMember),
            change: Change::Nothing,
        },
    ),
    (
        "no-thresholds",
        Rule {
            situation: Situation::Command(CommandSituation::NoThresholds),
            change: Change::Nothing,
        },
    ),
    (
        "threshold-out-of-range",
        Rule {
            situation: Situation::Command(CommandSituation::ThresholdOutOfRange),
            change: Change::Nothing,
        },
    ),
    (
        "repeated-threshold",
        Rule {
            situation: Situation::Command(CommandSituation::RepeatedThreshold),
            change: Change::Nothing,
        },
    ),
    (
        "ballot-closed",
        Rule {
            situation: Situation::Books(BooksSituation::Closed),
            change: Change::Nothing,
        },
    ),
    (
        "ballot-already-open",
        Rule {
            situation: Situation::Books(BooksSituation::Open),
            change: Change::Nothing,
        },
    ),
    (
        "ballot-opened",
        Rule {
            situation: Situation::Always,
            change: Change::BallotOpened,
        },
    ),
    (
        "ballot-not-open",
        Rule {
            situation: Situation::Books(BooksSituation::NotOpen),
            change: Change::Nothing,
        },
    ),
    (
        "not-a-member",
        Rule {
            situation: Situation::Ballot(BallotSituation::NotAMember),
            change: Change::Nothing,
        },
    ),
    (
        "already-counted",
        Rule {
            situation: Situation::Ballot(BallotSituation::AlreadyCounted),
            change: Change::Nothing,
        },
    ),
    (
        "equivocation",
        Rule {
            situation: Situation::Ballot(BallotSituation::Equivocates),
            change: Change::Nothing,
        },
    ),
    (
        "quorum-reached",
        Rule {
            situation: Situation::Ballot(BallotSituation::ReachesThreshold),
            change: Change::VoteCast,
        },
    ),
    (
        "vote-counted",
        Rule {
            situation: Situation::Always,
            change: Change::VoteCast,
        },
    ),
    (
        "close-behind",
        Rule {
            situation: Situation::Books(BooksSituation::CloseBehind),
            change: Change::Nothing,
        },
    ),
    (
        "ballots-closed",
        Rule {
            situation: Situation::Always,
            change: Change::BallotsClosed,
        },
    ),
    (
        "ballot-shown",
        Rule {
            situation: Situation::Always,
            change: Change::Nothing,
        },
    ),
    (
        "ballots-listed",
        Rule {
            situation: Situation::Always,
            change: Change::Nothing,
        },
    ),
    (
        "books-shown",
        Rule {
            situation: Situation::Always,
            change: Change::Nothing,
        },
    ),
];

// What the prose gives each row of the decision table: the situation in which
// it applies and the change it makes. The table gives the rest.
pub struct RuleBook;

impl RuleBook {
    #[must_use]
    pub fn row_ids() -> Vec<&'static str> {
        RULES.iter().map(|(id, _)| *id).collect()
    }

    #[must_use]
    pub fn rule(row_id: &str) -> Rule {
        RULES
            .iter()
            .find(|(id, _)| *id == row_id)
            .map(|(_, rule)| *rule)
            .expect("a row the model knows")
    }
}
