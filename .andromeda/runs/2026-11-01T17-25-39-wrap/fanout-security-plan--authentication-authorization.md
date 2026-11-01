proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 18
    section: '## Authentication & Authorization'
    change: 'In IPC access control (Unix), replace the whole «**Socket-directory failure at bind:**» sentence with: **Socket-directory failure at bind:** `socket_dir_ready` (`src/cmd/run.rs`) runs `check_endpoint_dir`, creates the parent only when absent, then checks again, so a race''s loser reads the same; a failed create and a refused check read one code, `socket-dir-refused`, and a `Server::bind` failure stays what it was.'
    sidecar: 'IPC access control (Unix): the bind-time socket-directory failure now has its code `socket-dir-refused`; the anyhow-at-the-catch-site reading and its owed clause retired.'
    rationale: 'The report''s Retired bullet names the socket-directory check and code owed to "Unix endpoint and home hardening" as built (report.md:26); Symbols names `socket_dir_ready` and the unchanged `Server::bind` failure (report.md:14); one code for a failed create and a refused check is a P5 lean (report.md:95).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:26
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 18
    section: '## Authentication & Authorization'
    change: 'In IPC access control (Unix), replace «`hook` fails open (exit 0, no body, a note in `diagnostics/`)» with «`hook` fails open (exit 0, no body, one `hook-decision` line at WARN, `detail` `socket-dir-refused`)».'
    sidecar: 'IPC access control (Unix): a hook meeting a refused socket directory leaves one `hook-decision` line, not a note in `diagnostics/`.'
    rationale: 'Spec claim disproved: the chunk writes one `hook-decision` at WARN, the form the server check took, a lean on the card `inputs#I3`, the overseer''s word as delegate, PROVISIONAL (report.md:59, report.md:95).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:59
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 18
    section: '## Authentication & Authorization'
    change: 'In IPC access control (Unix), the Directory check: replace «and not a symlink» with «and a directory, not a symlink (an absent path passes)», and replace «`run` refuses to start (exit 1), CLI/MCP clients report `instance-unreachable` (exit 21)» with «`run` refuses to start (exit 1, `socket-dir-refused`), CLI/MCP clients report `instance-unreachable` (exit 21, `detail` `socket-dir-refused`)».'
    sidecar: 'IPC access control (Unix): the directory check requires a directory, passes an absent path, and its refusal reads the closed word `socket-dir-refused`.'
    rationale: 'The check''s verdict and absent-path pass (report.md:11); the new closed word is the founder''s own, `inputs#I2` Q1 (report.md:93), the sixth cause of exit 21 (report.md:28).'
    basis: crates/viola-state/src/strict.rs:149
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'Add a row after this one. **Requirement:** IPC access control (Unix), the socket-directory check. **Implementation:** `viola_state::strict::check_socket_dir(dir)` takes one `symlink_metadata`: an absent path passes, an unstattable one is `Refused::Unreadable`, else the plain `socket_dir_verdict(is_symlink, is_dir, mode, uid, euid)` decides: a symlink `Refused::Symlink`, not a directory `Refused::NotADirectory` (fixed message «a trusted state path is not a directory»), else `unix_read_verdict`. Off Unix it returns `Ok(())`, Windows keeping the DACL. `check_endpoint_dir(endpoint)` is the same check on a recorded endpoint''s parent. The check never chmods: a refused directory stays as found. **Stack reference:** `viola-state` `strict` (`check_socket_dir`, `check_endpoint_dir`, `socket_dir_verdict`).'
    sidecar: 'New row: the socket-directory check, `check_socket_dir` / `check_endpoint_dir` / `socket_dir_verdict`, and `Refused::NotADirectory`.'
    rationale: 'New detail, report Symbols / APIs (report.md:11); deviation 2 names `check_endpoint_dir` (report.md:80).'
    basis: crates/viola-state/src/strict.rs:149
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'Add a row after the socket-directory check. **Requirement:** IPC access control (Unix), the start''s refusal. **Implementation:** `bind_endpoint` has three outcomes (`enum Bound`); on a refused directory `start` calls `refused_with(launch.json, "socket-dir-refused", …)` before any snapshot, child or `Server::bind`, exit 1, with one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}` line naming no path, pid or uid; `revive`''s start shares it. The human form `refuse_socket_dir(name)` prints `unable: the socket directory for <name> cannot be trusted` and `hint: viola''s socket directory must be a directory you own, readable only by you; viola does not change its permissions`, a planning form, the founder''s wording owed at Epoch 7''s boundary batch (`inputs#I3`, `inputs#I4` item 5). **Stack reference:** root bin `src/cmd/run.rs` (`Bound`, `socket_dir_ready`, `refuse_socket_dir`).'
    sidecar: 'New row: the start''s socket-directory refusal, exit 1 before any snapshot, child or bind, its pair a planning form.'
    rationale: 'New detail (report.md:14, report.md:70, report.md:76); the wording''s standing is the delegate''s word, PROVISIONAL (report.md:49, report.md:94).'
    basis: src/cmd/run.rs:578
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 19
    section: '## Authentication & Authorization'
    change: 'Add a row after the start''s refusal. **Requirement:** IPC access control (Unix), a killed wrapper''s socket file. **Implementation:** A force-killed wrapper''s socket file stays in the per-user directory until a next bind of that name takes it over; nothing removes it and no owner is proposed, a harmless file in a 0700 directory: the choice recorded under the overseer''s word as delegate, PROVISIONAL (`inputs#I3`). `agent-run cleanup`''s `endpoint_gone` reads a connect refused or a path absent, removing no leftover. **Stack reference:** `crates/viola-e2e/src/harness/cleanup.rs` (`unconnectable`).'
    sidecar: 'New row: a killed wrapper''s leftover socket file stays until a next bind of that name; nothing removes it.'
    rationale: 'New detail, the card''s third Beyond-the-plan row (report.md:51), the harness surface (report.md:20, report.md:42).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:51
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 21
    section: '## Authentication & Authorization'
    change: 'Replace the Implementation with: On every accept, `accept_loop(listener, dispatch, admitted)` filters `incoming()` by `admitted(peer_euid_of(stream))` (`Server::serve` through the private `serve_admitting(dispatch, peer_admitted)`: dependency injection, no env, flag or feature seam). A peer whose euid differs from `geteuid()`, or one it cannot read where euids are reported, is dropped on the accept thread, before its worker thread and before any frame: the peer reads end-of-file, and no line and nothing on the terminal is written.'
    sidecar: 'IPC server-side peer check: as built in `accept_loop`, an unreadable peer refused, the drop writing no line.'
    rationale: 'New detail, the wrapper''s drop (report.md:13, report.md:75); the no-line lean (report.md:95).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
  - detector: D-security-auth
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 22
    section: '## Authentication & Authorization'
    change: 'Replace the Stack reference «`viola-channel` sync + Tokio server» with «`viola-channel` `server.rs` (`accept_loop`, `serve_admitting`), the one sync server the code holds; `peer_euid_of` in `client.rs`».'
    sidecar: 'IPC server-side peer check: the stack reference names the one sync server; no Tokio server exists.'
    rationale: 'Spec claim disproved: one sync server is what the code holds, a lean on the card `inputs#I3` (report.md:60).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:60
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 24
    section: '## Authentication & Authorization'
    change: 'In the Unix identity check, replace «(the client-side euid check is not built, owed to the route entry "Unix endpoint and home hardening")» with «(on Linux and macOS, through `Client::server_euid` / `TokioClient::server_euid` and `peer_admitted`, a peer it cannot read refused)». The macOS sentence stays: euid + the verified directory, no pid check, a true claim.'
    sidecar: 'IPC client-side server verification: the client euid check is built; its owed clause retired.'
    rationale: 'The report''s Retired bullet names the client euid check as built (report.md:26); a row reading a built check as owed changes, a row stating macOS compares no peer pid stays (report.md:33).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 25
    section: '## Authentication & Authorization'
    change: 'Replace «`viola hook` through its shared `connect` (`connect_by`, then `verify_server`)» with «`viola hook` through its shared `connect` (`check_endpoint_dir`, `connect_by`, `peer_admitted`, then `verify_server`)».'
    sidecar: 'Strict-modes check first: the hook''s `connect` order gains the directory and euid checks.'
    rationale: 'The hook''s `connect` runs the same three checks in the same order as the CLI''s (report.md:15, report.md:16).'
    basis: src/cmd/hook.rs:231
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 36
    section: '## Authentication & Authorization'
    change: 'Replace from «proven by `cfg(unix)` cases only; `check_path` holds the home» through «owed to the route entry "Unix endpoint and home hardening"» with: proven on all three OSes (`list_refuses_a_home_another_user_may_read_with_and_without_an_instance`, `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance`, on `windows-2025` `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`); the board holds the home to the read rule on every OS, with or without an instance, so a home another user may read (on Unix a 0750 one) refuses the whole board (`ScanRefused::Home`), on the overseer''s technical answer as the founder''s delegate, PROVISIONAL, the founder''s own word owed at Epoch 7''s boundary batch (`inputs#I2` Q3).'
    sidecar: '`viola list`: the home''s two rules retired for the read rule on every OS, PROVISIONAL on the delegate''s answer; the CARRY discharged.'
    rationale: 'Retired: `viola list`''s two rules on the home (report.md:26), this line named by pattern `board-rule` (report.md:32); witnesses (report.md:112); Q3''s standing is the delegate''s, PROVISIONAL, not the founder''s (report.md:45, report.md:50, report.md:94).'
    basis: crates/viola-state/src/board.rs:557
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 35
    section: '## Authentication & Authorization'
    change: 'Replace «`viola_state::board::scan` runs `check_path` on the home and on `instances/`, each that exists» with «`viola_state::board::scan` runs `check_read` on the home and `check_path` on `instances/`, each that exists».'
    sidecar: '`viola list`: `scan` holds the home through `check_read`, `instances/` staying `check_path`.'
    rationale: 'The same retired two rules: `check_if_present(path, check)` now takes its check, the home through `check_read`, on every OS (report.md:18, report.md:26).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 65
    section: '## Authentication & Authorization'
    change: 'Add a row after this one. **Requirement:** IPC client-side server verification, the euid check. **Implementation:** `crate::client::peer_euid_of` reads `peer_creds().ok()?.euid()` on Unix and nothing on Windows, where the DACL and the pid comparison stay the controls; `Client::server_euid` and `TokioClient::server_euid` read it, writing and logging nothing. `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, with `PEER_EUID_REPORTED` = `cfg!(unix)`, so a peer it cannot read is refused where reported; `peer_admitted(observed)` is the decision every seam reads. A refused euid sends no frame and reads `server-verify-failed`: `CallError::ServerUnverified` at the CLI, `NotSent::ServerUnverified` at the hook, `UnreachableCause::ServerVerify` at MCP. That word now reaches Linux, Windows and macOS, on macOS meaning an euid mismatch alone; a client meeting a server of another euid reading it on Linux and macOS is the founder''s own word (`inputs#I2` Q2). **Stack reference:** `viola-channel` `client.rs`, `tokio_client.rs`; root bin `src/cmd/client.rs`, `src/cmd/hook.rs` (`connect`); `crates/viola-mcp/src/tools.rs` (`frame`); interprocess 2.4.4.'
    sidecar: 'New row: the client euid check, its closed decision and `server-verify-failed`''s reach onto macOS.'
    rationale: 'New detail (report.md:12, report.md:15, report.md:16, report.md:17), the word''s reach (report.md:31), the founder''s Q2 (report.md:93).'
    basis: crates/viola-channel/src/client.rs:63
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 77
    section: '## Authentication & Authorization'
    change: 'Add a row after this one. **Requirement:** IPC client-side server verification, the CLI socket-directory refusal. **Implementation:** `client::connect`, unedited for its eight callers (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`), runs `check_endpoint_dir` on the recorded endpoint before `Client::connect`, `peer_admitted` after it, then `verify_server`. A refused directory is `CallError::SocketDirRefused` → `Unreachable::SocketDir`: no frame and no `channel-request`, exit 21 `instance-unreachable` with `detail` `socket-dir-refused`, one `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` line at WARN, the hint the `socket-dir` cause, `hint:` last, a planning form, the founder''s wording owed at Epoch 7''s boundary batch; no path, pid or uid. **Stack reference:** root bin `src/cmd/client.rs` (`connect`, `Unreachable::SocketDir`); `src/human/hints.rs` (the `socket-dir` cause).'
    sidecar: 'New row: the CLI frame verbs'' socket-directory refusal, exit 21 `socket-dir-refused`.'
    rationale: 'New detail (report.md:15, report.md:19, report.md:71); wording''s standing (report.md:94).'
    basis: src/cmd/client.rs:173
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 77
    section: '## Authentication & Authorization'
    change: 'Add a row after the CLI socket-directory refusal. **Requirement:** IPC client-side server verification, the hook''s socket-directory refusal. **Implementation:** `src/cmd/hook.rs`''s `connect` runs the same checks in the same order. A refused directory is `NotSent::SocketDir` (`Asked::SocketDir` on the dialog path): exit 0, empty streams, no frame, one `hook-decision{detail:"socket-dir-refused"}` line at WARN, on the dialog path `hook-invoked{corr:null}` first and `decision_emitted:false`; no path, pid or uid. **Stack reference:** root bin `src/cmd/hook.rs` (`connect`, `NotSent::SocketDir`, `Asked::SocketDir`).'
    sidecar: 'New row: the hook''s socket-directory refusal, failing open with one `hook-decision` line.'
    rationale: 'New detail (report.md:16, report.md:72).'
    basis: src/cmd/hook.rs:231
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 79
    section: '## Authentication & Authorization'
    change: 'Replace «and a failed `server_verified` to `server-verify-failed`» with «a refused `check_endpoint_dir`, run after `live_endpoint`, to `socket-dir-refused`, and `!peer_admitted(client.server_euid()) || !server_verified(…)` to `server-verify-failed`».'
    sidecar: 'MCP refusal: `frame` gains `socket-dir-refused` and the euid read inside its `server-verify-failed` mapping.'
    rationale: 'MCP `frame` (report.md:17, report.md:73); pattern `causes`, a list extends (report.md:28).'
    basis: crates/viola-mcp/src/tools.rs:345
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 85
    section: '## Authentication & Authorization'
    change: 'Replace the Implementation with: The macOS cases are built: the per-user socket directory loosened to 0755 giving exit 21, the client euid check and `socket-dir-refused`, read on `macos-latest` in `tests/security_negatives_socket_dir.rs`. macOS''s pid and start-time comparison stays absent and named, `server_verdict_rows` keeping its row, a recorded gap owned by no route entry. The server-identity negatives compile out on macOS.'
    sidecar: 'macOS cases: the CARRY on "Unix endpoint and home hardening" discharged; the pid gap stays named, owned by no route entry.'
    rationale: 'Retired (report.md:26), owner clauses (report.md:33), macOS acceptance (report.md:107, report.md:110, report.md:113).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:110
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 86
    section: '## Authentication & Authorization'
    change: 'Extend the Stack reference with «; `tests/security_negatives_socket_dir.rs` (`#![cfg(unix)]`)».'
    sidecar: 'macOS cases: the stack reference names the new negatives binary.'
    rationale: 'The macOS witness landed in the new binary, compiled on Unix only (report.md:64, report.md:128).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:128
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 86
    section: '## Authentication & Authorization'
    change: 'Add a row after this one. **Requirement:** IPC socket-directory and euid checks, the tests holding them and what they leave. **Implementation:** `security_negatives_socket_dir` on `ubuntu-latest` and `macos-latest` reads a directory at 0755 or a symlink to 0700: `run` exit 1, no bind, no child; a frame verb exit 21, no frame; a hook exit 0, empty streams, no frame; the directory as found. `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit` reads a foreign owner by injected uid; `serve_drops_a_refused_peer_before_any_frame` the wrapper''s drop, `channel_endpoint_answers_protocol_faults` its control; `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user` the decision and the real read; `handlers_frame_tools_refuse_a_loosened_socket_directory` MCP; `setting_does_not_disable_the_socket_dir_check` no setting switching it off. Not proved: a foreign euid and owner are never driven, and each client seam''s euid wiring was read only once, by a forced refusal in the chunk''s remove-the-guard runs, which no committed test repeats, so an always-admit mutant survives: one account can make no foreign peer and no seam may be added. Owner: a dated note on `matrix#v1-44` at P7.3 and test-plan §1''s untestable zone. **Stack reference:** the tests named; `tests/security_negatives_socket_dir.rs`, `crates/viola-channel/src/server.rs`, `crates/viola-channel/src/client.rs`, `crates/viola-state/src/strict.rs`.'
    sidecar: 'New row: the witnesses of the socket-directory and euid checks, with the seam wiring''s weakest proof and its owner named.'
    rationale: 'Acceptance (report.md:107, report.md:108, report.md:109), the weakest proof and this wrap''s owner ruling (report.md:54), what the tests do not prove (report.md:130).'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
note: 'D-security-input and D-security-deps read no drift (closed peer decisions, no new external parser; Dependencies none added, report.md:23). Beyond this file, the body''s §Authentication & Authorization server-verification line, §Security Anti-Patterns Authentication and §Error Handling''s cause list still read the owed clauses and the five causes, outside this proposal list.'
