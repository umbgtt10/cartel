// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use serde_json::Value;
use serde_json::from_str;
use std::fs::read_to_string;
use std::path::PathBuf;

// The specification's decision table, at the repository root: read by the
// tests, never by the model, which is given it.
pub struct SpecFiles;

impl SpecFiles {
    pub fn matrix() -> Value {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../spec/matrix.json");
        from_str(&read_to_string(&path).expect("the matrix")).expect("the matrix's JSON")
    }
}
