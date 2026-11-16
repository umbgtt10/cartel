// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use serde_json::json;
use serde_json::Value;

use crate::call::Call;
use crate::figures::Figures;
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
    BallotClosed,
    CloseBehind,
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
            (RejectionName::BallotClosed, "BallotClosed"),
            (RejectionName::CloseBehind, "CloseBehind"),
        ],
    };

    // What a refusal carries (§5, §6, §10): the lowest member or threshold at
    // fault, the choice a member is already a voter of, or the close point --
    // a refusal changes nothing, so the state is the one the command met.
    #[must_use]
    pub fn encoded(self, call: &Call) -> Value {
        let name = Speller::name(&Self::SPELLING, self);
        let command = call.command;
        match self {
            RejectionName::RepeatedMember => {
                let member = Numbers::lowest_repeated(&Figures::members(command));
                Variant::of(name, json!({"member": member.map(View::number)}))
            }
            RejectionName::ThresholdOutOfRange => {
                let size = Figures::members(command).len();
                let threshold = Numbers::lowest_out_of_range(&Figures::thresholds(command), size);
                Variant::of(name, json!({"threshold": threshold.map(View::number)}))
            }
            RejectionName::RepeatedThreshold => {
                let threshold = Numbers::lowest_repeated(&Figures::thresholds(command));
                Variant::of(name, json!({"threshold": threshold.map(View::number)}))
            }
            RejectionName::Equivocation => {
                let ballot = Lookup::named(call.before, command);
                let counted = Lookup::choice(ballot, command.member.expect("a member"))
                    .expect("a choice counted");
                Variant::of(name, json!({"counted": View::choice(counted)}))
            }
            RejectionName::CloseBehind => {
                let point = call.before.close_point.expect("a close point");
                Variant::of(name, json!({"close_point": View::point(point)}))
            }
            _ => Variant::unit(name),
        }
    }
}
