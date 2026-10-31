# Operator pass — 2026-10-31-exit-cause-code-catalogue

Run by the session on the operator's word (`inputs#I4`, the /andromeda-implement invocation of 2026-10-31: "Run the operator pass with the ci.py conclusion read (leg=operator) as usual; its first push carries the two spine-door commits."). Base: HEAD `404f7496dbd7` before the pre-CI commit. `origin/build/viola-0.1.0` read `16ef6b4813e7`, so the first push also carries `df4ca577` and `404f7496`.

## Entry 20 re-run — `bash scripts/agent-run.sh pre-push` (the final uncommitted tree)
Fired through `gate.py run --entry 20`, started 2026-10-31T17:37:32Z:
```
 20 probe       green · exit 0 · 95.36s · 445566 B → 20.3.log · bash scripts/agent-run.sh pre-push
entries 23 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 22
```
Its log: `Summary [  72.546s] 3403 tests run: 3403 passed (5 slow), 0 skipped`, and `{"v":1,"cmd":"pre-push","ok":true,"stage":"linux-tests","linux":{"run":{"ok":true,"suites":[{"suite":"coverage","passed":3403,"failed":0,"skipped":0,"failures":[]},{"suite":"doctest","passed":0,"failed":0,"skipped":0,"failures":[]}]},"gate":{"ok":true,"breaches":[]}}}`.

Host record: `QUIET for viola - 0 s stalled on IO`.

WATCH, `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1`, in this firing: PASS `4.491 s` (`case_1_sigint`), PASS `4.484 s` (`case_2_sigterm`).

## Entry 21 — fired as written, read 2026-10-31T17:39:40Z
The first firing read `hygiene: refused 2 files — P1 2`: the phase run's `p5-controls.md:42` and `p5-resume.md:22`, each the temp path of the `target/e2e-home` link. Each was rewritten to its `$TMPDIR/…` spelling, nothing else changed, and the verb re-fired.

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`
```
gate v1.23 · 2601bda6
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 658 (runs 650 · evidence 2 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
This record is written after that read, as the plan template says.

## The pre-CI commit
`git add -A` once entry 21 read `clean`, then `115cdbb9 chore(2026-10-31-exit-cause-code-catalogue): operator pre-CI commit, for the run this chunk's verdict reads`; the tree clean after it. The master record stays `pending`, `v1-18` and `v1-39` `implemented`.

## Entry 22 — `python -X utf8 {tools_dir}/gate.py run --plan … --operator 22` (no run dir), read 2026-10-31T17:40:10Z
```
operator entry 22 · history tripwire: git
 22 probe       green · exit 0 · 3.57s · 92 B → 22.log · history moved: refs/remotes/origin/HEAD 16ef6b48→115cdbb9; refs/remotes/origin/build/viola-0.1.0 16ef6b48→115cdbb9 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
not run — excluded · 22: 1-21,23
entries 23 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 22
```
The history move is the push this entry exists for. It carried `df4ca577`, `404f7496` and the pre-CI commit, as the operator directed.

## Entry 23, first read — fired as written: red
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400`
```
ci v1.3 · d6f6c9c7
repo Turbolet85/viola (the push remote `origin`) · polled 3× over 63 s
115cdbb94736 verdict: red · checks 15/15 · first-fail +48 s lint (macos-latest) · runs ci#41225642322 in_progress/-
runs: ci#41225642322 push in_progress/-
failed 1: lint (macos-latest) (failure)
log: gh api --allow-escape-sequences repos/Turbolet85/viola/actions/jobs/124609103907/logs
running 12: oldest test (ubuntu-latest) 64 s
run open: ci#41225642322 in_progress
```
The cause, read from the job's log: `error[E0425]: cannot find function text in this scope` at `crates/viola-mcp/tests/handlers.rs:539`. This chunk's new case `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` runs on every OS, and the helper `text` was `#[cfg(not(target_os = "macos"))]`. The chunk's red, a fix in a listed file.

The macOS check-only clippy (test-plan §9) is a recipe, not an entry, and I had not run it before the push: it would have read this. It reads exit 0 after the fix, 2026-10-31T17:42:00Z.

The run was left to finish, 2026-10-31T17:53:39Z: 13 of 15 jobs green. `test (macos-latest)` is the other red, the same error, its tests never built. `test (windows-2025)` read `3353 tests run: 3353 passed (5 slow), 0 skipped`, `test (ubuntu-latest)` `3403 tests run: 3403 passed (14 slow), 0 skipped`. WATCH on this run, `test (ubuntu-latest)`: `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` PASS `6.297 s` / `6.273 s`.

## The fix: entries 2, 3, 4 and 20 re-run on the fixed tree
```
  2 lint        green · exit 0 · 1.34s · 0 B → 2.4.log · cargo fmt --all --check
  3 lint        green · exit 0 · 0.46s · 217 B → 3.5.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  4 lint        green · exit 0 · 0.36s · 155 B → 4.6.log · CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace … (140 chars)
 20 probe       green · exit 0 · 96.08s · 445566 B → 20.4.log · bash scripts/agent-run.sh pre-push
entries 23 · green 4 · red 0 · recorded 0 · timeout 0 · not-run 19
```
Pre-push's log: `Summary [  72.755s] 3403 tests run: 3403 passed (5 slow), 0 skipped`. Host record: `QUIET for viola - 0 s stalled on IO`. WATCH in this firing: PASS `4.478 s` / `4.465 s`.

Entry 21 re-fired before the fix commit, read 2026-10-31T17:55:45Z: `hygiene: clean — read 2 (runs 2 · evidence 0 · inputs 0) · trails 1 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1`. This section is written after that read.
