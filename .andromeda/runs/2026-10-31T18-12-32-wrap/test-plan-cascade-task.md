# Task — test-plan · 2026-10-31-exit-cause-code-catalogue

## The session
Every cause of exit 21 and exit 1 now has a distinct code across CLI `--json`, MCP and the process log, matching its human hint; no exit moved, no `RefusalReason` and no exit code was added, and exit 21's `error` stays `instance-unreachable`.
Exit 21's document used to carry `detail` null for every cause; it now carries one of five closed causes in `detail`, `v` still 1, and MCP's `isError` result is the same body less `v`, MCP's text unchanged.
The exit-cause table `tests/cross_exit_causes.rs` is built (567 lines, 39 tests on Linux), `revive --json`'s post-preflight gap is closed, the `cli` catch site prints a document under `--json`, and `verify` logs a code for each refusal.
Retired: exit 21's `detail` null, `already-live` as a stale holder's refusal, `instance-dead` with `during` `call`, the not-built lines of the exit-cause matrix and of the unwrapped-name hint, and the `NOT_ERROR_BODIES` row for post-preflight start refusals.
Measured: pre-push 3403 tests passed; CI green on `8a4c1a2c`, 15 of 15 checks, after a red first read on `115cdbb9` from one macOS compile error in `crates/viola-mcp/tests/handlers.rs:539`.
The change reached 41 files; `## Reached files` lists 40, the cap of this form, and the 41st, `src/cmd/pause.rs` (one line at :68, one of the six no-endpoint arms), is named under `## Touches`.

## Reached files
- .config/nextest.toml
- Cargo.toml
- schemas/diag-line.v1.json
- crates/viola-core/src/lib.rs
- crates/viola-state/src/liveness.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/tools.rs
- src/main.rs
- src/human/hints.rs
- src/cmd/mod.rs
- src/cmd/client.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/link.rs
- src/cmd/list.rs
- src/cmd/plugin.rs
- src/cmd/run.rs
- src/cmd/revive.rs
- src/cmd/verify.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-e2e/tests/mcp_tools.rs
- tests/cross_exit_causes.rs
- tests/support/start.rs
- tests/contract_error_bodies.rs
- tests/chaos_wait_vanish.rs
- tests/cli_send.rs
- tests/cli_list.rs
- tests/cli_machine_contract.rs
- tests/cli_wait_last.rs
- tests/cli_controls_not_disableable.rs
- tests/cli_plugin_install.rs
- tests/cli_links.rs
- tests/cli_error_bodies.rs
- tests/cli_instance_state.rs
- tests/cli_revive.rs
- tests/cli_verify.rs
- tests/security_negatives_server_identity.rs
- tests/security_negatives_strict_modes.rs

