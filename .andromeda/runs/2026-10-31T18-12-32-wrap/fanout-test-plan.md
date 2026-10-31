proposals:
  - detector: D-retired
    severity: warning
    section: §6 E2E Test Strategy, Exit-cause matrix
    change: 'The line no longer reads `cross_exit_causes.rs`, not built: it reads `tests/cross_exit_causes.rs`, built, a root `cross_` binary with one negative test per cause of exit 21 and of exit 1 it holds, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause; `no_two_codes_share_a_hint` is the shared-hint failure with `shared_hints_reads_a_planted_duplicate` its control, and `no_hint_names_release` holds the release ban; the causes named outside the table are those of its comment (server verification, stale heartbeat, the `.cmd` human pair, the version, recording, pinned-copy, interrupted and stamps refusals of `verify`, and the two silent exits of `viola mcp`); the clauses naming `tests/security_negatives_server_identity.rs` with its macOS case owed to "Unix endpoint and home hardening", `tests/security_negatives_strict_modes.rs`, `tests/run_cli.rs`, `tests/cli_verify.rs` and `tests/cli_revive.rs` stay.'
    sidecar: 'test-plan §6 Exit-cause matrix: `tests/cross_exit_causes.rs` is built, a root `cross_` binary; its not-built status retired, the causes outside the table named (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Retired** names the "not built" line of the exit-cause matrix, and Counts `cross-table` lists `test-plan.md:610` as changed; Changes names `tests/cross_exit_causes.rs` (567 lines) as new, and the same row names the causes that stay outside the table.'
    basis: .andromeda/test-plan.md:610
  - detector: D-retired
    severity: warning
    section: §6 E2E Test Strategy, Scenario E1 — R5 unwrapped hooks are a silent no-op
    change: 'The As-landed clause no longer reads the unwrapped-name hint not built, owed to "Exit-cause code catalogue": it reads exit 21, the cause `unwrapped-name` under `--json` and its hint built, the name never started held by the table case `a_name_never_started_is_unwrapped_name` in `tests/cross_exit_causes.rs`, with `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable` still the witness that a row `name` is never a target.'
    sidecar: 'test-plan §6 Scenario E1: the unwrapped-name hint and code are built; the owed-to clause naming this chunk retired (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Retired** names the "not built" line of the unwrapped-name hint; Counts `causes-new` lists `test-plan.md:553` as changed, and `title` says each "owed to" this entry changes; **Symbols / APIs** gives `without_endpoint` reading anything but a directory as `UnwrappedName`, and Outcome holds the hints at `src/human/hints.rs:50-55`.'
    basis: .andromeda/test-plan.md:553
  - detector: D-tests-framework
    severity: warning
    section: §2 Test Strategy, Test directory + naming conventions, File naming
    change: 'The file-naming line stops placing `cross_` in `viola-e2e` alone: the root prefix list gains `cross_` (`tests/cross_exit_causes.rs`, a root binary), which the E2E layer regex selects as it selects the root `path<N>_` binaries.'
    sidecar: 'test-plan §2 File naming: `cross_` is a root prefix too, `tests/cross_exit_causes.rs` the root binary the E2E regex selects (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report **Harness / gate surface** says the new table is a root `cross_` binary, which the E2E regex takes as it took `path1_`; Expected amendments names test-plan §2, the naming line, a root `cross_` binary. The body still lists `cross_` under `viola-e2e` only.'
  - detector: D-tests-count-literals
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project)
    change: 'The `ci` kill line clause reads the twenty-five verify-driven binaries in place of the twenty-four, and its list of the `binary(…)` names on that filter line gains `cross_exit_causes` between `contract_ledger_probes` and `cross_list_rows`, with `cross_exit_causes` recorded as joined at chunk 2026-10-31-exit-cause-code-catalogue.'
    sidecar: 'test-plan §3 Bootstrap phases: the `ci` kill line holds twenty-five verify-driven binaries, `cross_exit_causes` joined (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'The report Counts moved the `ci` kill line 24 → 25 `binary(` operands at `.config/nextest.toml:24`, and **Harness / gate surface** says it gains `binary(cross_exit_causes)`. This key file states the set by word and by name, "the twenty-four verify-driven binaries (the `binary(…)` names on that filter line: …), as `.config/nextest.toml` reads", so both the count and the list go stale. The digit-shaped sweep could not read the spelled number.'
    basis: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
note: 'The report row `kill-count` (0 changed, "no master states the count") and its Expected amendments line for test-plan are contradicted by the fourth proposal, which rests on a spelled-out count at bootstrap-phases-derive-for-route-setup-project.md:10, so the orchestrator should correct or supersede that disposition. Body lines left alone as still true, the error word staying `instance-unreachable`: §1 chaos trigger (report no-change), §1 and §6 Path 3 killed-wrapper signals, §5 On-disk and MCP, §6 Cross-surface parity and Security control negatives; their `doc-null` and `revive-gap` sites are in detail files and the cascade reads them. No drift for D-tests-coverage (the report Coverage of new surfaces names a test tier per surface) or D-tests-obs-harness (no harness verb, status shape or CI step moved; the log-format key file does not enumerate the `process-exit` `detail` values).'
