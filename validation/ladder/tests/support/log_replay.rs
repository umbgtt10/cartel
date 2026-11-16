// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::replay_report::ReplayReport;
use crate::support::spec_reader::SpecReader;
use crate::support::spec_writer::SpecWriter;
use cartel::cartel::Cartel;
use cartel::no_op_observer::NoOpObserver;
use serde_json::Value;

// A log -- an origin and lines in the specification's step form, a command
// and the receipt it was answered with -- replayed into a fresh instance: each
// line's command given, its receipt compared, and the gaps in the receipts'
// numbers found. A line lost is a gap; the lines after it are replayed as
// they come, and whether they rebuild is the report's to say.
pub struct LogReplay;

impl LogReplay {
    pub fn replay(origin: &Value, lines: &[Value]) -> ReplayReport {
        let mut cartel = Cartel::new(SpecReader::origin(origin), Box::new(NoOpObserver));
        let mut departures = Vec::new();
        let mut reconstructed = 0;
        for line in lines {
            let answered =
                SpecWriter::receipt(&cartel.process(SpecReader::command(&line["command"])));
            if answered == line["receipt"] {
                reconstructed += 1;
            } else {
                departures.push(format!(
                    "{}: answered {answered}",
                    line["receipt"]["number"]
                ));
            }
        }
        ReplayReport {
            reconstructed,
            total: lines.len(),
            gaps: Self::gaps(lines),
            departures,
        }
    }

    fn gaps(lines: &[Value]) -> Vec<u64> {
        let numbers: Vec<u64> = lines
            .iter()
            .map(|line| {
                line["receipt"]["number"]
                    .as_str()
                    .and_then(|digits| digits.parse().ok())
                    .expect("a receipt's number")
            })
            .collect();
        numbers
            .windows(2)
            .flat_map(|pair| pair[0] + 1..pair[1])
            .collect()
    }
}
