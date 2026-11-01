proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 12
    section: "## 1. Test Scope Summary"
    change: "Replace the last sentence («These negatives compile out on macOS, where the client reads no peer pid; the macOS cases are owed to the route entry …») with: These negatives stay compiled out on macOS, where the client reads no peer pid: the pid comparison is absent there, a named carve-out owned by no route entry, and `server_verdict_rows` keeps its row. `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone (`client_server_euid_is_this_user`), and the macOS socket-directory case stands in `tests/security_negatives_socket_dir.rs` on `macos-latest`."
    sidecar: "§1 viola-channel, As landed (server verification): the owed macOS clause discharged; macOS's pid gap stays a named carve-out with no route-entry owner, and `server-verify-failed`'s reach reads Linux, Windows and macOS."
    rationale: "Report Retired names the gaps owed to the route entry as built (pattern owner-title); Counts / qualifiers moved keeps macOS's pid gap named but owned by no route entry and moves the verify-word reach (each row changes); Outcome reads the server-identity negatives as staying compiled out on macOS."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:33
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 43
    section: "## 1. Test Scope Summary"
    change: "«(its ten codes)» reads «(its eleven codes)»."
    sidecar: "§1 cli surface: `revive --json`'s exit-1 codes ten to eleven, with `socket-dir-refused`."
    rationale: "Report Counts / qualifiers moved, pattern revive-codes, names this site by line: `revive --json`'s exit-1 codes move ten to eleven."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 34
    section: "## 1. Test Scope Summary"
    change: "`revive`'s list of `<code>` words gains `socket-dir-refused`, set after `squatted-name` and before `internal-error`, the list then holding eleven."
    sidecar: "§1 CLI verbs entity: `socket-dir-refused` added to `revive --json`'s list of exit-1 codes."
    rationale: "Report pattern revive-codes: each list that holds all ten extends; this list holds all ten, and `revive`'s start shares the socket-directory refusal."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:29
    dependent-of: D-tests-count-literals
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 69
    section: "## 1. Test Scope Summary"
    change: "The `<cause>` list of `instance-unreachable` reads `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` · `socket-dir-refused`, still the CLI exit-21 document less `v`."
    sidecar: "§1 MCP surface: exit 21's closed causes five to six, `socket-dir-refused` added to the `<cause>` list."
    rationale: "Report Counts / qualifiers moved, pattern causes: exit 21's closed causes five to six; a row holding a list of the five extends, and the report gives the list's words."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:28
  - detector: D-tests-obs-harness
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 101
    section: "## 1. Test Scope Summary"
    change: "«and the Unix socket file or pipe no longer exists» reads: and on Unix a connect to the socket is refused or its path is absent (`unconnectable`), or on Windows the pipe no longer exists; after a force-killed wrapper `endpoint_gone` is true while the socket file stays, nothing removing it, until a next bind of that name takes it over (`cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone`, `unconnectable_reads_a_leftover_socket_file_no_one_listens_on`)."
    sidecar: "§1 5-command requirements, `cleanup`: the endpoint-gone reading moves from the socket file's absence to a connect refused or a path absent, as measured after a force-killed wrapper."
    rationale: "Report Spec claims disproved: reading `endpoint_gone` by the socket file no longer existing was measured false after a force-killed wrapper on Unix; Harness / gate surface states the new reading with the document's shape unchanged; the choice to leave the file is `inputs#I3`'s third Beyond-the-plan row."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:57
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 11
    section: "## 1. Test Scope Summary"
    change: "Append to the reason: Each client seam's euid wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) has no committed test: one account can make no foreign peer and no seam may be added, so an always-admit mutant of a seam's `peer_admitted` read survives every committed test by construction. The decisions over injected euids and the wrapper's drop over an injected decision are committed tests; the wiring was read once, through a forced refusing decision at chunk 2026-11-01-unix-endpoint-and-home-hardening (`evidence/remove-the-guard.md`), its remainder dated on `matrix#v1-44`, and the mutation audit reads the same survivor at the route entry «Version done-check»."
    sidecar: "§1 viola-channel, reason if not fully testable: the three client seams' euid wiring named as untestable by a committed test, with what does hold it and its owner."
    rationale: "Report Insufficient fixes records the new path with no committed test, and this wrap's ruling makes test-plan §1's untestable zone name the wiring; Coverage of new surfaces reads the client euid check the same way."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 13
    section: "## 1. Test Scope Summary"
    change: "New sub-item after this line:   - **As landed (the peer euid):** `viola_channel::peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer it cannot read is refused where `PEER_EUID_REPORTED` (`cfg!(unix)`) holds; on Windows the read gives none, the DACL and the pid comparison staying the controls. `Client::server_euid` and `TokioClient::server_euid` write and log nothing. `Server::serve`'s `accept_loop` filters `incoming()` by `peer_admitted`, so a refused stream is dropped on the accept thread, before its worker thread and any frame, reading end-of-file, writing no line and nothing on the terminal; the decision is injected by dependency injection, with no env, flag or feature seam. A foreign euid is not driven: units `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user`, `client_server_euid_is_none_where_the_peer_reports_no_euid`, `tokio_client_server_euid_is_this_user`, `tokio_client_server_euid_is_none_where_the_peer_reports_no_euid`, `serve_drops_a_refused_peer_before_any_frame` (its control `channel_endpoint_answers_protocol_faults`)."
    sidecar: "§1 viola-channel: new As landed record of the peer euid decision, the client read and the wrapper's drop in `accept_loop`."
    rationale: "Report Symbols / APIs brings the peer euid on both sides to the entity whose cross-user part this file flags; Outcome lists the witnesses and What the tests do not prove says a foreign euid is not driven."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 37
    section: "## 1. Test Scope Summary"
    change: "New sub-item after this line:   - **As landed (the socket directory):** all eight CLI frame verbs, through `src/cmd/client.rs` `connect`, run `check_endpoint_dir` on the recorded endpoint, then `Client::connect`, `peer_admitted(client.server_euid())` and `verify_server`. A directory at 0755 or a symlink to 0700 exits 21 with no frame and no `channel-request`, as found, `detail` `socket-dir-refused` under `--json`, one `process-exit` at WARN with `during` `connect`, and the pair `unable: the socket directory for <name> cannot be trusted` / `hint: viola's socket directory must be a directory you own, readable only by you; viola does not change its permissions` (a planning form, the founder's wording owed); a refused euid reads `server-verify-failed`. `viola hook` runs the same three checks in the same order: exit 0, empty streams, no frame, one `hook-decision` `socket-dir-refused` at WARN. A foreign owner is held at `socket_dir_verdict` by an injected uid (`security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame`, `a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused`, `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span`, `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text`, `setting_does_not_disable_the_socket_dir_check`, `security_negative_a_hook_fails_open_on_the_socket_directory`, `hook_refuses_a_loosened_socket_directory_inside_hook_handle`, `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`)."
    sidecar: "§1 CLI verbs entity: new As landed record of the socket-directory check at the eight frame verbs and the hook, with its witnesses and the pair's wording still a planning form."
    rationale: "Report Symbols / APIs and Coverage of new surfaces bring the check to the frame verbs and the hook; Outcome's security acceptance gives the behaviour; `inputs#I4` item 5 leaves the pair's wording owed."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 37
    section: "## 1. Test Scope Summary"
    change: "New sub-item after this line:   - **As landed (the board's home):** `viola list`'s `board::scan` holds the home to the read rule through `check_read` on every OS, `instances/` staying `check_path`, with its order, the whole-board refusal and no-open unchanged; PROVISIONAL on the overseer's answer as delegate (`inputs#I2` Q3), the founder's own word owed (`list_refuses_a_home_another_user_may_read_with_and_without_an_instance`, `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance`, and on `windows-2025` alone `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`)."
    sidecar: "§1 CLI verbs entity: new As landed record of `viola list`'s home under the read rule on every OS, provisional on the delegate's answer."
    rationale: "Report Symbols / APIs and Retired move `viola list`'s home to the read rule (`inputs#I2` Q3); Outcome's architecture acceptance lists the witnesses on all three OSes; Decisions records the word as PROVISIONAL."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 72
    section: "## 1. Test Scope Summary"
    change: "New sub-item after this line:   - **As landed (the socket directory and the peer euid):** `frame` runs `check_endpoint_dir` after `live_endpoint`; a refused directory returns `isError:true`, `error` `instance-unreachable` with `detail` `socket-dir-refused`, no frame and no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_loosened_socket_directory`). A peer `peer_admitted` refuses reads `server-verify-failed`, as an unverified server does."
    sidecar: "§1 MCP surface: new As landed record of the socket-directory check and the peer euid at `frame`."
    rationale: "Report Symbols / APIs for `crates/viola-mcp/src/tools.rs` `frame`, and Coverage of new surfaces naming the witness; `isError` is the record, with no line by rule."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 122
    section: "## 1. Test Scope Summary"
    change: "Append to this constraint: A test placing an endpoint in a directory of its own makes that directory the same way: `tempfile` 3.27 leaves a `tempfile::tempdir()`'s mode to the umask, so on a default umask it reads 0755 and the socket-directory check refuses it."
    sidecar: "§1 test data strategy: a test-made socket directory goes through `create_private_dir`, as a home does."
    rationale: "Report Spec claims disproved measured the bare temp dir at 0755, the client check refusing nine `handlers` cases on the first run, and four sites narrowed through `viola_state::fs::create_private_dir`; Decisions names it a sweep hazard."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:56
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 128
    section: "## 1. Test Scope Summary"
    change: "«exclusive endpoint bind» reads «the socket-directory check (`socket_dir_ready`: `check_endpoint_dir`, the parent created only when absent, then the check again, so a race's loser reads the same), exclusive endpoint bind»."
    sidecar: "§1 Path `run` start sequence: the socket-directory check named ahead of the exclusive bind."
    rationale: "Report Symbols / APIs for `src/cmd/run.rs`: the refusal comes before any snapshot, child or `Server::bind`, and `revive`'s start shares it."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 134
    section: "## 1. Test Scope Summary"
    change: "Append to this signal: A socket directory at 0755 or a symlink to 0700 exits 1 with no bind and no child, leaves the directory as found and writes one `process-exit` with `subject` `self` and `detail` `socket-dir-refused`, naming no path, pid or uid; a clean start under an open umask keeps the socket private (`tests/security_negatives_socket_dir.rs` `security_negative_a_start_refuses_the_socket_directory_before_any_bind`, `security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private`; `tests/cross_exit_causes.rs` `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`)."
    sidecar: "§1 Path `run` start sequence, verification signal: the socket-directory refusal and its witnesses."
    rationale: "Report Outcome's security acceptance and Coverage of new surfaces give the exit, the absence of bind and child, the one line and its PII reading; the witnesses stand in New text, by line."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:70
note: "Outside this file: the body's §1 untestable zones (.andromeda/test-plan.md:86-90) do not yet name the client seams' euid wiring that report.md:54 assigns them, and the body's §6 Exit-cause matrix (.andromeda/test-plan.md:610) still reads its macOS case as owed to the route entry."
