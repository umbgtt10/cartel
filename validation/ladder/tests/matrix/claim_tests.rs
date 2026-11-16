// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The exhaustive matrix tier, ExamineClaim: every electorate of up to three
// members drawn from three, repeats included, at every threshold from 0 to
// 4, against every list of up to three voters drawn from four -- repeats and
// an outsider included -- the crate's receipt against the specification's
// model's, every row of the command reached, and claims both reaching and
// falling short.

use crate::support::both_sides::BothSides;
use cartel::bounds::Bounds;
use cartel::command::Command;
use serde_json::Value;
use std::collections::BTreeSet;

// The output's name, the rejection's for a refusal, and for an examined claim
// whether it reached its threshold.
fn kind(receipt: &Value) -> String {
    let (name, payload) = receipt["output"]
        .as_object()
        .and_then(|output| output.iter().next())
        .expect("an output");
    match name.as_str() {
        "CommandRejected" => payload
            .as_object()
            .and_then(|rejection| rejection.keys().next())
            .expect("a rejection")
            .clone(),
        _ => format!("{name}, reached {}", payload["reached"]),
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

#[test]
fn process_of_every_claim_answers_as_the_model_does() {
    // Arrange
    let mut departures = Vec::new();
    let mut reached = BTreeSet::new();
    let mut sides = BothSides::new(Bounds::default());
    let voter_lists = lists(&[1, 2, 3, 4], 3);

    // Act
    for electorate in lists(&[1, 2, 3], 3) {
        for threshold in 0..=4 {
            for voters in &voter_lists {
                let command = Command::ExamineClaim {
                    electorate: electorate.clone(),
                    threshold,
                    voters: voters.clone(),
                };
                let (from_crate, from_model) = sides.process(&command);
                if from_crate["output"] != from_model["output"] {
                    departures.push(format!(
                        "{command:?}: crate {from_crate}, model {from_model}"
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
            "ClaimExamined, reached false",
            "ClaimExamined, reached true",
            "EmptyElectorate",
            "RepeatedMember",
            "ThresholdOutOfRange",
        ]
        .map(String::from)
        .into_iter()
        .collect()
    );
}
