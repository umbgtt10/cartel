# cartel

## Meaning

`cartel` is a `no_std`, protocol-independent consensus bookkeeper: one of the four
bookkeepers, mutually unaware, that IBFT and Raft orchestrate.

Cartel keeps the books on votes -- ballots, members, opaque choices and
thresholds -- and decides nothing: no signature, no threshold formula, no vote
of its own, no clock, no storage, no protocol rule.

`docs/ADRs/` holds the foundations, `docs/DESIGN.md` the how and the
vocabulary, `docs/IMPLEMENTATION-PLAN.md` the order of building and the rules
that keep every slice additive. All are authoritative. Read them before
changing anything.

## Boundary Rule

This repository is **SELF-CONTAINED**.

The LLM **SHALL NOT cross its boundaries without asking**.

That means:
- do not inspect, edit, or rely on files outside `cartel/` unless the user explicitly asks
- do not pull assumptions from sibling repositories or crates
- do not propose cross-repository changes by default

## Quality Gates

### Mandatory after every change

Run:

`just stage1`
`just stage2`

If either gate is not green, the work is not complete.

Stage 1 is formatting, clippy over every target, the `no_std` check and tests.
The `no_std` check builds the crate and the specification's model for
`thumbv7em-none-eabihf`, a target with no std at all, which
`rust-toolchain.toml` installs.

Stage 2 is `cargo xtask stage2`, as beggar's: stern4rust, then dry4rust
against `dry4rust-baseline.json`, crap4rust, the coverage gate twice -- the
second on the build the first left --, twin4rust, iceberg4rust and cargo-deny, in that order. The gates come from
`serge4rust`, the house's gates as a library: the gate trait, the sequence, the
command runner, a gate per tool and the commit scoring are its, with their
tests. `xtask/` keeps only `src/main.rs` -- which gates stage 2 runs, in what
order, and cartel's constants, passed to serge4rust's constructors: packages,
thresholds, the baseline and its floor, the words the coverage failure names,
the library the scoring reads. A gate that needs changing is changed in
serge4rust, released, and taken here from crates.io (`serge4rust = "0.3.3"`),
never patched in place. stern runs over `cartel`, `cartel-core-validation`,
`cartel-ladder`, `cartel-memory`, `cartel-spec-model` **and** `xtask`; the rest
over the library, `cartel` and `cartel-spec-model`. dry4rust scans `crates/`,
what `dry4rust.toml` leaves in.

`just ripe` runs `cargo ripe4rust plan`: every upgrade the workspace could
take, judged ripe, worth asking about or not yet, against the gates
`[workspace.metadata.ripe4rust]` declares in `Cargo.toml` -- `just stage1`,
then `just stage2`, in the order they validate a change. It prints the plan and
applies nothing; applying an upgrade is a change of its own, on the owner's
word.

`cargo install cargo-ripe4rust`

The tests climb a ladder of seven tiers — `docs/ADRs/P2-ADR-TestingLadder.md`.
The coverage gate, `cargo llvm-cov` over `cartel`, `cartel-spec-model` and
`cartel-ladder`, fails on any line, region or function of the library left
uncovered: once over every tier, and once over the conformance tier alone, its
tests filtered to `conformance::` — the vectors must reach every line by
themselves (`docs/ADRs/P1-ADR-NoLineTheSpecificationDoesNotAskFor.md`). The
specification in `spec/` is normative; the crate is one implementation of it,
and `crates/spec-model/` a second, independent of the crate, against which the
vectors are checked (`docs/ADRs/P1-ADR-IndependentModelOfTheSpecification.md`).
`deny.toml` refuses it `cartel`: the two meet only in the specification's
encoding, in the ladder.

Released vectors are never removed or edited: `spec/released.json` lists them
by id and digest, and the conformance tier fails on one missing or changed
(`docs/ADRs/P1-ADR-TheQuorumShapeIsLeftOpen.md`).

### Every commit carries the library's scores

Every commit message carries the grip and braintax of the library's crates,
`core` and `spec-model` under `crates/`, and their ratio, scored as the commit
stages them, as beggar's and faction's do. `.githooks/commit-msg` runs
`cargo xtask grip-braintax` on the message's file -- serge4rust's scoring,
told the library is `core` and `spec-model` under `crates/`. A scoring that
fails stops the commit, and `--no-verify` skips it, deliberately -- never use
it. Git runs a repository's own hooks only where a clone asks it to:
`just hooks`, once a clone, on each machine.

