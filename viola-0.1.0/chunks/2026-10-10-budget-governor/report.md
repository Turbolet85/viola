# Report — 2026-10-10-budget-governor

**Chunk:** Budget governor — five-hour/seven-day thresholds, budget-paused on automated send only, lift at reset
or per-instance override, unknown with age, release --budget payload
**Date:** 2026-10-10
**Commits:** `c85961d` `chore(2026-10-10-budget-governor): operator pre-CI commit, for the run this chunk's verdict
reads` (the one commit since the last wrap's `396c108`; `git log --format='%h %s' 396c108..HEAD`). Every diff
reading below is `396c108` (the parent of that pre-CI commit) against the work tree.

## Changes (structured — detectors read this)

- **Files:** 31 (`gate.py scope`: changed 31, listed 31). Coordinates of added text are copied from the last
  section, `## New text, by line`.
  - New, 3: `src/run/budget.rs` (998 lines) · `tests/cli_budget.rs` (503) · `tests/support/statusline.rs` (89).
  - Modified, product, 20: `crates/viola-core/src/lib.rs` · `crates/viola-state/src/budget.rs` ·
    `crates/viola-state/src/snapshot.rs` · `crates/viola-state/src/liveness.rs` (a test literal) ·
    `crates/viola-state/src/strict.rs` (a test literal) · `crates/viola-state/src/replay.rs` (a test literal) ·
    `src/run/mod.rs` · `src/run/send.rs` · `src/run/wheel.rs` · `src/run/dialog.rs` (a test literal) ·
    `src/run/snapshot.rs` (a test literal) · `src/cmd/run.rs` · `src/cmd/release.rs` · `src/cmd/pause.rs` ·
    `src/cmd/send.rs` · `src/cmd/revive.rs` · `src/cmd/hook.rs` (a test literal) · `src/cmd/hook/statusline.rs` ·
    `src/human.rs` · `src/obs.rs`. The six marked "a test literal" gained only the new snapshot field inside a
    `#[cfg(test)]` module.
  - Modified, tests and config, 8: `crates/viola-state/tests/state_replay.rs` ·
    `crates/viola-state/tests/state_events.rs` · `tests/cli_revive.rs` · `tests/hook_statusline.rs` ·
    `tests/cli_controls_not_disableable.rs` · `tests/support/mod.rs` · `Cargo.toml` · `.config/nextest.toml`.
  - Untouched, by the plan's preservation guard (`git diff --quiet 396c108ba64c -- …` over 26 paths, exit 0):
    `crates/viola-agent-claude`, `crates/viola-channel`, `crates/viola-pty`, `crates/viola-e2e`, `schemas`,
    `plugin`, `fixtures`, `fuzz`, `scripts`, `e2e-web`, `.github`, both deny files, `rust-toolchain.toml`,
    `Cargo.lock`, `src/main.rs`, the fake agent, `src/cmd/verify*`, `src/cmd/hook/seam.rs`, `src/cmd/answer.rs`,
    `wait.rs`, `last.rs`, `client.rs`, `src/run/version_gate.rs`, `src/run/gate.rs`.

- **Symbols / APIs:**
  - **`viola-core`** (`crates/viola-core/src/lib.rs`): `pub enum BudgetWindowKind { FiveHour, SevenDay }`
    (202-209), serde kebab-case, `as_str` → `five-hour` · `seven-day` (211-218); a third word does not
    deserialize. `pub struct Percent(u8)`, a `nutype` newtype with `validate(less_or_equal = 100)`, built only
    through `Percent::try_new` (in the range 202-227). No dependency added to the crate.
  - **`viola-state`**: `pub fn read_budget(home: &Path) -> Option<BudgetReading>`
    (`crates/viola-state/src/budget.rs` 72-97). In order: `strict::check_path(home)`, `check_path(budget.json)`,
    a shared lock on `budget.json.lock` (`open_private_lock`), `File::open` through `take(MAX_FRAME + 1)`,
    `None` when the bytes pass `MAX_FRAME`, a tolerant parse (`Stored`, 64-70: an unknown key is skipped), `None`
    unless `v` is 1. Every failure is `None`. It writes nothing to `budget.json`; an absent `budget.json` under
    a home that passes its check returns `None` and creates no file (the lock sibling included). It is the
    file's first reader; `write_budget` is unchanged and `hook statusline` stays its only writer.
    `InstanceSnapshot` gains `pub budget_override_until: Option<String>` (`crates/viola-state/src/snapshot.rs`
    50-52, `#[serde(default, skip_serializing_if = "Option::is_none")]`); the envelope's `v` stays 1.
  - **The thresholds** (`src/run/budget.rs`): `Thresholds { five_hour: Percent, seven_day: Percent }` (33-38),
    defaults 90 and 85 (44-51). `read_thresholds(home) -> (Thresholds, Vec<ConfigRejection>)` (53-66): its own
    read of `<home>/config.json` through `take(MAX_FRAME)`. `parse_thresholds` (68-89): a document that is not
    an object, or whose `v` is not 1, gives both defaults and no rejection (that fault is
    `read_diagnostics_level`'s to report, once). A key's value must be a JSON whole number that fits `Percent`
    (`whole_percent`, 91-94: `as_u64` then `u8` then `Percent::try_new`), else that key takes its default and
    one `ConfigRejection::Malformed` is returned for it; the other key stands. So `101`, `-1`, `"90"`, `90.5`,
    `256`, `null` and `true` are each rejected (the seven rows of
    `thresholds_a_value_that_is_no_whole_percentage_takes_its_default`, 430-447), and two bad keys give two
    rejections. The thresholds are read once, at the wrapper's
    start; a changed `config.json` is seen at the next start.
  - **The decision** (`src/run/budget.rs`): `decide(reading, thresholds, override_until, now) -> Standing`
    (164-199), pure, the wall-clock instant an argument (`chrono::DateTime<Utc>`; no clock seam, no
    mock_instant). A window is crossed (`crossed`, 141-162) only when its `used_percentage` is known and at or
    above its threshold AND its `resets_at` is known, parses as RFC 3339 and is later than `now`. A window whose
    `resets_at` is `"unknown"`, unparseable or passed never pauses. An override whose end is later than `now`
    keeps the gate open whichever window is crossed; one whose end has passed is dropped. With both windows
    crossed the named one is `five-hour`. `Standing { paused: Option<Crossed>, override_until: Option<String> }`
    (104-110); `Crossed { window, used_percentage: f64, resets_at: String }` (96-102). `Standing::data()`
    (122-133) is the `budget-gate` `data`: `paused`, `window` only when paused, `override_until` only when an
    override stands, no other key.
  - **The gate** (`src/run/budget.rs`): `BudgetGate` (219-228), one per wrapper, its last recorded `Standing`
    under one `Mutex`. Four methods:
    `start(now)` (279-286) evaluates from `budget.json` alone, restores no earlier override, writes nothing
    and hands its verdict to the start;
    `evaluate(trigger, now) -> Result<Option<Crossed>, StateError>` (288-304) reads `budget.json`, decides over
    the standing override, and only when the triple (paused, window, override end) differs from the last
    recorded one appends ONE `budget-gate` event (`source: wrapper`, through `viola_state::events::append_event`)
    and then updates the snapshot's `budget_paused` and `budget_override_until` through `Snapshots::update`,
    both inline, under the gate's lock, before it returns; a record that cannot land is an error and the
    recorded state stays as it was;
    `lift()` (306-322) is `release --budget`: with a pause recorded it records
    `{paused:false, override_until:<the crossed window's resets_at>}` the same way and returns that end; with
    no pause recorded it records nothing and returns `None`; it reads no `budget.json`;
    `paused()` (252-255) is the last recorded state.
    A change of percentage alone, or a re-read of the same verdict, appends nothing. An expired override is
    dropped at the next evaluation point, which then appends either `{paused:false}` or
    `{paused:true, window}`. The gate starts no thread, holds no timer and never polls.
  - **The span**: `run.budget_eval`, declared once, on `BudgetGate::decide_at` (`src/run/budget.rs` 263-277;
    `grep -rn budget_eval src` → 1 declaration at `:260`, 4 test lines), `skip_all`, fields `trigger`
    (`start` · `send` · `turn-ended`), `paused`, and `window` recorded only when paused. Opened once per
    evaluation: at start, at a `send`'s arrival, at a `turn-ended`. `lift` opens none.
  - **The three evaluation points:** (1) `start_state` (`src/cmd/run.rs`, the added lines 536 and 545-546):
    the gate's `start` verdict is the first snapshot's `budget_paused` and start record two, whose `data` is
    `Standing::data()` in place of the constant `{"paused": false}`; the first snapshot's
    `budget_override_until` is always absent. `revive` starts through the same `start`. (2) `send`
    (`src/run/send.rs` 345-352): after the text refusals and the first `human-typing` read, before the
    running-turn read; evaluated once, never again after the readiness gate's wait. (3) `Methods::dispatch`
    (`src/cmd/run.rs` 120, 123-128): after `append_hook_event` has appended a line of kind `turn-ended`; no
    other hook kind evaluates. `append_hook_event`'s signature is unchanged.
  - **The refusal** (`src/run/send.rs`): `Ctx::budget_paused` (498-507) returns `{"refusal":"budget-paused",
    "detail":"five-hour"|"seven-day"}` through the one `send-refused` emitter (`refuse_with`), before
    `send-issued`, with no cursor; nothing is typed and no flight is reserved. Order as landed:
    control-character → empty-text → human-typing → budget-paused → turn-running → the readiness gate's wait.
    The wrapper's `send-refused` process-log line gains, on this refusal only, `window`, `used_percentage` (a
    JSON number), `resets_at` and `paused: true`; no other refusal carries them. A gate change that cannot be
    recorded at a `send` answers `-32603`. `SendSlot` holds the gate in a `OnceLock` set by the start
    (`budget_with`, 122-126). `answer`, `wait`, `last`, `pause` and `hook.dialog` do not read the gate: its
    product readers are five call sites (`grep -rn -E '\.evaluate\(|\.lift\(\)|budget\.start\(|\.paused\(\)' src`
    outside test modules: `src/cmd/run.rs:126`, `:536`, `src/run/send.rs:347`, `src/run/wheel.rs:214`, `:222`).
  - **Channel method `release`** (`src/run/wheel.rs` 195-202, 213-219, 221-222): `WheelSlot::release(call,
    gate)`. Params unchanged (`{budget?, from?}`); the string-`from` refusal and the `budget` type check stay
    first. `budget: true` → the wheel is not touched, `gate.lift()` runs, and the answer is
    `{"ok":{"wheel":<holder>,"budget_paused":false,"override_until":<end>}}` when a pause was lifted,
    `{"ok":{"wheel":<holder>,"budget_paused":false}}` when none stood; a lift whose record cannot land is
    `-32603` and the pause stands. A plain `release` (no `budget`, or `budget: false`) returns the wheel, lifts
    no pause, and answers `budget_paused` as the gate's last recorded state (it was the constant `false`).
    The result gains one optional key, `override_until`.
  - **CLI `viola release <name> [--budget] [--json]`** (`src/cmd/release.rs`): the `--budget` flag is new
    (26-28); its params are `{"budget": true}` only when given (47-54). `wheel_call` (`src/cmd/pause.rs`) now
    takes the verb's own params and hands its line writer the whole `ok` payload; its two callers, `pause` and
    `release`, changed with it, and it has no other caller. Human lines (`src/human.rs`): the lifted line
    `<name>  budget gate lifted until <end>  wheel <holder>` (`write_budget_lifted`, 205-214), where `<end>`
    (`override_end`, 189-203) is `HH:MM:SSZ` when the override's end falls on the current UTC date and
    `YYYY-MM-DD HH:MM:SSZ` on any other, and a value that is no RFC 3339 instant is shown escaped; and
    `<name>  budget gate open  wheel <holder>` (`write_budget_open`, 216-219) when the reply holds no
    `override_until`. The second line, and the date form of the first, have no words in any master: they are
    the plan's planning forms (step 9), built as planned, and are the founder's to word.
  - **CLI `viola send`** (`src/cmd/send.rs`): `hint_cause` (101-109) keys a refusal's hint: the detail of a
    `not-delivered`, the reason itself for `human-typing` and `budget-paused`, nothing otherwise.
    `human::send_hint` gains the `budget-paused` arm with the design's text (`design-system.md:760`): `budget
    gate is closed; it lifts at the window reset, or when the human lifts it`. Exit 11 was already mapped.
  - **`viola revive`, the start arm** (`src/cmd/revive.rs` 102-107, 119-124): `trusted(home, name)` runs
    `check_instance` over the absolute home and the instance directory BEFORE `open_wrapper_log`. A refusal
    prints the state-files `unable:` / `hint:` pair through `Refusal::write` and returns exit 1: no log is
    opened, no line is written (no `process-start`, no `process-exit`), no child is started, and the home's
    mode is left as found. `--list` reads the same `trusted`. `preflight_order` now takes three readings
    (holder, session, recorded directory); the strict-modes reading left it. `check_instance` has two product
    call sites now (`grep -rn 'check_instance(' src`: `src/cmd/hook/statusline.rs:45`, `src/cmd/revive.rs:123`),
    where it had three.
  - **`viola hook statusline`** (`src/cmd/hook/statusline.rs` 43-47, 53-56): the instance is canonicalised and
    then filtered by `trusted` (`check_instance`) before anything under the home is read or opened. A refused
    instance returns exit 0 at once: no `config.json` read, no role file opened, no `hook-invoked`, no
    `hook-decision`, no stdin read, no command run, nothing printed, `budget.json` untouched, the home's mode
    left as found. `handle` lost its `trusted` argument, and the arm no longer produces
    `hook-decision{detail:"strict-modes-failed"}`.
  - **`config.json`**: `CONFIG_KEYS` (`src/obs.rs` 33-39) holds five names, `v`, `diagnostics_level`,
    `claude_env_keep`, `five_hour`, `seven_day`, so the two threshold keys are not counted as unknown. Each
    threshold rejection is logged at start through `obs::log_config_rejection`, one
    `parse-rejected{parser:"config-json", detail:"malformed", count}` WARN line per bad key, in
    `run-<name>.ndjson`.
  - No new channel method, no new env var, no new `ObsEvent` value, no new `parse-rejected` parser member, no
    new event kind (`budget-gate` and its three data keys were already admitted by `EventKind::BudgetGate`).

- **Crates / modules:** added, one module: `crate::run::budget` in the root bin (`src/run/mod.rs:4`). No crate
  added, removed or re-edged. The root bin now calls `viola_state::budget::read_budget`; `viola-state` reads
  `viola_core::MAX_FRAME` in `budget.rs` (it already depended on `viola-core`).

- **Dependencies:** none added, none bumped. `Cargo.lock` is byte-identical to the base (the preservation
  guard). `Cargo.toml` gains one `[[test]]` entry only (`cli_budget`, `required-features = ["fake-agent"]`,
  lines 101-105). `src/human.rs` and `src/run/budget.rs` use `chrono`, already a dependency of the root
  package; `viola-core`'s dependencies stay `nutype` and `serde`.

- **Schema / config:**
  - `config.json`: two top-level keys, `five_hour` and `seven_day`, each a whole number 0 to 100, defaults 90
    and 85. No value of either switches the gate off.
  - `snapshot.json` `data`: `budget_override_until?` (an RFC 3339 string, the crossed window's `resets_at` as
    `budget.json` holds it), written by the wrapper; `budget_paused` is now the gate's verdict, not a constant.
  - `events.ndjson`: `budget-gate` `data` now carries `window` and `override_until` from a product writer (it
    carried only `{"paused": false}`, once at start).
  - Channel: `release`'s `ok` result gains optional `override_until`; `send` gains the refusal `budget-paused`
    with detail `five-hour` | `seven-day`.
  - `schemas/diag-line.v1.json` is unchanged. Its `send-*` arm already admitted `window`, `used_percentage`,
    `resets_at` and `paused`; its `hook-decision` `detail` enum still lists `strict-modes-failed`, which the
    statusline arm no longer writes.
  - `budget.json`'s own shape is unchanged.

- **Spec-master edits:** none. No file under `.andromeda/` but the route, the master record, the matrix note on
  `v1-35`, the run dirs and the friction ledger changed (`git status --short -- .andromeda/`).

- **Counts / qualifiers moved:**
  - `CONFIG_KEYS`: 3 → 5 names (the array at `src/obs.rs` 33-39).
  - `InstanceSnapshot` struct literals: 12 → 14 (`grep -rn -E 'InstanceSnapshot \{' src crates tests
    --include=*.rs`: 23 lines now, less the type's definition and 8 signature lines; 20 lines at the base, less
    the definition and 7). The two new ones are test literals at `src/run/wheel.rs:927` and
    `src/run/budget.rs:638`. Lines matching `^\s*budget_paused: `: 18 → 20, the same two.
  - `check_instance` product call sites: 3 → 2 (above).
  - `revive`'s preflight readings: 4 → 3, with the strict-modes check now ahead of the log and outside the
    preflight (architecture §Conventions says "whose four readings", `architecture.md:139`, the one hit of
    `four readings` in the seven masters and the registry files).
  - `hook-decision` fail-open codes the statusline arm writes: 4 → 3 (`oversize-stdin`, `malformed-json`,
    `deadline`; obs-plan §6 says "uses four of the seven", `obs-plan.md:837`).
  - Readers of the form `take(MAX_FRAME` under `src` and `crates`: 17 → 19 lines (`grep -rn -E
    'take\(MAX_FRAME' src crates --include=*.rs`): `read_budget` and `read_thresholds`.
  - The nextest 20 s `binary(…)` filter line: 13 → 14 names (`.config/nextest.toml:32`, the count of
    `binary(` on that line): `cli_budget` joined.
  - Root `[[test]]` entries in `Cargo.toml`: 21 → 22 (`grep -c '^\[\[test\]\]'`).
  - Root waits of the form `Instant::now() + WITHIN` under `tests/`: 26 → 28 lines (`grep -rn`).
  - `tests/cli_controls_not_disableable.rs`: one new parametrised negative with three rows
    (`setting_does_not_disable_the_budget_gate`, 345-376).
  - Snapshot readers stay five: no `read_snapshot(` call was added outside a test module (the gate reads no
    snapshot).
  - The capability ledger's row count is unchanged (seventeen gate `cli_verified`): the agent crate, the probes
    and the fixtures are under the preservation guard.
  - Test totals: the unit suite 1765 → 1865 locally (the harness document of the entry `bash
    scripts/agent-run.sh run --unit`, in the earlier implement run's log); `pre-push` coverage 2163 → 2279; CI
    totals 2169 / 2159 / 2163 → 2280 / 2275 / 2279 on `windows-2025` / `macos-latest` / `ubuntu-latest` (each
    run's own Summary line, run `38091081413`).
  - Two qualifiers in the masters are now measured, both as the masters guessed them: "`revive`'s start arm
    opens its wrapper log before the check (read in source, not measured on `revive`)"
    (`architecture.md:406`, `security-plan.md:213`) was measured at P3 on the base build (research.md, runs C
    and D: a home at 0770 came back 0700 and the revive refused `no-session`) and again as the witness's red
    reading (`evidence/red-green.md` (a)); and the statusline arm's "refusal lasts one invocation" hypothesis
    was measured (runs A1 and A2; `evidence/red-green.md` (b)). Both orders are fixed by this chunk, so each
    sentence now describes a past state.

- **Dev-tool versions:** none. Nothing was installed or upgraded. rustc 1.98.1 and cargo-nextest 0.9.146 were
  re-read unchanged (`evidence/red-green.md`, its first paragraph).

- **Harness / gate surface:** no harness file changed (`scripts`, `crates/viola-e2e` and `.github` are under
  the preservation guard). Two config changes: `.config/nextest.toml:32` (above) and the `[[test]]` entry.
  `agent-run boot --statusline-echo` is not built, by the plan's cut. The harness's own readiness stage
  (`start_records`) keeps reading `budget-gate` as record two; that record's `data` can now be
  `{"paused": true, "window": …}` on a home whose `budget.json` is over a threshold at start.

- **Cross-project / external claims:**
  - CI: run `38091081413` (`ci`, event `push`, `run_attempt` 1) on `c85961dc62b3`, conclusion `success`, 15 of
    15 checks, wall 726 s; the only run on that sha. Read with `ci.py conclusion` and from the run itself;
    recorded in `evidence/operator-pass.md`. This wrap's own commit adds to that tree.
  - No fact from another repository is relied on. No live `claude` start was made by this chunk
    (`inputs#I1`); the `rate_limits` shape the gate ends up trusting was read by hand at the previous chunk
    (`viola-0.1.0/chunks/2026-10-10-statusline-pass-through/evidence/live-statusline.md`, in this repository)
    and has no ledger row and no probe.
  - The chunk's inputs, from `inputs.py verify` at this wrap (9 entries: unchanged 1, n/a 8, drifted 0,
    vanished 0, broken 0, uncited 3 before this report existed):
    `inputs#I1` · message, the operator's take-up notes · copy · n/a;
    `inputs#I2` · `~/.claude/projects/…/memory/founder-rulings-stand-until-his-word.md` · copy · unchanged;
    `inputs#I3` · message, the P4 fork answers · copy · n/a;
    `inputs#I4` · message, the P5 review · copy · n/a;
    `inputs#I5` · message, the first implement invocation (steps 1 to 14 only, stop at step 15) · copy · n/a,
    cited here;
    `inputs#I6` · message, the revision's invocation (the hold lifted) · copy · n/a;
    `inputs#I7` · message, the revision's P5 review · copy · n/a;
    `inputs#I8` · message, the second implement invocation (the operator pass from step 16) · copy · n/a,
    cited here;
    `inputs#I9` · message, this wrap's invocation (Phase 1 only, then a resume point) · copy · n/a, snapped at
    this wrap and cited here;
    `inputs#I10` · message, this wrap's resume invocation (the amendment and route directions) · copy · n/a,
    snapped at the resume and cited here.

- **Reverted / negative API facts:**
  - The statusline arm's refused outcome was removed, not replaced: `handle(instance, trusted, …)` and
    `Outcome::failed("strict-modes-failed")` are gone, with the unit case that pinned them and its `NeverRead`
    reader.
  - `revive`'s start arm no longer writes `process-exit{detail:"strict-modes-failed"}`.
    `Refusal::StrictModes.detail()` still returns the word and no line carries it.
  - The witness for the `revive` fix had a first shape that was dropped: it waited for the revive to exit and
    only then read the home's mode. On the reverted order the revive ran on as a wrapper and the case read
    `TIMEOUT` under the 10 s kill (`evidence/red-green.md` (a), the earlier red). The kept shape reads the
    home's mode while the revive still runs.
  - Not built, each by the plan: `agent-run boot --statusline-echo` and the harness-session form of Path 6
    (the cut, proposed for the entry "The board: viola list"); a `statusline-source-unresolved` refusal (its
    premise is gone); the `rate_limits` ledger row, its `viola verify` probe, a re-stamp, any live `claude`
    start (`inputs#I1`); restoring an override at start; a poll or a fourth evaluation point; a new seam for a
    spawned wrapper's clock.

- **Insufficient fixes (written, kept, not the remedy):** none. The check-before-log fix covers `revive`'s
  start arm and the statusline arm, which are the two arms that run an instance check. The other verbs that
  open a role file before any check (`client::start` for `send`, `wait`, `last`, `answer`, `pause` and
  `release`; `hook`'s nine events; `verify`; `run`) run no instance check today and were not this chunk's;
  the route entry "Home and code-bearing file integrity" owns them.

- **Spec claims disproved by measurement:**
  1. **obs-plan §1 and §4, the start's Must-trace spans** (`obs-plan.md:307`: "`run.start` ›
     `run.collision_check` › `run.pin_copy` › `run.version_gate` › `channel.bind` › `state.snapshot_write` ›
     `state.heartbeat_start` › `pty.spawn`"; `obs-plan.md:606`, the same list with `run.conpty_sideload`). As
     landed the start opens `run.budget_eval` between `channel.bind` and `state.snapshot_write`. Evidence: the
     unit case that pins the start's span chain was restated to hold it (`src/cmd/run.rs:1464`) and asserts its
     fields `{"trigger":"start","paused":false}` (`src/cmd/run.rs` 1497-1500). The plan's Expected amendments
     do not name this list.
  2. **The plan's step 14**: "On the red tree step 10's witness lets a child start: it is the fake agent …
     with a null stdin, so it ends at once". Measured false: the revive passed its check, started the fake
     agent and ran on as a wrapper (`evidence/red-green.md` (a), `TIMEOUT [10.003s]`). A plan sentence, not a
     master's; the witness was reshaped (Deviations 1).
  3. **test-plan §4** (`test-plan.md:582`): "`budget_override_until` expiry via mock_instant". As landed the
     expiry is a unit case of a pure function that takes the instant as an argument
     (`gate_decide_an_expired_override_is_dropped`, `src/run/budget.rs` 590-601;
     `gate_release_override_ends_at_its_instant`, 933-957); no crate depends on mock_instant
     (research.md finding 3). The plan's Expected amendments name this.
  4. **architecture §Occupied Resources** (`architecture.md:406`): "a refusal there is exit 0 with nothing
     printed, no command run and no `budget.json` written (detail `strict-modes-failed`)", and
     **the obs-plan key file** `registries/contracts/obs-plan/otel-sdk-init.md:24`: "After a refused check the
     arm still opens the role file and writes `hook-invoked` and `hook-decision{detail:"strict-modes-failed"}`".
     As landed the refused arm writes no line and opens no file (`src/cmd/hook/statusline.rs` 53-56; the
     witness `hook_statusline_a_home_another_user_can_write_runs_nothing_however_often` asserts no added role
     line). Named by the plan's Expected amendments.
  5. **The `revive` start arm's `process-exit` line for `strict-modes-failed`**, stated in architecture
     §Conventions (`architecture.md:139`), security-plan §Authentication & Authorization
     (`security-plan.md:213`: "on the start arm one `process-exit` line in `run-<name>.ndjson`"), test-plan §5
     (`test-plan.md:647`), obs-plan §1 (`obs-plan.md:308`), §4 (`obs-plan.md:621`), §6 (`obs-plan.md:834`,
     `:840`) and the key file `registries/contracts/obs-plan/logging-stack.md:12`. As landed no line is written
     (`tests/cli_revive.rs`: both strict-modes cases assert that `run-builder.ndjson` gained no line). Named by
     the plan's Expected amendments.

- **Expected amendments (from plan):** 16 entries in the plan's list. The search behind each is one fixed-string
  or regex count over the seven masters and every file under `.andromeda/registries/` (a key file counted for
  the master that owns it); the site numbers are that search's hits, read at the lines named.
  1. architecture §Established Decisions [Budget Governor], the gate as landed — **carried**: Symbols, "The
     decision", "The gate", "The three evaluation points". Sites: `Budget Governor` → 2 lines, architecture 2
     (`architecture.md:72`, `:407`).
  2. architecture §Standard Contracts, `release`'s result gains `override_until`; the snapshot gains
     `budget_override_until`; the Hook contract's statusline arm opens no log after a refused check —
     **carried**: Symbols, "Channel method `release`", "`viola-state`", "`viola hook statusline`". Sites:
     `budget_paused` → 19 lines, architecture 6 (the method line is `architecture.md:285`);
     `budget_override_until` → 9 lines, architecture 2 (`:247`, `:255`), test-plan 7. The snapshot contract at
     `:255` already lists `budget_override_until?`; the code now has the field.
  3. architecture §Occupied Resources, the two `config.json` keys; `budget.json`'s reader and its check; the
     `revive` order sentence — **carried**: Symbols, "The thresholds", "`viola-state`", "`viola revive`";
     Counts, the two qualifiers. Sites: `five_hour` → 13 lines, architecture 4 (`:72`, `:216`, `:407`, `:464`;
     `:464` is in §Cross-cutting Patterns), test-plan 8, obs-plan 1; `budget.json` → 37 lines, architecture 10,
     security-plan 8, test-plan 15, obs-plan 4 (4 of the 37 in key files); the order sentence is
     `architecture.md:406`; the `config.json` rule is `:399`.
  4. architecture §Conventions, `revive`'s strict-modes refusal writes no `process-exit` line — **carried**:
     Spec claims disproved 5. Sites: `strict-modes-failed` → 24 lines, architecture 4 (`:139` in §Conventions;
     `:253`, `:406`, `:410`), security-plan 2, design-system 1, test-plan 3, obs-plan 14 (2 of them key files).
  5. security-plan §Authentication & Authorization, the `revive` and statusline rows as fixed; what the founder
     rules on `budget.json` — **carried**: Symbols, "`viola revive`", "`viola hook statusline`",
     "`viola-state`"; Deviations 5 for the founder's part, which is NOT ruled: the wrapper refusing a driver on
     `budget.json` lands as built and is recorded as not ratified. Sites: `security-plan.md:213` (both rows, in
     one line of 9093 characters).
  6. security-plan §Input Validation, the threshold rule in Configuration values; the gate's read in Own state
     files on read — **carried**: Symbols, "The thresholds", "`viola-state`". Sites: `Percent` → 14 lines,
     security-plan 1 (`security-plan.md:237`, the Configuration values row); the state-files row is `:241`.
  7. test-plan §6, Path 6 landed in part with its owed sites by title; E5's budget leg; `revive`'s strict-modes
     case without the log line — **carried**: Coverage of new surfaces; Outcome, criterion 12; Spec claims
     disproved 5. Sites: Path 6 is `test-plan.md:839` to `:863` (`release.{0,12}--budget` → 14 lines, test-plan
     6); E5 is `:943` to `:950` (`budget pause` → 4 lines, test-plan 2); the revive case list is `:991`, `:995`.
  8. test-plan §3, the `boot` key's flag re-pointed to "The board: viola list", the
     `statusline-source-unresolved` reason dropped — **carried as a fact, its re-point waiting on the route**:
     Reverted / negative, "Not built". The owner named is the plan's proposed cut, confirmed at its P5 review
     (`inputs#I4`); the CARRY's move is this wrap's Phase 5, on the operator's route directions. Sites:
     `statusline-echo` → 4 lines, architecture 1 (`architecture.md:361`), test-plan 3 (`test-plan.md:839`,
     `:1090`, and the key file `registries/contracts/test-plan/5-command-implementation.md:4`);
     `statusline-source-unresolved` → 1 line, that key file.
  9. test-plan §4, the gate's unit cases as landed, the instant as an argument — **carried**: Spec claims
     disproved 3; Coverage. Sites: `mock_instant` → 7 lines, test-plan 7 (`test-plan.md:582` in §4, `:845`,
     `:1069`, `:1120`, `:1135`, `:1351`, one key file).
  10. obs-plan §3, the `run` anchor for `revive` and the `hook` anchor for the arm: the check precedes the
      init, a refused check writes no line — **carried**: Symbols, "`viola revive`", "`viola hook statusline`";
      Spec claims disproved 4 and 5. Sites: the two key files `registries/contracts/obs-plan/otel-sdk-init.md`
      (`:21`, `:24`) and `logging-stack.md:12`.
  11. obs-plan §4, Scenario: Budget governor's `run` half as landed; the arm's refused `hook-decision` line
      dropped — **carried**: Symbols, "The span", "The refusal", "`viola hook statusline`". Sites:
      `budget_eval` → 4 lines, obs-plan 4 (`obs-plan.md:337`, `:582`, `:694`, `:698`); the arm's line is
      `obs-plan.md:704`.
  12. obs-plan §6, the `strict-modes-failed` rows of the two arms — **carried**: Counts (4 → 3); Spec claims
      disproved 4 and 5. Sites: `obs-plan.md:834`, `:837`, `:840`.
  13. obs-plan §10, a refused check as a path with no line, on both arms — **carried**: the same two bullets.
      Sites: `opens no process log` → 6 lines, architecture 1 (`architecture.md:405`), obs-plan 5
      (`obs-plan.md:512`, `:581`, `:903`, `:1070` in §10, and `otel-sdk-init.md:21`).
  14. design-system §Surface: cli, the two lines the founder words; the hint as landed — **carried**: Symbols,
      "CLI `viola release`", "CLI `viola send`". The two lines are NOT worded by the founder: they land in
      their planning forms (Deviations 5). Sites: `budget gate` → 8 lines, design-system 3 (`:616` in
      §Surface: web-spa; `:760` the hint, `:803` the lifted line), architecture 3, layout-templates 2.
  15. layout-templates §Output structure — wheel, handoff and dialog verbs, the same two lines — **carried**:
      the same bullet. Sites: `layout-templates.md:456`, `:457` (the second hit, `:381`, is `viola list`'s).
  16. a11y-plan §8, the budget pause listed as a bound that falls on an automated `send` only — **carried**:
      Symbols, "The refusal"; Coverage. Sites: `budget-paused` → 65 lines, a11y-plan 12; §8 starts at
      `a11y-plan.md:812`, and which of the 12 lines sit in it was not read here.
  - **Not on the plan's list, raised by this report:** the start's Must-trace span list (Spec claims
    disproved 1; `state.snapshot_write` → 3 lines, obs-plan 3: `obs-plan.md:307`, `:606`, `:614`).

- **Coverage of new surfaces:**
  - `config.json` keys `five_hour`, `seven_day` → validation `Percent::try_new` behind a whole-number read,
    `take(MAX_FRAME)`✓ · instrumentation `parse-rejected{parser:"config-json"}` per bad key✓ · PII n/a · tests
    unit (`thresholds_*`, `percent_*`) and integration (three rows in `cli_controls_not_disableable`) · a11y
    n/a · tokens n/a
  - `viola_state::budget::read_budget` → validation `check_path` on the home and the file, shared lock,
    `take(MAX_FRAME + 1)`, tolerant parse, `v` = 1✓ · instrumentation none of its own (a refused or unreadable
    file reads as no reading, with no line, by the plan's constraint)✗ by design · PII n/a (two percentages,
    two instants) · tests unit (`budget_read_*`, 8 cases, one `cfg(unix)`) · a11y n/a · tokens n/a
  - the gate at `send` (channel method `send`, refusal `budget-paused`) → validation closed detail
    (`BudgetWindowKind`)✓ · instrumentation span `run.budget_eval` + `send-refused` with four budget fields +
    the `budget-gate` event✓ · PII none (no sent text on the line)✓ · tests unit (`send_budget_paused_*`, 8
    functions) and integration (`path6_*` in `binary(cli_budget)`) · a11y the refusal goes to the driver only,
    a human key still reaches the child and the outer PTY shows none of viola's words (asserted by
    `path6_a_reading_over_the_threshold_refuses_the_send`)✓ · tokens n/a
  - the gate at start and at `turn-ended` → validation n/a · instrumentation span `run.budget_eval`, the
    `budget-gate` event, the snapshot's two fields✓ · PII n/a · tests unit (`gate_eval_*` in
    `src/run/budget.rs` and `src/cmd/run.rs`) and integration (`path6_a_turn_end_closes_the_gate`,
    `path6_a_restart_pauses_again`) · a11y n/a · tokens n/a
  - channel method `release` with `budget: true` → validation `budget` must be a bool, a string `from` refused
    first (`-32602`)✓ · instrumentation the `budget-gate` event and the snapshot; no span and no process-log
    line of its own for a lift (`lift` opens no `run.budget_eval`; the existing `release-from-driver` line
    stands)✗ for a line of its own · PII n/a · tests unit (`gate_release_*` in `src/run/budget.rs` and
    `src/run/wheel.rs`, the restated `methods_release_*`) and integration
    (`path6_release_budget_lifts_the_pause`) · a11y n/a · tokens n/a
  - CLI `viola release --budget` and its two human lines → validation clap flag✓ · instrumentation the verb's
    existing `release.client` span and `cli` role lines✓ · PII n/a · tests unit (`release_*` in
    `src/cmd/release.rs`, `budget_verb_lines_*` and `override_end_*` in `src/human.rs`) and integration · a11y
    one unstyled stdout line, no colour, no escape✓ · tokens n/a (plain text; the design's words for the lifted
    line, planning forms for the other two)
  - the `budget-paused` hint on `viola send` → validation n/a · instrumentation n/a · PII never the sent text✓
    · tests unit (`send_hint_is_the_design_string`, `hint_cause_*`, the no-`release` sweep) and integration
    (`path6_the_refusal_hint_never_names_release`) · a11y the struck `unable` line then one `hint:` line, no
    escape byte (asserted)✓ · tokens the design's string✓
  - `viola revive`'s check before its log → validation `check_instance`✓ · instrumentation none: a refused
    check writes no line, by the operator's P4 answer (`inputs#I3`)✗ by decision · PII no directory, pid or id
    in the pair✓ · tests unit (`preflight_check_*`) and integration (two `cfg(unix)` cases) · a11y n/a · tokens
    n/a
  - `viola hook statusline`'s check before anything → validation `check_instance`✓ · instrumentation none: a
    refused check leaves no trace anywhere✗ by decision · PII n/a · tests unit (`statusline_an_instance_*`) and
    integration (one `cfg(unix)` case of two invocations) · a11y n/a · tokens n/a
  - snapshot field `budget_override_until` → validation n/a (the wrapper's own write) · instrumentation n/a ·
    PII n/a · tests unit and integration (`e5_the_replay_reads_what_the_wrapper_wrote`,
    `state_replay_*`, `state_events_*`) · a11y n/a · tokens n/a

## Deviations from intent

1. **The `revive` witness reads the home's mode while the revive runs** (plan step 10 and step 14). The plan's
   witness asserted the mode after the revive exited, and step 14 expected the wrongly passing revive's child
   to end at once. It did not: the revive ran on as a wrapper, so the first shape read `TIMEOUT`. The landed
   case (`revive_over_a_home_another_user_could_write_is_refused_and_leaves_it_as_found`, `tests/cli_revive.rs`
   in the range 305-355) polls the mode while the process runs, so the reverted order fails it by its own
   assertion in about 3.5 s. Basis: `evidence/red-green.md` (a).
2. **`budget_read_*` cases plant files, each derived from the writer's own bytes** (plan Constraints against
   plan step 13). The constraint rejects a hand-written `budget.json` in any test; step 13 lists read cases for
   an over-cap, an unparseable and a `v` 2 file, which no writer produces. The cases write through
   `write_budget` first and then pad, replace or re-version those bytes (`crates/viola-state/src/budget.rs`
   251-316); the root cases feed only through `viola hook statusline`. The two plan sentences were authored
   together; the constraint holds for every booted-wrapper test and not for the reader's own unit cases.
3. **Two bad threshold keys give two `parse-rejected` lines** (plan step 3 says one rejection for a bad value
   and is silent on two). One rejection per key was chosen, after `ConfigRejection::count`. The acceptance
   criterion words it per value.
4. **A local field visitor in `src/run/send.rs`'s tests** (`NumberFields`, 2022-2041). The shared test visitor
   `viola_channel::test_support::JsonFields` records an `f64` field as its Debug text, so `used_percentage`
   read as a string. `crates/viola-channel` is under the preservation guard, so the shared visitor was left
   and a local one keeps the number. The shared visitor's behaviour stands as it was.
5. **The four items the plan first held for the founder landed on the operator's word, none ratified by the
   founder** (plan step 15 as revised; `inputs#I6`, `inputs#I7`): (a) the `budget-paused` refusal flows with no
   `rate_limits` ledger row; (b) the wrapper refuses a driver on `budget.json`, a file a hook arm writes, that
   the trusted-file list does not hold: landed as built (`read_budget`, its two `check_path` reads), recorded
   as NOT ratified, the founder's word owed; (c) a refused check writes no line on `revive`'s start arm and
   leaves no trace on the statusline arm; (d) the two human lines in their planning forms. Each is carried on
   the handoff for him. The operator's word is no ratification (the playbook's "Boundary widening — what
   ratifies it").
6. **Seven sentences of the plan still word the old hold** (the Goal's last paragraph, the Test Commands
   prose, three operator entries' notes, two Expected-amendments lines). They stand as written, governed by
   the closing clause of step 15 (`inputs#I7`, point 3). Where this report and one of them differ, step 15 is
   what was done.
7. **The chunk was built in two implement runs** (`inputs#I5`, then `inputs#I8`). The first wrote steps 1 to 14
   and stopped at step 15 as the plan then held it; a plan revision followed; the second ran the operator pass
   from step 16 and did not fire the whole block again (below, outcome basis).
8. **The commit and the push of the operator pass were made by the implementing session**, on the operator's
   word (`inputs#I8`); the implement letter leaves both to the operator.

scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 31 · listed 31 · recorded 0
(companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 81`, base `396c108b`, read at
this wrap).

## Decisions & corrections

- **The operator's take-up notes** (`inputs#I1`): no mutation run, witness run or dispatch; the `rate_limits`
  row, its probe, a re-stamp and any live start are the founder's; the two check-order defects in scope, each
  with its own acceptance and witness; a widening goes on the review card as itself; the CI read is the
  `ci.py conclusion` call.
- **The four P4 answers** (`inputs#I3`): a refused check writes no line and opens nothing under the home ("A
  refusal that lasts one invocation is no refusal."); a window with an `unknown` `resets_at` does not pause;
  `release --budget` with no pause standing records nothing; an override does not outlive its wrapper.
- **The cut** (`inputs#I4`): `boot --statusline-echo` and the harness-session form of Path 6 go to "The board:
  viola list".
- **The revision** (`inputs#I6`, `inputs#I7`): the operator pass does not wait for the founder; the four items
  land in their planning forms; the `budget.json` item is not ratified; the `rate_limits` row and its probe
  stay owed and need a next route owner at this wrap.
- **Leans the review confirmed** (plan Provenance): top-level keys `five_hour` and `seven_day`; a bad threshold
  falls back; with both windows crossed the detail is `five-hour`; an override covers both windows until its
  end.
- **Found in the work:**
  - Opening a role file narrows an existing home to 0700 (`open_role_file` → `create_private_dir`). A
    strict-modes check on the home therefore runs before anything opens a file under it, and a guard test for
    it widens the HOME, not a directory under it, and reads the mode while the process still runs.
  - A revive whose check wrongly passes does not exit: it becomes a wrapper. A witness that waits for exit
    reads a timeout, not a failure.
  - `tracing`'s shared test visitor in `viola-channel` turns an `f64` field into text; a test that asserts a
    numeric log field needs `record_f64`.
  - The stamp-ahead PostToolUse hook read rstest literals in `src/human.rs` (the day's date with later clock
    times) as stamps ahead of the clock; the cases were re-dated to 2025.
  - The `pre-push` document and its log carry no coverage figure. The three numbers an acceptance names are in
    `target/agent-run/artifacts/llvm-cov-summary.json`, `data[0].totals`, and its mtime shows whether the run
    wrote it.
  - Sweep hazards: `grep 'InstanceSnapshot \{'` counts signature lines (`-> InstanceSnapshot {`) and the
    type's definition beside the literals; `grep strict-modes-failed` over the masters returns the
    `ledger-stamps` rows and the exit-21 rows beside the two arms this chunk changed (24 lines, of which the
    `revive` and statusline ones are named in Spec claims disproved 4 and 5); `grep 'budget gate'` returns
    `viola list`'s line and the web page's beside the release verb's.
- **In the operator pass** (the second implement run): the Windows-target lint on the host
  (`CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc`) was read before the push
  and exited 0; CI then read green on its first attempt, where the previous chunk's first push read red on a
  Windows-only lint.

## Outcome

**Acceptance criteria, each re-asserted against the diff:**

1. (arch) placement and crate edges — **met**. The gate, its three points and the `release --budget` handling
   are in `src/run/` and `src/cmd/run.rs`; the reader is in `viola-state`; `Percent` and the window word are in
   `viola-core`. `Cargo.lock` is unchanged, no manifest gained a dependency, no Tokio, and no Claude shape is
   read outside the agent crate (the gate reads viola's own `budget.json`). The guard entry exits 0; clippy is
   green.
2. (arch) the `budget-paused` refusal, its details, exit 11, its place in the order; `answer`, `wait`, `last`
   and `pause` not gated — **met**. The order is asserted by `send_budget_paused_comes_after_human_typing`,
   `…_after_the_text_refusals` and `…_before_turn_running`; exit 11 and the two details by the `path6_` cases.
   That the other four methods are not gated holds by the diff (the gate's five product call sites, Symbols);
   no case drives one of them under a closed gate.
3. (arch) `budget-gate` once at start as record two, then only on a change, with only its three keys; the
   snapshot's two fields equal the last line; the replay recovers both — **met** (`gate_eval_*`,
   `path6_a_turn_end_closes_the_gate`, `path6_a_restart_pauses_again`,
   `e5_the_replay_reads_what_the_wrapper_wrote` with all three skip counts 0).
4. (arch) the thresholds, their defaults and boundaries, a bad value's fallback and line — **met**
   (`thresholds_*`, `percent_*`, `gate_decide_*`). Two bad keys give two lines (Deviations 3).
5. (arch) `release` keeps its params; a string `from` with `budget: true` is `-32602` `release-from-driver`
   with one line and the snapshot's two budget fields unchanged; a non-bool `budget` is `-32602` — **met**
   (`gate_release_refused_lifts_nothing`, the restated `methods_release_from_a_driver_is_refused_and_logged`,
   and over the CLI in `path6_the_refusal_hint_never_names_release`).
6. (security) `revive` over a group-writable home — **met**: exit 1, the state-files pair, no child, the
   home's mode unchanged, no line added. The witness is red on the reverted order and green on the fix
   (`evidence/red-green.md` (a)); it passed on `macos-latest` and `ubuntu-latest` in CI. "The case fails on the
   order at `396c108ba64c`" was read with the two statements put back in that order on this tree, not on a
   build of that commit.
7. (security) `hook statusline` over a group-writable home, twice — **met** (`evidence/red-green.md` (b); the
   case passed on both Unix runners).
8. (security) the gate's read of `budget.json` — **met** (`budget_read_*`; `evidence/red-green.md` (d): with
   both `check_path` results dropped, two of the eight cases read red). The read opens the lock sibling; it
   never opens `budget.json` for writing.
9. (security) no threshold value switches the gate off — **met** (three rows, each exit 11 after `five_hour`
   91, with exactly one `parse-rejected` line of parser `config-json`, detail `malformed`).
10. (tests) the unit entry and the two filter entries green; `pre-push` green with coverage at lines 85,
    functions 95, regions 80 or above; no mutation gate — **met**. Coverage read lines 97.98, functions 97.90,
    regions 97.76 in the operator pass's `pre-push`. The ignore regex lives in `crates/viola-e2e`, under the
    guard.
11. (tests) past an override's end the decision pauses again, driven in no cross-process case — **met**
    (`gate_decide_an_expired_override_is_dropped`, `gate_release_override_ends_at_its_instant`).
12. (tests) Path 6's cli and ipc signals — **met, landed in part as the criterion says**: the nine
    `binary(cli_budget)` cases passed on all three CI runners. Not asserted here, each owed elsewhere: the
    harness-session form, step 7's two-instance leg (a unit case holds its rule,
    `gate_release_on_one_instance_leaves_another_paused`), `/api/sessions`, `list --json`, the Playwright
    signal.
13. (obs) one `send-refused` line with its fields, no `send-issued`, the client's `process-exit` with
    `exit_code` 11 — **met** (`send_budget_paused_is_refused_with_the_window_word_and_no_cursor` compares the
    whole line; `path6_a_reading_over_the_threshold_refuses_the_send`).
14. (obs) `run.budget_eval` declared once with `trigger`, `paused`, `window`; G1 clean — **met**: one
    declaration; `gate_eval_opens_one_span_with_its_trigger_and_verdict`; the step `G1 bare instrument`
    concluded `success` in all three `lint` jobs of run `38091081413`. The criterion's own proof is that CI
    read.
15. (obs) every home-level line validates against the unchanged schema; G2 counts no panic line — **met**:
    `schema-check` and `g2-zero-panics.sh` green locally, and `G4 schema conformance` and `G2 zero panics`
    `success` in all three `test` jobs, where the test homes are kept.
16. (design) the human-mode refusal — **met** (`path6_the_refusal_hint_never_names_release` compares stderr
    whole: the struck `unable` line with `budget-paused  five-hour`, then the one hint, no escape byte, exit
    11, empty stdout).
17. (design) `viola release <name> --budget` on a paused instance — **met**: one unstyled stdout line derived
    from the result's `override_until`. Beside it two lines no master words landed in planning forms
    (Deviations 5).
18. (a11y) a human key reaches the child while the gate is closed, and the outer PTY holds none of viola's
    literals — **met** (the last two assertions of `path6_a_reading_over_the_threshold_refuses_the_send`).

No criterion is contradicted by the diff. No criterion rests on an input that drifted (`inputs.py verify`: 0
drifted, vanished or broken).

**Gates** (the plan's `[[gate]]` entries, in order, by their `run`):

- `cargo fmt --all --check` — green · exit 0
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` — green · exit 0
- `bash scripts/agent-run.sh run --unit` — green · exit 0, `"ok":true` (1865 of 1865)
- `bash scripts/agent-run.sh run --unit --filter 'test(/percent_|budget_read_|thresholds_|gate_decide_|gate_eval_|gate_release_|send_budget_paused_/)'`
  — green · exit 0, `"ok":true` (72 of 72)
- `bash scripts/agent-run.sh run --integration --filter 'binary(cli_budget) | binary(cli_revive) | binary(hook_statusline) | binary(cli_controls_not_disableable) | binary(state_replay) | binary(state_events)'`
  — green · exit 0, `"ok":true` (62 of 62)
- `git diff --quiet 396c108ba64c -- crates/viola-agent-claude …` (the preservation guard) — green · exit 0
- `bash scripts/agent-run.sh cleanup --session p-bg-smoke` — green
- `bash scripts/agent-run.sh boot --session p-bg-smoke --instance builder` — green (`"ok":true`, one instance)
- `bash scripts/agent-run.sh status --session p-bg-smoke` — green (`state:"ready"`)
- `bash scripts/g2-zero-panics.sh` — green (`g2: clean`)
- `bash scripts/agent-run.sh schema-check` — green (160 files, 2220 lines, 0 torn)
- `bash scripts/agent-run.sh cleanup --session p-bg-smoke` — green (`processes_gone:true`,
  `endpoint_gone:true`)
- `bash scripts/agent-run.sh pre-push` — green · exit 0, `"ok":true`, `"stage":"linux-tests"`; fired again in
  the operator pass on the uncommitted tree, green (coverage 2279 of 2279, playwright 1 of 1, no breach)
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` (`leg =
  'operator'`) — fired as written in the operator pass: exit 0, `hygiene: clean`, read twice
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD` (`leg = 'operator'`) — fired through
  `--operator`: green · exit 0, `history moved: … 396c108b→c85961dc`, a fast-forward, no force push
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800`
  (`leg = 'operator'`) — fired as written: exit 0, `c85961dc62b3 verdict: green · checks 15/15 · wall 726 s ·
  runs ci#38091081413 completed/success`

The first thirteen readings are the first implement run's second whole-block call (`.andromeda/runs/
2026-10-10T21-29-10-implement/`, `entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3`); the
last three and the second `pre-push` are the operator pass's, in `evidence/operator-pass.md`. No entry carries
`defer`; no `leg = 'live'` or `'round'` entry exists. Smoke: the boot path changed (the start goes through the
gate) and the smoke entries read green inside the block; the harness's readiness stage read `budget-gate` as
record two.

**Watches:** none. The entry carried no `WATCH` and the plan folded none.

**Outcome basis:** the operator pass ran. Its commit list is one commit, `c85961d`, and the final HEAD's CI run
is `38091081413`, green on attempt 1, recorded in `evidence/operator-pass.md`; the gate verdicts above rest on
that state. No fix commit was made. The second implement run's P4 report is in this wrap's conversation (the
same session). The first implement run's conversation is gone (a `/clear` between them), so what only it held
was read from this chunk's implement records in `.andromeda/friction-log.ndjson` (filtered to this marker and
both spellings of the skill: 13 records, 11 from the first run) and from `evidence/red-green.md`: Deviations 1
to 4, Spec claims disproved 1, and the items under "Found in the work" come from there, and each was re-read
in the diff. That run's tallies beyond what those records and the gate trail hold are unmeasured here. Between
the two runs the plan was revised on the operator's word (`inputs#I6`): step 15 alone changed, no built file.

**Process hygiene:**

| process | started by | final state |
|---|---|---|
| the first implement run's gates, smoke session `p-bg-smoke`, red and green runs | the first implement run | unmeasured here for that run's own census (its conversation is gone); its `cleanup` entry read `processes_gone:true`, `killed:[]`, and the process list read at 22:20:02Z held no process whose executable lies under this repository's `target/` or the test-home base |
| `pre-push` and its test children | the second implement run (the gate tool, its pre-push entry) | terminated: none under `target/` or the test-home base at 22:20:02Z and at 22:35:15Z |
| `ci.py conclusion` | the second implement run | terminated (exit 0 at 22:33:20Z) |
| `python scripts/code-graph.py refresh` | this wrap's Setup | terminated (exit 0; rust 5215 nodes / 27952 edges, ts 7 / 1) |

No live `claude` was started by any run of this chunk, and no rig home was made.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 396c108b (the parent of the oldest pre-CI commit c85961dc) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 32
### Cargo.toml — added 5 line(s) in 1 range(s)
added: 101-105
### crates/viola-core/src/lib.rs — added 53 line(s) in 2 range(s)
added: 202-227 · 629-655
- 202-209 @206 «pub enum BudgetWindowKind {»
- 211-218 «impl BudgetWindowKind {»
  - 212-217 «pub const fn as_str(self) -> &'static str {»
  - 629-643 @630 «fn budget_window_kinds_round_trip_kebab_case() {»
  - 645-654 @646 «fn percent_admits_zero_to_one_hundred_and_nothing_else() {»
### crates/viola-state/src/budget.rs — added 205 line(s) in 5 range(s)
added: 5-6 · 10-11 · 15 · 64-98 · 212-376
- 64-70 @66 «struct Stored {»
- 72-97 @77 «pub fn read_budget(home: &Path) -> Option<BudgetReading> {»
  - 84-88 «File::open(&path)»
  - 89-91 «if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME {»
  - 93-96 «(stored.v == BUDGET_V).then_some(BudgetReading {»
  - 212-218 @214 «fn private_home(tmp: &tempfile::TempDir) -> std::path::PathBuf {»
  - 220-225 «fn both_windows() -> BudgetReading {»
  - 227-239 @228 «fn budget_read_returns_what_write_budget_wrote() {»
  - 241-249 @242 «fn budget_read_of_an_absent_file_is_nothing_and_creates_nothing() {»
  - 251-267 @254 «fn budget_read_over_the_frame_cap_is_nothing() {»
  - 269-286 @270 «fn budget_read_of_bytes_that_are_no_envelope_is_nothing() {»
  - 288-303 @289 «fn budget_read_of_another_v_is_nothing() {»
  - 305-316 @307 «fn budget_read_skips_a_key_it_does_not_know() {»
  - 318-322 @319 «fn set_mode(path: &Path, mode: u32) {»
  - 324-328 @325 «fn mode_of(path: &Path) -> u32 {»
  - 330-351 @334 «fn budget_read_under_a_path_another_user_could_write_is_nothing() {»
  - 353-375 @354 «fn budget_read_leaves_the_file_as_it_was() {»
### crates/viola-state/src/liveness.rs — added 1 line(s) in 1 range(s)
added: 88
### crates/viola-state/src/replay.rs — added 1 line(s) in 1 range(s)
added: 184
### crates/viola-state/src/snapshot.rs — added 4 line(s) in 2 range(s)
added: 50-52 · 181
### crates/viola-state/src/strict.rs — added 1 line(s) in 1 range(s)
added: 733
### crates/viola-state/tests/state_events.rs — added 20 line(s) in 1 range(s)
added: 247-266
- 248-266 @251 «fn state_events_budget_gate_lines_with_a_window_and_with_an_override_read_back_whole() {»
  - 253-257 «let written = [»
  - 258-260 «for data in &written {»
### crates/viola-state/tests/state_replay.rs — added 53 line(s) in 3 range(s)
added: 14 · 31 · 250-300
- 251-261 «fn append_gate(dir: &Path, data: Value) {»
  - 253-259 «let line = EventLine::new(»
- 263-275 @264 «fn replayed_gate(dir: &Path) -> (Option<bool>, Option<String>) {»
  - 266-273 «assert_eq!(»
- 277-285 @279 «fn state_replay_a_budget_gate_line_with_a_window_replays_the_pause() {»
- 287-300 @289 «fn state_replay_a_budget_gate_line_with_an_override_replays_its_end() {»
  - 294-297 «assert_eq!(»
### src/cmd/hook.rs — added 1 line(s) in 1 range(s)
added: 740
### src/cmd/hook/statusline.rs — added 49 line(s) in 21 range(s)
added: 43-47 · 53-56 · 75 · 112 · 114 · 122 · 342 · 380 · 452 · 480 · 484 · 486 · 498 · 506 · 508-529 · 537 · 557 · 566
       605 · 634 · 683
- 43-46 @44 «fn trusted(instance: &Instance) -> bool {»
### src/cmd/pause.rs — added 17 line(s) in 7 range(s)
added: 31-38 · 42-45 · 50 · 52 · 70 · 72 · 76
  - 31-38 «let code = wheel_call(»
### src/cmd/release.rs — added 108 line(s) in 7 range(s)
added: 1-5 · 7 · 11 · 26-28 · 36-43 · 47-71 · 76-140
  - 36-43 «let code = wheel_call(»
- 47-54 @48 «fn params(budget: bool) -> Map<String, Value> {»
  - 50-52 «if budget {»
- 56-70 @58 «fn write_line(out: &mut impl Write, name: &str, ok: &Value, budget: bool) -> io::Result<()> {»
  - 60-62 «if !budget {»
  - 63-69 «match ok["override_until"].as_str() {»
- 77-140 @78 «mod tests {»
  - 83-87 @84 «struct Line {»
  - 89-93 «fn parse(words: &[&str]) -> ReleaseArgs {»
  - 95-101 @96 «fn release_args_take_the_budget_flag() {»
  - 103-107 @104 «fn release_params_carry_budget_only_when_asked() {»
  - 109-113 «fn line(ok: &Value, budget: bool) -> String {»
  - 115-122 @116 «fn release_line_of_a_plain_release_names_the_holder() {»
  - 124-133 @126 «fn release_line_of_a_lifted_pause_is_derived_from_override_until() {»
  - 135-139 @136 «fn release_line_with_no_pause_standing_says_the_gate_is_open() {»
### src/cmd/revive.rs — added 70 line(s) in 16 range(s)
added: 56-57 · 102-107 · 119-128 · 208 · 212-213 · 264-267 · 269 · 271 · 273 · 275 · 277 · 279-282 · 285 · 294 · 324-356
       407
  - 102-107 @104 «if !trusted(home, &args.name)? {»
- 119-124 @120 «fn trusted(home: &Path, name: &ViolaName) -> anyhow::Result<bool> {»
  - 324-326 «fn builder() -> ViolaName {»
  - 328-336 @329 «fn preflight_check_passes_a_home_viola_laid_down_and_one_not_there_yet() {»
  - 338-355 @344 «fn preflight_check_refuses_a_directory_another_user_could_write(#[case] widened: &str) {»
### src/cmd/run.rs — added 197 line(s) in 32 range(s)
added: 26 · 77 · 120 · 123-128 · 141 · 256-266 · 268 · 296 · 308 · 514 · 518 · 521-524 · 536 · 545-546 · 561 · 749 · 757
       759-767 · 771 · 775 · 949-950 · 957 · 965 · 1055 · 1058-1064 · 1075-1082 · 1090-1094 · 1127-1241 · 1392
       1394-1397 · 1464 · 1497-1500
  - 257-259 «for rejection in rejections {»
  - 260-266 «let budget = Arc::new(BudgetGate::new(»
  - 1130-1141 @1131 «fn feed_budget(home: &Path, percentage: f64) {»
  - 1143-1148 @1144 «fn private_home(root: &Path) -> PathBuf {»
  - 1150-1176 @1153 «fn gate_eval_at_start_is_the_first_snapshot_and_record_two() {»
  - 1178-1240 @1182 «fn gate_eval_at_a_turn_end_follows_its_line() {»
### src/cmd/send.rs — added 29 line(s) in 3 range(s)
added: 101-110 · 156 · 303-320
- 101-109 @103 «fn hint_cause<'a>(reason: &'a str, detail: Option<&'a str>) -> Option<&'a str> {»
  - 104-108 «match reason_of(reason) {»
  - 303-319 @313 «fn hint_cause_is_the_detail_of_a_not_delivered_and_the_reason_of_the_two_driver_refusals(»
### src/human.rs — added 85 line(s) in 8 range(s)
added: 7-8 · 189-220 · 242-243 · 247-250 · 481-484 · 605-607 · 623 · 643-679
- 189-203 @192 «pub(crate) fn override_end(override_until: &str, today: NaiveDate) -> String {»
  - 193-195 «let Ok(end) = DateTime::parse_from_rfc3339(override_until) else {»
  - 197-201 «let format = if end.date_naive() == today {»
- 205-214 @207 «pub(crate) fn write_budget_lifted(»
- 216-219 @217 «pub(crate) fn write_budget_open(out: &mut impl Write, name: &str, holder: &str) -> io::Result<()> {»
  - 643-655 @644 «fn budget_verb_lines_are_the_design_lines_in_one_write() {»
  - 657-659 «fn day(year: i32, month: u32, day: u32) -> NaiveDate {»
  - 672-678 @673 «fn override_end_is_the_utc_time_with_its_date_on_another_day(»
### src/obs.rs — added 17 line(s) in 2 range(s)
added: 33-39 · 449-458
- 33-39 «const CONFIG_KEYS: [&str; 5] = [»
### src/run/budget.rs — new file · 998 line(s)
- 17-20 «use viola_core::{»
- 33-38 @35 «pub(crate) struct Thresholds {»
- 40-42 «fn compiled(default: u8) -> Percent {»
- 44-51 «impl Default for Thresholds {»
  - 45-50 «fn default() -> Self {»
- 53-66 @56 «pub(crate) fn read_thresholds(home: &Path) -> (Thresholds, Vec<ConfigRejection>) {»
  - 58-65 @59 «match File::open(home.join("config.json")) {»
- 68-89 «fn parse_thresholds(bytes: &[u8]) -> (Thresholds, Vec<ConfigRejection>) {»
  - 69-71 «let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(bytes) else {»
  - 72-74 «if map.get("v").and_then(Value::as_u64) != Some(1) {»
  - 76-83 «let mut threshold = |key: &str, default: u8| match map.get(key).map(whole_percent) {»
  - 84-87 «let thresholds = Thresholds {»
- 91-94 «fn whole_percent(value: &Value) -> Option<Percent> {»
- 96-102 @98 «pub(crate) struct Crossed {»
- 104-110 @107 «pub(crate) struct Standing {»
- 112-134 «impl Standing {»
  - 113-120 @114 «fn triple(&self) -> (bool, Option<BudgetWindowKind>, Option<&str>) {»
  - 122-133 @124 «pub(crate) fn data(&self) -> Value {»
- 136-139 @137 «fn later(instant: &str, now: DateTime<Utc>) -> bool {»
- 141-162 @144 «fn crossed(»
  - 150-153 «let Reading::Known(BudgetWindow {»
  - 154-156 «else {»
  - 157-161 «(*used >= f64::from(threshold.into_inner()) && later(resets_at, now)).then(|| Crossed {»
- 164-199 @167 «pub(crate) fn decide(»
  - 173-178 «if let Some(end) = override_until.filter(|end| later(end, now)) {»
  - 179-194 «let paused = reading.and_then(|reading| {»
  - 195-198 «Standing {»
- 201-207 @203 «pub(crate) enum Trigger {»
- 209-217 «impl Trigger {»
  - 210-216 «const fn as_str(self) -> &'static str {»
- 219-228 @221 «pub(crate) struct BudgetGate {»
- 230-340 «impl BudgetGate {»
  - 231-246 «pub(crate) fn new(»
  - 248-250 «fn recorded(&self) -> MutexGuard<'_, Standing> {»
  - 252-255 @253 «pub(crate) fn paused(&self) -> bool {»
  - 263-277 «fn decide_at(»
  - 279-286 @282 «pub(crate) fn start(&self, now: DateTime<Utc>) -> Standing {»
  - 288-304 @292 «pub(crate) fn evaluate(»
  - 306-322 @309 «pub(crate) fn lift(&self) -> Result<Option<String>, StateError> {»
  - 324-339 «fn record(&self, standing: &Standing) -> Result<(), StateError> {»
- 341-998 @342 «mod tests {»
  - 359-363 «fn at(instant: &str) -> DateTime<Utc> {»
  - 365-370 «fn window(used: f64, resets_at: &str) -> Reading<BudgetWindow> {»
  - 372-380 «fn reading(»
  - 382-385 @383 «fn used(five_hour: f64, seven_day: f64) -> BudgetReading {»
  - 387-392 «fn pair(five_hour: u8, seven_day: u8) -> Thresholds {»
  - 394-398 «fn parsed(config: &str) -> (Thresholds, usize) {»
  - 400-414 @401 «fn thresholds_default_to_ninety_and_eighty_five() {»
  - 416-428 @422 «fn thresholds_read_each_key_as_a_whole_percentage(»
  - 430-447 @440 «fn thresholds_a_value_that_is_no_whole_percentage_takes_its_default(#[case] bad: &str) {»
  - 449-462 @450 «fn thresholds_read_the_home_config_file() {»
  - 464-473 «fn paused_by(window: BudgetWindowKind, used_percentage: f64, resets_at: &str) -> Standing {»
  - 475-477 «fn open() -> Standing {»
  - 479-486 «fn decided(reading: &BudgetReading, override_until: Option<&str>) -> Standing {»
  - 488-500 @489 «fn gate_decide_five_hour_pauses_at_ninety_and_not_below() {»
  - 502-513 @503 «fn gate_decide_seven_day_pauses_at_eighty_five_and_not_below() {»
  - 515-521 @516 «fn gate_decide_with_both_crossed_names_five_hour() {»
  - 523-531 @524 «fn gate_decide_reads_the_thresholds_it_is_given() {»
  - 544-561 @547 «fn gate_decide_a_window_it_cannot_judge_does_not_pause(#[case] unknown: Reading<BudgetWindow>) {»
  - 563-566 @564 «fn gate_decide_with_no_reading_is_open() {»
  - 568-588 @571 «fn gate_decide_a_standing_override_keeps_the_gate_open() {»
  - 590-601 @596 «fn gate_decide_an_expired_override_is_dropped(#[case] end: &str) {»
  - 603-618 @604 «fn gate_decide_data_holds_only_the_keys_that_apply() {»
  - 620-622 «fn name() -> ViolaName {»
  - 624-631 @626 «struct Rig {»
  - 633-663 «fn gate_in(home: &Path, instance: &str) -> (PathBuf, BudgetGate) {»
  - 665-675 «fn rig() -> Rig {»
  - 677-687 «impl Rig {»
  - 689-702 @690 «fn gate_records(instance_dir: &Path) -> Vec<Value> {»
  - 704-708 @705 «fn mirrored(instance_dir: &Path) -> (bool, Option<String>) {»
  - 710-730 @712 «fn gate_eval_at_start_hands_back_its_verdict_and_writes_nothing() {»
  - 732-789 @735 «fn gate_eval_records_a_line_only_on_a_change() {»
  - 791-811 @793 «fn gate_eval_a_passed_reset_lifts_the_pause_at_the_next_point() {»
  - 813-828 @816 «fn gate_eval_a_change_that_cannot_be_recorded_is_an_error() {»
  - 834-852 «impl<S: tracing::Subscriber> Layer<S> for Spans {»
  - 854-865 @855 «fn eval_spans(f: impl FnOnce()) -> Vec<Value> {»
  - 867-887 @868 «fn gate_eval_opens_one_span_with_its_trigger_and_verdict() {»
  - 889-910 @892 «fn gate_release_lifts_a_standing_pause_until_the_window_reset() {»
  - 912-920 @913 «fn gate_release_with_no_pause_standing_records_nothing() {»
  - 922-931 @924 «fn gate_release_does_not_evaluate() {»
  - 933-957 @936 «fn gate_release_override_ends_at_its_instant() {»
  - 959-987 @961 «fn gate_release_on_one_instance_leaves_another_paused() {»
  - 989-997 @990 «fn gate_release_that_cannot_be_recorded_is_an_error_and_the_pause_stands() {»
### src/run/dialog.rs — added 1 line(s) in 1 range(s)
added: 453
### src/run/mod.rs — added 1 line(s) in 1 range(s)
added: 4
### src/run/send.rs — added 354 line(s) in 15 range(s)
added: 33 · 88-89 · 95 · 111 · 122-126 · 313-314 · 345-352 · 494-517 · 541-544 · 557 · 559-560 · 562 · 565 · 567
       1970-2269
  - 122-125 @123 «pub(crate) fn budget_with(&self, gate: Arc<BudgetGate>) {»
  - 345-352 «if let Some(budget) = slot.budget.get() {»
  - 498-507 @500 «fn budget_paused(&self, crossed: &Crossed) -> Result<Value, ProtocolError> {»
  - 1974-1979 «fn used(percentage: f64) -> Reading<BudgetWindow> {»
  - 1981-1994 @1982 «fn budget_rig(root: &Path) -> (std::path::PathBuf, std::path::PathBuf, Arc<BudgetGate>) {»
  - 1996-2002 «fn feed_budget(home: &Path, five_hour: f64, seven_day: f64) {»
  - 2004-2016 @2006 «fn budgeted(»
  - 2022-2041 «impl tracing::field::Visit for NumberFields {»
  - 2047-2054 «impl<S: tracing::Subscriber> Layer<S> for Lines {»
  - 2056-2062 «fn capture_lines<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {»
  - 2064-2121 @2072 «fn send_budget_paused_is_refused_with_the_window_word_and_no_cursor(»
  - 2123-2142 @2125 «fn send_budget_paused_fields_are_on_no_other_refusal() {»
  - 2144-2156 @2147 «fn send_budget_paused_comes_after_human_typing() {»
  - 2158-2169 @2162 «fn send_budget_paused_comes_after_the_text_refusals(#[case] text: &str, #[case] detail: &str) {»
  - 2171-2201 @2176 «fn send_budget_paused_comes_before_turn_running(#[case] in_flight: bool) {»
  - 2203-2217 @2206 «fn send_budget_paused_does_not_refuse_under_the_thresholds() {»
  - 2219-2255 @2222 «fn send_budget_paused_is_not_read_again_after_the_gate_wait() {»
  - 2257-2269 @2259 «fn send_budget_paused_whose_record_cannot_land_is_internal() {»
### src/run/snapshot.rs — added 1 line(s) in 1 range(s)
added: 68
### src/run/wheel.rs — added 186 line(s) in 10 range(s)
added: 25 · 195-202 · 213-219 · 221-222 · 536-537 · 540 · 725 · 805 · 894 · 910-1071
  - 911-921 @912 «fn open_gate() -> BudgetGate {»
  - 926-943 «fn first_snapshot() -> InstanceSnapshot {»
  - 945-970 @946 «fn paused_gate(root: &std::path::Path) -> (PathBuf, BudgetGate) {»
  - 972-980 «fn budget_records(dir: &std::path::Path) -> Vec<Value> {»
  - 982-985 «fn budget_fields(dir: &std::path::Path) -> (bool, Option<String>) {»
  - 987-1014 @990 «fn gate_release_budget_answers_the_override_and_leaves_the_wheel() {»
  - 1016-1032 @1018 «fn gate_release_plain_lifts_no_pause_and_answers_it() {»
  - 1045-1057 @1047 «fn gate_release_refused_lifts_nothing(#[case] params: Value, #[case] fault: ProtocolError) {»
  - 1059-1071 @1061 «fn gate_release_budget_that_cannot_be_recorded_is_internal() {»
### tests/cli_budget.rs — new file · 503 line(s)
- 21-23 «use support::statusline::{»
- 31-32 «const REFUSED_FIVE_HOUR: &str =»
- 33-34 «const REFUSED_SEVEN_DAY: &str =»
- 35-36 «const HINT: &str =»
- 38-41 @39 «fn boot(stamped: StampedHome, script: Option<&str>) -> Wrapper {»
- 43-47 @44 «fn send(home: &Path) -> Ran {»
- 49-53 «fn release(home: &Path, flags: &[&str], from: Option<&str>) -> Ran {»
- 55-65 @56 «fn gate_records(instance_dir: &Path) -> Vec<Value> {»
  - 57-64 «events(instance_dir)»
- 67-70 «fn budget_json(home: &Path) -> Value {»
- 72-74 «fn diagnostics(home: &Path, file: &str) -> Vec<Value> {»
- 76-93 @77 «fn assert_logs_clean(home: &Path) {»
  - 80-92 «for entry in std::fs::read_dir(home.join("diagnostics")).expect("diagnostics") {»
- 95-97 «fn open() -> Value {»
- 99-101 «fn five_hour() -> Value {»
- 103-105 «fn seven_day() -> Value {»
- 107-184 @111 «fn path6_a_reading_over_the_threshold_refuses_the_send(stamped_home: StampedHome) {»
  - 119-123 «assert_eq!(»
  - 132-135 «let refusals: Vec<&Value> = lines»
  - 137-141 «assert_eq!(»
  - 143-146 «assert_eq!(»
  - 149-152 «let refused: Vec<&Value> = run»
  - 167-170 «let exits: Vec<Value> = diagnostics(&home, "cli-builder.ndjson")»
  - 175-177 «for word in ["unable", "hint:", "budget-paused"] {»
  - 179-181 «fake::wait_for(&wrapper.receipt(), "the human key's receipt", |l| {»
- 186-217 @189 «fn path6_the_boundaries_and_the_window_word(stamped_home: StampedHome) {»
  - 208-211 «assert_eq!(»
  - 212-215 «assert_eq!(»
- 219-246 @224 «fn path6_an_unknown_or_passed_reset_does_not_pause(»
  - 241-244 «assert_eq!(»
- 248-322 @252 «fn path6_release_budget_lifts_the_pause(stamped_home: StampedHome) {»
  - 261-264 «assert_eq!(»
  - 272-278 «assert_eq!(»
  - 280-283 «let record = lines»
  - 284-287 «assert_eq!(»
  - 288-290 «let instant = |value: &Value| {»
  - 294-297 «assert_eq!(»
  - 299-303 «assert_eq!(»
  - 312-315 «assert_eq!(»
- 324-382 @328 «fn path6_the_refusal_hint_never_names_release(stamped_home: StampedHome) {»
  - 338-341 «assert_eq!(»
  - 343-346 «assert!(»
  - 351-355 «assert_eq!(»
  - 358-361 «assert!(»
  - 362-366 «assert_eq!(»
  - 371-374 «assert_eq!(»
  - 375-378 «let from_driver = diagnostics(&home, "run-builder.ndjson")»
- 384-396 @385 «fn wait_snapshot(instance_dir: &Path, what: &str, pred: impl Fn(&Value) -> bool) -> Value {»
  - 388-395 «loop {»
- 398-429 @401 «fn path6_a_turn_end_closes_the_gate(stamped_home: StampedHome) {»
  - 412-415 «let lines = wait_events(&dir, "the gate's record behind the turn's end", |l| {»
  - 416-419 «let ended = lines»
  - 425-427 «wait_snapshot(&dir, "the snapshot's pause", |data| {»
- 431-466 @434 «fn path6_a_restart_pauses_again(stamped_home: StampedHome) {»
  - 442-445 «assert_eq!(»
  - 458-461 «assert!(»
- 468-472 «const NONE_SKIPPED: Skipped = Skipped {»
- 474-503 @477 «fn e5_the_replay_reads_what_the_wrapper_wrote(stamped_home: StampedHome) {»
  - 493-496 «assert_eq!(»
  - 497-500 «assert_eq!(»
### tests/cli_controls_not_disableable.rs — added 39 line(s) in 3 range(s)
added: 4-7 · 20-21 · 344-376
- 345-376 @352 «fn setting_does_not_disable_the_budget_gate(stamped_home: StampedHome, #[case] value: &str) {»
  - 354-357 «std::fs::write(»
  - 361-366 «assert_send_control_holds(»
  - 367-371 «let rejected: Vec<Value> =»
### tests/cli_revive.rs — added 66 line(s) in 4 range(s)
added: 2-4 · 279-286 · 300-303 · 305-355
- 279-283 @280 «fn mode_of(path: &Path) -> u32 {»
  - 326-328 «let child = revive_command(home, home.scratch(), &["builder"])»
  - 332-341 «while running.still_running() {»
  - 350-354 «assert_eq!(»
### tests/hook_statusline.rs — added 35 line(s) in 11 range(s)
added: 18 · 20-21 · 24 · 42 · 147-150 · 193 · 246-248 · 251 · 256-258 · 261-272 · 274-279
  - 147-150 «assert_eq!(»
  - 261-271 «for used in [12, 13] {»
### tests/support/mod.rs — added 1 line(s) in 1 range(s)
added: 14
### tests/support/statusline.rs — new file · 89 line(s)
- 26-33 @27 «pub fn payload(rate_limits: Option<Value>) -> Vec<u8> {»
  - 29-31 «if let Some(rate_limits) = rate_limits {»
- 35-40 @36 «pub fn reading(used: u64) -> Vec<u8> {»
  - 37-39 «payload(Some(»
- 42-48 @43 «pub fn usage(five_hour: u64, seven_day: u64) -> Vec<u8> {»
  - 44-47 «payload(Some(json!({»
- 50-73 @52 «pub fn hook_statusline(env: &[(&str, OsString)], stdin: Vec<u8>) -> Ran {»
  - 54-61 «command»
  - 62-64 «for (key, value) in env {»
  - 67-69 «let writer = std::thread::spawn(move || {»
- 75-82 @76 «pub fn hook_as_builder(instance_dir: &Path, stdin: Vec<u8>) -> Ran {»
  - 77-80 «let env = [»
- 84-89 @85 «pub fn feed(instance_dir: &Path, stdin: Vec<u8>) {»
