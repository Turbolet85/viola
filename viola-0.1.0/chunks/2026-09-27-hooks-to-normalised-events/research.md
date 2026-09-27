# Codebase Research — 2026-09-27-hooks-to-normalised-events

## Scope
- **Depth:** deep · **Reads:** 27 · **Globs/Greps:** 24 · **Graph queries:** 1 (rust plane) · **Probes run:** 1 (the replace
  probe below, a scratch cargo project outside the repo)
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`: read in full, 5 Session Additions. Those that bear on this chunk: boot readiness
    "Still to join: `events.ndjson` lines 1–3 … (with "Hooks to normalised events")"; boot passes no `--fixtures` today (below);
    "a booted-session test must stop its session in a `Drop` guard"; `test(=tests::name)` filter form (2026-09-27); never pipe
    `boot` (2026-09-25).
  - `.claude/rules/testing.md`: read in full, 15 Session Additions. Applied: force the window open, red before / green after
    (2026-09-27); every function needs an observable effect (2026-09-24, ext. 2026-09-27); a crate's security property needs a
    test inside that crate (2026-09-27); `cfg(unix)`/`cfg(windows)` bodies are killable only on their own leg (2026-09-24);
    one test per process-global `OnceLock` (2026-09-24); a deadline strictly below the kill line (2026-09-24, ext. 2026-09-27);
    proptest bodies build strings outside the format macro (2026-09-24); inline `#[cfg(test)]` modules only (2026-09-25).
  - `.claude/rules/host-win32.md` (always loaded): a running `.exe` cannot be relinked — the probe used its own
    `CARGO_TARGET_DIR`; stop by exact `ExecutablePath`.
- **Platform issues consulted:** none — no runner-only bullet was folded (Setup 5a read green) and no CI reading enters this
  chunk outside the operator leg.

## Files inspected
- `src/main.rs` (full) — `main` sets the panic hook first (43), then `catch_unwind(|| cmd::dispatch(cmd::Cli::parse()))`. A
  dispatch `Err` → `report_internal_error` + `ExitCode::from(1)` (46–48); a caught panic → `internal_error_exit_line` + exit 1
  (51–54). **Every caught panic and every dispatch error exits 1 today, whatever the verb** (CARRY 1(2) verified). clap's
  `Cli::parse()` runs INSIDE the catch but a clap usage error exits 2 through clap's own `exit` (not a panic), so a malformed
  `viola hook` argv would exit 2 today. The panic hook (`viola_panic_hook`, 60–92) writes only when `PANIC_SINK` is set, i.e.
  after `viola_obs_init`.
- `src/cmd/mod.rs` (full) — `Cli { home: Option<PathBuf> (global), command }`, `enum Command { Run(RunArgs) }` (27–30), no
  hidden verb anywhere (`grep -n 'hide\|hidden' src/cmd/*.rs`: 0 hits). `Failure { error: anyhow::Error, sink: Option<DetailSink> }`
  (33–36); `dispatch` resolves `--home` else `<user home>/.viola` (39–45).
- `src/cmd/run.rs` (full, 484 lines) — `NoMethods` answers every method `-32601` (58–65), served at `start` (131). `start_state`
  writes the first snapshot, touches the heartbeat and appends `wheel` + `budget-gate` with `Source::Wrapper` (218–252).
  `spawn_child` writes the **second** snapshot (with `child_pid`) right after the spawn (273–275). `pin_and_plugin` renders
  `viola_agent_claude::plugin_files(&pinned.path_fwd)` and writes each through `replace_private_shared` (180–198). The unit test
  `no_methods_answers_method_not_found_for_every_method` (410–417) pins `hook.event` → `-32601`.
- `src/run/mod.rs` (full) — `child_launch` sets exactly `VIOLA_NAME`, `VIOLA_DIR` (the instance dir), `VIOLA_BIN` (pinned,
  forward slashes) and a `PATH` with the pinned dir first (101–106); the test pins `env_set.len() == 4` (178). The home is
  `VIOLA_DIR`'s grandparent (test at 172).
