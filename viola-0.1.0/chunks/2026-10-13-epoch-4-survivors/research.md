# Codebase Research — 2026-10-13-epoch-4-survivors

## Scope
- **Depth:** deep · **Reads:** 31 · **Globs/Greps:** 22
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 16 Session Additions applied
  (no `run --mutants` in a gate; a hand-applied mutant is read from the test's own failure; `test(=tests::name)` filter
  form); `.claude/rules/testing.md` — read in full, 41 Session Additions applied (operator-token mutants, the
  remove-the-guard run, `const` flag mutants, I/O errors per OS, no reshaping for mutants, an unobservable body);
  `.claude/rules/testing-src.md` — its pointer list. No live leg in this chunk (`inputs#I1`: no live `claude`).
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a read `c6e1301` green, 15/15) and no CI-reading
  entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator relays the founder: no epoch boundary inside Epoch 5; no mutation
  run, witness run or workflow dispatch inside the chunk; no live `claude` start without his cap; sized against one
  window at P5.

## Files inspected
- `.andromeda/runs/2026-10-12T06-54-29-code-audit/record.json` (`mutation.*`) — the 47 survivors (re-derived:
  `len(record["mutation"]["survivors"]) == 47`), host `x86_64-unknown-linux-gnu`, per-unit `counts.missed` (viola 32,
  viola-pty 1, viola-state 10, viola-agent-claude 2, viola-e2e 2, viola-core 0 — sum 47).
- `.andromeda/runs/2026-10-12T06-54-29-code-audit/mutants-viola-pty/mutants.out/log/crates__viola-pty__src__lib.rs_line_202_col_9.log`
  (full, 187 lines) — the `PortablePty::resize` mutant's own test run (§Measured facts 1).
- `src/cmd/verify/typed.rs` (1–62, 195–320, 380–475, 520–700, 708–845) — Run B's driver and its unit tests.
- `src/run/wheel.rs` (100–135, 270–520, 530–700) — `Recorder::record`, `Classifier`, `is_reply`, the classifier
  tables (`classifier_typing_is_editing` 49 cases, `classifier_terminal_reply_is_not_editing` 8, `REPLIES` 23).
- `src/run/dialog.rs` (138–215) — `millis` (two callers, graph), `DialogSlot::hook_dialog`'s arm retain.
- `src/cmd/hook.rs` (390–450) — the capture arm's `answer`, `ordinal`, `free_k`.
- `src/cmd/send.rs` (105–130) — `Out<'_>::open`.
- `src/run/send/slot.rs` (195–225) — `close` (now `slot.rs:206`, its body `:207`).
- `crates/viola-state/src/board.rs` (38–75, 155–170) — `check_if_present`, `scan`, `SkippedTotal::add`.
- `crates/viola-state/src/fs.rs` (248–300, 380–450) — `retry_replace`, `replace_private_with`, its tests (two of
  them `#[cfg(windows)]`).
- `crates/viola-state/src/snapshot.rs` (95–110) — `write_snapshot_with`, the one product caller beside
  `replace_private`.
- `crates/viola-agent-claude/src/statusline.rs` (100–118, 375–386) — `shell_argv`, `shell_argv_on`, and the test
  that checks only `shell_argv(command).is_some() == cfg!(unix)`.
- `crates/viola-e2e/src/harness/run/mutants/scratch.rs` (35–75, 150–185) at HEAD and at `dffad3e` — `prepare`.
- `crates/viola-pty/src/lib.rs` (195–210, 598–800, 895–1000, 1175–1270) — `PortablePty::resize`, the child entry,
  `watch_size`, `wait_line`, the three resize tests.
