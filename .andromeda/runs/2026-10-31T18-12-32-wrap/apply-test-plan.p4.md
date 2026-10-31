# Apply — test-plan · part 4 of 4

## Verdicts
- fanout-test-plan.md 4 · applied · .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10#1, .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10#2
- cascade-compare-test-plan.md 2 · applied · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:18+#1
- cascade-compare-test-plan.md 4 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:10#1
- cascade-compare-test-plan.md 5 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:30+#1
- cascade-compare-test-plan.md 7 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:19#1
- cascade-compare-test-plan.md 9 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:81+#1

## Edits
- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
  old: «then the twenty-four verify-driven binaries»
  new: «then the twenty-five verify-driven binaries»
- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
  old: «contract_ledger_probes cross_list_rows»
  new: «contract_ledger_probes cross_exit_causes cross_list_rows»
- after: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:18
  anchor: «| Process mocking | mockall 0.15.0 on the `viola-pty` `Pty` trait and the liveness / `Clock` traits; native stub binaries (`viola-fake-agent`) | `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only. `viola-e2e` tests and the harness use the assert_cmd target-dir lookup (§3 boot step 5); a held live process whose pid an impostor's snapshot records is a real process, never a double: in root tests the fake agent's `--hold-stdout-internal` holder, in `crates/viola-mcp/tests/handlers.rs` its own `Held` child, entered at `handlers_held_child_entry` only when its `HELD` variable is set, each killed and reaped by a `Drop` guard |»
  add: «| Process faults | none (a real wrapper, a real signal or kill) | A stale holder is a real wrapper under the `Frozen` guard (`tests/cli_instance_state.rs`; on Unix a `SIGSTOP`), which `run_refuses_a_stale_name` stands on; a wrapper lost under an open call is a real kill, `support::start::kill_wrapper` (`tests/support/start.rs`) for the lost row of `tests/cross_exit_causes.rs` and for `tests/chaos_wait_vanish.rs`, `kill_by_pid` (`crates/viola-e2e/tests/mcp_tools.rs`) for `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`; no double stands in for either |»
- line: .andromeda/test-plan-detail/test-data-fixtures.md:10
  old: «Three exceptions: the chaos tests, which corrupt files the product already wrote; the server-identity impostor cases (below); and on Windows x64»
  new: «Four exceptions: the chaos tests, which corrupt files the product already wrote; the server-identity impostor cases (below); the MCP handler suite's own snapshots (`crates/viola-mcp/tests/handlers.rs`, `record_live` below); and on Windows x64»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:30
  anchor: «- The server-identity impostor cases (Linux and Windows: `served` in `tests/security_negatives_server_identity.rs`, `Impostor::serve` in `tests/hook_fail_open.rs`) write an `InstanceSnapshot` themselves, recording a held live process's pid (the fake agent's holder, below) beside an endpoint the test binds, so that a server that is not the recorded wrapper answers the frame-writing clients.»
  add: «- `crates/viola-mcp/tests/handlers.rs` `record_live(instance, endpoint, pid)` writes an `InstanceSnapshot` recording an endpoint no one ever bound, for the unserved row of `handlers_frame_tools_name_the_cause_of_an_unreachable_instance`: a Windows `Serving` has no `Drop` (its pipe lives in the accept thread until the process exits), so the row binds nothing.»
- line: .andromeda/test-plan-detail/test-data-fixtures.md:19
  old: «the printed time checked by shape) are literals in their test files»
  new: «the printed time checked by shape), and the exit-cause table's codes, exits and last `hint:` lines (`tests/cross_exit_causes.rs` `CAUSES`), are literals in their test files»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:81
  anchor: « the one MCP exchange of `2.1.302`, `Mcp.tools-list-1.json`, against `schemas/claude-mcp-fixture.v1.json` (`v` const 1; `client` JSON-RPC 2.0 frames that contain a `tools/list` request with an `id`; `results` keyed by method, `tools/list` holding a `tools` array of named tools) with the same path, username and email walk (`mcp_fixtures_pass_hygiene`; planted `planted_clean_mcp_exchange_passes`, `planted_dirty_mcp_exchange_is_rejected`, `planted_mcp_schema_violation_is_rejected`); and apart from them the `Screen.{modal,ready,turn}.json` screens of `2.1.287`, `2.1.288` and `2.1.302` against `schemas/claude-screen.v1.json` (`screen_phase` modal|ready|turn, `cols` const 80, exactly 24 `rows`); a planted drive-path file proves it red, and a planted nested-path variant proves the variant walk red.»
  add: «- A recording refused as unclean is `verify`'s `Ended::Refused`: under a parsed `VIOLA_NAME` its `process-exit` reads `detail` `unclean-fixture`, a member of the `process-exit` `detail` enum in `schemas/diag-line.v1.json`, and the fixture's `<file>` enters no code.»

## Patterns
[[pattern]]
id = "tp4-01"
fixed = 'twenty-four verify-driven'

[[pattern]]
id = "tp4-02"
fixed = 'contract_ledger_probes cross_list_rows'

[[pattern]]
id = "tp4-03"
fixed = 'Three exceptions'
ignore_case = true
