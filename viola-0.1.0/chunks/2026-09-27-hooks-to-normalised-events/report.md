# Report — 2026-09-27-hooks-to-normalised-events

**Chunk:** Hooks to normalised events — viola hook verb with exec-form absolute-path plugin hooks, hook-to-kind map,
prompt-submitted paste/tag/harness normalisation, silent unwrapped no-op, fail-open exit 0, perf-gated deadlines; snapshot
replace retry folded
**Date:** 2026-09-27T23:45Z
**Commits:** `bb35d6e chore(2026-09-27-hooks-to-normalised-events): operator pre-CI commit, for the run this chunk's verdict
reads` (`git log 273e1ab..HEAD`; its parent `273e1ab` is the basis for every "parent commit" below). The only later change
is `evidence/operator-pass.md`, still uncommitted.

## Changes (structured — detectors read this)
- **Files** (`git diff --name-only 273e1ab` = 83 paths; source and test paths listed, run dirs and chunk docs omitted):
  - `Cargo.toml`, `Cargo.lock`
  - `crates/viola-core/src/lib.rs`
  - `crates/viola-state/src/{events,fs,snapshot}.rs`
  - `crates/viola-channel/src/client.rs`
  - `crates/viola-agent-claude/{Cargo.toml,src/lib.rs,src/hook.rs (new),proptest-regressions/hook.txt (new)}`
  - `crates/viola-e2e/src/harness/run/mutants.rs`
  - `plugin/hooks/hooks.json`
  - `src/main.rs`, `src/cmd/{mod,run}.rs`, `src/cmd/hook.rs` (new), `src/bin/viola-fake-agent.rs`
  - `tests/{hook_fail_open,hook_events}.rs` (new), `tests/{cli_instance_state,tui_passthrough}.rs`
  - `fuzz/{Cargo.toml,Cargo.lock}`, `fuzz/fuzz_targets/hook_stdin.rs` (new), `fuzz/corpus/hook_stdin/` (new, 10 seeds)
  - `viola-0.1.0/verification-matrix.json`
