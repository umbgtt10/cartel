// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The exhaustive matrix tier, OpenBallot: every electorate of up to three
// members drawn from three, repeats included, against every list of up to two
// thresholds from 0 to 4, for a ballot not open and for one already open; and
// again under every bound up to two, none included, in each situation the
// bound rows are ordered against -- the crate's receipt against the
// specification's model's, and every row of the command reached.

use crate::support::both_sides::BothSides;
use cartel::ballot::Ballot;
use cartel::bounds::Bounds;
use cartel::close_point::ClosePoint;
use cartel::command::Command;
use serde_json::Value;
use std::collections::BTreeSet;

const BALLOT: Ballot = Ballot {
    question: 1,
    round: 0,
    phase: 0,
};
const BOUNDS: [Option<u64>; 4] = [None, Some(0), Some(1), Some(2)];
const OTHER: Ballot = Ballot {
    question: 2,
    round: 0,
    phase: 0,
};

// The output's name, or the rejection's for a refusal.
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

// Every list of up to `longest` values drawn from those given, repeats
// included.
fn lists(values: &[u64], longest: usize) -> Vec<Vec<u64>> {
    let mut found = vec![Vec::new()];
    let mut last = vec![Vec::new()];
    for _ in 0..longest {
        last = last
            .iter()
            .flat_map(|list| {
                values.iter().map(move |value| {
                    let mut longer = list.clone();
                    longer.push(*value);
                    longer
                })
            })
            .collect();
        found.extend(last.iter().cloned());
    }
    found
}

fn open(electorate: Vec<u64>, thresholds: Vec<u64>) -> Command {
    Command::OpenBallot {
        ballot: BALLOT,
        electorate,
        thresholds,
    }
}

// What the books hold before the opening: nothing; the ballot open; another
// ballot open; another open and the ballot below the close point.
fn situations() -> [Vec<Command>; 4] {
    let other = Command::OpenBallot {
        ballot: OTHER,
        electorate: vec![1],
        thresholds: vec![1],
    };
    [
        Vec::new(),
        vec![open(vec![1, 2], vec![1])],
        vec![other.clone()],
        vec![
            Command::CloseBallots {
                before: ClosePoint {
                    question: 2,
                    round: 0,
                },
            },
            other,
        ],
    ]
}

#[test]
fn process_of_every_opening_answers_as_the_model_does() {
    // Arrange
    let mut departures = Vec::new();
    let mut reached = BTreeSet::new();

    // Act
    for already_open in [false, true] {
        for electorate in lists(&[1, 2, 3], 3) {
            for thresholds in lists(&[0, 1, 2, 3, 4], 2) {
                let mut sides = BothSides::new(Bounds::default());
                if already_open {
                    let _ = sides.process(&open(vec![1, 2], vec![1]));
                }
                let command = open(electorate.clone(), thresholds);
                let (from_crate, from_model) = sides.process(&command);
                if from_crate != from_model {
                    departures.push(format!(
                        "{command:?}: crate {from_crate}, model {from_model}"
                    ));
                }
                reached.insert(kind(&from_crate));
            }
        }
    }

    // Assert
    assert!(departures.is_empty(), "{departures:#?}");
    assert_eq!(
        reached,
        [
            "BallotAlreadyOpen",
            "BallotOpened",
            "EmptyElectorate",
            "NoThresholds",
            "RepeatedMember",
            "RepeatedThreshold",
            "ThresholdOutOfRange",
        ]
        .map(String::from)
        .into_iter()
        .collect()
    );
}

#[test]
fn process_of_every_opening_under_every_bound_answers_as_the_model_does() {
    // Arrange
    let mut departures = Vec::new();
    let mut reached = BTreeSet::new();
    let thresholds: [&[u64]; 6] = [&[], &[0], &[1], &[3], &[4], &[1, 1]];
    let cases: Vec<(Bounds, Vec<Command>)> = BOUNDS
        .into_iter()
        .flat_map(|open_ballots| {
            BOUNDS.into_iter().flat_map(move |electorate| {
                situations().map(|setup| {
                    (
                        Bounds {
                            open_ballots,
                            electorate,
                            choice: None,
                        },
                        setup,
                    )
                })
            })
        })
        .collect();

    // Act
    for (bounds, setup) in &cases {
        for electorate in lists(&[1, 2, 3], 3) {
            for given in thresholds {
                let mut sides = BothSides::new(*bounds);
                for command in setup {
                    let _ = sides.process(command);
                }
                let command = open(electorate.clone(), given.to_vec());
                let (from_crate, from_model) = sides.process(&command);
                if from_crate != from_model {
                    departures.push(format!(
                        "{command:?} under {bounds:?}: crate {from_crate}, model {from_model}"
                    ));
                }
                reached.insert(kind(&from_crate));
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
    assert_eq!(
        reached,
        [
            "BallotAlreadyOpen",
            "BallotClosed",
            "BallotOpened",
            "BooksFull",
            "ElectorateTooLarge",
            "EmptyElectorate",
            "NoThresholds",
            "RepeatedMember",
            "RepeatedThreshold",
            "ThresholdOutOfRange",
        ]
        .map(String::from)
        .into_iter()
        .collect()
    );
}
