// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel_spec_model::vocabulary::command_name::CommandName;
use cartel_spec_model::vocabulary::speller::Speller;

#[test]
fn spelling_of_every_command_is_its_name() {
    // Arrange & Act
    let names: Vec<&str> = CommandName::SPELLING
        .words
        .iter()
        .map(|(_, name)| *name)
        .collect();

    // Assert
    assert_eq!(
        names,
        vec![
            "OpenBallot",
            "CastVote",
            "CloseBallots",
            "MeasureWatermark",
            "ShowBallot",
            "ListBallots",
            "ShowBooks"
        ]
    );
    assert_eq!(
        Speller::of(&CommandName::SPELLING, "CastVote"),
        CommandName::CastVote
    );
}
