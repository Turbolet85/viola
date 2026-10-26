# Basis loop — entry 21's coverage-merge red on the parent tree

The operator's direction (inputs#I12, the overseer's founder-delegated answer, 2026-10-26 11:18Z): measure the basis.

- Tree: `6bdfdda8` (the parent of the oldest pre-CI commit `4aa9def9`), checked out by `git worktree add` into
  `target/basis-wt-6bdfdda8` (gitignored, on the btrfs home; `/home` 357G free, `/tmp` 25G free at 2026-10-26T11:19:17Z).
- Command: `bash scripts/agent-run.sh pre-push` inside that worktree, up to 8 runs, stopping at the first run whose
  coverage suite fails with `llvm-cov-exit-1` (a refused profile); each run's final document and its refused-profile
  lines kept in the session scratchpad, their facts copied here at the end.
- Reading: a refusal = the basis (the same red on a tree without this chunk's edits); the owner then is a WATCH on
  «Explicit session links». 8 greens = no basis; reported back, the wrap not resumed.
- Started: 2026-10-26T11:19Z. The worktree is removed with `git worktree remove` when done.

## Runs
Each run `exit 0`, its final document `{"v":1,"cmd":"pre-push","ok":true,"stage":"linux-tests",…,"gate":{"ok":true,"breaches":[]}}`,
the coverage suite `failed 0`, no `invalid instrumentation profile data` line:

| run | start → end (UTC) | nextest summary |
|---|---|---|
| 1 | 11:19:49 → 11:22:00 | `3006 tests run: 3006 passed (21 slow), 0 skipped` |
| 2 | 11:22:00 → 11:23:33 | `3006 tests run: 3006 passed (5 slow), 0 skipped` |
| 3 | 11:23:33 → 11:25:09 | `3006 tests run: 3006 passed (6 slow), 0 skipped` |
| 4 | 11:25:09 → 11:26:43 | `3006 tests run: 3006 passed (5 slow), 0 skipped` |
| 5 | 11:26:43 → 11:28:16 | `3006 tests run: 3006 passed (5 slow), 0 skipped` |
| 6 | 11:28:16 → 11:29:49 | `3006 tests run: 3006 passed (5 slow), 0 skipped` |
| 7 | 11:29:49 → 11:31:24 | `3006 tests run: 3006 passed (5 slow), 0 skipped` |
| 8 | 11:31:24 → 11:32:59 | `3006 tests run: 3006 passed (5 slow), 0 skipped` |

The worktree built and ran in its own `target/` (its `llvm-cov-target/` held 12 504 profiles after run 8). Result: **8 of 8
green, no coverage-merge refusal — no basis.** The worktree was removed with `git worktree remove` (exit 0); `git worktree
list` names the repository alone.

## The census before it (this tree, `scripts/profraw-census.sh --case`)
- `security_negatives_server_identity` `…::case_01_send_json` 200 runs · 400 profiles · refused 0; `hook_fail_open`
  `…::case_12_impostor` and `…::case_5_question_impostor` 200 runs each · 400 profiles each · refused 0; `handlers`
  impostor case 200 runs · 200 profiles · refused 0. Two profiles a run (the test process and its one `viola`): the held
  fake agent, killed in its sleep, writes none.
- The cut-short profile's neighbours by pid and write time were `cli_instance_state` test processes (build id
  `561fcbbc…`); its four wrapper-kill tests (`run_takes_over_a_gone_name_and_appends`, `run_refuses_a_live_name`,
  `path1_start_writes_state_before_the_spawn`, `path1_session_start_is_record_three_through_the_absolute_hook`), 100 runs
  each: 20 500 profiles, refused 0.
- The writer stays unnamed: not this chunk's holders by measurement; one cut-short profile of `viola-fake-agent` in the
  wrap's pre-push (1 of 12 583), no reproduction in 1 600 census runs or 8 parent pre-push runs.

## Reading
Per the direction: 8 greens → no basis, reported back, the wrap not resumed. The red on the chunk's tree stands open (one
reproduction in 6 pre-push runs of that tree; 0 in 8 of the parent's).
