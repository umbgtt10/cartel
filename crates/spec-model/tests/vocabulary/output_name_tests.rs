// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::commands::Commands;
use crate::support::states::States;
use cartel_spec_model::call::Call;
use cartel_spec_model::instance_state::InstanceState;
use cartel_spec_model::rules::change::Change;
use cartel_spec_model::vocabulary::command_name::CommandName;
use cartel_spec_model::vocabulary::output_name::OutputName;
use cartel_spec_model::vocabulary::rejection_name::RejectionName;
use serde_json::json;

// Members 1 to 3, threshold 2; member 1 a voter of 0xaa.
fn before() -> InstanceState {
    States::holding(vec![States::ballot(
        States::key(1, 0, 0),
        &[1, 2, 3],
        &[2],
        &[(1, &[0xaa])],
    )])
}

#[test]
fn encoded_of_ballot_opened_carries_nothing() {
    // Arrange
    let command = Commands::open(States::key(2, 0, 0), &[1], &[1]);
    let state = before();
    let after = Change::BallotOpened.applied(&command, &state);

    // Act
    let output = OutputName::BallotOpened.encoded(
        None,
        &Call {
            command: &command,
            before: &state,
            after: &after,
        },
    );

    // Assert
    assert_eq!(output, json!({"BallotOpened": {}}));
}

#[test]
fn encoded_of_ballot_shown_is_the_ballot_s_record() {
    // Arrange
    let command = Commands::show(States::key(1, 0, 0));
    let state = before();

    // Act
    let output = OutputName::BallotShown.encoded(
        None,
        &Call {
            command: &command,
            before: &state,
            after: &state,
        },
    );

    // Assert
    assert_eq!(
        output,
        json!({"BallotShown": {"record": {
            "ballot": {"question": "1", "round": "0", "phase": "0"},
            "electorate": ["1", "2", "3"],
            "thresholds": ["2"],
            "tally": [{"choice": "aa", "voters": ["1"]}],
        }}})
    );
}

#[test]
fn encoded_of_ballots_closed_is_the_ballots_below_the_point_in_canonical_order() {
    // Arrange
    let command = Commands::close(2, 0);
    let state = States::holding(vec![
        States::ballot(States::key(1, 0, 1), &[1], &[1], &[]),
        States::ballot(States::key(2, 0, 0), &[1], &[1], &[]),
        States::ballot(States::key(1, 0, 0), &[1], &[1], &[]),
    ]);
    let after = Change::BallotsClosed.applied(&command, &state);

    // Act
    let output = OutputName::BallotsClosed.encoded(
        None,
        &Call {
            command: &command,
            before: &state,
            after: &after,
        },
    );

    // Assert
    assert_eq!(
        output,
        json!({"BallotsClosed": {"ballots": [
            {"question": "1", "round": "0", "phase": "0"},
            {"question": "1", "round": "0", "phase": "1"},
        ]}})
    );
}

#[test]
fn encoded_of_ballots_listed_is_the_open_ballots() {
    // Arrange
    let command = Commands::of(CommandName::ListBallots);
    let state = before();

    // Act
    let output = OutputName::BallotsListed.encoded(
        None,
        &Call {
            command: &command,
            before: &state,
            after: &state,
        },
    );

    // Assert
    assert_eq!(
        output,
        json!({"BallotsListed": {"ballots": [{"question": "1", "round": "0", "phase": "0"}]}})
    );
}

#[test]
fn encoded_of_books_shown_is_the_checkpoint_after_the_call() {
    // Arrange
    let command = Commands::of(CommandName::ShowBooks);
    let state = States::instance();
    let after = InstanceState {
        next_receipt: 6,
        ..States::instance()
    };

    // Act
    let output = OutputName::BooksShown.encoded(
        None,
        &Call {
            command: &command,
            before: &state,
            after: &after,
        },
    );

    // Assert
    assert_eq!(
        output,
        json!({"BooksShown": {"checkpoint": {
            "instance": {"node": "2", "incarnation": "3"},
            "ballots": [],
            "next_receipt": "6",
        }}})
    );
}

#[test]
fn encoded_of_claim_examined_is_the_members_counted_the_outsiders_the_repeats_and_whether_they_reach()
 {
    // Arrange
    let reaching = Commands::examine(&[4, 1, 2, 3], 3, &[9, 2, 1, 2, 9, 3]);
    let short = Commands::examine(&[1, 2, 3, 4], 3, &[2, 2, 1]);
    let state = States::instance();

    // Act
    let outputs = [&reaching, &short].map(|command| {
        OutputName::ClaimExamined.encoded(
            None,
            &Call {
                command,
                before: &state,
                after: &state,
            },
        )
    });

    // Assert
    assert_eq!(
        outputs,
        [
            json!({"ClaimExamined": {"counted": ["1", "2", "3"], "outsiders": ["9"], "repeated": ["2", "9"], "reached": true}}),
            json!({"ClaimExamined": {"counted": ["1", "2"], "outsiders": [], "repeated": ["2"], "reached": false}}),
        ]
    );
}

#[test]
fn encoded_of_command_rejected_is_its_rejection() {
    // Arrange
    let command = Commands::cast(States::key(1, 0, 0), 9, &[0xaa]);
    let state = before();

    // Act
    let output = OutputName::CommandRejected.encoded(
        Some(RejectionName::NotAMember),
        &Call {
            command: &command,
            before: &state,
            after: &state,
        },
    );

    // Assert
    assert_eq!(output, json!({"CommandRejected": {"NotAMember": {}}}));
}

#[test]
fn encoded_of_quorum_reached_is_the_count_the_threshold_reached_and_the_voters() {
    // Arrange
    let command = Commands::cast(States::key(1, 0, 0), 3, &[0xaa]);
    let state = before();
    let after = Change::VoteCast.applied(&command, &state);

    // Act
    let output = OutputName::QuorumReached.encoded(
        None,
        &Call {
            command: &command,
            before: &state,
            after: &after,
        },
    );

    // Assert
    assert_eq!(
        output,
        json!({"QuorumReached": {"count": "2", "thresholds": ["2"], "voters": ["1", "3"]}})
    );
}

#[test]
fn encoded_of_vote_counted_is_the_choice_s_count_with_the_vote() {
    // Arrange
    let command = Commands::cast(States::key(1, 0, 0), 2, &[0xbb]);
    let state = before();
    let after = Change::VoteCast.applied(&command, &state);

    // Act
    let output = OutputName::VoteCounted.encoded(
        None,
        &Call {
            command: &command,
            before: &state,
            after: &after,
        },
    );

    // Assert
    assert_eq!(output, json!({"VoteCounted": {"count": "1"}}));
}

#[test]
fn encoded_of_watermark_measured_is_the_highest_position_the_threshold_s_members_reached() {
    // Arrange
    let command = Commands::measure(&[(1, 10), (2, 8), (3, 8), (4, 3), (5, 1)], 3);
    let state = States::instance();

    // Act
    let output = OutputName::WatermarkMeasured.encoded(
        None,
        &Call {
            command: &command,
            before: &state,
            after: &state,
        },
    );

    // Assert
    assert_eq!(output, json!({"WatermarkMeasured": {"position": "8"}}));
}
