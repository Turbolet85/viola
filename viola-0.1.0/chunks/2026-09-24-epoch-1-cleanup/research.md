# Codebase Research — 2026-09-24-epoch-1-cleanup

## Scope
- **Depth:** deep · **Reads:** 13 · **Globs/Greps:** 14 · **Measurements:** 7 (tokei, rust-code-analysis, jscpd,
  `cargo mutants --list` ×2, a cargo-modules probe, the code-graph query)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full; 1 Session Addition applied: `--in-diff` mutates only
  lines the diff touches) · `.claude/rules/testing.md` (read in full; 9 Session Additions, 4 applied: unobservable body = unkillable
  mutant; waits bounded below the 20 s floor with `try_wait`; one process-global per test; `||`→`&&` token-swap coverage)
- **Platform issues consulted:** none to search. No failure signature exists: CI on 9df9e45 (run 36038410173) completed `success` at
  the P5 re-read, and scope carries no runner-only bullet. The plan's CI-reading entries read a future run of this chunk's push. A red
  there is searched at that time, with its signature (validation check 9).

## Files inspected
- `crates/viola-core/src/lib.rs` (1–35) — `MAX_FRAME: u64 = 16 * 1024 * 1024` at :9. It has an inline `#[cfg(test)] mod tests` (:32, ViolaName
  proptests/units), which is the pin test's home.
- `src/obs.rs` (160–234, 420–717) — `read_diagnostics_level` (:190) is the sole `MAX_FRAME` consumer: `file.take(MAX_FRAME).read_to_end` (:195),
  then `parse_diagnostics_level` (:202). The test module has the `level_of` helper (:423), the rstest case table (:431), `detail_line_…` (:506,
  home-level key asserts :521–527) and `internal_error_detail_line_carries_the_chain` (schema load :661–670).
- `src/main.rs` (1–30, 160–303) — `mod obs;` (:2). The test module holds `panic_line_has_the_binding_fields_and_no_corr` (:208, key asserts
  :218–228) and `panic_hook_writes_detail_line…` (:241, schema load :292–301). `main.rs` and `obs.rs` are modules of the same `viola` bin crate.
- `crates/viola-e2e/src/harness/run.rs` (full production part 1–733; test-module index 734–1728) — production is 733 physical lines. The
  inline `mod tests` is 994 physical lines (734–1728), about 60 % of the file. Functions by concern:
  - dispatch: `Selection`, `Suite`, `run`, `Refusal`, `run_with`, `run_forwarding`, `merge_summary`;
  - nextest/junit: `junit_source`, `junit_suite`, `nextest`, `exit_must_agree`, `attr`, `parse_junit`;
  - coverage: `coverage`, `COVERAGE_IGNORE`, `COVERAGE_FLOORS`;
  - doctest: `doctest`, `count_before`, `parse_doctest`;
  - fuzz (:399–475);
  - mutation gate (:477–732).
- `crates/viola-e2e/src/bin/viola-harness.rs` (full) — `main` (:107) is one clap match over 9 commands with inline handler bodies.
- `crates/viola-e2e/tests/cli.rs` (full) — tests the binary directly. Each command reaches a non-success or document-asserting path: boot
  invalid-instance, run invalid-leg, status/logs unknown-session, cleanup doc and keep, supervise exit 2, gate usage and suite-missing.
- `src/bin/viola-fake-agent.rs` (1–480) — `main` (:419) handles hold-stdout, version, step assembly (inject turn + `--script`), the agent and
  receipts, the script thread, and the stdin loop with the `exit_no_eof` branch. It keeps a crate-level print allow (:6).
- `tests/cli_fake_agent.rs` (1–440 + fn index) — the clone class is the "echo_plugin + fixtures + `Direct::spawn` with
  `--plugin-dir`/`--fixtures` (+ one mode flag)" preamble, repeated across the hook and mode tests.
- `crates/viola-e2e/src/harness/mod.rs` (mod list) — `pub mod run;` (:5). `gate.rs:10` imports `super::run::{COVERAGE_FLOORS, leg_verdict_path}`.
- `scripts/orphans-check.sh` (14–49) — it runs `cargo modules orphans -p … --lib|--bin … [--features] --deny`, with no `--cfg-test`.
- `.github/workflows/ci.yml` (mutants job) — the legs are `ubuntu-latest` and `windows-2025`, with `timeout-minutes: 360`; each runs `agent-run.sh run --mutants --leg`.

## Graph impact
- **run.rs external surface** (canonical query 5, `tree-query-2026-09-24-epoch-1-cleanup.json`, rust plane):
  - `Selection` ×3 · `run_forwarding` ×2 · `run_with` ×2 · `from_flags` ×1 plus its 5 fields, all in one file (`bin/viola-harness.rs`);
  - `leg_verdict_path` ×3 · `COVERAGE_FLOORS` ×2 (`harness/gate.rs`).

  Nothing else outside run.rs reaches it. So the split keeps exactly these names at the `viola_e2e::harness::run::` path (the binary's
  `use` at :13 and `gate.rs:10`); every other item is free to move.
