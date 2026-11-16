// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel_spec_model::vocabulary::variant::Variant;
use serde_json::json;

#[test]
fn of_a_name_and_a_payload_is_an_object_of_one_member() {
    // Arrange & Act
    let value = Variant::of("VoteCounted", json!({"count": "1"}));

    // Assert
    assert_eq!(value, json!({"VoteCounted": {"count": "1"}}));
}

#[test]
fn unit_of_a_name_carries_the_empty_object() {
    // Arrange & Act
    let value = Variant::unit("BallotOpened");

    // Assert
    assert_eq!(value, json!({"BallotOpened": {}}));
}
