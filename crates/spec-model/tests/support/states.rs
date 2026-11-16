// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel_spec_model::ballot_key::BallotKey;
use cartel_spec_model::bounds::Bounds;
use cartel_spec_model::choice::Choice;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::open_ballot::OpenBallot;
use cartel_spec_model::point::Point;
use cartel_spec_model::vote::Vote;

// Node 2, incarnation 3, receipt 5 next: an instance as a test sets it up.
pub struct States;

impl States {
    pub fn ballot(
        key: BallotKey,
        electorate: &[u64],
        thresholds: &[u64],
        votes: &[(u64, &[u8])],
    ) -> OpenBallot {
        OpenBallot {
            key,
            electorate: electorate.to_vec(),
            thresholds: thresholds.to_vec(),
            votes: votes
                .iter()
                .map(|(member, choice)| Vote {
                    member: *member,
                    choice: Self::choice(choice),
                })
                .collect(),
        }
    }

    pub fn choice(bytes: &[u8]) -> Choice {
        Choice {
            bytes: bytes.to_vec(),
        }
    }

    pub fn holding(ballots: Vec<OpenBallot>) -> InstanceState {
        InstanceState {
            ballots,
            ..Self::instance()
        }
    }

    pub fn instance() -> InstanceState {
        InstanceState {
            node: 2,
            incarnation: 3,
            bounds: Bounds::default(),
            close_point: None,
            ballots: Vec::new(),
            next_receipt: 5,
        }
    }

    pub fn point(question: u64, round: u64) -> Point {
        Point { question, round }
    }

    pub fn key(question: u64, round: u64, phase: u64) -> BallotKey {
        BallotKey {
            question,
            round,
            phase,
        }
    }
}
