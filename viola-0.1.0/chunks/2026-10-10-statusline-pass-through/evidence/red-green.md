# Red and green readings — 2026-10-10-statusline-pass-through

Every reading below was taken at /implement on the Linux dev host, on the uncommitted tree over HEAD `4d77eac`,
with `cargo nextest run` and the filter named. Times are UTC, read from `date -u` beside the run.

## 1. The property, red once on a stub (plan step 2)

- Subject: `statusline_prop_resets_at_never_fails` (`crates/viola-agent-claude/src/statusline.rs`), `cases: 512`.
- Stub: inside `resets_at`, a number that parses to an instant returned the known value `stub: not a time` in
  place of its RFC 3339 form. Nothing else was changed.
- First reading on the stub, with the strategy as first written (`any::<i64>()` as the only whole-number leaf):
  **green**, 1 passed. The property did not see the stub: an arbitrary 64-bit count of seconds almost never
  falls inside the range a date holds, so the epoch path was not reached. The strategy gained one leaf, whole
  seconds from 70 000 000 000 before the epoch to 300 000 000 000 after it (a little before year 0 to a little
  past year 9999).
- Second reading on the stub, with that leaf: **red**, read at 2026-10-10T19:01:30Z. `assertion failed:
  round_trips(&got.five_hour)`, minimal failing input `resets_at = Number(0), used_percentage = Null`. proptest
  wrote `crates/viola-agent-claude/proptest-regressions/statusline.txt` with that one seed.
- Stub removed (`grep -n stub` over the file: no hit), then the crate's statusline and budget cases: **green**,
  107 passed across `viola-core`, `viola-agent-claude` and `viola-state` (filter `test(/statusline|budget/)`).

## 2. The instance check, removed once (plan step 10, case 5)

- Subject: `hook_statusline_a_home_another_user_can_write_runs_nothing` (`tests/hook_statusline.rs`): a home
  set to mode 0770 after a first good run, then one more `viola hook statusline` with another payload.
- Guard: in `src/cmd/hook/statusline.rs`, `let trusted = check_instance(&instance.home, &instance.dir).is_ok();`.
- Guard removed inside the arm (the line read `… .is_ok() || true;`, confirmed by `grep -n 'let trusted'` before
  the run), started 2026-10-10T19:08:40Z: **red**. The case's stdout assertion read `viola-fake-statusline` and
  a newline where it expects nothing: the user's command ran from the group-writable home.
- Guard restored (the same `grep` reads the original line), read at 2026-10-10T19:09:06Z: **green**, 1 passed.

## 3. A second control: the check placed after the log opens

The plan's step 7 lists obs init (its item 3) ahead of the instance check (its item 4). The check was moved
there for one run, unchanged otherwise.

- Started 2026-10-10T19:08:55Z: **red**, the same assertion and the same bytes as in reading 2.
- Cause, read in the source: `viola_obs_init` opens the role file through `open_role_file`, which calls
  `create_private_dir` on the home, and that sets an existing home to mode 0700
  (`crates/viola-state/src/fs.rs`, `create_private_dir`, its last line). So a home at 0770 is 0700 by the time a
  check placed after it reads the mode, and the check passes.
- The arm therefore runs the check before the log opens, and writes the same lines in the same order
  (`hook-invoked`, then `hook-decision` with `strict-modes-failed`). The check-first order is what readings 2's
  green and the nine cases of `binary(hook_statusline)` were taken on.

## 4. Counts the plan asked to be read after step 10

- Root waits, `grep -rn -F 'Instant::now() + WITHIN' tests | wc -l`: 26 on this tree, 26 at HEAD (`git grep`
  over `HEAD -- tests`). The new root file adds none of its own: its waits are the shared helpers'
  (`Running::finish`, `Wrapper::boot`, `fake::wait_for`).
- `grep -rn -E 'InstanceSnapshot \{' src crates tests --include=*.rs | wc -l`: 20 lines (18 at the plan's
  count). The two added lines are one struct literal in the unit tests of `src/cmd/hook/statusline.rs` and one
  signature line in the tests of `src/cmd/run.rs`. Struct literals: 12, where the plan forecast 11.
- `ls fuzz/corpus/hook_stdin | wc -l`: 12.
