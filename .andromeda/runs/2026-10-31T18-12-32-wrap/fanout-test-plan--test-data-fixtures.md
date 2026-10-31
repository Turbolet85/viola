proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 10
    section: "## 7. Test Data & Fixtures"
    change: "On-disk product state row: read `Four exceptions:` in place of `Three exceptions:`, and after `the server-identity impostor cases (below);` add `the MCP handler suite's own snapshots (`crates/viola-mcp/tests/handlers.rs`: `TestWrapper`, §5, and `record_live`, below);`."
    sidecar: "test-data-fixtures: the on-disk state row's exceptions move from three to four, the handler suite's test-written snapshots named (chunk 2026-10-31-exit-cause-code-catalogue)."
    rationale: "The report's deviation 4 and its New text (`record_live`, `let snapshot = InstanceSnapshot {`) add a test-written snapshot the row's closed count of exceptions does not hold; `TestWrapper` (.andromeda/test-plan.md:457) is the same suite's and was never listed, so the row names the suite rather than one helper."
    basis: crates/viola-mcp/tests/handlers.rs:64-86
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 30
    section: "## 7. Test Data & Fixtures"
    change: "New bullet after the impostor bullet: `- `crates/viola-mcp/tests/handlers.rs` `record_live(instance, endpoint, pid)` writes an `InstanceSnapshot` recording an endpoint no one ever bound, for the unserved row of `handlers_frame_tools_name_the_cause_of_an_unreachable_instance`: a Windows `Serving` has no `Drop` (its pipe lives in the accept thread until the process exits), so dropping a served one would leave the endpoint held (chunk 2026-10-31-exit-cause-code-catalogue).`"
    sidecar: "test-data-fixtures: `record_live`, the handler suite's snapshot of an endpoint no one bound, recorded beside the impostor cases."
    rationale: "Report deviation 4 and its New text for handlers.rs: a new test-written snapshot, with the OS reason it binds nothing, which this file's list of test-built state does not hold."
    basis: crates/viola-mcp/tests/handlers.rs:64-86
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 31
    section: "## 7. Test Data & Fixtures"
    change: "New sentence after `tests/tui_env_strip.rs` do.: ``tool_inputs_that_pass_reach_the_liveness_check` shows the cost: its bare temp dir read 0755 under the umask, so the read rule refused it and it never reached the check it names, a miss the one shared `detail: null` body hid until a code per cause exposed it; its `empty_home` is now viola-made and it reads `unwrapped-name` (chunk 2026-10-31-exit-cause-code-catalogue).`"
    sidecar: "test-data-fixtures: the viola-made home rule gains the vacuous case this chunk exposed, `tool_inputs_that_pass_reach_the_liveness_check`."
    rationale: "Report deviation 3 and its sweep hazard under Decisions: a measured witness of the rule this line states, and of why a test home made before viola can hide a test that never reached its subject."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 19
    section: "## 7. Test Data & Fixtures"
    change: "Human-mode expected output row: after the `viola link` / `viola unlink` clause, add to the literals `, the exit-cause table's codes, exits and last `hint:` lines (`tests/cross_exit_causes.rs` `CAUSES`)`, before `are literals in their test files`."
    sidecar: "test-data-fixtures: the exit-cause table's hint literals join the human-mode expected output row."
    rationale: "The report's new `tests/cross_exit_causes.rs` holds `CAUSES` and `assert_hint`; its Outcome names the table and `no_two_codes_share_a_hint`. The row lists the surfaces pinned by literal in their test files, and this one is new."
    basis: tests/cross_exit_causes.rs:69-92
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 24
    section: "## 7. Test Data & Fixtures"
    change: "New table row after the channel error-body witness row: `| A start's shared setups | `tests/support/start.rs`: `first_life`, `stopped_life`, `made_dir`, `plant_agents`, `listed`, `revive`, `FIXTURE_SESSION` (the exit-cause table's `session-live` row), `remove_dir_once_free` (its `cwd-missing` row) and `kill_wrapper` (its lost row, and `tests/chaos_wait_vanish.rs`), used by `tests/cli_revive.rs` and `tests/cross_exit_causes.rs`; no root test file keeps its own copy of what two share (§2) | per-test |`"
    sidecar: "test-data-fixtures: `tests/support/start.rs` recorded as the home of the shared start setups."
    rationale: "Report Counts `frozen-guard` (`tests/support/start.rs` takes the start setups), deviation 1, and its New text for the file: a new shared home of test setups, which this file's helper rows do not name."
    basis: tests/support/start.rs:71-174
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 81
    section: "## 7. Test Data & Fixtures"
    change: "New bullet after the walk's bullet, in the fixture-hygiene group: `- A recording refused as unclean is `verify`'s `Ended::Refused`: under a parsed `VIOLA_NAME` its `process-exit` reads `detail` `unclean-fixture`, an enum member of `schemas/diag-line.v1.json` since chunk 2026-10-31-exit-cause-code-catalogue, and the fixture's `<file>` enters no code.`"
    sidecar: "test-data-fixtures: the unclean recording's log code, `unclean-fixture`, recorded in the fixture-hygiene group."
    rationale: "Report Symbols, `verify`'s codes, and Schema / config: a dirty fixture's refusal gains its own closed code; line 17 and this group state the refusal and hold no code for it."
    basis: schemas/diag-line.v1.json:78
