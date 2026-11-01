proposals:
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → Untestable zones
    change: 'Extend the cross-OS-user zone: the wiring of the euid check at each client seam (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) has no committed test, since one account can make no foreign peer and no seam may be added; a mutant replacing a seam `peer_admitted` read with an always-admit survives every committed test by construction; the decision over injected euids, the real read on Linux and macOS, and the wrapper drop over an injected decision are the committed tests.'
    sidecar: '§1 Untestable zones: names the euid wiring at the three client seams, read once by forced refusals and held by no committed test (chunk 2026-11-01-unix-endpoint-and-home-hardening).'
    rationale: 'The report, Insufficient fixes, rules the remainder of the weakest proof to test-plan §1 untestable zone naming the wiring; Coverage of new surfaces reads the same for the client euid check. The three seams are new paths with no test at the mandated tier, so the plan names the gap.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy → Skipped as untestable
    change: 'The Cross-OS-user peers row keeps its coverage by the peer-credential boundary unit test (§4) and the same-user SDDL read-back (§5), and adds that the euid wiring at the three client seams is held by no committed test (§1 Untestable zones).'
    sidecar: '§6 Skipped as untestable: the cross-user row states the seam wiring as uncovered, matching §1.'
    rationale: 'This row restates the §1 zone and reads the boundary unit test as the whole coverage; the report, Insufficient fixes, has that test hold the decision and not the wiring.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → Coverage triggers → security-vector-coverage (IPC)
    change: 'Beside `Unix socket dir 0700 and file 0600 asserted`, add the required negatives: a socket directory loosened to 0755 or a symlink to a 0700 directory refused and left as found (`run` exit 1 with no bind and no child, a frame verb exit 21 with no frame, the hook exit 0 with empty streams and no frame, MCP `isError` with no frame), a foreign owner at the verdict by injected uid; and a peer of another euid decided by injected euids, a client reading `server-verify-failed` and the wrapper dropping the peer before any frame.'
    sidecar: '§1 IPC trigger: the socket-directory refusal and the peer euid decision join the required negatives.'
    rationale: 'The report, Outcome (security rows) and Coverage of new surfaces, states these as the new negatives of the chunk; the trigger still lists only the modes.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107-109
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: 'In `cleanup` step 4, on Unix the endpoint reads gone when a connect to the `<per-user 0700 dir>/viola-<h12>.sock` path is refused or the path is absent, in place of the path no longer existing; after a force-killed wrapper the leftover socket file stays until a next bind of that name takes it over, cleanup removes nothing of it, and the document reads `endpoint_gone` true and `ok:true` (`cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone`, `unconnectable_reads_a_leftover_socket_file_no_one_listens_on`). The Windows arm and the directory list stay.'
    sidecar: '§3 cleanup step 4: Unix `endpoint_gone` reads a connect refused or a path absent; the leftover file of a force-killed wrapper no longer fails cleanup.'
    rationale: 'The report, Spec claims disproved and Harness / gate surface, measures the file-absent reading false after a force-killed wrapper and changes the rule in `crates/viola-e2e/src/harness/cleanup.rs`. The document shape is unchanged and obs-plan states no `endpoint_gone` rule, so test-plan §3 is the one stale side.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:57
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: 'In the clause on `Wrapper::stop` / `stop_keep`, `unconnectable` as the harness `endpoint_gone` rule now reads on Unix a connect refused or the path absent, not the socket path gone; so on Unix it reads what `holder_gone` reads, the kill path keeping `holder_gone` and its measurement staying as the record of the older rule.'
    sidecar: '§3 `run` fixtures clause: `unconnectable` on Unix restated beside `holder_gone`.'
    rationale: 'The report, Counts / qualifiers moved, by-line dispositions, rules this row changes on `holder_gone` versus `unconnectable`, and Symbols / APIs states the new Unix reading; the row restates the claim step 4 retires.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:40
    dependent-of: D-tests-obs-harness
  - detector: D-tests-count-literals
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project)
    change: 'In `[profile.ci]`, the `twenty-five verify-driven binaries` reads twenty-six, and the `binary(…)` names on that filter line gain `security_negatives_socket_dir`, joined at chunk 2026-11-01-unix-endpoint-and-home-hardening.'
    sidecar: '§3 bootstrap `[profile.ci]`: the kill line names twenty-six binaries, `security_negatives_socket_dir` added.'
    rationale: 'The report, Schema / config and Counts / qualifiers moved, moves the terms of this line 25 → 26 with this binary. It records no row stating the count, but this row spells the count in words and lists all 25 names as `.config/nextest.toml` reads, so the pattern did not reach it.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:24
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover → viola-channel
    change: 'After the peer-credential decision with injected euids, name `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, the real read on each sync and Tokio client (`client_server_euid_is_this_user`, `tokio_client_server_euid_is_this_user`), and the wrapper dropping a refused peer before any frame (`serve_drops_a_refused_peer_before_any_frame`, `channel_endpoint_answers_protocol_faults` its control).'
    sidecar: '§4 viola-channel: the peer euid decision, the real read and the wrapper drop named as built.'
    rationale: 'The report, Symbols / APIs and Outcome, adds these paths to `viola-channel` with these cases; §4 is listed under the expected amendments.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:108-109
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover → viola-state
    change: 'Add the socket-directory check (`socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found`, `check_endpoint_dir_reads_the_endpoints_directory`, `check_socket_dir_off_unix_reads_nothing`) and the board scan holding the home to the read rule (`board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance`, on `windows-2025` `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`).'
    sidecar: '§4 viola-state: the socket-directory check and the board read rule join the unit coverage.'
    rationale: 'The report, Symbols / APIs and New text, by line, adds `check_socket_dir`, `check_endpoint_dir`, `socket_dir_verdict` and the changed `scan` with these cases; the Retired bullet retires the two rules on the `viola list` home.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:112
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover → root bin
    change: 'Beside the server check refused inside each span, add the socket-directory check refused inside the frame verbs dispatch span and inside the hook handle (`connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span`, `hook_refuses_a_loosened_socket_directory_inside_hook_handle`), each case running on macOS as well.'
    sidecar: '§4 root bin: the socket-directory refusal inside the frame verb and hook spans.'
    rationale: 'The report, Deviations from intent 4 and 5 and New text, by line, adds these cases to `src/cmd/client.rs` and `src/cmd/hook.rs`, and moves the span recorder so the cases that run on macOS can read it.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:82-83
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns → MCP
    change: 'Add that the handler tests hold the MCP frame tools refusing a loosened socket directory (`handlers_frame_tools_refuse_a_loosened_socket_directory`: `isError` with `socket-dir-refused`, no frame and no line).'
    sidecar: '§5 MCP: the handler case for a refused socket directory.'
    rationale: 'The report, Symbols / APIs on `crates/viola-mcp/src/tools.rs` `frame` and Coverage of new surfaces, names this case as the test of the new MCP path.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:73
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns → CLI
    change: 'Add that `tests/cli_list.rs` holds the board on the read rule on all three OSes (`list_refuses_a_home_another_user_may_read_with_and_without_an_instance`), and `tests/cli_controls_not_disableable.rs` holds that no setting disables the socket-directory check (`setting_does_not_disable_the_socket_dir_check`).'
    sidecar: '§5 CLI: the `viola list` read rule and the undisableable socket-directory check.'
    rationale: 'The report, Outcome (arch) and New text, by line, names these cases for the board rule and the new control; the §5 CLI row already claims no setting disables a control.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:112
  - detector: D-retired
    severity: warning
    section: §6 E2E Test Strategy → Exit-cause matrix
    change: 'Drop `its macOS case owed to the route entry "Unix endpoint and home hardening"`: the server-identity cases stay compiled out on macOS, the comparison there absent and named by `server_verdict_rows`, and the owed macOS case, a loosened socket directory reading exit 21, landed in `tests/security_negatives_socket_dir.rs` (Linux and macOS, `#![cfg(unix)]`). Add the sixth cause to the table: `socket-dir-refused` by `a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused` and, for exit 1, `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`.'
    sidecar: '§6 Exit-cause matrix: the owed macOS clause discharged, `socket-dir-refused` joins the table.'
    rationale: 'The report, Retired, retires the dated gaps owed to this route entry; Expected amendments places the macOS case in the new binary; Counts / qualifiers moved adds the sixth cause; Outcome keeps the server-identity negatives compiled out on macOS.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:64
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy → Security control negatives
    change: 'Name the holder of the Unix socket directory refusal, `tests/security_negatives_socket_dir.rs` (Linux and macOS): a directory at 0755 or a symlink to a 0700 directory refuses `viola run` exit 1 before any bind, the eight frame verbs exit 21 with no frame and the hook exit 0 with empty streams and no frame, each left as found and naming no path, pid or uid, and a clean start under an open umask keeps the socket private. Beside `a pre-existing home another user may read refusing viola run exit 1`, add `viola list` by `list_refuses_a_home_another_user_may_read_with_and_without_an_instance`. The Linux and Windows clause on `tests/security_negatives_server_identity.rs` stays.'
    sidecar: '§6 Security control negatives: `security_negatives_socket_dir` named with its cases, and the `viola list` read rule added.'
    rationale: 'The report, Crates / modules, Outcome and Counts / qualifiers moved (pattern `neg-binary`), adds this binary; Expected amendments lists this suite and places the hook rows in the new binary.'
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107
note: 'D-tests-framework holds: no runner or framework moved. The new-socket-root helpers and the bare-temp-dir reading fall in existing detail rows, the cascade reach, and no proposal here touches a detail file; the kill-line count row is the one literal the report recorded as stating no count.'
