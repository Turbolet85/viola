proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 91
    section: '## 6. E2E Test Strategy › #### Scenario: Path 3 — `wait` / `last` event-driven readback (§6 live flow)'
    change: '  - Step 7: the parked CLI `wait` exits 21 with `{"v":1,"error":"instance-unreachable","detail":"endpoint-lost"}` (the wrapper lost with the call open, its `process-exit` `during:"call"`). MCP gives `is_error:true` with the CLI document less `v`, text `refused: instance-unreachable`: a wrapper killed under a parked MCP `wait` reads `detail:"endpoint-lost"` (`mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`), and an rmcp `wait` on the dead instance, its directory standing, reads `detail:"instance-dead"`.'
    sidecar: 'Path 3 step 7: the exit-21 document and the MCP body name their cause, `endpoint-lost` under a parked wait and `instance-dead` on the dead instance, in place of `detail:null`.'
    rationale: 'Report Retired names the exit-21 document `detail: null`; Symbols / APIs moves it to one document per cause, and MCP reads `Closed | Io` as `EndpointLost` and no live endpoint by `without_endpoint`.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 274
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '- **Exit-cause matrix** (root `tests/cross_exit_causes.rs`, its `CAUSES` table and one rstest test per cause, `#[case] json: bool` where both modes are driven). Exit 21 and exit 1 each have several causes. Every case asserts the exit code and, by mode, the `--json` document with stderr empty or a last stderr `hint:` line that names that cause. A hint shared across causes fails the table (`no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`), and the hints are pinned as literals in the file.'
    sidecar: 'Exit-cause matrix: the table is built; its pinning is literal in the file, no longer owed to trycmd.'
    rationale: 'Report Counts `cross-table` lists `e2e-test-strategy.md:274` changed; Retired names the "not built" lines of the exit-cause matrix; New text lists `CAUSES`, `no_two_codes_share_a_hint` and `shared_hints_reads_a_planted_duplicate`.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 275
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: ' As landed (chunk 2026-10-31-exit-cause-code-catalogue): a `[[test]]` with `required-features = ["fake-agent"]`, a root `cross_` binary in the E2E layer, on the `ci` profile kill line, `39 tests run: 39 passed` on Linux under `agent-run.sh run --e2e --filter binary(cross_exit_causes)`. Exit 21 rows: `strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log`, `strict_modes_on_the_board_is_exit_21_and_opens_no_log`, `strict_modes_on_plugin_install_is_exit_21_and_creates_nothing`, `a_stopped_instance_is_instance_dead`, `a_name_never_started_is_unwrapped_name`, `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`. Exit 1 and code rows: `a_live_holder_is_already_live`, `a_held_endpoint_is_squatted_name`, `a_tampered_pinned_copy_is_pinned_hash_mismatch`, `strict_modes_on_a_start_is_exit_1_and_opens_no_log`, `a_revive_with_no_such_session_is_no_session`, `a_session_open_elsewhere_is_session_live`, `a_recorded_directory_gone_is_cwd_missing`, `a_revive_with_no_claude_on_its_path_is_internal_error`, `a_frame_verb_that_cannot_open_its_log_is_internal_error`, `verify_with_no_claude_on_its_path_is_program_not_found`, `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`. Beside them, `no_hint_names_release` and `every_code_is_a_code_word`. Named outside the table, in the comment at the top of the file, each a case of the binary its row names: server verification, stale heartbeat, the `.cmd` human pair, the version of `verify`, its recording, pinned-copy, interrupted and stamps refusals, and the two silent exits of `viola mcp`.'
    sidecar: 'Exit-cause matrix: the "does not exist" line becomes the landed table, its rows, its reading and the causes it names outside itself.'
    rationale: 'Report Counts `cross-table` lists `e2e-test-strategy.md:275` changed; Changes names `tests/cross_exit_causes.rs` new (567 lines) and Harness / gate surface its `[[test]]`, kill line and E2E regex; Outcome reads `39 tests run: 39 passed` on Linux.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 278
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace the clause "a `tests/cli_links.rs` case, not a `cross_exit_causes.rs` row:" with "a `tests/cli_links.rs` case, beside the table row `strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log`:"; the rest of the line stands.'
    sidecar: 'Exit-cause matrix, strict-modes on `link` / `unlink`: the case stands beside the strict-modes row the table now holds.'
    rationale: 'Report Counts `cross-table` keeps "not a row" true only where the cause is named outside the table; the strict-modes cause of exit 21 is not on that list and the table holds its row.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 279
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace the clause ", not `cross_exit_causes.rs` rows:" with ", beside the table row `strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log`:"; the rest of the line stands.'
    sidecar: 'Exit-cause matrix, strict-modes on the six frame verbs: the negatives stand beside the strict-modes row the table now holds.'
    rationale: 'Report Counts `cross-table`: "not a row" stays true only for the causes named outside the table, and strict-modes is not one of them.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 282
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace the clause "its `--json` document is every exit-21 cause''s `{"v":1,"error":"instance-unreachable","detail":null}`" with "its `--json` document is `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`"; the rest of the line stands.'
    sidecar: 'Server verification: its `--json` document names its own cause, `server-verify-failed`.'
    rationale: 'Report Retired names the exit-21 document `detail: null`; Symbols / APIs lists `server-verify-failed` among the five closed causes.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 283
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - no instance is running for the name: the endpoint is gone after `cleanup`; cause `instance-dead` (the instance directory stands, `without_endpoint`), `during:"connect"`, document `{"v":1,"error":"instance-unreachable","detail":"instance-dead"}` or the not-running hint last, the table row `a_stopped_instance_is_instance_dead`; for `viola link` / `viola unlink`, as landed a `tests/cli_links.rs` case beside it, `link_verbs_against_a_stopped_instance_exit_21` (both verbs in both modes: the not-running hint last, or the `instance-unreachable` document with stderr empty)'
    sidecar: 'Exit-cause matrix, no instance running: the cause reads `instance-dead` and holds a table row, the `link` verbs case beside it.'
    rationale: 'Report Symbols / APIs: `without_endpoint` reads a directory `InstanceDead`, `instance-dead` is `during:"connect"`; New text lists `a_stopped_instance_is_instance_dead`, and Reverted facts keeps `not-running` as the hint key.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 284
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - the name is an unwrapped `claude agents --json` row, never a target (E1), or a name never started: cause `unwrapped-name` (`without_endpoint`: absent, a regular file or a symlink at the instance directory, the reading the delegate overseer''s word, `inputs#I2` Q2, PROVISIONAL), document `{"v":1,"error":"instance-unreachable","detail":"unwrapped-name"}` or the unwrapped-name hint last (`src/human/hints.rs`), the table row `a_name_never_started_is_unwrapped_name`; beside it the `tests/cli_list.rs` case `e1_send_naming_an_unwrapped_row_is_unreachable`'
    sidecar: 'Exit-cause matrix, unwrapped name: the hint is built and the cause holds its code and table row; the owed clause is spent.'
    rationale: 'Report Retired names the "not built" line of the unwrapped-name hint; Counts `causes-new` lists `e2e-test-strategy.md:284` changed; Decisions carries `inputs#I2` Q2 PROVISIONAL.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 284
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - a wrapper lost with the call open: cause `endpoint-lost`, `process-exit` `during:"call"`, document `{"v":1,"error":"instance-unreachable","detail":"endpoint-lost"}` or its own hint last (the delegate overseer''s word, `inputs#I2` Q3, PROVISIONAL), the table row `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`; a process-exit `instance-dead` no longer carries `during:"call"`'
    sidecar: 'Exit-cause matrix: new exit-21 cause row `endpoint-lost`.'
    rationale: 'Report Symbols / APIs, "A process-exit `during`": a wrapper lost with the call open is `endpoint-lost` · `during:"call"`; the file names no such cause.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 284
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - on MCP, each exit-21 cause above is the frame tools'' `isError` result, the CLI document less `v` with its cause in `detail`, text `refused: instance-unreachable`, no frame and no line (`handlers_frame_tools_name_the_cause_of_an_unreachable_instance`, `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`), `server-verify-failed` off macOS'
    sidecar: 'Exit-cause matrix: the MCP form of the five causes and its witnesses.'
    rationale: 'Report Symbols / APIs, MCP, and Outcome "MCP''s body less `v`, the text unchanged, each cause its word"; the matrix states no MCP form.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 287
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - the heartbeat is `stale` (live pid, heartbeat older than 5 s): code `stale-heartbeat`, printed and logged, apart from `already-live` for a live holder and a fresh replayed beat. The `run.collision_check` span `outcome` stays `already-live` for a live or a stale holder, and a frame verb against a stale instance still reads `instance-dead`. Named outside the table: `run_refuses_a_stale_name` and `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` (`tests/cli_instance_state.rs`, on the `Frozen` `SIGSTOP` guard, which exists on Unix too since this chunk)'
    sidecar: 'Exit-cause matrix, stale heartbeat: its own code `stale-heartbeat`, its witnesses, the span and the frame verbs unmoved.'
    rationale: 'Report Symbols / APIs, "A stale holder''s code", deviation 2 and New text (`revive_json_of_a_stale_holder_is_the_stale_heartbeat_document`); the row names no code today.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 291
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - Exit 1 (`viola verify` refuses) codes, each on the `process-exit` it writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside the standing `interrupted`. A refused home tree stays lineless, a failing row is no refusal and no code, the fixture `<file>` enters no code, and `verify` takes no `--json`. Cases: `verify_with_no_claude_on_its_path_is_program_not_found` (the table), `verify_a_tampered_pinned_copy_refuses_with_its_code` and `verify_window_a_widened_stamps_file_refuses_with_its_code` (`tests/cli_verify.rs`).'
    sidecar: 'Exit-cause matrix: the refusal codes of `viola verify` and their witnesses.'
    rationale: 'Report Symbols / APIs, "`verify`''s codes", Counts `verify-codes` ("New text wherever §6''s catalog stands") and deviation 7 name them; no line of this file states them.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 292
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace the clause "these are cases of root `tests/cli_revive.rs`, not rows of `cross_exit_causes.rs`:" with "these are cases of root `tests/cli_revive.rs`, beside the rows of `cross_exit_causes.rs`:"; the rest of the line stands.'
    sidecar: 'Exit-cause matrix, `revive` refusals: the `tests/cli_revive.rs` cases stand beside the rows the table now holds.'
    rationale: 'Report New text lists `strict_modes_on_a_start_is_exit_1_and_opens_no_log`, `a_revive_with_no_such_session_is_no_session`, `a_session_open_elsewhere_is_session_live` and `a_recorded_directory_gone_is_cwd_missing` in the table; these causes are not on the `cross-table` outside list.'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 293
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: ' Since chunk 2026-10-31-exit-cause-code-catalogue every refusal of the start `revive` shares with `run` prints one exit-1 document under `--json`, before `spawn_child`, the human pairs byte for byte as they stood: `batch-script-child` (`a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`, Windows), `pinned-hash-mismatch` (`a_tampered_pinned_copy_is_pinned_hash_mismatch`), `squatted-name` (`a_held_endpoint_is_squatted_name`) and a `claude` not on `PATH` as `internal-error` (`a_revive_with_no_claude_on_its_path_is_internal_error`). An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error` and the catch site still logs its chain and its line; a `pump_child` error prints no document, the child having held the terminal. The codes of `revive --json`: `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`.'
    sidecar: 'Exit-cause matrix, `revive --json`: the post-preflight gap is closed; its four start documents, witnesses and the ten codes replace the owed line.'
    rationale: 'Report Retired names the `revive --json` post-preflight gap, and Symbols / APIs says the sentence that a post-preflight refusal prints no document is no longer true; Counts `revive-gap` and `title` change each "owed to" this entry.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 295
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - `already-live` and `stale-heartbeat`: `run`''s own collision check and its two landed pairs, unchanged (`hint: viola list` last on a live name); a live holder or a fresh replayed beat is `already-live`, a `stale` holder `stale-heartbeat`, each its own `--json` document and logged code'
    sidecar: 'Exit-cause matrix, `revive` holder reading: a stale holder reads `stale-heartbeat`, no longer `already-live`.'
    rationale: 'Report Retired names `already-live` as a stale holder''s refusal; the `revive --json` code list in Symbols / APIs holds both words.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 299
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - The `cli` catch site under `--json`: where it prints an internal error, `--json` among the arguments before a `--` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr empty; without `--json` the line is byte for byte as it stood, and `run`, the start of `revive`, `mcp` and `hook` stay silent there. Cases: `a_frame_verb_that_cannot_open_its_log_is_internal_error` (the table), `cli_verbs_print_the_internal_error_document_under_json` (`tests/cli_wait_last.rs`).'
    sidecar: 'Exit-cause matrix: the `--json` document of the `cli` catch site and its witnesses.'
    rationale: 'Report Symbols / APIs, "The `cli` catch site under `--json`", and Coverage of new surfaces (the table, `cli_wait_last`); the matrix does not state this path.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 319
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the line stands.'
    sidecar: 'Security control negatives, strict-modes: the refused frame verbs print the `strict-modes-failed` document.'
    rationale: 'Report Retired names the exit-21 document `detail: null`; Counts `doc-null` says a strict-modes site takes `strict-modes-failed`.'
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 339
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - endpoint vanishes during `wait`: exit 21, cause `endpoint-lost`, `during:"call"` (its row in the exit-cause table `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`)'
    sidecar: 'Chaos suite: the vanished endpoint names its cause, `endpoint-lost`.'
    rationale: 'Report Symbols / APIs, "A process-exit `during`", and deviation 1, the lost row of the table sharing the kill helper of `chaos_wait_vanish`.'
note: 'Body lines test-plan.md:553 and :610 and the §2 naming line stand outside this file; D-tests-coverage, D-tests-framework, D-tests-obs-harness and D-tests-count-literals found no drift here; the suite blocks carry no origin line of their own, so section names the nearest, E5; the line at e2e-test-strategy.md:281 stays true (server verification is named outside the table) and :38 is the report''s no-change.'
