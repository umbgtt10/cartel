// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use cartel::ballot::Ballot;
use cartel::choice::Choice;
use cartel::close_point::ClosePoint;
use cartel::command::Command;
use proptest::collection::vec;
use proptest::prelude::Just;
use proptest::prelude::Strategy;
use proptest::prop_oneof;

// Commands over a small world, so random sequences meet every row: three
// ballots, members 0 to 4, four choices -- the empty one, two of one byte,
// one of two -- electorates and thresholds of every shape, well formed or not,
// and closes before points around the ballots, ahead of the close point or
// not.
pub struct RandomCommands;

impl RandomCommands {
    pub fn ballot() -> impl Strategy<Value = Ballot> {
        prop_oneof![
            Just(Ballot {
                question: 1,
                round: 0,
                phase: 0
            }),
            Just(Ballot {
                question: 1,
                round: 0,
                phase: 1
            }),
            Just(Ballot {
                question: 2,
                round: 0,
                phase: 0
            }),
        ]
    }

    pub fn choice() -> impl Strategy<Value = Choice> {
        prop_oneof![
            Just(Vec::new()),
            Just(vec![0xaa]),
            Just(vec![0xbb]),
            Just(vec![0x01, 0x00]),
        ]
        .prop_map(|bytes| Choice { bytes })
    }

    pub fn command() -> impl Strategy<Value = Command> {
        prop_oneof![
            2 => (Self::ballot(), vec(0..5_u64, 0..5), vec(0..6_u64, 0..3)).prop_map(
                |(ballot, electorate, thresholds)| Command::OpenBallot { ballot, electorate, thresholds }
            ),
            8 => (Self::ballot(), 0..5_u64, Self::choice())
                .prop_map(|(ballot, member, choice)| Command::CastVote { ballot, member, choice }),
            1 => (0..4_u64, 0..3_u64).prop_map(|(question, round)| Command::CloseBallots {
                before: ClosePoint { question, round },
            }),
            1 => Self::ballot().prop_map(|ballot| Command::ShowBallot { ballot }),
            1 => Just(Command::ListBallots),
            1 => Just(Command::ShowBooks),
        ]
    }

    pub fn sequence() -> impl Strategy<Value = Vec<Command>> {
        vec(Self::command(), 1..60)
    }
}
