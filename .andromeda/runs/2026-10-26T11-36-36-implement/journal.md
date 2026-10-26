# Implement run journal — 2026-10-26-server-verification-before-any-frame (re-entry)

The re-entry for the wrap's P7.1 red (the cut-short `viola-fake-agent` profile, `inputs#I13`), on the operator's word at
invocation (`inputs#I14`): the fix, its gates, the operator pass and the CI read. Long runs are journalled before each
wait. The wrap's uncommitted P1–P6 writes are left as they are.

- 2026-10-26 11:46Z · P1 done: `src/bin/viola-fake-agent.rs` (`ignore_hangup`, SIGHUP ignored from the exit's first
  step; a session leader cannot leave its group) and `tests/cli_fake_agent.rs`
  (`fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits`), both recorded in-intent on `inputs#I13`;
  red-before-green and a 40-run stress in `chunks/…/evidence/fake-agent-exit-hangup.md`.
- 2026-10-26 11:47Z · P2 block run 1 — `gate.py run --plan viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --run-dir .andromeda/runs/2026-10-26T11-36-36-implement --marker 2026-10-26-server-verification-before-any-frame`, backgrounded. 21 non-leg entries; the longest bounds entry 20 `run --perf` (3600 s) and entry 21 `pre-push` (7200 s), read at 16.95 s and 87.61 s at P5 and 88.21 s at the first implement run. The guard (entry 13) is expected to read `outside 3 files` (the earlier `tests/contract_fake_agent_drift.rs` and this fix's two files, each recorded).
  Result (2026-10-26 11:48Z): `entries 24 · green 19 · red 1 (13) · recorded 1 · timeout 0 · not-run 3` — 13 the
  guard's `outside 3 files`, each recorded in-intent; 21 `pre-push` green at 91.35 s; 20 `run --perf` 1.86 s (the
  release build cached), hook maxima 2.2–2.5 ms.
- 2026-10-26 11:49Z · two profile censuses on the instrumented build entry 21 left
  (`target/llvm-cov-target/debug/viola-fake-agent` written 11:46:45Z, after the fix), backgrounded one after the other:
  `bash scripts/profraw-census.sh --case chaos_wait_vanish chaos_wait_parked_wrapper_killed_exits_21 200 8 svf-exit-1`
  (the Epoch 5 kill-shaped case: a killed wrapper hangs up the fake agent's terminal), then `bash
  scripts/profraw-census.sh --case cli_fake_agent fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits 40 4
  svf-exit-2` (the hangup inside the exit). `/tmp` 25G avail, 22 % before.
  Result: `runs 200 · passed 200 · profiles 801 · refused 0` and `runs 40 · passed 40 · profiles 120 · refused 0`, both
  exit 0 (`chunks/…/evidence/fake-agent-exit-hangup.md`).
- 2026-10-26 11:50Z · P3 recorded from P2; P4 `scope: clean`, census none of this run's; outcome written.
- 2026-10-26 11:51Z · operator pass (`.claude/rules/ci.md` §The operator pass, `inputs#I14`): the macOS-target lint
  (`CARGO_TARGET_DIR=target/maccheck cargo clippy --workspace --all-targets --features fake-agent --target
  aarch64-apple-darwin -- -D warnings`) exit 0; step 1, the native gate on the uncommitted tree — `python -X utf8
  {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --entry 21`
  (no run dir), backgrounded.
  Step 1 green (90.64 s); step 2 `hygiene: clean`, re-read clean right before the commit.
- 2026-10-26 11:53Z · fix commit `7948850a` (the two fix files only; the wrap's writes left unstaged). Push entry 23
  from a clean worktree (`inputs#I15`): a detached one would leave `git push origin HEAD` no branch, so the worktree
  was `git worktree add --force target/push-wt-7948850a build/viola-0.1.0` (the branch also checked out here; nothing
  committed there); dry-run `uncommitted 0`, then fired: green, `history moved: refs/remotes/origin/build/viola-0.1.0
  775641fa→7948850a`, log `To …viola.git  775641fa..7948850a  HEAD -> build/viola-0.1.0`. Worktree removed (`git
  worktree remove`, exit 0; `git worktree list` names the repository alone).
- 2026-10-26 11:54Z · CI read, entry 24 fired as written, backgrounded: `python -X utf8
  ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400` (HEAD =
  `7948850acd4b835b23f0504ff77a53482c4d14b0`; the last green wall 854 s, ci#40295857215).
  Result: `7948850acd4b verdict: green · checks 15/15 · wall 841 s · runs ci#40318429755 completed/success`,
  `run_attempt` 1; the new case PASS on `test (ubuntu-latest)`, 0 FAIL. Operator pass complete; next the wrap's resume.
