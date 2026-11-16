# Cartel — the inventory

What Raft and IBFT keep today that is consensus bookkeeping. Each book is marked **Cartel's** or **the protocol's**,
which decides what leaves each protocol when it adopts Cartel.

Read on 2026-11-16, read-only:
- `etheram-raft` origin/main `86ca6ae`;
- `etheram-ibft` origin/main `42faf4e6`, after the mutants plan's chunk 2 (re-read the same day; the first reading was
  `fe904a8a`).

Paths are relative to each repository's `protocol/`.

## What decides the line

Cartel keeps the books on votes:
- who may vote in a ballot;
- who voted for which choice;
- when a threshold is reached;
- the highest position a threshold of members has reached;
- whether a set of voters someone presents reaches a threshold;
- who voted two different choices in one ballot.

Cartel never:
- checks a signature, or keeps one;
- computes a threshold (the protocol supplies it, with each ballot);
- keeps a member's position (it is told the positions each time it is asked);
- casts a vote of its own;
- applies a protocol's rule.

## Raft

| # | book | where | today | goes to |
|---|---|---|---|---|
| R1 | Election ballots. Pre-vote and vote, keyed by the term, won by a majority of the electorate. | `node/src/raft/bookkeepers.rs:126` `open_with_own_vote`, `:166` `won_with`; `election.rs` | port `ConsensusBookkeeping`; default `MajorityConsensusBookkeeping` and `OpenBallot` in node-infra | **Cartel** |
| R2 | The electorate: the configuration's voters. The node's own vote counts only if it is a voter (§4.2.2). | `bookkeepers.rs:126`, `runtime/context/seat.rs` | protocol | **protocol**; the electorate is handed in with each ballot |
| R3 | One ballot of each kind at a time; opening one supersedes the last. | `node-infra/.../majority_consensus_bookkeeping.rs` | default | **Cartel**, as an explicit close the protocol asks for, not a Raft-shaped rule |
| R4 | Commit: the highest index a majority of voters hold (the leader's last index and the followers' match indices). | `bookkeepers.rs:111` `majority_holds`; `raft_protocol.rs:508` | port (`ProgressReported`) | **Cartel** (watermark) |
| R5 | Raft's current-term commit rule on top of R4 (§5.4.2), capped at the node's own last index. | `client_correlation.rs:240` `commit_index_for` | protocol | **protocol** |
| R6 | ReadIndex: a read waits for a heartbeat round a majority has acknowledged (§6.4). | `rounds/heartbeat_rounds.rs:62` `answerable`, `:94` `held` | port, through `majority_holds` | **Cartel** (watermark) |
| R7 | Check-quorum: the leader steps down if no majority acknowledged the round opened one election timeout ago (§6.2). | `heartbeat_rounds.rs:111` `lapsed` | port, through `majority_holds` | **Cartel** (watermark) |
| R8 | The positions: match indices in the context, acknowledged rounds in `HeartbeatRounds`. | context; `heartbeat_rounds.rs` | protocol | **protocol**; Cartel is told them each time |
| R9 | The voter's side: one vote per term, kept durable; the up-to-date check; withholding within the minimum timeout (§4.2.3). | `election.rs`; hard state | protocol | **protocol** |
| R10 | Denied votes: ignored, with no "can no longer win" answer. | `election.rs:223` | protocol | **protocol**; a "lost" answer is a later option |

## IBFT