- `crates/viola-pty/src/pump.rs` (160–220) — what the pump calls on its `Pty`.
- `.andromeda/test-plan.md:1226` — §10 Mutation gate, the "not measured here" form and the `prepare` record.
- `viola-0.1.0/chunks/2026-10-13-epoch-4-cleanup/{scope,report}.md`, `.andromeda/runs/2026-10-13T07-07-28-phase/p5-card-1.md`
  — the split precedent.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-13T14-19-57-phase/tree-query-2026-10-13-epoch-4-survivors.json`, `db_state fresh`, plane `rust`)
- **replace_private_with** — 6 rows: `replace_private` @ `crates/viola-state/src/fs.rs:264`,
  `write_snapshot_with` @ `crates/viola-state/src/snapshot.rs:108`, three test sites and one `use` in
  `snapshot.rs:15`. A restatement that keeps its signature touches neither caller.
- **retry_replace** — 5 rows: `replace_private_with` @ `crates/viola-state/src/fs.rs:286` and four calls in
  `retry_replace_waits_only_for_a_windows_reader_refusal`.
- **shell_argv** — 3 rows: `serve` @ `src/cmd/hook/statusline.rs:145`, a root test `sh` @ `:591`, and the crate test
  @ `crates/viola-agent-claude/src/statusline.rs:385`.
- **prepare** — 4 rows: `mutate` @ `crates/viola-e2e/src/harness/run/mutants.rs:224`, three calls in
  `prepare_empties_the_scratch_and_reports_what_it_held`.
- **millis** (`src/run/dialog.rs`) — 2 rows: `DialogSlot::await_answer` @ `src/run/dialog.rs:278` (the span's
  `deadline_ms`) and `DialogSlot::answer` @ `:380` (`duration_ms`).

## Patterns detected
- **Classifier tables** (`src/run/wheel.rs:571-660`): rstest `#[case::label]` rows for typing and non-typing, plus the
  `REPLIES` list fed whole and split at every byte. Every kill here is a row in these tables, or a split-read case.
- **Pause seam on the replace** (`crates/viola-state/src/fs.rs:268`): `replace_private_with` takes the pause as
  `&mut dyn FnMut()`. The tests count pauses, and on Windows bound them with a panic past the last attempt
  (`fs.rs:440-447`).
- **Pure decision beside a const** (`fs.rs:257` `retry_replace(raw, host_is_windows)`,
  `statusline.rs:113` `shell_argv_on(command, wrapped)`): the const-gated decision is a plain function taking the
  flag, tested both ways. The survivors sit in the code that passes the const, not in the decision.
- **Child-entry PTY tests** (`crates/viola-pty/src/lib.rs:631`, `:895`): the test binary re-runs itself under the PTY
  as `tests::pty_child_entry`, which reports each step to a file; waits are on report lines under
  `CHILD_WITHIN` = 7 s, below the `mutants` profile's 10 s kill.

## Conventions to follow
- **Test names** `<subject>_<condition>_<expected>`, inline `#[cfg(test)] mod tests` (testing.md §Naming;
  2026-09-25 inline test modules).
- **Operator-token mutants**: cover each `&&`/`||` condition with a case whose leading operand is false
  (testing.md 2026-09-24) — `hook.rs:403`'s `!event.is_dialog() || !answers.is_absolute()`, `wheel.rs:518`.
- **The remove-the-guard run**: each new test is shown red under its hand-applied mutant, then green on the restored
  file, with the site re-read before each edit (the rustfmt hook reflows; testing.md 2026-09-25).
- **I/O errors per OS**: a forced persist failure must fail at the same call on every CI OS (testing.md 2026-09-24).
- **No reshaping for mutants** (testing.md 2026-09-28): never move a body out of cargo-mutants' generated set; a
  restatement keeps every mutant generated and makes it killable.

