# Report — 2026-10-17-end-to-end-path-binary-and-harness-legs

**Chunk:** the first path_ E2E binary (Path 1), run --e2e, boot --statusline-echo and --agents-mode, Path 6's two-instance harness leg, status reading viola list
**Date:** 2026-10-17
**Commits:** `ae21376 chore(2026-10-17-end-to-end-path-binary-and-harness-legs): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since `last_wrap` 2026-10-16T23:41:23Z beyond the last wrap's own `6042af6`; basis `git log --format='%h %s' 6042af6..HEAD`)

## Changes (structured — detectors read this)
- **Files:** new — `crates/viola-e2e/src/harness/session.rs`, `crates/viola-e2e/tests/path6_budget_harness.rs`,
  `tests/path1_run_start.rs`, `tests/support/start.rs`; modified — `crates/viola-e2e/src/harness/{run/nextest.rs,
  run.rs, boot.rs, mod.rs, status.rs, cleanup.rs, run/perf.rs}`, `crates/viola-e2e/src/bin/viola-harness.rs`,
  `crates/viola-e2e/tests/{harness_lifecycle.rs, cli.rs}`, `tests/cli_instance_state.rs`, `tests/support/mod.rs`,
  `Cargo.toml`, `.config/nextest.toml` (basis `git diff --name-only 6042af6` minus `.andromeda/`, `viola-0.1.0/`,
  `.claude/session-handoff.md`). No product source, plugin file, fixture or schema (gate `test -z "$( (git diff
  --name-only 6042af635c3e -- src crates/viola-core … schemas) …)"` green).
