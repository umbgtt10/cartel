// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::model_side::ModelSide;
use crate::support::spec_writer::SpecWriter;
use cartel::bounds::Bounds;
use cartel::cartel::Cartel;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::instance_id::InstanceId;
use cartel::no_op_observer::NoOpObserver;
use cartel::origin::Origin;
use serde_json::Value;

const INSTANCE: InstanceId = InstanceId {
    node: 1,
    incarnation: 7,
};

// The crate and the specification's model, created alike within the bounds
// given and given the same commands: every receipt each answers, in the
// specification's encoding, side by side.
pub struct BothSides {
    crate_side: Cartel,
    model_side: ModelSide,
}

impl BothSides {
    pub fn new(bounds: Bounds) -> Self {
        let origin = Origin::Fresh(Fresh {
            instance: INSTANCE,
            bounds,
        });
        Self {
            crate_side: Cartel::new(origin, Box::new(NoOpObserver)),
            model_side: ModelSide::new(&origin),
        }
    }

    // The crate's receipt and the model's, to the same command.
    pub fn process(&mut self, command: &Command) -> (Value, Value) {
        let from_crate = SpecWriter::receipt(&self.crate_side.process(command.clone()));
        (from_crate, self.model_side.process(command))
    }
}
