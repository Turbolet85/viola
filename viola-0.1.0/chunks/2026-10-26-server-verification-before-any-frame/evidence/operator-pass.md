# Operator pass — 2026-10-26-server-verification-before-any-frame

Fired by the implementer on the operator's word for this chunk (`inputs#I6`: "please run implement per the skill on
2026-10-26-server-verification-before-any-frame, through its operator pass and the CI read. …" — the operator, live,
2026-10-26). Order per `.claude/rules/ci.md` §The operator pass. Lines copied verbatim, without the tool header.

## 1. The native gate (plan entry 21, on the uncommitted tree)
`python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --entry 21`
```
 21 probe       green · exit 0 · 87.67s · 399328 B → 21.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
The Windows-target lint (ci.md 2026-10-10) read green on the host in the implement block, run 3, entry 4; no source
file changed after it.

## 2. Hygiene (plan entry 22, fired as written)
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — exit 0
```
hygiene: clean — read 459 (runs 447 · evidence 4 · inputs 8) · trails 15 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
Re-read right before the commit (this file then held sections 1–2), exit 0:
```
hygiene: clean — read 460 (runs 447 · evidence 5 · inputs 8) · trails 15 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## 3. The pre-CI commit
`git add -A` then `git commit` — exit 0:
`4aa9def9fc508bf24e1d315e8ce5825d536b3a24` · `chore(2026-10-26-server-verification-before-any-frame): operator pre-CI commit, for the run this chunk's verdict reads`
(parent `6bdfdda8`); `git status --short` empty after it. Sections 3–5 of this file were written after the push.

## 4. The push (plan entry 23)
`python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --operator 23`
```
operator entry 23 · history tripwire: git
 23 probe       green · exit 0 · 3.49s · 92 B → 23.log · history moved: refs/remotes/origin/HEAD 6bdfdda8→4aa9def9; refs/remotes/origin/build/viola-0.1.0 6bdfdda8→4aa9def9 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
The move is the one the entry intends (the pushed pre-CI commit); no force push.

## 5. The CI read (plan entry 24, fired as written) — red
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400` — exit 0
```
repo Turbolet85/viola (the push remote `origin`) · polled 3× over 64 s
4aa9def9fc50 verdict: red · checks 15/15 · first-fail +58 s lint (macos-latest) · runs ci#40295285860 in_progress/-
runs: ci#40295285860 push in_progress/-
failed 1: lint (macos-latest) (failure)
log: gh api --allow-escape-sequences repos/Turbolet85/viola/actions/jobs/121479746655/logs
running 11: oldest lint (ubuntu-latest) 69 s
run open: ci#40295285860 in_progress
```
Job 121479746655's log: `error: unused import: super::*` at `src/cmd/last.rs:85:9` and `src/cmd/pause.rs:95:9`, then
`could not compile viola (bin "viola" test) due to 2 previous errors`. The cause: the two test modules this chunk added
to `last.rs` and `pause.rs` held one case each, gated `#[cfg(not(target_os = "macos"))]`, so on macOS each module's
`use super::*` had nothing to serve. This host had no macOS target to lint.

## 6. Fix round (ci.md step 6, a new commit on the pre-CI commit, the pass repeated from step 1)
The fix: both modules gated whole (`#[cfg(test)]` stacked with `#[cfg(not(target_os = "macos"))]`), the inner gate
removed.