- **Crate edges:** `viola-e2e → viola-core` only (audit B2). No product crate depends on `viola-e2e`.

## Measured facts (re-derived at HEAD 9df9e45 — the folded CARRY's numbers, re-run)
- **Cognitive** (`rust-code-analysis-cli -m -O json` over the audit's `source-files.txt`, scratch `cog.py`):
  - over 15: `run_with` run.rs:110 = 24 · viola-harness `main` :107 = 19 · fake-agent `main` :419 = 16 (3 over 15, 831 functions);
  - at the ceiling: `scan_file` secret_scan.rs:103 = 15.

  Re-derived and equal to the CARRY.
- **Size:** `tokei --output json run.rs` → `code` 1 562 (physical 1 728). Re-derived, equal.
- **Clones** (the audit's jscpd command, verbatim, into scratch): 16 pairs, 1.42 %. The CARRY's 7 named pairs are all present. Two more
  pairs of the same helper class sit in the same two files: `tests/cli_fake_agent.rs:138↔380` (6 L) and `src/obs.rs:516↔645` (6 L, both
  inside its `#[cfg(test)]` module). Every other pair is in files outside the entry.
- **Mutants** (`cargo mutants --list --workspace --features fake-agent --file …`, lists at `runs/…-phase/mutants-list-*.txt`):
  - run.rs 154, at lines 44–721, **0 at or after :734**. cargo-mutants 27.1.0 generates no mutant inside the `#[cfg(test)]` module. The
    CARRY's 154 is re-derived and equal.
  - viola-harness.rs 4 (`emit`, `main`, `:154:47 delete !`, `:175:78 == → !=`).
  - viola-fake-agent.rs 67, of which `main` has 2 (`:420` Default, `:452:8 delete !`).
  - viola-core lib.rs 12, the 4 `MAX_FRAME` survivors at `:9:31`/`:9:38` among them.
- **run.rs mutation history:**
  - blame of lines 1–733: 346 from b0236ca · 12 from 966b7aa · 51 from 61f9676 · 324 from 3f385dd;
  - the `mutants` check runs on those commits: b0236ca failure, then 966b7aa success (same chunk) · 61f9676 success · 3f385dd success (both legs + union).

  [hypothesis: each chunk's CI diff base spanned its whole chunk, so every production line of run.rs was once mutation-tested green at
  authoring; a split re-tests them against today's tests, so survivors are possible but not expected.]
- **Orphans probe** (a scratch crate, cargo-modules 0.27.0): `lib.rs` with `#[cfg(test)] mod t;` plus `src/t.rs` → `Found 1 orphans`, rc 1.
  **An out-of-line `#[cfg(test)]` module file is an orphan to the gate as the script runs it.** Tests that move must stay inline in the
  file they test (`mod tests { … }` inside each new submodule file). A new `#[cfg(test)] mod x;` file is not an option unless the gate
  itself changes, which is out of this chunk.
- **`--in-diff` scope mechanism:**
  - `chunk_diff` (run.rs:510) is `git diff <merge-base>` plus untracked files as `/dev/null` diffs, with git 2.53 rename detection on by default.
  - Code moved into NEW files (`run/<concern>.rs`) appears as wholly added lines, so every moved production line is mutated.
  - Code left in `run.rs` is mutated only where its lines changed. Lines deleted from `run.rs` are never mutated.
  - Hence the split's mutant scope ≈ the moved production functions plus the reshaped `run_with`, up to the 154.
- **`--browser` at HEAD:** unbuilt. `cli.rs:60` asserts `run --browser` → exit 2 `reason:"usage"`. The a11y extract's
  `browser-linux-only` / `browser-missing` outcomes are target state with no code at HEAD. The only obligation here is that
  `run --browser` stays a usage error.
- **P4/P6 output-discipline cases:** none at HEAD (`ls tests` holds no `tui_*`; there is no `crates/viola-e2e/tests/tui_*`).
- **logs / status:** `harness/logs.rs` and `harness/status.rs` are not in this chunk's modify set, so their output cannot change.

## Patterns detected
- **Runner seam** (run.rs:29 `Runner`, :110 `run_with`): every tool invocation goes through an injected `FnMut(&mut Command)`. The harness's
  own tests drive `run_with` with a recording stand-in (run.rs:1241 `Calls`, :1268 `run_coverage`), so a submodule function stays testable
  without nesting cargo.
- **Mini-workspace tests** (run.rs:966 `GOOD_LIB`, :971 `mini`): a throwaway `viola`-named Cargo project runs real nextest/doctest/mutants
  for the `run_*` integration-style unit tests. This is the slow viola-e2e class (the 20.6 s test).
- **Document assembly order:** `json!({"v":1,"cmd":"run","ok":ok})`, then `reason`/`detail`, `suites`, `mutants` (run.rs:162–172). Key
  order is serde_json `preserve_order`, so an extracted document builder must insert in exactly this order.
- **Binary-level harness tests** (cli.rs:12 `harness`, :20 `document`): one document, no SGR, leading keys `v, cmd, ok`. This is the killing
  surface for any extracted `viola-harness` handler whose return is an `ExitCode`.
- **Receipt-driven fake-agent tests** (cli_fake_agent.rs:27 `Direct`): spawn, send, finish, and assert on receipt lines. They kill fake-agent
  `main` helper mutants through observed receipts and exits.

## Conventions to follow
- **Inline unit tests:** `#[cfg(test)] mod tests` inside the file under test (viola-core lib.rs:32, run.rs:734, obs.rs:341), named
  `<subject>_<condition>_<expected>`. For moved tests this is mandatory, per the orphans probe above.
- **Oracle literals** (testing.md §What to assert): the MAX_FRAME pin compares with a literal (`16 << 20` / `16_777_216`), never with an
  expression built from the constant.
- **Waits bounded, no sleeps** (testing.md Session Additions 2 and 6): any new test that a mutant can reach detects the watched process's
  exit and stays under the 20 s floor.
- **Harness lints:** `viola-e2e` keeps its own `[lints.clippy]` (prints allowed). The fake agent keeps the crate-level print allow at :6.

## New files to create
- `crates/viola-e2e/src/harness/run/` submodule files (one per concern, e.g. `junit.rs` / `coverage.rs` / `doctest.rs` / `fuzz.rs` /
  `mutants.rs`, final names left to /implement). Each carries its production functions AND its inline `#[cfg(test)] mod tests` (moved
  tests), and each stays ≤ 800 code lines. `run.rs` keeps the dispatch plus `pub mod`/`pub use` lines that preserve the 5 external names.

## Files to modify
- `crates/viola-core/src/lib.rs` — add the MAX_FRAME pin test inside its existing test module.
- `src/obs.rs` — test module only:
  - a boundary witness for `read_diagnostics_level` at the cap;
  - the `detail_line…` key asserts (:521–527) and the schema load (:661–670) switch to the shared test helper.
- `src/main.rs` — test module only: the panic-line key asserts (:218–227) and the schema load (:292–300) switch to the shared helper. The
  helper lives inline (a `#[cfg(test)]` item in an existing module; no new file, per the orphans probe).
- `crates/viola-e2e/src/harness/run.rs` — the split plus the `run_with` decomposition (≤ 15). The names `Selection` (+`from_flags`),
  `run_with`, `run_forwarding`, `COVERAGE_FLOORS` and `leg_verdict_path` stay reachable at `harness::run::`.
- `crates/viola-e2e/src/harness/gate.rs` — **no change** expected: its `use super::run::{COVERAGE_FLOORS, leg_verdict_path}` (:10) resolves
  through `run.rs`'s re-exports. Listed as a boundary member.
- `crates/viola-e2e/src/bin/viola-harness.rs` — the `main` decomposition (≤ 15). The `use` at :13 keeps resolving.
- `src/bin/viola-fake-agent.rs` — the `main` decomposition (≤ 15).
- `tests/cli_fake_agent.rs` — the clone-class preamble extracted into a local helper (the only consumer file, so not `tests/support/`).
- `crates/viola-e2e/tests/cli.rs` — **no change** expected. It is the binary-level killing surface for the viola-harness handlers, and a
  gap it leaves (a handler with no non-success test) is a new case here.
- Companion sweep:
  - `MAX_FRAME`: 3 hits over `*.rs` — lib.rs:9 (changed by a new test), obs.rs:19 (use, no change), obs.rs:195 (consumer, no change).
  - `harness::run`: 2 hits (viola-harness.rs:13, gate.rs:10), both no change.

## Open questions
- Split shape: a concern split that MOVES production code into new submodule files puts up to 154 mutants in `--in-diff` scope; that is the
  overseer's stated reason, the first viola-e2e mutation witness. Moving only the 994-line test module cannot satisfy the target: an
  out-of-line `#[cfg(test)]` file is an orphan, and inline it does not shrink run.rs. So "under 800" requires moving production code with
  its tests. → blocks: plan-decision. It narrows to which concern split, since a no-mutant test-only move is measured unavailable. Resolved
  at P4 as a lean, not a question.
- Survivors after the split (hypothesis above) → blocks: implementation-scope. Any survivor gets a killing test in this chunk
  (fold-do-not-carry), which can grow viola-e2e's test delta.
