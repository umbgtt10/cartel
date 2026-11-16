// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The exhaustive matrix tier, OpenBallot: every electorate of up to three
// members drawn from three, repeats included, against every list of up to two
// thresholds from 0 to 4, for a ballot not open and for one already open --
// the crate's receipt against the specification's model's, and every row of
// the command reached.

use crate::support::both_sides::BothSides;
use cartel::ballot::Ballot;
use cartel::command::Command;
use serde_json::Value;
use std::collections::BTreeSet;

const BALLOT: Ballot = Ballot {
    question: 1,
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

#[test]
fn process_of_every_opening_answers_as_the_model_does() {
    // Arrange
    let mut departures = Vec::new();
    let mut reached = BTreeSet::new();

    // Act
    for already_open in [false, true] {
        for electorate in lists(&[1, 2, 3], 3) {
            for thresholds in lists(&[0, 1, 2, 3, 4], 2) {
                let mut sides = BothSides::new();
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
