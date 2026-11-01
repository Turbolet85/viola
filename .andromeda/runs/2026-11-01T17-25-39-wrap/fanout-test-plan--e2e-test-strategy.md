proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 280
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "As landed, exit 21's six closed `detail` causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`, on the eight frame verbs, `list` and `plugin install` (`socket-dir-refused` at the eight frame verbs' `connect`), with `error` `instance-unreachable` and MCP's text `refused: instance-unreachable`."
    sidecar: Exit-cause matrix, exit 21's closed causes five to six, the sixth `socket-dir-refused`.
    rationale: The report moves exit 21's causes five to six, a row reading five closed changes (report.md:28), and places the sixth on the eight CLI frame verbs (report.md:71).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 289
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: End the row with `instance-dead`, `unwrapped-name`, `server-verify-failed` and `socket-dir-refused` are `during:"connect"`; the rest of the row is unchanged.
    sidecar: Exit-cause matrix, `socket-dir-refused` joins the causes read during `connect`.
    rationale: The sixth cause's line carries `during:"connect"` (report.md:71), and this row lists the connect causes by name, so the list extends (report.md:28).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
    dependent-of: D-tests-count-literals
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 289
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Add a row after this line at its indent, the socket directory fails its check (a 0755 directory or a symlink to a 0700 one, read at the recorded endpoint's parent), `detail` `socket-dir-refused`, `during:"connect"`, on the eight frame verbs, no frame and no `channel-request` line, one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, its own hint, cause `socket-dir` in `src/human/hints.rs`, the new `unable:` / `hint:` pair a planning form with the founder's wording owed at Epoch 7's boundary batch (`inputs#I4` item 5); the table's `a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused` in both modes, and MCP's `isError` record with no line by rule, `handlers_frame_tools_refuse_a_loosened_socket_directory` (`crates/viola-mcp/tests/handlers.rs`).
    sidecar: Exit-cause matrix gains the `socket-dir-refused` row of exit 21 and its witnesses.
    rationale: The sixth cause has no row among exit 21's causes; the report names its record, hint cause and witnesses (report.md:15, report.md:17, report.md:71, report.md:76, report.md:253).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 284
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: server verification fails. On Linux and Windows, `snapshot.json` pid or start time does not match the listening process (a same-user impostor listener on the recorded endpoint), as a `#[cfg(not(target_os = "macos"))]` case. On macOS the client reads no peer pid and compares no server pid (the recorded gap, the named carve-out, owned by no route entry). The client euid check is built on Linux and macOS, a peer of another euid reading `server-verify-failed`, on macOS an euid mismatch alone, held at the decision over injected euids since one account can make no foreign peer (Security control negatives, Peer euid). The case that loosens the verified per-user socket dir to 0755 landed in `tests/security_negatives_socket_dir.rs`, on `macos-latest` too, and reads exit 21 `socket-dir-refused`, not this cause. A same-user impostor inside a valid 0700 dir on macOS is an accepted upstream limitation, not asserted as a refusal.
    sidecar: Server-verification cause, the owed client euid check and macOS case discharged; `server-verify-failed` reaches macOS as an euid mismatch; the pid gap stays named, owned by no route entry.
    rationale: The dated gaps on the route entry are built and retired (report.md:26), its owner clauses all change but macOS's pid gap, which stays named and owned by no route entry (report.md:33), the word's reach moves (report.md:31), and the macOS case landed in the new binary (report.md:64).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 250
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Keep the matrix's clause and add directly after `server-verify-failed`'s parenthesis that on macOS the same word reads a peer of another euid alone, which no row of this matrix drives, since one account can make no foreign peer.
    sidecar: Hook fail-open matrix, `server-verify-failed`'s reach read beside its Linux and Windows row.
    rationale: The word's reach moves from off macOS to Linux, Windows and macOS, on macOS an euid mismatch alone, and a hook's refused euid is `NotSent::ServerUnverified` (report.md:31, report.md:16); the row names the word beside Linux and Windows only.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:31
    dependent-of: D-tests-count-literals
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 250
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Add a sentence at this line's end, a hook meeting a refused socket directory is no row of this matrix, its cases stand in `tests/security_negatives_socket_dir.rs` (`security_negative_a_hook_fails_open_on_the_socket_directory`, over `stop` and `permission-request`), exit 0, empty streams, no frame, one `hook-decision{detail:"socket-dir-refused"}` at WARN, on the dialog path after `hook-invoked{corr:null}` with `decision_emitted:false`, beside the unit `hook_refuses_a_loosened_socket_directory_inside_hook_handle`.
    sidecar: Security sweep names where the hook's refused-directory rows live.
    rationale: The P5 word's lean puts the hook's rows in the new binary, not `tests/hook_fail_open.rs` (report.md:95, report.md:64), and the report states the record (report.md:72).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:72
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 263
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Add a bullet after this line at its indent, the `socket-dir-refused` documents, `revive --json`'s included, are held to the check by `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`), and the enumeration in `tests/contract_error_bodies.rs` holds the new body.
    sidecar: Error sanitization names the socket-directory documents' witness.
    rationale: The chunk adds the case and the enumeration's body (report.md:8, report.md:235, report.md:249), and a new error document needs its surface witness named beside the check.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:235
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 300
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "`viola revive --json` prints one exit-1 document for `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused` and `internal-error`."
    sidecar: Exit-cause matrix, `revive --json`'s exit-1 list extends to eleven codes with `socket-dir-refused`.
    rationale: The codes move ten to eleven and each list holding all ten extends (report.md:29).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 299
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Extend the row's parenthesis of refusals with `socket-dir-refused` and its list of table rows with `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`; the rest of the row is unchanged.
    sidecar: Exit-cause matrix, the refusals of `run`'s start under `revive --json` gain `socket-dir-refused`.
    rationale: The start of `revive` shares the refusal, made before any snapshot, child or `Server::bind` (report.md:14), and this row names the start's refusals with each one's table row, its witness `cross_exit_causes` (report.md:111, report.md:255).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
    dependent-of: D-tests-count-literals
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 297
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Add a row after this line at its indent, the per-user socket directory fails its check at the start, exit 1 before any snapshot, child or `Server::bind`, one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` line (a step with no span), the fixed `unable:` / `hint:` pair, the directory as found; a failed create reads the same code and a `Server::bind` failure stays what it was; the table's `a_loosened_socket_directory_on_a_start_is_socket_dir_refused` in both modes, beside `security_negative_a_start_refuses_the_socket_directory_before_any_bind` (`tests/security_negatives_socket_dir.rs`).
    sidecar: Exit-cause matrix gains the `socket-dir-refused` row of `run`'s exit 1.
    rationale: The chunk adds a start refusal absent from exit 1's causes (report.md:14, report.md:70, report.md:95, report.md:107).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 317
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Replace the opening with **Unix socket directory refusal** (`#![cfg(unix)]`, on `ubuntu-latest` and `macos-latest`, IPC access control). With a test-owned socket root (`SOCKET_ROOT_VAR`, `TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), pre-create the per-user socket directory as (a) a symlink to another 0700 dir and (b) a 0755 dir; the rest of the row stands, its injected `lstat` result being the arguments of `socket_dir_verdict`.
    sidecar: Security control negatives, the socket-directory case's `cfg` widens from Linux to Unix and its root is named per OS.
    rationale: The macOS case landed in this binary with a `cfg` wider than the plan's `target_os = "linux"` (report.md:64, report.md:128), and the root variable is per OS (report.md:21).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:64
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 318
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: The socket-directory refusal's code is `socket-dir-refused`, in `start` before any snapshot, child or `Server::bind`, through `socket_dir_ready`, which reads `check_endpoint_dir`, creates the parent only when absent and reads it again, and `refuse_socket_dir`; a failed create and a refused check read one code, `revive`'s start shares it, and a `Server::bind` failure stays what it was. macOS's server check stays the named carve-out, owned by no route entry.
    sidecar: Security control negatives, the no-code socket-directory clause and its route-entry owner retired.
    rationale: The code owed to the route entry is built and retired (report.md:26), the refusal is `refused_with`'s `socket-dir-refused` (report.md:14), one code for a failed create and a refused check (report.md:95), macOS's gap owned by no route entry (report.md:33).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 318
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Add a line after this one at its indent, As landed (chunk 2026-11-01-unix-endpoint-and-home-hardening), `tests/security_negatives_socket_dir.rs` (`required-features = ["fake-agent"]`, on the `ci` profile's 10 s kill line) holds `security_negative_a_start_refuses_the_socket_directory_before_any_bind`, `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` and `security_negative_a_hook_fails_open_on_the_socket_directory` over each form, the directory read back as found and no path, pid or uid in any output or line (`assert_names_nothing`, `digit_runs`), beside `security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private`; the foreign owner is `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, by injected uid; `setting_does_not_disable_the_socket_dir_check` (`tests/cli_controls_not_disableable.rs`) holds that no setting turns it off. No foreign owner is driven.
    sidecar: Security control negatives records the socket-directory binary as landed.
    rationale: The chunk adds the binary and its acceptance reads MET on `ubuntu-latest` and `macos-latest` (report.md:22, report.md:24, report.md:107, report.md:116, report.md:130, report.md:231).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 328
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Add a sentence at this line's end, an impostor case's endpoint sits in a directory made through `viola_state::fs::create_private_dir`, here and in `tests/hook_fail_open.rs`, because `tempfile` 3.27 leaves a bare `tempfile::tempdir()` at the umask's mode, 0755 under a default umask, and the socket-directory check refuses it before the server check is reached.
    sidecar: Server identity negatives record the private directory their impostor endpoints need.
    rationale: Measured at implement, the new client check refused a bare temp dir, and four sites were narrowed through `create_private_dir` (report.md:56, report.md:96); these rows' setup now rests on it.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:56
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 329
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Add a bullet after this line at the Server identity row's indent, **Peer euid** (IPC access control, reported on Unix, `PEER_EUID_REPORTED`). The client's decision is `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported` over injected euids, the real read `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`, the wrapper's drop `serve_drops_a_refused_peer_before_any_frame` over an injected decision (end-of-file, no line, nothing on the terminal) with `channel_endpoint_answers_protocol_faults` its control. One account can make no foreign peer and no seam may be added, so no foreign euid is driven, and each client seam's wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) was read only once, through a forced refusing decision in the chunk's remove-the-guard runs (`evidence/remove-the-guard.md` rows 7, 8, 9), which no committed test repeats, an always-admit mutant surviving by construction; bound with §1's untestable zones.
    sidecar: Security control negatives gains the peer euid row and names the seam wiring no committed test holds.
    rationale: The report names the decisions, reads and drop as committed tests, and the wiring as the weakest proof, §1's untestable zone naming it by this wrap's ruling (report.md:12, report.md:13, report.md:54, report.md:74, report.md:75, report.md:108, report.md:109, report.md:130).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 330
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: Add a bullet after this line at its indent, **`viola list`'s home** (the three CI OSes; the read rule on every OS stands PROVISIONAL, the overseer's word as delegate, `inputs#I2` Q3). `list_refuses_a_home_another_user_may_read_with_and_without_an_instance` (`tests/cli_list.rs`) reads the whole-board refusal over a home another user may read, with and without an instance, beside the units `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and, on `windows-2025`, `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`; `instances/` stays on `check_path`.
    sidecar: Security control negatives gains the board's read rule on the home, PROVISIONAL.
    rationale: The two rules of `viola list` on the home are retired for the read rule (report.md:26, report.md:32), `scan` reading the home through `check_read` on every OS (report.md:18), MET on all three OSes (report.md:112), the word PROVISIONAL (report.md:94).
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:112
note: Outside this file, test-plan.md:610 still reads the server-verification macOS case as owed to the route entry "Unix endpoint and home hardening", which this detector may not edit, so the body's pass should carry it.
