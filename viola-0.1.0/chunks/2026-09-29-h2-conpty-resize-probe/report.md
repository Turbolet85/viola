# Report — 2026-09-29-h2-conpty-resize-probe

**Chunk:** H2 ConPTY resize probe — the key lost after a ConPTY resize localised (resize vs key), reproduced on Windows a set number of times, then fixed or documented
**Date:** 2026-09-29
**Commits:** (since last_wrap 2026-09-28T21:24:15Z; basis `git log --format='%h %s' 90aba7c..HEAD`)
- `3d04d1c` chore(…): operator pre-CI commit, for the run this chunk's verdict reads
- `d8b5051` chore(…): measurement only — H2 loop, 200 iterations on windows-2025
- `dce98ad` fix(…): the H2 red test sends its key once the child sees the new size, and the loop measures it (measurement only)
- `6d05367` chore(…): remove the H2 measurement loop, ci.yml byte-identical to 90aba7c
- `8a98b9d` fix(…): H2 documented as a measured ConPTY limit, and the harness self-tests' nested cargo no longer inherits the coverage run
- (`90aba7c` chore(setup-project): upgrade viola — U02, U07 — the base, not this chunk's)

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-only 90aba7c` + the working tree) `crates/viola-pty/src/lib.rs` (tests module only) ·
  `crates/viola-e2e/src/harness/run.rs` (`#[cfg(test)] mod test_support` + the `mod tests` import) ·
  `crates/viola-e2e/src/harness/run/mutants.rs` (`#[cfg(test)] mod tests` only) · `.claude/docs/gotchas.md` ·
  `.claude/docs/services/viola-pty.md` · `.github/workflows/ci.yml` (changed in `d8b5051`, restored byte-for-byte in
  `6d05367`: `git diff --quiet 90aba7c -- .github/workflows/ci.yml` exit 0) · the chunk folder (plan/research/scope,
  `scope-record.md`, `evidence/`: `localisation-rule.md`, `host-localisation.md`, `h2-loop-step.md`,
  `h2-reproduction.md`, `entry-6-e2e-plan-defect.md`, `operator-pass.md`) · route/master bookkeeping and run dirs.
- **Symbols / APIs:** no product symbol, API, endpoint, IPC method, port or product env var changed — every source
  edit sits inside a `#[cfg(test)]` module (basis: the diff hunks of the three `.rs` files, all below each file's
  `#[cfg(test)]` line).
  - viola-pty `mod tests` (test rig): `watch_size(path, start)` — a child-side thread polling `host_size()` with
    `yield_now`, appending `size {c}x{r}` on each change (every child mode); `Child` gains `dsr: Arc<AtomicUsize>`,
    `dsr_reported`, `test_report: PathBuf` and methods `step` · `report_dsr` · `reports`; `report_path()` →
    `report_paths()` (the child's `<test>.report` + the test's `<test>.test.report`, both removed on a pass, kept on a
    panic/kill; `Drop` writes `dsr-cpr {n}` on a panic not yet reported); `count_dsr(carry, read)` + `DSR_CPR` (counts
    `ESC [ 6 n` in the drained output, straddles included); `wait_line(child, want)` (exact line, deadline panic prints
    both reports) and `wait_start(child)` replace the index-based `lines(child, n)` (removed); `resize(child)` /
    `key(child, k)` helpers with steps `resize-returned` · `key-written` · `key-flushed`; new child mode `watch`
    (reports `start`, runs the watcher, parks, never reads) on `PTY_SEAM_TEST_MODE` — read only by the test child,
    beside the existing `run` · `hold` · `block`.
  - New tests: `spawn_reports_a_resize_to_a_child_that_reads_no_key` (key-free resize witness),
    `count_dsr_counts_a_request_split_across_reads_once`. Reshaped: `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code`
    now `resize` → `wait_line("size 120x40")` → `key(b'y')` (was: key written right after `resize` returned). The
    `hold` sibling keeps key-right-after-resize.
  - viola-e2e `test_support`: `COVERAGE_ENV` (`CARGO_LLVM_COV`, `LLVM_PROFILE_FILE`, `RUSTC_WRAPPER`,
    `__CARGO_LLVM_COV_RUSTC_WRAPPER_RUSTFLAGS`), `uninstrumented(cmd)` (env_remove of the four, then the production
    `run_forwarding`) and a test-side `run(ws, sel, filter, chunk_base)` over it. `run.rs`'s tests import it in place
    of the production `run` (explicit import over the `super::*` glob); `mutants.rs`'s tests import it, and
    `run_private`'s runner calls `uninstrumented`. The production `run` / `run_forwarding` / `run --coverage` are
    unchanged; remaining production callers of `run_forwarding`: `src/bin/viola-harness.rs:158`, `:233`, and
    `run::run` itself (basis: `grep -rn run_forwarding crates/viola-e2e/src`).
- **Crates / modules:** none added or removed.
- **Dependencies:** none (viola-pty `Cargo.toml` unchanged — the guard entry reads exit 0).
- **Schema / config:** none. No config key, schema, redaction shape.
- **Spec-master edits:** none this chunk before P2.
- **Counts / qualifiers moved:** the viola-pty unit count 22 → 24 (P5 baseline 22; `run --unit --filter
  'package(viola-pty)'` 24 passed, run-archive 289); the host resize-filter selection 4 → 5 tests per iteration
  (`Starting 5 tests` ×20, gate entry 4). `run` (default) 718 unit + 216 integration (gate entry 5); pre-push linux
  coverage 919, windows coverage 934. These appear in no master (basis: grep of `22 passed` / `4 tests selected`
  → 0 hits in `.andromeda/*.md` masters; the 4-tests baseline lives only in the plan).
- **Dev-tool versions:** none — cargo-llvm-cov re-read at 0.9.1 on the dev host (`cargo llvm-cov --version`); its
  measured refusal `--no-report may not be used together with --no-clean` is a flag fact at the pinned 0.9.1, not a
  version change. No lockfile-resolved crate involved.
- **Harness / gate surface:** CI: a temporary step `H2 loop (measurement only)` in the `test` job (windows-2025 only)
  existed in `d8b5051`/`dce98ad` and was removed in `6d05367` — the standing ci.yml is byte-identical to `90aba7c`
  (7 jobs / 15 check-runs unchanged). Harness self-tests: the nested cargo over the throwaway `mini` crate now runs
  without cargo-llvm-cov's four names, so the throwaway's binaries no longer write profiles into the outer
  `run --coverage` set (measured on the host, `cargo llvm-cov nextest --no-report --profile ci -p viola-e2e -E
  'test(=harness::run::tests::run_reports_unit_integration_and_doctest_suites)'`, new `.profraw` per run: channel
  open 17 ×2 runs, two signatures `2995…` ×1 · `3916…` ×3 present; channel closed 13 ×3 runs, those two absent).
  No harness command, status or verdict shape changed; `run --e2e` still does not exist (see Spec claims).
- **Cross-project / external claims:**
  - CI (repo Turbolet85/viola, read with `ci.py conclusion`): `3d04d1c` ci#36527341834 green 15/15 (wall 280 s) ·
    `d8b5051` ci#36527891850 green 15/15 (537 s; job `test (windows-2025)` 109274838484: `h2-loop: iterations 200 ·
    losses 13`) · `dce98ad` ci#36529038462 **red** — `test (ubuntu-latest)` 109278323561: 919/919 tests passed, then
    `llvm-profdata merge` failed on a corrupt-header `viola-4974-15303557482808059277_2.profraw` (folded, below);
    job `test (windows-2025)` 109278323590 `h2-loop: iterations 200 · losses 0` · **`8a98b9d` ci#36529984077 green
    15/15 (246 s)** — the final HEAD, confirmed by the overseer at this wrap's invocation. This wrap's own commit
    adds to that tree.
  - Runner image `windows-2025-vs2026` 20260922.246.2, provisioner 20260828.587, Windows Server 2025 (job logs) — the
    same image as the ci#36436266196 sighting. Host: `ver` 10.0.26200.9457, conhost 10.0.26100.8875.
  - microsoft/terminal PR #19535 (post-resize DSR CPR): no `ESC[6n` observed after a resize on either build.
- **Reverted / negative API facts:** the `H2 loop` ci.yml step (measurement only, inserted then byte-restored);
  `--no-clean` in the loop's `cargo llvm-cov nextest --no-report` (refused by cargo-llvm-cov 0.9.1, caught by the
  loop's fail-closed branch on the host dry run, dropped before any push); a temporary `Drop` edit keeping the
  test-side report on a pass (the host `dsr-cpr` debug read), restored (`grep -c TEMP-H2` → 0).
- **Insufficient fixes (written, kept, not the remedy):** the reshaped red test (key after the child's `size
  120x40`) makes the TEST deterministic (200/200 on the runner) but says nothing about a key racing a resize: the
  measured loss (13/200, a Rust test child reading with `ReadConsoleW`) lies between the ConPTY input-pipe write and
  that child's read, cause not established; whether the real `claude` (Node/libuv, `ReadConsoleInputW`) loses such a
  key in `viola run` is unmeasured — owned by the real-CLI verify entry (CARRY pinned at P5), beside the founder's open
  product question (entry CARRY 1), recorded in gotchas.md. The coverage-channel fold closes the one measured path
  that puts non-suite binaries into the coverage set; that the ci#36529038462 corrupt file came through it is an
  INFERENCE (the log names no writer) — a recurrence on a later HEAD places the cause elsewhere.
- **Spec claims disproved by measurement:**
  - Overseer relay (scope.md Folded freight 2, carried as HYPOTHESIS): "the RESIZE itself never reached the child" —
    measured false on 13 of 13 runner losses (class K; class R 0; `evidence/h2-reproduction.md`). Not a master claim.
  - Hypothesis H2-CPR (research.md, plan Goal): ConPTY requests the cursor position after a resize and waits —
    `dsr-cpr 0` on every runner loss and on the host. Not a master claim.
  - Plan `## Test Commands` entry `bash scripts/agent-run.sh run --e2e` (from test-plan §3's `run` command body,
    `.andromeda/test-plan.md:533`) — the harness has no `--e2e` selector: exit 2 `unexpected argument '--e2e'`, the
    same on a clean worktree at `90aba7c` (`evidence/entry-6-e2e-plan-defect.md`). test-plan §12 `2026-09-24` already
    records "Unbuilt selectors are usage errors"; the defect is the plan listing it as a gate. Disposition (operator,
    operator-pass invocation): a plan defect, not a red to fold — pinned on working-route :85 "The board: viola list",
    the entry whose CARRY lands the first `viola-e2e` `path_` E2E binary, its chunk the owner of `run --e2e`.
- **Expected amendments (from plan):**
  - architecture §Established Decisions [PTY] — the measured H2 finding (count, classes, builds, run ids) as an
    as-built fact: **carried** — Changes: Cross-project claims + Insufficient fixes + Spec claims (13/200 K, dsr-cpr 0,
    builds, ci#36527891850 / ci#36529038462). Site: `grep -n '\[PTY\]' .andromeda/architecture.md` → 1 hit (:47).
  - architecture §Occupied Resources → Filesystem — the `<test name>.test.report` sibling in
    `<temp dir>/viola-pty-watch/`, test-only: **carried** — Changes: Symbols (`report_paths`). Site: `grep -c
    viola-pty-watch` → architecture 1 hit (:396), every other master 0.
  - test-plan §5 Module ↔ PTY — document branch only: the witness's key-after-resize wait on the observed signal, with
    the measured limit cited: **carried** (the document branch was taken) — Changes: Symbols (the reshaped red test) +
    Insufficient fixes. Site: `grep -n 'Module ↔ PTY' .andromeda/test-plan.md` → :926 (the §5 row) and :1897/:1903
    (§12 entries).
- **Coverage of new surfaces:** no new external surface, hot-path op or UI element. Test-rig only:
  - `viola-pty tests::watch_size / *.test.report / count_dsr` → validation n/a · instrumentation n/a (test code; no
    `obs_event!`) · PII n/a (codes only: sizes, byte hex, counts) · tests unit ✓ (24 in the package; host loop 20/20;
    runner 200/200 after the reshape) · a11y n/a · tokens n/a
  - `viola-e2e test_support::uninstrumented` → validation n/a · instrumentation n/a · PII n/a · tests unit ✓ (238
    viola-e2e + viola-pty passed) · a11y n/a · tokens n/a

## Deviations from intent
- **Step 2 lines written as each step returns** (as planned) — kept; the added delay is a file append (sub-ms to ms)
  against the ~50 ms class window; push 1 still reproduced 13/200.
- **Step 3 `dsr-cpr` written at three points:** at the verdict, at a wait's deadline panic (before the reports are
  printed), and by `Drop` on any other panic not yet reported — the plan said "when it reads the verdict or panics".
- **Step 4 `wait_start`** (a `start ` prefix wait) for the tests whose start line is not asserted exactly; the run
  test waits on its exact start line, and its former assertion message "a key arrived only with Enter" became a code
  comment (the deadline panic names the missing line).
- **Step 6 counting:** `localisation-rule.md` counts only R and K toward the 3; E and UNCLASSIFIED are recorded, not
  counted — per the plan's "E … is not a loss".
- **Step 8 loop additions** beyond the plan's command (which is kept exactly, one test selected per iteration):
  `TMP`/`TEMP` pinned to `$RUNNER_TEMP/h2-tmp` so kept reports are found; the job's `target/nextest/ci/junit.xml`
  copied aside and restored (each loop iteration rewrites it and `Upload JUnit` ships it); fail-closed exit 1 on an
  iteration that fails with no kept report; `H2_ITERATIONS` (default 200). Host dry run 3/3 before any push.
- **Push 2 is a verification push on the document branch:** the plan lists a verification measurement push only on
  the fix branch; it was run here to measure the reshaped test before calling it non-flaky (2 of the 3 measurement
  pushes; the reproduction count was reached at push 1). It carried the reshape commit with the loop still in.
- **The ubuntu red of push 2 folded** — outside research's lists; recorded as widenings.
- **Entry 6 `run --e2e`** — a plan defect, recorded and pinned, not folded (operator).
- **Scope record** (`gate.py scope` at P1: `scope: clean — changed 3 · listed 1 · recorded 2 (… widening 2) ·
  excluded 42`):
  - widening · `crates/viola-e2e/src/harness/run.rs` · serves step 9 · word: "Fold every red into this chunk." — the
    operator, at the /andromeda-implement invocation, 2026-09-29
  - widening · `crates/viola-e2e/src/harness/run/mutants.rs` · serves step 9 · the same word

## Decisions & corrections
- Operator (implement invocation): "Fold every red into this chunk. Keep to the fixed count (3 losses, 3 pushes or
  fewer), and read CI through the ci.py conclusion tool call."
