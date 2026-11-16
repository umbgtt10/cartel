// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeSet;

// The numbers given more than once, and the lowest of them: the member an
// electorate or a measure's positions name twice, the threshold an opening
// gives twice, the voters a claim names twice.
pub struct Repeats;

impl Repeats {
    // Every number given more than once, each once, in ascending order.
    pub fn all(numbers: &[u64]) -> BTreeSet<u64> {
        let mut seen = BTreeSet::new();
        let mut repeated = BTreeSet::new();
        for number in numbers {
            if !seen.insert(*number) {
                repeated.insert(*number);
            }
        }
        repeated
    }

    pub fn lowest(numbers: &[u64]) -> Option<u64> {
        Self::all(numbers).pop_first()
    }
}