- `src/run/env.rs` (full) — `persistent_names` (config `claude_env_keep` / Windows registry); no hook bearing beyond the R8
  shape.
- `src/obs.rs` (1–360) — `role_file_name` already maps `Hook` + a name to `hook-<name>.ndjson` (48–62). `viola_obs_init(home,
  process, instance, level)` opens the role file then sets the panic sink, `ProcessCtx` and the global subscriber (123–135).
  `internal_error_exit_line` writes a line only for `ObsProcess::Run` (220–231). `report_internal_error` writes the `chain`
  detail line only when a `DetailSink` exists (242–248). `write_detail` opens `detail-<process>.ndjson` lazily, one `write_all`,
  every failure swallowed (308–319). `read_diagnostics_level` reads `config.json` once under `take(MAX_FRAME)` (170–180).
- `crates/viola-core/src/lib.rs` (full) — `EventKind` holds only `Wheel`, `BudgetGate` (22–35); `MAX_FRAME` 16 MiB (9).
- `crates/viola-core/src/obs.rs` (full) — `ObsEvent` already has `HookInvoked`, `HookDecision`, `ParseRejected` (12–32);
  `ObsProcess::Hook` exists (90–96); `obs_event!` attaches `event`/`process`/`instance` from `ProcessCtx` (151–168).
- `crates/viola-agent-claude/src/lib.rs` (full, 535 lines) — R8 `IDENTITY_FLOOR`/`plan_strip`, `PLUGIN_DIR_FLAG`, the embedded
  `PLUGIN_FILES` (3 templates via `include_str!`, 116–138) rendered by `render` with `@@VIOLA_VERSION@@` / `@@VIOLA_BIN@@`
  (141–150), `resolve_program`. No hook parsing, no serde. The unit test `plugin_files_are_the_three_layout_paths` (497–516)
  pins `files[1].1 == "{\n  \"hooks\": {}\n}\n"`; `plugin_files_render_substitutes_the_pinned_path` (519–525) already renders
  a hooks-shaped template.
- `crates/viola-agent-claude/Cargo.toml` (full) — dependencies: `thiserror` only; dev: `tempfile`. It is listed in
  `scripts/sync-crates.txt` (tokio-free check + sole-root `deny-sync.toml`).
- `crates/viola-state/src/{fs,snapshot,events,lib,liveness,heartbeat}.rs` (full) — `replace_private` = `NamedTempFile::new_in`
  → mode → `write_all` → `sync_all` → `persist` (83–91); `replace_private_shared` accepts a failed replace over byte-identical
  content (96–101). `write_snapshot` holds `snapshot.json.lock` then `replace_private` (56–67); `read_snapshot` is a plain
  `File::open` under `take(MAX_FRAME)` taking no lock (70–79). `append_event` locks `events.ndjson.lock`, one `write_all`
  (61–69); `Source` has only `Wrapper` (20–24). `StateError::Io` displays `"state file i/o failed"` (lib.rs 17). `same_process`
  = pid + sysinfo start time (liveness.rs 49–54). The snapshot is written only at start (twice); the heartbeat is a separate
  file (heartbeat.rs).
- `crates/viola-channel/src/{lib,client,server,server/win}.rs` — `Client::connect(endpoint, process)` builds `conn =
  "<process>-<pid>-<t0>-<n>"` (client.rs 33–42); **only `Client::request` exists — no id-less send** (51–80). Windows open is the
  pinned SQOS `CreateFileW` with a 2 s `ERROR_PIPE_BUSY` wait (145–191). **The client performs no server verification** (no
  `GetNamedPipeServerProcessId`, no `peer_creds`: `grep -n 'server_process_id\|peer_creds' crates/viola-channel/src`: 0 hits).
  The server already takes an id-less `hook.event`, dispatches it and never answers (server.rs 202–212); any other id-less frame
  is `parse-rejected{channel-frame, malformed}` (204–206); `conn` is stripped before dispatch (198–200). Tests pin both
  (server.rs 566–580).
