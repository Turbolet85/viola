@account n=1 entity=e2e-test-strategy#1.1 v=proposed
why: it holds dim `stale` at the unit tier only with its outer-PTY witness a CARRY on this entry, and the witness landed as `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` on the Unix legs
@account n=2 entity=e2e-test-strategy#2.2 v=not_touched
why: the report's changes to `tests/cli_instance_state.rs` and `tests/run_cli.rs` are the `Frozen` move, the `run_child_ends_when_its_wrapper_is_terminated` gate and reason lines, so the Path 1 cases it names stand as stated
@account n=3 entity=e2e-test-strategy#2.3 v=not_touched
why: it names the `ci` profile's 20 s kill line with no count, and the line still holds `path1_run_start` with `run_cli` only added after it
@account n=4 entity=e2e-test-strategy#4.4 v=proposed
why: its killed-wrapper cleanup by pid and start time is what `run_child_ends_when_its_wrapper_is_terminated` now asserts on every OS (F2)
@account n=5 entity=e2e-test-strategy#7.1 v=not_touched
why: the `tests/cli_instance_state.rs` case it names is not among the report's changes to that file, which are the `Frozen` move and three reason lines
@account n=6 entity=e2e-test-strategy#11.1 v=not_touched
why: `tests/cli_list.rs` gained one reason line at 618 on another case, and the E5 cases it names stand unchanged
@account n=7 entity=e2e-test-strategy#14.1 v=not_touched
why: `tests/cross_exit_causes.rs:12`'s top comment now names only the guard's new home, and every cause and case the entity names still stands in the binary it names
@account n=8 entity=e2e-test-strategy#16.1 v=proposed
why: `tests/cross_list_rows.rs` now also holds the stale-row dim witness under the outer PTY, beyond escaping and column alignment (F1)
@account n=9 entity=e2e-test-strategy#17.1 v=proposed
why: its `Frozen` mention names no file and stays true, while the hangup case's new reader and Linux-only reach belongs beside its process-fault items (F3)
@account n=10 entity=e2e-test-strategy#19.1 v=proposed
why: the contract suite gained `tests/contract_os_gated_tests.rs`, the OS-gated census, and its 52 + 7 count (F4, F5)
@account n=11 entity=test-data-fixtures#1.12 v=not_touched
why: the literals it names in `tests/cli_list.rs` and `tests/cross_exit_causes.rs` are not in the report's changes, which add reason lines and a top comment only
@account n=12 entity=test-data-fixtures#2.3 v=not_touched
why: `tests/cli_verify.rs` gained one reason line at 428 on `verify_a_batch_script_refuses`, and the catch-site case and per-OS oracle it states stand
@account n=13 entity=mocking-stubbing-discipline#2.4 v=proposed
why: it names `tests/cli_instance_state.rs` as the `Frozen` guard's home, and the guard moved whole to `tests/support/frozen.rs`
@account n=14 entity=ci-integration#1.12 v=proposed
why: the kill line's 27 binaries, the operator-pass JUnit reader and the per-OS JUnit reading are new CI facts that belong beside it (F7, F8, F9)
