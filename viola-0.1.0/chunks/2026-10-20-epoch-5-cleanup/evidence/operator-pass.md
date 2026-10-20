# Operator pass — 2026-10-20-epoch-5-cleanup

Run by the builder session on the operator's word (2026-10-20, this session, at the `/andromeda-implement`
invocation): «Then the operator pass as usual, the ci.py read included; stop before the wrap.» It follows implement's
run (`.andromeda/runs/2026-10-20T08-57-19-implement/`).

## `pre-push` on the uncommitted tree (entry 23)

Fired in implement's block on the uncommitted tree, after the last code edit (only evidence and the outcome file were
written after it):
`23 probe       green · exit 0 · 86.92s · 388254 B → 23.log · bash scripts/agent-run.sh pre-push`, with 0 `header is
corrupt` lines in its log.

## Entry 24 — `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`

Fired as written on the uncommitted tree, before 2026-10-20T09:23:44Z (the `date -u` read right after it). Its
output, verbatim:

```
gate v1.22 · 47c603a3
root . · case exact · planes rust · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 52 (runs 41 · evidence 6 · inputs 5) · trails 12 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

Clean on its first firing. It was re-fired once after this file was written, so that this file is read too:
`hygiene: clean — read 53 (runs 41 · evidence 7 · inputs 5) · trails 12 not read · copies 3 not read by P1 — 0 host
paths kept · binary 0 not read by P1`.

## The pre-CI commit

`5a3fb42a` — `chore(2026-10-20-epoch-5-cleanup): operator pre-CI commit, for the run this chunk's verdict reads`, made
with `git add -A` on top of `d35644a5`. The tree was clean after it.

## Entry 25 — `python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/plan.md --operator 25`

The entry's line and the call's summary, verbatim (header left out):

```
operator entry 25 · history tripwire: git
 25 probe       green · exit 0 · 2.44s · 92 B → 25.log · history moved: refs/remotes/origin/HEAD d35644a5→5a3fb42a; refs/remotes/origin/build/viola-0.1.0 d35644a5→5a3fb42a · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

The history move is the push the entry exists to make.

## Entry 26 — `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400`

Fired as written, in the background. It finished before 2026-10-20T09:38:57Z (the `date -u` read after it). Its
output, verbatim:

```
ci v1.3 · 49546645
repo Turbolet85/viola (the push remote `origin`) · polled 28× over 840 s
5a3fb42ab8fb verdict: green · checks 15/15 · wall 814 s · runs ci#39400402749 completed/success
runs: ci#39400402749 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

`gh run view 39400402749 --json attempt,conclusion,headSha` reads `1 · success · 5a3fb42ab8fb0aac4c43912407755dd7573cee59`.
Every job was green on attempt 1, so there was no fix commit.

### The OS-sensitive cases, by the legs that witness each (the entry's note)

Read from the run's JUnit artifacts (`junit-windows-2025` 12255074214, `junit-ubuntu-latest` 12255330289,
`junit-macos-latest` 12254669787) by `$TMPDIR/{scratchpad}/junit_read.py`. windows-2025 has 2 957 testcases, ubuntu
2 980 and macOS 2 976. None has a `failure`, `error`, `rerunFailure` or `flakyFailure` element.

- **Step 6's `cfg(windows)` scratch case** — `test (windows-2025)`:
  `harness::run::mutants::scratch::tests::wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not`
  passed in 0.061 s, with its control made by the explicit `icacls` grant (absent on the Unix legs, as its `cfg`
  says).
- **Step 7's per-test child copy and unconditional kill** — every `viola-pty` case green on all three legs: windows 44
  cases, ubuntu 40, macOS 40, 0 failing. `tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code`
  took 0.086 / 0.018 / 0.191 s, and `tests::close_hangs_up_a_live_child` (Unix) 0.015 / 0.064 s.
- **Step 10(c)'s `cfg(unix)` process group** — the hook-driven cases green on ubuntu and macOS, the whole suites
  included. Named: `chaos_wait_parked_wrapper_killed_exits_21` took 1.105 / 0.167 s (windows 0.256 s), and
  `path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human` (also step 9's `dialogless_home` caller) took
  6.063 / 5.483 s (windows 5.496 s).
- **Step 11's `cfg(unix)` `statvfs` read** — no leg has the e2e-home link, so the link branch is witnessed on none. The
  plain-directory branch ran on all three (every test home). The decision cases passed on every OS:
  `home_base_headroom_below_the_floor_is_refused_with_both_counts` 0.026 / 0.004 / 0.033 s, and
  `harness::tests::e2e_home_headroom_refuses_below_the_floor_and_passes_at_or_above_it` 0.019 / 0.005 / 0.018 s. The
  Unix reader `harness::tests::e2e_home_headroom_reads_a_real_filesystem` took 0.005 / 0.018 s on ubuntu and macOS.
- **Step 2's concurrent walks** — every leg's contract binaries green, under the `ci` profile's 20 s kill:

| case | windows-2025 | ubuntu-latest | macOS | before (run 39383206773, w / u / m) |
|---|---|---|---|---|
| `fake_agent_dialog_replay_matches_every_recorded_dialog_set` | 5.749 | 8.442 | 6.242 | 15.027 / 14.097 / 16.962 |
| `fake_agent_framing_replay_matches_every_recorded_framing_set` | 5.712 | 7.346 | 5.952 | 9.847 / 9.373 / 11.546 |
| `fake_agent_statusline_replay_matches_every_recorded_statusline_set` | — (Unix) | 6.842 | 6.103 | — / 9.266 / 11.348 |
| `contract_ledger_probes_pass_over_every_stamped_set` | 5.414 | 7.170 | 5.751 | 10.146 / 9.233 / 11.634 |

- The new pin `mutants_paste_hint_override_holds_the_class_above_the_default_kill` passed on all three legs.
- WATCH (the macOS readiness stall): `test (macos-latest)` was green on attempt 1, with no stall.

The operator pass stops here, before the wrap.