- `src/bin/viola-fake-agent.rs` (1–352) — `REGISTERED_EVENTS` (25–35); `fire` reads `<plugin-dir>/hooks/hooks.json`, returns
  `no-hooks` / `no-fixture`, then runs each absolute command (200–225); `run_hook` receipts `command_absolute:false, ran:false`
  for a non-absolute command, else `ran:true, exit_code, stderr_len, stdout_hex` (227–259). **It never fires SessionStart on its
  own**: hooks fire only from scripted steps (312) or a prompt submit (268). `start_receipts` writes `start`/`cwd`/`env`/`fds`
  (317–350).
- `crates/viola-e2e/src/harness/boot.rs` (1–340) — `readiness` checks the role file, snapshot (`pid`/`started_at`/`child_pid`),
  endpoint and heartbeat (271–308); **no `events.ndjson` check yet** (module doc 3–4). `crates/viola-e2e/src/harness/supervise.rs`
  passes `fake_args` only (53–59): **no `--fixtures` reaches the fake agent in a harness session** (`grep -n fixtures
  supervise.rs`: 0 hits).
- `crates/viola-e2e/src/harness/gate.rs` (1–230) — `SUITES` holds `perf` (15–25); `perf()` breaches `artifact-missing` without a
  `perf-*.json`, and gates `results[0].max < SPINE_DEADLINE_S = 1.0` per file (36–37, 192–219), tested (575–597). **The gate
  needs no product change for perf.**
- `crates/viola-e2e/src/harness/run.rs` (1–330, 751 lines total) — `Selection` has no `perf` (35–45); arms live in submodules
  (`browser`, `coverage`, `doctest`, `fuzz`, `mutants`, `nextest`); `run_with` → `test_suites` / `tool_arms` (114–146). A
  `--perf` arm is a new `run/perf.rs` submodule (run.rs is already 751 lines).
- `.github/workflows/ci.yml` (full) — 8 jobs (`test` ×3, `mutants` ×2, `mutants-verdict`, `msrv`, `fuzz-replay`, `lint` ×3,
  `release` ×3, `supply-chain`); no perf job, no hyperfine. The `test` job's gate is `coverage,doctest,playwright` (189).
- `schemas/diag-line.v1.json` (full) — **already carries every hook field this chunk needs**: `hook-invoked`/`hook-decision` with
  `hook_event`, `stdin_bytes`, `invoked_at`, `decision_emitted`, `deadline_hit`, `detail` ∈ {`oversize-stdin`, `malformed-json`,
  `channel-unreachable`, `server-verify-failed`, `strict-modes-failed`, `unverified-cli`, `deadline`}, `duration_ms`;
  `parse-rejected.parser` has `hook-stdin` and `prompt-submitted`; `$defs.hook_event` is the kebab list `session-start` …
  `statusline`. `schemas/diag-detail.v1.json` has `drift_report` (`["string","object","array"]`). No schema edit is owed unless
  the plan adds a value.
- `tests/support/fake.rs` (60–112) — `write_plugin` (84–94) and `write_fixture` (96–105) build synthetic hooks and fixtures in a
  test's scratch dir. `tests/cli_instance_state.rs:180,344–355` pins the plugin rewrite (a sentinel `{"hooks":{"Stop":…}}` is
  overwritten on restart). `tests/cli_fake_agent.rs:91–117,308,348–381` drives the fake agent with test-written fixtures.
- `.andromeda/test-plan.md` §3 (215–228, 526, 559, 651–671), §6 (1028), §9 (1440–1453), §10 (1510–1534), §3 ci-tool-install
  (788–804) — read for the perf, fixture and pre-push facts below.
