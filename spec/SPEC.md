# cartel — the specification

**Version 0.2.0.** The four parts of this specification are versioned together: this prose, the data model in
[`schema.json`](schema.json), the decision table in [`matrix.json`](matrix.json) and the conformance vectors in
[`vectors/`](vectors/). It is normative; the `cartel` crate is one implementation of it, and where the two disagree
the specification is right ([`P0-ADR-SpecificationIsNormativeCrateIsReference`](../docs/ADRs/P0-ADR-SpecificationIsNormativeCrateIsReference.md)).

**What 0.2.0 covers:** ballots and closing. An instance, created fresh, and the observer it reports every call to;
`OpenBallot`, `CastVote` and `CloseBallots`; the queries `ShowBallot`, `ListBallots` and `ShowBooks`, the
checkpoint; each with every row it has. 0.2.0 is 0.1.0 with closing (§10). What later versions add is §11.

## 1. Conventions

**Every u64 is a string of decimal digits**, with no sign, no leading zero and no exponent, its value within
`0..=18446744073709551615`. Many JSON parsers read numbers as 64-bit floats, exact only to 2⁵³.

**Every value of an enumeration is an object with exactly one member**, named for the value: `{"ListBallots": {}}`,
`{"CommandRejected": {"RepeatedMember": {"member": "3"}}}`. A value that carries nothing carries the empty object.

**Names are the vocabulary's** ([`DESIGN.md`](../docs/DESIGN.md) §4). Commands are imperative and outputs past
participle, so what a caller asked for and what happened cannot be confused.

**A choice is a sequence of bytes, possibly empty**, written as a string of lowercase hexadecimal digits, two to a
byte: the empty choice is `""`, the bytes `0x0a 0xff` are `"0aff"`. Choices are compared byte by byte, and ordered
lexicographically: the first byte that differs decides, and a choice that is the start of another comes before
it. Their written forms order the same way.

**Canonical order.** Wherever an answer lists them:
- ballots come in ascending order of question, then round, then phase;
- members, voters and thresholds come in ascending order;
- a tally's choices come in the choices' order.

An answer has exactly one form, so two implementations' logs of the same calls are identical.

**Growth.** A member a later version adds to a form is omitted wherever it holds the value that keeps the earlier
version's behaviour. Every answer this version gives is therefore given the same, character for character, by
every later version, and every vector released here passes unchanged there
([`P1-ADR-TheQuorumShapeIsLeftOpen`](../docs/ADRs/P1-ADR-TheQuorumShapeIsLeftOpen.md)).

## 2. An instance

**An instance is created fresh**, with an instance id and an observer (§3). The instance id is a node and an
incarnation; it tells one instance's receipts from another's. The caller guarantees that an incarnation never
repeats for its node: a node that restarts is a new instance
([`P1-ADR-TheLogIsTheRecord`](../docs/ADRs/P1-ADR-TheLogIsTheRecord.md)).

**A fresh instance holds no ballot**, and its first receipt is number 1.

## 3. One entry point, one output

**Every call is a command, and every command is answered with a receipt**: an output and a number. The number is
one more than the instance's previous receipt's, the first being 1, whatever the output. A refused command and a
query are numbered like the rest ([`P1-ADR-OneEntryPointOneOutput`](../docs/ADRs/P1-ADR-OneEntryPointOneOutput.md)).

**The answer is total and deterministic.** Every command has exactly one output in every state, given by the first
row of [`matrix.json`](matrix.json) that applies, in the matrix's order. The same state and the same command give
the same receipt.

**Every call is reported to the instance's observer**, given when the instance is created. It is told the command
and its receipt, exactly once, by the time the receipt is returned, a refused command and a query like the rest.
The observer is told and answers nothing, so it cannot change an output. An instance with no use for one is given
an observer that does nothing ([`P1-ADR-EveryCallIsObserved`](../docs/ADRs/P1-ADR-EveryCallIsObserved.md)).

**A refusal changes nothing** but the receipt number. Every refusal is `CommandRejected`, carrying a rejection that
says why, and what the log needs beside the command. **A query changes nothing** but the receipt number either.

**An output says what happened, never what was asked.** The command is beside it in the log, so an output does not
repeat the ballot, the member or the choice the command carried.

## 4. Ballots and the books

**A ballot is a question, a round and a phase**, three u64. Cartel never interprets them: the question is what is
being decided, the round one attempt at deciding it, the phase a stage of a round, each named by the protocol that
drives the instance ([`P1-ADR-ABallotIsAQuestionARoundAndAPhase`](../docs/ADRs/P1-ADR-ABallotIsAQuestionARoundAndAPhase.md)).

