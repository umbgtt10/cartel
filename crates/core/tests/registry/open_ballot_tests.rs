// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::support::fixtures::Fixtures;
use cartel::ballot_record::BallotRecord;
use cartel::output::Output;
use cartel::registry::open_ballot::OpenBallot;
use cartel::rejection::Rejection;
use cartel::votes::Votes;

fn ballot() -> OpenBallot {
    OpenBallot::new(Fixtures::set(&[1, 2, 3, 4]), Fixtures::set(&[1, 3]))
}

#[test]
fn count_of_a_vote_that_reaches_a_threshold_answers_the_quorum() {
    // Arrange
    let mut ballot = ballot();
    let _ = ballot.count(4, Fixtures::choice(&[0xaa]));
    let _ = ballot.count(1, Fixtures::choice(&[0xaa]));

    // Act
    let output = ballot.count(2, Fixtures::choice(&[0xaa]));

    // Assert
    assert_eq!(
        output,
        Output::QuorumReached {
            count: 3,
            thresholds: vec![3],
            voters: vec![1, 2, 4],
        }
    );
}

#[test]
fn count_of_a_vote_that_reaches_no_threshold_answers_the_count() {
    // Arrange
    let mut ballot = ballot();
    let _ = ballot.count(1, Fixtures::choice(&[0xaa]));

    // Act
    let output = ballot.count(2, Fixtures::choice(&[0xaa]));

    // Assert
    assert_eq!(output, Output::VoteCounted { count: 2 });
}

#[test]
fn counted_of_a_member_is_the_choice_it_is_a_voter_of() {
    // Arrange
    let mut ballot = ballot();
    let _ = ballot.count(2, Fixtures::choice(&[0xbb]));

    // Act
    let counted = [ballot.counted(2), ballot.counted(3)];

    // Assert
    assert_eq!(counted, [Some(&Fixtures::choice(&[0xbb])), None]);
}

#[test]
fn is_member_of_the_electorate_s_members_alone_holds() {
    // Arrange
    let ballot = ballot();

    // Act & Assert
    assert_eq!([ballot.is_member(4), ballot.is_member(5)], [true, false]);
}

#[test]
fn record_of_a_ballot_is_its_electorate_thresholds_and_tally_in_order() {
    // Arrange
    let mut ballot = ballot();
    let _ = ballot.count(3, Fixtures::choice(&[0xff]));
    let _ = ballot.count(2, Fixtures::choice(&[0x01, 0x00]));
    let _ = ballot.count(1, Fixtures::choice(&[0xff]));

    // Act
    let record = ballot.record(Fixtures::ballot(1, 2, 3));

    // Assert
    assert_eq!(
        record,
        BallotRecord {
            ballot: Fixtures::ballot(1, 2, 3),
            electorate: vec![1, 2, 3, 4],
            thresholds: vec![1, 3],
            tally: vec![
                Votes {
                    choice: Fixtures::choice(&[0x01, 0x00]),
                    voters: vec![2],
                },
                Votes {
                    choice: Fixtures::choice(&[0xff]),
                    voters: vec![1, 3],
                },
            ],
        }
    );
}

#[test]
fn refusal_of_a_first_vote_by_a_member_is_none() {
    // Arrange
    let mut ballot = ballot();
    let _ = ballot.count(1, Fixtures::choice(&[0xaa]));

    // Act
    let refused = ballot.refusal(2, &Fixtures::choice(&[0xbb]));

    // Assert
    assert_eq!(refused, None);
}

#[test]
fn refusal_of_a_vote_by_a_member_for_another_choice_is_an_equivocation() {
    // Arrange
    let mut ballot = ballot();
    let _ = ballot.count(1, Fixtures::choice(&[0xaa]));

    // Act
    let refused = ballot.refusal(1, &Fixtures::choice(&[0xbb]));

    // Assert
    assert_eq!(
        refused,
        Some(Rejection::Equivocation {
            counted: Fixtures::choice(&[0xaa]),
        })
    );
}

#[test]
fn refusal_of_a_vote_by_a_member_for_its_own_choice_is_already_counted() {
    // Arrange
    let mut ballot = ballot();
    let _ = ballot.count(1, Fixtures::choice(&[0xaa]));

    // Act
    let refused = ballot.refusal(1, &Fixtures::choice(&[0xaa]));

    // Assert
    assert_eq!(refused, Some(Rejection::AlreadyCounted));
}

#[test]
fn refusal_of_a_vote_by_a_non_member_is_not_a_member() {
    // Arrange
    let ballot = ballot();

    // Act
    let refused = ballot.refusal(5, &Fixtures::choice(&[0xaa]));

    // Assert
    assert_eq!(refused, Some(Rejection::NotAMember));
}
