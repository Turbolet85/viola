# Red and green readings — 2026-10-10-budget-governor (plan step 14)

Taken by /implement on the Linux dev host, on the uncommitted tree over `396c108`, with rustc 1.98.1 and
cargo-nextest 0.9.146, every run under the nextest `mutants` profile (its 10 s kill). Each time is `date -u`, read
right before and right after the run. Each neutralising edit was made inside the guarded function itself, read back
by `grep` before its run, and reverted; after the last reading a `grep` for the three neutralised forms counts 0 in
`src/run/send.rs`, `src/cmd/hook/statusline.rs` and `crates/viola-state/src/budget.rs`, and `src/cmd/revive.rs`
reads the check ahead of `open_wrapper_log`.

A witness for a fix is read red with the fix reverted in place: the two statements put back in the order they
stand in at `396c108`. No worktree at `396c108` was built, so "the tree before its fix" here is that order on this
tree, not the commit itself.

## (a) `viola revive`: the check before the log (step 10)

Reverted form, in `revive` (`src/cmd/revive.rs`): `open_wrapper_log(home, &args.name)?` moved back above the
`trusted` check.

Command, both readings:
`cargo nextest run -p viola --features fake-agent --profile mutants --no-fail-fast -E 'binary(cli_revive) & test(/another_user_could_write/)'`

- **Red**, 2026-10-10 21:53:29Z to 21:53:33Z, exit 100:
  - `FAIL [   3.502s] (2/3) viola::cli_revive revive_over_a_home_another_user_could_write_is_refused_and_leaves_it_as_found`
    — `assertion left == right failed: the revive changed the home's mode`, left 448 (0700), right 504 (0770)
  - `FAIL [   3.506s] (3/3) viola::cli_revive revive_refuses_state_another_user_could_write`
    — `assertion left == right failed: a refused check logged`, left 8, right 7
  - `PASS [   3.501s] (1/3) viola::cli_revive revive_list_refuses_state_another_user_could_write`
  - `Summary [   3.506s] 3 tests run: 1 passed, 2 failed, 9 skipped`
- **Green**, 2026-10-10 21:53:43Z to 21:53:47Z, exit 0: `Summary [   3.507s] 3 tests run: 3 passed, 9 skipped`

An earlier red of the same reverted form, with the witness in its first shape, 2026-10-10 21:52:32Z to 21:52:49Z:
the witness read `TIMEOUT [  10.003s]` and the restated case `FAIL` (8 lines against 7). The first shape waited
for the revive to exit and only then read the home's mode. On the reverted order the revive passes its check,
starts the fake agent as its child and runs on as a wrapper, so it did not exit: the plan's step 14 expected that
child to end at once, and it did not. The runner's 10 s kill came before the case's own 7 s wait could fail it.
The witness was reshaped to read the home's mode while the revive still runs, so the reverted order fails it by
its own assertion in about 3.5 s. After that timed-out run and after the red above, a scan of every process by
its executable path found none under the repository's `target/` or the test-home base: the revive the case started
was ended by the runner's kill in the first run and by the case's own `Running` guard in the second.

## (b) `viola hook statusline`: nothing after a refused check (step 11)

Reverted form, in `statusline` (`src/cmd/hook/statusline.rs`): the check's result taken first, the role file
opened whatever it said (`viola_obs_init`), and only then the return for a refused instance.

Command, both readings:
`cargo nextest run -p viola --features fake-agent --profile mutants --no-fail-fast -E 'binary(hook_statusline) & test(/however_often/)'`

- **Red**, 2026-10-10 21:53:59Z to 21:54:01Z, exit 100:
  - `FAIL [   0.187s] (1/1) viola::hook_statusline hook_statusline_a_home_another_user_can_write_runs_nothing_however_often`
    — `assertion left == right failed: after reading 12`, left 448 (0700), right 504 (0770): the first refused
    invocation narrowed the home
  - `Summary [   0.188s] 1 test run: 0 passed, 1 failed, 10 skipped`
- **Green**, 2026-10-10 21:54:10Z to 21:54:11Z, exit 0: `Summary [   0.193s] 1 test run: 1 passed, 10 skipped`

## (c) The `send` rung (step 7)

Neutralised form, in `send` (`src/run/send.rs`): the gate still evaluated, its verdict dropped (`let _ = crossed;`
in place of the `budget_paused` return).

Command, both readings:
`cargo nextest run -p viola --features fake-agent --profile mutants --no-fail-fast -E 'test(/send_budget_paused_/) | (binary(cli_budget) & test(=path6_a_reading_over_the_threshold_refuses_the_send))'`

- **Red**, 2026-10-10 21:54:19Z to 21:54:25Z, exit 100: `Summary [   3.839s] 14 tests run: 6 passed, 8 failed, 1124 skipped`.
  Failed: `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` (its four cases),
  `send_budget_paused_comes_before_turn_running` (its two cases),
  `send_budget_paused_is_not_read_again_after_the_gate_wait`, and `viola::cli_budget
  path6_a_reading_over_the_threshold_refuses_the_send`. Passed, as they must with or without the refusal:
  `send_budget_paused_comes_after_human_typing`, `send_budget_paused_comes_after_the_text_refusals` (two cases),
  `send_budget_paused_fields_are_on_no_other_refusal`, `send_budget_paused_does_not_refuse_under_the_thresholds`,
  `send_budget_paused_whose_record_cannot_land_is_internal`.
- **Green**, 2026-10-10 21:54:32Z to 21:54:37Z, exit 0: `Summary [   3.502s] 14 tests run: 14 passed, 1124 skipped`

## (d) `read_budget`'s strict-modes check (step 2)

Neutralised form, in `read_budget` (`crates/viola-state/src/budget.rs`): both `check_path` results dropped
(`let _ = check_path(home); let _ = check_path(&path);`).

Command, both readings:
`cargo nextest run -p viola-state --profile mutants --no-fail-fast -E 'test(/budget_read_/)'`

- **Red**, 2026-10-10 21:54:44Z to 21:54:47Z, exit 100: `Summary [   0.230s] 8 tests run: 6 passed, 2 failed, 187 skipped`.
  Failed: `budget_read_under_a_path_another_user_could_write_is_nothing` (a reading came back from a 0770 home),
  and `budget_read_of_an_absent_file_is_nothing_and_creates_nothing` (left 1, right 0: the lock sibling was
  created in a home with no `budget.json`).
- **Green**, 2026-10-10 21:54:55Z to 21:54:56Z, exit 0: `Summary [   0.219s] 8 tests run: 8 passed, 187 skipped`

## Not read

- No reading on Windows or macOS: the two witnesses are `cfg(unix)`, and this host is Linux.
- No reading at the commit `396c108` itself (above).
- No mutation run (the operator's direction).
