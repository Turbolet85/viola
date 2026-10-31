proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 76
    section: '## 6. E2E Test Strategy › #### Scenario: Path 3 — `wait` / `last` event-driven readback (§6 live flow)'
    change: 'Add after this line: As landed (chunk 2026-10-31-exit-cause-code-catalogue), `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` (`crates/viola-e2e/tests/mcp_tools.rs`) witnesses MCP''s lost cause, and the `mcp_tools` row after `cleanup` reads its cause from the directory: a home kept by `AGENT_RUN_KEEP_HOMES` reads `instance-dead`, a removed one `unwrapped-name`, its body still compared with the CLI''s.'
    sidecar: 'Path 3 gains MCP''s lost-cause witness and the directory reading of the row after cleanup.'
    rationale: 'Report Deviations 5 and Outcome (MCP''s body less v) name both; the task''s F9.'
    fact: F9
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 91
    section: '## 6. E2E Test Strategy › #### Scenario: Path 3 — `wait` / `last` event-driven readback (§6 live flow)'
    change: 'Step 7: CLI exit 21 with `{"error":"instance-unreachable","detail":"<cause>"}`: `endpoint-lost` for a wrapper killed under a parked `wait`, and after `cleanup` `instance-dead` in a kept home or `unwrapped-name` in a removed one; MCP `is_error:true` with the same body (the CLI document less `v`), text `refused: instance-unreachable`.'
    sidecar: 'Path 3 step 7 reads each cause in detail, no longer detail null.'
    rationale: 'Report Retired names the exit-21 document''s detail null; Symbols / APIs gives the causes and MCP''s same body less v; Deviation 5 gives the row after cleanup.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 260
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line, as a bullet: Under `--json` the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout and leaves stderr empty, witnessed by `cli_verbs_print_the_internal_error_document_under_json` (`tests/cli_wait_last.rs`) and `a_frame_verb_that_cannot_open_its_log_is_internal_error` (`tests/cross_exit_causes.rs`); without `--json` the line `error: internal error` stands byte for byte, and `run`, `revive`''s start, `mcp` and `hook` stay silent there.'
    sidecar: 'The error-body rule gains the catch site''s --json document and its witnesses.'
    rationale: 'Report Symbols / APIs, the cli catch site under --json; the task''s F8.'
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 274
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '- **Exit-cause matrix** (`tests/cross_exit_causes.rs`, a root `cross_` binary, `required-features = ["fake-agent"]`, on the `ci` profile''s kill line; rstest `#[case]` by mode, one negative test per cause). Exit 21 and exit 1 each have several causes. Every case asserts the exit code, the `--json` document, and a last stderr `hint:` line that names that cause, a fault carrying none. A shared hint across causes fails the table, and the exact strings are literals in the file''s constants.'
    sidecar: 'The matrix head names the built table and replaces the trycmd clause with its literals.'
    rationale: 'Report Harness / gate surface and the cross-table row name this line; New text lists the hint constants and the `#[case] json` cases; Outcome holds a fault with no hint.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:274
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 275
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'As landed (chunk 2026-10-31-exit-cause-code-catalogue), `tests/cross_exit_causes.rs` is built, 39 tests on Linux; a cause its top comment names outside the table (server verification, stale heartbeat, the `.cmd` human pair, `verify`''s version, recording, pinned-copy, interrupted and stamps refusals, `viola mcp`''s two silent exits) stays a case of the binary its row names.'
    sidecar: 'The does-not-exist line becomes the built table and what stands outside it.'
    rationale: 'Report Retired names the matrix''s not-built lines; the cross-table row names this line and the causes outside the table; Outcome reads 39 tests on Linux.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:275
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 275
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line: `tests/cross_exit_causes.rs` asserts each cause in both modes where a mode exists: the document under `--json`, the human form ending in its hint, and the cause''s one `process-exit` line with its own `detail`.'
    sidecar: 'The matrix states what each table row asserts in each mode.'
    rationale: 'Report Outcome, each logged cause one process-exit and the hints last on stderr; the task''s F2.'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 276
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line: As landed, exit 21''s five closed `detail` causes are `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost` and `server-verify-failed`, on the frame verbs, `list` and `plugin install`, with `error` still `instance-unreachable` and MCP''s text staying `refused: instance-unreachable`.'
    sidecar: 'The exit-21 list names its five closed causes.'
    rationale: 'Report Symbols / APIs, the exit-21 document moved; the task''s F1.'
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 279
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'As landed for `send`, `wait`, `last`, `answer`, `pause` and `release`, the Security control negatives'' strict-modes case (`tests/security_negatives_strict_modes.rs`, the three CI OSes), beside the table''s `strict_modes_on_a_frame_verb_is_exit_21_and_opens_no_log`: a widened instance is refused first, exit 21 with `link`''s instance-state pair or the `instance-unreachable` document, `detail` `strict-modes-failed`, no frame and no line.'
    sidecar: 'The frame verbs'' strict-modes case stands beside the table''s row and reads its cause.'
    rationale: 'Report New text lists the table''s strict-modes frame-verb case, so the cause is no longer outside the table; strict-modes is not among the cross-table row''s causes named outside it.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 282
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'End the line: its `--json` document is `{"v":1,"error":"instance-unreachable","detail":"server-verify-failed"}`.'
    sidecar: 'The server-verification cause reads its own word in detail.'
    rationale: 'Report Retired names the exit-21 document''s detail null; the shared document of every cause is gone.'
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 283
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'no instance is running for the name: the endpoint is gone after `cleanup` with the instance''s directory standing, `detail` `instance-dead`, the table''s `a_stopped_instance_is_instance_dead`; for `viola link` / `viola unlink`, as landed a `tests/cli_links.rs` case, `link_verbs_against_a_stopped_instance_exit_21` (both verbs in both modes: the not-running hint last, or the `instance-unreachable` document with stderr empty), not a `cross_exit_causes.rs` row'
    sidecar: 'The not-running cause names its code and its table row.'
    rationale: 'Report Symbols / APIs, `without_endpoint` reads a directory as InstanceDead; New text lists the table''s row.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 284
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'the name holds no instance directory, as an unwrapped `claude agents --json` row, never a target (E1), or a name never started: `detail` `unwrapped-name`, its hint built; as landed the table''s `a_name_never_started_is_unwrapped_name` and the `tests/cli_list.rs` case `e1_send_naming_an_unwrapped_row_is_unreachable`'
    sidecar: 'The unwrapped-name cause is built; its not-built clause and its owed-to debt are spent.'
    rationale: 'Report Retired names the unwrapped-name hint''s not-built line; the causes-new row names this line.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:284
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 284
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line, as a list item: a wrapper lost with the call open: `detail` `endpoint-lost`, `during:"call"`, its own hint, the table''s `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during:"connect"`.'
    sidecar: 'The exit-21 list gains the endpoint-lost cause and each cause''s during.'
    rationale: 'Report Symbols / APIs, a process-exit during; Retired names instance-dead with during call; the list holds no lost-wrapper cause.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 287
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line: As landed, this cause prints and logs `stale-heartbeat`, not `already-live`. A frame verb against a stale instance reads `instance-dead`, because `live_endpoint` gives none for a stale wrapper; stale heartbeat is exit 1''s cause alone.'
    sidecar: 'The stale-heartbeat cause names its code and why a frame verb reads instance-dead.'
    rationale: 'Report Symbols / APIs, a stale holder''s code; the task''s F4.'
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 292
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace `not rows of `cross_exit_causes.rs`` with `beside the table''s rows `strict_modes_on_a_start_is_exit_1_and_opens_no_log`, `a_revive_with_no_such_session_is_no_session`, `a_session_open_elsewhere_is_session_live` and `a_recorded_directory_gone_is_cwd_missing``; the rest of the line stands.'
    sidecar: 'Revive''s preflight causes are now rows of the table beside their own cases.'
    rationale: 'Report New text lists these table cases; none of these causes is among those the cross-table row names outside the table.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 293
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'A refusal `run`''s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints one exit-1 document under `revive --json`, before `spawn_child`, each a row of the table (`a_revive_whose_claude_is_a_script_is_the_batch_script_child_document`, `a_tampered_pinned_copy_is_pinned_hash_mismatch`, `a_held_endpoint_is_squatted_name`, `a_revive_with_no_claude_on_its_path_is_internal_error`).'
    sidecar: 'The revive --json post-preflight gap is closed and its owed clause spent.'
    rationale: 'Report Retired names the gap; Symbols / APIs says the sentence is no longer true; Deviation 6 names the table''s four start rows.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 293
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line: `viola revive --json` prints one exit-1 document for `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and `internal-error`.'
    sidecar: 'Revive''s ten --json codes are listed.'
    rationale: 'Report Symbols / APIs, viola revive --json; the task''s F5.'
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 293
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line: A `pump_child` error under `revive --json` prints no document, because the child has held the terminal; a program not found at the start is `internal-error`, witnessed by `a_revive_with_no_claude_on_its_path_is_internal_error`.'
    sidecar: 'A pump_child error stays documentless; a program not found is internal-error.'
    rationale: 'Report Symbols / APIs and Reverted / negative API facts; the task''s F6.'
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 293
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line: `a_revive_whose_claude_is_a_script_is_the_batch_script_child_document` is `verification-matrix.json#v1-39`''s Windows witness, and `run` passes `json` false to `Launch` so its human pairs stand byte for byte.'
    sidecar: 'The batch-script document names its Windows witness, and run''s pairs stay byte for byte.'
    rationale: 'Report Outcome on v1-39 and Symbols / APIs on Launch; the task''s F7.'
    fact: F7
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 295
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add at the end of the item: `already-live` is a live holder''s and a fresh replayed beat''s code; a stale holder in the same check prints and logs `stale-heartbeat` (`revive_json_of_a_stale_holder_is_the_stale_heartbeat_document`).'
    sidecar: 'A stale holder under revive no longer reads already-live.'
    rationale: 'Report Retired names already-live as a stale holder''s refusal; New text lists the stale document''s case.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 300
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line: The table''s self-checks are `no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word`.'
    sidecar: 'The matrix names the table''s self-checks.'
    rationale: 'Report Outcome and Deviation 7; New text lists all four; the task''s F3.'
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 307
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after this line: The socket-directory refusal has no code in chunk 2026-10-31-exit-cause-code-catalogue: `bind_endpoint`''s `create_private_dir` failure is an anyhow error at the catch site, left to the route entry "Unix endpoint and home hardening" with macOS''s server check.'
    sidecar: 'The socket-directory refusal is recorded as codeless and left to its route entry.'
    rationale: 'Report Spec claims disproved by measurement, and Outcome on v1-18''s carve-outs; the task''s F10.'
    fact: F10
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 319
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace the document with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`; the rest of the line stands.'
    sidecar: 'The frame verbs'' strict-modes negative reads its own word in detail.'
    rationale: 'Report Retired names the exit-21 document''s detail null; Symbols / APIs says a strict-modes site reads `strict-modes-failed`, no frame and no line unmoved.'
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 339
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'endpoint vanishes during `wait`: exit 21, the `--json` `detail` `endpoint-lost` (`a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`); `tests/chaos_wait_vanish.rs` kills through `support::start::kill_wrapper`'
    sidecar: 'A vanished endpoint under wait names its cause and the shared kill.'
    rationale: 'Report Symbols / APIs, a wrapper lost with the call open; Deviation 1 moves the file''s kill to `support::start::kill_wrapper`.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 340
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'SIGSTOP the wrapper (Unix) for over 5 s: `liveness:"stale"`, and a new `run` exits 1 with `stale-heartbeat` (`run_refuses_a_stale_name`, on the `Frozen` guard); after SIGKILL: omitted as gone'
    sidecar: 'A stale name refused to run names its code and its case.'
    rationale: 'Report Symbols / APIs, a stale holder''s code; Deviation 2 stands `run_refuses_a_stale_name` on `Frozen`.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan.md
    near: 553
    section: '## 6. E2E Test Strategy › #### Scenario: E1 — R5 unwrapped hooks are a silent no-op'
    change: '  - A `send` to the unwrapped row''s name exits 21: a row `name` is never a target. As landed: `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable` (exit 21, its cause `unwrapped-name` and its hint built).'
    sidecar: 'E1''s line drops the not-built hint and its owed-to clause.'
    rationale: 'Report Retired names the unwrapped-name hint''s not-built line; the causes-new row names test-plan.md:553.'
    basis: .andromeda/test-plan.md:553
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan.md
    near: 610
    section: '## 6. E2E Test Strategy'
    change: 'Replace `(`cross_exit_causes.rs`, not built)` with `(`tests/cross_exit_causes.rs`, a root `cross_` binary)`, and `the server-verification cause is a case of` with `beside the table, the server-verification cause is a case of`; the rest of the line stands.'
    sidecar: 'The body''s matrix line reads built, with the named cases standing beside it.'
    rationale: 'Report Retired names the matrix''s not-built lines; the cross-table row names test-plan.md:610; Harness / gate surface says a root `cross_` binary.'
    basis: .andromeda/test-plan.md:610
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 619
    section: '## 6. E2E Test Strategy'
    change: 'Replace `an endpoint vanishing during `wait` (exit 21)` with `an endpoint vanishing during `wait` (exit 21, `endpoint-lost`)`, and `its name refused to a new `run`` with `its name refused to a new `run` `stale-heartbeat``; the rest of the line stands.'
    sidecar: 'The body''s chaos line names the lost cause and the stale code.'
    rationale: 'Report Symbols / APIs, a process-exit during and a stale holder''s code; the fresh replayed beat''s `already-live` on this line stands.'
note: 'Not this group''s: the naming line test-plan.md:324 (a root `cross_` binary) is test-strategy''s, the sites at test-plan.md:145, :553-adjacent §1 and §5 clauses are their own groups''; e2e-test-strategy.md:132''s `detail` null is another refusal''s, per the report, and is left alone.'
