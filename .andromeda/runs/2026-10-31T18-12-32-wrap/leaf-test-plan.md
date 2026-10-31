group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 106 · «- 2026-10-27: Every test or control that spawns `viola run`,»
read: .claude/rules/verification-harness.md · 72 · «- 2026-10-20: `target/debug/viola` is a hard link cargo re-p»
- leaf: .claude/docs/tests-summary.md:32   class: stale
  leaf-text: «as landed `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable`; the unwrapped-name hint not built, owed to "Exit-cause code catalogue"»
  source: .andromeda/test-plan.md:553 «`tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable` (exit 21, `detail` `unwrapped-name` under `--json`, its hint built;»
  replace-with: «as landed `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable`, `detail` `unwrapped-name` under `--json`, its hint built; the table's `a_name_never_started_is_unwrapped_name`, `tests/cross_exit_causes.rs`, holds a name never started»
- leaf: .claude/docs/tests-summary.md:33   class: stale
  leaf-text: «exit-cause matrix (`cross_exit_causes.rs` not built: each built cause is a case of the binary its row names, the server-verification cause in `tests/security_negatives_server_identity.rs` on Linux and Windows)»
  source: .andromeda/test-plan.md:610 «**Exit-cause matrix** (`tests/cross_exit_causes.rs`, a root `cross_` binary): one negative test per cause of exit 21 and of exit 1 it holds,»
  replace-with: «exit-cause matrix (`tests/cross_exit_causes.rs`, a root `cross_` binary: one negative test per cause of exit 21 and of exit 1 it holds, in both modes where a mode exists, a hint shared across causes failing the table; a cause its top comment names outside the table is a case of the binary its row names, the server-verification cause in `tests/security_negatives_server_identity.rs` on Linux and Windows)»
- leaf: .claude/docs/tests-summary.md:27   class: omission
  leaf-text: «typed timeout, exit 21 on a vanished wrapper»
  source: .andromeda/test-plan-detail/e2e-test-strategy.md:92 «`endpoint-lost` for a wrapper killed under a parked `wait`, and after `cleanup` `instance-dead` in a kept home or `unwrapped-name` in a removed one;»
  replace-with: «typed timeout, exit 21 on a vanished wrapper (`detail` `endpoint-lost` for a wrapper killed under a parked `wait`, after `cleanup` `instance-dead` in a kept home or `unwrapped-name` in a removed one; MCP the same body less `v`, `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`)»
- leaf: .claude/rules/testing.md:19   class: omission
  leaf-text: «plus the root package's numbered sync Path binaries (`tests/path1_run_start.rs`, `[[test]]` `required-features = ["fake-agent"]`); both report»
  source: .andromeda/test-plan-detail/test-strategy.md:44 «and a `cross_` binary (`tests/cross_exit_causes.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report in the E2E layer»
  replace-with: «plus the root package's numbered sync Path binaries (`tests/path1_run_start.rs`, `[[test]]` `required-features = ["fake-agent"]`) and its `cross_` binary (`tests/cross_exit_causes.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report»
- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «the one complete-lines reader in `ndjson.rs` (every reader of a live-appended file parses only newline-terminated lines),»
  source: .andromeda/test-plan-detail/test-strategy.md:43 «`tests/support/start.rs` shares the start setups across root test files: `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`,»
  replace-with: «the one complete-lines reader in `ndjson.rs` (every reader of a live-appended file parses only newline-terminated lines), the shared start setups in `start.rs` (`first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper`),»
- leaf: .claude/rules/testing.md:34   class: stale
  leaf-text: «save the server-identity impostor cases' test-built snapshot, Linux and Windows, recording a held live process's pid beside the endpoint the test binds)»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:10 «the server-identity impostor cases (below); the MCP handler suite's own snapshots (`crates/viola-mcp/tests/handlers.rs`, `record_live` below);»
  replace-with: «save the server-identity impostor cases' test-built snapshot, Linux and Windows, recording a held live process's pid beside the endpoint the test binds, and the MCP handler suite's own snapshots, `crates/viola-mcp/tests/handlers.rs`, whose `record_live` records an endpoint no one ever bound for the unserved row)»
- leaf: .claude/rules/verification-harness.md:33   class: omission
  leaf-text: «`degraded` when one is `stale` or absent, `down` on a non-zero exit (`list-exit-<code>`) or no exit, spawn or `supervise.json` (`list-exit-none`)»
  source: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:17 «or exits 0 with no JSON object on stdout (`list-exit-0`, `list:null`)»
  replace-with: «`degraded` when one is `stale` or absent or the document holds no `items`, `down` on a non-zero exit (`list-exit-<code>`), on no exit, spawn or `supervise.json` (`list-exit-none`), or on exit 0 with no JSON object on stdout (`list-exit-0`, `list:null`)»
