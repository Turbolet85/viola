proposals:
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary (Entity "CLI verbs and the --json / exit-code contract")
    change: 'Retire "`revive` takes no `--json`": `revive` takes `--json` on both arms; an exit-1 preflight refusal prints `{"v":1,"error":"<code>","detail":null}` on stdout with `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, and `--list --json` prints `{"v":1,"ok":{"sessions":[{"ts","cause","id"},…]}}` in log order (`no-session` document, exit 1, on an empty chain); its exits stay 0 / 1 / 2 and, without `--json`, the `unable:` / `hint:` pair and the `<ts>  <cause>  <id>` rows are byte-identical.'
    sidecar: 'chunk 2026-10-17-cli-machine-contract: §1 CLI verbs entity — revive takes --json on both arms (the five-code exit-1 document, the --list ok payload); "takes no --json" retired.'
    rationale: 'Report Changes, Symbols / APIs (`src/cmd/revive.rs`: `ReviveArgs` gains `--json`, `Refusal::write(name, json)`, `ListRow`) and Counts / qualifiers moved, pattern `revive-nojson` (test-plan.md:67 stale); Expected amendments names test-plan §1 Coverage scope / Surfaces under test (revive machine form).'
    basis: .andromeda/test-plan.md:67
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary (Surface "cli")
    change: 'Replace "`revive` has the human signal only … and no JSON document" with: `revive` has the human signal without `--json` (exit 0 / 1 / 2, one `unable:` line and a last-line `hint:` on stderr with empty stdout, for `--list` one `<ts>  <cause>  <id>` row per logged session) and, under `--json`, the exit-1 `{"v":1,"error":"<code>","detail":null}` document (the five preflight codes) or the `--list` `{"v":1,"ok":{"sessions":[…]}}` payload.'
    sidecar: 'chunk 2026-10-17-cli-machine-contract: §1 cli surface — revive no longer human-signal only; its --json documents named.'
    rationale: 'Same retired claim as the §1 CLI verbs entity (report Symbols / APIs, `src/cmd/revive.rs`; `revive-nojson`), restated in the cli Surface entry; a single-site apply would leave it.'
    basis: src/cmd/revive.rs
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns → On-disk (strict-modes refusals bullet)
    change: 'For `viola revive`, say the state-files `unable:` / `hint:` stderr pair is the form without `--json`; under `--json` both arms print the `strict-modes-failed` document on stdout instead (exit 1), and in either form a refused check writes no line, starts no child and leaves the home mode as found (`tests/cli_revive.rs` `revive_json_refuses_state_another_user_could_write_on_both_arms`).'
    sidecar: 'chunk 2026-10-17-cli-machine-contract: §5 strict-modes refusals — revive under --json prints the strict-modes-failed document; still no line on a refused check.'
    rationale: 'Report Symbols / APIs (`src/cmd/revive.rs`: "A refused check still opens nothing and writes no line, on both arms"; `Refusal::write` under `--json`) and Coverage of new surfaces (`revive_json_*`); Outcome (obs) names `revive_json_refuses_state_another_user_could_write_on_both_arms`. The bullet''s "stderr pair alone" is the retired no-`--json` claim restated.'
    basis: src/cmd/revive.rs
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy → Non-path suites → Exit-cause matrix (Exit 1 `revive` refuses to start)
    change: 'Replace "They assert no `--json` document, because `revive` takes none … and the `--json` document to "CLI machine contract"" with: the `--json` document landed (chunk 2026-10-17-cli-machine-contract) as `tests/cli_revive.rs` cases, one per code — `revive_json_on_a_live_name_is_the_already_live_document`, `revive_json_of_a_name_never_started_is_the_no_session_document`, `revive_json_with_an_id_the_log_does_not_hold_is_the_no_session_document`, `revive_json_of_a_session_open_in_another_claude_is_the_session_live_document`, `revive_json_of_a_snapshot_with_no_cwd_is_the_cwd_missing_document`, `revive_json_refuses_state_another_user_could_write_on_both_arms` — each exit 1 with the `{"v":1,"error":"<code>","detail":null}` document and the human pair unchanged without `--json`; joining the rstest table stays owed to "Exit-cause code catalogue"; and the refusals `run::start` can still make after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) print no document under `revive --json` and no case asserts one (the gap the report surfaces against `v1-39`; its owner the wrap''s P2/P7).'
    sidecar: 'chunk 2026-10-17-cli-machine-contract: §6 exit-cause matrix — revive --json documents landed in cli_revive.rs (five codes); owner clause to "CLI machine contract" retired; the post-preflight run::start refusals recorded as printing no document.'
    rationale: 'Report Counts / qualifiers moved: `revive-nojson` and `owner-title` both stale at test-plan.md:862; Expected amendments names §6 Exit-cause matrix (the revive document owner done); Spec claims disproved records that `run::start` refusals after a passed preflight print no `--json` document (the `v1-39` gap, owner P2/P7) — the matrix must not claim every revive exit-1 case carries a document.'
    basis: .andromeda/test-plan.md:862
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy → Non-path suites → Exit-cause matrix (`viola revive --list` sub-bullet)
    change: 'Replace "as the stderr pair and exit 1 with no log line" with: exit 1 with the stderr pair, or under `--json` the `strict-modes-failed` / `no-session` document (`revive_list_json_of_a_name_never_started_is_the_no_session_document`); a refused strict-modes check writes no line, while after a passed check under a parsed `VIOLA_NAME` `--list` logs one `process-start` and one `process-exit` (`exit_code` 1 `detail:"no-session"` on an empty chain, `exit_code` 0 otherwise) into the caller''s `cli-<VIOLA_NAME>.ndjson` (`revive_list_under_a_name_logs_its_start_and_exit`); a malformed `--id` is the clap usage exit 2.'
    sidecar: 'chunk 2026-10-17-cli-machine-contract: §6 exit-cause matrix — revive --list logs as cli under a VIOLA_NAME and prints its --json document; "no log line" kept only for the refused check.'
    rationale: 'Report Symbols / APIs (`src/cmd/revive.rs` `--list`: opens the caller''s `cli-<VIOLA_NAME>.ndjson`, logs `process-start` / `process-exit`; `src/main.rs` `role_of` makes it `Role::Cli`), pattern `list-nolog` (revive --list opens no process log → logs as cli), Outcome (obs) MET by `revive_list_under_a_name_logs_its_start_and_exit`.'
    basis: src/cmd/revive.rs
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns → CLI (`tests/cli_controls_not_disableable.rs` bullet)
    change: 'Replace "the other verb negatives still to join, the remaining rows and the completeness case are listed in the file''s header" with the landed state: the completeness case `every_flag_variable_and_config_key_is_a_row_of_the_table` (the flags read from the `Options:` section of `viola --help`, the `VIOLA_*` literals walked from the product sources with `viola-e2e` left out, the keys read from `schemas/config.v1.json`) and the rows `viola_dir_does_not_disable_the_home_scan` (`cfg(unix)`), `viola_variable_does_not_disable_the_send_paste_control` and `config_key_does_not_disable_the_identity_strip`; `setting_does_not_disable_the_budget_gate` now also tries `seven_day` and `v` (cases `seven_day_over_one_hundred`, `v_two`) — the bullet names these functions and cases, no row count.'
    sidecar: 'chunk 2026-10-17-cli-machine-contract: §5 cli_controls_not_disableable — completeness case and three new rows landed; budget-gate table tries seven_day and v.'
    rationale: 'Report Counts / qualifiers moved, pattern `completeness` (test-plan.md:593 stale), Deviations step 15 (the new rows, the `VIOLA_*` source walk, the `Options:` section, the two added budget-gate cases), Outcome (tests) completeness case MET.'
    basis: .andromeda/test-plan.md:593
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns → Wrapper channel (out-of-range `Percent` in `config.json` bullet)
    change: 'Retire "`setting_does_not_disable_the_budget_gate`, three rows": name its cases instead (the three `five_hour` cases plus `seven_day_over_one_hundred` and `v_two`, chunk 2026-10-17-cli-machine-contract), each asserting exit 11 on the `send` and one `parse-rejected{parser:"config-json"}` line, with no row count written.'
    sidecar: 'chunk 2026-10-17-cli-machine-contract: §5 out-of-range Percent — the budget-gate table''s count retired, its seven_day and v cases named.'
    rationale: 'Same retired claim (the budget-gate table holds three rows over `five_hour`) restated in the §5 Wrapper channel bullet; report Deviations step 15: the table "gained `seven_day_over_one_hundred` and `v_two`" (first three case labels and order kept).'
    basis: tests/cli_controls_not_disableable.rs
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns → CLI (completeness case sub-bullet)
    change: 'Name the sources: the completeness case fails if `viola --help` (its `Options:` section) lists a flag, `schemas/config.v1.json` a property, or the product sources a `VIOLA_*` literal that is missing from the table; the schema properties equal `CONFIG_KEYS` as a set (`src/obs.rs` unit `config_keys_are_the_config_schema_properties`).'
    sidecar: 'chunk 2026-10-17-cli-machine-contract: §5 completeness case — the config schema is schemas/config.v1.json, pinned to CONFIG_KEYS; the VIOLA_* leg added.'
    rationale: 'Report Counts / qualifiers moved: `config-schema` now-true at test-plan.md:595 and `completeness` now-true there ("the landed case also requires every `VIOLA_*` literal the product reads to be a row"); Schema / config: new `schemas/config.v1.json`, the five `CONFIG_KEYS` pinned by `config_keys_are_the_config_schema_properties`.'
    basis: .andromeda/test-plan.md:595
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns → CLI
    change: 'Add a bullet: home resolution and the machine contract — `tests/cli_machine_contract.rs` reads each driver-facing verb''s `--json` document and literal exit over one live wrapper''s life (`every_verb_prints_one_document_and_its_literal_exit_over_a_wrapper_s_life`, a labelled case table per world), over a home with no instance and over a home another user could write, and pins the resolution order `--home` → the `VIOLA_NAME` + `VIOLA_DIR` instance home → `<user home>/.viola` with canonicalisation of an existing home (`home_resolution_takes_the_flag_then_the_session_then_the_default`, `two_homes_are_isolated`, `a_home_reached_through_a_link_is_canonicalised`), backed by the `src/cmd/mod.rs` units `resolve_home_*`, `canonical_home_*` and `plain_drive_*` (the plain drive form on Windows); a test-started `viola` strips `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` (`tests/support/home.rs` `UNWRAPPED`) and compares paths against `resolved(home)`.'
    sidecar: 'chunk 2026-10-17-cli-machine-contract: §5 CLI — the per-verb machine-contract file and the home-resolution cases (unit + integration) named.'
    rationale: 'Report Coverage of new surfaces: home resolution through `VIOLA_DIR` + canonicalisation tested unit (`resolve_home_*`, `canonical_home_*`, `plain_drive_*`) + integ (`home_resolution_…`, `two_homes_are_isolated`, `a_home_reached_through_a_link_is_canonicalised`); Files: `tests/cli_machine_contract.rs` new; Harness / gate surface: `UNWRAPPED`, `resolved(home)`. A new CLI code path whose tests the plan does not name (pattern `machine-file`: no master names `cli_machine_contract`).'
    basis: tests/cli_machine_contract.rs
note: 'D-tests-framework and D-tests-obs-harness hold (the gates ran through the harness `run --unit` / `--integration` / `--e2e`, nextest + rstest + assert_cmd; Harness / gate surface "none", the §3 keyed contracts untouched; log-format.md''s `state-recovered` `corr` = null row unaffected by the added `scanned` field). Not proposed, the wrap''s: the Part A criterion "reads red with the fix reverted" UNMET for `stamped_home_concurrent_verify_runs_each_stamp_the_home` (green on this host either way; the witness is the unit `captured_reads_no_capture_while_the_claim_is_empty`), routed to P2 as an unlinked plan criterion with no test-plan site; and the `v1-39` revive --json gap''s owner (P2/P7), which the §6 proposal records without deciding.'
