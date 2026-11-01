# Apply — test-plan · part 3 of 5

## Verdicts
- fanout-test-plan--cascade-g1.md 14 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:10#1
- fanout-test-plan--cascade-g1.md 15 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:18#1, .andromeda/test-plan-detail/unit-test-strategy.md:18+#1
- fanout-test-plan--cascade-g1.md 16 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:18+#2
- fanout-test-plan--cascade-g1.md 18 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:35+#1
- fanout-test-plan--cascade-g1.md 19 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:35+#2
- fanout-test-plan--cascade-g1.md 21 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:90+#1
- fanout-test-plan--cascade-g1.md 22 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:90#1
- fanout-test-plan--cascade-g1.md 23 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:91+#1
- fanout-test-plan--cascade-g2.md 1 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:18#1
- fanout-test-plan--cascade-g2.md 2 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:18#2
- fanout-test-plan--cascade-g2.md 3 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:41#1
- fanout-test-plan--cascade-g2.md 4 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:56#1
- fanout-test-plan--cascade-g2.md 5 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:58+#1
- fanout-test-plan--cascade-g2.md 6 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:60#1
- fanout-test-plan--cascade-g2.md 7 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:74#1
- fanout-test-plan--cascade-g2.md 8 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:102+#1
- fanout-test-plan--cascade-new-integration-test-strategy.md 1 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:33+#1

