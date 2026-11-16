// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

// What §5 asks of a command's lists of numbers: the lowest given more than
// once, and the lowest threshold out of range for an electorate's size.
pub struct Numbers;

impl Numbers {
    #[must_use]
    pub fn lowest_out_of_range(thresholds: &[u64], size: usize) -> Option<u64> {
        thresholds
            .iter()
            .copied()
            .filter(|threshold| {
                *threshold == 0 || usize::try_from(*threshold).map_or(true, |at| at > size)
            })
            .min()
    }

    #[must_use]
    pub fn lowest_repeated(numbers: &[u64]) -> Option<u64> {
        let mut sorted: Vec<u64> = numbers.to_vec();
        sorted.sort_unstable();
        sorted
            .windows(2)
            .find(|pair| pair[0] == pair[1])
            .map(|pair| pair[0])
    }
}