`cargo install cargo-grip4rust`
`cargo install cargo-braintax4rust`

## Layout

A workspace laid out by role (`docs/ADRs/P2-ADR-WorkspaceLaidOutByRole.md`):
`spec/` holds the specification; `crates/` the library -- `crates/core/`, the
crate, and `crates/spec-model/`, the specification's independent model;
`validation/` every package that checks it -- `validation/ladder/` the testing
ladder's scenario tiers, `validation/core-validation/` its cluster simulation,
`validation/memory/` its memory tier; and `xtask/` runs the gates,
serge4rust's.

`crates/core/tests/` and `crates/spec-model/tests/` are the unit tier, one test
file per source file, under every rule. The tiers that are scenarios rather
than mirrors of a file live in `validation/ladder/` and
`validation/core-validation/`, where stern's `paired-test-file` stands down.
The memory tier is a package of its own because a `#[global_allocator]` is per
binary, and holds the workspace's one `unsafe`, the allocator's.

## Strict Rules

### Test-driven

Tests are written first and seen failing before any implementation.

### Test preservation

**NEVER remove a test without explicitly asking the user first.**

### Assertion integrity

**NEVER relax a test assertion without explicitly asking the user first.**

### No re-export shims

Every `use` points directly to the module where the symbol is defined.

### The specification comes first

The specification in `spec/` is normative and the crate is one implementation of it
(`docs/ADRs/P0-ADR-SpecificationIsNormativeCrateIsReference.md`). A change of behaviour
changes the specification first -- the prose, the schema, the decision table's rows, a
vector for every row it adds or changes, and its version -- then the crate. Vectors are
written by hand and checked against the specification's model before the crate runs them;
never export them from the crate.

The model is `crates/spec-model/` (`cartel-spec-model`,
`docs/ADRs/P1-ADR-IndependentModelOfTheSpecification.md`): `no_std`, free of unsafe code,
and held to every gate the library is held to. It is written from `spec/SPEC.md` and
`spec/matrix.json`, never from `crates/core/`, and depends on nothing of `cartel` --
`deny.toml` refuses it -- so the two meet only in the specification's encoding, in the
ladder. What it owns is what the table cannot say, in `rules/rule_book.rs`: each row's
situation and change; a name the vocabulary gains is in `vocabulary/`.

### Additive releases

A release only adds: commands, outputs, rejections, origins, construction parameters,
checkpoint fields. It never changes what an earlier release answers: every vector of an
earlier release passes unchanged (`docs/IMPLEMENTATION-PLAN.md` §1).

## Orthogonality, trait surface and cognitive complexity

**When changing productive code, always maximize orthogonality and testable surface through traits, and minimize cognitive complexity.**

Specifically:
- prefer extracting behavior behind traits so individual pieces can be tested and swapped independently
- prefer small, focused methods with a single responsibility over large methods with many branches
- prefer named structs with methods over free functions operating on external state
- when `crap4rust` or a reviewer flags a function as too complex, reduce it by extracting internal structs with methods and adding integration coverage — not by extracting standalone helper functions
- never increase cognitive complexity to pass a test; find the root cause and fix it there
- when introducing a new protocol dependency seam, place the contract in `traits/`, place the protocol-facing state/data model parallel to the protocol, and place the concrete implementation in its own dedicated implementation area
- make constructors depend on traits, not directly on concrete implementations
- ALL dependencies are injected through the SINGLE constructor and stored in the struct
- apply the same split recursively to nested dependencies: trait first, state/data model second, concrete implementation third

## User coding standards

- one struct per file
- do not use fully qualified paths; use `use` imports instead
- no unnecessary comments in code
- no `&mut` input parameters; prefer return values (`&mut self` is allowed in traits and impl blocks)
- only use `pub mod` in `mod.rs` and `lib.rs`
- one test file per source file, named `<source file name>_tests.rs`
- in `all_tests.rs`, reference test files one by one without `#[path = ...]`
- AAA (`Arrange`, `Act`, `Assert`) structure in tests; `// Arrange & Act` or `// Act & Assert` where two merge
- the header in `docs/header.txt` on every Rust source file
- **test naming**: `<method>_<description>_<outcome>` where `<method>` is the function called in the Act section
