// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

// The conformance tier, the crate: every vector run against cartel through its
// one entry point -- each step's receipt, the closing ShowBooks's checkpoint,
// and every call reported to the observer as it was answered.

use crate::support::recording_observer::Calls;
use crate::support::recording_observer::RecordingObserver;
use crate::support::spec_files::SpecFiles;
use crate::support::spec_reader::SpecReader;
use crate::support::spec_writer::SpecWriter;
use cartel::cartel::Cartel;
use cartel::command::Command;
use cartel::no_op_observer::NoOpObserver;
use cartel::observer::Observer;
use serde_json::json;
use serde_json::Value;

fn departures(name: &str, vector: &Value) -> Vec<String> {
    let mut cartel = Cartel::new(
        SpecReader::origin(&vector["origin"]),
        Box::new(NoOpObserver),
    );
    let mut found: Vec<String> = steps(vector)
        .iter()
        .enumerate()
        .filter_map(|(index, step)| {
            let answered =
                SpecWriter::receipt(&cartel.process(SpecReader::command(&step["command"])));
            (answered != step["receipt"]).then(|| {
                format!(
                    "{name}, step {}: cartel answers {answered}, the vector {}",
                    index + 1,
                    step["receipt"]
                )
            })
        })
        .collect();
    let closing = SpecWriter::receipt(&cartel.process(Command::ShowBooks));
    let expected = json!({"BooksShown": {"checkpoint": vector["final"]}});
    if closing["output"] != expected {
        found.push(format!(
            "{name}: cartel closes with {}, the vector {expected}",
            closing["output"]
        ));
    }
    found
}

fn misreport(name: &str, vector: &Value) -> Option<String> {
    let calls = Calls::default();
    let observer: Box<dyn Observer> = Box::new(RecordingObserver::new(calls.clone()));
    let mut cartel = Cartel::new(SpecReader::origin(&vector["origin"]), observer);
    let answered: Vec<Value> = steps(vector)
        .iter()
        .map(|step| {
            let command = SpecReader::command(&step["command"]);
            let receipt = cartel.process(command.clone());
            json!([format!("{command:?}"), SpecWriter::receipt(&receipt)])
        })
        .collect();
    let reported: Vec<Value> = calls
        .borrow()
        .iter()
        .map(|(command, receipt)| json!([format!("{command:?}"), SpecWriter::receipt(receipt)]))
        .collect();
    (reported != answered).then(|| format!("{name}: reported {reported:?}, answered {answered:?}"))
}

fn steps(vector: &Value) -> &Vec<Value> {
    vector["steps"].as_array().expect("a vector's steps")
}

#[test]
fn process_of_every_vector_answers_its_receipts_and_its_final_state() {
    // Arrange
    let vectors = SpecFiles::vectors();

    // Act
    let departures: Vec<String> = vectors
        .iter()
        .flat_map(|(name, vector)| departures(name, vector))
        .collect();

    // Assert
    assert_eq!(vectors.len(), 42);
    assert!(departures.is_empty(), "{departures:#?}");
}

#[test]
fn process_of_every_vector_reports_every_call_to_the_observer_as_answered() {
    // Arrange
    let vectors = SpecFiles::vectors();

    // Act
    let misreports: Vec<String> = vectors
        .iter()
        .filter_map(|(name, vector)| misreport(name, vector))
        .collect();

    // Assert
    assert!(misreports.is_empty(), "{misreports:#?}");
}
