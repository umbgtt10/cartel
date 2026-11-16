// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeSet;

// The lowest number given more than once: the member an electorate or a
// measure's positions name twice, the threshold an opening gives twice.
pub struct Repeats;

impl Repeats {
    pub fn lowest(numbers: &[u64]) -> Option<u64> {
        let mut seen = BTreeSet::new();
        let mut repeated = BTreeSet::new();
        for number in numbers {
            if !seen.insert(*number) {
                repeated.insert(*number);
            }
        }
        repeated.first().copied()
    }
}
