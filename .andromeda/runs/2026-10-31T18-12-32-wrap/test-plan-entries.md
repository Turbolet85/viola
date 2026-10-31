## 2026-10-31-exit-cause-code-catalogue — the `ci` kill line's verify-driven binaries
**Section:** §3 Bootstrap phases (registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md: test-runner-install, `[profile.ci]`)
**Change:**
- The `ci` profile's second override holds the twenty-five verify-driven binaries (was twenty-four), its `binary(…)` name list gaining `cross_exit_causes` between `contract_ledger_probes` and `cross_list_rows`, as `.config/nextest.toml` reads; its `{ period = "10s", terminate-after = 2 }` (the 20 s kill) stands.
**Why:** `.config/nextest.toml`'s `ci` kill line gained `binary(cross_exit_causes)` with the new table. The report read no master as stating this count, its pattern requiring the noun right after a number; the spelled-out count here disproves that reading, its rule the `binary(…)` names on the filter line.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — MCP's `instance-unreachable` body names its cause
**Section:** §1 Test Scope Summary (test-scope-summary.md: the MCP surface's Signal and As landed (the instance check), the server verification's MCP half); §5 Integration Test Strategy (integration-test-strategy.md: the impostor case, the widened-instance case, MCP's mapping, the On-disk strict-modes item); §6 E2E Test Strategy (test-plan.md: Cross-surface parity)
**Change:**
- The MCP `isError` body reads `{"error":"instance-unreachable","detail":"<cause>"}` (was `detail` null), `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the CLI exit-21 document less `v`; its text stays `refused: <code>`.
- A refused instance check on the frame tools carries `strict-modes-failed` (`handlers_frame_tools_refuse_a_widened_instance_before_any_frame`, the On-disk item); a server that is not the recorded wrapper carries `server-verify-failed` (`handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`); each still with no frame and no line.
- New: MCP's mapping in `crates/viola-mcp/src/tools.rs` `frame` and `crates/viola-mcp/src/result.rs` `of_reply`: a refused `check_instance` is `strict-modes-failed`, no `live_endpoint` is `without_endpoint`'s reading, a failed `server_verified` is `server-verify-failed` (off macOS), `Connect` is `instance-dead`, `Closed` or `Io` is `endpoint-lost`, no frame and no line; `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` and `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` hold each cause its word.
- Cross-surface parity gains a row: for each cause of exit 21, the MCP `{error, detail}` equals the CLI `--json` document less `v`, text `refused: instance-unreachable`, beside the standing `{refusal, detail}` row.
**Why:** the founder's own word (`inputs#I2` Q1, live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him) put the cause in `detail` on CLI and MCP, `error`, `v` and MCP's text unmoved; his `inputs#I2` Q4, the same authority, holds MCP to the exit-21 causes its five tools can meet, its rejects lineless and `isError` their record. The rows' reading inside those answers, and the mapping's item applied over its contradiction with a standing line (Q5), are `inputs#I6`, delegate the overseer, PROVISIONAL; the ratification stays the founder's. The no-endpoint reading is `inputs#I2` Q2, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — MCP's lost-cause witness, its row after `cleanup` and the unserved row
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Path 3); §5 Integration Test Strategy (integration-test-strategy.md: the MCP cases); §7 Test Data (test-data-fixtures.md: On-disk product state, the impostor cases' group)
**Change:**
- Path 3 gains `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` (`crates/viola-e2e/tests/mcp_tools.rs`), MCP's lost cause.
- The `mcp_tools` row after `cleanup` reads its cause from the directory: a home kept by `AGENT_RUN_KEEP_HOMES` reads `instance-dead`, a removed one `unwrapped-name`, its body compared with the CLI's.
- The unserved row in `crates/viola-mcp/tests/handlers.rs` records, through `record_live(instance, endpoint, pid)`, an endpoint no one ever bound, because a Windows `Serving` has no `Drop` and its pipe lives in the accept thread until the process exits.
- The On-disk product state row's hand-written exceptions read four (was three), the MCP handler suite's own snapshots joining; `record_live` stands beside the server-identity impostor cases.
**Why:** a kept home, as on CI, holds the stopped instance and a removed one does not, so the row reads the directory. A served row would hold a Windows pipe for the process's life, so the row binds nothing. The Path 3 and `record_live` items, read as contradicting standing lines, were applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — exit 21's CLI document, one per cause
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Path 3 step 7, the Exit-cause matrix's exit-21 head and its strict-modes and server-verification causes, Security control negatives' strict-modes item); §5 Integration Test Strategy (integration-test-strategy.md: On-disk, the link verbs); §1 Test Scope Summary (test-scope-summary.md: the server verification, the CLI's As landed (the instance check)); §2 Test Strategy (test-strategy.md: the Filesystem (V7) row)
**Change:**
- Exit 21's document reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` (was `detail` null for every cause); the five closed causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, on the eight frame verbs, `list` and `plugin install`.
- Path 3 step 7: exit 21 with `<cause>`, `endpoint-lost` for a wrapper killed under a parked `wait`, and after `cleanup` `instance-dead` in a kept home or `unwrapped-name` in a removed one.
- A refused instance check reads `strict-modes-failed`: the six frame verbs' negative `frame_verb_refuses_a_widened_instance_before_any_log`, the link verbs, the eight verbs' instance check, the V7 row; the frame verbs' case now stands beside the table's `strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log` (was: not `cross_exit_causes.rs` rows).
- The failed server check reads `server-verify-failed` (was every exit-21 cause's one document); its `--json` is named in the scope summary's eight-verb record.
**Why:** the founder's own word, `inputs#I2` Q1 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him): `detail` turns from null to the closed cause word. The rows' reading inside his answer is `inputs#I6`, delegate the overseer, PROVISIONAL, the V7 row and the five causes' item being applied at the same record's Q5; the ratification stays the founder's.
**Kept:** `error` stays `instance-unreachable`, `v` stays 1 and every refusal stays exit 21; `{"v":1,"error":"<code>","detail":null}` stays the exit-1 documents' form.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the `cli` catch site's document under `--json`
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Error sanitization and secret scan); §4 Unit Test Strategy (unit-test-strategy.md: the root bin)
**Change:**
- New rule: under `--json` the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout and leaves stderr empty, witnessed by `cli_verbs_print_the_internal_error_document_under_json` (`tests/cli_wait_last.rs`) and `a_frame_verb_that_cannot_open_its_log_is_internal_error` (`tests/cross_exit_causes.rs`).
- Without `--json` the line `error: internal error` stands byte for byte, and `run`, `revive`'s start, `mcp` and `hook` stay silent there.
- `json_asked` (`src/main.rs`) reads `--json` from argv only before a `--`, held by `json_asked_reads_the_flag_before_a_separator`.
**Why:** the founder's own word, `inputs#I2` Q1 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him): the catch site's document under `--json` replaces its stderr line, exit 1 and the human line staying. The e2e item, read as contradicting a standing line, was applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the exit-cause table is built
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's head, As landed and self-checks; test-plan.md: Exit-cause matrix, Security control negatives); §7 Test Data (test-data-fixtures.md: Human-mode expected output)
**Change:**
- The matrix reads built (was `cross_exit_causes.rs` not existing, no rstest table): `tests/cross_exit_causes.rs`, a root `cross_` binary, `required-features = ["fake-agent"]`, on the `ci` profile's kill line, rstest `#[case]` by mode, one negative test per cause it holds.
- Every case asserts the exit code, the `--json` document and a last `hint:` line naming its cause, a fault carrying none. Each cause is asserted in both modes where a mode exists, with each logged cause's one `process-exit` line and its own `detail`.
- Exact strings are literals in the file's constants (was pinned with trycmd once the product defined them); `CAUSES`' codes, exits and last `hint:` lines join the Human-mode row.
- The causes its top comment names outside the table are server verification, stale heartbeat, the `.cmd` human pair, `verify`'s version, recording, pinned-copy, interrupted and stamps refusals, and `viola mcp`'s two silent exits: each a case of the binary its row names.
- Self-checks: `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`.
- Security control negatives' body gains `viola list`'s whole-board refusal opening no log and `viola plugin install`'s creating nothing (`strict_modes_on_the_board_is_exit_21_and_opens_no_log`, `strict_modes_on_plugin_install_is_exit_21_and_creates_nothing`).
**Why:** the chunk built the table and retired the matrix's not-built lines. `every_code_is_a_code_word` was added beyond the table's planned list. The both-modes item, read as contradicting a standing line, was applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL.
**Kept:** a cause named outside the table keeps its own binary's case, so the matrix's remaining "not a `cross_exit_causes.rs` row" lines, for the link verbs' two cases and the server identity's, stand.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a stopped instance and an unwrapped name
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's exit-21 list; test-plan.md: Scenario E1); §4 Unit Test Strategy (unit-test-strategy.md: viola-state's `without_endpoint`)
**Change:**
- No instance running for the name: the endpoint is gone after `cleanup` with the instance's directory standing, `detail` `instance-dead`, the table's `a_stopped_instance_is_instance_dead`.
- The name holds no instance directory, as an unwrapped `claude agents --json` row or a name never started: `detail` `unwrapped-name`, its own hint; `a_name_never_started_is_unwrapped_name` and `e1_send_naming_an_unwrapped_row_is_unreachable` hold it (was exit 21 only, its hint not built, owed to "Exit-cause code catalogue").
- `viola_state::liveness::without_endpoint(instance_dir)` reads by `symlink_metadata` a directory as `InstanceDead` and anything else (absent, a regular file, a symlink) as `UnwrappedName`, opening nothing and writing no line; `without_endpoint_maps_the_directory_reading` (`src/cmd/client.rs`) holds the client's mapping.
**Why:** the cause in `detail` is the founder's own word, `inputs#I2` Q1 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him). Unwrapped reading by the instance directory is `inputs#I2` Q2, delegate the overseer, PROVISIONAL. This chunk built the hint the E1 line owed, so its owed clause is spent.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a wrapper lost under an open call
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's exit-21 list, the Chaos suite; test-plan.md: Chaos suite); §8 Mocking (mocking-stubbing-discipline.md: Process faults); §1 Test Scope Summary (test-scope-summary.md: the chaos trigger)
**Change:**
- New matrix row: a wrapper lost with the call open is `detail` `endpoint-lost`, `during:"call"`, its own hint, the table's `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during:"connect"`.
- The chaos line and the chaos trigger for an endpoint vanishing during `wait` name `endpoint-lost` (was exit 21, `instance-unreachable`, with no cause); `tests/chaos_wait_vanish.rs` kills through `support::start::kill_wrapper`.
- The new Process faults row: a wrapper lost under an open call is a real kill, `kill_wrapper` (`tests/support/start.rs`) for the table's lost row and for `tests/chaos_wait_vanish.rs`, `kill_by_pid` (`crates/viola-e2e/tests/mcp_tools.rs`) for the MCP case; no double stands in.
**Why:** a wrapper that goes away mid-call is a cause of its own, apart from a dead instance, whose `during` moved to `connect`. The cause word is the founder's own, `inputs#I2` Q1 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him); the lost cause's hint is `inputs#I2` Q3, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — a stale holder reads `stale-heartbeat`
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's exit-1 list and `already-live`, the Chaos suite; test-plan.md: Chaos suite); §5 Integration Test Strategy (integration-test-strategy.md: `Frozen`); §8 Mocking (mocking-stubbing-discipline.md: Process faults); §1 Test Scope Summary (test-scope-summary.md: the run start path)
**Change:**
- A stale holder refuses the start exit 1 with `stale-heartbeat`, printed and logged (was `already-live`); a live holder or a fresh replayed beat keeps `already-live` (`a_live_holder_is_already_live`).
- A frame verb against a stale instance reads `instance-dead`, because `live_endpoint` gives none for a stale wrapper; stale heartbeat is exit 1's cause alone.
- Witnesses: `run_refuses_a_stale_name` and `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document` (`tests/cli_instance_state.rs`).
- `Frozen` is a guard on Windows and on Unix, a `SIGSTOP` on Unix; both stale cases stand on it, and the Process faults row names it: a stale holder is a real wrapper, no double.
- The SIGSTOP chaos item and the body's chaos line name `stale-heartbeat` on the refused `run`.
**Why:** the stale holder's own code retires `already-live` as its refusal. `Frozen` was Windows-only, so a stale case on it would have read nothing on Linux or macOS. The scope summary's item, read as contradicting a standing line, was applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `viola revive --json`'s ten codes and the closed gap
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: the Exit-cause matrix's exit-1 `revive` causes); §1 Test Scope Summary (test-scope-summary.md: the CLI verbs entity, the cli surface)
**Change:**
- A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints one exit-1 document under `revive --json`, before `spawn_child`, each a table row: `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`, `a_tampered_pinned_copy_is_pinned_hash_mismatch`, `a_held_endpoint_is_squatted_name`, `a_revive_with_no_claude_on_its_path_is_internal_error` (was no document, owed to "Exit-cause code catalogue").
- Its codes read ten (was five): `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`; its exit 1 now also covers a refusal of the start.
- A `pump_child` error prints no document, the child having held the terminal; a program not found at the start is the `internal-error` row.
- The script-child row is `verification-matrix.json#v1-39`'s Windows witness; `run` passes `json` false to `Launch`, so its human pairs stand byte for byte.
- The `tests/cli_revive.rs` cases stand beside the table's `strict_modes_on_a_start_is_exit_1_and_opens_no_log`, `a_revive_with_no_such_session_is_no_session`, `a_session_open_elsewhere_is_session_live` and `a_recorded_directory_gone_is_cwd_missing` (was not rows of the table).
**Why:** the founder's own word, `inputs#I2` Q4 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him): `run` takes no `--json`, so its start's causes reach the log and the hint and, for that start, `revive --json`. The gap was the CARRY of chunk 2026-10-17-cli-machine-contract. The rows' reading inside his answer and the ten-code, `pump_child` and witness items (Q1, Q5) are `inputs#I6`, delegate the overseer, PROVISIONAL; the ratification stays the founder's.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the socket-directory refusal and macOS's server check stay with their route entry
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Security control negatives, the Unix socket directory refusal); §1 Test Scope Summary (test-scope-summary.md: the server verification)
**Change:**
- New: the socket-directory refusal has no code; `bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site, owned by the route entry "Unix endpoint and home hardening" with macOS's server check.
- The macOS server-verification cases read owed to that route entry (was owed to §6 Exit-cause matrix).
**Why:** `v1-18`'s requirement listed a socket-directory refusal the code does not hold. The founder's own word, `inputs#I2` Q4 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him), keeps both carve-outs with this route entry; the row leaving that code out of the chunk is `inputs#I3`, delegate the overseer, 2026-10-31, PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the bare temp-dir home and the liveness-check case
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: Setup / teardown lifecycle, Bare temp-dir home)
**Change:**
- New: a bare temp dir reads 0755 under the umask and the read rule refuses it; `tool_inputs_that_pass_reach_the_liveness_check` reaches the check it names from a viola-made `empty_home` and reads `unwrapped-name`.
**Why:** before this chunk the case's bare temp dir never reached the liveness check, its strict-modes refusal sharing one body with a dead instance. The trap for later chunks: an oracle moved by a shared body can hide a test that never reached its subject.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — `viola verify`'s refusal codes
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: the verify items); §7 Test Data (test-data-fixtures.md: the fixture-hygiene group)
**Change:**
- `viola verify` writes its refusal's code on the `process-exit` it writes under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed` and `unclean-fixture`, beside `interrupted`.
- A refused home tree under `verify` stays lineless, a failing row is no refusal and no code, the fixture's file name enters no code, and `verify` takes no `--json`.
- Witnesses: `verify_a_tampered_pinned_copy_refuses_with_its_code` and `verify_window_a_widened_stamps_file_refuses_with_its_code` (`tests/cli_verify.rs`), the second under the `verify_window_` prefix because it runs two whole verifies, and `verify_with_no_claude_on_its_path_is_program_not_found` (`tests/cross_exit_causes.rs`).
- A recording refused as unclean is `verify`'s `Ended::Refused`: its `process-exit` reads `detail` `unclean-fixture`, a member of the `process-exit` `detail` enum in `schemas/diag-line.v1.json`.
**Why:** the founder's own word, `inputs#I2` Q4 (live, 2026-10-31 before 16:52Z, through the overseer's dialog, given after the widening was shown to him): `verify` has no `--json`, so its causes reach the log and the hint.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the root `cross_` binary and the shared start helpers
**Section:** §2 Test Strategy (test-strategy.md: the source layout; test-plan.md: File naming)
**Change:**
- `cross_` was a `viola-e2e` prefix alone; the root package now also holds a `cross_` binary, `tests/cross_exit_causes.rs` (sync, `[[test]]` `required-features = ["fake-agent"]`), which the E2E layer regex selects as it selects `path1_`.
- New: `tests/support/start.rs` shares the start setups across root test files: `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper`.
**Why:** the new table is a root `cross_` binary, which the E2E regex takes as it took `path1_`, and no harness verb moved for it; the setups moved to `tests/support/start.rs` because a root test file holds no copy of its own of what two share.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/

