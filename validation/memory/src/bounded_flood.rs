// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::allocation_counter::AllocationCounter;
use crate::flood_profile::FloodProfile;
use cartel::ballot::Ballot;
use cartel::bounds::Bounds;
use cartel::cartel::Cartel;
use cartel::choice::Choice;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::instance_id::InstanceId;
use cartel::no_op_observer::NoOpObserver;
use cartel::origin::Origin;

// An instance given every bound, filled to them and then flooded: ballots
// opened until the books are full, each over the largest electorate, every
// member voting a choice of its own of the longest length -- the most the
// bounds let the books hold -- then, round after round, every command that
// would exceed a bound: an opening of one ballot more, an electorate of one
// member more, and in every ballot a vote one byte too long and an
// equivocation. Each command is built only when it is given, so what the
// counter shows between two calls is what the books hold. The counter runs
// from before the instance is created.
pub struct BoundedFlood<'a> {
    counter: &'a dyn AllocationCounter,
    open_ballots: u64,
    electorate: u64,
    choice: u64,
}

impl<'a> BoundedFlood<'a> {
    pub fn new(
        counter: &'a dyn AllocationCounter,
        open_ballots: u64,
        electorate: u64,
        choice: u64,
    ) -> Self {
        Self {
            counter,
            open_ballots,
            electorate,
            choice,
        }
    }

    pub fn run(&self, rounds: u64) -> FloodProfile {
        self.counter.start();
        let mut cartel = Cartel::new(
            Origin::Fresh(Fresh {
                instance: InstanceId {
                    node: 1,
                    incarnation: 1,
                },
                bounds: Bounds {
                    open_ballots: Some(self.open_ballots),
                    electorate: Some(self.electorate),
                    choice: Some(self.choice),
                },
            }),
            Box::new(NoOpObserver),
        );
        for question in 1..=self.open_ballots {
            let _ = cartel.process(self.opening(question, self.electorate));
            for member in 1..=self.electorate {
                let _ = cartel.process(self.vote(question, member, member as u8, self.choice));
            }
        }
        let full = self.counter.live();
        let mut most_while_flooded = full;
        for _ in 0..rounds {
            for at in 0..2 + 2 * self.open_ballots {
                let _ = cartel.process(self.excess(at));
                most_while_flooded = most_while_flooded.max(self.counter.live());
            }
        }
        let after_flood = self.counter.live();
        drop(cartel);
        self.counter.stop();
        FloodProfile {
            full,
            most_while_flooded,
            after_flood,
        }
    }

    // The flood's command at a place in its round: one ballot more, one
    // member more, then for each ballot a vote one byte too long and a member
    // voting a second choice.
    fn excess(&self, at: u64) -> Command {
        let question = at / 2;
        match (at, at % 2) {
            (0, _) => self.opening(self.open_ballots + 1, self.electorate),
            (1, _) => self.opening(self.open_ballots + 1, self.electorate + 1),
            (_, 0) => self.vote(question, 1, 0xfe, self.choice + 1),
            _ => self.vote(question, 1, 0xff, self.choice),
        }
    }

    fn opening(&self, question: u64, members: u64) -> Command {
        Command::OpenBallot {
            ballot: Ballot {
                question,
                round: 0,
                phase: 0,
            },
            electorate: (1..=members).collect(),
            thresholds: vec![self.electorate],
        }
    }

    fn vote(&self, question: u64, member: u64, byte: u8, length: u64) -> Command {
        Command::CastVote {
            ballot: Ballot {
                question,
                round: 0,
                phase: 0,
            },
            member,
            choice: Choice {
                bytes: vec![byte; length as usize],
            },
        }
    }
}
