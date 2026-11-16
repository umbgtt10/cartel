// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::command::Command;
use crate::observer::Observer;
use crate::origin::Origin;
use crate::output::Output;
use crate::receipt::Receipt;
use crate::registry::books::Books;
use crate::rows::claim_rows::ClaimRows;
use crate::rows::watermark_rows::WatermarkRows;
use alloc::boxed::Box;

// The one entry point: every command answered by a receipt, the first row of
// the specification's matrix that applies giving the output, and reported
// with it to the observer. The books hold the state and what the commands do
// to it; this numbers the calls, sends each command where it belongs, and
// tells the observer.
pub struct Cartel {
    books: Books,
    observer: Box<dyn Observer>,
}

impl Cartel {
    #[must_use]
    pub fn new(origin: Origin, observer: Box<dyn Observer>) -> Self {
        let Origin::Fresh(fresh) = origin;
        Self {
            books: Books::new(fresh.instance, fresh.bounds),
            observer,
        }
    }

    #[must_use]
    pub fn process(&mut self, command: Command) -> Receipt {
        let number = self.books.count_call();
        let output = match &command {
            Command::OpenBallot {
                ballot,
                electorate,
                thresholds,
            } => self.books.open(*ballot, electorate, thresholds),
            Command::CastVote {
                ballot,
                member,
                choice,
            } => self.books.cast(*ballot, *member, choice.clone()),
            Command::CloseBallots { before } => self.books.close(*before),
            Command::MeasureWatermark {
                positions,
                threshold,
            } => WatermarkRows::new(positions, *threshold).answer(),
            Command::ExamineClaim {
                electorate,
                threshold,
                voters,
            } => ClaimRows::new(electorate, *threshold, voters).answer(),
            Command::ShowBallot { ballot } => self.books.query_rows().ballot(*ballot),
            Command::ListBallots => self.books.query_rows().ballots(),
            Command::ShowBooks => Output::BooksShown(self.books.checkpoint()),
        };
        let receipt = Receipt { number, output };
        self.observer.observe(&command, &receipt);
        receipt
    }
}