## Measured facts
1. **`PortablePty::resize` (`viola-pty` `lib.rs:202:9`).** The audit's log for this mutant shows viola-pty's own three
   resize tests ran under it and PASSED: `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code`
   5.352 s, `spawn_reports_a_resize_to_a_child_that_reads_no_key` 5.404 s, and
   `spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading` 6.106 s, all marked SLOW past 5 s
   (log lines 177–184; `40 tests run: 40 passed`). Each waits for the child's `size 120x40` report line
   (`lib.rs:1213`, `:1247`, `:1265`). So under a resize that does nothing, the child still reported 120×40 about 5 s
   later. What changed the child's size is NOT measured: the watch reports under `<temp dir>/viola-pty-watch/` are
   gone (the audit scratch's dir is empty). The tests sit in `viola-pty` itself, so the tests-history item that
   places the resize tests in the root package does not hold for these three.
   HYPOTHESIS (read at P4, not measured): a report written by another job. The audit ran cargo-mutants with
   `-j 2` (`.andromeda/runs/2026-10-12T06-54-29-code-audit/mut-driver.py:18`, `JOBS = 2`, passed as `-j` at `:53`)
   and one `TMPDIR` for both jobs (`:59`). The child's report path is `<temp dir>/viola-pty-watch/<test name>.report`,
   keyed by test name only (`crates/viola-pty/src/lib.rs:852-866` `report_paths`). The child appends to it
   (`report`, `:618-626`), and `wait_line` takes any matching line in the file (`:951-964`). So the other job running
   the same test on an unmutated `resize` can write `size 120x40` into the file the mutant's test waits on.
2. **`crates/viola-state/src/fs.rs` `replace_private_with` (6).** With `HOST_IS_WINDOWS` false the guard
   `attempts < REPLACE_ATTEMPTS && retry_replace(raw, false)` is false, whatever `attempts` is. So `:281:18`
   (`+=`→`*=`), `:285:20` (guard→`false`) and the three `:285:29` swaps behave as the original on Linux. But
   `:286:21` (`&&`→`||`) does not: on any failed `persist` it reads `attempts < 100 || false`, pauses, and retries 99
   times before returning the error. No Linux test makes `persist` fail, which is why it read missed. A target that
   is an existing non-empty directory fails `persist` on all three OSes.
3. **`scratch.rs` `prepare` (2).** At `dffad3e` and at HEAD, `:47` is `fn prepare`, `:48` its first statement
   `if !HOST_SCRATCH`, and `:54` `if !scratch_allowed(&root, &scratch)`, inside the `HOST_SCRATCH` branch. Both
   mutants are reached only when `HOST_SCRATCH` (`cfg!(windows)`) is true. test-plan §10 already records them "not
   measured here", read caught on `windows-2025` in run 38036448183 (job `mutants (viola-e2e)`).
4. **`typed.rs` `SharedPty` (5).** `typed.rs`'s pump is `pump_with_paste(&mut SharedPty(..), .., Size::DEFAULT,
   &mut || None, ..)` (`typed.rs:569-578`). The pump calls `resize` only when its size closure reports a new size
   (`pump.rs:186-190`), so never here, and never calls `child_pid` (`pump.rs`: `try_wait` `:170`/`:211`, `close`
   `:171`/`:219`, `resize` `:190`, `kill` `:208`). `SharedPty::resize` and `::child_pid` are trait-required
   forwarders no product path calls. `close` is called at the child's exit. `SharedPty` wraps the concrete
   `PortablePty`, and `Run` holds `Arc<Mutex<PortablePty>>`: there is no `Pty` double seam in `typed.rs`.
5. **`typed.rs`, the rest (13).** `max_gap`'s filter (`:304:54`) is a pure function; its test
   (`typed.rs:715`) has no chunk outside the window on BOTH sides that changes the answer. `Feed::take`'s
   `total > MAX_FRAME` (`:439:23`) is tested past the cap only, never at exactly `MAX_FRAME`. `framing_turns`
   (`:271:84` ×2, `:286:29`, `:288:40` ×2) needs a `Run` with a live pump and paste handle. `Run::end_by_ctrl_c`
   (`:648:12`, `:651:12`, `:648:45`, `:651:45`), `::wait_pump` (`:659:9`) and `::finish` (`:671:53`) need a `Run`
   whose pump the test controls. No unit test builds a `Run`; the root tests reach these only through a whole
   `viola verify` against the fake agent, where the fake's speed hides each mutant.
6. **`src/cmd/send.rs` `Out<'_>::open` (`:119:12`).** The condition is `!self.json && io::stdout().is_terminal()`.
   Under nextest stdout is never a terminal, so both the original and the mutant print nothing in every test.
