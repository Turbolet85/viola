proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 10
    section: "## 4. Unit Test Strategy"
    change: "`viola_core::UnreachableCause` holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost`, `ServerVerify` and `SocketDir` and has no serde path; `unreachable_cause_is_each_its_own_code_word` holds its words, the sixth `socket-dir-refused` (`crates/viola-core/src/lib.rs`)."
    sidecar: "§4 viola-core: `UnreachableCause`'s list extended with `SocketDir`, the sixth cause (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    rationale: "The row lists the five causes; the chunk adds `UnreachableCause::SocketDir` as the sixth, and a row listing the five extends (report.md:10, report.md:28)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 90
    section: "## 4. Unit Test Strategy"
    change: "In the server-check row, replace the `unreachable_logs_its_cause` clause with: `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs the closed `Unreachable` causes `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` (`endpoint-lost` `during` `call`, the other three `during` `connect`); `Unreachable` holds a fifth closed cause since chunk 2026-11-01-unix-endpoint-and-home-hardening, `socket-dir-refused` (`Unreachable::SocketDir`), `during` `connect`."
    sidecar: "§4 root bin: the closed `Unreachable` causes gain `socket-dir-refused`, `during` `connect`."
    rationale: "The row reads as the whole closed set of `Unreachable`; `CallError::SocketDirRefused` now maps to `Unreachable::SocketDir`, `during` `connect` (report.md:15), the same move as the cause list (report.md:28)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-tests-count-literals
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 18
    section: "## 4. Unit Test Strategy"
    change: "  - The peer-euid decision `peer_euid_admitted(observed, own, reported)` (`crates/viola-channel/src/client.rs`), `!reported || observed == Some(own)`, so a peer it cannot read is refused where euids are reported (`PEER_EUID_REPORTED` = `cfg!(unix)`), `peer_admitted(observed)` its reading against this process's own euid: held with injected euids by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, covering the cross-user rejection boundary from test-scope Sec 1."
    sidecar: "§4 viola-channel: the peer-credential decision named as built, `peer_euid_admitted` and its unit case."
    rationale: "The row describes the decision by a signature the chunk built otherwise; the report names the function, its rule and its case (report.md:12, report.md:109, report.md:152)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 20
    section: "## 4. Unit Test Strategy"
    change: "  - The connected stream's peer euid, `Client::server_euid` and `TokioClient::server_euid` over the crate-private `peer_euid_of` (`peer_creds().ok()?.euid()` on Unix, none on Windows, where the DACL and the pid comparison stay the controls), writing and logging nothing, a real read on Linux and macOS: `client_server_euid_is_this_user` and `client_server_euid_is_none_where_the_peer_reports_no_euid` (`crates/viola-channel/src/client.rs`) and their Tokio twins `tokio_client_server_euid_is_this_user` and `tokio_client_server_euid_is_none_where_the_peer_reports_no_euid` (`tokio_client.rs`)."
    sidecar: "§4 viola-channel: the connected stream's peer euid read and its four unit cases added."
    rationale: "New API and unit cases beside the server pid row (report.md:12, report.md:109, report.md:153-154, report.md:166-167)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 20
    section: "## 4. Unit Test Strategy"
    change: "  - The wrapper's drop (`crates/viola-channel/src/server.rs`): `Server::serve` calls the private `serve_admitting(dispatch, peer_admitted)`, and `accept_loop` filters `incoming()` by `admitted(peer_euid_of(stream))`, so a refused stream is dropped on the accept thread, before its worker thread and before any frame, the peer reading end-of-file, no line written; `serve_drops_a_refused_peer_before_any_frame` injects the decision (`type Admission = fn(Option<u32>) -> bool`, no env, flag or feature seam), `channel_endpoint_answers_protocol_faults` its control."
    sidecar: "§4 viola-channel: the wrapper's peer drop in `accept_loop` and its injected-decision unit case added."
    rationale: "New path with its unit case and control, dependency injection the seam (report.md:13, report.md:75, report.md:108)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 20
    section: "## 4. Unit Test Strategy"
    change: "  - Not held by a committed test: each client seam's euid wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`), a refused euid reading `server-verify-failed` (on macOS an euid mismatch alone). One account can make no foreign peer and no seam may be added, so a mutant replacing a seam's `peer_admitted` read with an always-admit survives; the decisions over injected euids and the wrapper's drop over an injected decision are the committed tests, and §1's untestable zones name the wiring (test-scope-summary)."
    sidecar: "§4 viola-channel: the client seams' euid wiring recorded as outside the committed unit tests, bound to §1's untestable zone."
    rationale: "The seams are new paths whose wiring was read only once, through forced refusals no committed test repeats; the report owns the remainder at §1 and `matrix#v1-44`, so §4's decision rows must not read as covering it (report.md:54, report.md:74, report.md:109)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 35
    section: "## 4. Unit Test Strategy"
    change: "  - The socket-directory check (`crates/viola-state/src/strict.rs`): the pure `socket_dir_verdict(is_symlink, is_dir, mode, uid, euid)` reads a symlink `Refused::Symlink`, not a directory `Refused::NotADirectory` «a trusted state path is not a directory», else `unix_read_verdict` (`socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, a foreign owner by injected uid); `check_socket_dir` takes one `symlink_metadata`, an absent path passing and an unstattable one `Refused::Unreadable`, and never chmods (`check_socket_dir_reads_the_path_itself_and_leaves_it_as_found`), off Unix returning `Ok(())` (`check_socket_dir_off_unix_reads_nothing`); `check_endpoint_dir` reads a recorded endpoint's parent (`check_endpoint_dir_reads_the_endpoints_directory`)."
    sidecar: "§4 viola-state: the socket-directory check, its pure verdict and its four unit cases added."
    rationale: "New symbols and unit cases, the verdict's foreign owner held only by an injected uid (report.md:11, report.md:107, report.md:201-204)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 35
    section: "## 4. Unit Test Strategy"
    change: "  - The board's home rule (`board::scan`, `crates/viola-state/src/board.rs`): `check_if_present(path, check)` takes its check, the home through `check_read` and `instances/` through `check_path`, on every OS (`inputs#I2` Q3, the overseer's answer as delegate, PROVISIONAL): `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and, on `windows-2025`, `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`."
    sidecar: "§4 viola-state: `board::scan` holds the home to the read rule, its two unit cases added, the rule PROVISIONAL."
    rationale: "`viola list`'s home moved from two rules to the read rule, with new unit cases; the strict-modes row's «two rules» stays true (report.md:18, report.md:32, report.md:112, report.md:188-189)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 90
    section: "## 4. Unit Test Strategy"
    change: "  - The socket-directory check before any frame on the CLI: `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` (`src/cmd/client.rs`) opens the span by hand (`wait.block`) around `connect`, where the check sits for its eight callers; the span recorder `Seen` stands in the test module `refused_seen`, out of the macOS-gated `refused_inside`, so this case runs on macOS."
    sidecar: "§4 root bin: the frame verbs' socket-directory unit case and the `refused_seen` recorder module added."
    rationale: "New unit case and test module, both deviations the report records (report.md:15, report.md:82-83, report.md:212)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:83
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 91
    section: "## 4. Unit Test Strategy"
    change: "  - The `socket-dir` hint cause is built in `src/human/hints.rs`, its pair `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions`, held by `send_hint_is_the_design_string`, `no_hint_names_release`, `no_two_codes_share_a_hint` and `every_code_is_a_code_word`; the wording is a planning form, the founder's own owed at Epoch 7's boundary batch (`inputs#I3`, `inputs#I4`)."
    sidecar: "§4 root bin: the `socket-dir` hint cause and its holders added, the wording a planning form."
    rationale: "New hint cause beside the row on the other causes' hints, its witnesses named by the report (report.md:19, report.md:49, report.md:76, report.md:94)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 92
    section: "## 4. Unit Test Strategy"
    change: "  - The hook's socket-directory check (`src/cmd/hook.rs` `connect`, ahead of its euid and server checks in the CLI's order): `hook_refuses_a_loosened_socket_directory_inside_hook_handle` reads `NotSent::SocketDir` / `Asked::SocketDir`, `detail` `socket-dir-refused`, one `hook-decision` at WARN, with exit 0, empty streams and no frame; through the `refused_seen` recorder it runs on macOS."
    sidecar: "§4 root bin: the hook's socket-directory unit case added."
    rationale: "New closed variants and their unit case in the hook's span (report.md:16, report.md:72, report.md:82, report.md:217)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:16
note: "§4's body broad lines (viola-channel, viola-state, root bin) name neither the socket-directory check nor the board's read rule, a body edit outside this file; the report does not state whether `unreachable_logs_its_cause` gained a `socket-dir-refused` row, so the near-90 edit is worded true either way; `unconnectable`'s own unit case is left to §3's `cleanup` contract."