## Touches
- Any statement that exit 21 prints one document, or that its `detail` is null: each now reads its cause, a strict-modes site `strict-modes-failed` and a not-running site its own cause.
- A `detail` null that belongs to another refusal is unmoved: `.andromeda/test-plan-detail/e2e-test-strategy.md:132` and `.andromeda/test-plan-detail/test-scope-summary.md:155` were re-read and stay.
- Exit-cause matrix status: `.andromeda/test-plan.md:610`, `.andromeda/test-plan-detail/e2e-test-strategy.md:274`, `:275` and `.andromeda/test-plan-detail/test-scope-summary.md:12` say or imply not built; the table is built.
- Text placing a cause outside the table stays true where the cause is named outside it: server verification, stale heartbeat, the `.cmd` human pair, `verify`'s version, recording, pinned-copy, interrupted and stamps refusals, and `viola mcp`'s two silent exits; 7 such sites were re-derived, unmoved.
- The unwrapped-name hint and its code: `.andromeda/test-plan.md:553` and `.andromeda/test-plan-detail/e2e-test-strategy.md:284` say no code or not built; both are built, and the test `e1_send_naming_an_unwrapped_row_is_unreachable` in `tests/cli_list.rs` moved with them.
- Every owed-to clause naming the entry Exit-cause code catalogue, as at `.andromeda/test-plan.md:553`: that debt is spent.
- `revive --json`: any site listing `revive`'s codes or saying a refusal of the start after a passed preflight prints no document, and the both-arms clause at `.andromeda/test-plan.md:62`, now hold ten codes and one document per refusal.
- A stale holder's code: the SIGSTOP chaos clause at `.andromeda/test-plan.md:619` refuses a stale name with `stale-heartbeat`, not `already-live`; a live holder and a fresh beat over a replayed snapshot stay `already-live`.
- The stale-heartbeat and live-pid distinction at `.andromeda/test-plan.md:241` and `.andromeda/test-plan-detail/test-scope-summary.md:183` were re-read, unmoved: the distinction is now built.
- `instance-dead` and `during`: `.andromeda/test-plan-detail/unit-test-strategy.md:87` was re-derived; a wrapper lost with the call open is `endpoint-lost` with `during` `call`, and `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during` `connect`.
- The CLI surface's one-document signal at `.andromeda/test-plan.md:94` and the per-exit `--json` clause at `.andromeda/test-plan.md:453`: a site stating catch-site output under `--json` takes the document; the human line `error: internal error` stands byte for byte.
- Path 3's killed wrapper at `.andromeda/test-plan.md:145` and `:512`: still exit 21 and `instance-unreachable`, its `detail` now `endpoint-lost` under a parked wait.
- The chaos clause of an endpoint vanishing during `wait` at `.andromeda/test-plan.md:619`: `tests/chaos_wait_vanish.rs` now uses `support::start::kill_wrapper`, the file's own kill gone.
- Security negatives at `.andromeda/test-plan.md:612` and the matrix clauses at `:610`: the six frame verbs refusing a widened instance read `strict-modes-failed`, and a server that is not the recorded wrapper reads `server-verify-failed`, exit, no frame and no line unmoved.
- Security triggers at `.andromeda/test-plan.md:184` and `:201`: the same exit 21 now carries `server-verify-failed` and `strict-modes-failed`; MCP's `isError` still carries codes only.
- The On-disk clause at `.andromeda/test-plan.md:444`: strict-modes exit 21 carries its cause and still writes no line; the frame-verb count there says six, the arms are `src/cmd/send.rs`, `wait`, `last`, `answer`, `src/cmd/pause.rs` and `link`, with `release` and `unlink` reaching them through `wheel_call` and `link_call`, and the outcome counts eight frame verbs.
- Cross-surface parity at `.andromeda/test-plan.md:617` and the MCP clauses of §5: MCP's `isError` body now equals the CLI's exit-21 document less `v`, each cause its own word, which `result.rs` `unreachable` writes.
- Error sanitization at `.andromeda/test-plan.md:606`: `tests/contract_error_bodies.rs` holds each new word as a row, `product_error_writers_are_each_named` passes on three OSes, and its `NOT_ERROR_BODIES` post-preflight row is gone.
- Schema conformance at `.andromeda/test-plan.md:608`: the `process-exit` `detail` enum in `schemas/diag-line.v1.json:78` moved from 13 to 19 members, G4 reading no failure; `channel-response` and `hook-decision` did not move.
- The naming line at `.andromeda/test-plan.md:324` places `cross_` in `viola-e2e`; `tests/cross_exit_causes.rs` is a root `cross_` binary, which the E2E regex takes as it took `path1_`.
- The shared-helper rule at `.andromeda/test-plan.md:319`: `tests/support/start.rs` took the start setups, so root files no longer hold their own copies; the mention at `.andromeda/test-plan-detail/e2e-test-strategy.md:38` was re-read and stays true.
- The `ci` profile's kill line in `.config/nextest.toml:24` gains `binary(cross_exit_causes)`, 24 to 25 operands; no test-plan site states that count, and a site naming the kill line without a number stays true.
- A new root binary in `Cargo.toml` with its `[[test]]` and `fake-agent` in `required-features`: any standing count or inventory of root test binaries or of feature-gated tests.
- Unit inventories at `.andromeda/test-plan.md:375`, `:382` and `:393`: `viola-core`, `viola-state` and the root bin each gain cases, and the root bin's closed unreachable causes now include the new words.
- `verify`'s inventory: `tests/cli_verify.rs` gains cases, one under the `verify_window_` prefix because it runs two whole verifies, so any site listing that prefix's cases or `verify`'s logged refusals is reached.
- `verification-matrix.json#v1-18` is met with named carve-outs and `verification-matrix.json#v1-39` is met with its ref extended: any test-plan clause citing either as open or owed.
- The non-gate clause at `.andromeda/test-plan.md:748` for dev-host check-only clippy runs is unmoved: no gate or CI step changed, although the macOS recipe, not run before the first push, would have read the compile error locally.