- **Symbols / APIs:** all in the test-only `viola-e2e` package or root test support, none in a product crate.
  - `harness::run::nextest`: `INTEGRATION` = `kind(test) & !binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)`
    (was `kind(test)`); new `E2E` = `binary(/^(path[0-9]*|tui|mcp|cross|chaos|contract)_/)`. The regex WIDENS test-plan
    §3's `^(path|tui|mcp|cross|chaos|contract)_` by `[0-9]*` after `path` (the overseer's answer, `inputs#I2`): the
    plan's binaries `path1_run_start` / `path6_budget_harness` do not match the written regex (measured, block run 1).
  - `harness::run::Selection` gains `e2e: bool`; `from_flags` sets it under no selector and under `--all`;
    `test_suites` runs the E2E step third, after unit and integration, as suite `nextest-e2e`, through the one
    `nextest(…)` shape and target dir, a `--filter` intersecting it; `--coverage` still replaces all three nextest runs;
    `run --e2e` alone runs no doctest (doctest stays with unit / integration / coverage); the archive list is unchanged
    (unit, integration, coverage JUnit).
  - `harness::boot`: new `AgentsMode` (`recorded` · `oversize` · `malformed`, kebab serde, clap `ValueEnum`, default
    `recorded`), `AGENTS_MODE_FILE = "agents-mode"`, `statusline_echo_command(bin_dir, home) -> Option<String>` (`None`
    when either path holds a character outside ASCII letters, digits and `_ - . / :`); `BootOptions` gains
    `statusline_echo: bool` and `agents_mode: AgentsMode` (every literal updated: `viola-harness.rs`, `boot.rs` tests,
    `run/perf.rs`, `tests/cli.rs`, `tests/harness_lifecycle.rs`); `SuperviseSpec` gains `#[serde(default)] agents_mode`.
    `start` removes a stale `<session>/bin/agents-mode` before the stamp and writes it (`replace_private`, 0600) AFTER
    the stamp and the source plant, before `supervise.json`: verify's Run B reads `claude agents --json` through the
    same session copy, so a mode written first failed the `agents-join` row (`verify-failed`, measured block run 1).
    `--statusline-echo` writes `<home>/statusline-source.json` (`replace_private` 0600, the settings shape
    `{"statusLine":{"type":"command","command":"<bin dir>/viola-fake-agent[.exe] statusline-echo
    <home>/fake/statusline.marker"}}`, forward slashes) after the stamp, making `<home>/fake/` through
    `create_private_dir`; a non-shell-plain path fails boot `build-failed`. No env var set.
  - `harness::SessionRecord` gains `#[serde(default)] agents_mode` (written by `boot`; an older record parses).
  - `harness::status`: `pub fn derive(code, list, booted) -> Derived{state, last_error}` and `status` now run
    `<bin dir>/viola --home <home> list --json` (bin dir and session `bin/` from `supervise.json`), `PATH` =
    `session_path(session_bin)`, `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` removed, stdout through
    `take(MAX_FRAME + 1)`. `ready` = exit 0 and every booted name an item `wrapped:true, liveness:"live"`; `degraded` =
    one `stale`, absent (a gone instance is omitted) or no `items`; `down` = non-zero exit (`last_error
    "list-exit-<code>"`), no exit / no spawn / no `supervise.json` (`"list-exit-none"`, a new closed value), exit 0 with
    no JSON object (`"list-exit-0"`, `list:null`). Unwrapped items ignored. Document `{v, cmd, ok, state, pid:null,
    uptime_ms:null, last_error, list, instances}`, `list` the parsed document (an exit-21 error document included); exit
    0 ready, 1 otherwise, 2 unknown session.
  - New `harness::session`: `SessionOptions` (default `overseer` + `builder`, no echo, `recorded`), `harness_session(test,
    label, opts) -> HarnessSession` (library `boot`, `build:false`, `stamp:true`, `bin_dir_from_exe(current_exe)`,
    session id `<test>-<label>-<pid>`; panics with the boot document unless ready), `home()` · `instance_dir(name)` ·
    `bin_dir()` · `session()` · `workspace()` · `cleanup(self) -> Outcome`; `Drop` runs `cleanup` (home kept) unless
    already run, then removes the home parent unless `keep_home(KEEP_HOMES, KEEP_FAILED, panicking)` = `KEEP_HOMES=1` or
    (`KEEP_FAILED=1` and panicking). The spawned `supervise` process stays (fork 2, `inputs#I1`). A plain function and
    guard, never an rstest fixture; never shells out to `agent-run`.
  - `viola-harness` CLI: `run --e2e`; `boot --statusline-echo` (clap `conflicts_with = "unstamped"`, misuse = usage
    `arguments`) and `boot --agents-mode recorded|oversize|malformed` (an unknown value = usage `arguments`); `boot_options`
    takes a `BootFlags` struct.
  - `harness::cleanup` test `remove_session_home_removes_a_boot_created_parent` prepares its base through
    `ws.ensure_e2e_home()` (was `fs::create_dir_all(ws.e2e_home())`, which reads `AlreadyExists` over a dangling link —
    CARRY 5).
  - Root test support: new `tests/support/start.rs` (`role_lines`, `child_starts`, `run_refused`, `refused_with`,
    `endpoint_of`, `last_plugin_dir`, moved from `tests/cli_instance_state.rs`, whose cases are unchanged and import them).
  - New test binaries: root `tests/path1_run_start.rs` (`path1_run_start_one_home_in_order(stamped_home)`, Path 1 steps
    1–6 in one flow over one home; `[[test]]` `required-features = ["fake-agent"]`); `crates/viola-e2e/tests/
    path6_budget_harness.rs` (`path6_two_instance_release_over_a_harness_session`, Path 6 steps 1 and 7 over
    `harness_session` with `--statusline-echo`).
  - New / changed cases: `nextest.rs` `e2e_layer_is_the_e2e_prefix_regex`, `e2e_layer_is_excluded_from_the_integration_layer`;
    `run.rs` `e2e_layer_joins_the_default_and_all_selections`, `e2e_layer_named_alone_selects_only_itself`,
    `e2e_layer_runs_third_as_suite_nextest_e2e` (stand-in runner), and `with_an_e2e_binary` planting `tests/path_it.rs` in
    `run_reports_unit_integration_and_doctest_suites` / `run_with_a_filter_matching_nothing_is_red` (nextest refuses a
    regex matching no binary in the throwaway workspace); `boot.rs` `boot_flags_agents_mode_reads_its_kebab_value`,
    `boot_flags_statusline_echo_command_names_the_fake_agent_and_the_marker`; `viola-harness.rs`
    `boot_flags_agents_mode_parses_and_defaults_to_recorded`, `boot_flags_statusline_echo_parses`,
    `boot_flags_statusline_echo_with_unstamped_is_refused`, `run_e2e_is_a_flag_of_its_own`; `status.rs`
    `status_derives_each_state_from_the_list_read`, `status_derives_ready_for_no_booted_instance`,
    `status_derives_down_with_no_session_bin_to_run`; `session.rs` `keep_home_is_keep_homes_or_a_failed_test_under_keep_failed`,
    `session_options_default_to_the_pair_recorded_without_the_echo`; `harness_lifecycle.rs` the lifecycle case reads the
    nested `list` (each booted name `wrapped:true, liveness:"live"`, STATUS `idle`|`busy`), the dead-wrapper case reads
    `degraded` from the absent item, new `harness_session_with_a_malformed_agents_mode_reads_ready_and_unknown`; `cli.rs`
    the `status` case asserts the `list` member.
- **Crates / modules:** `viola-e2e` gains `harness::session`; root test support gains `support::start`. No crate added.
- **Dependencies:** none (no manifest dependency line; `Cargo.toml` gains only the `[[test]]` table).
- **Schema / config:** `.config/nextest.toml` `[profile.ci]` verify-driven 20 s override gains `binary(path1_run_start)`;
  `session.json` / `supervise.json` gain `agents_mode` (serde default, no `deny_unknown_fields`); `last_error` gains
  `list-exit-none` (harness closed value). No viola format, schema file or config key changed.
