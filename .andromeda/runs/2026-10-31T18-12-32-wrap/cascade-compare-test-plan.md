# Cascade compare — test-plan · wrap · 2026-10-31-exit-cause-code-catalogue
E items 56 · held 14 · lost at classify 8 · lost at strong 4 · beside a placed fact 0 · cascade only 4 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### ci-integration#1.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 11
    section: "## 9. CI Integration"
    change: Add after line 11, in its one-space continuation form, At chunk 2026-10-31-exit-cause-code-catalogue the recipe did not run before the first push, and `lint (macos-latest)` and `test (macos-latest)` went red on `115cdbb9` (`ci#41225642322`) on one compile error, an every-OS case in `crates/viola-mcp/tests/handlers.rs` calling a helper gated off macOS; on the fixed tree the recipe reads exit 0 (2026-10-31T17:42:00Z), and CI reads green on `8a4c1a2c` (`ci#41226917636`).
    sidecar: test-plan CI Integration gains the macOS check-only clippy recipe's reading at chunk 2026-10-31-exit-cause-code-catalogue, unrun before the first push, CI's first read red on the compile error, exit 0 on the fixed tree.
    rationale: The report's Deviations item 8, Decisions (the cfg-gate sweep hazard) and Cross-project claims bring a new reading of the recipe this file's line 11 describes; the recipe's standing stays as line 11 says, and the report records no skipped gate.

