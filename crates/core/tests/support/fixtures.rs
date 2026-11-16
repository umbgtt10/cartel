// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::ballot::Ballot;
use cartel::choice::Choice;
use cartel::instance_id::InstanceId;
use std::collections::BTreeSet;

// The values the unit tier builds its cases from.
pub struct Fixtures;

impl Fixtures {
    pub fn ballot(question: u64, round: u64, phase: u64) -> Ballot {
        Ballot {
            question,
            round,
            phase,
        }
    }

    pub fn choice(bytes: &[u8]) -> Choice {
        Choice {
            bytes: bytes.to_vec(),
        }
    }

    pub fn instance() -> InstanceId {
        InstanceId {
            node: 2,
            incarnation: 3,
        }
    }

    pub fn set(numbers: &[u64]) -> BTreeSet<u64> {
        numbers.iter().copied().collect()
    }
}