- **Spec-master edits:** none (implement authors none; this wrap's P2 applies them).
- **Counts / qualifiers moved:**
  - The E2E layer: 18 binaries / 155 tests → 20 / 157 (`run --e2e`, P2 block run 2, `10.2.log`: `Starting 157 tests
    across 20 binaries`; 18/155 is research's measurement, now `Starting 155 tests across 18 binaries` in block run 1's
    `10.log`); the re-cut integration layer selects no E2E-prefixed binary (`binary(path1_run_start)` → 0 tests,
    `nextest-exit-4`, `12.2.log`). Patterns `e2e-regex`, `e2e-selector`: test-plan's key file
    `5-command-implementation.md:21` states "`kind(test)` until the first E2E-prefixed binary exists" (stale) and
    `:22` the regex (to widen); `test-plan.md:1023-1024` the E2E row (standing).
  - Sweep reach: 7 patterns · 9 moved facts, 7 with a pattern (the report's own count: the two left are the walls and
    the watch tally, measurements with no master site) · owner pattern 4 standing · by line 3 lines · 3 offsets
    (`cascade.py sweep --pre-pass`, trail `cascade-2026-10-17-end-to-end-path-binary-and-harness-legs-pre-pass.json`;
    `carry5-keeper` and `owner-generic` refused `control UNFIRED over the seven masters at ae213762` — their zero is
    that refusal).
- **Dev-tool versions:** none — cargo-nextest re-read at 0.9.146 (`cargo nextest --version`, dev host).
- **Harness / gate surface:** `run --e2e` (suite `nextest-e2e`, in the default and `--all`); the integration layer's
  exclusion; `boot --statusline-echo` / `--agents-mode`; `status` running `viola list` with the derivation above;
  `harness_session`. `gate::SUITES` already held `nextest-e2e`; no `ci.yml` edit (CI's `run --coverage` runs every
  binary). The obs side: `status`'s `viola list` carries no `VIOLA_NAME`, so it writes no role line (obs-plan §3
  `cli-<name>` only under a name); no event, field or log format changed.
- **Cross-project / external claims:**
  - CI: `ci#38992234808` on `ae2137622b3a` — `verdict: green · checks 15/15 · wall 549 s`
    (`evidence/operator-pass.md`); the final HEAD's run.
  - `I1 · message · copy message · n/a` — the P4 fork answers (fork 1: Path 1 as root `tests/path1_run_start.rs`; fork 2:
    the spawned `supervise`), the overseer as the founder's delegate, PROVISIONAL until the founder's Epoch 5 batch;
    cited by scope / research / plan.
  - `I2 · message · copy message · n/a` — cited here: the Viola overseer (technical, founder-delegated), implement P2,
    2026-10-17: "numbered path binaries name the test plan's numbered Paths; the widened pattern matches no other binary
    today. The wrap amends test-plan §3's regex and the testing.md / a11y copies to the same form." PROVISIONAL as a
    delegate's word on a technical fork.
  - `inputs.py verify`: `inputs: 2 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable
    0 · n/a 2 · uncited 1 · unparsed 0` (the uncited one is I2, cited by this bullet). No P5 review card is named by
    an inputs entry (the phase's `p5-card-1.md` `## Beyond the plan` reads `none`).
- **Reverted / negative API facts:** the `agents-mode` write first sat right after the `claude` copy (plan step 3);
  moved after the stamp (above). No surface removed.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - test-plan §3's E2E regex `^(path|tui|mcp|cross|chaos|contract)_` (`5-command-implementation.md:21-22`) does not
    select the plan's numbered Path binaries: block run 1 read `run --e2e` at 18 binaries and `run --integration
    --filter 'binary(path1_run_start)'` ran `path1_run_start_one_home_in_order` (`12.log`, 1 passed). Disposition: the
    regex widened in the harness on `inputs#I2`; the master and its copies to amend (pattern `e2e-regex`: test-plan
    key file :21, :22; `test-plan.md:398` `tests/{path,mcp,cross}_<topic>.rs`; leaves `testing.md:20`, `:45`,
    `commands.md:37`, `:38`).
  - test-plan §3's "a `binary()` regex that matches no binary is a nextest filterset parse error" (key :21, measured on
    0.9.133) still holds on 0.9.146 (scratch crate, exit 94 "operator didn't match any binary names"); not disproved —
    its version qualifier moves (pattern `e2e-regex`, key :21).
  - Plan step 3's "start writes `<session>/bin/agents-mode` right after the `claude` copy": measured `verify-failed`
    under `malformed`; the write follows the stamp. The master `5-command-implementation.md:4` (`boot`'s flags, pattern
    `boot-flags`) to state the order.
