// Copyright (c) 2026 Umberto Gotti
// SPDX-License-Identifier: MIT

use crate::cluster_setup::ClusterSetup;
use crate::delivery::Delivery;
use crate::member_conduct::MemberConduct;
use crate::seeded_random::SeededRandom;
use crate::sighting::Sighting;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use cartel::ballot::Ballot;
use cartel::cartel::Cartel;
use cartel::choice::Choice;
use cartel::command::Command;
use cartel::fresh::Fresh;
use cartel::instance_id::InstanceId;
use cartel::no_op_observer::NoOpObserver;
use cartel::origin::Origin;
use cartel::output::Output;

const BALLOT: Ballot = Ballot {
    question: 1,
    round: 0,
    phase: 0,
};

// One ballot counted on every node of a cluster, the way a protocol would
// drive its books: every node opens it over all the members with the setup's
// threshold, every member's votes are sent to every node over the faulty
// transport, and a node that crashes restarts fresh under its next
// incarnation and opens the ballot again, the votes it had counted lost.
pub struct ClusterSimulation {
    setup: ClusterSetup,
    random: SeededRandom,
}

impl ClusterSimulation {
    pub fn new(setup: ClusterSetup) -> Self {
        Self {
            setup,
            random: SeededRandom::new(setup.seed),
        }
    }

    // The choices the members cast: what an honest member votes, and the
    // second choice of an equivocating one.
    pub fn cast() -> [Choice; 2] {
        [Choice { bytes: vec![0xaa] }, Choice { bytes: vec![0xbb] }]
    }

    pub fn conduct(&self, member: u64) -> MemberConduct {
        if member <= self.setup.equivocating {
            MemberConduct::Equivocating
        } else if member <= self.setup.equivocating + self.setup.silent {
            MemberConduct::Silent
        } else {
            MemberConduct::Honest
        }
    }

    // Every vote in flight, after the transport's losses, duplicates and
    // reordering.
    pub fn deliveries(&mut self) -> Vec<Delivery> {
        let [honest, other] = Self::cast();
        let mut sent = Vec::new();
        for member in 1..=self.setup.members {
            for node in 1..=self.setup.members {
                let choice = match self.conduct(member) {
                    MemberConduct::Silent => continue,
                    MemberConduct::Honest => honest.clone(),
                    MemberConduct::Equivocating if self.random.chance(50) => other.clone(),
                    MemberConduct::Equivocating => honest.clone(),
                };
                let delivery = Delivery {
                    node,
                    member,
                    choice,
                };
                if self.random.chance(self.setup.faults.loss) {
                    continue;
                }
                if self.random.chance(self.setup.faults.duplication) {
                    sent.push(delivery.clone());
                }
                sent.push(delivery);
            }
        }
        for at in (1..sent.len()).rev() {
            let other_at = self.random.below(at as u64 + 1) as usize;
            sent.swap(at, other_at);
        }
        sent
    }

    // Every quorum any node's books reported, in the order they were reported.
    pub fn run(&mut self) -> Vec<Sighting> {
        let mut nodes: Vec<(u64, Cartel)> = (1..=self.setup.members)
            .map(|node| (1, self.opened(node, 1)))
            .collect();
        let mut sightings = Vec::new();
        for delivery in self.deliveries() {
            let index = (delivery.node - 1) as usize;
            if self.random.chance(self.setup.faults.crash) {
                let incarnation = nodes[index].0 + 1;
                nodes[index] = (incarnation, self.opened(delivery.node, incarnation));
            }
            let (incarnation, books) = &mut nodes[index];
            let receipt = books.process(Command::CastVote {
                ballot: BALLOT,
                member: delivery.member,
                choice: delivery.choice.clone(),
            });
            if let Output::QuorumReached {
                thresholds, voters, ..
            } = receipt.output
            {
                sightings.extend(thresholds.into_iter().map(|threshold| Sighting {
                    node: delivery.node,
                    incarnation: *incarnation,
                    choice: delivery.choice.clone(),
                    threshold,
                    voters: voters.clone(),
                }));
            }
        }
        sightings
    }

    fn opened(&self, node: u64, incarnation: u64) -> Cartel {
        let mut books = Cartel::new(
            Origin::Fresh(Fresh {
                instance: InstanceId { node, incarnation },
            }),
            Box::new(NoOpObserver),
        );
        let _ = books.process(Command::OpenBallot {
            ballot: BALLOT,
            electorate: (1..=self.setup.members).collect(),
            thresholds: vec![self.setup.threshold],
        });
        books
    }
}
