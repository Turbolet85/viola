proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 12
    section: "## 1. Test Scope Summary"
    change: "Replace the last sentence with: These negatives compile out on macOS, where the client reads no peer pid, a named carve-out owned by no route entry; there `server-verify-failed` means an euid mismatch alone, the decision and the real read held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`."
    sidecar: "§1 viola-channel server verification: the macOS clause owed to the route entry is discharged; macOS's absent pid comparison stays a named carve-out with no owner, and macOS reads an euid mismatch as server-verify-failed."
    rationale: "The report's Retired bullet retires the dated owner, and its owner clauses all change except macOS's pid gap, which stays named but owned by no route entry (report.md:26, report.md:31, report.md:33); the server-identity negatives stay compiled out on macOS (report.md:110)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 12
    section: "## 1. Test Scope Summary"
    change: "Add after this line:   - **As landed (the euid wiring, untestable):** each client seam's euid wiring (`connect` in `src/cmd/client.rs` and `src/cmd/hook.rs`, `frame` in `crates/viola-mcp/src/tools.rs`) has no committed witness, since one account can make no foreign peer and no seam may be added; it was read once, through forced refusals in the run's remove-the-guard evidence rows 7, 8 and 9. The decisions over injected euids and the wrapper's drop over an injected decision are committed tests."
    sidecar: "§1 viola-channel: the client seams' euid wiring is named as untestable on one account, read once through forced refusals."
    rationale: "The report's Insufficient fixes rules that test-plan §1's untestable zone names the wiring, and the cross-user part this entity reasons about is the same boundary (report.md:54, report.md:130)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 88
    section: "## 1. Test Scope Summary"
    change: "Append to the Cross-OS-user attacks zone: so each client seam's euid wiring has no committed witness, read once through forced refusals; the decisions are held by injected euids."
    sidecar: "§1 untestable zones: the cross-OS-user zone names the client seams' euid wiring."
    rationale: "The report's ruling names test-plan §1's untestable zone as the owner of this remainder, and this body list is that zone (report.md:54)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 34
    section: "## 1. Test Scope Summary"
    change: "In the `<code>` list of `revive --json`'s exit-1 document, add `socket-dir-refused` after `squatted-name`, so the list holds eleven codes."
    sidecar: "§1 CLI verbs: revive --json's exit-1 code list gains socket-dir-refused."
    rationale: "`revive --json`'s exit-1 codes moved ten to eleven and each list holding all ten extends; `revive`'s start shares the refusal (report.md:14, report.md:29)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 43
    section: "## 1. Test Scope Summary"
    change: "«its ten codes» now reads «its eleven codes»."
    sidecar: "§1 cli surface: revive --json's exit-1 codes ten to eleven."
    rationale: "The report names this literal as moved, its pattern `revive-codes` (report.md:29)."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:43
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 69
    section: "## 1. Test Scope Summary"
    change: "In the `instance-unreachable` `<cause>` list, add `socket-dir-refused` after `server-verify-failed`; the clause `the CLI exit-21 document less `v`` stands."
    sidecar: "§1 MCP surface: the instance-unreachable cause list gains socket-dir-refused."
    rationale: "Exit 21's closed causes moved five to six and a row listing the five extends; MCP's `frame` reads the new word as the CLI document less `v` (report.md:17, report.md:28, report.md:111)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 101
    section: "## 1. Test Scope Summary"
    change: "Replace `and the Unix socket file or pipe no longer exists` with: and on Unix a connect to the socket path is refused or the path is absent, the pipe no longer existing on Windows; `cleanup` removes no leftover file, so a force-killed wrapper's file stays until a next bind of that name takes it over."
    sidecar: "§1 cleanup requirement: endpoint gone on Unix reads a connect refused or a path absent, measured false by file absence after a force-killed wrapper."
    rationale: "The report's disproved claim: reading the socket file's absence is false after a force-killed wrapper, now a connect refused or a path absent through `unconnectable`, removing no leftover (report.md:20, report.md:42, report.md:57, report.md:95)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:57
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 128
    section: "## 1. Test Scope Summary"
    change: "In the sequence, before `exclusive endpoint bind`, add `the socket directory check (`socket_dir_ready`: the endpoint's parent checked, created only when absent, then checked again)`; the closing sentence reads: A second `run` of the same name, a squatted endpoint or a refused socket directory exits 1."
    sidecar: "§1 Path 1: the start sequence gains the socket directory check before the endpoint bind."
    rationale: "`socket_dir_ready` runs before any snapshot, child or `Server::bind`, and a refused directory exits 1 (report.md:14, report.md:70)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 134
    section: "## 1. Test Scope Summary"
    change: "Add after this line:     - A refused socket directory (at 0755, or a symlink to 0700) exits 1 with no bind and no child, left as found, on `viola run` and on `revive`'s start, with one `process-exit` line, `detail` `socket-dir-refused` (`tests/security_negatives_socket_dir.rs` `security_negative_a_start_refuses_the_socket_directory_before_any_bind`; `tests/cross_exit_causes.rs` `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`); a `Server::bind` failure stays what it was."
    sidecar: "§1 Path 1: the verification signal gains a refused socket directory's exit 1."
    rationale: "The report's acceptance and coverage rows for the start: exit 1, no bind, no child, as found, one line (report.md:14, report.md:70, report.md:107)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 135
    section: "## 1. Test Scope Summary"
    change: "In the Path 1 broad line, add `the socket directory check` before `exclusive endpoint bind`, and end: a second `run` of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree exits 1."
    sidecar: "§1 Path 1 broad line: the socket directory check before the bind, its refusal exit 1."
    rationale: "The broad line of the entity changed beside it, for the same report fact (report.md:14)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 59
    section: "## 2. Test Strategy"
    change: "In the plan location, after the `security_negatives_server_identity.rs` clause, add: root `security_negatives_socket_dir.rs`, Unix: a directory at 0755 or a symlink to 0700 refusing a start, the eight CLI frame verbs and the hook, each left as found; in its §5 clause add `handlers_frame_tools_refuse_a_loosened_socket_directory`; the two `off macOS` clauses stand, naming the impostor cases."
    sidecar: "§2 IPC trigger row: gains the socket directory negatives and the MCP refusal."
    rationale: "The new binary and the MCP case are the IPC vector's new coverage; the hook's rows live in the new binary; the server-identity negatives stay compiled out on macOS (report.md:39, report.md:73, report.md:95, report.md:107, report.md:110)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 64
    section: "## 2. Test Strategy"
    change: "After the closing parenthesis of the `security_negatives_strict_modes.rs` clause, add: ; root `cli_list.rs` `list_refuses_a_home_another_user_may_read_with_and_without_an_instance`: that same home refusing `viola list`, with and without an instance."
    sidecar: "§2 Filesystem trigger row: the read rule's refusal now names viola list."
    rationale: "`viola list`'s home moved from two rules to the read rule on every OS, and this case witnesses it (report.md:18, report.md:32, report.md:112)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:112
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 74
    section: "## 2. Test Strategy"
    change: "Beside root `cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text`, add its sibling `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text`; the surfaces stay three, and `contract_error_bodies.rs`'s enumeration holds the new document's row."
    sidecar: "§2 error sanitization row: the CLI surface gains the socket directory witness."
    rationale: "The report's new text adds this case and a `Body` row to `tests/contract_error_bodies.rs`, and an enumerating row extends (report.md:8, report.md:116)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:8
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 10
    section: "## 4. Unit Test Strategy"
    change: "`viola_core::UnreachableCause` holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost`, `ServerVerify` and `SocketDir`, the last with the word `socket-dir-refused`, and has no serde path; the rest of the line stands."
    sidecar: "§4 viola-core: UnreachableCause gains its sixth variant SocketDir."
    rationale: "The sixth cause is added with its word and no serde path or `RefusalReason` (report.md:10, report.md:28)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 18
    section: "## 4. Unit Test Strategy"
    change: "Add after this line:   - The peer euid decision and read: `peer_euid_admitted(observed, own, reported)`, where a peer it cannot read is refused where the euid is reported, held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`; `client_server_euid_is_this_user` and `tokio_client_server_euid_is_this_user`, with their siblings `client_server_euid_is_none_where_the_peer_reports_no_euid` and `tokio_client_server_euid_is_none_where_the_peer_reports_no_euid` (`crates/viola-channel/src/client.rs`, `tokio_client.rs`)."
    sidecar: "§4 viola-channel: the peer euid decision and the real euid read land as unit cases."
    rationale: "The report's new symbols and new text name these cases, the decision's rule and the read's seam (report.md:12, report.md:109)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
    fact: F18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 18
    section: "## 4. Unit Test Strategy"
    change: "Add after the peer euid row:   - The wrapper's drop: `serve_drops_a_refused_peer_before_any_frame` (`crates/viola-channel/src/server.rs`) holds a refused peer dropped on the accept thread, before any worker or frame, the peer reading end-of-file with no line written and nothing on the terminal; `channel_endpoint_answers_protocol_faults` is its control."
    sidecar: "§4 viola-channel: the wrapper's drop of a refused peer lands as a unit case."
    rationale: "The report's server change and acceptance name the drop and its control (report.md:13, report.md:75, report.md:108)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
    fact: F19
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 380
    section: "## 4. Unit Test Strategy"
    change: "After `the peer-credential decision with injected euids`, add `, the real euid read on each client, the wrapper's drop of a refused peer before any frame`."
    sidecar: "§4 viola-channel broad line: the euid read and the wrapper's drop."
    rationale: "The broad line the F18 and F19 rows detail (report.md:12, report.md:13)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:13
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 35
    section: "## 4. Unit Test Strategy"
    change: "Add after this line:   - The socket directory check (`strict::check_socket_dir`, `check_endpoint_dir`, the plain `socket_dir_verdict`): `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit` (a foreign owner by injected uid, a symlink, a non-directory reading `Refused::NotADirectory`), `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found`, `check_endpoint_dir_reads_the_endpoints_directory` and `check_socket_dir_off_unix_reads_nothing`."
    sidecar: "§4 viola-state: the socket directory verdict and its checks land as unit cases."
    rationale: "The report's new symbols and new text name the verdict, the check that never chmods and these cases (report.md:11, report.md:107)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
    fact: F16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 35
    section: "## 4. Unit Test Strategy"
    change: "Add after the socket directory row:   - The board's rule on the home: `board::scan` holds the home to the read rule and `instances/` to the path check on every OS, held by `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and, on `windows-2025`, `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`."
    sidecar: "§4 viola-state: the board's read rule on the home lands as unit cases."
    rationale: "The report's board change and acceptance name both cases; `unit-test-strategy.md:31`'s two rules stay true (report.md:18, report.md:32, report.md:112)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
    fact: F17
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 382
    section: "## 4. Unit Test Strategy"
    change: "After `the home-tree and diagnostics checks`, add `, the socket directory verdict and checks, the board's read rule on the home`."
    sidecar: "§4 viola-state broad line: the socket directory verdict and the board's read rule."
    rationale: "The broad line the F16 and F17 rows detail (report.md:11, report.md:18)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:11
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 90
    section: "## 4. Unit Test Strategy"
    change: "Add after this line:   - The socket directory check before any frame: `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` (`src/cmd/client.rs`), which opens the `wait.block` span by hand around `connect`, and `hook_refuses_a_loosened_socket_directory_inside_hook_handle` (`src/cmd/hook.rs`); the span recorder `Seen` stands in the test module `refused_seen`, out of the macOS-gated `refused_inside`, so these run on macOS."
    sidecar: "§4 root bin: the socket directory check inside the dispatch and hook spans lands as unit cases."
    rationale: "The report's new text names both cases, and its deviations name the hand-opened span and the moved recorder (report.md:82, report.md:83)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:82
    fact: F20
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 90
    section: "## 4. Unit Test Strategy"
    change: "After the `unreachable_logs_its_cause` parenthesis, add: ; `Unreachable::SocketDir` logs `socket-dir-refused`, `during` `connect`."
    sidecar: "§4 root bin: the closed Unreachable causes gain SocketDir, during connect."
    rationale: "The report maps `CallError::SocketDirRefused` to `Unreachable::SocketDir`, `during` `connect` (report.md:15, report.md:71); it does not say whether `unreachable_logs_its_cause` gained a row, so this edit claims the log line and leaves that unit's listed causes alone."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 91
    section: "## 4. Unit Test Strategy"
    change: "Add after this line:   - `send_hint_is_the_design_string` pins the hint of cause `socket-dir` in `src/human/hints.rs`, its wording a planning form with the founder's wording owed at Epoch 7's boundary batch."
    sidecar: "§4 root bin: the new hint pinned, its wording a planning form the founder still owes."
    rationale: "The report's hint change and its coverage row, with the wording owed per `inputs#I4` item 5 (report.md:19, report.md:49, report.md:76, report.md:94)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76
    fact: F21
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 393
    section: "## 4. Unit Test Strategy"
    change: "After `the server check refused inside each frame verb's and the hook's span with each closed unreachable cause logged`, add `, the socket directory check refused inside the `connect` dispatch span and the hook's`."
    sidecar: "§4 root bin broad line: the socket directory check inside the spans."
    rationale: "The broad line the F20 row details (report.md:15, report.md:16)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:15
    dependent-of: D-new-detail
note: "Body lines 186 and 254 (§1 triggers: the socket dir assertion; macOS euid + dir-only server verification) are reached by the Touches but detail no entity picked here, so no proposal is written; D-tests-obs-harness: `cleanup`'s endpoint rule moved, so test-plan §3 and obs-plan §3 need reading together; F24 goes to the new-detail pass; pick trail: not held by this detector, which read picked.g1.md only, so it stands with the run that fired `pick --draws 2`."
