// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::command::Command;
use cartel::no_op_observer::NoOpObserver;
use cartel::observer::Observer;
use cartel::output::Output;
use cartel::receipt::Receipt;

#[test]
fn observe_of_any_call_does_nothing() {
    // Arrange
    let mut observer = NoOpObserver;
    let receipt = Receipt {
        number: 1,
        output: Output::BallotsListed {
            ballots: Vec::new(),
        },
    };

    // Act & Assert
    observer.observe(&Command::ListBallots, &receipt);
}
