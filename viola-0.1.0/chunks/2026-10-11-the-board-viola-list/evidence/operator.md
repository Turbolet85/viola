# Operator pass — 2026-10-11-the-board-viola-list

On the operator's word in this session's `/andromeda-implement` arguments (`inputs#I4`): the operator pass
with the `ci.py conclusion` read, as usual. Lines are copied verbatim, the tools' headers left out.

## Entry 13 re-fired before the pre-CI commit

`gate.py run … --only 13` (firing 5, journal line 5) on the final uncommitted tree:

```
 13 probe       green · exit 0 · 64.95s · 310343 B → 13.3.log · bash scripts/agent-run.sh pre-push
entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
```

The document read `"ok":true,"stage":"linux-tests"`: coverage 2406 passed / 0 failed, playwright 1 passed, gate
breaches `[]`.

## Entry 14 — hygiene, fired as written before the pre-CI commit

```
hygiene: clean — read 55 (runs 46 · evidence 3 · inputs 6) · trails 16 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`f1ee63419e14` — `chore(2026-10-11-the-board-viola-list): operator pre-CI commit, for the run this chunk's verdict
reads`, the whole tree (`git add -A`), on `build/viola-0.1.0`. The record above rode it; the sections below were
written after it and ride the wrap commit.

## Entry 15 — the push, fired through `gate.py run --operator 15`

```
operator entry 15 · history tripwire: git
 15 probe       green · exit 0 · 2.39s · 90 B → 15.log · history moved: refs/remotes/origin/HEAD 995dc55a→f1ee6341; refs/remotes/origin/build/viola-0.1.0 995dc55a→f1ee6341 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
```

After it, `git rev-list --count @{u}..HEAD` read 0.

## Entry 16 — the CI read for the pre-CI commit: red

`ci.py conclusion --sha HEAD --wait 1800`, fired as written:

```
f1ee63419e14 verdict: red · checks 15/15 · first-fail +501 s test (windows-2025) · wall 501 s · runs ci#38123340339 com…
runs: ci#38123340339 push completed/failure
failed 1: test (windows-2025) (failure)
run failed: ci#38123340339 failure
```

The failed step's log (`gh run view 38123340339 --log-failed`), `Coverage and doctest (pwsh shim)`:
`2402 tests run: 2398 passed (5 slow), 4 failed`. The four reds, each `viola list --json` exiting `Some(21)`
where `Some(0)` was asserted, stderr empty:

- `chaos_forward_compat chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads`
- `chaos_torn_append chaos_torn_tail_left_by_a_stopped_wrapper_is_healed_by_the_next_start` (the new reading)
- `cli_list list_logs_its_start_and_exit_only_under_a_name`
- `cli_list list_of_a_stopped_wrapper_is_an_empty_board`

**Cause, read from the runner's split and the fixture's source.** These four were exactly the cases that booted
`StampedHome::unstamped(TestHome::new())`. Every stamped-home case of the same binaries passed on that runner
(`path1_…`, `list_human_…`, `list_after_a_pause_…`, both `e5_…`, `path6_list_…`), and so did
`list_of_a_home_that_does_not_exist_creates_nothing`. On Windows x64, `Wrapper::boot` calls
`tests/support/home.rs` `seed_conpty`, which creates the home with a plain `fs::create_dir_all` before `viola
run`: the home then carries the runner's inherited DACL, not the protected one viola lays down. The board's
strict-modes check refused that home, as specified, with exit 21. In a stamped home `viola verify` creates the
home first with viola's own DACL. The product read is right; the four cases' fixture was not, and plan step 6
already asks every `cli_list` case to start from `stamped_home`. Not reproducible on the Linux host:
`seed_conpty` is a no-op off Windows x64, so the local `pre-push` was green on the red tree.

**The fix (test-side, in this chunk's own files):** the four cases boot from `stamped_home`.
`chaos_forward_compat` and `chaos_torn_append` therefore join the `[profile.ci]` 20 s line by its rule (every
verify-driven root binary). Host clippy, the Windows-target clippy (`target/wincheck`) and fmt read exit 0.
Firing 6 (`--only 1,2,5,13`): all four green. Entry 5 read `35 passed`, the four moved cases each about 4 s;
entry 13 `pre-push` read `"ok":true`.

## The fix commit, its push and its CI read: green

`acad7afc865a` — `fix(2026-10-11-the-board-viola-list): operator fix after CI run 38123340339, the four list
readings on an unstamped home start from a stamped one`. Hygiene before it read `clean`.

```
 15 probe       green · exit 0 · 1.54s · 90 B → 15.log · history moved: refs/remotes/origin/HEAD f1ee6341→acad7afc; refs/remotes/origin/build/viola-0.1.0 f1ee6341→acad7afc · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
```

```
acad7afc865a verdict: green · checks 15/15 · wall 511 s · runs ci#38124062614 completed/success
runs: ci#38124062614 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

The run this chunk's verdict reads: **ci#38124062614, green, 15/15, on `acad7afc865a`.**