7. **`src/cmd/hook.rs` `answer` (`:403:27`).** `!event.is_dialog() || !answers.is_absolute()`: with the leading
   operand false (a dialog event) and a relative `answers`, the original returns at once. The mutant reads the answer
   file at `answers.join(..)` relative to the process cwd, which no test has planted.
8. **`statusline.rs` `shell_argv` (2).** The crate test checks `shell_argv(command).is_some() == cfg!(unix)`
   (`statusline.rs:385`), which `Some([""; 3])` and `Some(["xyzzy"; 3])` both pass on Unix.
9. **`board.rs` (4), `wheel.rs` (9), `dialog.rs` (2), `slot.rs` `close` (1)** — each reachable on Linux through a unit
   case in its own file; none needs a restatement (sites read; the cases are P4's).

## The size against one window (`inputs#I1`)
- **Precedent:** the Epoch 3 cleanup II was one builder window: 18 survivors, shared test scaffolding, three
  functions split, one whole-unit score (`viola-0.1.0/chunks/2026-10-09-epoch-3-cleanup-ii/report.md:4`). The Epoch 4
  cleanup's card sized 47 survivors plus two file splits as not fitting, and the founder split them off
  (`.andromeda/runs/2026-10-13T07-07-28-phase/p5-card-1.md:3-14`, `inputs#I2` there).
- **This entry:** 45 sites to dispose by test or restatement, plus 2 standing records, in 11 files across five
  packages. 31 of them are unit cases in their own files (`wheel.rs` 9, `board.rs` 4, `dialog.rs` 2, `hook.rs` 1,
  `slot.rs` 1, `statusline.rs` 2, `typed.rs` `max_gap` and `Feed::take` 2, `viola-pty` `resize` 1 after its
  measurement) or rest on a small restatement (`fs.rs` 6, `Out::open` 1). The other 16 (`typed.rs` `framing_turns` 5,
  `SharedPty` 5, `Run`'s ending sequence 6) rest on one restatement of verify's Run B driver: a `Pty` double seam in
  `SharedPty` and `Run`, and named predicates for the framing waits. That is new test scaffolding the size of the
  Epoch 3 cleanup II's.
- **Reading:** the whole entry is about 2.5× the precedent's measured window. The 31 alone are about 1.7× the
  precedent's survivor count, but with no scaffolding and no score run.

## New files to create
- none

## Files to modify
- `src/run/wheel.rs` — classifier and recorder cases
- `src/run/dialog.rs` — `millis` and the per-tool arm cases
- `src/cmd/hook.rs` — the capture arm's relative-answers case
- `src/cmd/send.rs` — the issue-line decision as a named function, with its cases
- `src/run/send/slot.rs` — the `close` case
- `src/cmd/verify/typed.rs` — `max_gap` and `Feed::take` boundary cases only (the Run B driver restatement left with the split, `inputs#I2`)
- `crates/viola-state/src/board.rs` — the skip-count sum and NotFound-guard cases
- `crates/viola-state/src/fs.rs` — the retry decision passed in, with its cases
- `crates/viola-agent-claude/src/statusline.rs` — the whole-argv case
- `crates/viola-pty/src/lib.rs` — the resize kill, once the mutant's run is measured

## Sweeps
- none — no name sweep ran (no rename; every signature P4 may change is named with its callers above).

## Open questions
- Whole entry (47) or split, with `typed.rs`'s 16 Run B driver survivors as their own entry directly after this one?
  → blocks: plan-decision. RESOLVED at P4 by the founder's own answer, split (`inputs#I2`).
- What changes the PTY's size about 5 s into the `resize` mutant's run? → blocks: implementation-scope. The
  `viola-pty` disposal's form (a test, or a restatement of the tests' wait) is read from the hand-applied mutant's
  child report, the first step of that item.