### mocking-stubbing-discipline#2.3 · lost at strong · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 18
    section: "## 8. Mocking & Stubbing Discipline"
    change: Add this table row after the Process mocking row | Process faults | none (a real wrapper, a real signal or kill) | A stale holder is a real wrapper under `Frozen` (`tests/cli_instance_state.rs:149-181`, Windows-only before chunk 2026-10-31-exit-cause-code-catalogue, on Unix a `SIGSTOP` guard), and `run_refuses_a_stale_name` stands on it; a wrapper lost under an open call is a real kill, `support::start::kill_wrapper` (`tests/support/start.rs:149-173`) for `tests/cross_exit_causes.rs`' lost row and `tests/chaos_wait_vanish.rs`, `kill_by_pid` (`crates/viola-e2e/tests/mcp_tools.rs:931-939`) for `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`; no double stands in for either |
    sidecar: §8 detail gains a Process faults row, the `Frozen` guard on Unix and the shared `kill_wrapper`, real process faults behind the stale and lost causes (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Deviations 1 and 2 add a Unix `SIGSTOP` `Frozen` guard and move the wrapper kill into `tests/support/start.rs`, and the file's Process mocking row names only the held live process as a real-process stand-in.
    basis: tests/cli_instance_state.rs:149-181

### mocking-stubbing-discipline#2.4 · lost at strong · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 19
    section: "## 8. Mocking & Stubbing Discipline"
    change: Add this table row after the Fake servers row | Unserved endpoints | none (a test-built snapshot, no listener) | In `crates/viola-mcp/tests/handlers.rs` the unserved row of `handlers_frame_tools_name_the_cause_of_an_unreachable_instance` binds nothing and records, through `record_live` (`crates/viola-mcp/tests/handlers.rs:64-86`), an endpoint no one ever bound, because a Windows `Serving` has no `Drop` and its pipe lives in the accept thread until the process exits |
    sidecar: §8 detail gains an Unserved endpoints row, the `handlers` row that records an endpoint no one bound through `record_live` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Deviation 4 states this stand-in and its Windows reason, and the file's Fake servers row names only endpoints a listener holds.
    basis: crates/viola-mcp/tests/handlers.rs:64-86

### test-data-fixtures#1.1 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:10
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 10
    section: "## 7. Test Data & Fixtures"
    change: "On-disk product state row: read `Four exceptions:` in place of `Three exceptions:`, and after `the server-identity impostor cases (below);` add `the MCP handler suite's own snapshots (`crates/viola-mcp/tests/handlers.rs`: `TestWrapper`, §5, and `record_live`, below);`."
    sidecar: "test-data-fixtures: the on-disk state row's exceptions move from three to four, the handler suite's test-written snapshots named (chunk 2026-10-31-exit-cause-code-catalogue)."
    rationale: "The report's deviation 4 and its New text (`record_live`, `let snapshot = InstanceSnapshot {`) add a test-written snapshot the row's closed count of exceptions does not hold; `TestWrapper` (.andromeda/test-plan.md:457) is the same suite's and was never listed, so the row names the suite rather than one helper."
    basis: crates/viola-mcp/tests/handlers.rs:64-86

### test-data-fixtures#2.1 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 30
    section: "## 7. Test Data & Fixtures"
    change: "New bullet after the impostor bullet: `- `crates/viola-mcp/tests/handlers.rs` `record_live(instance, endpoint, pid)` writes an `InstanceSnapshot` recording an endpoint no one ever bound, for the unserved row of `handlers_frame_tools_name_the_cause_of_an_unreachable_instance`: a Windows `Serving` has no `Drop` (its pipe lives in the accept thread until the process exits), so dropping a served one would leave the endpoint held (chunk 2026-10-31-exit-cause-code-catalogue).`"
    sidecar: "test-data-fixtures: `record_live`, the handler suite's snapshot of an endpoint no one bound, recorded beside the impostor cases."
    rationale: "Report deviation 4 and its New text for handlers.rs: a new test-written snapshot, with the OS reason it binds nothing, which this file's list of test-built state does not hold."
    basis: crates/viola-mcp/tests/handlers.rs:64-86

### test-data-fixtures#2.2 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 31
    section: "## 7. Test Data & Fixtures"
    change: "New sentence after `tests/tui_env_strip.rs` do.: ``tool_inputs_that_pass_reach_the_liveness_check` shows the cost: its bare temp dir read 0755 under the umask, so the read rule refused it and it never reached the check it names, a miss the one shared `detail: null` body hid until a code per cause exposed it; its `empty_home` is now viola-made and it reads `unwrapped-name` (chunk 2026-10-31-exit-cause-code-catalogue).`"
    sidecar: "test-data-fixtures: the viola-made home rule gains the vacuous case this chunk exposed, `tool_inputs_that_pass_reach_the_liveness_check`."
    rationale: "Report deviation 3 and its sweep hazard under Decisions: a measured witness of the rule this line states, and of why a test home made before viola can hide a test that never reached its subject."

### test-data-fixtures#1.10 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 19
    section: "## 7. Test Data & Fixtures"
    change: "Human-mode expected output row: after the `viola link` / `viola unlink` clause, add to the literals `, the exit-cause table's codes, exits and last `hint:` lines (`tests/cross_exit_causes.rs` `CAUSES`)`, before `are literals in their test files`."
    sidecar: "test-data-fixtures: the exit-cause table's hint literals join the human-mode expected output row."
    rationale: "The report's new `tests/cross_exit_causes.rs` holds `CAUSES` and `assert_hint`; its Outcome names the table and `no_two_codes_share_a_hint`. The row lists the surfaces pinned by literal in their test files, and this one is new."
    basis: tests/cross_exit_causes.rs:69-92

### test-data-fixtures#1.15 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:24
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 24
    section: "## 7. Test Data & Fixtures"
    change: "New table row after the channel error-body witness row: `| A start's shared setups | `tests/support/start.rs`: `first_life`, `stopped_life`, `made_dir`, `plant_agents`, `listed`, `revive`, `FIXTURE_SESSION` (the exit-cause table's `session-live` row), `remove_dir_once_free` (its `cwd-missing` row) and `kill_wrapper` (its lost row, and `tests/chaos_wait_vanish.rs`), used by `tests/cli_revive.rs` and `tests/cross_exit_causes.rs`; no root test file keeps its own copy of what two share (§2) | per-test |`"
    sidecar: "test-data-fixtures: `tests/support/start.rs` recorded as the home of the shared start setups."
    rationale: "Report Counts `frozen-guard` (`tests/support/start.rs` takes the start setups), deviation 1, and its New text for the file: a new shared home of test setups, which this file's helper rows do not name."
    basis: tests/support/start.rs:71-174

### test-data-fixtures#7.1 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:81
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 81
    section: "## 7. Test Data & Fixtures"
    change: "New bullet after the walk's bullet, in the fixture-hygiene group: `- A recording refused as unclean is `verify`'s `Ended::Refused`: under a parsed `VIOLA_NAME` its `process-exit` reads `detail` `unclean-fixture`, an enum member of `schemas/diag-line.v1.json` since chunk 2026-10-31-exit-cause-code-catalogue, and the fixture's `<file>` enters no code.`"
    sidecar: "test-data-fixtures: the unclean recording's log code, `unclean-fixture`, recorded in the fixture-hygiene group."
    rationale: "Report Symbols, `verify`'s codes, and Schema / config: a dirty fixture's refusal gains its own closed code; line 17 and this group state the refusal and hold no code for it."
    basis: schemas/diag-line.v1.json:78

### test-scope-summary#8.1 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:134
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 134
    section: '## 1. Test Scope Summary'
    change: 'New Verification signal row: `    - A stale holder refuses the start exit 1 with `stale-heartbeat`, a live holder or a fresh replayed beat with `already-live`; a frame verb against a stale instance still reads `instance-dead`, stale heartbeat being the cause of exit 1 alone (`tests/cli_instance_state.rs` `run_refuses_a_stale_name`, `revive_json_of_a_stale_holder_is_the_stale_heartbeat_document`, `tests/cross_exit_causes.rs` `a_live_holder_is_already_live`).'
    sidecar: test-scope-summary §1 run start path gains the stale holder code (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report bullet A stale holder code, and Retired names `already-live` as a stale holder refusal; the path signal states only that a duplicate run exits 1.
    basis: tests/cli_instance_state.rs:190-222

### test-scope-summary#10.1 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:182
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 182
    section: '## 1. Test Scope Summary'
    change: 'The row reads: endpoint vanishes during `wait`: `instance-unreachable`, `detail` `endpoint-lost` (`tests/cross_exit_causes.rs` `a_wrapper_killed_under_a_parked_wait_is_endpoint_lost`).'
    sidecar: test-scope-summary §1 chaos trigger, the vanished endpoint names its cause (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report states a wrapper lost with the call open is `endpoint-lost`; the row stays true and gains the cause.
    basis: tests/cross_exit_causes.rs:328-361

### test-strategy#3.6 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:63
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 63
    section: "## 2. Test Strategy"
    change: In the Filesystem (V7) row, at the six frame verbs' refusal, read "exit 21 with the instance pair or the `instance-unreachable` document, its `detail` `strict-modes-failed`, no frame and no line", the rest of the row unchanged.
    sidecar: test-plan §2 trigger map, the strict-modes refusal of the six frame verbs names its `detail`, `strict-modes-failed`, now that exit 21's document is one per cause (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Symbols / APIs moves exit 21 from one document to one per cause, and its Counts row `doc-null` gives a strict-modes site the word `strict-modes-failed`; the row's bare "the `instance-unreachable` document" names the form this chunk split.
    basis: viola-0.1.0/chunks/2026-10-31-exit-cause-code-catalogue/report.md:29

## Held

- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- e2e-test-strategy#17.1
- e2e-test-strategy#4.3
- integration-test-strategy#4.1
- integration-test-strategy#4.2
- integration-test-strategy#5.1
- test-scope-summary#1.1
- test-scope-summary#3.1
- test-scope-summary#4.1
- test-scope-summary#5.3
- test-strategy#2.1
- unit-test-strategy#1.1
- unit-test-strategy#2.1

## Cascade only

- e2e-test-strategy#13.1
- e2e-test-strategy#4.1
- integration-test-strategy#3.0
- unit-test-strategy#1.3