**The books are the open ballots and the close point.** Each open ballot has its record:
- **the ballot;**
- **its electorate:** the members who may vote in it, distinct, in ascending order;
- **its thresholds:** distinct numbers, each from 1 to the electorate's size, in ascending order;
- **its tally:** for each choice voted in it, its voters, in ascending order. A choice no one voted for is not in
  the tally.

A ballot is open from the `OpenBallot` that opens it until a `CloseBallots` closes it (§10).

**The close point** is none on a fresh instance, and a question and a round once a close has moved it. Points are
ordered by question, then round. A ballot is **below** the close point when its question and round come before the
point's, whatever its phase; a ballot whose question and round are the point's is not below it. A ballot below the
close point is **closed**: it is not open, and cannot be opened again, and the rows that meet it say it is closed,
not that it is unknown ([`P1-ADR-ClosingIsExplicitAndRemembered`](../docs/ADRs/P1-ADR-ClosingIsExplicitAndRemembered.md)).

**A member is a voter of at most one choice in a ballot.** A choice's **count** is how many voters it has. A
threshold is **reached** for a choice by the vote that makes the choice's count equal to it: once, and only by
that vote. A vote raises its choice's count by exactly one and a ballot's thresholds are distinct, so a vote
reaches at most one threshold.

**Ballots do not interfere.** A command that names a ballot changes no other ballot's record, and what it answers
depends on no other ballot's record. A member may vote one choice in one ballot and another in the next. A close
names a point, not a ballot: it closes every ballot below the point and leaves the rest as they were.

**Forgetting is safe.** Given any subset of the votes counted in a ballot, the rules reach no threshold for a choice
that the whole set did not reach for it. A crash can make an instance miss a quorum, and never report one the votes
it had counted did not reach. One case is the protocol's to weigh: a member that voted two choices, a faulty one,
may have its second counted by an instance that forgot its first. Quorum intersection, which the protocol's choice
of thresholds gives, keeps that from deciding two choices ([`DESIGN.md`](../docs/DESIGN.md) §8).

## 5. `OpenBallot`

**Carries:** `ballot`; `electorate`, a list of members; `thresholds`, a list of numbers. The order of either list
does not matter. The electorate and the thresholds are the protocol's, handed in with the ballot and counted against
for as long as it is open ([`P1-ADR-TheElectorateTravelsWithTheBallot`](../docs/ADRs/P1-ADR-TheElectorateTravelsWithTheBallot.md),
[`P1-ADR-TheProtocolSuppliesTheThresholds`](../docs/ADRs/P1-ADR-TheProtocolSuppliesTheThresholds.md)).

**Checked in this order:**

| row | situation | output |
|---|---|---|
| `empty-electorate` | `electorate` is empty | `CommandRejected(EmptyElectorate)` |
| `repeated-member` | `electorate` names a member more than once | `CommandRejected(RepeatedMember { member })`, the lowest such member |
| `no-thresholds` | `thresholds` is empty | `CommandRejected(NoThresholds)` |
| `threshold-out-of-range` | a threshold is 0, or above the number of members in `electorate` | `CommandRejected(ThresholdOutOfRange { threshold })`, the lowest such threshold |
| `repeated-threshold` | a threshold is given more than once | `CommandRejected(RepeatedThreshold { threshold })`, the lowest such threshold |
| `ballot-closed` | `ballot` is below the close point | `CommandRejected(BallotClosed)` |
| `ballot-already-open` | `ballot` is open | `CommandRejected(BallotAlreadyOpen)` |
| `ballot-opened` | otherwise | `BallotOpened` |

**Opened:** the ballot is open, with its electorate and its thresholds as given, each in ascending order, and an
empty tally.

## 6. `CastVote`

**Carries:** `ballot`; `member`, the voter; `choice`, what it votes for. A vote is cast only after the protocol has
verified it: Cartel checks no signature and keeps no evidence
([`P1-ADR-NoEvidenceInTheBooks`](../docs/ADRs/P1-ADR-NoEvidenceInTheBooks.md)).

**Checked in this order:**

| row | situation | output |
|---|---|---|
| `ballot-closed` | `ballot` is below the close point | `CommandRejected(BallotClosed)` |
| `ballot-not-open` | `ballot` is not open | `CommandRejected(BallotNotOpen)` |
| `not-a-member` | `member` is not in the ballot's electorate | `CommandRejected(NotAMember)` |
| `already-counted` | `member` is a voter of `choice` in the ballot | `CommandRejected(AlreadyCounted)` |
| `equivocation` | `member` is a voter of another choice in the ballot | `CommandRejected(Equivocation { counted })`, the choice it is a voter of |
| `quorum-reached` | otherwise, and the vote reaches a threshold of the ballot | `QuorumReached { count, thresholds, voters }` |
| `vote-counted` | otherwise | `VoteCounted { count }` |

