# Remove-the-guard readings

Written 2026-10-31T17:31:11Z, nextest `mutants` profile (its 10 s kill), `-E 'binary(cross_exit_causes)'`, in the working tree; each plant restored before the next.
Tree diff sha before and after the plants: `23f1b7b37c90` -> `23f1b7b37c90`.

| guard | file | exit | reading | failed cases |
|---|---|---|---|---|
| without_endpoint forced to InstanceDead | `crates/viola-state/src/liveness.rs` | 100 | Summary [   1.227s] 39 tests run: 37 passed, 2 failed, 0 skipped | a_name_never_started_is_unwrapped_name::case_1_json; a_name_never_started_is_unwrapped_name::case_2_human |
| Lost mapped to Dead | `src/cmd/client.rs` | 100 | Summary [   1.267s] 39 tests run: 37 passed, 2 failed, 0 skipped | a_wrapper_killed_under_a_parked_wait_is_endpoint_lost::case_1_json; a_wrapper_killed_under_a_parked_wait_is_endpoint_lost::case_2_human |
| json dropped at the start | `src/cmd/revive.rs` | 100 | Summary [   1.367s] 39 tests run: 36 passed, 3 failed, 0 skipped | a_held_endpoint_is_squatted_name::case_1_json; a_revive_with_no_claude_on_its_path_is_internal_error::case_1_json; a_tampered_pinned_copy_is_pinned_hash_mismatch::case_1_json |

Restored: exit 0, `Summary [   1.270s] 39 tests run: 39 passed, 0 skipped`, failed none.
