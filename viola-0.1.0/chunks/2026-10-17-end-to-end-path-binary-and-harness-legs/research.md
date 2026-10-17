# Codebase Research — 2026-10-17-end-to-end-path-binary-and-harness-legs

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 22
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, Session Additions included (17 additions; the 2026-09-29 unbuilt-selector rule and the 2026-09-25 never-pipe-`boot` rule applied) · `.claude/rules/testing.md` — read in full (46 additions; 2026-09-28 "a test that pins a not-yet-built selector", 2026-10-07 "`VIOLA_*` removed in a builder session", 2026-10-16 "the gate filter selects a shared helper's callers" applied) · `.claude/rules/host-linux.md` (always loaded)
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a read one green run) and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the P4 fork answers (fork 1: Path 1 as a root `tests/path1_run_start.rs` flow; fork 2: `harness_session` keeps the spawned `supervise`), given by the overseer as the founder's delegate, provisional until the founder's Epoch 5 batch (snapped at P4, after this file's first write)

## Files inspected
- `crates/viola-e2e/Cargo.toml` (full) — no `[dev-dependencies]`, so no rstest in this package; deps include `viola-channel`, `viola-state`, `viola-pty`, `tempfile`, `sysinfo`; own `[lints.clippy]` (print-ban exempt)
- `crates/viola-e2e/src/lib.rs` (full) — `pub mod harness;` only: no `fixtures` module exists
- `crates/viola-e2e/src/bin/viola-harness.rs` (full) — clap `Cmd::Boot { session, instances, cli_version, unstamped }` (no `--statusline-echo` / `--agents-mode`); `RunArgs` has no `e2e`; a clap error maps to `Outcome::usage(cmd, "arguments")` (`main`, lines 218-230); `boot_options` builds `BootOptions` by struct literal (line 142)
- `crates/viola-e2e/src/harness/boot.rs` (full) — `start` (lines 116-181): session bin copy → `ensure_e2e_home` → `tempdir_in` parent → `stamp` (step 4) → `supervise.json` write (`SuperviseSpec { v, session, home, bin_dir, session_bin, cli_version, instances }`) → `spawn_supervisor` → `wait_ready` per instance → `session.json`. The point "before boot step 5" is between the `stamp` call and `write_json(supervise.json)` (the home exists only after `stamp`)
- `crates/viola-e2e/src/harness/status.rs` (full, 31 lines) — the interim: `list: null`, `state` from wrapper pid liveness
- `crates/viola-e2e/src/harness/run.rs` (lines 1-160, 230-270, 520-560, 750-800) — `Selection` (derives `Default`), `from_flags` (no selector or `--all` → unit + integration), `test_suites` (coverage replaces nextest; unit then integration)
- `crates/viola-e2e/src/harness/run/nextest.rs` (full) — `UNIT`, `INTEGRATION = "kind(test)"` with the comment that the exclusion "arrives with the first E2E binary"; `nextest(ws, suite, layer, filter, runner)` is the one invocation shape
- `crates/viola-e2e/src/harness/run/coverage.rs` (lines 19-50, 110-190) — `cargo llvm-cov nextest` with no `-E` unless `--filter`: every binary, the E2E-prefixed included, runs under CI's `run --coverage`
- `crates/viola-e2e/src/harness/gate.rs` (lines 14-40) — `SUITES: [&str; 8]` already holds `nextest-e2e`
- `crates/viola-e2e/src/harness/cleanup.rs` (lines 355-372) — the CARRY 5 case: line 362 `fs::create_dir_all(ws.e2e_home()).expect("mkdir");`
- `crates/viola-e2e/src/harness/mod.rs` (index; lines 84-127) — `Workspace::ensure_e2e_home`, and its own cases `e2e_home_backing_gone_target_is_made_again_owner_only` (line 394) and four refusal cases
- `crates/viola-e2e/src/harness/supervise.rs` (lines 20-80) — `spawn_wrapper` builds the `viola run` argv; the child env is `PATH` = `session_path(session_bin)` only (no `env_remove`)
- `crates/viola-e2e/tests/harness_lifecycle.rs` (lines 1-185) — `options()` struct literal (line 16), `Booted` drop guard, `harness_session_boots_reports_logs_and_tears_down` asserts `st.doc["list"].is_null()` (line 111), `harness_session_with_a_dead_wrapper_reads_degraded` (lines 156-182: child killed, wrapper exits, `status` exit 1 `degraded`, `instances[0].alive == false`)
- `crates/viola-e2e/tests/cli.rs` (lines 55-235) — `booted()` struct literal (line 167); `status_logs_and_cleanup_drive_a_booted_session` (`state: ready`, exit 0); `unknown_selectors_and_unknown_commands_are_usage` uses flags "no route entry will ever build" (no `--e2e` pin)
- `tests/cli_instance_state.rs` (index; lines 273-335) — the root Path 1 cases (map under Patterns detected)
- `tests/cli_budget.rs` (lines 1-112, 253-432) — `feed`, `usage`, `release(home, flags, from)`, `board`, `path6_release_budget_lifts_the_pause`, `path6_list_shows_the_reading_and_the_pause`
- `tests/support/home.rs` (index; lines 265-314) — `plant_statusline_source` (plain `fs::write`, settings shape), `statusline_marker` = `<home>/fake/statusline.marker`, `statusline_echo_command` (shell-plain words assert), `STATUSLINE_ECHO_OUTPUT`
- `tests/support/statusline.rs` (index) — `hook_statusline` removes `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN` then sets its own
- `tests/support/cli.rs` (line 113) — `viola()` removes `VIOLA_NAME`
- `src/bin/viola-fake-agent.rs` (lines 40-60, 805-860) — `AGENTS_MODE = "agents-mode"` (`recorded` · `oversize` · `malformed`, absent = `recorded`), `AGENTS_ROWS`, `SESSIONS` registry beside the exe
- `crates/viola-channel/src/endpoint.rs` (lines 31, 55) — `pub fn endpoint_name(name: &ViolaName, home: &Path)`; `viola_channel::Server::bind` is already used by `viola-e2e` cleanup tests (`cleanup.rs:390`)
- `.config/nextest.toml` (full) — `ci` profile (retries 0), the verify-driven `binary(…)` 20 s kill line (19 names), `mutants` overrides with `package(viola-e2e)`
- `.github/workflows/ci.yml` (lines 30-82) — test job: `run --coverage`, then the ci-smoke lifecycle `boot --instance builder` → `status` → `logs` → `cleanup` ×2; `gate --require coverage,doctest`
- `.claude/rules/testing.md:45`, `.claude/docs/commands.md:28`, `.claude/docs/services/viola.md:53`, `.claude/rules/verification-harness.md` §status — the docs that state `--e2e` unbuilt / the status interim

## Graph impact (from the code-graph query)
One query, rust plane, trail `.andromeda/runs/2026-10-17T00-19-29-phase/tree-query-2026-10-17-end-to-end-path-binary-and-harness-legs.json` (`refs` on `BootOptions`, `status`, `ensure_e2e_home`, `from_flags`, `test_suites`, `SessionRecord`, `SuperviseSpec`, `callee_crate = 'viola-e2e'`; 66 rows, editor lines = `line + 1`):
- **BootOptions** — struct literals (every one must carry new fields): `crates/viola-e2e/src/bin/viola-harness.rs:142`, `crates/viola-e2e/src/harness/boot.rs:756`, `:783`, `:800`, `crates/viola-e2e/src/harness/run/perf.rs:166` (and its test sites `:227`, `:244`), `crates/viola-e2e/tests/cli.rs:167`, `crates/viola-e2e/tests/harness_lifecycle.rs:16` (+ `:239`, `:268`, `:290` uses) — a field added without `Default` touches all five files.
- **SuperviseSpec** — written at `boot.rs:140`, read through `load_supervise_spec` (`boot.rs:426`) by `supervise.rs:54`; it records `bin_dir` and `session_bin`, which `status` needs to run `viola list` on the harness `PATH` (`SessionRecord`, read by `status`, records neither).
- **status** — callers `viola-harness.rs:241`, `harness_lifecycle.rs:108`, `:175`, plus the CLI-level `tests/cli.rs:188`.
- **Selection** (by grep, `Selection \{` over `crates/viola-e2e`: 15 hits) — literals at `viola-harness.rs:167` (full literal, must gain the field), `run.rs:534`, `:755`, `:764`, `:784`, `:788`, `:796`, `:966`, `coverage.rs:119`, `:143`, `fuzz.rs:221`, `perf.rs:528`, `:546`, `mutants.rs:681`, `:824`; those using `..Default::default()` / `..named` need no change — implement reads each.
- **ensure_e2e_home** — callers `boot.rs:128`, `run.rs:377`, and its own tests in `mod.rs`; the CARRY 5 case does not call it.

## Patterns detected
- **Path 1 already held at the root tier, step by step** (`tests/cli_instance_state.rs`): step 1 + records 1-3 → `path1_start_writes_state_before_the_spawn` (line 109), `path1_session_start_is_record_three_through_the_absolute_hook` (line 525); step 2's `list --json` `{wrapped:true, liveness:"live"}` → `tests/cli_list.rs` (lines 94, 241, 342, outside a Path 1 flow); step 3 → `run_refuses_a_live_name` (183) and `run_refuses_a_stale_name` (201); step 4 → `run_refuses_a_squatted_endpoint` (312, `viola_channel::Server::bind` on the computed endpoint); step 5 → `run_refuses_a_tampered_pinned_copy` (274), `run_rewrites_the_plugin_folder_each_start` (336), `run_rewrites_the_settings_override_each_start` (376); step 6 (restore by deleting the pinned copy) → the tail of `run_refuses_a_tampered_pinned_copy` (lines 300-307). What no case holds: the six steps as ONE ordered flow over one home, and step 2's `list --json` read inside it.
- **The E2E regex already matches 18 root binaries** — measured on cargo-nextest 0.9.146 over `target/harness` (`cargo nextest list --workspace --features viola/fake-agent -E … --message-format json`, counted by `scratchpad/suites.py`): `binary(/^(path|tui|mcp|cross|chaos|contract)_/)` → 18 binaries / 155 tests (`chaos_*` ×5, `contract_*` ×7, `cross_list_rows`, `tui_*` ×5); `kind(test) & !binary(…)` → 27 / 336; `kind(test)` → 45 / 491; overlap 0, remainder 0 — an exact partition. Control: `binary(/^nomatch_zz_/)` exits 94 `no binary names matched this` / `failed to parse filterset` at 0.9.146, so the hazard stands but no longer bites.
- **Harness boot in tests is the library `boot` with the test exe's bin dir** (`harness_lifecycle.rs:16-31`, `tests/cli.rs:164-182`): `build: false`, `bin_dir_from_exe(current_exe)`, a `Booted` drop guard calling `cleanup`. This spawns the `supervise` process from that bin dir; no fixture-owned supervisor thread exists.
- **Root env hygiene** — every root helper that spawns a verb removes `VIOLA_NAME` (`tests/support/cli.rs:113`) and `hook_statusline` removes `VIOLA_NAME`/`VIOLA_DIR`/`VIOLA_BIN` (`tests/support/statusline.rs:56-58`); the harness removes none. This session's own shell carries 3 `VIOLA_*` variables (`env | grep -c '^VIOLA_'` → 3; a builder under the bridge), so an unscrubbed `viola release` from a harness or `viola-e2e` test would carry a `from` and exit 20.
- **A gone instance is absent from `list --json`** (architecture §Standard Contracts `list` payload, per the arch extract), so `harness_session_with_a_dead_wrapper_reads_degraded`'s wrapper, once exited, is no `stale` item: the shape's three rules (`ready` all booted live · `degraded` any stale · `down` list non-zero) leave a booted-but-absent instance undecided.

## Conventions to follow
- **One JSON document per harness command, closed usage details** (`viola-harness.rs:218-230`): a new clap flag's misuse surfaces as the existing `arguments` detail; no new `detail` value (test-plan §3 Closed enums).
- **Labelled case tables, no rstest, in `viola-e2e`** (`.claude/rules/testing.md` §Naming: "`viola-e2e` carries no `[dev-dependencies]`, so its table-driven cases are a labelled case table").
- **Owner-only writes through `viola_state::fs`** (`replace_private`, `create_private_dir`; security extract §Patterns): the planted source and the `agents-mode` file.
- **A booted-session test stops its session in a `Drop` guard** (`harness_lifecycle.rs:60-85`; verification-harness.md: a leaked supervisor locks its `.exe` on Windows).
- **Per-child env, never `set_var`** (`boot.rs:204`, `supervise.rs:73`).

## New files to create
- `crates/viola-e2e/src/harness/session.rs` — the `harness_session` form for `viola-e2e` scenarios: boot through the library `boot` from the test exe's bin dir with a session id `<test>-<label>-<pid>`, a drop guard that runs `cleanup`, and `session.cleanup()` returning the report
- `crates/viola-e2e/tests/path6_budget_harness.rs` — Path 6 in harness-session form, booted with `--statusline-echo`: the statusline feed, step 7's two-instance leg, the `cfg(unix)` marker reading
- `tests/path1_run_start.rs` — Path 1 steps 1–6 as one ordered flow over one stamped home on the root chain (fork 1, `inputs#I1`)
- `tests/support/start.rs` — the start-refusal helpers `tests/cli_instance_state.rs` keeps file-local today (`run_refused`, `refused_with`, `child_starts`, `endpoint_of`), moved so the flow file defines no copy of its own (testing.md §Framework, "a root test file defines no copy of its own")

## Files to modify
- `crates/viola-e2e/src/bin/viola-harness.rs` — `run --e2e`; `boot --statusline-echo` (conflicts with `--unstamped`) and `--agents-mode recorded|oversize|malformed`; the `BootOptions` / `Selection` literals; its tests
- `crates/viola-e2e/src/harness/run.rs` — `Selection.e2e`; `from_flags` (default / `--all` add e2e); `test_suites` runs the E2E step after unit and integration; its tests
- `crates/viola-e2e/src/harness/run/nextest.rs` — `INTEGRATION` re-cut to `kind(test) & !binary(…)`, an `E2E` layer constant, the stale comment
- `crates/viola-e2e/src/harness/boot.rs` — `BootOptions` fields; the source plant before step 5; the `agents-mode` file; both recorded in `SuperviseSpec` / `SessionRecord`; its tests
- `crates/viola-e2e/src/harness/mod.rs` — `SessionRecord` gains the agents mode (and `mod session`)
- `crates/viola-e2e/src/harness/status.rs` — runs `viola list --json --home <home>` on the harness `PATH`, `VIOLA_*` removed, nests it, derives `state` / `ok` / `last_error`
- `crates/viola-e2e/src/harness/cleanup.rs` — the CARRY 5 case through `Workspace::ensure_e2e_home`
- `crates/viola-e2e/src/harness/run/perf.rs` — the `BootOptions` and `Selection` literals
- `crates/viola-e2e/src/harness/run/coverage.rs` — `Selection` literals where full (implement reads)
- `crates/viola-e2e/src/harness/run/fuzz.rs` — `Selection` literal where full (implement reads)
- `crates/viola-e2e/src/harness/run/mutants.rs` — `Selection` literals where full (implement reads)
- `crates/viola-e2e/tests/harness_lifecycle.rs` — `options()` literal; the lifecycle's `list` assertion (now the nested document); the dead-wrapper case's reading under the new derivation
- `crates/viola-e2e/tests/cli.rs` — `booted()` literal; `status` over a real `list`
- `.config/nextest.toml` — the verify-driven 20 s kill line gains `binary(path1_run_start)` by name
- `Cargo.toml` — a `[[test]]` table for `path1_run_start` with `required-features = ["fake-agent"]`, as every root test target carries
- `tests/cli_instance_state.rs` — imports the moved helpers from `tests/support/start.rs`; its cases unchanged
- `tests/support/mod.rs` — `pub mod start;`

## Leaf docs (the wrap's cascade, not implement's)
The leaves that restate the retiring clauses are re-derived by the wrap's cascade from the amended test-plan §3, so
they stay off the two lists: `.claude/rules/testing.md:45` ("`--e2e` is a usage error"),
`.claude/rules/verification-harness.md` §status (the interim sentence), `.claude/docs/commands.md:28`,
`.claude/docs/services/viola.md:53`.

## Sweeps
- `run --integration` naming an E2E-prefixed binary over `.claude scripts .github crates src tests` (regex `run --integration[^\n]{0,80}(tui_|chaos_|contract_|cross_|path_|mcp_)`): 0 hits — no live gate or doc selects a re-cut binary through `--integration`.
- `"--e2e"` over `crates/viola-e2e/tests/cli.rs` and `crates/viola-e2e/src`: 0 hits — no test pins `--e2e` as usage (testing.md 2026-09-28 rule has nothing to retarget).
- `harness_session` over `crates/ tests/ src/`: 2 hits, both test NAMES in `harness_lifecycle.rs` (93, 157) — no fixture of that name exists.
- `VIOLA_` over `crates/viola-e2e/src/harness/*.rs`: 0 hits — the harness scrubs no `VIOLA_*` variable.
- `statusline-echo|agents-mode` over `crates/viola-e2e`: 0 hits — neither flag is built.

## Open questions
- none — both plan-decision forks were answered at P4 (`inputs#I1`): fork 1, Path 1 as a root `tests/path1_run_start.rs` flow; fork 2, `harness_session` keeps the spawned `supervise` process.
