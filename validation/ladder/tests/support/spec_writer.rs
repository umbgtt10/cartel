// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::ballot::Ballot;
use cartel::ballot_record::BallotRecord;
use cartel::bounds::Bounds;
use cartel::checkpoint::Checkpoint;
use cartel::choice::Choice;
use cartel::close_point::ClosePoint;
use cartel::command::Command;
use cartel::origin::Origin;
use cartel::output::Output;
use cartel::receipt::Receipt;
use cartel::rejection::Rejection;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;
use std::fmt::Write;

// Cartel's answers written in the specification's JSON, so a receipt compares
// with a vector's as the specification states it.
pub struct SpecWriter;

impl SpecWriter {
    // The checkpoint, its bounds written only when one is given and its close
    // point only once a close has moved it, as the specification's growth rule
    // asks (§1).
    pub fn checkpoint(checkpoint: &Checkpoint) -> Value {
        let mut written = json!({
            "instance": {
                "node": Self::number(checkpoint.instance.node),
                "incarnation": Self::number(checkpoint.instance.incarnation),
            },
        });
        if let Some(bounds) = Self::bounds(checkpoint.bounds) {
            written["bounds"] = bounds;
        }
        if let Some(point) = checkpoint.close_point {
            written["close_point"] = Self::point(point);
        }
        written["ballots"] = Value::Array(checkpoint.ballots.iter().map(Self::record).collect());
        written["next_receipt"] = Self::number(checkpoint.next_receipt);
        written
    }

    pub fn command(command: &Command) -> Value {
        match command {
            Command::OpenBallot {
                ballot,
                electorate,
                thresholds,
            } => json!({"OpenBallot": {
                "ballot": Self::ballot(ballot),
                "electorate": Self::numbers(electorate),
                "thresholds": Self::numbers(thresholds),
            }}),
            Command::CastVote {
                ballot,
                member,
                choice,
            } => json!({"CastVote": {
                "ballot": Self::ballot(ballot),
                "member": Self::number(*member),
                "choice": Self::choice(choice),
            }}),
            Command::CloseBallots { before } => {
                json!({"CloseBallots": {"before": Self::point(*before)}})
            }
            Command::MeasureWatermark {
                positions,
                threshold,
            } => json!({"MeasureWatermark": {
                "positions": positions
                    .iter()
                    .map(|given| json!({
                        "member": Self::number(given.member),
                        "position": Self::number(given.position),
                    }))
                    .collect::<Vec<Value>>(),
                "threshold": Self::number(*threshold),
            }}),
            Command::ExamineClaim {
                electorate,
                threshold,
                voters,
            } => json!({"ExamineClaim": {
                "electorate": Self::numbers(electorate),
                "threshold": Self::number(*threshold),
                "voters": Self::numbers(voters),
            }}),
            Command::ShowBallot { ballot } => {
                json!({"ShowBallot": {"ballot": Self::ballot(ballot)}})
            }
            Command::ListBallots => json!({"ListBallots": {}}),
            Command::ShowBooks => json!({"ShowBooks": {}}),
        }
    }

    pub fn origin(origin: &Origin) -> Value {
        let Origin::Fresh(fresh) = origin;
        let mut written = json!({"instance": {
            "node": Self::number(fresh.instance.node),
            "incarnation": Self::number(fresh.instance.incarnation),
        }});
        if let Some(bounds) = Self::bounds(fresh.bounds) {
            written["bounds"] = bounds;
        }
        json!({"Fresh": written})
    }

    pub fn receipt(receipt: &Receipt) -> Value {
        json!({"number": Self::number(receipt.number), "output": Self::output(&receipt.output)})
    }

    fn ballot(ballot: &Ballot) -> Value {
        json!({
            "question": Self::number(ballot.question),
            "round": Self::number(ballot.round),
            "phase": Self::number(ballot.phase),
        })
    }

    // The bounds given, each by its name, or none when none is given.
    fn bounds(bounds: Bounds) -> Option<Value> {
        let given: Map<String, Value> = [
            ("open_ballots", bounds.open_ballots),
            ("electorate", bounds.electorate),
            ("choice", bounds.choice),
        ]
        .into_iter()
        .filter_map(|(name, bound)| bound.map(|bound| (name.to_string(), Self::number(bound))))
        .collect();
        (!given.is_empty()).then_some(Value::Object(given))
    }

