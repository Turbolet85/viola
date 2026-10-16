proposals:
  - detector: D-tests-coverage
    severity: warning
    section: '§7 Test Data & Fixtures → Seed strategies (the `claude agents --json` output row)'
    change: 'The row reads: `viola-fake-agent` placed as `claude` in a test-scoped bin dir (`tests/support/fake.rs` `claude_bin(scratch, name, mode, rows)` / `claude_in`; `empty_bin` for a `PATH` holding no `claude`) answers `agents --json` from files beside its exe — `agents-mode` (`recorded` · `oversize` · `malformed`, absent = `recorded`), `agents-rows.json` (planted rows) and the live `sessions/<pid>.json` registry its interactive runs write — with no env var; every test-started `list` / `revive` runs under a test-scoped `PATH` (`support::cli::spawn_on` / `viola_on` with a planted bin dir; a plain `spawn` of `list` with `PATH` = an empty bin dir); lifecycle per-test (the files in the test''s own bin dir), not `Command::env`.'
    sidecar: '§7 Seed strategies: the `claude agents --json` row trades `FAKE_CLAUDE_AGENTS_MODE` for the fake agent''s files beside its exe (founder fork 2, inputs#I1) and the test-scoped `PATH` helpers (chunk 2026-10-16-unwrapped-sessions-on-the-board).'
    rationale: 'Report Spec claims disproved: the `FAKE_CLAUDE_AGENTS_MODE=recorded|oversize|malformed` env row (test 926) is superseded by the founder''s fork 2, no env seam exists; Symbols / APIs fake agent and test support (files beside the exe, `claude_bin`, `spawn_on`, the empty-bin `PATH`); Outcome (tests) every test-started `list` / `revive` under a test-scoped `PATH`.'
    basis: '.andromeda/test-plan.md:926; viola-0.1.0/chunks/2026-10-16-unwrapped-sessions-on-the-board/report.md:210-212'
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → 5-command implementation'
    change: 'The `boot` body''s `--agents-mode` sentence reads: the flag (still not built, owed to "End-to-end path binary and harness legs") writes the fake agent''s `agents-mode` file beside the session''s `bin/claude[.exe]` copy, which the fake reads beside its exe; it sets no `FAKE_CLAUDE_AGENTS_MODE` and passes no env via `Command::env`; its record in `session.json` and `supervise.json` stays.'
    sidecar: '§3 5-command implementation: `boot --agents-mode` writes the fake''s `agents-mode` file, no env var (fork 2; the P5 card row, inputs#I2).'
    rationale: 'The report''s P5 card row (inputs#I2) places `boot --agents-mode` on "End-to-end path binary and harness legs" and states "Under fork 2 the flag writes the fake''s `agents-mode` file rather than an env var"; the env seam the row names is the one the report''s Spec claims disproved retires.'
    basis: '.andromeda/registries/contracts/test-plan/5-command-implementation.md:4; viola-0.1.0/chunks/2026-10-16-unwrapped-sessions-on-the-board/report.md:186-188'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§7 Test Data & Fixtures → the fake agent''s Modes bullet'
    change: 'Replace "Beside the options the agent takes one mode word, `statusline-echo` … `agents --json` lands with its consumer chunk; no shape is invented before a recorded fixture" with: two mode words — `statusline-echo` (as landed) and `agents` (chunk 2026-10-16-unwrapped-sessions-on-the-board): a first word `agents` answers `agents --json` from files beside its exe (`agents-mode`, `agents-rows.json`, the live `sessions/<pid>.json` registry; a row written just before its launch SessionStart fires, its `cwd` the launch fixture''s recorded `cwd`, `~` without one; dead or reused pids skipped by one `viola_state::liveness::start_times()` read), built on the recorded `Agents.recorded-1.json`; no argv option added, no env; units `agents_output_*`, `live_sessions_skip_a_dead_or_reused_pid`, `launch_session_replays_the_recorded_id_and_cwd`, `planted_rows_of_a_malformed_rows_file_are_none`.'
    sidecar: '§7 fake agent: the `agents` mode word lands beside `statusline-echo`, answering `agents --json` from files beside its exe over the recorded `Agents.recorded-1.json`.'
    rationale: 'Report Symbols / APIs (fake agent `agents`, deviations 12 and 13); Counts `fake-mode` 1 → 2 (test 953 "one mode word" stale) and `agents-read` (test 953 @c4133 stale).'
    basis: '.andromeda/test-plan.md:953; src/bin/viola-fake-agent.rs:806-866'
  - detector: D-tests-coverage
    severity: warning
    section: '§7 Test Data & Fixtures → Seed strategies (the Recorded hook payloads row)'
    change: 'Add Run B''s one `claude agents --json` read recorded by `viola verify --record` as `Agents.recorded-1.json` (Run B''s own row only, through the scrub, validated against `schemas/claude-agents-fixture.v1.json`; one today, under `2.1.287` only), and the set count reads "the 2.1.287 set holds 33 files" (`ls fixtures/claude/2.1.287 | wc -l`).'
    sidecar: '§7 Recorded hook payloads: `Agents.recorded-1.json` joins; the 2.1.287 set 32 → 33 files.'
    rationale: 'Report Counts fixture set 32 → 33 (`ls fixtures/claude/2.1.287 | wc -l`; test 927 stale); Symbols `viola verify` (`--record` writes `Agents.recorded-1.json` holding Run B''s own row only, through the scrub); Files new.'
    basis: '.andromeda/test-plan.md:927'
  - detector: D-tests-coverage
    severity: warning
    section: '§2 Test Strategy → Test pyramid (the Contract row)'
    change: 'Add: `contract_fake_agent_drift` also compares the key and type shape of the fake agent''s `agents` rows with each recorded `Agents.*.json` set (`fake_agent_agents_replay_matches_every_recorded_agents_set`), and jsonschema checks every `fixtures/claude/*/Agents.*.json` against `schemas/claude-agents-fixture.v1.json`, so the "every other `fixtures/claude/*/*.json` hook payload to `schemas/claude-fixture.v1.json`" clause excludes the agents class too.'
    sidecar: '§2 Contract row: the agents fixture class gets its drift replay and its own schema.'
    rationale: 'Report Files (new `schemas/claude-agents-fixture.v1.json`, `fixtures/claude/2.1.287/Agents.recorded-1.json`), Schema / config, Coverage (`agents-join` row + `--record` agents variant: `contract_fixture_hygiene`), New text `tests/contract_fake_agent_drift.rs` 466-530; without the exclusion the row routes an Agents fixture to the hook-payload schema.'
    basis: 'tests/contract_fake_agent_drift.rs:466-530; tests/contract_fixture_hygiene.rs:144-159'
  - detector: D-tests-coverage
    severity: warning
    section: '§7 Test Data & Fixtures → the fixture hygiene walk (the "Today the walk" bullet)'
    change: 'Add: `2.1.287`''s one agents fixture, `Agents.recorded-1.json`, against `schemas/claude-agents-fixture.v1.json` (one array of rows over the measured keys) with the same path, username and email walk (`agents_fixtures_pass_hygiene`; planted `planted_clean_agents_read_passes`, `planted_agents_schema_violation_is_rejected`, `planted_agents_absolute_path_is_rejected`, `planted_agents_username_is_rejected`).'
    sidecar: '§7 hygiene walk: the agents fixture class walked against its own schema.'
    rationale: 'Report New text `tests/contract_fixture_hygiene.rs` (the agents schema, `is_agents`, the walk and the four planted functions); Coverage (`contract_fixture_hygiene`); the same schema-routing claim as the §2 Contract row.'
    basis: 'tests/contract_fixture_hygiene.rs:144-229'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy → viola-agent-claude (the statusline-arms bullet)'
    change: 'The relied-set parenthetical reads `LedgerRow::relied()`: 23 rows on Unix, 20 on Windows (`LedgerRow::ALL` 23, `agents-join` appended last and relied on every OS); the version gate''s older-stamp oracle clause is re-read from `src/run/version_gate.rs` (changed this chunk) rather than carrying twenty-one / eighteen.'
    sidecar: '§4 ledger relied set 22 / 19 → 23 / 20 (`LedgerRow::relied()`), `agents-join` last.'
    rationale: 'Report Counts `LedgerRow::ALL` 22 → 23, relied 22 / 19 → 23 / 20 (test 505 stale); Symbols `LedgerRow::AgentsJoin` appended last, relied on every OS; Outcome (arch) `ALL` = 23, `[NN/23]` / `[NN/20]`.'
    basis: '.andromeda/test-plan.md:505'
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy → CLI (the `tests/cli_verify.rs` bullet)'
    change: 'The step-line clause reads `LedgerRow::relied()`: 23 on Unix, the last `[23/23] agents-join …`; 20 on Windows, the last `[20/20] agents-join …`; `STEPS_PASS` and `ROW_IDS` `[&str; 23]` on Unix and `[&str; 20]` on Windows; Run B''s description gains its one `claude agents --json` read once its first turn has settled, before the framing pastes; each `N pass  M fail` literal in the bullet is re-read from `tests/cli_verify.rs` (the report states only `dialogless_home`''s `19 pass  4 fail`, `16` on Windows).'
    sidecar: '§5 cli_verify: step lines `[NN/23]` / `[NN/20]` ending `agents-join`; Run B''s agents read.'
    rationale: 'Report Counts relied rows (test 586 @c549 · @c609 stale); Symbols `viola verify` (Run B''s one read before the framing pastes, deviation 7); Harness / gate surface (`dialogless_home` `19 pass  4 fail`, `16` on Windows).'
    basis: '.andromeda/test-plan.md:586'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy → CLI (the `contract_ledger_probes.rs` bullet)'
    change: '`ROW_IDS: [&str; 23]` (the three statusline ids relied on Unix only, `resume-session` and `agents-join` relied on every OS), `<n>` the relied count: 23 on Unix, 20 on Windows.'
    sidecar: '§5 contract_ledger_probes: `ROW_IDS` 22 → 23, relied 23 / 20.'
    rationale: 'Report Counts relied rows (test 596 stale); Files modified `tests/contract_ledger_probes.rs`.'
    basis: '.andromeda/test-plan.md:596'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → 5-command implementation'
    change: '`boot` step 4 reads: stamp every relied ledger row (`LedgerRow::relied()`: all twenty-three on Unix, elsewhere twenty, the first eighteen, `resume-session` and `agents-join`), Run B also making its one `claude agents --json` read.'
    sidecar: '§3 boot step 4: relied twenty-two / nineteen → twenty-three / twenty.'
    rationale: 'Report Counts relied rows (registry `test-plan/5-command-implementation.md:8` stale); Symbols `viola verify` Run B.'
    basis: '.andromeda/registries/contracts/test-plan/5-command-implementation.md:8'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → 5-command implementation'
    change: '`--local-live` reads `LEDGER_ROWS: [&str; 23]` under `cfg(unix)`, its list ending `resume-session` · `agents-join`, and the 20-id form (the first eighteen, `resume-session` and `agents-join`) under `cfg(not(unix))`, `LIVE_PASS` under the same split; the dated live-firing history stays as written.'
    sidecar: '§3 run --local-live: `LEDGER_ROWS` / `LIVE_PASS` follow the 23 / 20 relied rows.'
    rationale: 'Report Harness / gate surface (`crates/viola-e2e/src/harness/run.rs` `LEDGER_ROWS` / `LIVE_PASS` follow the 23 / 20 relied rows); Counts relied rows (registry `:43` stale).'
    basis: '.andromeda/registries/contracts/test-plan/5-command-implementation.md:43'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy → viola-agent-claude (a ledger-arm bullet after the statusline / resume-session bullet)'
    change: 'Add: the `agents-join` arm (chunk 2026-10-16-unwrapped-sessions-on-the-board) reads Run B''s one agents capture (`AgentsCapture`, carried in memory) and passes only when `claude agents --json` names the live session by pid and session id: `check_agents_join_passes_on_the_recorded_shape`, `check_agents_join_fails_on_each_broken_post_condition`, `check_agents_join_fails_with_no_capture`, beside `agents_variant_keeps_only_run_b_s_own_row` (`--record`''s filter to Run B''s own row).'
    sidecar: '§4 viola-agent-claude: the `agents-join` ledger arm and its unit functions.'
    rationale: 'Report Symbols (`agents_joined`, `agents_variant`, `AgentsCapture`); Coverage (`agents-join` ledger row: unit `check_agents_join_*`, `agents_variant_*`); New text `ledger.rs` 2863-2970. §4 names every other ledger arm''s functions.'
    basis: 'crates/viola-agent-claude/src/ledger.rs:881-927'
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy → viola-agent-claude (the `claude agents --json` parser bullet)'
    change: 'As landed: `agents::parse` returns `AgentsList { rows, skipped, tolerated }` or the closed `AgentsFailure` (`Oversize` · `Malformed`); a failed, oversize or malformed read leaves the board as `board::read` gave it (STATUS `unknown`, no unwrapped row); the row''s `sessionId` is its NAME and never a `ViolaName`; cases `agents_parse_*`, `agents_status_maps_each_measured_word_and_nothing_else`, `agents_row_joins_by_child_pid_or_session_id`, `agents_failure_details_are_the_closed_words`.'
    sidecar: '§4 viola-agent-claude: the agents parser bullet names its landed cases and the closed failure.'
    rationale: 'Report Symbols (`AgentsList`, `AgentsFailure`, `parse`; NAME = the row''s `sessionId`; a failed read leaves the board as `board::read` gave it; deviation 1); New text `agents.rs` tests 161-324.'
    basis: '.andromeda/test-plan.md:501; crates/viola-agent-claude/src/agents.rs:88-110'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Property suite'
    change: 'The agents bullet reads: the `claude agents --json` parser over arbitrary JSON (landed at `cases: 512`, the `agents.rs` `proptest!`, with `crates/viola-agent-claude/proptest-regressions/agents.txt` committed; its oracle compares a row against the input re-parsed from its bytes, because a float does not round-trip through serde_json text).'
    sidecar: '§6 Property suite: the agents parser property landed at 512 with its regressions file.'
    rationale: 'Report Symbols (the 512-case property, `agents.rs` 361-387); Files new `proptest-regressions/agents.txt`; deviation 2; Outcome (tests) property at 512 with regressions committed.'
    basis: '.andromeda/test-plan.md:899'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Property suite (the cargo-fuzz targets paragraph)'
    change: 'Add `claude_agents_json` to the joined targets: `agents::parse` over arbitrary bytes, 6 synthetic seeds (`background-row`, `control-session-id`, `deep-array`, `empty-array`, `measured-row`, `non-array`), kept byte-exact by `.gitattributes` `fuzz/corpus/claude_agents_json/** binary`.'
    sidecar: '§6 fuzz: `claude_agents_json` joins (fuzz targets 5 → 6 by `ls fuzz/fuzz_targets | wc -l`).'
    rationale: 'Report Counts fuzz targets 5 → 6 (gains `claude_agents_json`); Files new (`fuzz/fuzz_targets/claude_agents_json.rs`, 6 seeds); Schema / config `.gitattributes:7`; Outcome (tests) `claude_agents_json` replays green. The paragraph''s list sits outside the sweep''s pattern wording.'
    basis: 'fuzz/fuzz_targets/claude_agents_json.rs:9-18; .gitattributes:7'
  - detector: D-tests-coverage
    severity: warning
    section: '§2 Test Strategy → Test pyramid (the Property-based row)'
    change: 'The "today" target list adds `claude_agents_json` (the `claude agents --json` parser) after `paste_text`.'
    sidecar: '§2 Property-based row: the fuzz target list gains `claude_agents_json`.'
    rationale: 'Same claim as the §6 fuzz paragraph: the report''s fuzz targets 5 → 6; the row''s "today" list names five.'
    basis: 'fuzz/fuzz_targets/claude_agents_json.rs:9-18'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§3 → 5-command implementation'
    change: 'The `--fuzz-replay` seed enumeration adds `claude_agents_json`: 6 synthetic seeds, byte-exact under `.gitattributes`.'
    sidecar: '§3 run --fuzz-replay: the per-target seed list gains `claude_agents_json`.'
    rationale: 'The report reads registry `:42` no-change for its generic `<target>` path, but its parenthetical seed list enumerates each target and omits the sixth (Files new, 6 seeds; Schema / config `.gitattributes:7`).'
    basis: '.andromeda/registries/contracts/test-plan/5-command-implementation.md:42'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy → CLI (the trycmd / `viola list` board bullet)'
    change: 'Replace "the `-- UNWRAPPED - READ-ONLY --` separator is not built" with: the unwrapped rack landed at chunk 2026-10-16-unwrapped-sessions-on-the-board — `tests/cli_list.rs` `list_prints_planted_rows_as_unwrapped_rows` (the separator then the rows, `n/a` in WHEEL / DIALOG / CLI), `list_status_of_a_joined_wrapper_is_idle_then_busy`, `list_of_an_answer_that_does_not_parse_equals_the_no_claude_board`, `list_under_a_name_logs_the_agents_read`, `list_without_a_name_logs_no_agents_read`, backed by the `src/human/board.rs` units `board_row_*` and the `viola-state` units `board_join_*`.'
    sidecar: '§5 CLI: the unwrapped rack and STATUS idle / busy are built; their `cli_list` and unit witnesses named.'
    rationale: 'Report Counts `unwrapped-rack` (test 585 stale) and `title`; Symbols `viola list`; New text `tests/cli_list.rs` 278-479, `src/human/board.rs` 515-610, `board.rs` 865-959; Outcome (arch) v1-37 MET.'
    basis: '.andromeda/test-plan.md:585'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Exit-cause matrix (the `revive` exit 1 causes)'
    change: 'The causes read: the strict-modes check first, then the four preflight readings in their order (holder, session, `session-live`, recorded directory); the `session-live` cases are `tests/cli_revive.rs` `revive_of_a_session_open_in_another_claude_is_session_live` (exit 1, the `unable:` / `hint:` pair, one `process-exit` detail `session-live`, no child), `revive_passes_the_session_live_reading` (`--fork` skips it) and `revive_whose_agents_read_fails_starts` (a failed read passes), backed by the `preflight_order_*` units.'
    sidecar: '§6 exit-cause matrix: revive''s preflight readings three → four, `session-live` and its `cli_revive` witnesses.'
    rationale: 'Report Symbols `viola revive` (four readings, was three; the pair; fail open; skipped under `--fork`); Coverage (`viola revive` `session-live`: unit `preflight_order_*`, integ `cli_revive`); New text `tests/cli_revive.rs` 459-534. The sweep''s `revive-pairs` count words hit 0 master lines, so this wording sits outside it.'
    basis: 'tests/cli_revive.rs:459-534; src/cmd/revive.rs:153-158'
  - detector: D-tests-coverage
    severity: warning
    section: '§4 Unit Test Strategy → viola-state (the snapshot read-or-replay bullet)'
    change: '"in its second preflight reading, the logged session, of three" reads "of four" (holder, session, `session-live`, recorded directory).'
    sidecar: '§4 viola-state: revive''s preflight readings counted four.'
    rationale: 'Same claim as the exit-cause matrix: the report''s `viola revive` preflight chain of four readings (was three).'
    basis: 'src/cmd/revive.rs:180'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → `list` row integrity'
    change: 'Add "As landed": `tests/cross_list_rows.rs` `cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode` (piped, `NO_COLOR` / `TERM=dumb`, the outer PTY); on Windows the terminal case reads each escaped name as text under a 200-column terminal (ConPTY re-renders escapes) and the no-ESC assertion is Unix-only; the binary sits on the nextest 20 s kill line.'
    sidecar: '§6 `list` row integrity landed in `cross_list_rows.rs`, its Windows terminal leg qualified.'
    rationale: 'Report Files new `tests/cross_list_rows.rs`; deviation 10 (the Windows terminal case); Schema / config `.config/nextest.toml:24` (`binary(cross_list_rows)`); Counts `row-integrity` (test 875, now true).'
    basis: '.andromeda/test-plan.md:875; tests/cross_list_rows.rs:75-142'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Exit-cause matrix (the exit 21 unwrapped-name cause)'
    change: 'Add: as landed the unwrapped-name cause is `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable`, not a `cross_exit_causes.rs` row, and it proves exit 21 only; the unwrapped-name hint is not built, owed to the existing entry "Exit-cause code catalogue" (`v1-18`).'
    sidecar: '§6 exit-cause matrix: the unwrapped-name exit 21 witnessed in `cli_list`, its hint owed to "Exit-cause code catalogue".'
    rationale: 'Report Outcome (arch, v1-37: exit 21 on an unwrapped name, `e1_send_naming_an_unwrapped_row_is_unreachable`); the P5 card row (inputs#I2): the unwrapped-name `instance-unreachable` hint not built, "This chunk proves the exit 21 only", owned by "Exit-cause code catalogue".'
    basis: '.andromeda/test-plan.md:853; viola-0.1.0/chunks/2026-10-16-unwrapped-sessions-on-the-board/report.md:189-191'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy → Scenario E1'
    change: 'The exit-21 signal adds "as landed: `tests/cli_list.rs` `e1_send_naming_an_unwrapped_row_is_unreachable` (exit 21; the hint owed to "Exit-cause code catalogue")".'
    sidecar: '§6 E1: the unwrapped-name send witness named.'
    rationale: 'Same claim as the exit-cause matrix cause (report Outcome v1-37; P5 card row).'
    basis: '.andromeda/test-plan.md:789'
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy → CLI (the `tests/cli_verify.rs` bullet)'
    change: '"An uninterrupted verify closes seven spawn pairs (`version-probe`, `verify-probe`, five `verify-pty-probe`" reads eight, adding Run B''s one `agents-probe` pair; the dispatch-error case''s name is re-read from `tests/cli_verify.rs`.'
    sidecar: '§5 cli_verify: verify''s spawn pairs 7 → 8 (the agents read).'
    rationale: 'Report Counts verify spawn pairs 7 → 8 (`tests/cli_verify.rs` literal; test 586 @c3773 stale); Symbols `run::agents::read_logged` (the `agents-probe` spawn pair), `viola verify` (spawn pairs per uninterrupted verify 7 → 8).'
    basis: '.andromeda/test-plan.md:586'
  - detector: D-tests-framework
    severity: warning
    section: '§2 Test Strategy → Test directory + naming conventions (the sync E2E suites / `tests/support` bullet)'
    change: '`cli.rs` also holds `spawn_on` / `viola_on` (a planted bin dir on `PATH`), and a plain `spawn` of `list` runs with `PATH` = an empty bin dir; `fake.rs` holds `empty_bin`, `claude_bin` and `claude_in`, the one place a test plants `claude`.'
    sidecar: '§2 tests/support: the test-scoped `PATH` helpers in `cli.rs` and `fake.rs`.'
    rationale: 'Report Symbols Test support (`tests/support/fake.rs` `empty_bin` · `claude_bin` · `claude_in`; `tests/support/cli.rs` `spawn_on` · `viola_on`; a plain `spawn` of `list` under an empty bin dir, deviation 9); scope record in-intent `tests/support/cli.rs`.'
    basis: 'tests/support/cli.rs:79-99; tests/support/fake.rs:26-55'
note: 'D-tests-obs-harness: no proposal — the report adds no harness verb, status shape, CI step or log-format change (Harness / gate surface); the diag-line `session-live` member touches no test-plan §3 Log format clause. Sites cited by section with no line (§2 Contract and Property-based rows, §2 tests/support bullet, §4 viola-state replay bullet, §6 fuzz paragraph and exit-cause revive causes, §7 hygiene walk) lie outside the report''s sweep listing, worded beyond its patterns; several `N pass M fail` and oracle counts are left to the orchestrator''s one read of the named test file.'
