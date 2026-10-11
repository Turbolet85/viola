# Remove-the-guard readings — 2026-10-11-the-board-viola-list

Plan step 6 (testing.md, 2026-09-25): each guard of the two refusal cases neutralised and each absence case's
behaviour forced on, one at a time, in the working tree. Each run was
`cargo nextest run -p viola --features fake-agent --profile mutants --test cli_list -E '<filter>' --no-fail-fast`
on the Linux dev host; the first run's log closed at 2026-10-11 07:53:11Z and the last one's at
2026-10-11 07:55:06Z (each log's modification time). Before the first edit, a copy and the SHA-256 of
`crates/viola-state/src/board.rs`, `crates/viola-state/src/budget.rs` and `src/cmd/list.rs` were kept in the
session scratchpad. After each run the file was restored from its copy, and all three hashes were read back
`OK` before the next edit (testing.md 2026-09-25: the edit landed was read with a grep before each run).
The green half of every row is the final block run (`evidence/gates.md`, firing 3, entry 5: `binary(cli_list)`).

| # | guard or forced behaviour | edit | case(s) | red reading |
|---|---|---|---|---|
| R1 | the scan's home and `instances/` check | `if false && (check_if_present(home)… )` in `board::scan` | `list_refuses_a_home_another_user_could_write` | red: stderr held the instance pair (`builder's state files can be written by another user`) in place of the home pair, because `check_instance` checks the home again. `list_refuses_an_instance_…` stayed green |
| R2 | the per-instance `check_instance` | `if false && check_instance(home, dir).is_err()` in `board::scan` | `list_refuses_an_instance_another_user_could_write` | red: exit `Some(0)`, expected `Some(21)`. The home case stayed green |
| R3 | the order: no log before the checks | `client::start` moved ahead of `board::scan` in `list` | both `list_refuses_*` cases | red, both: the home case exit `Some(0)` (the log's open narrowed the 0770 home, so the check passed); the instance case `a file was added or changed` (`diagnostics/cli-overseer.ndjson` added) |
| F1 | a log opened with no `VIOLA_NAME` | `own_name().or_else(… "board" …)` in `list` | `list_logs_its_start_and_exit_only_under_a_name`, `list_of_a_home_that_does_not_exist_creates_nothing` | red, both: `cli_logs` read `["cli-board.ndjson"]`, expected `[]`; `assertion failed: !home.exists()` |
| F2 | the `budget.json.lock` sibling opened on a home with no `budget.json` | `open_private_lock` ahead of the file's check in `read_budget_stamped` | `path1_list_shows_a_booted_wrapper` | red: the absence assertion naming `budget.json.lock` |
| F3 | a state file written by the read | `write_snapshot(dir, &snapshot)` in `board::read`'s snapshot arm | `path1_list_shows_a_booted_wrapper` | red: `state_bytes` (`snapshot.json`, `events.ndjson`) not byte-identical after the run |

R3 was taken twice. The first run's instance-case red dumped whole file bytes: `tree()` then held each file's
bytes, and the home holds the pinned `viola` copy. `tree()` was changed to keep each file's path below the
home and its SHA-256, and R3 was taken again. That second reading is the row above.

The other new cases assert behaviour the verb itself brings (a row, a count, a literal line), and no guard
stands in front of it. Their red-before-product is the plan's P5 baseline of entries 4 and 5 (`plan.md`,
`## Test Commands`: no case carried the prefixes, and `cli_list` / `chaos_forward_compat` matched no binary).
