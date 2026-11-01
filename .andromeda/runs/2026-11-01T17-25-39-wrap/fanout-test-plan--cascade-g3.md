proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 37
    section: "## 6. E2E Test Strategy › #### Scenario: Path 1 — `run` start sequence"
    change: "Add a sub-bullet after this one: The start checks the socket directory (`socket_dir_ready`) before any snapshot, child or `Server::bind`, and `revive`'s start shares it: a refused directory exits 1, logs `process-exit` with `subject` self, `exit_code` 1 and `detail` `socket-dir-refused`, a step with no span and one line, binds nothing and spawns no child; a `Server::bind` failure stays what it was (`tests/security_negatives_socket_dir.rs` `security_negative_a_start_refuses_the_socket_directory_before_any_bind`)."
    sidecar: "Path 1: the start's socket directory refusal and its one `process-exit` line recorded."
    rationale: "report.md:14 places the check before any snapshot, child or bind, and report.md:70 states the line."
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 250
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "After `a server that is not the recorded wrapper (`server-verify-failed`)` add: and, on Unix, a socket directory the check refuses or a server of another euid (`server-verify-failed`), each exit 0, empty streams, no frame; the socket directory's rows are `security_negative_a_hook_fails_open_on_the_socket_directory` in `tests/security_negatives_socket_dir.rs`, not this matrix, and the refused euid's wiring has no committed witness."
    sidecar: "Security sweep: a refused socket directory and a refused euid join the hook's fail-open causes."
    rationale: "report.md:16 gives the hook both refusals at exit 0 with empty streams and no frame, report.md:95 keeps the hook's rows in the new binary, and report.md:54 records the wiring's gap."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 604
    section: "## 6. E2E Test Strategy"
    change: "After `a server that is not the recorded wrapper` add: and, on Unix, a socket directory the check refuses or a server of another euid; the clause `each exit 0 with no body` then closes the list."
    sidecar: "§6 Security sweep broad line: the two new fail-open causes named."
    rationale: "The broad line of the matrix detail above enumerates the causes, and report.md:16 adds two."
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 252
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add after the impostor rows: A refused socket directory's record is one `hook-decision` line at WARN carrying `detail:\"socket-dir-refused\"`, on the dialog path after `hook-invoked{corr:null}` and with `decision_emitted:false`."
    sidecar: "Security sweep: the hook's record of a refused socket directory."
    rationale: "report.md:72 states the record, the same form the server check took (report.md:59)."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add a sub-bullet after this one: `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) holds the socket directory's error documents, `revive --json`'s among them; the new binary's `assert_names_nothing` and `digit_runs` (`tests/security_negatives_socket_dir.rs`) hold the logs free of path, pid and uid."
    sidecar: "Error sanitization: the socket directory's document and log witnesses."
    rationale: "report.md:116 and the new-text rows for `tests/cli_error_bodies.rs` and `tests/security_negatives_socket_dir.rs` name these witnesses."
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 278
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add after this line: `tests/cross_exit_causes.rs` holds `a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused` and `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`, each in human and `--json` form, pins the new hint as the literal `SOCKET_DIR`, and the table's self-checks read the new word."
    sidecar: "Exit-cause matrix: the socket directory's two table cases and its pinned hint."
    rationale: "The new-text rows for `tests/cross_exit_causes.rs` name both cases and the constant; report.md:113 reads the self-checks."
    fact: F6
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 280
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Now reads: As landed, exit 21's six closed `detail` causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`, on the eight frame verbs, `list` and `plugin install` (`socket-dir-refused` on the eight frame verbs alone), with `error` `instance-unreachable` and MCP's text `refused: instance-unreachable`."
    sidecar: "Exit-cause matrix: exit 21's closed causes five to six."
    rationale: "report.md:28 moves the causes five to six, and report.md:71 places the check at the eight CLI frame verbs."
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 284
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Now reads: server verification fails. On Linux and Windows: `snapshot.json` pid or start time does not match the listening process (a same-user impostor listener on the recorded endpoint), as a `#[cfg(not(target_os = \"macos\"))]` case. On Linux and macOS a server of another euid reads `server-verify-failed` too, through the client euid check, its decision held by injected values and each seam's wiring by no committed test; on macOS that is the word's whole meaning, the client reading no peer pid and comparing no server (the named carve-out, owned by no route entry). A loosened socket directory on macOS is the socket directory cause below, not this one. A same-user impostor inside a valid 0700 dir on macOS is an accepted upstream limitation, not asserted as a refusal."
    sidecar: "Exit-cause matrix: the macOS CARRY on the route entry discharged; the pid gap stays named, unowned."
    rationale: "report.md:26 retires the owed euid check and socket directory check, report.md:31 moves the word's reach, report.md:33 keeps macOS's gap named with no owner, and report.md:54 states the wiring's gap."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 289
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add an exit 21 cause bullet after this one: the socket directory is refused (Unix: a directory at 0755 or a symlink): `detail` `socket-dir-refused`, its own hint, cause `socket-dir` in `src/human/hints.rs`; its log line is `process-exit` with `exit_code` 21, `detail` `socket-dir-refused` and `during` `connect` at WARN, and no `channel-request`."
    sidecar: "Exit-cause matrix: the sixth exit 21 cause and its log line."
    rationale: "report.md:15 and report.md:71 state the cause, its hint word and its line."
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 289
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "The closing clause now reads: `instance-dead`, `unwrapped-name`, `server-verify-failed` and `socket-dir-refused` are `during:\"connect\"`."
    sidecar: "Exit-cause matrix: `socket-dir-refused` joins the causes read during connect."
    rationale: "report.md:15 and report.md:71 give the new cause `during` `connect`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 297
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add an exit 1 cause bullet after this one: the socket directory is refused at the start (Unix), before any snapshot, child or `Server::bind`, `revive`'s start sharing it: exit 1, one `process-exit` line with `detail` `socket-dir-refused`, the table's `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`; a failed create and a refused check read one code, and a `Server::bind` failure stays what it was."
    sidecar: "Exit-cause matrix: the socket directory joins `run`'s exit 1 causes."
    rationale: "report.md:14, report.md:70 and report.md:95 state the refusal, its line and the one code."
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 300
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Now reads: `viola revive --json` prints one exit-1 document for `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error` and `socket-dir-refused`."
    sidecar: "Exit-cause matrix: `revive --json`'s exit-1 codes ten to eleven."
    rationale: "report.md:29 moves the codes ten to eleven, and each list holding all ten extends."
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan.md
    near: 610
    section: "## 6. E2E Test Strategy"
    change: "Replace `(its macOS case owed to the route entry \"Unix endpoint and home hardening\")` with: (macOS compares no peer pid, its server-identity negatives compiled out, the named carve-out; a loosened socket directory on macOS is a case of `tests/security_negatives_socket_dir.rs`)."
    sidecar: "§6 Exit-cause matrix broad line: the owed macOS case landed; the route entry no longer owns it."
    rationale: "report.md:26 and report.md:33 discharge the owed clause, and report.md:64 lands the macOS case in the new binary."
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 317
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Now reads: **Unix socket directory refusal** (`#![cfg(unix)]`, IPC access control). With a test-owned socket root (`SOCKET_ROOT_VAR`), pre-create the per-user socket directory as (a) a symlink to another 0700 dir and (b) a 0755 dir. In each case `viola run` exits 1, a CLI client verb against an existing instance exits 21, and `viola hook <event>` exits 0 with no body and writes no frame, the directory left as found. The foreign-owner case needs another uid, which a non-root runner cannot create, so it runs at unit level: `socket_dir_verdict` gets an injected uid other than the euid and must refuse; a non-directory reads `Refused::NotADirectory` there."
    sidecar: "Security negatives: the socket directory refusal as built, on Unix, not Linux alone."
    rationale: "report.md:64 widens the `cfg` past Linux, report.md:107 reads the acceptance on `ubuntu-latest` and `macos-latest` and the injected uid, and report.md:11 states the non-directory and the no-chmod."
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 318
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Now reads: The socket-directory refusal's code is the closed word `socket-dir-refused` (`check_socket_dir`, which never chmods): exit 1 at `run` and `revive`'s start, exit 21 at the eight CLI frame verbs, exit 0 with empty streams at the hook, and `instance-unreachable` with no line at MCP's `frame`; off Unix the check returns `Ok(())`, the DACL and the pid comparison staying the controls."
    sidecar: "Security negatives: the refusal has its code; the anyhow reading and the route entry's ownership retired."
    rationale: "report.md:26 retires the code owed to the route entry; report.md:11, report.md:12, report.md:14, report.md:16 and report.md:17 state it at each surface."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 318
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add a sub-bullet after this line: As landed: `tests/security_negatives_socket_dir.rs`, a Unix-only binary, drives a directory at 0755 and a symlink to 0700 against a start, the eight frame verbs and the hook, each left as found, through `security_negative_a_start_refuses_the_socket_directory_before_any_bind`, `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` and `security_negative_a_hook_fails_open_on_the_socket_directory`."
    sidecar: "Security negatives: the new binary and its three negatives."
    rationale: "report.md:39 and report.md:107 and the new-text rows for `tests/security_negatives_socket_dir.rs` name the binary and its cases."
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 323
    section: "## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery"
    change: "Add a sub-bullet after this one: `security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private` (`tests/security_negatives_socket_dir.rs`) holds a clean start's socket private under an open umask."
    sidecar: "Security negatives: the socket's privacy under an open umask."
    rationale: "The new-text rows for `tests/security_negatives_socket_dir.rs` name the case."
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 612
    section: "## 6. E2E Test Strategy"
    change: "`the Unix socket directory refusal` now reads `the Unix socket directory refusal (`tests/security_negatives_socket_dir.rs`, Linux and macOS)`, and `a pre-existing home another user may read refusing `viola run` exit 1` now reads `a pre-existing home another user may read refusing `viola run` exit 1 and `viola list` exit 21`."
    sidecar: "§6 Security control negatives broad line: the new binary named; the board's read rule on the home."
    rationale: "report.md:107 reads the binary on both Unix runners; report.md:32 and report.md:112 move `viola list` to the read rule on all three OSes."
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 14
    section: "## 7. Test Data & Fixtures"
    change: "Add a row after this one: | A test-owned socket root | `tests/support/home.rs`, under `/tmp`: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`, the last there and not in `tests/support/start.rs` | per-test |"
    sidecar: "Test data: the socket root helpers."
    rationale: "report.md:21 names the helpers, report.md:63 the roots under `/tmp`, and report.md:81 the place of `first_life_on`."
    fact: F22
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 30
    section: "## 7. Test Data & Fixtures"
    change: "After `beside an endpoint the test binds` add: , in a directory made through `viola_state::fs::create_private_dir`, because a bare `tempfile::tempdir()` reads 0755 under the default umask and the client's socket directory check would refuse it"
    sidecar: "Test data: the impostor cases' endpoint directory is made private."
    rationale: "report.md:56 measures the bare temp dir at 0755 and narrows these sites, `tests/security_negatives_server_identity.rs` and `tests/hook_fail_open.rs` among them."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 21
    section: "## 8. Mocking & Stubbing Discipline"
    change: "Before the row's closing `|` add: ; a loosened socket directory is likewise made on the real file, at 0755 or as a symlink to 0700 (`Form` in `tests/security_negatives_socket_dir.rs`), refused by the product's own check and read back as found, and the pure `socket_dir_verdict` takes its uid by injection at the unit tier"
    sidecar: "Filesystem faults: the socket directory's real-file fault and its pure verdict."
    rationale: "report.md:107 states the forms and the injected uid; the new-text rows name `Form`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 22
    section: "## 8. Mocking & Stubbing Discipline"
    change: "Add a row after this one: | Peer euid decisions | none (a parameter of the decision or of the private function under test) | `peer_euid_admitted(observed, own, reported)` and the private `Admission` function passed to `serve_admitting` are tested by dependency injection, with no env, flag or feature seam |"
    sidecar: "Mocking: the peer euid decisions are injected, with no seam."
    rationale: "report.md:12 and report.md:13 state both decisions and the absence of any seam."
    fact: F15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 18
    section: "## 10. Quality Gates & Coverage Targets"
    change: "Add after this line: A mutant replacing a client seam's `peer_admitted` read with an always-admit survives every committed test by construction: one account can make no foreign peer and no seam may be added. The mutation audit reads that same survivor at the route entry \"Version done-check\"."
    sidecar: "Mutation gate: the client seams' named survivor; no threshold moves."
    rationale: "report.md:54 states the survivor, its cause and its reader at \"Version done-check\"."
    fact: F14
note: "No picked entity here carries test-plan.md:618 (the parity line can gain `handlers_frame_tools_refuse_a_loosened_socket_directory`, F8's) or test-plan.md:254 and :775 (`euid + dir-only`), which sit in other categories; the euid wiring's coverage gap (D-tests-coverage) is carried by F13 and F14 and the report's own owner at report.md:54."
