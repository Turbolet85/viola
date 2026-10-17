proposals:
  - detector: D-tests-framework
    severity: warning
    section: §3 → 5-command implementation
    change: 'run step 1 integration layer reads `kind(test) & !binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)` (E2E-prefixed binaries exist since chunk 2026-10-17-end-to-end-path-binary-and-harness-legs; the "`kind(test)` until the first E2E-prefixed binary exists" clause retires); the integration and E2E layers are complements by construction; the rule that a `binary()` regex matching no binary is a filterset parse error still holds, re-measured on cargo-nextest 0.9.146 on 2026-10-17 (scratch crate, exit 94 "operator didn''t match any binary names"), the 0.9.133 qualifier moving to it.'
    sidecar: '5-command run step 1: integration layer re-cut to kind(test) minus the widened E2E regex; the until-first-E2E-binary clause retires; parse-error rule re-measured on nextest 0.9.146.'
    rationale: 'Report Symbols `harness::run::nextest`: INTEGRATION was `kind(test)`, now excludes the E2E regex; Counts: the re-cut integration layer selects no E2E-prefixed binary (`binary(path1_run_start)` → 0 tests, `nextest-exit-4`); Spec claims disproved: the parse-error rule still holds on 0.9.146, its version qualifier moves (pattern `e2e-regex`, key :21).'
    basis: .andromeda/registries/contracts/test-plan/5-command-implementation.md:21
  - detector: D-tests-framework
    severity: warning
    section: §3 → 5-command implementation
    change: 'run step 2 E2E filterset is `binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)` (widened by `[0-9]*` after `path` so the numbered Path binaries `path1_run_start` / `path6_budget_harness` match; the Viola overseer, technical, founder-delegated, `inputs#I2`, PROVISIONAL); the step runs third, after unit and integration, as suite `nextest-e2e` in the default and `--all`, through the same nextest shape and target dir, a `--filter` intersecting it; `run --e2e` alone runs no doctest (doctest stays with unit / integration / coverage); the JUnit archive list is unchanged (unit, integration, coverage).'
    sidecar: '5-command run step 2: E2E regex widened to ^(path[0-9]*|tui|mcp|cross|chaos|contract)_ (inputs#I2, provisional); suite nextest-e2e runs third; run --e2e alone runs no doctest.'
    rationale: 'Report Spec claims disproved: the written regex `^(path|…)_` does not select `path1_run_start` (block run 1: `run --e2e` 18 binaries, `run --integration` ran path1); Deviations 1 and 7; Symbols `harness::run::Selection` / `test_suites`; Counts: E2E layer 18/155 → 20/157.'
    basis: .andromeda/registries/contracts/test-plan/5-command-implementation.md:22
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §3 → 5-command implementation
    change: 'run examples: `--e2e` runs the E2E layer as suite `nextest-e2e` (e.g. `scripts/agent-run.sh run --e2e --filter "binary(path1_run_start)"`); the "`--e2e` selects nothing yet: it is a usage error (exit 2) until the first E2E binary lands" clause retires.'
    sidecar: '5-command run examples: --e2e is built (suite nextest-e2e); the usage-error clause retires.'
    rationale: 'Report Harness / gate surface: `run --e2e` (suite `nextest-e2e`, in the default and `--all`); Outcome: `run --e2e` reports `nextest-e2e` passed 157; Expected amendments: the usage-error clause retires (search `e2e-selector`, key :53).'
    basis: .andromeda/registries/contracts/test-plan/5-command-implementation.md:53
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §3 Test Harness Contract
    change: 'Exit codes bullet: the "a flag or selector whose surface is not built yet … is a usage error until its chunk lands" rule stays, but `run --e2e` leaves its example (built at chunk 2026-10-17-end-to-end-path-binary-and-harness-legs).'
    sidecar: '§3 Exit codes: run --e2e drops out of the not-built-yet usage-error example.'
    rationale: 'Report Harness / gate surface and Symbols (`viola-harness` CLI: `run --e2e`, case `run_e2e_is_a_flag_of_its_own`): the flag is built, so naming it as a not-yet-built usage error restates the retired claim.'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §2 Test Strategy
    change: 'test locations: the E2E-prefixed binaries include the numbered Path binaries `path<N>_<topic>.rs` in both homes: root `tests/path1_run_start.rs` (Path 1, sync, `[[test]]` required-features `fake-agent`) and `crates/viola-e2e/tests/path6_budget_harness.rs` (Path 6), the viola-e2e form reading `tests/{path<N>,mcp,cross}_<topic>.rs`; both report in `nextest-e2e` by the widened regex.'
    sidecar: '§2 test locations: numbered path<N>_ binaries named in the root sync and viola-e2e homes; both in the E2E layer.'
    rationale: 'Report Spec claims disproved lists `test-plan.md:398` `tests/{path,mcp,cross}_<topic>.rs` under pattern `e2e-regex`; Expected amendments: the E2E layer includes root E2E-prefixed binaries; Symbols: new test binaries `tests/path1_run_start.rs` and `crates/viola-e2e/tests/path6_budget_harness.rs`.'
    basis: .andromeda/test-plan.md:398
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project)
    change: '`[profile.ci]` second override: the verify-driven binaries on that filter line are nineteen, `path1_run_start` joined at chunk 2026-10-17-end-to-end-path-binary-and-harness-legs (its one case starts from `stamped_home`); the count rule stays "the `binary(…)` names on that filter line".'
    sidecar: 'Bootstrap phases [profile.ci]: path1_run_start joins the verify-driven 20 s override, eighteen → nineteen binaries.'
    rationale: 'Report Schema / config: `.config/nextest.toml` `[profile.ci]` verify-driven 20 s override gains `binary(path1_run_start)`; New text, by line: `.config/nextest.toml` added 24.'
    basis: .config/nextest.toml:24
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: 'boot: `--statusline-echo` and `--agents-mode` are built (both "not built … owed to End-to-end path binary and harness legs" clauses retire). `--statusline-echo` (clap-conflicting with `--unstamped`, usage `arguments`) writes `<home>/statusline-source.json` after the stamp (`replace_private` 0600, `{"statusLine":{"type":"command","command":"<bin dir>/viola-fake-agent[.exe] statusline-echo <home>/fake/statusline.marker"}}`, forward slashes), making `<home>/fake/` through `create_private_dir`; a bin dir or home holding a character outside ASCII letters, digits and `_ - . / :` fails boot `build-failed`; no env var is set. `--agents-mode recorded|oversize|malformed` (an unknown value usage `arguments`): `start` removes a stale `<session>/bin/agents-mode` before the stamp and writes it (`replace_private` 0600) AFTER the stamp and the source plant, before `supervise.json`, because verify''s Run B reads `claude agents --json` through the same session copy (a mode written first failed `agents-join`, `verify-failed`); it is recorded as `agents_mode` (serde default) in `session.json` and `supervise.json`.'
    sidecar: '5-command boot: --statusline-echo and --agents-mode built; the agents-mode file is written after the stamp; --statusline-echo conflicts with --unstamped.'
    rationale: 'Report Symbols `harness::boot` and `viola-harness` CLI; Spec claims disproved: plan step 3 write order measured `verify-failed`, the master key :4 to state the order (pattern `boot-flags`); Expected amendments: the not-built clauses retire, the conflict with `--unstamped`.'
    basis: .andromeda/registries/contracts/test-plan/5-command-implementation.md:4
  - detector: D-tests-obs-harness
    severity: warning
    section: §6 E2E Test Strategy (Path 6 — budget governor)
    change: 'step 1: the "`agent-run boot --statusline-echo` is not built; it and the harness-session form of this step are owed to the Epoch 5 entry End-to-end path binary and harness legs" sentence retires: both landed at chunk 2026-10-17-end-to-end-path-binary-and-harness-legs as `crates/viola-e2e/tests/path6_budget_harness.rs` `path6_two_instance_release_over_a_harness_session` (steps 1 and 7 over `harness_session` with `--statusline-echo`).'
    sidecar: 'Path 6 step 1: boot --statusline-echo and the harness-session form landed (path6_budget_harness).'
    rationale: 'Report Symbols (new test binary `path6_budget_harness.rs`), Outcome (Path 6 step 7 over `harness_session` MET); Expected amendments: Path 6 harness-session form (search `owner-title` and `boot-flags`, test-plan :742).'
    basis: .andromeda/test-plan.md:742
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §6 E2E Test Strategy (Path 6 — budget governor)
    change: 'As landed: "the harness-session form and step 7''s two-instance leg" leave the not-asserted list, landed as `path6_two_instance_release_over_a_harness_session`: `.ok.budget.paused` stays true, `builder` `budget_paused:false`, `overseer` true; one `budget-gate{paused:false, override_until}` for `builder` alone, equal to its snapshot''s and later than its `ts` (compared as text); `send` exits 0 to `builder` and 11 `five-hour` to `overseer`; on Unix the marker is written and stdout is `viola-fake-statusline\n`, on Windows the `cfg(not(unix))` branch reads empty stdout and no marker (run on `windows-2025` in CI); the leg''s `release` runs with no `VIOLA_NAME`. The MCP form stays owed to "MCP server for drivers".'
    sidecar: 'Path 6 As landed: step 7 two-instance leg landed over harness_session; only the MCP form stays owed.'
    rationale: 'Report Outcome (tests) Path 6 step 7 MET and (security) the leg''s `release` with no `VIOLA_NAME`; Deviation 8 (instants compared as text); Expected amendments: Path 6 step 7 leg (search `owner-title`, test-plan :764).'
    basis: .andromeda/test-plan.md:764
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: 'run step 2 boot model: `harness_session` is `viola_e2e::harness::session::harness_session(test, label, opts) -> HarnessSession`, a plain function and guard, never an rstest fixture and never shelling out to `agent-run`: library `boot` with `build:false`, `stamp:true`, the bin dir from the test executable (`bin_dir_from_exe(current_exe)`), session id `<test>-<label>-<pid>`, panicking with the boot document unless ready; `SessionOptions` defaults to `overseer` + `builder`, no echo, `recorded`; accessors `home()` · `instance_dir(name)` · `bin_dir()` · `session()` · `workspace()` · `cleanup(self)`. The supervisor stays the spawned `supervise` process (fork 2, `inputs#I1`, PROVISIONAL until the founder''s Epoch 5 batch); the "fixture-owned thread instead of a spawned supervise process" clause retires. `Drop` runs `cleanup` (home kept) unless already run, then removes the home parent unless `keep_home` holds: `KEEP_HOMES=1`, or `KEEP_FAILED=1` and the test panicking.'
    sidecar: '5-command harness_session: a plain function + guard over library boot; the spawned supervise stays (fork 2, provisional); Drop keep rule stated.'
    rationale: 'Report Symbols new `harness::session` (plain function and guard, never an rstest fixture; the spawned `supervise` stays, fork 2); Deviation 4 (keep rule); Expected amendments: the `harness_session` sentence becomes the spawned `supervise` (search `harness-session`, key :26).'
    basis: .andromeda/registries/contracts/test-plan/5-command-implementation.md:26
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: 'run step 2: "`crates/viola-e2e` scenarios use one of two fixtures" reads "one of two boot models": the `harness_session` guard (a plain function, not an rstest fixture) or the TempDir-drop chain.'
    sidecar: '5-command run step 2: the viola-e2e boot models no longer call harness_session a fixture.'
    rationale: 'Report Symbols `harness::session`: "A plain function and guard, never an rstest fixture" — the lead-in that names both boot models fixtures restates the retired rstest-fixture claim.'
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: 'status: it runs `<bin dir>/viola --home <home> list --json`, the bin dir and the session `bin/` read from `supervise.json`, with `PATH` the session bin path (so the fake agent answers `claude agents --json`) and `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` removed (so it writes no role line), stdout read through `take(MAX_FRAME + 1)`; with no `supervise.json` it spawns nothing and reads `down` / `list-exit-none`.'
    sidecar: '5-command status: runs the session bin dir''s viola list --json with VIOLA_* removed and the session PATH.'
    rationale: 'Report Symbols `harness::status` and Harness / gate surface (`status`''s `viola list` carries no `VIOLA_NAME`, no role line); Deviation 5 (`viola --home <home> list --json` spelling); Expected amendments: `status` reads the bin dir from `supervise.json` and runs `viola list` with `VIOLA_*` removed (search `status-list`).'
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → Status endpoint shape
    change: 'the Interim paragraph (status runs no `viola list`) retires; the derivation reads: `ready` = `viola list` exit 0 and every booted name an item `wrapped:true, liveness:"live"` (unwrapped items ignored; no booted instance reads ready); `degraded` = a booted item `stale`, absent (a gone instance is omitted) or no `items`; `down` = a non-zero exit (`last_error` `list-exit-<code>`), no exit / no spawn / no `supervise.json` (`list-exit-none`), or exit 0 with no JSON object (`list-exit-0`, `list:null`); `list` is the parsed document, an exit-21 error document included; `pid` and `uptime_ms` stay `null`; `ok` only for `ready`; exit 0 ready, 1 otherwise, 2 unknown session.'
    sidecar: 'Status endpoint shape: interim retires; ready/degraded/down derived from the viola list read; an absent booted instance is degraded; list-exit-none added.'
    rationale: 'Report Symbols `harness::status` (`derive(code, list, booted)` and the document); Expected amendments: the interim paragraph retires, an absent booted instance reads `degraded`, `last_error` adds `list-exit-none` (`status-endpoint-shape.md:17` the interim). obs-plan §3 (heartbeat ticks: `degraded` when any item is `stale`; `state` / `last_error` closed enum) still agrees.'
    basis: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:17
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → Status endpoint shape
    change: 'the JSON example''s `last_error` reads `null | <harness code: list-exit-<code>, list-exit-none>` (e.g. `list-exit-21`).'
    sidecar: 'Status endpoint shape example: last_error names list-exit-none.'
    rationale: 'Report Schema / config: `last_error` gains `list-exit-none` (harness closed value); Expected amendments cite `status-endpoint-shape.md:11` the `last_error` example.'
    basis: .andromeda/registries/contracts/test-plan/status-endpoint-shape.md:11
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 → 5-command implementation
    change: 'Closed enums: `status.last_error`: `null`, `list-exit-<code>`, `list-exit-none`.'
    sidecar: '5-command Closed enums: status.last_error gains list-exit-none.'
    rationale: 'Report Expected amendments: test-plan §3 → Closed enums: `list-exit-none` (key `5-command-implementation.md:136` lists `list-exit-<code>`); Schema / config.'
    basis: .andromeda/registries/contracts/test-plan/5-command-implementation.md:136
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: §3 Test Harness Contract
    change: 'Session record: `session.json` carries `{v, session, home, instances:[{name, wrapper_pid, started_at, fake_args}], supervisor_pid, supervisor_started_at, agents_mode}`, `agents_mode` (serde default, written by `boot`) so an older record still parses; `supervise.json` gains the same `agents_mode` (serde default).'
    sidecar: '§3 Session record: session.json / supervise.json gain agents_mode (serde default).'
    rationale: 'Report Symbols: `harness::SessionRecord` gains `#[serde(default)] agents_mode` (written by `boot`; an older record parses), `SuperviseSpec` gains the same; Schema / config: `session.json` / `supervise.json` gain `agents_mode`.'
  - detector: D-tests-coverage
    severity: warning
    section: §6 E2E Test Strategy (Path 1 — `run` start sequence)
    change: 'add an As landed line: Path 1 steps 1–6 run as one ordered flow over one home in the root path binary `tests/path1_run_start.rs` `path1_run_start_one_home_in_order(stamped_home)` (`[[test]]` required-features `fake-agent`; in the E2E layer, `nextest-e2e`; on the ci profile''s verify-driven 20 s kill line), the `settings.json` half Unix''s, its helpers in `tests/support/start.rs` (fork 1, `inputs#I1`, PROVISIONAL until the founder''s Epoch 5 batch).'
    sidecar: 'Path 1: the E2E-tier owner tests/path1_run_start.rs recorded as landed (fork 1, provisional).'
    rationale: 'Report Symbols (new test binary `tests/path1_run_start.rs`, root support `tests/support/start.rs`), Outcome (Path 1 steps 1–6 as one ordered flow MET), Schema / config (20 s override); Expected amendments: test-plan §6 Path 1 as landed, a root path binary (fork 1, provisional).'
note: 'No obs-plan-side drift: obs-plan §3 / heartbeat-ticks keep "state + last_error closed enum" and "degraded when any item is stale", which the new derivation preserves, and the report changes no log format or event; D-tests-coverage holds otherwise (every new surface has unit/integ/e2e cases), the `oversize` agents mode covered by the status derivation unit only, not a booted case (report Outcome).'
