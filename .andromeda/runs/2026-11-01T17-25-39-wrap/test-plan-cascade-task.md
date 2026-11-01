# Task — test-plan · 2026-11-01-unix-endpoint-and-home-hardening

## The session
Built the verified per-user socket directory check on every Unix path that binds or reaches an endpoint: `viola run` and `revive`'s start refuse exit 1, the eight CLI frame verbs exit 21 with the new closed cause `socket-dir-refused`, `viola hook` fails open with exit 0, MCP's `frame` reads `instance-unreachable`.
Built the peer euid check on both sides: the wrapper drops a refused peer in `accept_loop` before any frame, and the three client seams read a refused euid as `server-verify-failed`, now on Linux, Windows and macOS.
Moved `viola list`'s home from two rules to the read rule alone on every OS, and changed the harness `cleanup` so `endpoint_gone` reads a connect refused or a path absent, measured false before after a force-killed wrapper.
Added the test binary `tests/security_negatives_socket_dir.rs` and retired the dated owner route entry Unix endpoint and home hardening: its owed clauses are discharged, and macOS's absent peer pid comparison stays named, owned by no route entry.
Measured a false premise: a bare `tempfile::tempdir()` is 0755 under the default umask, not 0700, so four test sites narrowed their endpoint directory through `viola_state::fs::create_private_dir`.
Recorded the weakest proof: each client seam's euid wiring is pinned by no committed test. CI `ci#41329861618` read green at `c4bf0a92`.

## Reached files
- crates/viola-core/src/lib.rs
- crates/viola-state/src/strict.rs
- crates/viola-state/src/board.rs
- crates/viola-channel/src/lib.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- crates/viola-channel/src/server.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/run.rs
- src/cmd/client.rs
- src/cmd/hook.rs
- src/human/hints.rs
- schemas/diag-line.v1.json
- crates/viola-e2e/src/harness/cleanup.rs
- tests/security_negatives_socket_dir.rs
- crates/viola-e2e/tests/harness_lifecycle.rs
- crates/viola-mcp/tests/handlers.rs
- tests/support/home.rs
- tests/cli_list.rs
- tests/cross_exit_causes.rs
- tests/contract_error_bodies.rs
- tests/cli_error_bodies.rs
- tests/cli_controls_not_disableable.rs
- tests/hook_fail_open.rs
- tests/security_negatives_server_identity.rs
- Cargo.toml
- .config/nextest.toml