    fn choice(choice: &Choice) -> Value {
        Value::String(choice.bytes.iter().fold(String::new(), |mut digits, byte| {
            write!(digits, "{byte:02x}").expect("writing to a string");
            digits
        }))
    }

    fn number(value: u64) -> Value {
        Value::String(value.to_string())
    }

    fn numbers(values: &[u64]) -> Value {
        Value::Array(values.iter().copied().map(Self::number).collect())
    }

    fn output(output: &Output) -> Value {
        match output {
            Output::BallotOpened => json!({"BallotOpened": {}}),
            Output::VoteCounted { count } => {
                json!({"VoteCounted": {"count": Self::number(*count)}})
            }
            Output::QuorumReached {
                count,
                thresholds,
                voters,
            } => json!({"QuorumReached": {
                "count": Self::number(*count),
                "thresholds": Self::numbers(thresholds),
                "voters": Self::numbers(voters),
            }}),
            Output::BallotsClosed { ballots } => json!({"BallotsClosed": {
                "ballots": ballots.iter().map(Self::ballot).collect::<Vec<Value>>(),
            }}),
            Output::WatermarkMeasured { position } => {
                json!({"WatermarkMeasured": {"position": Self::number(*position)}})
            }
            Output::ClaimExamined {
                counted,
                outsiders,
                repeated,
                reached,
            } => json!({"ClaimExamined": {
                "counted": Self::numbers(counted),
                "outsiders": Self::numbers(outsiders),
                "repeated": Self::numbers(repeated),
                "reached": reached,
            }}),
            Output::BallotShown(record) => json!({"BallotShown": {"record": Self::record(record)}}),
            Output::BallotsListed { ballots } => {
                json!({"BallotsListed": {"ballots": ballots.iter().map(Self::ballot).collect::<Vec<Value>>()}})
            }
            Output::BooksShown(checkpoint) => {
                json!({"BooksShown": {"checkpoint": Self::checkpoint(checkpoint)}})
            }
            Output::CommandRejected(rejection) => {
                json!({"CommandRejected": Self::rejection(rejection)})
            }
        }
    }

    fn point(point: ClosePoint) -> Value {
        json!({"question": Self::number(point.question), "round": Self::number(point.round)})
    }

    fn record(record: &BallotRecord) -> Value {
        json!({
            "ballot": Self::ballot(&record.ballot),
            "electorate": Self::numbers(&record.electorate),
            "thresholds": Self::numbers(&record.thresholds),
            "tally": record
                .tally
                .iter()
                .map(|votes| json!({"choice": Self::choice(&votes.choice), "voters": Self::numbers(&votes.voters)}))
                .collect::<Vec<Value>>(),
        })
    }

    fn rejection(rejection: &Rejection) -> Value {
        match rejection {
            Rejection::EmptyElectorate => json!({"EmptyElectorate": {}}),
            Rejection::RepeatedMember { member } => {
                json!({"RepeatedMember": {"member": Self::number(*member)}})
            }
            Rejection::NoThresholds => json!({"NoThresholds": {}}),
            Rejection::ThresholdOutOfRange { threshold } => {
                json!({"ThresholdOutOfRange": {"threshold": Self::number(*threshold)}})
            }
            Rejection::RepeatedThreshold { threshold } => {
                json!({"RepeatedThreshold": {"threshold": Self::number(*threshold)}})
            }
            Rejection::BallotAlreadyOpen => json!({"BallotAlreadyOpen": {}}),
            Rejection::BallotNotOpen => json!({"BallotNotOpen": {}}),
            Rejection::NotAMember => json!({"NotAMember": {}}),
            Rejection::AlreadyCounted => json!({"AlreadyCounted": {}}),
            Rejection::Equivocation { counted } => {
                json!({"Equivocation": {"counted": Self::choice(counted)}})
            }
            Rejection::BallotClosed => json!({"BallotClosed": {}}),
            Rejection::CloseBehind { close_point } => {
                json!({"CloseBehind": {"close_point": Self::point(*close_point)}})
            }
            Rejection::ElectorateTooLarge { bound } => {
                json!({"ElectorateTooLarge": {"bound": Self::number(*bound)}})
            }
            Rejection::BooksFull { bound } => json!({"BooksFull": {"bound": Self::number(*bound)}}),
            Rejection::ChoiceTooLong { bound } => {
                json!({"ChoiceTooLong": {"bound": Self::number(*bound)}})
            }
        }
    }
}