- Operator (operator-pass invocation): "Entry 6 --e2e is a plan defect, not a red to fold: record it, and have the
  wrap pin it on the route entry that lands the first E2E test binary, with its owner named."
- Operator (wrap invocation): "Pin entry 6 --e2e on working-route :85 with its owner. The coverage-channel CARRY on
  :59 is now done here, so retire it there by name." CI verified by the overseer: ci#36529984077 on `8a98b9d`, 15/15.
- Branch decision by the plan's pre-stated rule: document (K losses, dsr-cpr 0; the only named viola-controlled cause
  needed dsr-cpr ≥ 1).
- **Overseer correction (wrap, before the commit):** microsoft/terminal's current source (the overseer's web research,
  sources in the overseer log — a relay, not on disk here) shows the resize path does not flush the input buffer, and
  no issue reports resize input loss, so "the loss is inside ConPTY" is NOT established. The measured loss is between
  the pipe write and the Rust test child's `ReadConsoleW` read; the real `claude` (Node/libuv, `ReadConsoleInputW`) is
  unmeasured. Applied: gotchas.md and services/viola-pty.md worded "cause unestablished, product impact unmeasured for
  claude, owner the real-CLI verify entry"; this wrap's arch [PTY] and test-plan §5 amendments and their two sidecar
  entries reworded to match; the evidence carries a dated correction; a CARRY pinned on working-route "First live test
  and self-drive". The branch stands (no loss localised to a viola-controlled cause). `crates/viola-pty/src/lib.rs`'s
  test comment ("can be lost below viola, the child reading") stays: the loss is after viola's write, and the wrap
  touches no source.
