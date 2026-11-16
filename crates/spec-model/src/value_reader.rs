// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;
use core::str::from_utf8;
use serde_json::Value;

use crate::choice::Choice;

// The specification's encoding read at its smallest (§1): a string, a u64
// written in decimal digits, a list, and a choice written in hexadecimal
// digits. A value of another shape is outside the specification, and reading
// it panics.
pub struct ValueReader;

impl ValueReader {
    #[must_use]
    pub fn choice(value: &Value) -> Choice {
        let digits = Self::string(value).as_bytes();
        Choice {
            bytes: digits
                .chunks(2)
                .map(|pair| {
                    let pair = from_utf8(pair).expect("two hexadecimal digits");
                    u8::from_str_radix(pair, 16).expect("a byte in hexadecimal digits")
                })
                .collect(),
        }
    }

    pub fn list(value: &Value) -> impl Iterator<Item = &Value> {
        value.as_array().expect("a list").iter()
    }

    #[must_use]
    pub fn number(value: &Value) -> u64 {
        Self::string(value)
            .parse()
            .expect("a u64 in decimal digits")
    }

    #[must_use]
    pub fn numbers(value: &Value) -> Vec<u64> {
        Self::list(value).map(Self::number).collect()
    }

    #[must_use]
    pub fn string(value: &Value) -> &str {
        value.as_str().expect("a string")
    }
}
