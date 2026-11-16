// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Write;
use serde_json::json;
use serde_json::Value;

use crate::ballot_key::BallotKey;
use crate::choice::Choice;
use crate::instance_state::InstanceState;
use crate::open_ballot::OpenBallot;
use crate::point::Point;

// What an instance shows of its books, in the specification's encoding: every
// u64 a string of decimal digits, a choice its hexadecimal digits (§1), and
// everything listed in canonical order.
pub struct View;

impl View {
    #[must_use]
    pub fn ballot(key: BallotKey) -> Value {
        json!({
            "question": Self::number(key.question),
            "round": Self::number(key.round),
            "phase": Self::number(key.phase),
        })
    }

    // The open ballots (§8), in canonical order.
    #[must_use]
    pub fn ballots(state: &InstanceState) -> Value {
        let mut keys: Vec<BallotKey> = state.ballots.iter().map(|ballot| ballot.key).collect();
        keys.sort_unstable();
        Value::Array(keys.into_iter().map(Self::ballot).collect())
    }

    // The checkpoint (§9): the close point only once a close has moved it (§1).
    #[must_use]
    pub fn checkpoint(state: &InstanceState) -> Value {
        let mut ballots: Vec<&OpenBallot> = state.ballots.iter().collect();
        ballots.sort_unstable_by_key(|ballot| ballot.key);
        let mut checkpoint = json!({
            "instance": {
                "node": Self::number(state.node),
                "incarnation": Self::number(state.incarnation),
            },
        });
        if let Some(point) = state.close_point {
            checkpoint["close_point"] = Self::point(point);
        }
        checkpoint["ballots"] = Value::Array(ballots.into_iter().map(Self::record).collect());
        checkpoint["next_receipt"] = Self::number(state.next_receipt);
        checkpoint
    }

    #[must_use]
    pub fn choice(choice: &Choice) -> Value {
        let mut digits = String::new();
        for byte in &choice.bytes {
            write!(digits, "{byte:02x}").expect("writing to a string");
        }
        Value::String(digits)
    }

    #[must_use]
    pub fn number(value: u64) -> Value {
        Value::String(value.to_string())
    }

    #[must_use]
    pub fn numbers(values: &[u64]) -> Value {
        let mut sorted = values.to_vec();
        sorted.sort_unstable();
        Value::Array(sorted.into_iter().map(Self::number).collect())
    }

    #[must_use]
    pub fn point(point: Point) -> Value {
        json!({
            "question": Self::number(point.question),
            "round": Self::number(point.round),
        })
    }

    // A ballot's record (§4): its electorate and thresholds in ascending
    // order, its tally in the choices' order, each choice's voters ascending.
    #[must_use]
    pub fn record(ballot: &OpenBallot) -> Value {
        let mut tally: BTreeMap<&Choice, Vec<u64>> = BTreeMap::new();
        for vote in &ballot.votes {
            tally.entry(&vote.choice).or_default().push(vote.member);
        }
        json!({
            "ballot": Self::ballot(ballot.key),
            "electorate": Self::numbers(&ballot.electorate),
            "thresholds": Self::numbers(&ballot.thresholds),
            "tally": tally
                .into_iter()
                .map(|(choice, voters)| json!({"choice": Self::choice(choice), "voters": Self::numbers(&voters)}))
                .collect::<Vec<Value>>(),
        })
    }
}
