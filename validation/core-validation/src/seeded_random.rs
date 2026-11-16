// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// A deterministic source of numbers, SplitMix64: a simulation run from one
// seed runs alike every time, so a failure replays from its seed.
pub struct SeededRandom {
    state: u64,
}

impl SeededRandom {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    // A number below the bound given, which is above 0.
    pub fn below(&mut self, bound: u64) -> u64 {
        self.draw() % bound
    }

    // Whether an event of the chance given, in percent, happens.
    pub fn chance(&mut self, percent: u64) -> bool {
        self.below(100) < percent
    }

    pub fn draw(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut mixed = self.state;
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        mixed ^ (mixed >> 31)
    }
}