## 2026-10-31-exit-cause-code-catalogue — the closed causes at the unit tier and the two built hints
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: viola-core, the root bin's server check and hints)
**Change:**
- New: `viola_core::UnreachableCause` holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost` and `ServerVerify` and has no serde path; `unreachable_cause_is_each_its_own_code_word` holds its words.
- `unreachable_logs_its_cause` logs four closed `Unreachable` causes, `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed` (was two); `endpoint-lost` is `during` `call`, the other three `during` `connect`.
- New: the `unwrapped-name` and `endpoint-lost` hints are built in `src/human/hints.rs`, held by `send_hint_is_the_design_string`; the hint cause of `instance-dead` keys on `not-running`, a word apart from the log's.
**Why:** every word is written by the two clients. A strict-modes check refused before a log opens writes no line (`inputs#I2` Q4, the founder's own word, live, 2026-10-31 before 16:52Z, through the overseer's dialog), and the logged causes are the other four. The `UnreachableCause` item, read as contradicting a standing line, was applied at the halt's Q5, `inputs#I6`, delegate the overseer, PROVISIONAL. The `endpoint-lost` hint is `inputs#I2` Q3, delegate the overseer, PROVISIONAL.
**Kept:** `not-running` stays the hint cause's key, not renamed to the log's `instance-dead`.
**Ref:** .andromeda/runs/2026-10-31T18-12-32-wrap/
