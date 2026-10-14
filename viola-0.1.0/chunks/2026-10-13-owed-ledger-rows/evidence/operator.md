# Operator pass — 2026-10-13-owed-ledger-rows

Run on the operator's word: the `/andromeda-implement` arguments ("Run the operator pass with the ci.py conclusion
read (leg=operator) as usual."). It was held on the surfaced fixture handle until the founder's own word,
`inputs#I3`: accept and push, given live in the overseer's dialog at 2026-10-14T05:57Z. Lines are copied verbatim;
the tools' headers are left out.

## Entry 23 (pre-push) on the final uncommitted source tree

`gate.py run` (the whole non-leg block, `gate-run-5.txt`), after the last source edit:

```
 23 probe       green · exit 0 · 66.74s · 329411 B → 23.3.log · bash scripts/agent-run.sh pre-push
```

After it, only evidence and run-dir records were written, and no source changed.

## Entry 24 — hygiene, fired as written before the pre-CI commit

```
 24 probe       green · exit 0 · 0.24s · 459 B → 24.log · history: unmoved · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

Its output (`gate.py show … --n 24`):

```
hygiene: clean — read 58 (runs 51 · evidence 2 · inputs 5) · trails 15 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

The commit `chore(2026-10-13-owed-ledger-rows): operator pre-CI commit, for the run this chunk's verdict reads`, sha `9ee83095ace5`.
- It holds the whole tree (`git add -A`) on `build/viola-0.1.0`, and the two StatusLine fixtures are in it.
- The sections above ride this commit. The sections below were written after it and ride the wrap commit.

## Entry 25 — the push, fired through `gate.py run --operator 25`

```
operator entry 25 · history tripwire: git
 25 probe       green · exit 0 · 2.26s · 90 B → 25.log · history moved: refs/remotes/origin/HEAD 14e62329→9ee83095; refs/remotes/origin/build/viola-0.1.0 14e62329→9ee83095 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

After the push, `git rev-list --count @{u}..HEAD` read 0.

## Entry 26 — the CI read for the pre-CI commit: red

`ci.py conclusion --sha HEAD --wait 1800`, fired as written:

```
 26 probe       red · contains 'verdict: green' ✗ · exit 0 · 435.64s · 467 B → 26.log · history: unmoved · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 26 · green 0 · red 1 (26) · recorded 0 · timeout 0 · not-run 25
```

Its log (`gate.py show … --n 26`):

```
repo Turbolet85/viola (the push remote `origin`) · polled 15× over 436 s
9ee83095ace5 verdict: red · checks 15/15 · first-fail +441 s test (macos-latest) · runs ci#38519633153 in_progress/-
runs: ci#38519633153 push in_progress/-
failed 1: test (macos-latest) (failure)
running 1: oldest test (windows-2025) 452 s
run open: ci#38519633153 in_progress
```

The run then ended with two failed jobs, each one test (read from the job logs):

- `test (macos-latest)`: `viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`
  panicked at `tests/cli_send.rs:919:5` (exit `Some(0)`, want `Some(10)`); 2526 of 2527 passed.
- `test (windows-2025)`: `viola-agent-claude ledger::tests::verified_needs_every_row_pass_under_that_version`
  panicked at `crates\viola-agent-claude\src\ledger.rs:1985:9`, "a missing row" (`Ok(true)`, want `Ok(false)`);
  2522 of 2523 passed.

Both causes are this chunk's own:

- **macOS.**
  - The cause: the fake agent's first-Stop status line replay ran between the Stop hook and the paste-hint clear.
    While it spawned `viola hook statusline`, the earlier input box stood on screen, and the second send's gate wait
    read it as settled. So the text was typed before the human key arrived.
  - Read on Linux with a 2 s probe sleep in the replay: red at `cli_send.rs:919`.
  - The fix: the replay moved to the end of `end_turn`, after the turn screen. With the sleep still in, the case
    went green, and so did `cli_send`, `cli_verify`, `hook_statusline` and the drift contract (80 of 80). The sleep
    was then removed.
- **Windows.**
  - The cause: the case popped `ALL`'s last row, `statusline-rate-limits`, which Windows does not rely on, so the
    stamp still read verified.
  - The fix: the case now drops `relied()`'s last row. That row is the same one on Unix and `paste-newline` on
    Windows.

## The fix commit

- Before it: the whole non-leg block, green 17 of 17 (`gate-run-6.txt`, `pre-push` 68.02 s), and a Windows-target
  clippy, clean.
- Entry 24 re-fired: green.
- `1c4f2258`: `fix(2026-10-13-owed-ledger-rows): the fake agent's status line runs after the turn screen; the
  missing-row case drops a relied row`.

```
operator entry 25 · history tripwire: git
 25 probe       green · exit 0 · 1.65s · 90 B → 25.2.log · history moved: refs/remotes/origin/HEAD 9ee83095→1c4f2258; refs/remotes/origin/build/viola-0.1.0 9ee83095→1c4f2258 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

## Entry 26 — the CI read for the fix commit: green

```
 26 probe       green · exit 0 · 560.72s · 569 B → 26.2.log · history: unmoved · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

```
repo Turbolet85/viola (the push remote `origin`) · polled 19× over 561 s
1c4f2258ef7c verdict: green · checks 15/15 · wall 550 s · runs ci#38520999442 completed/success
runs: ci#38520999442 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```