- **Expected amendments (from plan):**
  - test-plan §3 → 5-command `run` step 1 (`kind(test) & !binary(…)`, the 18-binary measurement; "`--e2e` selects nothing
    yet" and the usage-error clause retire) — carried (Symbols `nextest`, Counts); search `e2e-selector` 13 standing
    (test-plan 3 + key file `5-command-implementation.md` :18, :21, :45, :53 ×2, :113; a11y-plan 4 + key 1),
    `e2e-regex` 3 standing (test-plan :398, key :21, :22). Now 20 binaries with the widened regex.
  - test-plan §3 → `boot`'s "not built" clauses for `--statusline-echo` / `--agents-mode` retire, the conflict with
    `--unstamped` — carried; search `boot-flags` 5 standing (architecture :299, test-plan :742, :927, :954, key :4 ×9).
  - test-plan §3 → the `harness_session` sentence becomes the spawned `supervise` (fork 2, `inputs#I1`, provisional) —
    carried; search `harness-session` 4 standing (test-plan :748, key :9, :26, :28).
  - test-plan §3 → `status` reads the bin dir from `supervise.json` and runs `viola list` with `VIOLA_*` removed —
    carried; search `status-list` 9 standing (test-plan :167; key `5-command-implementation.md` :65, :136;
    `status-endpoint-shape.md` :1, :3, :11, :17; `bootstrap-phases-…md:26`; `test-plan-contracts.toml:12`).
  - test-plan §3 → Status endpoint shape: the interim paragraph retires, an absent booted instance reads `degraded`,
    `last_error` adds `list-exit-none` — carried; search `status-list` (as above; `status-endpoint-shape.md:17` the
    interim, `:11` the `last_error` example).
  - test-plan §3 → Closed enums: `list-exit-none` — carried; search `status-list` (key `5-command-implementation.md:136`
    lists `list-exit-<code>`).
  - test-plan §6 Path 1: as landed, `tests/path1_run_start.rs`, a root path binary (fork 1, provisional) — carried;
    search `owner-title` 4 standing (test-plan :742, :764 @c1366, key `5-command-implementation.md:4` ×2,
    `status-endpoint-shape.md:17`) and `path-binaries` 1 standing (key `5-command-implementation.md:27` @c157/@c594, the
    `viola_e2e::fixtures` copy "lands with its first consumer").
  - test-plan §6 Path 6: the harness-session form and step 7's two-instance leg as
    `crates/viola-e2e/tests/path6_budget_harness.rs` — carried; search `owner-title` (test-plan :742, :764) and
    `boot-flags` (test-plan :742).
  - test-plan §3 `run` step 2 / §5: the E2E layer includes root E2E-prefixed binaries (testing.md's E2E location line a
    leaf) — carried; search `e2e-regex` (test-plan :398; leaf `testing.md:20`).
  - a11y-plan §3 / §9: the `tui_` / `chaos_` / `cross_` cases report in `nextest-e2e` locally — carried; search
    `e2e-selector` (a11y-plan :102, :105, :323, :340; key `a11y-testing-tool-pick.md:4`).
  - Added by `inputs#I2` (owed by the operator's invocation): test-plan §3's regex and its testing.md / a11y copies to
    `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` — carried (Symbols `nextest`); search `e2e-regex` (no a11y-plan site
    holds the regex text — its sites name the suites, `e2e-selector`).
- **Coverage of new surfaces:**
  - `run --e2e` → validation clap✓ · instrumentation n/a (harness) · PII n/a · tests unit (`e2e_layer_*`) + gate entry
    `run --e2e` · a11y n/a · tokens n/a
  - `boot --statusline-echo` / `--agents-mode` → validation clap value enum + conflict✓, shell-plain path check✓ ·
    instrumentation n/a · PII n/a · tests unit (`boot_flags_*`) / integ (lifecycle malformed case, Path 6 leg) / smoke ·
    a11y n/a · tokens n/a
  - `status` over `viola list` → validation `take(MAX_FRAME + 1)` + object check✓ · instrumentation n/a (no role line:
    no `VIOLA_NAME`) · PII n/a (codes only in `last_error`) · tests unit (`status_derives_*`) / integ (lifecycle, cli) /
    smoke · a11y n/a · tokens n/a
  - `harness_session` → validation n/a · instrumentation n/a · PII n/a · tests e2e (Path 6 leg) · a11y n/a · tokens n/a

## Deviations from intent
1. The E2E regex is `^(path[0-9]*|…)_`, not test-plan §3's `^(path|…)_` — plan↔spec conflict on the plan's binary
   names; `inputs#I2` chose widening over renaming (the plan's names and entries stand).
2. The `agents-mode` file is written after the stamp, not right after the `claude` copy (plan step 3): the stamp's
   agents read goes through the same copy.
3. The harness's throwaway-workspace cases plant one E2E binary (`tests/path_it.rs`): the re-cut layer cannot parse
   there otherwise (nextest 0.9.146, exit 94, scratch-crate measurement).
4. `harness_session`'s keep rule (plan step 7 read two ways): keep = `KEEP_HOMES=1` or (`KEEP_FAILED=1` and panicking).
5. `status` spells `viola --home <home> list --json` (plan: `list --json --home <home>`), as boot's stamp does.
6. `last_plugin_dir` moved to `tests/support/start.rs` with the four named helpers (the flow needs it).
7. `run --e2e` alone runs no doctest.
8. Cases beyond the plan's names (listed in Symbols); the Path 6 leg compares instants as text (`viola-e2e` has no
   chrono; viola's one RFC 3339 ms form sorts as time).
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 18 · listed 18 · recorded 0`, implement
  P4 and wrap P1).

## Decisions & corrections
- The overseer's word on the E2E prefix (`inputs#I2`, technical, founder-delegated, 2026-10-17): widen the regex; the wrap
  amends test-plan §3 and the testing.md / a11y copies.
- Sweep hazard: a plan naming a numbered `path<N>_` binary against the E2E regex `^(path|…)_` — a name that reads as
  E2E-prefixed by eye matches no `path_` (the digit precedes the underscore); measured only by the layer's own selection
  count (block run 1: 18 binaries, and `run --integration` ran `path1`).
- A harness self-test over a throwaway mini workspace cannot carry a `binary()` regex filterset unless the workspace holds
  a matching binary (nextest 0.9.146 exit 94 "operator didn't match any binary names", scratch crate) — a layer re-cut
  reaches the harness's own real-nextest cases.
- A boot step that writes into the fake agent's exe dir runs after `viola verify`'s stamp when it changes what the stamp's
  probes read (the `agents-mode` file and Run B's `claude agents --json` read; `verify-failed` measured).
- The operator's invocation rules for implement: no live `claude` start (none made); each tool run bare and read as
  printed; each long run journalled in the run dir before the wait (`.andromeda/runs/2026-10-17T01-21-28-implement/journal.md`);
  the watched `cli_send` case recorded each time, never re-run for green; the operator pass with the `ci.py conclusion`
  read — all honoured.
- The operator's wrap invocation: the 85 % / 90 % context alarms are expected; stop only at 95 %; stop after P1 if P2
  will not fit.
- `evolve.py append` refused one implement batch (`REFUSED ts: … later than the clock`): a `ts` hand-typed ahead of the
  read clock; re-fired with the `date -u` reading.
- Journal stamps were twice written before the clock was read and corrected after (01:33Z → 01:32Z; `01:3x/4x/5x`
  placeholders) — a journal line written "before the wait" takes the clock read in the same call.
- A `pgrep`-free census: `comm` truncates at 15 chars (`viola-fake-agen`), so the census reads `/proc/*/exe` against the
  repository root (host-linux.md 2026-10-07 rule held).

## Outcome
- (tests) `v1-02` — on the three CI OSes the shims drive a fake-agent session through boot · run · status · cleanup ·
  logs: CI `ci#38992234808` green on `ae2137622b3a` (`test (macos-latest)`, `test (ubuntu-latest)`, `test
  (windows-2025)`, the ci-smoke lifecycle `boot → status → logs → cleanup ×2`); locally `status` exits 0 `state:"ready"`
  with `list` the `viola list --json` document naming both booted instances `wrapped:true, liveness:"live"`
  (`15.2.log`), `cleanup` `processes_gone:true, endpoint_gone:true` (`18.2.log`); `cli.rs` / `harness_lifecycle.rs`
  under `run --coverage` (pre-push). MET.
- (tests) `run --e2e` reports `nextest-e2e` passed 157, failed 0; `run --integration --filter 'binary(path1_run_start)'`
  selects nothing (`nextest-exit-4`); every root binary without an E2E prefix stays in `nextest-integration` (the
  layers are complements by construction). MET, under the widened regex (`inputs#I2`).
- (tests) `boot --statusline-echo` plants `<home>/statusline-source.json` naming `viola-fake-agent statusline-echo`
  (0600 Unix, asserted in the Path 6 leg) before the supervisor; `--agents-mode` writes `<session>/bin/agents-mode` and
  records `agents_mode` in `session.json` / `supervise.json` (lifecycle malformed case); `--statusline-echo --unstamped`
  is refused (`boot_flags_statusline_echo_with_unstamped_is_refused`, clap → usage `arguments`). MET.
- (tests) `status` nests `viola list --json`; `ready` / `degraded` / `down` + `list-exit-<code>` per the derivation
  table; a `malformed` mode leaves it `ready` with STATUS `unknown`. MET (`oversize` covered by the derivation and the
  security rule, not by a booted case).
- (tests) Path 6 step 7 over `harness_session` with the spawned `supervise`: `.ok.budget.paused` stays true, `builder`
  `budget_paused:false`, `overseer` true; one `budget-gate{paused:false, override_until}` for `builder` alone, equal to
  the snapshot's and later than its `ts`; `send` 0 to `builder`, 11 `five-hour` to `overseer`; on Unix the marker is
  written and stdout is `viola-fake-statusline\n` (Windows: `cfg(not(unix))` branch, empty stdout, no marker — run on
  `windows-2025` in CI). MET.
- (tests) Path 1 steps 1–6 as one ordered flow over one home in `tests/path1_run_start.rs`, the `settings.json` half
  Unix's. MET.
- (design) the human `list` read shows `overseer`'s WHEEL cell `driver - budget-paused`, `builder`'s without it; no ESC
  byte in any `--json` read. MET.
- (security) CARRY 5 through `Workspace::ensure_e2e_home`, red-before recorded (`evidence/carry5-dangling-link.md`). MET.
- (security) the leg's `release` runs with no `VIOLA_NAME` and exits 0; no harness or leg verb carries the builder's
  `VIOLA_*`. MET.
- (obs) G2 (`g2: clean`) and G4 (`schema-check` 11 files, 0 failures) pass over the booted home; `status`'s `viola list`
  writes no role line (no `VIOLA_NAME`). MET.
- (arch) no product source changed (guard green); no new env var read; `--agents-mode` reaches the fake agent only as the
  file beside `bin/claude[.exe]`. MET.
- (a11y) every a11y-named case reports in exactly one of `nextest-integration` / `nextest-e2e` (complement layers) and in
  CI's `coverage` suite. MET.
- Gates — the light gate's subject, recorded at implement P2 block run 2 (trail
  `.andromeda/runs/2026-10-17T01-21-28-implement/gate-2026-10-17-end-to-end-path-binary-and-harness-legs.json`):
  `cargo fmt --all --check` green · `cargo clippy … --features fake-agent -- -D warnings` green · the `target/wincheck`
  Windows clippy green · the G1 `rg …instrument…` green (exit 1, no output) · `bash scripts/lint-probes.sh` green ·
  `run --unit` green · `run --unit --filter 'test(/e2e_layer_|status_derives_|boot_flags_/)'` green · the
  no-product-change guard green · `run --integration --filter 'binary(harness_lifecycle) | …'` green · `run --e2e`
  green · `run --e2e --filter 'binary(path1_run_start) | binary(path6_budget_harness)'` green · `run --integration
  --filter 'binary(path1_run_start)'` green (exit 1, `nextest-exit-4`) · `cleanup --session p-e2e-smoke` green ·
  `boot --session p-e2e-smoke --statusline-echo --agents-mode recorded` green · `status --session p-e2e-smoke` green ·
  `bash scripts/g2-zero-panics.sh` green · `schema-check` green · `cleanup --session p-e2e-smoke` green · `pre-push`
  green (and re-run green on the final tree in the operator pass) · `gate.py hygiene` (operator) `hygiene: clean` ·
  `git … push origin HEAD` (operator) green, `history moved: … 6042af63→ae213762` · `ci.py conclusion --sha HEAD --wait
  1800` (operator) `verdict: green · checks 15/15` — operator results in `evidence/operator-pass.md`. No `defer`; no gate
  deferred (rust delta 17 files).
- Smoke: the plan's `role = 'smoke'` entries ran as P2 gates (boot ready, status ready, cleanup gone).
- Watches: `viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` · 3 green run(s) [implement
  block run 1 pre-push `19.log` L424, block run 2 `19.2.log` L423, operator-pass re-run `19.3.log` L424] · and CI
  `ci#38992234808` green on all three `test` jobs (the case's per-runner line not read) · `evidence/watch-cli-send.md`.
- Outcome basis: the operator pass ran — commit list `ae21376` (basis `6042af6`), the final HEAD's CI run
  `ci#38992234808` in `evidence/operator-pass.md`; implement's report held in this window (and persisted,
  `.andromeda/runs/2026-10-17T01-21-28-implement/outcome-2026-10-17-end-to-end-path-binary-and-harness-legs.md`).
- Process hygiene: implement's census — every session and nextest run it started terminated; no process whose executable
  lies under this repository was running at 2026-10-17T01:43Z, and none at 01:58:58Z (re-measured here by `/proc/*/exe`).
  Left on disk for the operator: `target/carry5-witness/` and the scratch export under the session scratchpad.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 6042af63 (the parent of the oldest pre-CI commit ae213762) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 24
### Cargo.toml — added 5 line(s) in 1 range(s)
added: 131-135
### crates/viola-e2e/src/bin/viola-harness.rs — added 81 line(s) in 13 range(s)
added: 9 · 40-45 · 90-91 · 132-139 · 146 · 162 · 164-166 · 170-171 · 181 · 251-261 · 348-349 · 354-359 · 365-401
- 132-138 @133 «struct BootFlags {»
  - 365-381 @366 «fn boot_flags_agents_mode_parses_and_defaults_to_recorded() {»
  - 383-388 @384 «fn boot_flags_statusline_echo_parses() {»
  - 390-394 @391 «fn boot_flags_statusline_echo_with_unstamped_is_refused() {»
  - 396-400 @397 «fn run_e2e_is_a_flag_of_its_own() {»
### crates/viola-e2e/src/harness/boot.rs — added 126 line(s) in 14 range(s)
added: 5-7 · 18 · 47-70 · 81-82 · 95-98 · 160-162 · 175-183 · 193 · 216 · 230-259 · 843-844 · 851-892 · 912-913
       933-934
- 47-56 @51 «pub enum AgentsMode {»
- 58-66 «impl AgentsMode {»
  - 59-65 «pub fn as_str(self) -> &'static str {»
  - 175-177 «if opts.statusline_echo {»
  - 178-182 «replace_private(»
- 230-242 @233 «pub fn statusline_echo_command(bin_dir: &Path, home: &Path) -> Option<String> {»
  - 237-240 «let plain = |word: &str| {»
- 244-258 @246 «fn plant_statusline_echo(bin_dir: &Path, home: &Path) -> Result<(), HarnessError> {»
  - 248-249 «let command =»
  - 252-256 «replace_private(»
  - 851-868 @852 «fn boot_flags_agents_mode_reads_its_kebab_value() {»
  - 870-891 @871 «fn boot_flags_statusline_echo_command_names_the_fake_agent_and_the_marker() {»
### crates/viola-e2e/src/harness/cleanup.rs — added 4 line(s) in 4 range(s)
added: 179 · 278 · 324 · 365
### crates/viola-e2e/src/harness/mod.rs — added 3 line(s) in 2 range(s)
added: 9 · 262-263
### crates/viola-e2e/src/harness/run.rs — added 93 line(s) in 11 range(s)
added: 1 · 39-40 · 54 · 63 · 267-275 · 770 · 815 · 819-883 · 928 · 941-950 · 970
  - 819-824 @820 «fn e2e_layer_joins_the_default_and_all_selections() {»
  - 826-833 @827 «fn e2e_layer_named_alone_selects_only_itself() {»
  - 835-882 @838 «fn e2e_layer_runs_third_as_suite_nextest_e2e() {»
  - 941-949 @943 «fn with_an_e2e_binary(ws: &Workspace) {»
### crates/viola-e2e/src/harness/run/nextest.rs — added 19 line(s) in 2 range(s)
added: 10-15 · 138-150
- 10-14 @13 «pub(super) const INTEGRATION: &str =»
- 138-141 @139 «fn e2e_layer_is_the_e2e_prefix_regex() {»
- 143-149 @144 «fn e2e_layer_is_excluded_from_the_integration_layer() {»
  - 145-148 «assert_eq!(»
### crates/viola-e2e/src/harness/run/perf.rs — added 5 line(s) in 2 range(s)
added: 12-14 · 181-182
- 12-14 «use super::super::boot::{»
### crates/viola-e2e/src/harness/session.rs — new file · 152 line(s)
- 12-18 @14 «pub struct SessionOptions {»
- 20-28 «impl Default for SessionOptions {»
  - 21-27 «fn default() -> Self {»
- 30-34 @32 «pub fn keep_home(keep_homes: bool, keep_failed: bool, panicking: bool) -> bool {»
- 36-44 @38 «pub struct HarnessSession {»
- 46-77 @47 «pub fn harness_session(test: &str, label: &str, opts: SessionOptions) -> HarnessSession {»
  - 51-55 «let instances = opts»
  - 56-66 «let booted = boot(&BootOptions {»
  - 68-69 «let record: super::SessionRecord =»
  - 70-76 «HarnessSession {»
- 79-105 «impl HarnessSession {»
  - 80-82 «pub fn session(&self) -> &str {»
  - 84-86 «pub fn workspace(&self) -> &Workspace {»
  - 88-90 «pub fn home(&self) -> &Path {»
  - 92-94 «pub fn instance_dir(&self, name: &str) -> PathBuf {»
  - 96-98 «pub fn bin_dir(&self) -> &Path {»
  - 100-104 @101 «pub fn cleanup(mut self) -> Outcome {»
- 107-122 «impl Drop for HarnessSession {»
  - 108-121 «fn drop(&mut self) {»
- 124-152 @125 «mod tests {»
  - 128-143 @129 «fn keep_home_is_keep_homes_or_a_failed_test_under_keep_failed() {»
  - 145-151 @146 «fn session_options_default_to_the_pair_recorded_without_the_echo() {»
### crates/viola-e2e/src/harness/status.rs — added 194 line(s) in 7 range(s)
added: 1-7 · 10 · 12-80 · 87-96 · 104 · 107-109 · 115-217
- 15-20 @17 «pub struct Derived {»
- 22-49 @25 «pub fn derive(code: Option<i32>, list: Option<&Value>, booted: &[String]) -> Derived {»
  - 26-29 «let down = |code: &str| Derived {»
  - 30-34 «let doc = match (code, list) {»
  - 36-40 «let live = |name: &String| {»
  - 41-48 «Derived {»
- 51-80 @53 «fn read_list(viola: &Path, home: &Path, session_bin: &Path) -> (Option<i32>, Option<Value>) {»
  - 54-65 «let spawned = Command::new(viola)»
  - 66-68 «let Ok(mut child) = spawned else {»
  - 70-72 «if let Some(out) = child.stdout.take() {»
  - 75-78 «let doc = within»
  - 87-94 «let (code, list) = match load_supervise_spec(&ws.session_dir(session)) {»
- 116-217 @117 «mod tests {»
  - 120-122 «fn names(list: &[&str]) -> Vec<String> {»
  - 124-126 «fn item(name: &str, wrapped: bool, liveness: &str) -> Value {»
  - 128-130 «fn board(items: &[Value]) -> Value {»
  - 132-137 «fn ready() -> Derived {»
  - 139-144 «fn degraded() -> Derived {»
  - 146-151 «fn down(code: &str) -> Derived {»
  - 153-204 @154 «fn status_derives_each_state_from_the_list_read() {»
  - 206-209 @207 «fn status_derives_ready_for_no_booted_instance() {»
  - 211-216 @212 «fn status_derives_down_with_no_session_bin_to_run() {»
### crates/viola-e2e/tests/cli.rs — added 15 line(s) in 3 range(s)
added: 7 · 175-176 · 192-203
  - 195-197 «let items = doc["list"]["ok"]["items"]»
  - 198-203 «assert!(»
### crates/viola-e2e/tests/harness_lifecycle.rs — added 59 line(s) in 6 range(s)
added: 7 · 27-28 · 32-38 · 120-129 · 196-201 · 208-240
- 32-37 @33 «fn listed<'a>(st: &'a serde_json::Value, name: &str) -> Option<&'a serde_json::Value> {»
  - 34-36 «st["list"]["ok"]["items"]»
  - 121-129 «for name in ["builder", "overseer"] {»
  - 198-201 «assert!(»
- 208-239 @211 «fn harness_session_with_a_malformed_agents_mode_reads_ready_and_unknown() {»
  - 213-216 «let opts = BootOptions {»
  - 225-228 «for file in ["session.json", "supervise.json"] {»
### crates/viola-e2e/tests/path6_budget_harness.rs — new file · 283 line(s)
- 24-25 «const REFUSED_FIVE_HOUR: &str =»
- 28-32 «struct Ran {»
- 34-74 @36 «fn viola(s: &HarnessSession, args: &[&str], stdin: Option<&[u8]>, env: &[(&str, OsString)]) -> Ran {»
  - 38-41 «let mut command = Command::new(»
  - 42-56 «command»
  - 57-59 «for (key, value) in env {»
  - 61-67 «if let Some(bytes) = stdin {»
  - 69-73 «Ran {»
- 76-88 @77 «fn usage(five_hour: u64, seven_day: u64) -> Vec<u8> {»
  - 78-85 «json!({»
- 90-93 «fn send(s: &HarnessSession, name: &str) -> Ran {»
- 95-98 «fn json_doc(ran: &Ran) -> Value {»
- 100-106 «fn events(instance_dir: &Path) -> Vec<Value> {»
  - 102-105 «text.split_inclusive('\n')»
- 108-114 @109 «fn overrides(instance_dir: &Path) -> Vec<Value> {»
  - 110-113 «events(instance_dir)»
- 116-121 «fn item<'a>(board: &'a Value, name: &str) -> &'a Value {»
  - 117-120 «board["ok"]["items"]»
- 123-128 @124 «fn stored_instant(value: &Value) -> &str {»
- 130-283 @131 «fn path6_two_instance_release_over_a_harness_session() {»
  - 132-139 «let s = harness_session(»
  - 149-152 «assert!(»
  - 153-161 @154 «{»
  - 163-167 @164 «let feed_env = [»
  - 172-177 @173 «{»
  - 178-183 @180 «{»
  - 185-189 «assert_eq!(»
  - 191-196 @192 «for name in ["builder", "overseer"] {»
  - 198-204 @199 «let lifted = viola(»
  - 218-221 «assert_eq!(»
  - 224-227 «assert_eq!(»
  - 228-230 «assert!(»
  - 237-244 «let row = |name: &str| {»
  - 245-249 «assert!(»
  - 250-254 «assert!(»
  - 268-272 «for name in ["builder", "overseer"] {»
  - 278-282 «assert!(»
### tests/cli_instance_state.rs — added 5 line(s) in 3 range(s)
added: 11 · 17 · 24-26
- 24-26 «use support::start::{»
### tests/path1_run_start.rs — new file · 190 line(s)
- 23-24 «const SQUATTED: &str = "unable: the endpoint for builder is held by another process\n\»
- 25-26 «const TAMPERED: &str = "unable: the pinned viola copy failed its integrity check\n\»
- 29-43 @30 «fn commands(value: &Value, found: &mut Vec<String>) {»
  - 31-42 «match value {»
- 45-190 @46 «fn path1_run_start_one_home_in_order(stamped_home: StampedHome) {»
  - 53-57 «let kinds: Vec<(&Value, &Value)> = records»
  - 58-65 «assert_eq!(»
  - 68-71 «let started_at = of_kind(&receipt, "start")[0]["started_at"]»
  - 73-76 «assert!(»
  - 80-85 «for key in ["pinned_bin", "pid", "started_at", "child_pid", "endpoint"] {»
  - 86-89 «let pinned = snapshot["pinned_bin"]»
  - 93-96 «let item = board["ok"]["items"]»
  - 107-110 «assert_eq!(»
  - 137-141 @138 «fs::write(»
  - 155-158 «assert_eq!(»
  - 162-165 «commands(»
  - 168-170 «for command in &found {»
  - 171-185 @172 «{»
### tests/support/mod.rs — added 1 line(s) in 1 range(s)
added: 14
### tests/support/start.rs — new file · 68 line(s)
- 15-18 @16 «pub fn role_lines(home: &Path) -> Vec<Value> {»
- 20-26 @21 «pub fn child_starts(home: &Path) -> usize {»
  - 22-25 «role_lines(home)»
- 28-39 @30 «pub fn run_refused(home: &Path) -> Output {»
  - 31-36 «let piped = Piped::spawn(»
- 41-50 @42 «pub fn refused_with(home: &Path, detail: &str) -> bool {»
  - 43-49 «role_lines(home).iter().any(|l| {»
- 52-57 @53 «pub fn endpoint_of(home: &Path) -> String {»
- 59-68 @60 «pub fn last_plugin_dir(wrapper: &Wrapper) -> PathBuf {»
  - 62-66 «let start = of_kind(&receipt, "start")»
