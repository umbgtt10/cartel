// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use serde_json::json;
use serde_json::Value;

use crate::call::Call;
use crate::lookup::Lookup;
use crate::numbers::Numbers;
use crate::view::View;
use crate::vocabulary::speller::Speller;
use crate::vocabulary::spelling::Spelling;
use crate::vocabulary::variant::Variant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectionName {
    EmptyElectorate,
    RepeatedMember,
    NoThresholds,
    ThresholdOutOfRange,
    RepeatedThreshold,
    BallotAlreadyOpen,
    BallotNotOpen,
    NotAMember,
    AlreadyCounted,
    Equivocation,
}

impl RejectionName {
    pub const SPELLING: Spelling<RejectionName> = Spelling {
        kind: "a rejection",
        words: &[
            (RejectionName::EmptyElectorate, "EmptyElectorate"),
            (RejectionName::RepeatedMember, "RepeatedMember"),
            (RejectionName::NoThresholds, "NoThresholds"),
            (RejectionName::ThresholdOutOfRange, "ThresholdOutOfRange"),
            (RejectionName::RepeatedThreshold, "RepeatedThreshold"),
            (RejectionName::BallotAlreadyOpen, "BallotAlreadyOpen"),
            (RejectionName::BallotNotOpen, "BallotNotOpen"),
            (RejectionName::NotAMember, "NotAMember"),
            (RejectionName::AlreadyCounted, "AlreadyCounted"),
            (RejectionName::Equivocation, "Equivocation"),
        ],
    };

    // What a refusal carries (§5, §6): the lowest member or threshold at
    // fault, or the choice a member is already a voter of -- a refusal changes
    // nothing, so the state is the one the command met.
    #[must_use]
    pub fn encoded(self, call: &Call) -> Value {
        let name = Speller::name(&Self::SPELLING, self);
        let command = call.command;
        match self {
            RejectionName::RepeatedMember => {
                let member =
                    Numbers::lowest_repeated(command.electorate.as_deref().expect("an electorate"));
                Variant::of(name, json!({"member": member.map(View::number)}))
            }
            RejectionName::ThresholdOutOfRange => {
                let electorate = command.electorate.as_deref().expect("an electorate");
                let thresholds = command.thresholds.as_deref().expect("thresholds");
                let threshold = Numbers::lowest_out_of_range(thresholds, electorate.len());
                Variant::of(name, json!({"threshold": threshold.map(View::number)}))
            }
            RejectionName::RepeatedThreshold => {
                let threshold =
                    Numbers::lowest_repeated(command.thresholds.as_deref().expect("thresholds"));
                Variant::of(name, json!({"threshold": threshold.map(View::number)}))
            }
            RejectionName::Equivocation => {
                let ballot = Lookup::named(call.before, command);
                let counted = Lookup::choice(ballot, command.member.expect("a member"))
                    .expect("a choice counted");
                Variant::of(name, json!({"counted": View::choice(counted)}))
            }
            _ => Variant::unit(name),
        }
    }
}
