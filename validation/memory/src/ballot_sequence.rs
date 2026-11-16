// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::allocation_counter::AllocationCounter;
use crate::memory_profile::MemoryProfile;
use cartel::ballot::Ballot;
use cartel::bounds::Bounds;
use cartel::cartel::Cartel;
use cartel::choice::Choice;
use cartel::close_point::ClosePoint;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::instance_id::InstanceId;
use cartel::no_op_observer::NoOpObserver;
use cartel::origin::Origin;

const HASH: [u8; 32] = [0xaa; 32];
const OTHER: [u8; 32] = [0xbb; 32];

// Ballots one after another on one instance: each opened over the members
// given, a majority their threshold, and every member's vote for a 32-byte
// choice counted -- and, when busy, every vote followed by a repeat, an
// equivocation, a non-member's vote, an opening of the open ballot and a
// list of the ballots, each refused or a query -- and, when closing, every
// ballot closed once its votes are counted. The counter runs from before the
// instance is created.
pub struct BallotSequence<'a> {
    counter: &'a dyn AllocationCounter,
    members: u64,
    busy: bool,
    closing: bool,
}

impl<'a> BallotSequence<'a> {
    pub fn new(
        counter: &'a dyn AllocationCounter,
        members: u64,
        busy: bool,
        closing: bool,
    ) -> Self {
        Self {
            counter,
            members,
            busy,
            closing,
        }
    }

    pub fn run(&self, ballots: u64) -> MemoryProfile {
        self.counter.start();
        let mut cartel = Cartel::new(
            Origin::Fresh(Fresh {
                instance: InstanceId {
                    node: 1,
                    incarnation: 1,
                },
                bounds: Bounds::default(),
            }),
            Box::new(NoOpObserver),
        );
        let skeleton = self.counter.live();
        for question in 1..=ballots {
            let ballot = Ballot {
                question,
                round: 0,
                phase: 0,
            };
            let _ = cartel.process(self.opening(ballot));
            for member in 1..=self.members {
                let _ = cartel.process(Self::vote(ballot, member, &HASH));
                if self.busy {
                    for noise in self.noise(ballot, member) {
                        let _ = cartel.process(noise);
                    }
                }
            }
            if self.closing {
                let _ = cartel.process(Command::CloseBallots {
                    before: ClosePoint {
                        question: question + 1,
                        round: 0,
                    },
                });
            }
        }
        let held_after_all = self.counter.live();
        drop(cartel);
        let after_drop = self.counter.live();
        self.counter.stop();
        MemoryProfile {
            skeleton,
            held_after_all,
            after_drop,
        }
    }

    fn noise(&self, ballot: Ballot, member: u64) -> [Command; 5] {
        [
            Self::vote(ballot, member, &HASH),
            Self::vote(ballot, member, &OTHER),
            Self::vote(ballot, self.members + 1, &HASH),
            self.opening(ballot),
            Command::ListBallots,
        ]
    }

    fn opening(&self, ballot: Ballot) -> Command {
        Command::OpenBallot {
            ballot,
            electorate: (1..=self.members).collect(),
            thresholds: vec![self.members / 2 + 1],
        }
    }

    fn vote(ballot: Ballot, member: u64, choice: &[u8]) -> Command {
        Command::CastVote {
            ballot,
            member,
            choice: Choice {
                bytes: choice.to_vec(),
            },
        }
    }
}
