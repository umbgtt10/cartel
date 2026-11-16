// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel_spec_model::rules::ballot_situation::BallotSituation;
use cartel_spec_model::rules::bound_situation::BoundSituation;
use cartel_spec_model::rules::change::Change;
use cartel_spec_model::rules::rule::Rule;
use cartel_spec_model::rules::rule_book::RuleBook;
use cartel_spec_model::rules::situation::Situation;

#[test]
fn row_ids_of_the_rule_book_are_its_rows_in_check_order() {
    // Arrange & Act
    let ids = RuleBook::row_ids();

    // Assert
    assert_eq!(
        ids,
        vec![
            "empty-electorate",
            "repeated-member",
            "no-thresholds",
            "threshold-out-of-range",
            "repeated-threshold",
            "electorate-too-large",
            "choice-too-long",
            "ballot-closed",
            "ballot-already-open",
            "books-full",
            "ballot-opened",
            "ballot-not-open",
            "not-a-member",
            "already-counted",
            "equivocation",
            "quorum-reached",
            "vote-counted",
            "close-behind",
            "ballots-closed",
            "watermark-measured",
            "ballot-shown",
            "ballots-listed",
            "books-shown",
        ]
    );
}

#[test]
fn rule_of_a_bound_s_row_is_its_bound_situation_changing_nothing() {
    // Arrange & Act
    let rules = [
        RuleBook::rule("electorate-too-large"),
        RuleBook::rule("choice-too-long"),
        RuleBook::rule("books-full"),
    ];

    // Assert
    assert_eq!(
        rules,
        [
            BoundSituation::ElectorateTooLarge,
            BoundSituation::ChoiceTooLong,
            BoundSituation::BooksFull,
        ]
        .map(|situation| Rule {
            situation: Situation::Bound(situation),
            change: Change::Nothing,
        })
    );
}

#[test]
fn rule_of_a_row_is_its_situation_and_its_change() {
    // Arrange & Act
    let rules = [
        RuleBook::rule("quorum-reached"),
        RuleBook::rule("ballot-shown"),
    ];

    // Assert
    assert_eq!(
        rules,
        [
            Rule {
                situation: Situation::Ballot(BallotSituation::ReachesThreshold),
                change: Change::VoteCast,
            },
            Rule {
                situation: Situation::Always,
                change: Change::Nothing,
            },
        ]
    );
}
