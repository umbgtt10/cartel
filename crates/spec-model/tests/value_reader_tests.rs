// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::states::States;
use cartel_spec_model::value_reader::ValueReader;
use serde_json::Value;
use serde_json::json;

#[test]
fn choice_of_hexadecimal_digits_is_their_bytes() {
    // Arrange & Act
    let choices = [json!("0aff"), json!("")].map(|value| ValueReader::choice(&value));

    // Assert
    assert_eq!(
        choices,
        [States::choice(&[0x0a, 0xff]), States::choice(&[])]
    );
}

#[test]
fn list_of_an_array_is_its_items() {
    // Arrange
    let value = json!(["a", "b"]);

    // Act
    let items: Vec<&Value> = ValueReader::list(&value).collect();

    // Assert
    assert_eq!(items, vec![&json!("a"), &json!("b")]);
}

#[test]
fn number_of_decimal_digits_is_the_u64_they_write() {
    // Arrange & Act
    let number = ValueReader::number(&json!("18446744073709551615"));

    // Assert
    assert_eq!(number, u64::MAX);
}

#[test]
fn numbers_of_an_array_are_each_item_read() {
    // Arrange & Act
    let numbers = ValueReader::numbers(&json!(["3", "0", "12"]));

    // Assert
    assert_eq!(numbers, vec![3, 0, 12]);
}

#[test]
fn string_of_a_string_is_its_text() {
    // Arrange
    let value = json!("aa");

    // Act
    let text = ValueReader::string(&value);

    // Assert
    assert_eq!(text, "aa");
}