## Edits
- line: .andromeda/test-plan-detail/unit-test-strategy.md:10
  old: «`EndpointLost` and `ServerVerify` and has no serde path»
  new: «`EndpointLost`, `ServerVerify` and `SocketDir`, the last with the word `socket-dir-refused`, and has no serde path»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:18
  old: «function with injected `(peer_euid, own_euid)`»
  new: «function `peer_euid_admitted(observed, own, reported)` with injected values»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:18
  anchor: «  - The peer-credential decision function with injected `(peer_euid, own_euid)`, covering the cross-user rejection boundary from test-scope Sec 1.»
  add: «  - The peer euid decision and read: a peer `peer_euid_admitted` cannot read is refused where the euid is reported (`PEER_EUID_REPORTED`, `cfg!(unix)`), held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`; the real read by `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`, with their siblings `client_server_euid_is_none_where_the_peer_reports_no_euid` and `tokio_client_server_euid_is_none_where_the_peer_reports_no_euid` (`crates/viola-channel/src/client.rs`, `tokio_client.rs`).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:18
  anchor: «  - The peer-credential decision function with injected `(peer_euid, own_euid)`, covering the cross-user rejection boundary from test-scope Sec 1.»
  add: «  - The wrapper's drop: `serve_drops_a_refused_peer_before_any_frame` (`crates/viola-channel/src/server.rs`) holds a peer an injected decision refuses dropped on the accept thread, before its worker thread and before any frame, the peer reading end-of-file, with no line written and nothing on the terminal; `channel_endpoint_answers_protocol_faults` is its control.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:35
  anchor: «  - The home-tree and diagnostics checks: `check_home_tree_of_a_tree_viola_wrote_passes`, `check_home_tree_refuses_each_widened_path`, `check_diagnostics_of_what_viola_wrote_or_of_nothing_passes`, `check_diagnostics_refuses_a_widened_mode` and `check_diagnostics_refuses_a_symlinked_role_file_and_folder` (a symlink is refused, a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows, and an absent path passes), and `strict_instance_holds_the_content_folders_to_the_read_rule`.»
  add: «  - The socket directory check (`crates/viola-state/src/strict.rs`): the plain `socket_dir_verdict(is_symlink, is_dir, mode, uid, euid)` reads a symlink `Refused::Symlink`, a non-directory `Refused::NotADirectory` and else `unix_read_verdict`, held by `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, a foreign owner by injected uid; `strict::check_socket_dir`, which passes an absent path and never chmods, by `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found` and, off Unix, `check_socket_dir_off_unix_reads_nothing`; `check_endpoint_dir` by `check_endpoint_dir_reads_the_endpoints_directory`.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:35
  anchor: «  - The home-tree and diagnostics checks: `check_home_tree_of_a_tree_viola_wrote_passes`, `check_home_tree_refuses_each_widened_path`, `check_diagnostics_of_what_viola_wrote_or_of_nothing_passes`, `check_diagnostics_refuses_a_widened_mode` and `check_diagnostics_refuses_a_symlinked_role_file_and_folder` (a symlink is refused, a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows, and an absent path passes), and `strict_instance_holds_the_content_folders_to_the_read_rule`.»
  add: «  - The board's rule on the home: `board::scan` (`crates/viola-state/src/board.rs`) holds the home to the read rule through `check_read` and `instances/` to `check_path` on every OS, held by `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and, on `windows-2025`, `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`.»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:90
  old: «the other three `during` `connect`).»
  new: «the other three `during` `connect`); `Unreachable::SocketDir` logs `socket-dir-refused`, `during` `connect`.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:90
  anchor: «  - The server check before any frame: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release,link,unlink}.rs` and `hook_refuses_an_unverified_server_inside_hook_handle` (`src/cmd/hook.rs`) assert the refusal inside the verb's or the hook's span with no frame written, each through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`; compiled out on macOS, §2 Unit); `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; `endpoint-lost` `during` `call`, the other three `during` `connect`).»
  add: «  - The socket directory check before any frame: `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` (`src/cmd/client.rs`), which opens the `wait.block` span by hand around `connect`, and `hook_refuses_a_loosened_socket_directory_inside_hook_handle` (`src/cmd/hook.rs`); the span recorder `Seen` stands in the test module `refused_seen`, out of the macOS-gated `refused_inside`, so these two run on macOS.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:91
  anchor: «  - The `unwrapped-name` and `endpoint-lost` hints are built in `src/human/hints.rs`, held by `send_hint_is_the_design_string`; the hint cause of `instance-dead` keys on `not-running`, a word apart from the log's.»
  add: «  - The hint of cause `socket-dir`, a word apart from the log's `socket-dir-refused`, is built in `src/human/hints.rs`, held by `send_hint_is_the_design_string`.»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:18
  old: «are read back with `std::fs::metadata`.»
  new: «are read back with `std::fs::metadata`. On Unix the directory is also checked before use: `check_socket_dir`, through `check_endpoint_dir`, refuses a directory with a foreign bit or a foreign owner, a symlink or a non-directory, passes an absent path and never chmods, a refused directory staying as found; a directory at 0755 and a symlink to 0700 are refused in §6 Security control negatives (`tests/security_negatives_socket_dir.rs`), a foreign owner by injected uid at `socket_dir_verdict` (§4 viola-state).»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:18
  old: «(`tests/security_negatives_channel.rs`).»
  new: «(`tests/security_negatives_channel.rs`). `viola_channel::Client::connect` itself carries no directory check, as `viola-channel` has no `viola-state` edge; `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:33
  anchor: «- **Forced-window witness, filesystem placement** (no PTY spawn; beside the PTY ones above): `place_private_new_placed_copy_execs_at_once` (`crates/viola-state/src/fs.rs`, Unix) execs the placed copy right after a `Placed::Written` placement, through `place_private_new_with`'s `placed` hook; with the guard removed (the copy persisted while its write handle is still open) it is red on the Linux dev host with `ExecutableFileBusy` (raw 26, `ETXTBSY`) and green with the guard (as measured at `viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/red-green.md`), and green on `ubuntu-latest` and `macos-latest`; its Windows form `place_private_new_placed_copy_runs_at_once` is green-only on `windows-2025` (as measured at ci#40439188053), the red leg read on the Linux dev host only.»
  add: «- **Harness `cleanup` witnesses** (the `endpoint_gone` reading on Unix, §3 `cleanup` step 4): `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone` (`crates/viola-e2e/tests/harness_lifecycle.rs`) on `ubuntu-latest`, and the unit `unconnectable_reads_a_leftover_socket_file_no_one_listens_on` (`crates/viola-e2e/src/harness/cleanup.rs`).»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:41
  old: «`empty_home` and reads `unwrapped-name`.»
  new: «`empty_home` and reads `unwrapped-name`. A socket directory meets the same trap: a test placing an endpoint in a temp directory makes it through `viola_state::fs::create_private_dir`, because a bare `tempfile::tempdir()` meets the socket directory check as a plain home meets the read rule; `crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs` and `refused_inside` in `src/cmd/client.rs` do so.»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:56
  old: «macOS reports no peer pid, so there the check passes with no comparison, the recorded gap; its euid and directory check is owed to the route entry "Unix endpoint and home hardening".»
  new: «macOS reports no peer pid, so there the pid and start-time comparison is absent, the named carve-out, owned by no route entry, and the server-identity negatives stay compiled out; its peer euid and socket directory checks are built, a refused euid reading `server-verify-failed` and a refused directory `socket-dir-refused`.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:58
  anchor: «  - the MCP widened-instance case `handlers_frame_tools_refuse_a_widened_instance_before_any_frame` (`crates/viola-mcp/tests/handlers.rs`) drives the MCP tools over stdio after an `initialize` handshake: with the instance directory widened and its recorded wrapper serving, MCP `send`, `wait`, `last` and `answer` each get `isError` with `{"error":"instance-unreachable","detail":"strict-modes-failed"}` before any snapshot read or connection; no frame reaches the wrapper, no line is written and the widening is left as found.»
  add: «  - the MCP socket directory case `handlers_frame_tools_refuse_a_loosened_socket_directory` (`crates/viola-mcp/tests/handlers.rs`) holds MCP's frame tools at no frame and no line past `live_endpoint`, with `isError` the record, `instance-unreachable` with `detail` `socket-dir-refused`.»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:60
  old: «a failed `server_verified` is `server-verify-failed` (off macOS)»
  new: «a refused `check_endpoint_dir` after `live_endpoint` is `socket-dir-refused`, a peer euid `peer_admitted` refuses or a failed `server_verified` is `server-verify-failed` (Linux, Windows and macOS, where on macOS it means an euid mismatch alone)»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:74
  old: «`a_home_another_user_may_read_refuses_run`).»
  new: «`a_home_another_user_may_read_refuses_run`). The same home refuses `viola list`, held to the read rule on all 3 OSes by `list_refuses_a_home_another_user_may_read_with_and_without_an_instance` (`tests/cli_list.rs`), with and without an instance, the refusal the whole board's, `ScanRefused::Home`, and opening nothing.»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:102
  anchor: « The human-wheel negative (`send` under a human wheel → exit 10) joined the table with chunk 2026-10-04-the-wheel, and the turn-running negative (`send` during a running harness turn → exit 13 `not-delivered` / `turn-running`) with chunk 2026-10-04-running-turn-refusal, and the budget-gate negative (`setting_does_not_disable_the_budget_gate`, over the `config.json` key `five_hour`) with chunk 2026-10-10-budget-governor, its cases `seven_day_over_one_hundred` and `v_two` (the `seven_day` and `v` keys) with chunk 2026-10-17-cli-machine-contract, which also landed the completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` and the rows `viola_dir_does_not_disable_the_home_scan` (`cfg(unix)`: a `VIOLA_DIR` naming a 0770 home's instance dir, `viola list` exit 21), `viola_variable_does_not_disable_the_send_paste_control` (`VIOLA_BIN` / `VIOLA_NAME`, `send` exit 13) and `config_key_does_not_disable_the_identity_strip` (`diagnostics_level` / `claude_env_keep`); chunk 2026-10-17-cli-output-discipline added the colour variables `NO_COLOR`, `TERM` and `COLORTERM` as cases of `setting_does_not_disable_the_send_paste_control`, whose helper removes the three inherited values before setting the case's own; chunk 2026-10-19-dialog-hold-for-the-driver added the `config.json` key `dialog_hold_s` as the row `setting_does_not_unbound_the_dialog_hold` (booted on a stamped home, five cases — `0`, `3601`, `-1`, `"3600"`, `false` — each giving exactly one `parse-rejected{parser:"config-json", detail:"malformed"}` line in `run-builder.ndjson`; the value each falls back to is the `src/run/dialog.rs` unit `read_hold_falls_back_to_sixty_with_one_rejection`), so `SETTINGS` holds ten rows (rule: `SETTINGS` in `tests/cli_controls_not_disableable.rs`), its `--home` row the case `setting_does_not_disable_the_home_check` (`cfg(unix)`: under each `SETTINGS` row in turn, `viola send` against a 0770 home exits 21 with the instance pair before any log opens, the home's mode as found); the other verb negatives still to join are listed in the file's header.»
  add: «    - `setting_does_not_disable_the_socket_dir_check` (`tests/cli_controls_not_disableable.rs`) runs every `SETTINGS` row against the socket directory check, a case of its own beside the five negatives below.»

## Patterns
[[pattern]]
id = "tp3-01"
fixed = 'holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost` and `ServerVerify`'

[[pattern]]
id = "tp3-02"
fixed = 'with injected `(peer_euid, own_euid)`'

[[pattern]]
id = "tp3-03"
fixed = 'its euid and directory check is owed'

[[pattern]]
id = "tp3-04"
fixed = '`server-verify-failed` (off macOS)'
