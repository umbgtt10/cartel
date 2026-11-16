// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;
use serde_json::Value;

use crate::ballot_key::BallotKey;
use crate::command::Command;
use crate::instance_state::InstanceState;
use crate::point::Point;
use crate::table::decision_table::DecisionTable;
use crate::table::row::Row;
use crate::value_reader::ValueReader;
use crate::vocabulary::command_name::CommandName;
use crate::vocabulary::output_name::OutputName;
use crate::vocabulary::rejection_name::RejectionName;
use crate::vocabulary::speller::Speller;

// The specification's encoding read: a command, an origin, the decision
// table. What it reads is valid against the schema, or outside the
// specification, and a reading of what is outside it panics.
pub struct SpecReader;

impl SpecReader {
    #[must_use]
    pub fn command(value: &Value) -> Command {
        let (name, payload) = value
            .as_object()
            .and_then(|object| object.iter().next())
            .expect("a command: an object of one member");
        Command {
            name: Speller::of(&CommandName::SPELLING, name),
            before: payload.get("before").map(Self::point),
            ballot: payload.get("ballot").map(Self::ballot),
            electorate: payload.get("electorate").map(ValueReader::numbers),
            thresholds: payload.get("thresholds").map(ValueReader::numbers),
            member: payload.get("member").map(ValueReader::number),
            choice: payload.get("choice").map(ValueReader::choice),
        }
    }

    // A fresh instance (§2) holds no ballot, and its first receipt is number 1.
    #[must_use]
    pub fn origin(value: &Value) -> InstanceState {
        let instance = &value["Fresh"]["instance"];
        InstanceState {
            node: ValueReader::number(&instance["node"]),
            incarnation: ValueReader::number(&instance["incarnation"]),
            close_point: None,
            ballots: Vec::new(),
            next_receipt: 1,
        }
    }

    #[must_use]
    pub fn table(value: &Value) -> DecisionTable {
        DecisionTable::new(ValueReader::list(&value["rows"]).map(Self::row).collect())
    }

    fn ballot(value: &Value) -> BallotKey {
        BallotKey {
            question: ValueReader::number(&value["question"]),
            round: ValueReader::number(&value["round"]),
            phase: ValueReader::number(&value["phase"]),
        }
    }

    fn point(value: &Value) -> Point {
        Point {
            question: ValueReader::number(&value["question"]),
            round: ValueReader::number(&value["round"]),
        }
    }

    fn row(value: &Value) -> Row {
        Row {
            id: String::from(ValueReader::string(&value["id"])),
            commands: ValueReader::list(&value["commands"])
                .map(|command| Speller::of(&CommandName::SPELLING, ValueReader::string(command)))
                .collect(),
            output: Speller::of(&OutputName::SPELLING, ValueReader::string(&value["output"])),
            rejection: value.get("rejection").map(|rejection| {
                Speller::of(&RejectionName::SPELLING, ValueReader::string(rejection))
            }),
        }
    }
}