**Counted**, in the last two rows: `member` becomes a voter of `choice` in the ballot.
- `count` is the choice's count with the vote.
- `thresholds` lists the thresholds the vote reached: in 0.1.0 exactly one, the count itself. It is a list so that
  forms of the quorum a later version may add, where one vote can reach several, answer in the same form.
- `voters` are the choice's voters with the vote, in ascending order: the quorum.

**An equivocation is reported, not judged.** The vote counted stays counted, the vote offered is not counted, and
nothing else follows: what to do with a member that voted two choices is the protocol's
([`P1-ADR-EquivocationIsReportedNotJudged`](../docs/ADRs/P1-ADR-EquivocationIsReportedNotJudged.md)). The choice
offered is the command's, beside the output in the log.

## 7. `ShowBallot`

**Carries:** `ballot`. **Changes nothing** but the receipt number.

| row | situation | output |
|---|---|---|
| `ballot-closed` | `ballot` is below the close point | `CommandRejected(BallotClosed)` |
| `ballot-not-open` | `ballot` is not open | `CommandRejected(BallotNotOpen)` |
| `ballot-shown` | otherwise | `BallotShown { record }`, the ballot's record |

## 8. `ListBallots`

**Carries nothing. Changes nothing** but the receipt number.

| row | situation | output |
|---|---|---|
| `ballots-listed` | always | `BallotsListed { ballots }`, the open ballots, in canonical order |

## 9. `ShowBooks`, the checkpoint

**Carries nothing. Changes nothing** but the receipt number.

| row | situation | output |
|---|---|---|
| `books-shown` | always | `BooksShown { checkpoint }` |

**The checkpoint** is the instance id, the close point when there is one, every open ballot's record in canonical
order, and the next receipt's number. While there is no close point, the checkpoint has no member for it (§1). It describes the state after its own call: its next receipt number is one more than the number of the
receipt that carries it.

## 10. `CloseBallots`

**Carries:** `before`, a point: a question and a round. When to let go is the protocol's: when a round or a
question is done, it closes what lies before it
([`P1-ADR-ClosingIsExplicitAndRemembered`](../docs/ADRs/P1-ADR-ClosingIsExplicitAndRemembered.md)).

| row | situation | output |
|---|---|---|
| `close-behind` | there is a close point, and `before` is at it or before it | `CommandRejected(CloseBehind { close_point })`, the close point |
| `ballots-closed` | otherwise | `BallotsClosed { ballots }`, the ballots it closed, in canonical order |

**Closed:** the close point becomes `before`, and every open ballot below it is closed and forgotten, its record
gone. Ballots at or above the point are left as they were. A close that closes no ballot still moves the point.

**The point only moves forward.** A close at or behind it is refused and changes nothing; the refusal says where
the point stands, so a protocol that closes backwards -- a bug, or a replay after a restart -- shows in the log.

## 11. Not yet

The order of building is [`IMPLEMENTATION-PLAN.md`](../docs/IMPLEMENTATION-PLAN.md). Each later version only adds:

| version | adds |
|---|---|
| 0.3.0 | the watermark: `MeasureWatermark` |
| 0.4.0 | restoring an instance from a checkpoint, under a new incarnation |
| 0.5.0 | the optional bounds: open ballots, electorate, choice length |
| 0.6.0 | claims: `ExamineClaim` |

**Later, when a protocol asks:**
- withdrawing a vote;
- closing one ballot at a time;
- an answer that a threshold can no longer be reached;
- weights, joint consensus and federated quorums, as new forms of the quorum.

## 12. Conformance

**A vector is an initial state, steps, and a final state.** The initial state is an origin: in 0.1.0, fresh. Each
step is a command and the receipt it must be answered with. The final state is the checkpoint a `ShowBooks` issued
after the last step must answer: its own receipt numbered one more than the last step's, so its next receipt
number is one more again.

**Each vector names the rows of the matrix it exercises.** Every vector is valid against the schema's `vector`, the
matrix against its `matrix`, and [`released.json`](released.json) against its `released`. Every row of the matrix
is exercised by at least one vector.

**Released vectors are kept.** `released.json` lists every vector of every released version by its file's name and
the SHA-256 digest of its bytes. A listed vector is never removed or edited.

**An implementation conforms** when:
1. created from each vector's origin with an observer that does nothing and given each step's command in turn, it
   answers each with the step's receipt, and the closing `ShowBooks` with the final state;
2. created with an observer that keeps what it is told, it has reported each of those calls with the receipt it
   answered, in order, and nothing else.