The host now lints the macOS target, check only: `rustup target add aarch64-apple-darwin`, then
`CARGO_TARGET_DIR=target/maccheck cargo clippy --workspace --all-targets --features fake-agent --target aarch64-apple-darwin -- -D warnings`
— exit 0 on the fix. Its known-positive control, `last.rs` put back in the pushed shape: exit 101,
`src/cmd/last.rs:85:9: error: unused import: super::*` (CI's error, line for line); restored, re-read (one module gate).
Host clippy (Linux, and the Windows target under `target/wincheck`): exit 0 each.

1. The native gate (plan entry 21) on the uncommitted fix:
```
 21 probe       green · exit 0 · 88.17s · 399328 B → 21.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
2. Hygiene (plan entry 22), exit 0:
```
hygiene: clean — read 2 (runs 1 · evidence 1 · inputs 0) · trails 0 not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
Steps 3–5 of the fix round follow below, written after its push.

3. The fix commit: `git add -A` then `git commit` — exit 0:
   `775641fa58b3f3e5cf2db1a7dfb966e398b92e91` · `fix(2026-10-26-server-verification-before-any-frame): the last and pause test modules gated whole off macOS, closing the macos lint red`
   (parent `4aa9def9`, never an amend); `git status --short` empty after it.
4. The push (plan entry 23):
```
operator entry 23 · history tripwire: git
 23 probe       green · exit 0 · 1.6s · 92 B → 23.log · history moved: refs/remotes/origin/HEAD 4aa9def9→775641fa; refs/remotes/origin/build/viola-0.1.0 4aa9def9→775641fa · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
5. The CI read (plan entry 24, fired as written) — exit 0:
```
repo Turbolet85/viola (the push remote `origin`) · polled 29× over 869 s
775641fa58b3 verdict: green · checks 15/15 · wall 854 s · runs ci#40295857215 completed/success
runs: ci#40295857215 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```
The run itself (`gh api repos/Turbolet85/viola/actions/runs/40295857215`): `run_attempt` 1, `head_sha`
`775641fa58b3f3e5cf2db1a7dfb966e398b92e91`, event `push`, conclusion `success`.

The OS witnesses, read from each test job's log (`PASS` lines naming the new cases; `FAIL` lines 0 in each):
- `test (windows-2025)` (job 121481615334): 39 — among them `viola-channel client::tests::client_server_pid_is_the_serving_process`
  and `tokio_client::tests::tokio_client_server_pid_is_the_serving_process` (the server's pid read on the SQOS-adopted
  client end, `GetNamedPipeServerProcessId`), `viola-mcp::handlers handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`,
  `hook_fail_open …::case_12_impostor` and `…::case_5_question_impostor`, the twelve `security_negative_cli_verb_…` rows.
- `test (ubuntu-latest)` (job 121481615549): 39.
- `test (macos-latest)` (job 121481615684): 10 — the decision table and the two no-peer-pid cases; the impostor rows
  compile out there (the recorded gap).

The pass is complete on green; no further fix round.

## 7. Second fix round — the wrap's P7.1 red folded (implement re-entry, `inputs#I13`, `inputs#I14`)
Fired by the implementer on the operator's word at the re-entry (`inputs#I14`: "… then its gates, the operator pass and
the CI read. Leave the wrap's uncommitted P1-P6 writes exactly as they are …" — the operator, live, 2026-10-26 11:36Z).
The fix: the fake agent ignores SIGHUP from its exit's first step (`chunks/…/evidence/fake-agent-exit-hangup.md`).

The macOS-target lint (`CARGO_TARGET_DIR=target/maccheck cargo clippy --workspace --all-targets --features fake-agent
--target aarch64-apple-darwin -- -D warnings`): exit 0, `Checking viola v0.1.0`. The Windows-target lint and host
clippy: the implement block run's entries 4 and 3, green; no source changed after them.

1. The native gate (plan entry 21) on the uncommitted tree:
```
 21 probe       green · exit 0 · 90.64s · 399447 B → 21.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
2. Hygiene (plan entry 22, fired as written), exit 0, read twice, the second right before the commit:
```
hygiene: clean — read 488 (runs 477 · evidence 2 · inputs 9) · trails 16 not read · copies 8 not read by P1 — 0 host paths kept · binary 0 not read by P1
hygiene: clean — read 488 (runs 477 · evidence 2 · inputs 9) · trails 16 not read · copies 8 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
3. The fix commit — only the two fix files staged (`git add -- src/bin/viola-fake-agent.rs tests/cli_fake_agent.rs`,
   nothing staged before it), so the wrap's uncommitted P1–P6 writes stay in the tree as they were, unstaged; exit 0:
   `7948850acd4b835b23f0504ff77a53482c4d14b0` · `fix(2026-10-26-server-verification-before-any-frame): the fake agent ignores SIGHUP from its exit's first step, closing the cut-short fake-agent profile red`
   (parent `775641fa`, never an amend).
4. The push (plan entry 23), from a clean worktree of the fix commit (`inputs#I15`, the overseer's founder-delegated
   answer: the entry runs as written with its guard; the main tree is never touched). A detached worktree leaves `git
   push origin HEAD` no branch to name, so the worktree took the branch itself: `git worktree add --force
   target/push-wt-7948850a build/viola-0.1.0` (HEAD `7948850a`, `status` empty; nothing committed there). Dry-run
   `delta none · uncommitted 0`, then:
   `python -X utf8 {tools_dir}/gate.py run --plan target/push-wt-7948850a/viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --operator 23 --project-root target/push-wt-7948850a`
```
operator entry 23 · history tripwire: git
 23 probe       green · exit 0 · 2.27s · 92 B → 23.log · history moved: refs/remotes/origin/HEAD 775641fa→7948850a; refs/remotes/origin/build/viola-0.1.0 775641fa→7948850a · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
   Its log: `To https://github.com/Turbolet85/viola.git` / `775641fa..7948850a  HEAD -> build/viola-0.1.0`. No force
   push. The worktree was removed (`git worktree remove target/push-wt-7948850a`, exit 0; `git worktree list` names the
   repository alone, at `7948850a`).
5. The CI read (plan entry 24, fired as written, from the main tree, HEAD `7948850a`) — exit 0:
```
repo Turbolet85/viola (the push remote `origin`) · polled 28× over 840 s
7948850acd4b verdict: green · checks 15/15 · wall 841 s · runs ci#40318429755 completed/success
runs: ci#40318429755 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```
The run itself (`gh api repos/Turbolet85/viola/actions/runs/40318429755`): `run_attempt` 1, `head_sha`
`7948850acd4b835b23f0504ff77a53482c4d14b0`, event `push`, conclusion `success`. `test (ubuntu-latest)` (job
121556826215): `PASS [   1.011s] (  84/3056) viola::cli_fake_agent fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits`,
`FAIL [` lines 0. The case is Linux-only (the disposition is read from `/proc`): it compiles out on `windows-2025` and
`macos-latest`, whose test jobs read `success`.

The second fix round is complete on green.
