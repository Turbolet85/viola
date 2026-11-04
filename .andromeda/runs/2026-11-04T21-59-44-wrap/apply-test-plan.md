@apply_row n=1 ref_file=fanout-test-plan.md ref_n=2 v=refused
why: fct-hgewdmj2 stands in the key file doc:test-plan/contracts/5-command-implementation, which is not doc:test-plan's body and not among this batch's read addresses, so its rev and its text are unread and no in-line change can be written against it; the report also gives the root waits' count (32, by `grep -rn 'Instant::now() + WITHIN' tests/ | wc -l`) but no file count, so the `as:` direction's «24 files» is not in the report
@apply_row n=2 ref_file=fanout-test-plan--cascade-g1.md ref_n=1 v=applied edit=E1
@apply_row n=3 ref_file=fanout-test-plan--cascade-g1.md ref_n=2 v=applied edit=E2
@apply_row n=4 ref_file=fanout-test-plan--cascade-g1.md ref_n=3 v=applied edit=E3
@apply_row n=5 ref_file=fanout-test-plan--cascade-g1.md ref_n=4 v=applied edit=E4
@apply_row n=6 ref_file=fanout-test-plan--cascade-g2.md ref_n=1 v=applied edit=E5
@apply_row n=7 ref_file=fanout-test-plan--cascade-g2.md ref_n=2 v=applied edit=E6
@apply_row n=8 ref_file=fanout-test-plan--cascade-g3.md ref_n=1 v=applied edit=E7
@apply_row n=9 ref_file=fanout-test-plan--cascade-g3.md ref_n=2 v=applied edit=E8
@apply_row n=10 ref_file=fanout-test-plan--cascade-g3.md ref_n=3 v=applied edit=E9
@apply_row n=11 ref_file=fanout-test-plan--cascade-g3.md ref_n=4 v=applied edit=E10
@apply_row n=12 ref_file=fanout-test-plan--cascade-g3.md ref_n=5 v=applied edit=E11
@apply_row n=13 ref_file=fanout-test-plan--cascade-g3.md ref_n=6 v=applied edit=E12
@apply_row n=14 ref_file=fanout-test-plan--cascade-g3.md ref_n=7 v=applied edit=E13
@apply_row n=15 ref_file=fanout-test-plan--cascade-g3.md ref_n=8 v=applied edit=E14
@apply_row n=16 ref_file=fanout-test-plan--cascade-g3.md ref_n=9 v=applied edit=E15
@apply_row n=17 ref_file=fanout-test-plan--cascade-g3.md ref_n=10 v=applied edit=E16
@apply_row n=18 ref_file=fanout-test-plan.md ref_n=1 v=applied edit=E17
@fact id=fct-fnukifof expect_rev=1
old: not by screen inspection of the child agent.
new: not by screen inspection of the child agent. The dim `stale` row is witnessed under the outer PTY on Unix by `tests/cross_list_rows.rs` `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`TERM=xterm-256color`, the wrapper frozen through `Frozen` and its `heartbeat` back-dated 60 s): its escape sequences are exactly `[DIM, BOLD, RESET, DIM, RESET]`, with `stale` shown (§6 `list` row integrity).

@fact id=fct-hckuzazj expect_rev=1
old: `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`
new: `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs,frozen.rs}`
old: No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.
new: `frozen.rs` holds `Frozen`, the one freeze guard (`attach(pid)` and `Drop`): on Unix `kill -STOP`, `ps -o stat=` polled until `T` within `WITHIN`, and `kill -CONT` on drop; on Windows a debugger attach, detached on drop. No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard, the freeze guard or the result struct.

@fact after=fct-hckuzazj as=n0
- **Root `contract_` binaries:** `tests/contract_os_gated_tests.rs` is a root `contract_` binary of the sync suites, auto-discovered with no `[[test]]` table; the E2E layer's regex selects it, so it runs under `run --e2e`, in suite `nextest-e2e`.

@fact id=fct-gldxhydz expect_rev=1
old: | multi-os-compat | CI matrix | §9 Matrix builds |
new: | multi-os-compat | CI matrix + contract | §9 Matrix builds; §6 Contract suite (root `contract_os_gated_tests.rs`: every OS-gated test function and gated file under `tests/` and `crates/*/tests/` a `GATES` row with its reason, an unnamed or stale gate failing the run) |

@fact id=fct-5qxwlgko expect_rev=1
old: (`crates/viola-e2e/tests/harness_lifecycle.rs`) on `ubuntu-latest`,
new: (`crates/viola-e2e/tests/harness_lifecycle.rs`, `#[cfg(unix)]`) on both Unix runners, `ubuntu-latest` and `macos-latest`,

@fact id=fct-lzdm6e7w expect_rev=1
old: `Frozen` (`tests/cli_instance_state.rs`) is a guard on Windows and on Unix, a `SIGSTOP` on Unix; `run_refuses_a_stale_name` and `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` stand on it.
new: `Frozen` (`tests/support/frozen.rs`, declared by `pub mod frozen;` in `tests/support/mod.rs`, its only home) is a guard on Windows and on Unix, a `SIGSTOP` on Unix; the cases that stand on it are `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` (every OS), `run_refuses_a_stale_name` (Unix) and `run_refuses_a_frozen_wrapper_as_stale_on_windows` (Windows) in `tests/cli_instance_state.rs`, and `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (Unix) in `tests/cross_list_rows.rs`.

@fact id=fct-4m5t4osf expect_rev=1
old: dim `stale` at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), its outer-PTY witness a CARRY on "Linux and macOS parity".
new: dim `stale` at the unit tier (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`) and under the outer PTY on `ubuntu-latest` and `macos-latest` (`cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, `tests/cross_list_rows.rs`, `cfg(unix)`; `list` row integrity).

@fact after=fct-hdfefjry as=n1
- **The stale-row dim witness:** `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` (`cfg(unix)`, on `booted_wrapper`), `v1-40`'s integration-tier witness, freezes the wrapper through `Frozen`, back-dates its `heartbeat` 60 s and runs `viola list` under the outer PTY with `TERM=xterm-256color`: its stream's escape sequences are exactly `[DIM, BOLD, RESET, DIM, RESET]` (the stale row dim from first byte to last, NAME dim and bold, no SGR on BAY, caption or separator) and `stale` is shown; green on `ubuntu-latest` and `macos-latest` (as measured at ci#41918694243).

@fact after=fct-sefkfuph as=n2
- **A terminated wrapper's child:** `tests/run_cli.rs` `run_child_ends_when_its_wrapper_is_terminated`, on every OS, terminates the wrapper (Windows `OpenProcess` + `TerminateProcess`; Unix `kill -KILL <wrapper pid>`, its exit asserted) and asserts that the child, its pid and start time read from `snapshot_data`, is gone within `WITHIN`.

@fact after=fct-6whu4pj5 as=n3
- **A hangup landing while the fake agent exits** (`tests/cli_fake_agent.rs` `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits`, `cfg(target_os = "linux")`): the case reads the agent's SIGHUP disposition through `ps -o sigignore= -p <pid>` (`ignores_sighup`), a failed read carrying `ps`'s exit and both streams; it does not run on `macos-latest`, whose `ps` has no keyword that prints the ignored-signal mask (as measured at ci#41915621814), a gate set by the overseer's technical answer (founder-delegated) to the implement dialog, so the fake agent's SIGHUP-ignoring exit, shared Unix code, is witnessed on Linux only (§7 Fake agent).

@fact after=fct-sn6e6lxw as=n4
- **The census of OS-gated tests** (`tests/contract_os_gated_tests.rs`, a `contract_` binary, `run --e2e`, its file carrying the `andromeda:walks-tree` token): its `GATES` table holds a `file`, `test`, `gate` and `reason` per row, and the census walks every `.rs` directly under `tests/` and in each `crates/*/tests/`, counting a leading `#![cfg(…)]` and a test function whose contiguous attribute run holds `#[test]`, `#[rstest]` or `#[tokio::test]` and a `#[cfg(…)]` or `#[cfg_attr(…)]` whose predicate names `windows`, `unix` or `target_os` outside its string values; `faults` reads both ways, an unnamed gate and a row that no longer stands. Its tests are `os_gated_tests_are_each_named_with_their_reason`, `gates_table_rows_are_unique` and the self-check `census_finds_an_unnamed_gate_and_passes_a_named_one` over eight planted cases.

