// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::ballot::Ballot;
use cartel::choice::Choice;
use cartel::close_point::ClosePoint;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::instance_id::InstanceId;
use cartel::origin::Origin;
use serde_json::Value;

// The specification's JSON read into cartel's own types: a command and an
// origin, as a caller would hand them to an instance.
pub struct SpecReader;

impl SpecReader {
    pub fn command(value: &Value) -> Command {
        let (name, payload) = value
            .as_object()
            .and_then(|object| object.iter().next())
            .expect("a command: an object of one member");
        match name.as_str() {
            "OpenBallot" => Command::OpenBallot {
                ballot: Self::ballot(&payload["ballot"]),
                electorate: Self::numbers(&payload["electorate"]),
                thresholds: Self::numbers(&payload["thresholds"]),
            },
            "CastVote" => Command::CastVote {
                ballot: Self::ballot(&payload["ballot"]),
                member: Self::number(&payload["member"]),
                choice: Self::choice(&payload["choice"]),
            },
            "CloseBallots" => Command::CloseBallots {
                before: ClosePoint {
                    question: Self::number(&payload["before"]["question"]),
                    round: Self::number(&payload["before"]["round"]),
                },
            },
            "ShowBallot" => Command::ShowBallot {
                ballot: Self::ballot(&payload["ballot"]),
            },
            "ListBallots" => Command::ListBallots,
            "ShowBooks" => Command::ShowBooks,
            other => panic!("no command named {other}"),
        }
    }

    pub fn origin(value: &Value) -> Origin {
        let instance = &value["Fresh"]["instance"];
        Origin::Fresh(Fresh {
            instance: InstanceId {
                node: Self::number(&instance["node"]),
                incarnation: Self::number(&instance["incarnation"]),
            },
        })
    }

    fn ballot(value: &Value) -> Ballot {
        Ballot {
            question: Self::number(&value["question"]),
            round: Self::number(&value["round"]),
            phase: Self::number(&value["phase"]),
        }
    }

    fn choice(value: &Value) -> Choice {
        let digits = value.as_str().expect("a choice's digits");
        Choice {
            bytes: (0..digits.len())
                .step_by(2)
                .map(|at| u8::from_str_radix(&digits[at..at + 2], 16).expect("a byte"))
                .collect(),
        }
    }

    fn number(value: &Value) -> u64 {
        value
            .as_str()
            .expect("a u64's digits")
            .parse()
            .expect("a u64")
    }

    fn numbers(value: &Value) -> Vec<u64> {
        value
            .as_array()
            .expect("a list")
            .iter()
            .map(Self::number)
            .collect()
    }
}
