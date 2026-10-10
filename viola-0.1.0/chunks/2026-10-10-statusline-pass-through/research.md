# Codebase Research — 2026-10-10-statusline-pass-through

## Scope
- **Depth:** deep · **Reads:** 31 · **Globs/Greps:** 27
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, 17 Session Additions; applied:
  the fake agent's argv-only modes and its absolute-command rule, `binary(<stem>)` for a whole root test file,
  never pipe `boot`, the stalled-start rule. `.claude/rules/testing.md` — read whole, 38 Session Additions;
  applied: a new stamped-home case is sized against the 10 s `mutants` kill, a remove-the-guard run per guard
  test, the fake agent as `claude` first on `PATH` for a verb that looks its program up by name, a kill only at
  a process the test started, a wait bounded and exit-aware.
- **Platform issues consulted:** `https://code.claude.com/docs/en/statusline`, fetched 2026-10-10. The fetched
  page states, and nothing here was measured on the installed build:
  - `statusLine` goes in user settings (`~/.claude/settings.json`) or project settings; its keys are `type`
    (`"command"`), `command`, and optional `padding`, `refreshInterval`, `hideVimModeIndicator`. It documents no
    `args` key. "The `command` field runs in a shell";
  - on Windows the command runs "through Git Bash when Git Bash is installed, or through PowerShell when Git
    Bash is absent", and paths in `command` take forward slashes;
  - `rate_limits.<window>.used_percentage` is a number from 0 to 100 (its example reads `23.5`) and
    `rate_limits.<window>.resets_at` is "Unix epoch seconds" (its example reads `1738425600`). `rate_limits`
    "appears only for claude.ai Pro and Max subscribers ... and only after the first API response in the
    session"; each window may be absent on its own; a window is dropped once its `resets_at` passes; a third
    window `spend_limit` exists behind a gateway;
  - the script runs once at session start (a resume included) and again on a new assistant message and other
    listed triggers, debounced at 300 ms; "If a new update triggers while your script is still running, Claude
    Code cancels the in-flight script"; Claude Code captures the output and sets `COLUMNS` and `LINES` for the
    script;
  - a folder that is not trusted, `disableAllHooks` and `allowManagedHooksOnly` each leave the status line off.
- **External inputs:** `inputs#I1` — the operator's take-up direction (no mutation, witness or dispatch; rows,
  verify children, re-stamps and live starts are the founder's and none is capped; the two boundary matters go
  on the card unsettled; the cut at the card; the `ci.py conclusion` read). `inputs#I2` — a founder ruling
  stands until his word; work that departs from one is held behind a stop rule. `inputs#I3` — the operator's
  answers to the P4 fork round: the per-home file is the planning form of the source, a widening being shown to
  the founder and held behind the stop rule; the Windows half is cut provisionally.
- **A crate fact found at P4:** `chrono` is a dependency of `viola-state` and the root bin, and not of
  `viola-core` or `viola-agent-claude` (`grep -n '^chrono' crates/*/Cargo.toml`: one hit, `viola-state`). The
  statusline reader turns epoch seconds into RFC 3339, so the agent crate takes the workspace's `chrono`.

## Files inspected
- `src/cmd/hook.rs` (1-205, 436-516) — `hook()` dispatch: `HookEvent::from_arg` first (`:91`, an unknown word
  exits 0 with nothing written), then `instance_of` (`:101`) from `VIOLA_NAME` / `VIOLA_DIR` (absolute, ending
  `<home>/instances/<name>`, not canonicalised), obs init, the seam, then `handle` / `handle_dialog`.
  `read_snapshot` is called with no strict-modes check (`:254`, `:469`). `decided` writes `hook-decision`.
