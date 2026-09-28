# Codebase Research — 2026-09-28-capability-ledger-and-viola-verify

## Scope
- **Depth:** deep · **Reads:** 16 · **Globs/Greps:** 22 · **Graph queries:** 1 (rust plane)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 5 Session Additions applied (the E1
  2026-09-25 extension, never piping `boot`, `--in-diff` regeneration); `.claude/rules/testing.md` — read in full, 16
  Session Additions applied (unobservable bodies, cfg-split readers, waits below the kill line, one process-global test per
  `OnceLock`); `.claude/rules/host-win32.md` (always loaded)
- **Platform issues consulted:** none (no runner-only bullet: Setup 5a read CI green on `9b4f6f4`, ci#36408767764)

## Files inspected
- `src/main.rs` (full) — the catch site (`:44-66`): `dispatch` error → `obs::report_internal_error`, caught panic →
  `obs::internal_error_exit_line`; `exit_code` (`:105-111`) maps every non-hook failure to exit 1. **Neither path writes
  stderr**, so the `error: internal error` line has no writer today.
- `src/cmd/mod.rs` (full) — `Command` has `Run` and a hidden `Hook` only (`:28-34`); `dispatch` builds a `DetailSink` only
  for `run` (`:55-59`). `verify` has no module slot yet.
- `src/human.rs` (full) — one writer, `write_refusal` / `refuse` (stderr `unable:` + `hint:`, one `write_all`, `:9-17`); no
  stdout result writer and no error-line writer.
- `src/obs.rs` (`:1-320`) — `role_file_name` already names `cli-<name>.ndjson` for `ObsProcess::Cli` with an instance and
  returns `None` without one (`:48-62`, obs D-06); `internal_error_exit_line` writes only for `Run` (`:220-231`);
  `report_internal_error` writes the chain only into the detail file (`:242-248`); `parse-rejected` is the pattern for
  `config-json` (`:208-216`). The file is 701 lines.
- `src/cmd/run.rs` (`:1-340`) — `start` (`:146-197`) runs resolution (`:153`) → collision (`:164`) → `pin_and_plugin`
  (`:167`) → `bind_endpoint` (`:171`) → `start_state` (`:179`) → spawn. The doc comment says "The version gate has no step
  yet" (`:143-145`). `start_state` hard-codes `cli_verified: false, cli_version: None` (`:278-279`). `Dispatch` answers only
  `hook.event`, and every other method (including `hook.dialog`) is `-32601` (`:98-108`, test at `:467`).
- `src/run/mod.rs` (full) — `resolve_program` (`:117-126`) wraps `viola_agent_claude::resolve_program` over this process's
  `PATH`/`PATHEXT`; `log_child_start` is the `claude-child` `process-start` without `cli_version`/`cli_verified` (`:31-44`).
- `crates/viola-agent-claude/src/lib.rs` (outline + `:1-160`) and `Cargo.toml` — the landed Claude-specific constants:
  `IDENTITY_FLOOR` (`:11`), `SHIM_TARGET` (`:29`), `PLUGIN_DIR_FLAG` (`:110`), `PLUGIN_FILES` (`:118`), `resolve_program`
  (`:178`). Dependencies are viola-core, serde, serde_json, serde_path_to_error and thiserror, and **no viola-state**. The
  feature is `fake-agent = []`.
- `crates/viola-agent-claude/src/hook.rs` (outline) — `HookEvent` (`:11`), `AgentError` (`:64`), `HARNESS_PREFIXES`
  (`:95`), the paste unwrap (`:171-204`) and `unescape_tags` (`:206`). These are hard-coded behaviours that arch requires to
  be ledger rows. No decision body exists: `src/cmd/hook.rs:208-218` logs `hook-decision` with `decision_emitted = false`
  on both arms.
- `src/bin/viola-fake-agent.rs` (`:1-530`) — it answers `--version` as `"<v> (Claude Code)"` (`:85-88`), with
  `--cli-version` / `--report-version` (`:62-64`) and default `2.1.0` (`:17`). It is interactive only: SessionStart/default
  at launch (`:522`), UserPromptSubmit on a submitted line (`:261-280`) and scripted steps. A payload is read only from
  `<fixtures>/<cli-version>/<Event>.<variant>.json`, otherwise `"no-fixture"` and no hook runs (`:210-219`). There is no
  print (`-p`) mode.
- `crates/viola-e2e/src/harness/boot.rs` (`:1-180`) — steps 1-3 and 5 only; the header says step 4 `viola verify` arrives
  with the verb (`:1-4`). `DEFAULT_CLI_VERSION = "2.1.0"` (`:21`); there is no `--unstamped` and no `verify-failed`.
- `crates/viola-e2e/src/harness/supervise.rs` (`:40-69`) — `spawn_wrapper` passes `--cli-version` but **no `--fixtures`**
  (`:55-59`).
- `tests/support/home.rs` (`:140-200`) — `stamped_home` is the interim no-stamp seam (`stamped: false`, `:152-168`).
- `tests/support/hygiene.rs` (`:1-40`) and `tests/contract_fixture_hygiene.rs` (outline) — the class-only checker exists,
  and the walk covers only `fixtures/fake-scripts/*.json` (`:22`). Its header defers the `fixtures/claude/*/*.json` walk
  (`:3`).
- `schemas/diag-line.v1.json` (probe) — already admits `cli_version` / `cli_verified` (`:73`), `subject: "version-probe"`
  (`:127`), `parser: "ledger-stamps"` (`:100`) and `process: "cli"`. **G4 needs no schema edit** for the named lines.
- `.github/workflows/ci.yml` (grep) — the `test` job already runs a harness `boot`/`status`/`logs`/`cleanup` smoke
  (`:59-79`). A boot that runs step 4 puts fake-agent `viola verify` into CI with no new job.
- `viola-0.1.0/working-route.md:57-68` — Epoch 3 owns "Readiness gate … input-box and modal signatures as ledger rows"
  (`:58`), "/clear post-condition" (`:60`), "Dialog answers by dialog_id" (`:62`) and "First live test and self-drive —
  real-CLI verify" (`:68`).

## Graph impact
- **resolve_program** — production callers are `cmd/run/start()` @ `src/cmd/run.rs:153` and `run/resolve_program()` @
  `src/run/mod.rs:125` (graph lines 152 / 124, +1). The rest are viola-agent-claude's own tests. `verify` and the version
  gate are two new callers of the root wrapper, and the signature is unchanged.
- **replace_private** — the only production caller is `replace_private_shared` @ `crates/viola-state/src/fs.rs:135`; the
  rest are its tests (trace `tree-query-2026-09-28-capability-ledger-and-viola-verify.json`). The `stamps.json` writer is
  its first direct product caller, and the signature is unchanged.
- **report_internal_error** — one caller, `main()` @ `src/main.rs:56`, plus two obs tests. The catch-site line changes its
  side effects for the `cli` role only.
- **start_state** — one caller, `cmd/run/start()` @ `src/cmd/run.rs:179`. The version gate threads
  `cli_version`/`cli_verified` into it.

## Patterns detected
- **Consumer-first shapes** (`tests/support/home.rs:152-154`; `contract_fixture_hygiene.rs:3`; the prior chunk's P4 ruling):
  a seam, schema or table lands with its first real consumer and a test. No shape is invented before a recorded fixture
  exists.
- **One human writer** (`src/human.rs:9-17`): fixed text, one `write_all`, and a closed pipe is swallowed.
- **Tolerant parse → degrade + `parse-rejected`** (`src/obs.rs:170-216`): an unusable persisted file degrades and is logged
  at `warn`, never fatal.
- **Fake agent flags, not env seams** (`fake-agent.rs:56-79`): every test mode is a CLI flag; the two ratified env seams
  are the closed count.

## Conventions to follow
- **Start order as landed** (`src/cmd/run.rs:143-197`): the gate slots between `pin_and_plugin` and `bind_endpoint`, a
  `.cmd`/`.bat` is refused before any probe spawn, and `run` prints nothing on the success path.
- **Oracles are literals** (testing.md): the row ids and count asserted in tests are written out, never imported.
- **Waits below the kill line** (testing.md 2026-09-24): every new spawn-and-wait (a probe child, `verify` in
  `stamped_home`) detects exit and is bounded below the 7 s `WITHIN` / 10 s mutants kill.

## New files to create
<!-- Narrowed at P4 to the HEAD (scope.md §Narrowed at P4). The tail's files — tests/support/home.rs,
the harness boot/supervise/mod, tests/contract_ledger_probes.rs — leave with the tail entry. -->
- `crates/viola-agent-claude/src/ledger.rs` — the closed row table, `--version` parse, stamp shape, capture plugin, scrub, post-conditions
- `crates/viola-state/src/stamps.rs` — `ledger/stamps.json` bytes under its `.lock` sibling: the one writer path and the capped read
- `src/cmd/verify.rs` — the `viola verify` verb: resolution, pin, probe runner, step-counter output, stamp write, `--record`
- `src/run/version_gate.rs` — `run`'s `--version` probe and stamp read, `cli_version`/`cli_verified`, its obs lines
- `tests/cli_verify.rs` — `viola verify` against the fake agent: lines, stamps, sole writer, `--record`, catch-site line
- `tests/cli_version_gate.rs` — unstamped / unlisted / unparseable / corrupt / stamped homes → snapshot and obs lines
- `schemas/claude-fixture.v1.json` — the tolerant claude-fixture schema the hygiene walk checks
- derived `fixtures/claude/*/*.json` by `viola verify --record fixtures/claude` — the first recorded, scrubbed set (operator pass)

## Files to modify
- `crates/viola-agent-claude/src/lib.rs` — declare and re-export the ledger module
- `crates/viola-state/src/lib.rs` — declare the stamps module
- `src/cmd/mod.rs` — the `Verify` subcommand, its dispatch and detail sink
- `src/cmd/hook.rs` — the hidden `--capture <dir>` arm: raw stdin into the probe dir, fail-open
- `src/cmd/run.rs` — the version-gate step in `start`, `cli_version`/`cli_verified` into `start_state`
- `src/run/mod.rs` — declare the gate module; `log_child_start` carries `cli_version`/`cli_verified`
- `src/main.rs` — the `verify` role and its catch-site stderr line on a failure or a caught panic
- `src/human.rs` — the `error: internal error` writer and the verify result-line writer
- `src/obs.rs` — the `cli` role's exit line at the catch site
- `src/bin/viola-fake-agent.rs` — the `-p`/`--print` mode: spine hooks from the fixture set, one reply line, exit
- `tests/contract_fixture_hygiene.rs` — the `fixtures/claude/*/*.json` walk and its planted cases
- `Cargo.toml` — the `[[test]]` entries for the two new root tests (`required-features = ["fake-agent"]`)

## Open questions (all three resolved at P4 by operator ruling — scope.md §Narrowed at P4)
- **The probe mechanism is specified nowhere.** Arch :91 says only "runs the live probe suite (Haiku, local only), checks
  post-conditions, stamps, records fixtures". Test-plan fixes only the result line (:290/:518) and the contract test (:967).
  How `verify` drives the CLI (a print-mode `claude -p` with a capture plugin, or a PTY drive like `run`) and how it captures
  RAW hook payloads for recording (today `viola hook` forwards only normalised events, `src/cmd/run.rs:77-108`) must be
  decided. Against the fake agent, any probe needs a fixture set, because without one no hook fires
  (`fake-agent.rs:217`). → blocks: plan-decision
- **Size and split.** The full entry is the ledger, about 17 rows, a new verb with a probe runner and a raw-payload capture,
  live recording, the scrub, a schema and hygiene walk, the version gate, the catch-site line, `stamped_home` across 5 root
  test files (~55 references, `grep -c 'stamped_home\|booted_wrapper' tests/*.rs`), harness boot step 4, readiness line 3
  and `supervise --fixtures`. Recent code diffs for comparison: 233 / 1102 / 2702 / 1167 insertions for cli-output-tokens,
  hook-perf-gate, hooks-to-normalised-events and browser-verdict-reachability (`git diff --shortstat <pre-CI>^ <feat>`,
  spec dirs excluded). At several times the largest of those, it will not fit one implement window. → blocks: plan-decision
- **Rows without a consumer.** Many rows (S3/S7/S8 dialogs, screen signatures, statusline, `agents --json` join, plugin
  precedence, local commands, turn end without Stop, dialog concurrency) have no consumer at HEAD. The route already owns
  several of them with their consumers (:58, :60, :62, :68). The rows viola relies on at HEAD are the identity floor, npm-shim
  resolution, the hook tier map as registered, harness prefixes, the long-paste wrapper, tag escaping and the
  largest-hook-payload row. → blocks: plan-decision
