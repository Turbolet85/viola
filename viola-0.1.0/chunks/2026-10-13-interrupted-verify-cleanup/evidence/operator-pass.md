# Operator pass — 2026-10-13-interrupted-verify-cleanup

Fired by the implementer on the operator's word in the `/andromeda-implement` invocation ("Run the operator pass with
the ci.py conclusion read (leg=operator) as usual"), in `.claude/rules/ci.md`'s order. Times are the run's
`journal.md` lines (`.andromeda/runs/2026-10-13T18-48-52-implement/journal.md`).

## 1. The native gate (entry 17), on the final uncommitted tree
```
 17 probe       green · exit 0 · 66.18s · 320761 B → 17.4.log · bash scripts/agent-run.sh pre-push
entries 20 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 19
```
The Windows-target lint (entry 3) read green on the same code in the final whole-block run (run 4).

## 2. Hygiene (entry 18, fired as written)
```
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust · P3 ts — each fired on its synthetic known positive
hygiene: clean — read 48 (runs 42 · evidence 1 · inputs 5) · trails 13 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
Read again right before the commit, with this file written:
```
hygiene: clean — read 49 (runs 42 · evidence 2 · inputs 5) · trails 13 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## 3. The pre-CI commit
`c22161e791ff` — `chore(2026-10-13-interrupted-verify-cleanup): operator pre-CI commit, for the run this chunk's
verdict reads`, the whole tree, on `d9f01b1b8642`.

## 4. The push (entry 19, `gate.py run --operator 19`, no run dir)
The first firing read red. Its guard found the tree dirty: a `journal.md` line appended after the commit. Nothing was
pushed:
```
operator entry 19 · history tripwire: git
 19 probe       red · exit 0 ✗ (exit 1) · exit 1 · 0.01s · 0 B → 19.log · history: unmoved · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 20 · green 0 · red 1 (19) · recorded 0 · timeout 0 · not-run 19
```
That line was set aside, `journal.md` restored to HEAD (the only difference), and the entry re-fired on the clean tree:
```
operator entry 19 · history tripwire: git
 19 probe       green · exit 0 · 2.42s · 90 B → 19.log · history moved: refs/remotes/origin/HEAD d9f01b1b→c22161e7; refs/remotes/origin/build/viola-0.1.0 d9f01b1b→c22161e7 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 20 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 19
```
`git rev-list --count @{u}..HEAD` read `0` after it; HEAD and `@{u}` are both `c22161e791ff`. The set-aside line was
put back after the push.

## 5. The CI read (entry 20, fired as written)
```
c22161e791ff verdict: red · checks 15/15 · wall 902 s · runs ci#38453057445 completed/failure
runs: ci#38453057445 push completed/failure
run failed: ci#38453057445 failure
not success: test (macos-latest)=cancelled
```
**Cause, read from the job itself.** `gh api …/actions/jobs/115488618043` read `steps: 0` and `runner_name: ""`; the
log download answered 404 `BlobNotFound`. The check run's annotations: `failure — The job was not acquired by Runner of
type hosted even after multiple attempts`, and three notices `Due to capacity constraints, jobs targeting macOS arm64
runners may experience longer queue times.`

The job never ran a step on any runner, so the chunk's code was not measured on macOS. Every other job read
`success`:
- `test (ubuntu-latest)` and `test (windows-2025)`: the stop table on Linux, and the `cfg(windows)` direct-call case
  on Windows;
- `lint (windows-2025)`, `lint (macos-latest)`, `release (macos-latest)` and `perf (macos-latest)`.

Read from the two test jobs' logs (`gh api …/actions/jobs/<id>/logs`):
```
test (windows-2025), job 115488617598:
PASS [   0.022s] ( 609/2461) viola::bin/viola cmd::verify::stop::tests::console_routine_stops_on_ctrl_c_and_ctrl_break_and_leaves_close_to_the_os::case_1_ctrl_c
PASS [   0.023s] ( 610/2461) viola::bin/viola cmd::verify::stop::tests::console_routine_stops_on_ctrl_c_and_ctrl_break_and_leaves_close_to_the_os::case_2_ctrl_break
PASS [   0.022s] ( 611/2461) viola::bin/viola cmd::verify::stop::tests::console_routine_stops_on_ctrl_c_and_ctrl_break_and_leaves_close_to_the_os::case_3_close
test (ubuntu-latest), job 115488617646:
PASS [   0.006s] ( 627/2468) viola::bin/viola cmd::verify::stop::tests::install_turns_the_signal_into_the_stop_with_the_process_alive::case_1_sigint
PASS [   0.005s] ( 628/2468) viola::bin/viola cmd::verify::stop::tests::install_turns_the_signal_into_the_stop_with_the_process_alive::case_2_sigterm
PASS [   1.101s] ( 191/2468) viola::cli_verify verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1::case_1_sigint
PASS [   1.265s] ( 192/2468) viola::cli_verify verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1::case_2_sigterm
```

## 5b. The cancelled job re-run, and the verdict of record
The operator's word (2026-10-13, given here): the no-dispatch direction covers the mutation workflow, not a re-run of
the ordinary CI job a runner never picked up. `gh run rerun 38453057445 --failed` re-ran the one cancelled job on the
same sha, with no new commit and no `workflow_dispatch`. Entry 20 was re-fired as written:
```
c22161e791ff verdict: green · checks 15/15 · attempt 2 · wall 1824 s · runs ci#38453057445 completed/success
runs: ci#38453057445 push completed/success
re-run: ci#38453057445 attempt 2 — each check is its latest attempt's; an earlier attempt's result is unread
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```
`test (macos-latest)`, attempt 2, job 115496299136: `success`, 30 steps, on a hosted runner. Its log:
```
PASS [   0.014s] ( 628/2464) viola::bin/viola cmd::verify::stop::tests::install_turns_the_signal_into_the_stop_with_the_process_alive::case_1_sigint
PASS [   0.015s] ( 629/2464) viola::bin/viola cmd::verify::stop::tests::install_turns_the_signal_into_the_stop_with_the_process_alive::case_2_sigterm
PASS [   1.327s] ( 192/2464) viola::cli_verify verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1::case_1_sigint
PASS [   1.154s] ( 193/2464) viola::cli_verify verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1::case_2_sigterm
```
The attempt-1 red's cause was known before the re-run: the runner was never acquired. The re-run measured the leg; it
does not stand in for a cause.
