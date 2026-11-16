// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::member_position::MemberPosition;
use crate::output::Output;
use crate::rejection::Rejection;
use crate::rows::repeats::Repeats;
use alloc::vec::Vec;

// A measure, answered from the positions it is given and nothing the books
// keep: refused if they name a member twice or the threshold is out of range,
// and otherwise the watermark -- the highest position at least the threshold's
// number of members have reached, which is the threshold's place among the
// positions from the highest down.
pub struct WatermarkRows<'a> {
    positions: &'a [MemberPosition],
    threshold: u64,
}

impl<'a> WatermarkRows<'a> {
    pub fn new(positions: &'a [MemberPosition], threshold: u64) -> Self {
        Self {
            positions,
            threshold,
        }
    }

    pub fn answer(&self) -> Output {
        let members: Vec<u64> = self.positions.iter().map(|given| given.member).collect();
        if let Some(member) = Repeats::lowest(&members) {
            return Output::CommandRejected(Rejection::RepeatedMember { member });
        }
        if !(1..=members.len() as u64).contains(&self.threshold) {
            return Output::CommandRejected(Rejection::ThresholdOutOfRange {
                threshold: self.threshold,
            });
        }
        let mut reached: Vec<u64> = self.positions.iter().map(|given| given.position).collect();
        reached.sort_unstable_by(|first, second| second.cmp(first));
        Output::WatermarkMeasured {
            position: reached[(self.threshold - 1) as usize],
        }
    }
}