- `.andromeda/architecture.md` 62–69 ([Hook Transport], [Hook Contract]) — the tiers and fail-open contract.
- `viola-0.1.0/working-route.md:88–99` (Epoch 6) — the entries "Server verification before any frame" and "Home and
  code-bearing file integrity" (strict-modes, per-start plugin rewrite) are their own route entries.

## Graph impact (rust plane; trace `{run_dir}/tree-query-2026-09-27-hooks-to-normalised-events.json`)
Canonical query 1 over `replace_private`, `read_snapshot`, `write_snapshot`, `plugin_files`, `dispatch`,
`report_internal_error`, `internal_error_exit_line`, `viola_obs_init`, `role_file_name`, `append_event` — 46 rows (the trace's
`rows`, `db_state: fresh`), every name hit. Editor lines (`line + 1`):
- **`replace_private`** — production: `replace_private_shared` (`crates/viola-state/src/fs.rs:97`), `write_snapshot`
  (`crates/viola-state/src/snapshot.rs:66`); the rest are its own tests (fs.rs 132, 204–205, 221, 229, 266, 268). A retry inside
  it reaches every snapshot write and the plugin/pin writes through `replace_private_shared`.
- **`write_snapshot`** — `start_state` (`src/cmd/run.rs:238`) and `spawn_child` (`src/cmd/run.rs:275`), the second landing
  after the spawn, when the child's first hook runs.
- **`read_snapshot`** — `collision_check` (`src/cmd/run.rs:160`) + its tests. The hook becomes a second production reader.
- **`dispatch`** (the `Dispatch` trait method) — `dispatched` (`crates/viola-channel/src/server.rs:243`), `main`
  (`src/main.rs:44`, the `cmd::dispatch` fn), and the `NoMethods` test (`src/cmd/run.rs:413`).
- **`internal_error_exit_line`** — `main` (`src/main.rs:52`), `report_internal_error` (`src/obs.rs:243`), its test (607, 612).
- **`report_internal_error`** — `main` (`src/main.rs:47`) + two tests (627, 665).
- **`viola_obs_init`** — only `run` (`src/cmd/run.rs:69`); `hook` becomes its second caller.
- **`role_file_name`** — `open_role_file` (`src/obs.rs:75`) + its test.
- **`plugin_files`** — `pin_and_plugin` (`src/cmd/run.rs:192`) + the pinning test (`crates/viola-agent-claude/src/lib.rs:498`).
- **`append_event`** — `start_state` (`src/cmd/run.rs:249`) + 3 tests; the wrapper's `hook.event` handler becomes its second
  production caller.