@fact after=@n4 as=n5
- **The census count:** the census reads 52 OS-gated test functions and 7 gated files, 59 `GATES` rows (rule: the rows of `GATES` in `tests/contract_os_gated_tests.rs`), its rule taking the `cfg_attr` case rows (`tests/hook_fail_open.rs`'s two functions).

@fact id=fct-63urz52n expect_rev=1
old: under the `Frozen` guard (`tests/cli_instance_state.rs`; on Unix a `SIGSTOP`)
new: under the `Frozen` guard (`tests/support/frozen.rs`, its sole home, shared through `tests/support/mod.rs`; on Unix a `SIGSTOP`)

@fact after=fct-k7t43itf as=n6
- **The `ci` kill line:** the `ci` profile's stamped-home override filter (`.config/nextest.toml:24`) is the 20 s kill (`slow-timeout = { period = "10s", terminate-after = 2 }`) over 27 binaries, the count the `binary(…)` names on that filter line (the list in §3 Bootstrap phases, `test-runner-install`).

@fact after=@n6 as=n7
- **The per-OS JUnit reader:** `scripts/junit-os-sets.py --run <id>` (python 3 standard library and the host's `gh`) is the operator pass's per-OS JUnit reading, never a CI step: it downloads `junit-windows-2025`, `junit-macos-latest` and `junit-ubuntu-latest` with `gh run download` into a temporary directory removed on exit, prints `{os}: {n} cases · {f} failed · {s} skipped` per OS, then each set's count and its sorted `classname::name` lines for `ubuntu-not-macos`, `macos-not-ubuntu`, `windows-not-ubuntu` and `ubuntu-not-windows`, and exits 0, 1 (a report missing or unparsable, named) or 2 (usage); it reads no environment variable and writes nothing in the tree.

@fact after=@n7 as=n8
- **The per-OS JUnit reading,** as measured at ci#41918694243 (`cbdd3874`, `verdict: green · checks 15/15`): `windows-2025` 3386 · `macos-latest` 3412 · `ubuntu-latest` 3458 cases, 0 failed and 0 skipped; `ubuntu-not-macos` 48 (43 of the macOS no-peer-pid carve-out and 5 Linux-only, each by a named reason) · `macos-not-ubuntu` 2 · `windows-not-ubuntu` 88 · `ubuntu-not-windows` 160.

@fact id=fct-2j6rht5l expect_rev=1
old: then the twenty-six verify-driven binaries
new: then the twenty-seven verify-driven binaries
old: path1_run_start security_negatives_socket_dir
new: path1_run_start run_cli security_negatives_socket_dir
old: was missing from this list; `hook_statusline` joined
new: was missing from this list; `run_cli` (`tests/run_cli.rs`) stands on it too; `hook_statusline` joined
@pattern id=tp-01
fixed: `harness_lifecycle.rs`) on `ubuntu-latest`
@pattern id=tp-02
fixed: `Frozen` (`tests/cli_instance_state.rs`) is a guard
@pattern id=tp-03 ignore_case=true
fixed: its outer-PTY witness a CARRY on
@pattern id=tp-04
fixed: under the `Frozen` guard (`tests/cli_instance_state.rs`
@pattern id=tp-05 ignore_case=true
fixed: twenty-six verify-driven binaries
