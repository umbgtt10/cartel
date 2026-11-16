// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

//! The testing ladder's memory tier: a counting allocator, measured where the
//! allocator sees every `Layout` rather than where the books believe they
//! are, and a run of ballots one after another that it measures. Not
//! published: it exists for its tests.

pub mod allocation_counter;
pub mod ballot_sequence;
pub mod counting_allocator;
pub mod memory_profile;