## Measured facts
- **M1 — the replace window (CARRY 5), measured on this host 2026-09-27 with the pinned `tempfile =3.27.0`:** a scratch probe
  mirroring `replace_private` (temp in the target's dir → `write_all` → `sync_all` → `persist`) against a `snapshot.json` held
  open by a reader, one case per reader share mode. Output, verbatim:
  - `std File::open (share R|W|D): held -> Some(PermissionDenied) (raw Some(5)); target during = "old"; after release -> None; final = "newer"`
  - `share R|W (no delete): held -> Some(PermissionDenied) (raw Some(5)); … after release -> None; final = "newer"`
  - `share R only: held -> Some(PermissionDenied) (raw Some(5)); … after release -> None; final = "newer"`
  - `share R|D: held -> Some(PermissionDenied) (raw Some(5)); … after release -> None; final = "newer"`

  So the failure is `ERROR_ACCESS_DENIED` (raw 5), **not** a sharing violation (raw 32), under EVERY reader share mode,
  std's default (which includes `FILE_SHARE_DELETE`) included. The target keeps its old bytes, and the replace lands as soon as
  the reader lets go. This matches the CARRY's measured chain (`Access is denied. (os error 5)`). The witness is a real held
  `File::open` handle — no test seam needed — and today's code fails it at the first attempt.
- **M2 — why here:** the second snapshot write (`src/cmd/run.rs:275`) lands right after the spawn, which is exactly when the
  real CLI (and, after this chunk, the fake agent) fires SessionStart; the hook reads `snapshot.json` for the endpoint and pid.
  Hook processes make the reader hot-path.
- **M3 — `VIOLA_NAME` / `VIOLA_DIR` survive the R8 strip:** `plan_strip` touches only names starting `CLAUDE`
  (`crates/viola-agent-claude/src/lib.rs:94`); `child_launch` sets the pair (`src/run/mod.rs:101–106`).
- **M4 — the pre-push Linux leg runs no perf:** test-plan §3 `pre-push` `linux-tests` = `run --coverage`, `run --browser`,
  `gate --require coverage,doctest,playwright` (test-plan.md:662); no stage names perf or hyperfine.

- **M5 — the long-paste wrapper on the wire (operator, P5 review; re-read at P5 from the log):**
  - The 23:16 UserPromptSubmit prompt in `~/.viola/sessions/viola-builder/events.ndjson` starts `\n\n<p` and carries
    `<pasted_content id="2f85">` … `</pasted_content id="2f85">\n`: the CLI's wrapper, unescaped, with the id repeated on the
    slash close.
  - The operator's later typed review message in the same log carries `<\pasted_content id="2f85">` and
    `<\/pasted_content id="2f85">`: typed tag text, escaped.
  - The prototype pins the same split (`D:/dev/projects/additional/viola-lab/prototype/src/store.rs:241–249`).

  This confirms architecture.md:89 (escaping separates the wrapper from typed text), with one refinement: the typed close form
  is `<\/`. The leading `\n\n` stays outside the pair (attribution open, a :58 matching question).

## Patterns detected
- **Arm per submodule** (`crates/viola-e2e/src/harness/run/fuzz.rs`, `browser.rs`): a tool-missing refusal before the suite, the
  suite counted from the tool's own export, driven through the `run_with` runner seam so harness tests never nest the tool.
- **Pinned tool install in CI**: taiki-e `tool:` line (ci.yml:41) or `cargo install --locked <crate>@<v>` in the job that
  invokes it (ci.yml:310 cargo-fuzz, 372 cargo-modules, 417 zizmor); test-plan §9 names `cargo install --locked` for hyperfine.
- **Role init** (`src/cmd/run.rs:67–82`): `read_diagnostics_level` → `viola_obs_init` → `process-start` → config rejections.
- **Synthetic fixtures written by the test** (`tests/support/fake.rs:96–105`), never committed until `viola verify` records.
- **One-literal flag constants pinned by a test** (`crates/viola-channel/src/client.rs:111–121, 313–323`).

## Conventions to follow
- **Fixed-message errors**: every new error enum displays a fixed literal (`crates/viola-state/src/lib.rs:15–21`,
  `crates/viola-channel/src/lib.rs:29–47`).
- **Codes-only role lines through `obs_event!`** with `#[instrument(skip_all, name = …, fields(..))]`
  (`src/cmd/run.rs:102,158,179`).
- **Oracles as literals in tests** (`crates/viola-core/src/obs.rs:175–195`, `crates/viola-core/src/lib.rs:60–68`).
- **proptest config** `cases: 512`, `FileFailurePersistence::SourceParallel("proptest-regressions")`
  (`crates/viola-core/src/lib.rs:70–78`).
- **Fuzz target shape** (`fuzz/fuzz_targets/viola_name.rs`, `channel_frame.rs`) + a synthetic seeded `fuzz/corpus/<target>/`.

## New files to create
- `src/cmd/hook.rs` — the hidden `hook <event>` verb and its fail-open body.
- `crates/viola-agent-claude/src/hooks.rs` (or a sibling module) — the tolerant payload parse, the hook-to-kind map, the
  `prompt-submitted` normalisation and harness classification (Claude shapes stay in this crate).
- `crates/viola-agent-claude/proptest-regressions/` — committed seeds for the two properties.
- `fuzz/fuzz_targets/hook_stdin.rs` (+ `prompt_normalise` if a second target) and `fuzz/corpus/<target>/` synthetic seeds.
- `tests/hook_fail_open.rs`, `tests/hook_events.rs` (root integration, `hook_` prefix).
- `crates/viola-e2e/src/harness/run/perf.rs` — the `--perf` arm (if perf stays in this chunk).

## Files to modify
- `plugin/hooks/hooks.json` — the exec-form entries (template tokens `@@VIOLA_BIN@@`).
- `crates/viola-agent-claude/src/lib.rs` — the module declaration + the `plugin_files` pin test (`files[1].1`, 513).
- `crates/viola-agent-claude/Cargo.toml` — `serde`, `serde_json`, `serde_path_to_error` (new to the workspace: `Cargo.toml`
  `[workspace.dependencies]` + `deny.toml` licence check), dev `proptest`.
- `crates/viola-core/src/lib.rs` — `EventKind` gains the kinds this chunk writes (+ its `as_str` test).
- `crates/viola-state/src/events.rs` — `Source::Hook`.
- `crates/viola-state/src/fs.rs` — the bounded retry in `replace_private` + the held-open witness (in-crate, testing.md 2026-09-27).
- `crates/viola-channel/src/client.rs` — an id-less `notify` for `hook.event` (+ `lib.rs` export if needed).
- `src/cmd/mod.rs` — the `Hook` arm (hidden), its `Failure` sink with `ObsProcess::Hook`.
- `src/main.rs` — the pre-clap argv role classification; `hook` → exit 0 on a dispatch error and on a caught panic; the clap
  usage path for `hook` never exits 2.
- `src/obs.rs` — `internal_error_exit_line` / the catch site's role awareness for `hook`.
- `src/cmd/run.rs` — `NoMethods` → a dispatcher that handles `hook.event` (append with `Source::Hook`); its pinning test (410–417).
- `src/bin/viola-fake-agent.rs` — fire SessionStart at start (the real CLI's behaviour), receipted.
- `crates/viola-e2e/src/harness/{boot,supervise}.rs` — `--fixtures` to the fake agent; readiness line 3 (`session-start`).
- `crates/viola-e2e/src/harness/run.rs`, `crates/viola-e2e/src/bin/viola-harness.rs` — the `--perf` selector (if in scope).
- `.github/workflows/ci.yml` — perf job/step + `cargo install --locked hyperfine@1.20.0` (if in scope).
- `fuzz/Cargo.toml` (+ `fuzz/Cargo.lock`) — the new target and a path dep on `viola-agent-claude`.
- Companion sweep (`grep -rn 'NoMethods\|"hooks": {}\|hooks\\": {}' src crates tests`): `src/cmd/run.rs:58–65,131,410–417`
  (change), `crates/viola-agent-claude/src/lib.rs:513` (change), `tests/cli_instance_state.rs:180,344–355` (no change: it
  asserts the rewrite, not the body).

## Open questions
- Server verification and home strict-modes before the first `hook.event` frame: route entries "Server verification before any
  frame" and "Home and code-bearing file integrity" (Epoch 6) own them, while security.md states the rule for `hook.event` now →
  blocks: plan-decision (P4 fork).
- Committed `fixtures/claude/2.1.0/*.json` for the harness `boot` readiness line 3, versus :51 (`viola verify`) owning recorded
  fixtures → blocks: plan-decision (P4 fork).
- The perf gate's CI shape (its own per-OS job per test-plan §9, or inside the `test` job per obs-plan §10) and whether the perf
  arm joins this chunk at all given its size → blocks: plan-decision (P4 fork, with the overseer's size check).