## Touches
- Exit 21's closed causes moved five to six, the sixth `socket-dir-refused`: a row saying five closed causes changes, a row listing the five extends, a row naming only the type does not change. The full list is `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`.
- `revive --json`'s exit-1 codes moved ten to eleven: `.andromeda/test-plan-detail/test-scope-summary.md:43` changes and each list holding all ten extends; a row naming `batch-script-child` as `run`'s other code or as a refusal's own word does not change.
- `schemas/diag-line.v1.json` moved `process-exit`'s `detail` enum 19 to 20 and `hook-decision`'s `detail` enum 7 to 8, each gaining `socket-dir-refused`: any row stating those sizes or resting on them through schema conformance.
- `server-verify-failed`'s reach moved from off macOS to Linux, Windows and macOS, where on macOS it means an euid mismatch alone: each row stating the word's OS reach changes, including the OS-branch rows that read macOS as euid plus directory-only server verification.
- `viola list`'s home moved from two rules to the read rule on every OS, `instances/` staying on the path check; `.andromeda/test-plan-detail/unit-test-strategy.md:31` on strict-modes having two rules stays true, since the write rule and the read rule both still stand and `read_budget_stamped`, `mcp`'s start and `plugin install` keep the write rule on the home.
- The dated owner Unix endpoint and home hardening: each row owing a client euid check, a socket-directory check, its code or a macOS case to that route entry changes, as those are built.
- The owed macOS case landed in `tests/security_negatives_socket_dir.rs` on `macos-latest`, its `cfg` wider than the plan's Linux-only one, so the exit-cause matrix row that owes it changes; the server-identity negatives stay compiled out on macOS and `server_verdict_rows` keeps its row.
- A row stating macOS compares no peer pid stays a true claim, its gap wording kept as the named carve-out with no route-entry owner; only a row reading a built check as owed changes.
- Rows on the Unix socket directory (the security-vector IPC trigger on directory 0700 and file 0600, the security control negatives' socket directory refusal, multi-OS rows): the refusal is now built and also refuses a symlink and a non-directory, the check never chmods and a refused directory stays as found.
- Rows on the peer credential (the `viola-channel` scope entity, its partial testability, the §4 decision with injected euids, the cross-OS-user coverage note): a foreign euid and a foreign owner are still not driven, the decisions held by injected values, and the wiring those rows imply is now the named gap in F13.
- §3 `cleanup` step 4 and its contract `.andromeda/registries/contracts/test-plan/5-command-implementation.md:27`: `endpoint_gone` on Unix now reads a connect refused or `NotFound` through `unconnectable`, in place of the path no longer existing, beside the rule on `holder_gone`; the document's shape is unchanged and key-file rows naming `cleanup` are in reach.
- The `run` start sequence (critical path, Path 1 and §1's refused-home list): `socket_dir_ready` comes before any snapshot, child or `Server::bind`, and a refused directory exits 1 through `refused_with` in `launch.json`; `revive`'s start shares it and a `Server::bind` failure stays what it was.
- Exit-cause matrix suite: the socket directory cause joins the table with its own hint, and the suite's named cases and top-comment list of causes held elsewhere can move; `no_two_codes_share_a_hint`, `no_hint_names_release` and `every_code_is_a_code_word` now cover the new word.
- Cross-surface parity for each cause of exit 21: MCP's `isError` body gains `socket-dir-refused` as the CLI document less `v`, witnessed in `crates/viola-mcp/tests/handlers.rs`.
- Security sweep and the hook fail-open matrix: a refused socket directory and a refused euid join the fail-open causes, each exit 0, empty streams, no frame; the hook's rows live in the new binary, not `tests/hook_fail_open.rs`, whose edit is only the private directory.
- Frame-verb counts: the socket directory check reaches the eight callers of `connect` (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`); the six frame verbs of the strict-modes case stay six, so a row counts its own case and neither figure replaces the other.
- Refusals that write no line: the §5 and security-negative rows saying a refused strict-modes check writes no line stay true of that check; the socket directory refusal does write one line at `run`, the CLI frame verbs and the hook, and none at MCP's `frame`.
- §4 unit coverage per crate gains cases: `viola-core` the sixth cause, `viola-channel` the decision and the drop, `viola-state` the directory verdict and the board's read rule, root bin the checks inside the dispatch and hook spans and the new hint.
- Security negatives for the board: the home another user may read now refuses `viola list` as well as `viola run`, with and without an instance, witnessed on all three OSes; the whole-board refusal's order, exit 21 and no-open are unchanged.
- Error sanitization witnesses: `tests/cli_error_bodies.rs` and `tests/contract_error_bodies.rs` gain the socket directory documents, so any row enumerating the bodies extends; the new word's lines hold no path, pid or uid.
- `no env / flag / config disables a control`: the socket directory check joins `tests/cli_controls_not_disableable.rs`.
- Temp directories: `.andromeda/test-plan-detail/integration-test-strategy.md:41` on a bare temp dir reading 0755 is a true claim about a home and extends to a socket directory; §7's server-identity impostor setup is in reach through `tests/security_negatives_server_identity.rs`.
- Test inventory: rows naming the sibling `security_negatives_server_identity` do not change of themselves, but rows listing the `security_negatives_*` binaries or the root targets gain the new binary, which `Cargo.toml` registers with one `[[test]]` table requiring `fake-agent`.
- `.config/nextest.toml`'s `ci` 10 s kill line gains `binary(security_negatives_socket_dir)`, its `binary(` terms 25 to 26: no row states this count, so a row describing the line stays true unless it enumerates the binaries.
- §9's OS matrix and CI witnesses: no CI step changed; the socket directory binary runs on `ubuntu-latest` and `macos-latest` and is `#![cfg(unix)]`, with no case on `windows-2025`, where the DACL and the pid comparison stay the controls and the check returns `Ok(())`.
- Perf re-read, no value moves (`.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26`): the five hook rows read means of 2.0, 2.0, 2.0, 2.0 and 2.3 ms, highest max 2.8 ms, both checks in the timed path; the band max 2.4–3.3 ms stays a true record of earlier readings and `SPINE_DEADLINE` is unmoved.
- Re-reads unmoved: the exit set, `RefusalReason` (the cause has no serde path and no refusal reason), `TreeRefused`, the `thiserror` derive count of 9, and every dependency and tool version.
- Census: `human::refuse(` under `src/` moved 18 to 19, `run` 7 to 8; any test row carrying that derived figure moves.
- Line citations into a reached file may be stale through added lines, most of all `src/cmd/client.rs`, `src/cmd/hook.rs`, `src/cmd/run.rs`, `crates/viola-state/src/strict.rs`, `crates/viola-channel/src/client.rs`, `crates/viola-channel/src/server.rs` and `tests/support/home.rs`.

## New facts
- F1 · e2e-test-strategy · `tests/security_negatives_socket_dir.rs` is a Unix-only binary driving a directory at 0755 and a symlink to 0700 against a start, the eight frame verbs and the hook, each left as found, through `security_negative_a_start_refuses_the_socket_directory_before_any_bind`, `security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame` and `security_negative_a_hook_fails_open_on_the_socket_directory`.
- F2 · e2e-test-strategy · `security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private` holds a clean start's socket private under an open umask, in the same binary.
- F3 · e2e-test-strategy · A refused directory at `run` or `revive`'s start logs `process-exit` with `subject` self, `exit_code` 1 and `detail` `socket-dir-refused`, a step with no span and one line, and binds nothing and spawns no child.
- F4 · e2e-test-strategy · A refused directory at a CLI frame verb logs `process-exit` with `exit_code` 21, `detail` `socket-dir-refused` and `during` `connect` at WARN, and no `channel-request`.
- F5 · e2e-test-strategy · The hook's record of a refused directory is one `hook-decision` with `detail` `socket-dir-refused` at WARN, on the dialog path after `hook-invoked` with a null `corr` and with `decision_emitted` false.
- F6 · e2e-test-strategy · `tests/cross_exit_causes.rs` holds `a_loosened_socket_directory_on_a_frame_verb_is_socket_dir_refused` and `a_loosened_socket_directory_on_a_start_is_socket_dir_refused`, each in human and `--json` form, and pins the new hint as the literal `SOCKET_DIR`.
- F7 · e2e-test-strategy · `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text` in `tests/cli_error_bodies.rs` holds the error documents, `revive --json`'s among them, and the new binary's `assert_names_nothing` and `digit_runs` hold the logs free of path, pid and uid.
- F8 · integration-test-strategy · `handlers_frame_tools_refuse_a_loosened_socket_directory` holds MCP's frame tools at no frame and no line with `isError` the record.
- F9 · integration-test-strategy · `setting_does_not_disable_the_socket_dir_check` in `tests/cli_controls_not_disableable.rs` runs every setting against the check.
- F10 · integration-test-strategy · `list_refuses_a_home_another_user_may_read_with_and_without_an_instance` in `tests/cli_list.rs` holds `viola list` to the read rule.
- F11 · integration-test-strategy · A test placing an endpoint in a temp directory makes it through `viola_state::fs::create_private_dir`, because a bare `tempfile::tempdir()` meets the socket directory check as a plain home meets the read rule; `crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs` and `refused_inside` in `src/cmd/client.rs` do so.
- F12 · integration-test-strategy · `viola_channel::Client::connect` itself carries no directory check, as `viola-channel` has no `viola-state` edge, and `channel_link_names_are_validated_by_the_wrapper` writes through that client past every check by design.
- F13 · test-scope-summary · Untestable zone: each client seam's euid wiring (`connect` in `src/cmd/client.rs` and `src/cmd/hook.rs`, `frame` in `crates/viola-mcp/src/tools.rs`) has no committed witness, since one account can make no foreign peer and no seam may be added; it was read once, through forced refusals in the run's remove-the-guard evidence rows 7, 8 and 9.
- F14 · quality-gates-coverage-targets · A mutant replacing a client seam's `peer_admitted` read with an always-admit survives every committed test by construction, and the mutation audit reads that same survivor at the route entry Version done-check.
- F15 · mocking-stubbing-discipline · The peer euid decisions are tested by dependency injection, `peer_euid_admitted(observed, own, reported)` and the private `Admission` function passed to `serve_admitting`, with no env, flag or feature seam.
- F16 · unit-test-strategy · `viola-state` unit cases: `socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit` (a foreign owner by injected uid, a symlink, a non-directory reading `Refused::NotADirectory`), `check_socket_dir_reads_the_path_itself_and_leaves_it_as_found`, `check_endpoint_dir_reads_the_endpoints_directory` and `check_socket_dir_off_unix_reads_nothing`.
- F17 · unit-test-strategy · `viola-state` board cases: `board_scan_holds_the_home_to_the_read_rule_with_and_without_an_instance`, and on `windows-2025` `board_scan_holds_a_read_granted_home_to_the_read_rule_with_and_without_an_instance`.
- F18 · unit-test-strategy · `viola-channel` client cases: `peer_euid_admitted_takes_only_the_owning_user_where_it_is_reported`, where a peer it cannot read is refused where the euid is reported, `client_server_euid_is_this_user`, `tokio_client_server_euid_is_this_user` and their none-where-unreported siblings.
- F19 · unit-test-strategy · `serve_drops_a_refused_peer_before_any_frame` holds the wrapper's drop on the accept thread, before any worker or frame, the peer reading end-of-file with no line written and nothing on the terminal; `channel_endpoint_answers_protocol_faults` is its control.
- F20 · unit-test-strategy · Root bin cases `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` and `hook_refuses_a_loosened_socket_directory_inside_hook_handle`; the former opens the `wait.block` span by hand, and the span recorder `Seen` moved to the test module `refused_seen`, out of the macOS-gated `refused_inside`, so these run on macOS.
- F21 · unit-test-strategy · `send_hint_is_the_design_string` pins the new hint for cause `socket-dir` in `src/human/hints.rs`, its wording a planning form with the founder's wording owed at Epoch 7's boundary batch.
- F22 · test-data-fixtures · `tests/support/home.rs` holds the test-owned socket roots under `/tmp`: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`, the last there and not in `tests/support/start.rs`.
- F23 · integration-test-strategy · Harness witnesses `cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone` in `crates/viola-e2e/tests/harness_lifecycle.rs` on `ubuntu-latest`, and unit `unconnectable_reads_a_leftover_socket_file_no_one_listens_on` in `crates/viola-e2e/src/harness/cleanup.rs`.
- F24 · new · §3 harness contract: a force-killed wrapper's socket file stays in the per-user 0700 directory until a next bind of that name takes it over; `cleanup` removes no leftover and reads `ok:true`, the choice recorded with no owner.

## Untouched
- The fake agent's contract, its modes and receipts, the recorded `fixtures/claude/` sets and fixture hygiene: no fixture or fake-agent source changed and no live `claude` ran.
- Path 5 (the wheel), Path 7 (the version gate) and E2 (the R8 strip): the session reaches none of their state machines, gates or strip rules.
- Coverage thresholds, the `COVERAGE_IGNORE` regex and the mutation gate's verdict arithmetic: unchanged; F14 adds a named survivor, not a threshold.
- CI workflows, jobs, Action pins, permissions, caches and artifact uploads: no CI step changed, so these stay as written.
- `tests/contract_bounded_readers.rs`, its tables and the cargo-fuzz targets: not among the changed files.
- `tests/cross_list_rows.rs`, the `viola list` columns and captions, and the `viola --help` literal: those pins are unedited and the layouts are unmoved.
- `crates/viola-mcp/src/result.rs` and its unit table: a row was made and taken back with `git checkout`, so the file stands as it was.
- `.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10`: a by-line match on the kill line, an earlier chunk's record, no change.
- Test-case counts: no test-plan row should gain one from this run's readings, as a master states no test-case count.
- §11's anti-pattern bans: the session adds and lifts none; dependency injection here follows the standing ban on runtime patching.