## New facts
- F1 · e2e-test-strategy · Exit 21's five closed `detail` causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, on the frame verbs, `list` and `plugin install`, with MCP's text staying `refused: instance-unreachable`.
- F2 · e2e-test-strategy · `tests/cross_exit_causes.rs` asserts each cause in both modes where a mode exists: the document under `--json`, the human form ending in its hint, and the cause's one `process-exit` line with its own `detail`.
- F3 · e2e-test-strategy · The table's self-checks are `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`.
- F4 · e2e-test-strategy · A frame verb against a stale instance reads `instance-dead`, because `live_endpoint` gives none for a stale wrapper; stale heartbeat is exit 1's cause alone.
- F5 · e2e-test-strategy · `viola revive --json` prints one exit-1 document for `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `internal-error`.
- F6 · e2e-test-strategy · A `pump_child` error under `revive --json` prints no document, because the child has held the terminal; a program not found at the start is `internal-error`, witnessed by `a_revive_with_no_claude_on_its_path_is_internal_error`.
- F7 · e2e-test-strategy · `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document` is `verification-matrix.json#v1-39`'s Windows witness, and `run` passes `json` false to `Launch` so its human pairs stand byte for byte.
- F8 · e2e-test-strategy · Under `--json` the `cli` catch site prints the `internal-error` document with `detail` null on stdout and leaves stderr empty, witnessed by `cli_verbs_print_the_internal_error_document_under_json` and `a_frame_verb_that_cannot_open_its_log_is_internal_error`; `run`, `revive`'s start, `mcp` and `hook` stay silent there.
- F9 · e2e-test-strategy · `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` witnesses MCP's lost cause, and the `mcp_tools` row after `cleanup` reads its cause from the directory: a home kept by `AGENT_RUN_KEEP_HOMES` reads `instance-dead`, a removed one `unwrapped-name`.
- F10 · e2e-test-strategy · The socket-directory refusal has no code in this chunk: `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site, left to the route entry Unix endpoint and home hardening with macOS's server check.
- F11 · integration-test-strategy · `verify` writes its refusal's code on the `process-exit` under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` and `unclean-fixture`, beside `interrupted`.
- F12 · integration-test-strategy · A refused home tree under `verify` stays lineless, a failing row is no refusal and no code, the fixture's file name enters no code, and `verify` takes no `--json`.
- F13 · integration-test-strategy · `verify_a_tampered_pinned_copy_refuses_with_its_code` and `verify_window_a_widened_stamps_file_refuses_with_its_code` are new in `tests/cli_verify.rs`, and `verify_with_no_claude_on_its_path_is_program_not_found` is a row of the table.
- F14 · integration-test-strategy · `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` holds MCP's mapping: a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is `without_endpoint`'s reading, a failed `server_verified` is `server-verify-failed`, `Connect` is `instance-dead`, `Closed` or `Io` is `endpoint-lost`, with no frame and no line.
- F15 · integration-test-strategy · The unserved row in `crates/viola-mcp/tests/handlers.rs` records an endpoint no one bound through `record_live`, because a Windows `Serving` has no `Drop` and its pipe lives until the process exits.
- F16 · integration-test-strategy · `tool_inputs_that_pass_reach_the_liveness_check` was vacuous, refused by the read rule on a bare 0755 temp dir; its `empty_home` is now viola-made and it reads `unwrapped-name`.
- F17 · integration-test-strategy · `Frozen` in `tests/cli_instance_state.rs` is a `SIGSTOP` guard on Unix as well as Windows; `run_refuses_a_stale_name` and `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` stand on it.
- F18 · unit-test-strategy · `viola_core::UnreachableCause` holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost` and `ServerVerify`, has no serde path, and `unreachable_cause_is_each_its_own_code_word` holds its words.
- F19 · unit-test-strategy · `viola_state::liveness::without_endpoint` reads a directory by `symlink_metadata` as `InstanceDead` and anything else, absent, a regular file or a symlink, as `UnwrappedName`, opening nothing and writing no line, witnessed in `crates/viola-state/src/liveness.rs` and by `without_endpoint_maps_the_directory_reading`.
- F20 · unit-test-strategy · `json_asked` in `src/main.rs` reads `--json` from argv only before a `--`, held by `json_asked_reads_the_flag_before_a_separator`.
- F21 · unit-test-strategy · The `unwrapped-name` and `endpoint-lost` hints are built in `src/human/hints.rs`, held by `send_hint_is_the_design_string`; the hint cause keeps the key `not-running`, not renamed to the log's `instance-dead`.
- F22 · test-strategy · `tests/support/start.rs` now shares `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper` across root test files.

## Untouched
- Hook fail-open and decision bodies: `src/cmd/hook.rs` is not in the change, `hook_fail_open` stands in the block, and hooks stay exit 0 with empty stderr.
- The wrapper channel's framing, `MAX_FRAME`, JSON-RPC error mapping, `sanitised_fault`, endpoint naming and peer credentials: nothing in `viola-channel` moved.
- Recorded fixtures, their schemas and insta snapshots: nothing under `fixtures` moved and no refresh ran.
- Fixture hygiene: `tests/contract_fixture_hygiene.rs` and `tests/support/hygiene.rs` are not in the change.
- `viola-fake-agent` and its binding contract: no mode, argument, receipt or script format moved.
- The CI pipeline, workflows, matrix, artifact uploads and supply-chain jobs: nothing in `.github` or `scripts` moved and no CI step changed.
- Harness verbs, status shape, session record and harness exit codes: no harness verb or status shape moved.
- Coverage thresholds and the mutation gate: no floor, exclusion or verdict rule moved.
- The hyperfine `max` gate and its rows: no row, run rule or bound moved.
- Mocking discipline: no double was added, and the filesystem stays real.
- The R8 strip, the identity floor and the version gate in `viola-agent-claude`: not in the change.
- The bounded-reader enumeration in `tests/contract_bounded_readers.rs`: `without_endpoint` opens nothing, and no reader was added.
- A count of 24 naming the ledger's or the screen's rows: a true claim sharing a number with the kill line, unmoved.
- The test tier, its justification and the untestable zones: no surface, entity class or runner was added.