| # | book | where | today | goes to |
|---|---|---|---|---|
| I1 | Prepare and commit tallies per (height, round, block hash): distinct voters, decided at the quorum. | `node/src/runtime/state/volatile_state.rs:28-29`; `traits/vote_tally.rs`; `node-infra/src/tallies/quorum_tally.rs` | port `VoteTally` | **Cartel** |
| I2 | The quorum, 2n/3+1, and f = (n−1)/3. | `types/src/validator_set.rs:19`, `:42` | protocol | **protocol**; it supplies the thresholds |
| I3 | Resize on a validator-set change: every vote and view change is cleared. | `volatile_state.rs:270` `replace_vote_trackers` | port (`Resized`) | **gone**: thresholds travel with each ballot |
| I4 | Votes cleared at each new round and after each commit. | `volatile_state.rs:276` `reset_after_commit`, `:285` `reset_for_new_round` | port (`Cleared`) | **Cartel** (close) |
| I5 | View changes per (height, round): distinct senders, each kept with its proof (a signature and a prepared certificate). | `volatile_state.rs:32`, `:245` | protocol | **split**: the count and the voters are **Cartel's**; the proofs are **the protocol's** |
| I6 | Two thresholds on the same view changes: f+1 to join a higher round, the quorum to send NewView. | `ibft_protocol/consensus.rs:692`, `:700` | protocol | **Cartel**: one ballot with two thresholds |
| I7 | View changes below the current round are discarded; ahead of it, each sender keeps one, its latest. This bounds memory against flooding. | `volatile_state.rs:93`, `:252`, `:372`, `:392` | protocol | **Cartel**: retention and bounds |
| I8 | Prepare and commit signatures per (height, round, voter). | `volatile_state.rs:36-37` | protocol | **protocol** (evidence) |
| I9 | Assembling the prepared and commit certificates: the quorum's voters plus their signatures, at least a quorum of them. | `consensus.rs:1516` `try_advance_to_commit`, `:1308` `commit_certificate` | protocol | **protocol**, from the voters Cartel names |
| I10 | Verifying a commit certificate: distinct signers, each a member of the set in force at that height, at least a quorum of valid signatures. Outsiders' signatures are not counted, but a repeated signer rejects the whole certificate. | `ibft_protocol/commit_certificate_verifier.rs` | protocol | **split**: the signatures are **the protocol's**; distinct, members and threshold are **Cartel's** (a claim) |
| I11 | Checking a NewView's senders: distinct, all validators, at least a quorum. An unknown sender rejects the message. | `ibft_protocol/new_view_validator.rs` `senders_meet_quorum`, `has_unique_validator_senders` | protocol | **Cartel** (a claim); what an outsider costs stays **the protocol's** choice |
| I12 | NewView justification: its certificate covers the highest certificate among the proofs. | `new_view_validator.rs` `certificate_covers_the_proofs` | protocol | **protocol** (IBFT's locking rule) |
| I13 | Merging prepared certificates: the higher round wins. | `consensus.rs:790` | protocol | **protocol** |
| I14 | Pre-prepare equivocation: one proposer sends two blocks at the same (height, round). Both hashes are rejected and the sender is marked malicious for the height. | `volatile_state.rs:196`; `dispatching/dispatch.rs:130`; `message_security.rs:69` | protocol | **open**: Cartel detects it if a proposal is a ballot of one; the consequences stay **the protocol's** |
| I15 | Prepare and commit equivocation: one voter, two hashes in one round. Today it counts for both and nobody notices. Safety still holds, because two quorums share at least f+1 voters. | `volatile_state.rs` (tallies keyed by hash) | not kept | **Cartel**: new evidence it reports |
| I16 | The node's own vote flags: `prepare_sent`, `commit_sent`, `new_view_sent_round`. | `volatile_state.rs:33-35` | protocol | **protocol** (the voter's side) |
| I17 | The votes as a whole, read through the tally's export into the volatile snapshot. Since D20 the snapshot is no longer written to the WAL, and since D21 a restart starts from the hard state alone: nothing restores votes. | `volatile_state.rs:303` `snapshot`; `ibft_protocol/protocol.rs:588` `volatile_snapshot`, `:674` `restarted_volatile_state`; `VoteTally` `Exported` | port | **Cartel** (the checkpoint, a query); a restore IBFT no longer needs |
| I18 | The validator set in force at a height, including scheduled changes. | `commit_certificate_verifier.rs` `validator_set_at`; `consensus.rs:1284` | protocol | **protocol**; the electorate is handed in |
| I19 | The bootstrapper's 2n/3+1. | `ibft_protocol/bootstrapping/bootstrapper.rs:446` | protocol | **not consensus**: it is Faction's readiness count, the same formula in a different book |
| I20 | Observer events carrying counts and voters (`PrepareVoteAccepted`, `CommitQuorumReached` with a `VoterMask`, …). | `observer/consensus/*` | protocol | **protocol**, fed from Cartel's outputs |
| I21 | Replay protection: one sequence per peer and message kind. | `message_security.rs` | protocol | **protocol** (message security, not votes) |

## Findings

1. **Raft reports positions as an anonymous list** (`held: Vec<u64>`). Weights could never apply to it later, so
   positions must be reported per member from the start. This is a fourth guard to keep the door open.
2. **IBFT puts two thresholds on one ballot** (I6). A ballot carries one or more thresholds, and Cartel reports
   each as it is reached.
3. **IBFT's two certificate checks treat an outsider differently** (I10 against I11). Cartel reports the facts:
   who counted, who is an outsider, who is repeated, whether the threshold is reached. The protocol decides what
   an outsider costs.
4. **IBFT doesn't detect equivocation in prepares and commits** (I15). Cartel would report it. What to do with it
   is IBFT's decision.
5. **The bootstrapper's 2n/3+1 is not a second copy of the quorum** (I19). It is Faction's readiness count: the same
   formula in a different book.
6. **Retention is explicit.** Raft's superseding (R3) and IBFT's clearing (I4, I7) both become an explicit close
   the protocol asks for. Cartel doesn't infer it from either protocol's shape.
7. **IBFT no longer restores its votes** (I17). Chunk 2 of its mutants plan stopped writing the volatile snapshot
   (D20) and starts a node from genesis or its hard state alone (D21), as Raft does. Restore stays in Cartel's plan
   for a replay's resynchronisation and for a protocol that would keep its books across a restart; IBFT is no
   longer the one that needs it.
