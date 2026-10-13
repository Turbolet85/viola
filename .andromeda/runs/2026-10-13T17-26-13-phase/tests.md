# tests extract

## Relevance
relevant — the chunk is a test-strength chunk: 16 mutation survivors in `src/cmd/verify/typed.rs` disposed under test-plan §10's Mutation gate, by owning-package nextest cases, behaviour-preserving restatements or not-measured records.

## Constraints
- A survivor is disposed only by a test that kills it or, for a mutant this host cannot compile or reach (inside a `cfg(windows)` item or block, or behind a const such as `cfg!(windows)`), by a record "not measured here; owed to {route entry}" by coordinate — never "equivalent", never counted as caught; a mutant behind a const compiles on both hosts and stays a record by coordinate (per test-plan §10 Mutation gate). Whether any of the 16 sites is actually host-unreachable on Linux is research's question.
- A mutant caught only by a doctest counts as missed under nextest, so each kill must be a nextest case, not a doctest (per test-plan §10 Mutation gate).
- No chunk, pre-push or CI mutation gate exists; mutation runs only at the epoch boundary (`/andromeda-code-audit`) or on demand via `run --mutants`, and the boundary audit's run is the reader that confirms kills (per test-plan §10 Mutation gate; per test-plan §2 Test pyramid, Mutation row; per test-plan §9 Mutation row). The chunk's no-run direction sits inside this.
- Unit tests live in the inline `#[cfg(test)] mod tests` of the same source file, named `<subject>_<condition>_<expected>`, with table cases as rstest `#[case]` carrying readable labels (per test-plan §4 Conventions).
- Mock only traits the product already defines at its seams (`Pty`, liveness probe, `Clock`), through `#[cfg_attr(test, mockall::automock)]` on seam traits only, with dependency injection over runtime patching (per test-plan §8 Mocking libraries table, Process mocking row; per test-plan §8 Anti-monkey-patching). Whether `viola-pty`'s `Pty` double is reachable from the root bin's tests, and whether `typed.rs` already takes an injected `Clock`, is research's question.
- Time in test-side code comes only from an injected clock (the `viola_core::Clock` seam, or an instant passed as an argument to a pure decision); no test-owned timer, sleep or elapsed-time verdict (per test-plan §8 What to mock, Time; per test-plan §11 Universal).
- Zero flakiness: nextest `retries = 0`; no `#[ignore]` as a parking place; a flake keeps the chunk red until its root cause is fixed inside the chunk (per test-plan §10 Zero-flakiness budget).

## Patterns to follow
- The earlier chunk's restatement forms that made a site reachable on Linux and then killed: a decision passed in by the caller (`viola_state::fs`'s retry decision) and a decision made a named function (`viola send`'s issue-line decision), each shown red under the hand-applied mutant and green restored (per test-plan §10 Mutation gate).
- PTY consumers are unit-tested with a mockall double of the six-method `viola-pty` seam whose reader never returns EOF while `wait()` returns an exit status (per test-plan §4 What unit tests cover, viola-pty).
- A real-PTY test spawns `viola-fake-agent` (never `claude`) through `viola-pty`; exit comes from the handle's `wait()`, and a test that acts after a resize waits for the child's own size report (per test-plan §5 Boundary types covered, Module ↔ PTY row).
- Pure-function boundary cases at unit tier: thresholds and deadlines as labelled clock cases at their boundary instants, the deadline passed or injected rather than waited out (per test-plan §4 What unit tests cover, viola-agent-claude screen-signature item; per test-plan §8 Mocking libraries table, Time mocking row).

## Anti-patterns to avoid
- NEVER run the real `claude` CLI or `viola verify` against the real CLI in CI; a green fake-agent run is no proof of real-CLI behaviour (per test-plan §11 Test Strategy).
- NEVER monkey-patch production code at runtime or over-stub; NEVER introduce a failpoint crate that pulls tokio into the sync crates — fault seams go through mockall doubles of existing traits (per test-plan §11 Mocking).
- NEVER use `sleep(N)` or real time without injection for synchronisation, and NEVER treat a Linux-only run as proof for Windows OS-branch code (per test-plan §11 E2E; per test-plan §11 Universal; per test-plan §11 Test Strategy).

## Contract bindings
- tests ↔ harness: the unit tier runs as `scripts/agent-run.sh run --unit` locally and as the instrumented `run --coverage` nextest run in each per-OS CI `test` job ending `viola-harness gate --require coverage,doctest,playwright` (per test-plan §9 Pipeline structure, Unit and Coverage report rows; test-plan §3 `run`).
- tests ↔ harness (mutation verdict): the Epoch 5 boundary audit's `run --mutants --package viola` counts `missed` / `timeout` / `unviable` / `caught` from `outcomes.json` and leaves only `cfg`-false host-excluded records out, so a const-gated record still reads missed there (per test-plan §10 Mutation gate; test-plan §3 `run` step 4, Verdict).
- tests ↔ security: any PTY child a `typed.rs` test starts sits under the security plan's `viola verify` interactive-child rules (direct spawns, R8 strip, `MAX_FRAME`, no byte typed into a dialog) and never a live `claude` (per test-plan §11 Test Strategy; security-plan §Authentication & Authorization owns the child rules).
- tests ↔ architecture: a `Pty` double seam must be one of the traits the product already defines at its seams; a change to `viola-pty`'s `Pty` trait is architecture's (per test-plan §8 Anti-monkey-patching).

## Acceptance criteria contributions
- Each of the 16 `typed.rs` survivors holds exactly one disposal: a named nextest case in the root package's `typed.rs` `mod tests` shown red under its hand-applied mutant and green on the original, or "not measured here; owed to {route entry}" by coordinate for a host-unreachable site; none worded "equivalent" and none disposed by argument (per test-plan §10 Mutation gate).
- `scripts/agent-run.sh run --unit` (nextest, `retries = 0`) passes with every new case and no `#[ignore]` added (per test-plan §9 Pipeline structure, Unit row; per test-plan §10 Zero-flakiness budget).
- Per-OS coverage holds after any restatement or seam: `--fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80` (per test-plan §10 Coverage thresholds).
- Any test-only double or seam is a mockall double of an existing seam trait or an injected `Clock` / instant, behind `cfg(test)`, with no test-owned sleep or timer (per test-plan §8 Mocking libraries table; per test-plan §11 Universal).
