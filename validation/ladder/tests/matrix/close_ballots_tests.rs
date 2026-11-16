// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The exhaustive matrix tier, CloseBallots: with the books never closed or
// closed to a point already, every set of four ballots open around it, and
// every close before a point of questions 0 to 3 and rounds 0 to 2 -- then an
// opening, a vote and a query for each of the four -- the crate's receipt
// against the specification's model's, and every row closing adds reached.

use crate::support::both_sides::BothSides;
use cartel::ballot::Ballot;
use cartel::choice::Choice;
use cartel::close_point::ClosePoint;
use cartel::command::Command;
use serde_json::Value;
use std::collections::BTreeSet;

const BALLOTS: [Ballot; 4] = [
    Ballot {
        question: 0,
        round: 5,
        phase: 0,
    },
    Ballot {
        question: 1,
        round: 0,
        phase: 0,
    },
    Ballot {
        question: 1,
        round: 1,
        phase: 0,
    },
    Ballot {
        question: 2,
        round: 0,
        phase: 3,
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

// An opening, a vote and a query for each ballot.
fn probes() -> Vec<Command> {
    BALLOTS
        .iter()
        .flat_map(|ballot| {
            [
                Command::OpenBallot {
                    ballot: *ballot,
                    electorate: vec![1, 2],
                    thresholds: vec![2],
                },
                Command::CastVote {
                    ballot: *ballot,
                    member: 1,
                    choice: Choice { bytes: vec![0xaa] },
                },
                Command::ShowBallot { ballot: *ballot },
            ]
        })
        .collect()
}

#[test]
fn process_of_every_close_and_what_meets_it_after_answers_as_the_model_does() {
    // Arrange
    let mut departures = Vec::new();
    let mut reached = BTreeSet::new();
    let probes = probes();

    // Act
    for prior in [
        None,
        Some(ClosePoint {
            question: 1,
            round: 1,
        }),
    ] {
        for mask in 0..1_u32 << BALLOTS.len() {
            for question in 0..4 {
                for round in 0..3 {
                    let mut sides = BothSides::new();
                    if let Some(before) = prior {
                        let _ = sides.process(&Command::CloseBallots { before });
                    }
                    for (_, ballot) in BALLOTS
                        .iter()
                        .enumerate()
                        .filter(|(at, _)| mask & (1 << at) != 0)
                    {
                        let _ = sides.process(&Command::OpenBallot {
                            ballot: *ballot,
                            electorate: vec![1],
                            thresholds: vec![1],
                        });
                    }
                    let close = Command::CloseBallots {
                        before: ClosePoint { question, round },
                    };
                    for command in [close].iter().chain(&probes).chain([&Command::ShowBooks]) {
                        let (from_crate, from_model) = sides.process(command);
                        if from_crate != from_model {
                            departures.push(format!(
                                "{command:?}: crate {from_crate}, model {from_model}"
                            ));
                        }
                        reached.insert(kind(&from_crate));
                    }
                }
            }
        }
    }

    // Assert
    assert!(
        departures.is_empty(),
        "{} departures, the first {:?}",
        departures.len(),
        departures.first()
    );
    let closing: BTreeSet<String> = ["BallotsClosed", "CloseBehind", "BallotClosed"]
        .map(String::from)
        .into();
    assert!(closing.is_subset(&reached), "{reached:?}");
}