- **Symbols / APIs:**
  - **`viola hook <event>` CLI verb** — clap `Command::Hook`, `#[command(hide = true)]`, event taken as a plain string; the
    one caller is the plugin's `hooks.json`.
    - Body: `instance_of(VIOLA_NAME, VIOLA_DIR)` → `viola_obs_init(home, ObsProcess::Hook, …)` with its error dropped (a
      failed role-file open leaves no subscriber) → `hook-invoked{hook_event, invoked_at}` → stdin under
      `take(MAX_FRAME + 1)` → normalise → drift report to `detail-hook.ndjson` → `read_snapshot(VIOLA_DIR).endpoint` →
      `Client::connect_by(endpoint, "hook", t0 + 750 ms)` + `notify("hook.event", {ts, event:{kind, data}})` → SessionEnd
      only: `try_append_event` with `Source::Hook` → `hook-decision{hook_event, decision_emitted:false, duration_ms,
      detail?}`.
    - `detail` ∈ `oversize-stdin` · `malformed-json` · `channel-unreachable`; `parse-rejected{parser:"hook-stdin",
      detail:"oversize"|"malformed"}`.
    - Crate-private const `SPINE_DEADLINE` = 750 ms (provisional).
    - `VIOLA_DIR` must be absolute and end `<home>/instances/<name>`, else a silent no-op.
    - Unknown event, clap error (`--help`/`--version` included), `VIOLA_NAME` absent/invalid → exit 0 and nothing written
      anywhere.
    - No server verification and no strict-modes (operator ruling fork 2).
  - **`src/main.rs`:** pre-clap `role_of(argv)`, which skips `--home <v>` / `--home=<v>`; `Outcome{Done, Unparsed, Failed,
    Panicked}`; `exit_code(role, outcome)` (hook → 0 always, other roles keep their code / 1). `Cli::try_parse`: a clap
    error is dropped unprinted for `hook`, `usage.exit()` otherwise. The panic-line writer is split out as
    `write_panic_lines(sink, location, thread, payload)`, which `viola_panic_hook` calls. The panic hook stays the first
    statement of `main`.
  - **Wrapper channel:** `hook.event` is now served. `NoMethods` → `Methods{name, instance_dir}` (`src/cmd/run.rs`), which
    re-validates the event: kind ∈ the 5 hook kinds, `data` an object, and for `prompt-submitted` a string `text` and
    `origin` ∈ harness|human. It then appends `{v, ts: <append time>, instance, kind, source:"hook", data}` via
    `append_event`. An invalid event is not appended; an append failure is `-32603`. Every other method stays `-32601`.
    `append_event` gains its second production caller.
  - **`viola-channel`:** `Client::notify(method, params)` (id-less, stamps `v`/`sender`/`conn`, logs `channel-request` with
    no `corr`, span `channel.request`) and `Client::connect_by(endpoint, process, busy_until)`. `connect` =
    `connect_by(…, busy_deadline(now))`, keeping 2 s. `busy_deadline`/`BUSY_WITHIN` are no longer `cfg(windows)`-only.
    Internal: `open` → `open_by(endpoint, busy_until)`, with `open` kept `#[cfg(all(windows, test))]` for `server/win.rs`'s
    test, the other caller.
  - **`viola-core`:** `EventKind` + `SessionStart`, `PromptSubmitted`, `TurnEnded`, `SessionEnd`, `Activity`
    (`session-start` · `prompt-submitted` · `turn-ended` · `session-end` · `activity`).
  - **`viola-state`:**
    - `Source::Hook` (`"hook"`).
    - `try_append_event(instance_dir, line) -> Result<bool>`: `Ok(false)` with nothing written while `events.ndjson.lock`
      is held.
    - `fs::REPLACE_ATTEMPTS` = 100 (pub).
    - `replace_private` retries `persist` every 10 ms while `retry_replace(raw_os_error, HOST_IS_WINDOWS)` holds (raw 5,
      Windows only), at most 100 attempts, re-persisting the same temp file; any other error returns at once.
    - Crate-private seams `replace_private_with(…, pause)` and `snapshot::write_snapshot_with(…, pause)`.
    - Callers inheriting the retry: `write_snapshot`, `replace_private_shared` (pin and plugin writes).
  - **`viola-agent-claude::hook`** (new public module): `HookEvent` (7 events, `as_str` = diag-line `$defs.hook_event`,
    `from_arg`, `kind()`, `ALL`); `AgentError::Malformed`; `Normalised{kind, data, drift}`; `normalise(event, bytes)`.
    - Parse: tolerant, with a `serde_path_to_error` typed view of `session_id`, `source`, `prompt`,
      `last_assistant_message`, `tool_name`. A wrong-typed field → drift entry `{path, expected:"string"}` and read as
      absent.
    - `data` per kind: session-start `{cause ∈ startup|clear|resume|compact|unknown, agent_session_id|null}` ·
      prompt-submitted `{text, origin ∈ harness|human}` · turn-ended `{last_assistant_message|null}` · session-end `{}` ·
      activity `{}` + `tool`.
    - Prompt order: classify on the RAW prefix (`<agent-message from=` / `<task-notification>`), then unwrap the CLI's
      unescaped pair `<pasted_content id="X">\n…\n</pasted_content id="X">` (same id, ends kept byte for byte), then
      un-escape `<\` before an ASCII letter or `/`.
  - **Fake agent:** fires `SessionStart` variant `default` once after `start_receipts`. It is silent without a registered
    hook or a fixture, as `fire` already was.
  - **`PLUGIN_FILES`:** the comment now says only `mcpServers` stays empty.
- **Crates / modules:**
  - new: `viola-agent-claude::hook`, the root `cmd::hook`, and fuzz target `hook_stdin`;
  - dependency edge: `viola-agent-claude` → `viola-core`;
  - no crate added or removed.
- **Dependencies:**
  - `viola-agent-claude` gains `viola-core` (path), `serde`, `serde_json`, and the new `serde_path_to_error =0.1.20`
    (`[workspace.dependencies]`, MIT/Apache-2.0; `cargo deny check` green).
  - Its dev-deps gain `proptest` and `rstest`.
  - `fuzz/Cargo.toml` gains the `viola-agent-claude` path dependency; `fuzz/Cargo.lock` adds `serde_path_to_error 0.1.20`
    and `viola-agent-claude` (`git diff 273e1ab -- fuzz/Cargo.lock`: 23 lines added).
  - The root `Cargo.lock` adds `serde_path_to_error` (17 lines added).
- **Schema / config:**
  - `plugin/hooks/hooks.json`: `{"hooks": {}}` → seven exec-form entries, each `"command": "@@VIOLA_BIN@@", "args":
    ["hook", "<kebab event>"]`. SessionStart / UserPromptSubmit / Stop take `"timeout": 5`; SessionEnd has no timeout key;
    Notification / PostToolUse / PostToolUseFailure take `"async": true`. No PreToolUse or PermissionRequest entry.
  - No `schemas/*.json` change: `diag-line`/`diag-detail` already carried every field. The `drift_report` detail line uses
    `event:"parse-rejected"`, target `viola::cmd::hook`, `level:"WARN"`.
  - No new config key, and no new env var: `VIOLA_NAME`/`VIOLA_DIR` are now read by `hook` (they are set by `run`).
  - Harness: `run --mutants` sets `CARGO_TARGET_DIR` = `<repo>/target/mutants` for the root pre-build and relative
    `target/mutants` for `cargo mutants` (was `env_remove`).
- **Spec-master edits:** none. No master was touched by this chunk; the P4 scope rulings were recorded in scope.md.
- **Counts / qualifiers moved:**
  - Registered plugin hooks: 0 → 7 (`grep -c '"command": "@@VIOLA_BIN@@"' plugin/hooks/hooks.json` = 7).
  - `EventKind` variants: 2 → 7.
  - Fuzz targets: 2 → 3 (`ls fuzz/fuzz_targets`).
  - Workspace crates with a committed `proptest-regressions/`: 1 → 2.
  - Wrapper channel methods served: 0 → 1 (`hook.event`).
  - Coverage suites on the operator pre-push: Linux 745, Windows 759.
- **Dev-tool versions:** none — nothing installed or upgraded. cargo-mutants 27.1.0 and cargo-nextest 0.9.146 re-read
  unchanged.
- **Harness / gate surface:** `crates/viola-e2e/src/harness/run/mutants.rs`: the mutation run's own target dir,
  `MUTANTS_TARGET = "target/mutants"`.
  - The root pre-build writes `<repo>/target/mutants`; cargo-mutants gets relative `CARGO_TARGET_DIR=target/mutants`, so
    its copied tree builds every test binary in its own copy.
  - It is still `--copy-target=true`. New test: `run_mutants_builds_in_its_own_target_dir`.
  - No change to the status or verdict shapes, the 5 agent commands, or `ci.yml`.
- **Cross-project / external claims:**
  - CI run **ci#36358593772** on sha **`bb35d6e`**: `ci.py conclusion`: `verdict: green · checks 15/15 · wall 972 s`
    (23:24:41Z → 23:41:13Z). Every job `success` (`gh run view 36358593772 --json jobs`): test ×3, mutants ×2,
    mutants-verdict, lint ×3, release ×3, msrv, fuzz-replay, supply-chain. The overseer has verified the same run.
  - M5 (paste wrapper on the wire) was measured at P5 from `~/.viola/sessions/viola-builder/events.ndjson` and
    `viola-lab/prototype/src/store.rs:241–249` (research.md).
- **Reverted / negative API facts:**
  - No panic trigger of any kind: `FAKE_AGENT_HOOK_PANIC` was rejected at P5 (entry 13's grep: exit 1, no output).
  - No `HookEvent::is_spine()`: the plan named it, but nothing consumes it.
  - No new hook init fn in `src/obs.rs` (see Deviations).
  - No PreToolUse / PermissionRequest registration (that is :62's).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - (a) The plan's companion sweep says "`tests/cli_instance_state.rs:344–355` asserts the rewrite, not the body: no
    change". It asserted the literal `"{\n  \"hooks\": {}\n}\n"` and needed an edit; it now compares against the first
    start's bytes. This is a plan-only claim, no master: disposition ≈ plan error, closed by the edit.
  - (b) Research's graph impact omits `client::open`'s second caller, `crates/viola-channel/src/server/win.rs:149`
    (test). Plan-only; closed by the test-only `open` wrapper.
  - (c) The implicit harness claim that a local `run --mutants` / pre-push Windows leg grades root-integration-only
    mutants was false while `target/debug` held a root test binary. Where it is stated: test-plan §3 `run` step 4 and
    `verification-harness.md` (the run --mutants bullet: "`--copy-target`"; `grep -c copy-target test-plan.md` = 1).
    Evidence: `evidence/red-mutants-stale-test-binary.md`. A hand-applied mutant failed 4 tests in the repo but passed in
    the scratch, where the copied binary embedded `D:\dev\projects\viola\target\debug\viola.exe`. Fixed in the harness;
    test-plan §3 should state the `target/mutants` target dir.
- **Expected amendments (from plan)** (site counts from
  `grep -c -- "<pattern>" .andromeda/{architecture,security-plan,design-system,layout-templates,test-plan,obs-plan,a11y-plan}.md`):
  1. **arch [Deployment / Distribution]**, the `{"hooks": {}}` placeholder retired and the 7 entries and tiers stated
     (pattern `hooks": {}`: arch 1, test 1): carried, Schema/config bullet. The test-plan hit is the same placeholder,
     also stale.
  2. **arch §Occupied Resources Repository**: `fuzz/fuzz_targets/hook_stdin.rs`, `fuzz/corpus/hook_stdin/`,
     `crates/viola-agent-claude/proptest-regressions/` (`fuzz_targets`: arch 2, test 2): carried, Files / Counts.
  3. **arch [CLI Version Compatibility] paste-wrapper row**: the close is `</pasted_content id="X">`, and a typed close
     arrives as `<\/` (`pasted_content`: arch 4, test 5): carried, the Symbols `hook` module prompt order.
  4. **arch §Workspace crates**: `viola-agent-claude` depends on viola-core, serde, serde_json, serde_path_to_error
     (`serde_path_to_error`: arch 3, security 7, obs 6): carried, Dependencies.
  5. **arch [Hook Transport]**: the hook-local provisional 750 ms spine deadline (`spine deadline`: arch 1, test 5,
     obs 2): carried, Symbols `SPINE_DEADLINE`.
  6. **security-plan Decisions Log**: the interim gap, `hook.event` without server verification or strict-modes until
     Epoch 6 (security `Decisions Log` section): carried, Symbols (the `viola hook` bullet's last line).
  7. **security-plan §Input Validation**: the hook stdin `take(MAX_FRAME)` joins the consumer list (`MAX_FRAME`:
     security 18): carried, Symbols.
  8. **test-plan §7 Fake agent**: it fires SessionStart at start (`SessionStart`: test 1): carried, Symbols (Fake agent).
  9. **test-plan §3 `boot` readiness**: line 3 moves to :51 (`session-start{source`: test 1): carried; the harness boot is
     unchanged (Harness bullet: no boot change).
  10. **test-plan §6 Property suite**: the statusline `resets_at` property moves to :71 (`resets_at`: test 8): carried;
      not built here (only the hook stdin + round-trip properties landed).
  11. **test-plan §6 Security sweep**: the forced-panic case and its trigger move to the "Hook perf gate" tail as
      UNRATIFIED, pending the founder's live word (`forced`: test 13): carried, Reverted (no panic trigger).
  12. **obs-plan §3 D-28 / test-plan §5**: the concurrent-append check lands without a >4 KiB detail line, and that half
      moves to the tail (`4 KiB`: obs 5, test 2, arch 1): carried; `hook_processes_append_whole_lines_side_by_side` (8
      processes, 16 + 8 lines).
  13. **obs-plan §10 / test-plan §9**: the perf rows move to the tail entry "Hook perf gate" (`hyperfine`: test 21,
      obs 18, arch 1): carried; no perf arm, hyperfine or CI job here.
  - Not in the plan's list but carried: test-plan §3 `run` step 4 / `verification-harness.md`, the `target/mutants` target
    dir (claim (c) above).
- **Coverage of new surfaces:**
  - `viola hook` stdin (external input) → validation `take(MAX_FRAME+1)` + one-object parse + closed `HookEvent` +
    `ViolaName::try_new` + absolute `<home>/instances/<name>` check ✓ · instrumentation `hook.handle` span,
    `hook-invoked`/`hook-decision`/`parse-rejected` ✓ · PII: payload text never in a role line, drift reports carry
    path + type only ✓ · tests unit (`src/cmd/hook.rs` 11) + integ (`hook_fail_open` 13, `hook_events` 3) + proptest +
    fuzz ✓ · a11y n/a · tokens n/a.
  - Wrapper `hook.event` dispatch (external input over the channel) → re-validation (kind closed set, object data, prompt
    text/origin) ✓ · instrumentation: the existing `channel.dispatch` span + `channel-request` ✓ · PII: `data` is written
    only to `events.ndjson` (instance log), never a role line ✓ · tests unit (`methods_*` 15 cases) + integ ✓ · a11y n/a
    · tokens n/a.
  - `Client::notify` → no input (client side) n/a · instrumentation `channel.request` span + `channel-request` without
    `corr` ✓ · PII: params never logged ✓ · tests unit ✓.
  - `replace_private` retry → n/a validation · instrumentation: the existing `state.snapshot_write` span, no per-attempt
    line (a hot path) ✓ · PII n/a · tests unit + Windows witnesses (a)(b)(c) + the remove-the-guard pair ✓.
  - Outer terminal while hooks fire → a11y: the tui clause-1 variant `tui_hooks_firing_add_no_viola_bytes` ✓.

## Deviations from intent
- **Plan step 9.3's "hook-safe init path in `src/obs.rs`" is not a new fn.** `viola_obs_init` already returns before
  setting a subscriber when the role file will not open, which is a sink in effect. A second init would be an
  unobservable duplicate and an unkillable mutant (testing.md 2026-09-24, ext. 2026-09-27). The hook calls
  `viola_obs_init` and drops its error; `src/obs.rs` is untouched.
- **Plan step 6's `HookEvent::is_spine()` is omitted.** It has no consumer: one 750 ms deadline covers every event.
- **Plan step 5, the `client::open` split:** kept as a `#[cfg(all(windows, test))]` wrapper over `open_by`, so
  `server/win.rs` (outside the lists) is untouched.
- **Plan step 13's Path 1 check is a sibling test**, `path1_session_start_is_record_three_through_the_absolute_hook`. The
  existing `path1_start_writes_state_before_the_spawn` keeps its no-fixture `[wheel, budget-gate]` chain; the plan
  allowed either.
- **The P2 red fold outside the lists:** `crates/viola-e2e/src/harness/run/mutants.rs` (the CLAUDE.md learning: a red met
  during a chunk folds into it).
- **The operator-pass hygiene fold:** `.andromeda/runs/2026-09-27T21-14-58-phase/control-seam/hook.rs` →
  `hook.rs.txt`, content kept. The plan's entry-13 `baseline` still cites the old name; the evidence maps the two.

## Decisions & corrections
- **Overseer direction at implement:** "The founder is asleep: fold every red found in the operator pass into this chunk,
  and read CI through the ci.py conclusion tool call." The session ran the operator pass: pre-push, hygiene, pre-CI
  commit, guarded push, `ci.py conclusion`.
- **Overseer at wrap:** CI is verified (run 36358593772 on bb35d6e, 15/15 green). The Hook perf gate tail carries the
  seam items as **UNRATIFIED**, pending the founder's live word before its phase.
- **Sweep hazard — a copied cargo target keeps the ORIGINAL tree's `CARGO_BIN_EXE_*` paths.** Cargo reads the copied
  test binary as fresh, so a `--copy-target` mutation run drives the unmutated binary for root integration tests. Only a
  hand-applied known-positive mutant showed it; the union hid it behind the other leg.
- **An ad-hoc `cargo nextest`/`cargo test` in the default `target/` is what planted the stale binary.** The harness
  builds in `target/harness`.
- **`cargo test` (not nextest) runs a crate's `capture_global` tests in one process,** and the channel crate's
  global-subscriber test reads another test's lines: run crate tests through nextest.
- **Windows `ExitCode` has `PartialEq` but not `Eq`:** a derive on an enum holding it can take only `PartialEq`.
- **`TestHome` is removed on `Wrapper::stop()`:** a test that reads the home after the wrapper stops needs
  `stop_keep()`.
- **The bash-guard blocks a heredoc carrying a doubled backslash:** a Rust-escape payload goes through the Edit tool.

## Outcome
- **Acceptance** (re-asserted against the diff):
  - **(arch) hooks.json:** 7 exec-form entries on the pinned forward-slash path, `args ["hook", "<kebab>"]`, no dialog
    tier, none on PATH. MET: `plugin_files_are_the_three_layout_paths` literal and entries 10–12.
  - **(arch) every registered event:** one line of its kind, `source:"hook"`, literal `data` key sets, no raw payload
    key. MET: `hook_every_registered_event_lands_as_one_line_of_its_kind` (v1-28).
  - **(arch/design) prompt normalisation:** harness origin by prefix, the CLI pair unwrapped, the typed pair un-escaped
    and never unwrapped, every kind/origin/cause a known word. MET: `hook_prompts_arrive_normalised_with_their_origin` +
    unit rstest tables + the round-trip property.
  - **(security/obs) fail-open matrix:** 11 cases exit 0, empty stdout/stderr, < 1.0 s; the no-instance cases create no
    `diagnostics/`/`instances/`. MET: `hook_fails_open_silently_within_the_spine_bound`.
  - **(obs) in-process panic path.** MET: `exit_code_maps_role_and_outcome` (hook × panic → 0, run → 1),
    `role_of_skips_the_home_flag_and_its_value` (`--home <dir> hook …` → hook),
    `panic_lines_for_a_hook_split_the_payload_into_the_detail_file` (1 payload-free role line + 1 schema-valid detail
    line). The forced panic on the real binary is CARRYd to the tail, UNRATIFIED.
  - **(obs) 8 concurrent processes:** 16 + 8 whole lines. MET on 3 CI OSes (ci#36358593772 test ×3 success); the
    >4 KiB half is CARRYd.
  - **(obs) drift report only in `detail-hook.ndjson`,** schema-valid; the canary reaches no home-level file. MET:
    `handle_writes_a_drift_report_only_to_the_detail_file`, the concurrent test, and `secret-scan` green.
  - **(tests) Path 1:** records 1–3 `wheel` → `budget-gate` → `session-start{source:"hook", cause:"startup"}`, and the
    receipt `command_absolute:true, ran:true, exit_code:0`. MET.
  - **(tests) two proptest properties at 512 cases,** `proptest-regressions/hook.txt` committed; CI `fuzz-replay`
    success over the 10-seed corpus. MET.
  - **(CARRY 5) held reader.** MET: witnesses (a) released → lands, (b) never released → exactly `REPLACE_ATTEMPTS`
    attempts, old bytes; the remove-the-guard pair red then green (`evidence/replace-retry.md`).
  - **(layouts) `viola --help` lists no `hook`.** MET: `hook_verb_is_hidden_from_the_help`.
  - **(a11y) no viola bytes while hooks fire.** MET: `tui_hooks_firing_add_no_viola_bytes`.
  - **(tests) coverage floors + CI mutation union.** MET: ci#36358593772 `test` ×3 (coverage gate), `mutants` ×2 +
    `mutants-verdict` success.
  - No criterion is contradicted by the diff.
- **Gates** (implement's second full block, run dir `2026-09-27T21-48-40-implement`, after the fold) — every entry
  green:
  - `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`,
    `bash scripts/lint-probes.sh`, `bash scripts/orphans-check.sh`,
    `cargo check -p viola-core -p viola-pty -p viola-agent-claude -p viola-state -p viola-channel`, `cargo deny check`,
    `cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`, the deny-sync ban on viola-agent-claude,
    `bash scripts/deny-probes.sh`;
  - the hooks.json greps: 7 · 0 (exit 1) · 3;
  - the panic-seam grep: exit 1, no output;
  - `ls fuzz/corpus/hook_stdin | wc -l` = 10;
  - `run --unit`, `run --unit --filter 'package(viola-state)'`, `run --integration` (homes kept), `schema-check`,
    `secret-scan`;
  - the three scoped mutants runs over `fs.rs`, `agent-claude hook.rs` and `src/cmd/hook.rs`, each `verdict:"scoped"`
    exit 0 (`src/cmd/hook.rs` 22 caught / 3 unviable);
  - harness smoke cleanup → boot → status `ready` → cleanup `processes_gone`/`endpoint_gone`;
  - `release-check.sh` (viola only), MSRV 1.96 check, and `pre-push` (union, breaches []).
  - The first block's red, `run --mutants --file src/cmd/hook.rs` (4 missed), is fixed with its cause known (the evidence
    file). The `leg = 'operator'` entries were driven by hand in the operator pass: hygiene clean (after the fold),
    pre-push green (`op-31.*`), the guarded push `273e1ab..bb35d6e`, and `ci.py conclusion` green (ci#36358593772).
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran. Its commit list (Setup 4) is `bb35d6e` only, with no fix commits; the final
  HEAD's CI run ci#36358593772 is recorded in `evidence/operator-pass.md`. Implement's P4 report (this conversation) is
  the basis for the deviations and the P2 fold.
- **Process hygiene:** re-measured at 2026-09-27T23:41Z (`Get-CimInstance Win32_Process`). No process started by this
  chunk's runs remains. Three `viola.exe` from `D:\dev\projects\additional\viola-lab\prototype` run; they were not started
  by this chunk (the prototype's).
