// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel_spec_model::vocabulary::command_name::CommandName;
use cartel_spec_model::vocabulary::speller::Speller;

#[test]
fn name_of_a_word_is_how_it_is_spelled() {
    // Arrange & Act
    let name = Speller::name(&CommandName::SPELLING, CommandName::ShowBooks);

    // Assert
    assert_eq!(name, "ShowBooks");
}

#[test]
fn of_a_name_is_the_word_it_spells() {
    // Arrange & Act
    let word = Speller::of(&CommandName::SPELLING, "OpenBallot");

    // Assert
    assert_eq!(word, CommandName::OpenBallot);
}
