## 2026-11-01-unix-endpoint-and-home-hardening — `cleanup`'s `endpoint_gone` on Unix
**Section:** §3 5-command implementation (`registries/contracts/test-plan/5-command-implementation.md`: `cleanup` step 4, the `Wrapper::stop` / `stop_keep` clause); §5 Integration Test Strategy (`integration-test-strategy.md`: Harness `cleanup` witnesses, new); §1 Test Scope Summary (`test-scope-summary.md`: the cleanup requirement)
**Change:**
- `cleanup` step 4 on Unix: was the `<per-user 0700 dir>/viola-<h12>.sock` path no longer exists; now a connect to that path is refused or the path is absent.
- After a force-killed wrapper on Unix the leftover socket file stays until a next bind of that name takes it over: cleanup removes nothing of it, and its document reads `endpoint_gone` true and `ok:true`.
- `unconnectable` on Unix: was the socket path gone; now a connect refused or the path absent, what `holder_gone` reads there.
- §1's requirement: was the Unix socket file or pipe no longer exists; now on Unix a connect refused or the path absent, on Windows the pipe no longer exists, and `cleanup` removes no leftover socket file.
- §5 names the witnesses: `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone` (`crates/viola-e2e/tests/harness_lifecycle.rs`, on `ubuntu-latest`) and the unit `unconnectable_reads_a_leftover_socket_file_no_one_listens_on` (`crates/viola-e2e/src/harness/cleanup.rs`).
**Why:** Reading the socket file's absence was measured false: after a force-killed wrapper on Unix `cleanup` read `false` and exit 1. The new rule and the file left in place stand on the overseer's P5 word as the founder's delegate, PROVISIONAL (`inputs#I3`): a harmless file in a 0700 directory, no owner proposed.
**Kept:** The Windows arm and the per-user directory list. The kill path stays on `holder_gone` and the stop path on `unconnectable`.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the `ci` kill line's binaries
**Section:** §3 Bootstrap phases (`registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md`: `[profile.ci]`)
**Change:**
- The verify-driven binaries: was twenty-five; now twenty-six.
- The `binary(…)` names on that filter line gain `security_negatives_socket_dir`.
**Why:** The chunk's new test binary joined the line's filter. The row spells the count in words, worded otherwise than the sweep's pattern, so the chunk's reading that no row states this count does not hold for it.
**Kept:** The line's `{ period = "10s", terminate-after = 2 }`, a 20 s kill: no bound moved.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a start's socket directory refusal
**Section:** §6 E2E Test Strategy (`e2e-test-strategy.md`: Path 1, Exit-cause matrix's `run` exit 1 causes); §1 Test Scope Summary (`test-scope-summary.md`, `test-plan.md`: Path 1)
**Change:**
- Path 1's start gains, after the version gate and before the spawn, on Unix the socket directory check (`socket_dir_ready`: the endpoint's parent checked, created only when absent, then checked again), `revive`'s start sharing it.
- A refused directory (at 0755, or a symlink to 0700) exits 1 before any snapshot, child or `Server::bind`, left as found, and logs one `process-exit{subject:"self", exit_code:1, detail:"socket-dir-refused"}`, a step with no span.
- A new `run` exit 1 cause: a failed create and a refused check read one code, and a `Server::bind` failure is not this cause.
- Witnesses: `security_negative_a_start_refuses_the_socket_directory_before_any_bind` (`tests/security_negatives_socket_dir.rs`), `a_loosened_socket_directory_on_a_start_is_socket_dir_refused` (`tests/cross_exit_causes.rs`).
- `test-plan.md`'s Path 1 line names the check and the refused directory beside the squatted endpoint.
**Why:** `inputs#I2` Q1, the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; the wrap's card confirmed by the overseer as his delegate, PROVISIONAL. One code for a failed create and a refused check is a lean the overseer's P5 word took as that delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the hook's two new fail-open causes
**Section:** §6 E2E Test Strategy (`e2e-test-strategy.md`: Security sweep, the fail-open matrix; `test-plan.md`: the sweep's broad line)
**Change:**
- On Unix, beyond the matrix, a socket directory the check refuses (`socket-dir-refused`, `security_negative_a_hook_fails_open_on_the_socket_directory` in `tests/security_negatives_socket_dir.rs`) and a server of another euid (`server-verify-failed`, its wiring with no committed witness) each exit 0 with empty streams and no frame.
- A refused directory's record is one `hook-decision` line at WARN carrying `detail:"socket-dir-refused"`, on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`.
- The broad line names both, the euid's decision held by injected euids.
**Why:** `inputs#I2` Q1 and Q2, the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; the wrap's card confirmed by the overseer as his delegate, PROVISIONAL. One `hook-decision` line, the form the server check took, and the hook's rows in the new binary are leans the overseer's P5 word took as that delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket directory's error bodies
**Section:** §6 E2E Test Strategy (`e2e-test-strategy.md`: Error sanitization); §2 Test Strategy (`test-strategy.md`: the error sanitization row)
**Change:**
- `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) holds the socket directory's error documents, `revive --json`'s among them.
- `assert_names_nothing` and `digit_runs` (`tests/security_negatives_socket_dir.rs`) hold its log lines free of path, pid and uid.
- §2 names that case beside its sibling, and the socket-directory document a row of the `contract_error_bodies.rs` enumeration.
**Why:** The refusal names no path, pid or uid on any surface, and these are its witnesses.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — exit 21's sixth cause and `revive`'s eleventh code
**Section:** §6 E2E Test Strategy (`e2e-test-strategy.md`, `test-plan.md`: Exit-cause matrix); §1 Test Scope Summary (`test-scope-summary.md`: CLI verbs, cli surface, MCP surface); §4 Unit Test Strategy (`unit-test-strategy.md`: `viola-core`, root bin)
**Change:**
- Exit 21's closed causes: was five; now six, `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed` and `socket-dir-refused`, the sixth on the eight frame verbs alone.
- A new cause bullet: a refused socket directory (Unix: at 0755 or a symlink to 0700), its own hint, cause `socket-dir` in `src/human/hints.rs`, its log line `process-exit{exit_code:21, detail:"socket-dir-refused", during:"connect"}` at WARN, with no `channel-request`. `during:"connect"`'s list gains it.
- `tests/cross_exit_causes.rs` holds `a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused` and `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`, each in human and `--json` form, pins the hint as the literal `SOCKET_DIR`, and the table's self-checks read the word.
- `revive --json`'s exit-1 codes: was ten; now eleven, `socket-dir-refused` after `squatted-name`, `internal-error` last.
- §1's MCP `instance-unreachable` `<cause>` list gains the word.
- §4: `viola_core::UnreachableCause` gains `SocketDir`, its word `socket-dir-refused`; `Unreachable::SocketDir` logs that word, `during` `connect`; the hint of cause `socket-dir`, a word apart from the log's, is held by `send_hint_is_the_design_string`.
**Why:** `inputs#I2` Q1, the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; the wrap's card confirmed by the overseer as his delegate, PROVISIONAL.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `server-verify-failed`'s reach and macOS's carve-out
**Section:** §6 E2E Test Strategy (`e2e-test-strategy.md`, `test-plan.md`: Exit-cause matrix, server verification); §5 Integration Test Strategy (`integration-test-strategy.md`: the wrapper channel's pid and start-time item, MCP's mapping); §1 Test Scope Summary (`test-scope-summary.md`: As landed, server verification)
**Change:**
- The word's reach: was off macOS; now Linux, Windows and macOS, where on macOS it means an euid mismatch alone, the client reading no peer pid and comparing no server. A server of another euid reads it on Linux and macOS.
- macOS's client euid check and socket directory check: were owed to the route entry "Unix endpoint and home hardening"; now built.
- macOS's absent pid and start-time comparison: was the recorded gap; now the named carve-out, owned by no route entry, the server-identity negatives staying compiled out there.
- A loosened socket directory on macOS: was an owed `#[cfg(target_os = "macos")]` case; now the socket directory cause, a case of `tests/security_negatives_socket_dir.rs`, on Linux and macOS.
- MCP's mapping gains a refused `check_endpoint_dir` after `live_endpoint` as `socket-dir-refused`, and a peer euid `peer_admitted` refuses as `server-verify-failed`.
- §1 names the euid decision's and real read's cases (`peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, `client_server_euid_is_this_user`, `tokio_client_server_euid_is_this_user`).
**Why:** `inputs#I2` Q2, the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him, and Q1 for the socket directory word; the wrap's card confirmed by the overseer as his delegate, PROVISIONAL. The chunk built what the route entry owed.
**Kept:** A same-user impostor inside a valid 0700 dir on macOS stays an accepted upstream limitation, not asserted as a refusal. §9's `macOS euid + dir-only server verification` reads true now both checks are built, no edit.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket directory's security negatives
**Section:** §6 E2E Test Strategy (`e2e-test-strategy.md`, `test-plan.md`: Security control negatives); §5 Integration Test Strategy (`integration-test-strategy.md`, `test-plan.md`: CLI, the not-disableable table); §2 Test Strategy (`test-strategy.md`: IPC vector row); §1 Test Scope Summary (`test-plan.md`: Coverage triggers, IPC)
**Change:**
- The negative: was `#[cfg(target_os = "linux")]` over a test-owned `XDG_RUNTIME_DIR` and a pre-created `viola/`; now `#![cfg(unix)]` over a test-owned socket root (`SOCKET_ROOT_VAR`) and the per-user socket directory, left as found.
- A foreign owner: was an injected `lstat` result; now `socket_dir_verdict` with an injected uid. A non-directory reads `Refused::NotADirectory` there.
- The refusal's code: was none, an anyhow error at the catch site owned by the route entry; now `socket-dir-refused` (`check_socket_dir`, which never chmods): exit 1 at `run` and `revive`'s start, exit 21 at the eight CLI frame verbs, exit 0 with empty streams at the hook, `instance-unreachable` with no line at MCP's `frame`. Off Unix the check returns `Ok(())`, the DACL and the pid comparison staying the controls.
- Landed as `tests/security_negatives_socket_dir.rs`: `security_negative_a_start_refuses_the_socket_directory_before_any_bind`, `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame`, `security_negative_a_hook_fails_open_on_the_socket_directory`, and `security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private`.
- `setting_does_not_disable_the_socket_dir_check` (`tests/cli_controls_not_disableable.rs`) runs every `SETTINGS` row against the check, a case of its own.
- §1's IPC trigger gains the required negatives: the directory at 0755 or a symlink to 0700, a foreign owner by injected uid, and a peer of another euid decided by injected euids, a client reading `server-verify-failed` and the wrapper dropping it before any frame. §2's row and `test-plan.md`'s negatives line name the binary.
**Why:** `inputs#I2` Q1, the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; the wrap's card confirmed by the overseer as his delegate, PROVISIONAL. The `cfg` is wider than the plan's because the owed macOS case landed in this binary.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the IPC boundary's directory check and MCP's refusal
**Section:** §5 Integration Test Strategy (`integration-test-strategy.md`: Module ↔ IPC, the wrapper channel's MCP cases; `test-plan.md`: MCP); §6 E2E Test Strategy (`test-plan.md`: the exit-21 parity line); §2 Test Strategy (`test-strategy.md`: IPC vector row)
**Change:**
- Module ↔ IPC gains, on Unix: `check_socket_dir`, through `check_endpoint_dir`, refuses a directory with a foreign bit or a foreign owner, a symlink or a non-directory, passes an absent path and never chmods, a refused directory staying as found.
- `viola_channel::Client::connect` itself carries no directory check, as `viola-channel` has no `viola-state` edge; `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design.
- `handlers_frame_tools_refuse_a_loosened_socket_directory` (`crates/viola-mcp/tests/handlers.rs`) holds MCP's frame tools at no frame and no line past `live_endpoint`, `isError` the record, `instance-unreachable` with `detail` `socket-dir-refused`; §5's MCP line, §2's row and the parity line name it.
**Why:** `inputs#I2` Q1, the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; the wrap's card confirmed by the overseer as his delegate, PROVISIONAL. The client's missing check is a deliberate boundary, recorded so no later chunk reads it as a hole.
**Kept:** The socket dir 0700 and file 0600 read-back stands beside the check.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — a test's endpoint directory is made private
**Section:** §5 Integration Test Strategy (`integration-test-strategy.md`: Bare temp-dir home); §8 Mocking & Stubbing Discipline (`mocking-stubbing-discipline.md`: fake servers); §7 Test Data & Fixtures (`test-data-fixtures.md`: the impostor cases, the endpoint directory rule); §1 Test Scope Summary (`test-scope-summary.md`: test data strategy)
**Change:**
- A test placing an endpoint in a directory of its own makes it through `viola_state::fs::create_private_dir`, never a bare `tempfile::tempdir()`: `tempfile` 3.27 leaves the mode to the umask, 0755 on a default one, and the socket directory check (`check_endpoint_dir`) refuses it, as a plain home meets the read rule.
- The sites that do so: `crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs` and `refused_inside` in `src/cmd/client.rs`.
- §7's impostor cases: their test-bound endpoint now stands in a private directory.
**Why:** The plan's reading of those endpoints in a 0700 `tempfile::tempdir()` was measured false: the new client check refused nine `handlers` cases on the first run. A standing rule for any later test that binds an endpoint in a temp directory.
**Kept:** §5's bare temp dir reading 0755 under the umask stays a true claim about a home; this extends it.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — `viola list` on the read rule
**Section:** §5 Integration Test Strategy (`integration-test-strategy.md`: on-disk strict-modes; `test-plan.md`: CLI); §4 Unit Test Strategy (`unit-test-strategy.md`, `test-plan.md`: `viola-state`); §2 Test Strategy (`test-strategy.md`: Filesystem vector row); §6 E2E Test Strategy (`test-plan.md`: Security control negatives)
**Change:**
- A home another user may read refuses `viola list` as it refuses `viola run`, on all 3 OSes, exit 21, with and without an instance, the refusal the whole board's, `ScanRefused::Home`, opening nothing: `list_refuses_a_home_another_user_may_read_with_and_without_an_instance` (`tests/cli_list.rs`).
- `board::scan` (`crates/viola-state/src/board.rs`) holds the home to the read rule through `check_read` and `instances/` to `check_path` on every OS: `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance` and, on `windows-2025`, `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`.
**Why:** `viola list`'s home moved from two rules to the read rule, on `inputs#I2` Q3, the overseer's technical answer as the founder's delegate, PROVISIONAL, not on the wrap's card: no widening, a refusal tightened, the founder's own word owed in the batch at Epoch 7's boundary.
**Kept:** §4's strict-modes two rules stay true: the write rule and the read rule still stand.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket directory's fixtures and helpers
**Section:** §8 Mocking & Stubbing Discipline (`mocking-stubbing-discipline.md`: filesystem faults); §7 Test Data & Fixtures (`test-data-fixtures.md`: the fixture rows, human-mode expected output); §2 Test Strategy (`test-strategy.md`: naming)
**Change:**
- A test-owned socket root, per-test, in `tests/support/home.rs`, under `/tmp`: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`, the last there and not in `tests/support/start.rs`.
- A loosened socket directory, per-test, set by the test on the real directory, never by viola: `Form` (`apply` · `assert_as_found` · `restore`) in `tests/security_negatives_socket_dir.rs`, at 0755 or a symlink to 0700, read back as found. A foreign owner is not driven: `socket_dir_verdict` takes it by an injected uid.
- Human-mode expected output gains the refusal's last `hint:` line: `tests/cross_exit_causes.rs` `SOCKET_DIR`, `tests/security_negatives_socket_dir.rs` `HINT`.
**Why:** The chunk's negatives need a directory the test owns and loosens itself, viola's check never chmodding it. `first_life_on` stands apart from `first_life` because the guard entry's pattern does not allow it in `tests/support/start.rs`.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the peer euid decision and the wrapper's drop
**Section:** §8 Mocking & Stubbing Discipline (`mocking-stubbing-discipline.md`: Peer euid decisions, new row); §4 Unit Test Strategy (`unit-test-strategy.md`, `test-plan.md`: `viola-channel`)
**Change:**
- The decision's injected form: was `(peer_euid, own_euid)`; now `peer_euid_admitted(observed, own, reported)`, a peer it cannot read refused where the euid is reported (`PEER_EUID_REPORTED`, `cfg!(unix)`), held by `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`.
- The real read: `client_server_euid_is_this_user`, `tokio_client_server_euid_is_this_user`, with their siblings `client_server_euid_is_none_where_the_peer_reports_no_euid`, `tokio_client_server_euid_is_none_where_the_peer_reports_no_euid`.
- The wrapper's drop: `serve_drops_a_refused_peer_before_any_frame` (`crates/viola-channel/src/server.rs`) holds a peer an injected decision refuses dropped on the accept thread, before its worker thread and any frame, the peer reading end-of-file, no line and nothing on the terminal; `channel_endpoint_answers_protocol_faults` its control.
- §8: the euids are arguments and the private `serve_admitting` takes its decision as an `Admission` argument: dependency injection, no env, flag or feature seam.
**Why:** One account can make no foreign peer, so the decisions are held over injected values. The client's reading is `inputs#I2` Q2, the founder's own word, live, 2026-11-01 before 16:27 Z, through the overseer's dialog, given after the widening was shown to him; the wrap's card confirmed by the overseer as his delegate, PROVISIONAL. The drop writing no line is a lean the overseer's P5 word took as that delegate, PROVISIONAL (`inputs#I3`).
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the euid wiring, untestable and a known survivor
**Section:** §10 Quality Gates & Coverage Targets (`quality-gates-coverage-targets.md`: the mutation gate); §1 Test Scope Summary (`test-scope-summary.md`, new As landed item; `test-plan.md`: Untestable zones); §2 Test Strategy (`test-strategy.md`: Mutation row); §6 E2E Test Strategy (`test-plan.md`: Skipped as untestable)
**Change:**
- Each client seam's euid wiring (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) has no committed witness, read only through forced refusing decisions outside the suite.
- A mutant replacing a seam's `peer_admitted` read with an always-admit survives every committed test by construction, standing against the `missed` rule; the mutation audit reads that survivor at the route entry "Version done-check".
- The committed tests hold the decision over injected euids, the real euid read on Linux and macOS and the wrapper's drop over an injected decision.
- §6's cross-OS-user row: was covered by the boundary unit test and the SDDL read-back; those stand, and the wiring is named as held by no committed test.
**Why:** One account can make no foreign peer and no seam may be added (security-plan §Security Anti-Patterns, Universal), so this is the chunk's weakest proof. This wrap's ruling names §1's untestable zone as the owner of that remainder.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the timed hook path's two checks
**Section:** §10 Quality Gates & Coverage Targets (`quality-gates-coverage-targets.md`: perf session)
**Change:**
- Each timed hook's `connect` also runs the socket directory check (`check_endpoint_dir`, before it connects) and the peer euid decision (`peer_admitted`, after it connects), both before its first frame, each a real check on Unix only.
- With those two in the timed path the five hook rows read a mean of 2.0, 2.0, 2.0, 2.0 and 2.3 ms and a highest `max` of 2.8 ms, under `SPINE_DEADLINE`.
**Why:** The chunk put both checks in the timed path, and the predicted rise did not show.
**Kept:** The earlier `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms stays, read as the path without these two. The gate stays `SPINE_DEADLINE`: no value moved.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/

## 2026-11-01-unix-endpoint-and-home-hardening — the socket directory's unit cases
**Section:** §2 Test Strategy (`test-strategy.md`: Unit row); §4 Unit Test Strategy (`unit-test-strategy.md`, `test-plan.md`: `viola-state`, root bin)
**Change:**
- `socket_dir_verdict(is_symlink, is_dir, mode, uid, euid)` (`crates/viola-state/src/strict.rs`) reads a symlink `Refused::Symlink`, a non-directory `Refused::NotADirectory` and else `unix_read_verdict`: `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit`, a foreign owner by injected uid.
- `strict::check_socket_dir`, which passes an absent path and never chmods: `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found`, off Unix `check_socket_dir_off_unix_reads_nothing`; `check_endpoint_dir` by `check_endpoint_dir_reads_the_endpoints_directory`.
- Before any frame: `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` (`src/cmd/client.rs`), which opens the `wait.block` span by hand around `connect`, and `hook_refuses_a_loosened_socket_directory_inside_hook_handle` (`src/cmd/hook.rs`).
- Both run on macOS: the span recorder `Seen` stands in the test module `refused_seen`, out of the macOS-gated `refused_inside`, while the server check's cases stay compiled out there.
**Why:** The check sits in `connect`, whose eight callers stay unedited, so the case opens the span itself; `Seen` moved so the two cases that run on macOS can read it.
**Ref:** .andromeda/runs/2026-11-01T17-25-39-wrap/
