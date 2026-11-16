// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::spec_files::SpecFiles;
use crate::support::spec_writer::SpecWriter;
use cartel::command::Command;
use cartel::origin::Origin;
use cartel_spec_model::model::Model;
use cartel_spec_model::spec_reader::SpecReader;
use serde_json::Value;

// The specification's model, given the crate's origins and commands written in
// the specification's encoding, and answering in it: the other side of a tier
// that sets the crate against the model.
pub struct ModelSide {
    model: Model,
}

impl ModelSide {
    pub fn new(origin: &Origin) -> Self {
        Self {
            model: Model::new(
                MATRIX.with(SpecReader::table),
                SpecReader::origin(&SpecWriter::origin(origin)),
            ),
        }
    }

    pub fn process(&mut self, command: &Command) -> Value {
        self.model.process(&SpecWriter::command(command)).receipt
    }
}

// The decision table's file, read once a test thread: the matrix tier creates a
// model for every situation it sets the crate in.
thread_local! {
    static MATRIX: Value = SpecFiles::matrix();
}