- **Sweep hazard:** cargo-llvm-cov names EVERY instrumented process's profile `<workspace>-%p-%m` (here `viola-…`),
  so a `.profraw` file name never identifies its writer; only the `%m` signature separates binaries, and it maps to
  no name in the job log.
- **Measured tool fact:** cargo-llvm-cov 0.9.1 refuses `--no-report` with `--no-clean`.
- **Measured shell fact:** on the GitHub Windows runner, a `shell: bash` step reaching `std::env::temp_dir()` is made
  deterministic by exporting `TMP`/`TEMP` as a `cygpath -w` path (the redirect reached the test on the host).
- A test that writes a key after a ConPTY resize waits for the child to observe the new size first (the reshape rule).

## Outcome
**Acceptance criteria** (re-asserted against the diff):
- (tests) a lost post-resize key localises itself — MET: both reports, kept on panic (13 runner losses printed whole),
  removed on a pass; witness `spawn_reports_a_resize_to_a_child_that_reads_no_key` passes in `run --unit --filter
  'package(viola-pty)'`.
- (tests) the host answer — MET: `size 120x40` with no key read, host loop 20/20, `dsr-cpr 0` in
  `host-localisation.md`; no host result used as runner evidence.
- (tests) the fixed count — MET: 3 within ≤ 3 pushes of 200, fixed before the run; `h2-reproduction.md` records
  push 1 (ci#36527891850, `d8b5051`, job 109274838484, 200, 13, K ×13, dsr-cpr 0 each) and push 2; verdict
  *reproduced 3 (classes K)*; nothing retried to reach it.
- (tests) the branch its rule selects — MET: document; gotchas.md H2 states the product window stays open (unmeasured
  for `claude`, owned by the real-CLI verify entry), names the measured 13/200 (6.5 %) as the test child's rate, says
  the reshaped test does not cover it; no H2 test retried, `#[ignore]`d, skipped or compiled out.
- (tests) every OS green — MET: `run` and `pre-push` green locally (before every push); final HEAD
  ci#36529984077 `verdict: green · checks 15/15`; coverage floors held (the `Gate verdict` / pre-push gate `breaches:
  []`); `COVERAGE_IGNORE` unchanged (`coverage.rs` untouched).
- (arch) ci.yml byte-identical to `90aba7c` — MET (`cmp` equal; guard exit 0; 7 jobs / 15 check-runs).
- (arch) viola-pty `Cargo.toml` unchanged, deps unchanged, `cargo deny check` green — MET (gate entries 7, 9).
- (security) no `viola` build reads a new env var; `watch` rides the test child's spawn env inside `#[cfg(test)]`; no
  `PtyError` arm — MET. The viola-e2e change only REMOVES names from a test's child command.
- (a11y) no wait/queue/reorder on `pump`'s write path — MET (`pump.rs` untouched); pump contract tests and the three
  tui resize cases green (gate entry 5).
- (obs) no `obs_event!`, event, field or span; the DSR count lives only in the test rig — MET.

**Gates** (/implement run `implement-2026-09-29T05-24-12`, by `run`):
- `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green
  · `bash scripts/agent-run.sh run --unit --filter 'package(viola-pty)'` green (24) · the 20× host resize loop green
  (52 s) · `bash scripts/agent-run.sh run` green · `bash scripts/agent-run.sh run --e2e` **red · exit 0 ✗ (exit 2)** —
  `red — not this chunk's: the same exit 2 on a clean worktree at 90aba7c (control) → the working-route :85 "The board:
  viola list" pin (P5)` · `cargo deny check` green · `bash scripts/agent-run.sh pre-push` green · `git diff --quiet
  90aba7c -- .github/workflows/ci.yml crates/viola-pty/Cargo.toml` green · the four smoke entries (cleanup · boot ·
  status · cleanup `processes_gone`+`endpoint_gone`) green.
- `leg = 'operator'` entries (recorded in `evidence/operator-pass.md`): hygiene `clean` before the pre-CI commit ·
  the push entry exit 0 for each of 4 pushes · the report-only `ci.py conclusion --wait 2400` read: push 1 green
  15/15 ci#36527891850 (recorded; H2 losses 13 — its purpose), push 2 **red** ci#36529038462 (recorded; the ubuntu
  corrupt profile — folded by `8a98b9d`) · `gh run view <id> --log` tallies 13 and 0 · the acceptance read on
  `8a98b9d`: `verdict: green · checks 15/15` ci#36529984077 (atoms held).
- Smoke: boot/status/cleanup green; no boot-path change (all source edits under `#[cfg(test)]`).

**Watches:** none folded.

**Outcome basis:** the operator pass ran (pre-CI `3d04d1c`, parent `90aba7c`); the verdicts rest on its final state —
`8a98b9d`, ci#36529984077, recorded in `evidence/operator-pass.md` — and on this conversation, which holds implement
and the pass whole.

**Process hygiene:** implement's census and the post-pass census (`Get-CimInstance Win32_Process` by repo
`ExecutablePath` and harness tool names): none left running; the smoke's wrapper 20568 / child 21064 gone; the
WSL pre-push VM `terminated:true` each run; the control worktree removed (`git worktree list` → the main tree only).
Five `viola.exe` of `additional/viola-lab/prototype` were running — not this chunk's.
