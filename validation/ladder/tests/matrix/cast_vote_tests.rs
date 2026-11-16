// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The exhaustive matrix tier, CastVote: a ballot open over every electorate
// drawn from three members, with every set of thresholds it admits and every
// way its members may already have voted two choices, given every vote of
// four members for three choices -- the empty one among them -- in it and in
// a ballot not open: the crate's receipt and the books it leaves against the
// specification's model's, and every row of the command reached.

use crate::support::both_sides::BothSides;
use cartel::ballot::Ballot;
use cartel::choice::Choice;
use cartel::command::Command;
use serde_json::Value;
use std::collections::BTreeSet;

const NOT_OPEN: Ballot = Ballot {
    question: 1,
    round: 0,
    phase: 1,
};
const OPEN: Ballot = Ballot {
    question: 1,
    round: 0,
    phase: 0,
};

// Every way each member may have voted: not at all, or one of two choices.
fn assignments(electorate: &[u64]) -> Vec<Vec<(u64, Option<u8>)>> {
    electorate.iter().fold(vec![Vec::new()], |found, member| {
        found
            .iter()
            .flat_map(|assigned| {
                [None, Some(0xaa), Some(0xbb)].map(|vote| {
                    let mut longer = assigned.clone();
                    longer.push((*member, vote));
                    longer
                })
            })
            .collect()
    })
}

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

// Every non-empty subset of the values given.
fn subsets(values: &[u64]) -> Vec<Vec<u64>> {
    (1..1_u32 << values.len())
        .map(|mask| {
            values
                .iter()
                .enumerate()
                .filter(|(at, _)| mask & (1 << at) != 0)
                .map(|(_, value)| *value)
                .collect()
        })
        .collect()
}

fn vote(ballot: Ballot, member: u64, choice: &[u8]) -> Command {
    Command::CastVote {
        ballot,
        member,
        choice: Choice {
            bytes: choice.to_vec(),
        },
    }
}

#[test]
fn process_of_every_vote_answers_as_the_model_does() {
    // Arrange
    let mut departures = Vec::new();
    let mut reached = BTreeSet::new();
    let probes: Vec<Command> = [OPEN, NOT_OPEN]
        .into_iter()
        .flat_map(|ballot| {
            (1..=4_u64).flat_map(move |member| {
                [&[][..], &[0xaa][..], &[0xbb][..]].map(|choice| vote(ballot, member, choice))
            })
        })
        .collect();

    // Act
    for electorate in subsets(&[1, 2, 3]) {
        let size = electorate.len() as u64;
        for thresholds in subsets(&(1..=size).collect::<Vec<u64>>()) {
            for assigned in assignments(&electorate) {
                for probe in &probes {
                    let mut sides = BothSides::new();
                    let _ = sides.process(&Command::OpenBallot {
                        ballot: OPEN,
                        electorate: electorate.clone(),
                        thresholds: thresholds.clone(),
                    });
                    for (member, voted) in &assigned {
                        if let Some(byte) = voted {
                            let _ = sides.process(&vote(OPEN, *member, &[*byte]));
                        }
                    }
                    let answered = sides.process(probe);
                    let left = sides.process(&Command::ShowBooks);
                    if answered.0 != answered.1 || left.0 != left.1 {
                        departures.push(format!(
                            "{probe:?} after {assigned:?}: crate {answered:?}, books {left:?}"
                        ));
                    }
                    reached.insert(kind(&answered.0));
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
    assert_eq!(
        reached,
        [
            "AlreadyCounted",
            "BallotNotOpen",
            "Equivocation",
            "NotAMember",
            "QuorumReached",
            "VoteCounted",
        ]
        .map(String::from)
        .into_iter()
        .collect()
    );
}
