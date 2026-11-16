// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The exhaustive matrix tier, MeasureWatermark: every list of up to four
// members drawn from three, repeats included, each at a position from 0 to 3,
// at every threshold from 0 to 5 -- the crate's receipt against the
// specification's model's, and every row of the command reached.

use crate::support::both_sides::BothSides;
use cartel::command::Command;
use cartel::member_position::MemberPosition;
use serde_json::Value;
use std::collections::BTreeSet;

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

// Every list of up to four members from 1 to 3, each at a position from 0 to 3.
fn lists() -> Vec<Vec<MemberPosition>> {
    let pairs: Vec<MemberPosition> = (1..=3_u64)
        .flat_map(|member| (0..=3_u64).map(move |position| MemberPosition { member, position }))
        .collect();
    let mut found = vec![Vec::new()];
    let mut last = vec![Vec::new()];
    for _ in 0..4 {
        last = last
            .iter()
            .flat_map(|list: &Vec<MemberPosition>| {
                pairs.iter().map(move |pair| {
                    let mut longer = list.clone();
                    longer.push(*pair);
                    longer
                })
            })
            .collect();
        found.extend(last.iter().cloned());
    }
    found
}

#[test]
fn process_of_every_measure_answers_as_the_model_does() {
    // Arrange
    let mut departures = Vec::new();
    let mut reached = BTreeSet::new();
    let mut sides = BothSides::new();

    // Act
    for positions in lists() {
        for threshold in 0..=5 {
            let command = Command::MeasureWatermark {
                positions: positions.clone(),
                threshold,
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

    // Assert
    assert!(
        departures.is_empty(),
        "{} departures, the first {:?}",
        departures.len(),
        departures.first()
    );
    assert_eq!(
        reached,
        ["RepeatedMember", "ThresholdOutOfRange", "WatermarkMeasured"]
            .map(String::from)
            .into_iter()
            .collect()
    );
}
