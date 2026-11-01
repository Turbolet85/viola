proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 56
    section: '## 5. Integration Test Strategy'
    change: 'Replace the clause «its euid and directory check is owed to the route entry "Unix endpoint and home hardening"» with «its euid and directory checks are built (chunk 2026-11-01-unix-endpoint-and-home-hardening): a server of another euid reads `server-verify-failed` on Linux and macOS, the decision held over injected euids by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported` and the real read by `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`; the wiring of that read at each client seam has no committed test (§1 untestable zones)». The sentence that macOS reports no peer pid and passes with no comparison stays, the recorded gap now the named carve-out, owned by no route entry.'
    sidecar: 'Integration Test Strategy, Wrapper channel: the macOS euid and directory checks no longer owed to "Unix endpoint and home hardening", built; the pid gap stays the named carve-out with no route-entry owner.'
    rationale: 'The dated gaps on this owner are retired as built (report.md:26); each owed clause changes, and a row stating macOS compares no peer pid stays true (report.md:33). The wiring limit is the report own record (report.md:54, report.md:109).'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:56
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 60
    section: '## 5. Integration Test Strategy'
    change: 'Replace «a failed `server_verified` is `server-verify-failed` (off macOS)» with «a refused `peer_admitted(client.server_euid())` or a failed `server_verified` is `server-verify-failed` (Linux, Windows and macOS, on macOS an euid mismatch alone)».'
    sidecar: 'Integration Test Strategy, MCP mapping: the reach of `server-verify-failed` moves from off macOS to Linux, Windows and macOS, on macOS an euid mismatch alone; the euid decision named in `frame`.'
    rationale: 'The verify-word qualifier moved and each standing row changes (report.md:31); `frame` now reads `!peer_admitted(client.server_euid()) || !server_verified(…)` as `UnreachableCause::ServerVerify` (report.md:17).'
    basis: crates/viola-mcp/src/tools.rs:352
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 60
    section: '## 5. Integration Test Strategy'
    change: 'In the MCP mapping, after «no `live_endpoint` is `without_endpoint`s reading» add «a refused `check_endpoint_dir` on the live endpoint is `socket-dir-refused`», and close the item with «`handlers_frame_tools_refuse_a_loosened_socket_directory` holds that word».'
    sidecar: 'Integration Test Strategy, MCP mapping: `socket-dir-refused` added, the sixth closed cause, with its witness.'
    rationale: '`frame` runs `check_endpoint_dir` after `live_endpoint` and maps a refusal to `UnreachableCause::SocketDir` (report.md:17); exit 21 causes five to six, the sixth `socket-dir-refused` (report.md:28); the witness (report.md:73).'
    basis: crates/viola-mcp/src/tools.rs:345
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 58
    section: '## 5. Integration Test Strategy'
    change: 'Add a sub-item: the MCP socket-directory case `handlers_frame_tools_refuse_a_loosened_socket_directory` (`crates/viola-mcp/tests/handlers.rs`): with the socket directory of a recorded wrapper loosened, the MCP frame tools each get `isError` with `{"error":"instance-unreachable","detail":"socket-dir-refused"}`; no frame reaches the wrapper and no line is written, `isError` the record.'
    sidecar: 'Integration Test Strategy, Wrapper channel: the MCP socket-directory case added.'
    rationale: 'New MCP refusal and its named test (report.md:17, report.md:73, report.md:111).'
    basis: crates/viola-mcp/tests/handlers.rs:521
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 56
    section: '## 5. Integration Test Strategy'
    change: 'Add a sub-item: a start on a refused socket directory, `tests/security_negatives_socket_dir.rs` (`#![cfg(unix)]`, green on `ubuntu-latest` and `macos-latest`) `security_negative_a_start_refuses_the_socket_directory_before_any_bind`: the directory at 0755 or a symlink to 0700, `viola run` exits 1 with `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` before any bind or child, the directory as found; the check never chmods.'
    sidecar: 'Integration Test Strategy, Wrapper channel: the start refusal of a loosened socket directory added, in the new binary `security_negatives_socket_dir`.'
    rationale: 'New start path and its witness (report.md:14, report.md:70, report.md:107); the binary runs on Unix only (report.md:128).'
    basis: tests/security_negatives_socket_dir.rs:205
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 56
    section: '## 5. Integration Test Strategy'
    change: 'Add a sub-item: a frame verb on a refused socket directory, `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` (same binary): each of the eight verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) exits 21 with `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, no `channel-request` and no frame, the directory as found, and no path, pid or uid in its text (`assert_names_nothing`, `digit_runs`).'
    sidecar: 'Integration Test Strategy, Wrapper channel: the frame-verb refusal of a loosened socket directory added.'
    rationale: 'The same check at the eight CLI frame verbs (report.md:15, report.md:71, report.md:107, report.md:116).'
    basis: tests/security_negatives_socket_dir.rs:241
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 56
    section: '## 5. Integration Test Strategy'
    change: 'Add a sub-item: a hook on a refused socket directory, `security_negative_a_hook_fails_open_on_the_socket_directory` (same binary, not `tests/hook_fail_open.rs`): `stop` and `permission-request` each exit 0 with empty streams and no frame, one `hook-decision{detail:"socket-dir-refused"}` at WARN, on the dialog path `hook-invoked{corr:null}` first and `decision_emitted:false`.'
    sidecar: 'Integration Test Strategy, Wrapper channel: the hook fail-open on a loosened socket directory added.'
    rationale: 'The same check at `viola hook`, and the lean placing its rows in the new binary (report.md:16, report.md:72, report.md:95, report.md:107).'
    basis: tests/security_negatives_socket_dir.rs:355
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 56
    section: '## 5. Integration Test Strategy'
    change: 'Add a sub-item: the clean-start control `security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private` (same binary): a wrapper started under an open umask keeps its socket private, read from the endpoint its snapshot records.'
    sidecar: 'Integration Test Strategy, Wrapper channel: the clean start under an open umask added.'
    rationale: 'New case in the new binary, listed with its snapshot endpoint read (report.md:313, report.md:314), beside `first_life_on` and its `open_umask` argument (report.md:325).'
    basis: tests/security_negatives_socket_dir.rs:408
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 56
    section: '## 5. Integration Test Strategy'
    change: 'Add a sub-item: the wrapper drop, `accept_loop` filtering `incoming()` by `admitted(peer_euid_of(stream))`: a refused stream is dropped on the accept thread, before its worker thread and any frame, the peer reading end-of-file, with no line and nothing on the terminal; `serve_drops_a_refused_peer_before_any_frame` over an injected decision (dependency injection, no env, flag or feature seam), `channel_endpoint_answers_protocol_faults` the control.'
    sidecar: 'Integration Test Strategy, Wrapper channel: the wrapper drop of a refused peer added, with its injected decision and control.'
    rationale: 'New channel path and its witness (report.md:13, report.md:75, report.md:108).'
    basis: crates/viola-channel/src/server.rs:1067
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 74
    section: '## 5. Integration Test Strategy'
    change: 'Add a sub-item: `viola list` holds the home to the read rule on every OS (`inputs#I2` Q3, the delegate word, PROVISIONAL), its whole-board refusal unchanged: `tests/cli_list.rs` `list_refuses_a_home_another_user_may_read_with_and_without_an_instance`, green on all 3 OSes, backed by the `viola-state` units `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and, on `windows-2025`, `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`.'
    sidecar: 'Integration Test Strategy, On-disk: `viola list` on a home another user may read added, PROVISIONAL on the delegate answer.'
    rationale: 'The board home moved from two rules to the read rule (report.md:18, report.md:32), met on all three OSes (report.md:112); the word stands PROVISIONAL (report.md:94).'
    basis: tests/cli_list.rs:555
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 102
    section: '## 5. Integration Test Strategy'
    change: 'After the `setting_does_not_disable_the_home_check` clause add: chunk 2026-11-01-unix-endpoint-and-home-hardening added `setting_does_not_disable_the_socket_dir_check`, run under each `SETTINGS` row in turn.'
    sidecar: 'Integration Test Strategy, CLI: the socket-directory negative joined `tests/cli_controls_not_disableable.rs`.'
    rationale: 'New no-disable negative over the `SETTINGS` table (report.md:231, report.md:232).'
    basis: tests/cli_controls_not_disableable.rs:648
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 41
    section: '## 5. Integration Test Strategy'
    change: 'Extend the Bare temp-dir home record: a socket directory in a bare temp dir meets the socket-directory check as a home meets the read rule, because `tempfile` 3.27 leaves the mode to the umask (0755 as measured at this chunk), so a test placing an endpoint in a temp dir makes it through `viola_state::fs::create_private_dir` (`crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs`, `refused_inside` in `src/cmd/client.rs`).'
    sidecar: 'Integration Test Strategy, Bare temp-dir home: extended to a socket directory, with the four sites narrowed.'
    rationale: 'The report names this line and rules the disposition the extension: the 0700 tempdir claim measured false, nine `handlers` cases refused on the first run, four sites narrowed (report.md:56, report.md:96).'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:41
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 41
    section: '## 5. Integration Test Strategy'
    change: 'Add a record after Bare temp-dir home: **Socket root:** `tests/support/home.rs` `SocketRoot`, a test-owned socket root under `/tmp` named by `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), with `Wrapper::boot_on_socket_root` and `first_life_on` (its `open_umask` argument), lets a test reach the per-user socket directory of its wrapper; `first_life_on` stands in `tests/support/home.rs`, not beside `first_life` in `tests/support/start.rs`.'
    sidecar: 'Integration Test Strategy, Setup / teardown: the test-owned socket root helpers added.'
    rationale: 'New test helpers (report.md:21, report.md:325), their roots under `/tmp` (report.md:63), the placement a recorded deviation (report.md:81).'
    basis: tests/support/home.rs:586
note: D-tests-framework and D-tests-obs-harness fire nothing in this file; the seam wiring gap of report.md:54 is the only new path with no committed test, ruled to test-plan §1 and matrix#v1-44, so it is stated in the D-retired row and not raised as D-tests-coverage drift.
