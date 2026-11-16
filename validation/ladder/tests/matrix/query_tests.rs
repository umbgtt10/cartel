// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The exhaustive matrix tier, the queries: every set of three ballots open,
// each with a vote, asked to show each ballot, to list them and to show the
// books -- the crate's receipt against the specification's model's, and every
// row of the queries reached.

use crate::support::both_sides::BothSides;
use cartel::ballot::Ballot;
use cartel::choice::Choice;
use cartel::command::Command;
use serde_json::Value;
use std::collections::BTreeSet;

const BALLOTS: [Ballot; 3] = [
    Ballot {
        question: 2,
        round: 0,
        phase: 0,
    },
    Ballot {
        question: 1,
        round: 1,
        phase: 0,
    },
    Ballot {
        question: 1,
        round: 0,
        phase: 1,
    },
];

fn kind(receipt: &Value) -> String {
    let (name, payload) = receipt["output"]
        .as_object()
        .and_then(|output| output.iter().next())
        .expect("an output");
    if name == "CommandRejected" {
        payload
            .as_object()
            .and_then(|rejection| rejection.keys().next())
            .expect("a rejection")
            .clone()
    } else {
        name.clone()
    }
}

#[test]
fn process_of_every_query_answers_as_the_model_does() {
    // Arrange
    let mut departures = Vec::new();
    let mut reached = BTreeSet::new();
    let queries: Vec<Command> = BALLOTS
        .iter()
        .map(|ballot| Command::ShowBallot { ballot: *ballot })
        .chain([Command::ListBallots, Command::ShowBooks])
        .collect();

    // Act
    for mask in 0..1_u32 << BALLOTS.len() {
        for query in &queries {
            let mut sides = BothSides::new();
            for (at, ballot) in BALLOTS
                .iter()
                .enumerate()
                .filter(|(at, _)| mask & (1 << at) != 0)
            {
                let _ = sides.process(&Command::OpenBallot {
                    ballot: *ballot,
                    electorate: vec![1, 2],
                    thresholds: vec![2],
                });
                let _ = sides.process(&Command::CastVote {
                    ballot: *ballot,
                    member: at as u64 % 2 + 1,
                    choice: Choice {
                        bytes: vec![at as u8],
                    },
                });
            }
            let (from_crate, from_model) = sides.process(query);
            if from_crate != from_model {
                departures.push(format!(
                    "{query:?} at {mask}: crate {from_crate}, model {from_model}"
                ));
            }
            reached.insert(kind(&from_crate));
        }
    }

    // Assert
    assert!(departures.is_empty(), "{departures:#?}");
    assert_eq!(
        reached,
        [
            "BallotNotOpen",
            "BallotShown",
            "BallotsListed",
            "BooksShown"
        ]
        .map(String::from)
        .into_iter()
        .collect()
    );
}