- `src/cmd/run.rs` (136-325, 436-512, 1022-1075) — `Launch`, `start` (the one start path, called by `run` at
  `:160` and by `revive`), `pin_and_plugin` (`:344`, plugin files through `replace_private_shared`),
  `start_state` (`:437`, the first snapshot's literal at `:448`), `spawn_child`; the span-order test
  `start_opens_the_scenario_one_spans_under_run_start` (`:1028`) starts `whoami` in a temp home in-process.
- `src/run/mod.rs` (94-143) — `child_launch` builds the child's env (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`,
  `PATH`) and argv (`--plugin-dir <dir>` first, then the user's arguments).
- `src/cmd/mod.rs` (156-164) — `resolve_home`: `--home`, else `std::env::home_dir()` joined with `.viola`.
- `crates/viola-state/src/snapshot.rs` (1-135) — `InstanceSnapshot` and the `cwd` field's serde form
  (`#[serde(default, skip_serializing_if = "Option::is_none")]`, `:58-61`), the envelope, the classified read.
- `crates/viola-state/src/strict.rs` (1-76) — `check_instance(home, instance_dir)`: the home, `instances/`, the
  instance dir, `snapshot.json`, `events.ndjson`, each that exists; `check_path` has a Unix and a Windows reader.
- `crates/viola-state/src/fs.rs` (pub surface) — `replace_private` (`:263`), `replace_private_shared` (`:299`),
  `open_private_lock` (`:237`), `FILE_MODE`.
- `crates/viola-agent-claude/src/lib.rs` (pub surface, 160-192) — `plugin_files` renders three templates with
  the pinned forward-slash path; `PLUGIN_DIR_FLAG`, `RESUME_FLAG`; `AgentError` is the crate's one error.
- `crates/viola-agent-claude/src/ledger.rs` (14-203) — the seventeen rows; `DIALOG_SETTINGS` and `plan_settings`
  show `viola verify` already starts the real CLI with `--settings <one JSON object>` (Runs C and D).
- `crates/viola-agent-claude/src/hook.rs` (59-83, 312-328) — `HookEvent::from_arg`; the unit test
  `hook_event_args_are_the_diag_line_names` asserts `from_arg("statusline")` is `None`.
- `crates/viola-agent-claude/Cargo.toml` — dependencies: `viola-core`, serde, serde_json, serde_path_to_error,
  thiserror, vt100. No `viola-state`, no chrono.
- `src/bin/viola-fake-agent.rs` (28-160, 262-480) — argv options (ten), `hook_commands`, `run_hook` (an absolute
  command spawned directly, receipted with `stdin_hex` / `stdout_hex`), `payload`, `fire_answered`.
- `schemas/diag-line.v1.json` (40-133) — `hook_event` holds `statusline`; `subject` holds `statusline-shell`;
  `hook-invoked` / `hook-decision` admit `stdin_bytes`, `budget_written`, `deadline_hit` and the seven `detail`
  codes; `process-exit` admits `shell_exit_status`.
- `tests/hook_fail_open.rs` (150-200) — the fail-open matrix; its `unknown_event` case is `["hook",
  "statusline"]` (`:180`).
- `tests/cli_instance_state.rs` (300-380) — `run_rewrites_the_plugin_folder_each_start` (`:334`): boot, stop,
  plant a sentinel in `hooks.json`, boot again, read the file back.
- `tests/tui_passthrough.rs` (1-60) — the zero-viola-bytes cases and `VIOLA_LITERALS`.
- `tests/contract_diag_schema.rs` (170-180) — two cases already hold `hook_event: "statusline"`.
- `fuzz/fuzz_targets/hook_stdin.rs` (whole) — runs `normalise` for every `HookEvent`; ten seeds.
- `Cargo.toml` (root) — every root test file is a `[[test]]` entry with `required-features = ["fake-agent"]`.
- `.config/nextest.toml` — the verify-driven binaries are a named `binary(...)` list with their own kill lines.
- `.andromeda/registries/contracts/test-plan/5-command-implementation.md` (line 4) — `boot --statusline-echo`.
- `.andromeda/obs-plan.md` (691-705) — Scenario: Budget governor.
- `.andromeda/architecture.md` (454-458) — Config management.

## Graph impact (from the code-graph query; rust plane, trace `tree-query-2026-10-10-statusline-pass-through.json`)
- **`InstanceSnapshot`** — 39 references (query 1). Its struct literals are 11 sites, re-derived by `grep -rn -E
  '^\s+cwd: ' src crates tests --include=*.rs` read against the 39 rows: `src/cmd/run.rs:448` (the one product
  literal), `src/cmd/hook.rs:718`, `src/cmd/revive.rs:382`, `src/run/dialog.rs:444`, `src/run/wheel.rs:780`,
  `src/run/snapshot.rs:59`, `crates/viola-state/src/snapshot.rs:165`, `liveness.rs:79`, `replay.rs:175`,
  `strict.rs:724`, `crates/viola-state/tests/state_replay.rs:22`. A new field is added at each.
- **`child_launch`** — 3 callers: `start` @ `src/cmd/run.rs:283`, two unit tests @ `src/run/mod.rs:178`, `:212`.
  A new parameter threads through these three.
- **`start_state`** — 2 callers: `start` @ `src/cmd/run.rs:274`, the test helper `first_snapshot` @ `:962`.
- **`start`** (`src/cmd/run.rs`) — callers `run` @ `src/cmd/run.rs:160`, `revive` @ `src/cmd/revive.rs:103`, the
  span-order test @ `src/cmd/run.rs:1036` (the query's other rows are other functions named `start`).
- **`plugin_files`** — `pin_and_plugin` @ `src/cmd/run.rs:356` and two unit tests in `lib.rs`.
- **`HookEvent::from_arg`** — `hook` @ `src/cmd/hook.rs:91`, `parse` @ `src/run/dialog.rs:103`, one unit test.
- **`check_instance`** — product callers `preflight` @ `src/cmd/revive.rs:145`, `list` @ `:205`. No hook calls it.
- **`read_snapshot`** — product callers `live_endpoint` @ `src/cmd/client.rs:96`, `ask` @ `src/cmd/hook.rs:254`,
  `deliver` @ `:469`, `collision_check` @ `src/cmd/run.rs:324`.
- **crate edges** — `viola` → `viola-agent-claude`, `viola` → `viola-state`, each of the two → `viola-core`. No
  edge between `viola-agent-claude` and `viola-state` (query 3).
- **names** — no symbol named `statusline*`, `Statusline*`, `settings*` exists; `budget*` exists only as the
  snapshot's `budget_paused`, the replay's two fields and two `viola-core` enum values (query 3, second call).

## Findings
1. **Nothing statusline-shaped is built.** The search in `scope.md` stands (`grep -rn -i -l -E
   'statusline|status_line' src crates plugin fixtures scripts`: two files, both test data).
2. **The diagnostics schema is ahead of the code.** Every line and field obs-plan's scenario names for the hook
   side is already admitted (`diag-line.v1.json:53-58`, `:78-83`, `:129`, `:131`); no schema change is needed
   unless a new `detail` code is added. The seven `hook-decision` codes hold `oversize-stdin`, `malformed-json`,
   `strict-modes-failed` and `deadline`.
3. **Every test home would read the developer's own settings.** A start that reads `<user home>/.claude/
   settings.json` whenever no redirect is planted would do so in every existing test that boots a wrapper,
   and under `pre-push` (whose children get the passwd `HOME`). A test home does not exist before viola creates
   it (test-plan §7; the Windows DACL), so a redirect that must be written into the home first cannot be the
   default for tests. The form that keeps a test home hermetic with no per-test step is: a home that is not the
   default home never reads the user's settings.
4. **`resets_at` is documented as epoch seconds, a number.** Test-plan §6 Path 6 step 2 feeds an RFC 3339
   string, and security-plan §Input Validation says it is "parsed with chrono in UTC only". A parser that took
   only a string would record `"unknown"` for every reading the documentation describes.
5. **`rate_limits` is absent in most invocations** (before the first response; for a non-subscriber). A write on
   every invocation would replace a good reading with an all-unknown one. `hook-decision.budget_written` is
   already a boolean in the schema.
6. **The override's `command` is a shell string by documentation**, and no `args` key is documented, so the
   exec-form the plugin's hooks use (`hooks.json`, measured through the spine rows) is not known to work for
   `statusLine`. A pinned path holding a space or a shell-special character then needs quoting that differs by
   shell.
7. **The Windows shell cannot be found honestly now.** The documentation names Git Bash when installed, else
   PowerShell. How the CLI finds Git Bash is not documented on that page and not measured, a search of `PATH` or
   a read of an environment variable would break architecture's Config management sentence, and no interactive
   Windows host exists (the route's `BLOCKED-ON` on "Windows-only live measurements").
8. **`viola verify` already passes `--settings` to the real CLI** (`ledger.rs:192-198`; measured on 2.1.287 and
   2.1.288 for `permissions.ask` and `plansDirectory`). That a `statusLine` key in it replaces the user's
   status line is not measured.
9. **The harness has no `--statusline-echo`** and no `statusline-source-unresolved` refusal (`grep -rn -i
   statusline crates/viola-e2e scripts`: 0 hits). Its first user is Path 6's test, which needs the governor.
10. **`hook statusline` is not a hook event.** `HookEvent` stays at nine; the arm is dispatched beside it, and
    the unit test at `hook.rs:323` keeps reading `None`.
11. **No master states a bound on the wait for the user's command**, its environment, its working directory or
    where its stderr goes (the security extract's last binding). The documentation says the CLI itself cancels
    a script still running when the next update fires; how it cancels is not stated.
12. **The fail-open matrix's `unknown_event` case names `statusline`** and must name another unknown word once
    the arm exists (`tests/hook_fail_open.rs:180`).
13. **The span-order test would read the developer's settings** under any form but finding 3's: it calls
    `start` in-process over a temp home (`src/cmd/run.rs:1036`).

## Patterns detected
- **An additive optional snapshot field** (`crates/viola-state/src/snapshot.rs:58-61`): `cwd` is the template
  for `statusline_command`; its two unit tests (`:315`, `:335`) are the template for the new field's.
- **Shape in the agent crate, I/O outside it** (the version gate, architecture's amendment of 2026-09-28): the
  statusline and settings parsers are pure functions over bytes in `viola-agent-claude`; the reads and writes
  are the root bin's and `viola-state`'s. No `viola-agent-claude` → `viola-state` edge is added.
- **Rewrite each start, read back by a sentinel** (`tests/cli_instance_state.rs:334-350`).
- **A child spawn logs its pair at the call site** (obs-plan §6 Boundary-call wrappers): `process-start` /
  `process-exit` with a closed `subject`, no argv, no path.
- **Check, then use** (`src/cmd/revive.rs:145`): `strict::check_instance` runs before a snapshot value is used.
- **The fake agent runs only an absolute command, directly** (`src/bin/viola-fake-agent.rs:421-428`).
- **A timing value with no measurement is a named PROVISIONAL constant** (`CONNECT_DEADLINE`, `src/cmd/hook.rs:48`;
  `DIALOG_DEADLINE`).

## Conventions to follow
- **One stdout write, through a handle**: the workspace bans `print!`; the dialog arm writes its body with one
  `write` to a locked stdout (`src/cmd/hook.rs:122`).
- **Fixed-message errors**: a parse failure joins `AgentError` (`crates/viola-agent-claude/src/lib.rs:192`),
  never a second enum and never a serde source in an anyhow chain.
- **Every root test file is registered**: a `[[test]]` entry in the root `Cargo.toml`, and a binary that boots
  stamped homes is named in `.config/nextest.toml`'s verify-driven list.
- **Tests in the parser's own crate**: a crate's mutants are graded by its own tests (test-plan §3 `run`).
- **Editor lines**: every graph line above is the query's value plus one.

## New files to create
- `crates/viola-agent-claude/src/statusline.rs` — the statusline payload reader, the settings reader, the override document, the override flag, the shell argv, with their unit and property tests
- `crates/viola-agent-claude/proptest-regressions/statusline.txt` — the property's committed seeds
- `crates/viola-state/src/budget.rs` — `budget.json` under `budget.json.lock`, with its unit tests
- `src/cmd/hook/statusline.rs` — the `hook statusline` arm
- `tests/hook_statusline.rs` — the arm on the real binary
- `fuzz/corpus/hook_stdin/statusline-epoch` — a synthetic seed, `resets_at` as a number
- `fuzz/corpus/hook_stdin/statusline-absent` — a synthetic seed, no `rate_limits`

## Files to modify
- `crates/viola-agent-claude/src/lib.rs` — the new module and its error variant
- `crates/viola-agent-claude/Cargo.toml` — chrono, a workspace dependency the crate does not yet name
- `Cargo.lock` — the one dependency line the crate's new edge adds
- `crates/viola-core/src/lib.rs` — the normalised budget reading both crates share
- `crates/viola-state/src/lib.rs` — the new module
- `crates/viola-state/src/snapshot.rs` — the optional `statusline_command` field and its unit tests
- `crates/viola-state/src/liveness.rs` — one snapshot literal in a test
- `crates/viola-state/src/replay.rs` — one snapshot literal in a test
- `crates/viola-state/src/strict.rs` — one snapshot literal in a test
- `crates/viola-state/tests/state_replay.rs` — one snapshot literal
- `src/cmd/mod.rs` — the default-home test the source rule reads
- `src/cmd/run.rs` — the source read, the override write, the snapshot field, the span-order test
- `src/cmd/revive.rs` — one snapshot literal in a test
- `src/cmd/hook.rs` — the dispatch to the new arm, one snapshot literal in a test
- `src/run/mod.rs` — `child_launch` takes the override path; its two unit tests
- `src/run/dialog.rs` — one snapshot literal in a test
- `src/run/wheel.rs` — one snapshot literal in a test
- `src/run/snapshot.rs` — one snapshot literal in a test
- `src/bin/viola-fake-agent.rs` — the settings read, the statusline run, the `statusline-echo` mode
- `tests/hook_fail_open.rs` — the `unknown_event` case's word
- `tests/cli_fake_agent.rs` — one case per new fake-agent option and mode
- `tests/cli_instance_state.rs` — the override rewritten each start, the snapshot field, the hermetic home
- `tests/tui_passthrough.rs` — one statusline-bearing start
- `tests/support/home.rs` — the helper that plants a statusline source
- `tests/support/fake.rs` — the statusline receipt reader
- `fuzz/fuzz_targets/hook_stdin.rs` — the target also feeds the statusline reader
- `Cargo.toml` — the `[[test]]` entry for `hook_statusline`
- `.config/nextest.toml` — `hook_statusline` in the verify-driven list, if its cases boot stamped homes

## Open questions
- Which form does the per-home source take (finding 3), and is it the founder's to ratify as a new input that
  decides a command? → blocks: plan-decision
- Is the Windows half (the override and the shell-out) built from the documentation, or left out until a
  Windows reading exists (finding 7)? → blocks: plan-decision
- Does `.config/nextest.toml` need the new binary on its list? It depends on whether the cases boot stamped or
  unstamped homes. → blocks: implementation-scope
