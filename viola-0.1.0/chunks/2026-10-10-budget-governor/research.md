# Codebase Research — 2026-10-10-budget-governor

## Scope
- **Depth:** deep · **Reads:** 40 · **Globs/Greps:** 22 (counted from this session's calls; one code-graph
  query, rust plane, `db_state` fresh, 102 rows by the trail's `rows` field,
  `tree-query-2026-10-10-budget-governor.json` in the phase run dir)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read whole, 15 Session Additions) and
  `.claude/rules/testing.md` (read whole, 38 Session Additions). Applied: `budget.json` is produced only by
  `viola hook statusline`; a root case that looks a program up by name puts the fake agent first on `PATH`; a
  wait keys on the asserted line; no kill at a process the test did not start; a new stamped-home root binary
  joins the 20 s `binary(…)` filter line of `.config/nextest.toml`; a `cfg(unix)`-only helper is gated like
  its callers; no mutation entry in a chunk's gates.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's six notes at take-up; `inputs#I2` — a founder ruling
  stands until his own word, planned behind a stop rule; `inputs#I6` — the operator's word at the revision:
  the operator pass no longer waits, the four held items land in their planning forms as built, the
  `budget.json` refusal is recorded as not ratified, and step 15 alone changes.

## Revision: P3 as premise closure (the stop its finding)
- **The stop.** /andromeda-implement ended at the plan's step 15, as the plan held it. Nothing the stopped run
  left falsifies a scope bullet; the cause of the revision is the operator's word (`inputs#I6`).
- **What the stopped run left, read where it is.** `evidence/red-green.md`: four red and green pairs, for
  steps 10, 11, 7 and 2. No `scope-record.md` (the chunk folder holds `evidence/`, `inputs/`, `plan.md`,
  `research.md` and `scope.md`). The tree's edits: the implement trail
  (`.andromeda/runs/2026-10-10T21-29-10-implement/gate-2026-10-10-budget-governor.json`) reads `entries 16 ·
  green 13 · red 0` with `not-run 3` on each of its two `run` calls, and `scope: clean — changed 31 · listed
  31 · recorded 0` on its last `scope` call. HEAD is `396c108`, with no pre-CI commit for this marker
  (`git log --oneline --grep='chore(2026-10-10-budget-governor)'` printed no line).
- **The four items' built forms, read in the tree at this revision** (3 reads, 1 grep batch):
  - the rung: `src/run/send.rs:347-350` evaluates the gate with the `send` trigger and returns
    `ctx.budget_paused`; the ledger, the probes and the fixtures stand at the base (`git diff --quiet
    396c108ba64c -- crates/viola-agent-claude src/cmd/verify.rs src/cmd/verify fixtures` exits 0);
  - the read: `read_budget` at `crates/viola-state/src/budget.rs:77-82` runs `check_path` on the home and on
    `budget.json` before the shared lock;
  - the refused check: `revive` reads its check at `src/cmd/revive.rs:104`, ahead of `open_wrapper_log` at
    `:108` (`evidence/red-green.md`, reading (a)); the statusline arm returns at
    `src/cmd/hook/statusline.rs:56-58`, ahead of `viola_obs_init` at `:60` (reading (b));
  - the two lines: `write_budget_open` (`src/human.rs:216-219`) and `override_end` (`src/human.rs:192-203`),
    which puts the date ahead of the time when the end falls on another UTC date.
- **The write set is unchanged.** The two lists below stand as P3 wrote them. No extract is voided: the word
  moves when the operator pass starts, not what a master says, so P2 did not re-run and no distiller was
  re-spawned. The take-up's graph trace stands in the take-up run dir
  (`.andromeda/runs/2026-10-10T20-53-16-phase/tree-query-2026-10-10-budget-governor.json`).
- **Closure.** `scope.md` gained the section "The revision: the operator pass no longer waits", each bullet
  verified against the snapshot or the tree, and one corrected bullet in item 11 (the last clause of
  `inputs#I4`).

## Measured at P3 (the session scratchpad's rig `m-20261010T210657Z/`, 2026-10-10T21:06:57Z)
`target/debug/viola` (built 2026-10-10 21:39 local, after the last commit that touches `src/` or `crates/`,
`58f8720`; no product file is newer than it: `find src crates -name '*.rs' -newer target/debug/viola` printed
nothing). Every run under `env -i` with `PATH` one empty directory, so no `VIOLA_*` or `CLAUDE*` value crossed
and no `claude` could be resolved. Scratch homes only; nothing under the repository or the user's home.

| Run | Before | Reading |
|---|---|---|
| A1. `viola hook statusline`, a payload with `rate_limits` (`five_hour` 91) | home 0770, instance directory 0700 | exit 0, stdout and stderr empty, `budget.json` absent, role lines `hook-invoked` then `hook-decision` WARN `strict-modes-failed` `budget_written:false`; home after: **0700** |
| A2. the same invocation again, nothing touched between | home 0700 (as A1 left it) | exit 0, `budget.json` **present**, `hook-decision` INFO `budget_written:true` |
| B. control: the arm over a home at 0700 | home 0700 | exit 0, `budget.json` present, `budget_written:true` |
| C. `viola --home <home> revive rv`, no logged session | home 0770, instance directory 0700 | exit 1, stderr `unable: rv has no logged session to resume` + its hint, role lines `process-start` then `process-exit` `no-session`; home after: **0700** |
| D. control: the same, home 0700, instance directory 0770 | — | exit 1, stderr `unable: rv's state files can be written by another user` + its hint, `process-exit` `strict-modes-failed`; instance directory after: 0770 |

What the readings show:
- **The statusline arm's one-invocation hypothesis is confirmed** (A1, A2): the refused invocation opens its
  role file, which narrows the home, and the next invocation passes the check and writes the reading.
- **The `revive` defect is measured** (C against D): over a group-writable home the start arm narrows the
  home, passes the check and refuses for the next cause in its order. The check itself refuses a widened
  instance directory (D), and the log's opening does not narrow the instance directory.
- Not measured: Windows and macOS; a home another user owns; a home with an other-writable bit only.

## Files inspected
- `src/cmd/revive.rs` (60-230) — `revive` calls `open_wrapper_log` (:101) then `preflight` (:102);
  `preflight`'s first reading is `check_instance` (:145); `Refusal::write` prints through `human::refuse`
  (:91-94), which needs no log; `refused(detail)` (`src/cmd/run.rs:323`) writes the `process-exit` line and
  needs one. `list` (:202-218) is the landed form of a check with no log: the stderr pair and exit 1.
- `src/cmd/run.rs` (140-345, 395-560) — `open_wrapper_log` (:169) reads `config.json` for the level, opens
  the role file, logs the start, reads `claude_env_keep`; `start` (:206) builds the slots and calls
  `start_state` (:500), which writes the first snapshot with `budget_paused: false` (:519) and appends
  `wheel` then `budget-gate{paused:false}` (:529-537); `Methods::dispatch_call` (:124-133) routes `send` and
  `release`; `Methods::dispatch` (:113-118) appends a hook's event through `append_hook_event`.
- `src/obs.rs` (30-232) — `open_role_file` calls `ensure_private_dir(home)` (:77), and `create_private_dir`
  narrows an existing directory to 0700 (:37-40): the mechanism behind A1 and C. `read_diagnostics_level`
  (:170) and `parse_diagnostics_level` (:183) are the landed `config.json` reader: `MAX_FRAME` cap, `v` = 1,
  a bad value → the default plus `ConfigRejection::Malformed`, unknown keys counted against `CONFIG_KEYS`
  (:33, three names: `v`, `diagnostics_level`, `claude_env_keep`).
- `src/run/env.rs` (20-70) — the second `config.json` reader (`claude_env_keep`): its own bounded read, a bad
  value reported `Malformed`, the document's own faults left to `read_diagnostics_level` to report once.
- `src/cmd/hook/statusline.rs` (1-205) — `check_instance` (:53) runs before `viola_obs_init` (:55), which
  runs whatever the check said; `handle` (:110) logs `hook-invoked`, then serves or records
  `strict-modes-failed`; `serve` (:128) writes the reading through `write_budget`.
- `src/run/send.rs` (255-470) — `send` (:306) is the refusal order as code: params, `control-character`
  (:321), `empty-text` (:326), `human-typing` (:329), `turn-running` (:339), the gate's wait, then the second
  read of the wheel and the turn (:357-366). `Ctx::refuse` is the one `send-refused` emitter. The clock is
  `viola_core::Clock`. `append_hook_event` (:216) is where a `turn-ended` line is seen before it is appended
  (:236-238).
- `src/run/wheel.rs` (24-290) — the model: state under one `Mutex` (`Held`), a worker thread fed `Job`s
  (`record_with`, :118), `Recorder::record` (:271) appending the event and updating the snapshot under the
  span `run.wheel_transition` (:262-270, declared, asserted by no test: `grep -rn wheel_transition src` → 1
  hit). `release` (:195-210) refuses a string `from` first, parses `budget`, and answers the constant
  `budget_paused: false`.
- `src/cmd/release.rs` (whole, 40 lines) and `src/cmd/pause.rs` (40-74) — `ReleaseArgs` holds `name` and
  `json` only; `wheel_call` builds params with `from` alone and hands its line writer `(name, holder)`.
- `src/human.rs` (178-232) — `write_released` (:183); `send_hint` (:209) has arms for `human-typing` and six
  `not-delivered` causes, none for `budget-paused`; a unit case `no_hint_names_release` exists (:569 by the
  graph, 0-indexed 568).
- `src/cmd/send.rs` (87-156) — `exit_of` maps `budget-paused` to 11 (:90); `Out::unable` (:138) looks a hint
  up only for a `not-delivered` detail or the `human-typing` reason.
- `src/cmd/client.rs` (14-30) — `client::start` opens the `cli` role file for `send`, `wait`, `last`,
  `answer`, `pause` and `release` before any check.
- `crates/viola-core/src/lib.rs` (15-40, 100-130, 225-262) — `Clock::now()` returns `std::time::Instant`
  (:23-25); `Reading<T>`, `BudgetWindow`, `BudgetReading` (:234-257); `EventKind::BudgetGate` admits
  `paused`, `window`, `override_until` (:108). `viola-core`'s dependencies are `nutype` and `serde` only
  (`crates/viola-core/Cargo.toml`).
- `crates/viola-state/src/budget.rs` (1-66) — `write_budget` alone; the envelope `{v, five_hour, seven_day,
  read_at}`; the lock sibling opened through `open_private_lock`.
- `crates/viola-state/src/snapshot.rs` (37-66) — `InstanceSnapshot` holds `budget_paused: bool` and no
  `budget_override_until`; additive fields are `#[serde(default, skip_serializing_if = …)]`.
- `crates/viola-state/src/replay.rs` (20-75) — the replay sets `budget_paused` and `budget_override_until`
  from each `budget-gate` line, so the last one decides.
- `crates/viola-state/src/strict.rs` (20-80) — `check_instance`, `check_path`, `unix_verdict`.
- `schemas/diag-line.v1.json` (40-68) — the `send-*` arm already admits `window` (string),
  `used_percentage` (number), `resets_at` (string), `paused` (boolean), `side`, `wheel`; the `hook-decision`
  `detail` enum holds `strict-modes-failed`.
- `tests/hook_statusline.rs` (30-150, the case list) — the runner `hook_statusline`, `payload`, `reading`,
  and `Session` (a stopped `builder` in a home viola created); `tests/cli_revive.rs` (the case list) — the
  strict-modes case widens the instance directory (:285) and reads `assert_refused` (:61), which asserts the
  `process-exit` line; `tests/cli_controls_not_disableable.rs` (the case list) — five negatives, no threshold
  row and no completeness case; `tests/support/home.rs` (the function list).
- `.config/nextest.toml` (:32) — the `binary(…)` line of verify-driven root binaries under the 20 s kill.
- `.andromeda/architecture.md` (:72, :211, :216, :285, :399, :464), `.andromeda/test-plan.md` (:273-282,
  :582, :641, :835-863, :939-951), `.andromeda/design-system.md` (:758-762, :800-805),
  `.andromeda/layout-templates.md` (:450-460), `.andromeda/security-plan.md` (:237), `.andromeda/playbook.md`
  (the three `verdict: escalate` patterns) — the contract lines the findings quote.
- `viola-0.1.0/chunks/2026-10-10-statusline-pass-through/evidence/live-statusline.md` (:6-7, :95-100, :122,
  :136) — the hand reading the third CARRY cites still says what the CARRY says.

## Graph impact (rust plane; editor line = the graph's line + 1)
- **`open_wrapper_log`** — 2 callers: `run` @ `src/cmd/run.rs:151`, `revive` @ `src/cmd/revive.rs:101`. The
  reorder touches `revive` alone; `run`'s own order is not this chunk's.
- **`viola_obs_init`** — 5 product callers: `client::start` @ `src/cmd/client.rs:24`, `hook` @
  `src/cmd/hook.rs:119`, `statusline` @ `src/cmd/hook/statusline.rs:55`, `open_wrapper_log` @
  `src/cmd/run.rs:171`, `verify` @ `src/cmd/verify.rs:60`. These are "every verb that opens a role file":
  only `revive` and the statusline arm run an instance check at all today.
- **`check_instance`** — 3 product callers: `statusline` @ `:53`, `preflight` @ `src/cmd/revive.rs:145`,
  `list` @ `src/cmd/revive.rs:205`.
- **`refused`** — `preflight` @ `src/cmd/revive.rs:141` and five sites in `src/cmd/run.rs` (:217, :219, :229,
  :243, :345): each writes a `process-exit` line and so needs the log open.
- **`wheel_call`** — 2 callers: `pause` @ `src/cmd/pause.rs:31`, `release` @ `src/cmd/release.rs:31`. A
  changed signature changes both.
- **`send_hint`** — 3 product callers: `Out::unable` @ `src/cmd/send.rs:151`, `Out::unreachable` @ `:159`,
  `unable_unreachable` @ `src/cmd/client.rs:154`.
- **`write_released`** — 1 product caller: `release` @ `src/cmd/release.rs:32`.
- **`append_hook_event`** — 1 product caller: `Methods::dispatch` @ `src/cmd/run.rs:118`; 20 test callers in
  `src/run/send.rs`. A changed signature changes all 21.
- **`start_state`** — `start` @ `src/cmd/run.rs:276` and one test @ `:1036`.
- **`write_budget`** — 1 product caller: `serve` @ `src/cmd/hook/statusline.rs:141`.
- **`read_diagnostics_level`** — 5 product callers (`client::start`, `hook`, `statusline`,
  `open_wrapper_log`, `verify`): the threshold read is a separate function, so this one's signature stands.
- `InstanceSnapshot { … }` literal sites: 12 (`grep -rn -E '^\s*budget_paused: ' --include=*.rs src crates
  tests` → 18 hits, of which 6 are `Replayed` literals with an `Option`): `src/cmd/run.rs:519` (product), and
  eleven test literals in `src/cmd/hook.rs`, `src/cmd/revive.rs`, `src/cmd/hook/statusline.rs`,
  `src/run/dialog.rs`, `src/run/wheel.rs`, `src/run/snapshot.rs`, `crates/viola-state/src/liveness.rs`,
  `strict.rs`, `snapshot.rs`, `replay.rs` and `crates/viola-state/tests/state_replay.rs`. A new snapshot field
  is added at each.

## Findings
1. **Nothing reads `budget.json`.** `write_budget` is the file's only function. The reader is new, beside it.
2. **`Percent` does not exist** (`grep -rn Percent --include=*.rs crates src` → 0 hits), though CLAUDE.md and
   architecture name it as `viola-core`'s. This chunk mints it; `nutype` is already a dependency there.
3. **The clock seam gives no wall time.** `viola_core::Clock::now()` is an `Instant`, `resets_at` is an RFC
   3339 UTC instant, `viola-core` has no chrono, and no crate depends on mock_instant (`grep -rn mock_instant
   Cargo.toml crates/*/Cargo.toml` → 0 hits). So the gate's decision takes the wall-clock instant as an
   argument: the product passes `Utc::now()` at each of the three points, a unit case passes a literal.
4. **`--budget` is not a CLI flag.** The wrapper's `release` takes `budget` (`src/run/wheel.rs:202`), the
   contract lists the flag (`architecture.md:285`, `:362`), and `ReleaseArgs` does not have it. The verb, its
   param, its human line and the `wheel_call` plumbing are this chunk's.
5. **No `budget-paused` hint is printed.** `send_hint` has no arm, and `Out::unable` asks for a hint only for
   `human-typing` and the `not-delivered` details. The design's text exists (`design-system.md:760`).
6. **The process-log schema already admits the refusal's four budget fields** on `send-refused`. `window` and
   `resets_at` are strings, `used_percentage` a number, `paused` a boolean. `run.budget_eval` is a span, and
   spans are not serialized, so `trigger` needs no schema row. No schema change is needed for the refusal.
7. **A bad threshold has a landed rule to follow.** `architecture.md:399`: "An unusable file or value falls
   back to the default and is reported as a `parse-rejected{parser:"config-json"}` line." The landed
   `diagnostics_level` arm reports a bad value as `malformed` (`src/obs.rs:190-206`). `test-plan.md:641`
   allows either a fallback or a refusal to start and fails only if a `send` is accepted after `five_hour` 91.
8. **The masters spell the threshold keys `five_hour` and `seven_day` and give no nesting**
   (`architecture.md:464`, `test-plan.md:641`, `security-plan.md:237`). `config.json`'s landed keys are
   top-level and flat.
9. **Three behaviours have no master line** (`grep -n -i budget` over the seven masters, read at P1 and
   through the seven extracts, each of which says the same): what `release --budget` does and prints when no
   pause stands; whether a window over its threshold with an `unknown` `resets_at` pauses, and what an
   override over it lasts until; whether an override outlives its wrapper. The layouts extract adds a fourth
   gap in the human line: an override end on a later day than the clock time shows.
10. **The reordered refusal has no carrier that keeps both the line and the home.** `refused` needs the log;
    opening the log narrows the home (finding C); obs-plan §3 step 4 says no diagnostics line for a refused
    home, while the same key's `run` anchor, architecture, security-plan and test-plan each require a
    `process-exit` line for `strict-modes-failed` on `revive`'s start arm. `revive --list` is the landed
    precedent of the stderr pair with no line. The statusline arm has the same tension: its contracted
    `hook-decision{detail:"strict-modes-failed"}` line is what narrows the home (A1).
11. **The harness-session form of Path 6 has no place in this chunk.** `crates/viola-e2e` has no
    `harness_session` fixture (`grep -rln harness_session crates/viola-e2e/src` → 0 files), no `path_` binary
    (`ls crates/viola-e2e/tests` → `cli.rs`, `harness_lifecycle.rs`, `scan_patterns.rs`, `zero_retries.rs`),
    and no statusline code. The working route pins the first `path_` E2E binary and `run --e2e` on the entry
    "The board: viola list" (its CARRY from chunk 2026-09-29-h2-conpty-resize-probe), which also owns Path 6's
    `list --json` signal. The test-plan `boot` key also says the flag implies `--ui`, a surface not built. So
    `boot --statusline-echo` would land with no consumer; under the mutation rule an effect no test observes is
    left out until its consumer exists (`testing.md`, 2026-09-24).
12. **Path 6's per-instance leg needs two wrappers in one home.** The root fixture gives one wrapper per
    `StampedHome` (`Wrapper::boot` takes it by value). "overseer" appears in root tests only as a `from`
    value. The cross-process form of step 7 rides the harness session; a unit case holds the rule (two gates
    over one `budget.json`).
13. **E5's budget leg cannot run through `revive`.** A revive over a readable snapshot does not take the
    replay, and over an unreadable one is refused `cwd-missing` (`test-plan.md:950`). Root tests already call
    `viola_state` (`grep -rln viola_state tests` → 6 files), so the leg is a root case: the real wrapper writes
    the lines, `viola_state::replay::replay` reads them.
14. **No trycmd case exists** (`grep -rn trycmd tests Cargo.toml` → 0 hits; no `tests/cmd/`). Human lines are
    pinned by unit cases in `src/human.rs` and by root cases that compare stderr and stdout whole.
15. **`run`'s design pair for a home that is not private is not built** (`grep -rn 'not private to you' src`
    → 0 hits). A widened home on `revive` prints the state-files pair (D): no third wording is needed.
16. **On Windows a root case that needs a booted wrapper and an accepted `hook statusline` write starts from
    a stamped home** (`tests/hook_statusline.rs:132-137`, `session`): the fixture's own seeded home is refused
    by the arm's check.
17. **The hand reading behind the third CARRY stands as written** (`evidence/live-statusline.md:6-7`,
    `:95-100`, `:122`): two windows, each a whole-number `used_percentage` and a ten-digit `resets_at`; present
    0.35 s after the first payload in one start. The reader takes both forms of `resets_at`
    (`crates/viola-agent-claude/src/statusline.rs:57`). The gate leans on `budget.json`, not on when the
    payload first carries the object.

## Patterns detected
- **A gate held in the wrapper** (`src/run/wheel.rs:74-128`, `:236-290`): the decision in memory under one
  lock; each change's event and snapshot update on a worker, under a point-in-time span; a method answered
  after its record lands. ≤2 sentences: the budget gate takes this shape, with the event appended through the
  one shared path.
- **A `config.json` reader** (`src/obs.rs:170-206`; `src/run/env.rs:31-67`): each key has its own bounded
  read; a bad value falls back and is reported once as `parse-rejected{parser:"config-json"}`.
- **A refusal rung** (`src/run/send.rs:321-342`): each rung returns through `Ctx::refuse` or
  `Ctx::not_delivered`, before `send-issued`, with no cursor.
- **A check with no log** (`src/cmd/revive.rs:202-208`): `--list` checks, prints the pair, exits 1.
- **A stopped session fed through the arm** (`tests/hook_statusline.rs:74-147`).

## Conventions to follow
- **Wire words are literals in tests** (`tests/cli_wheel.rs:136`): the JSON document is compared whole.
- **A new root test file names its paths `path<N>_<slug>`** and joins `.config/nextest.toml:32`.
- **Additive snapshot fields** are `Option` with `#[serde(default, skip_serializing_if = "Option::is_none")]`
  (`crates/viola-state/src/snapshot.rs:40-41`).
- **One thiserror enum per crate**: a reader error in `viola-state` is a `StateError`.
- **Spans**: `#[instrument(skip_all, name = "…", fields(…))]`, neither `err` nor `ret`
  (`src/run/wheel.rs:262-270`).

## New files to create
- `src/run/budget.rs`
- `tests/cli_budget.rs`
- `tests/support/statusline.rs`

## Files to modify
- `crates/viola-core/src/lib.rs`
- `crates/viola-state/src/budget.rs`
- `crates/viola-state/src/snapshot.rs`
- `crates/viola-state/src/liveness.rs`
- `crates/viola-state/src/strict.rs`
- `crates/viola-state/src/replay.rs`
- `crates/viola-state/tests/state_replay.rs`
- `crates/viola-state/tests/state_events.rs`
- `src/run/mod.rs`
- `src/run/send.rs`
- `src/run/wheel.rs`
- `src/run/dialog.rs`
- `src/run/snapshot.rs`
- `src/cmd/run.rs`
- `src/cmd/release.rs`
- `src/cmd/pause.rs`
- `src/cmd/send.rs`
- `src/cmd/revive.rs`
- `src/cmd/hook.rs`
- `src/cmd/hook/statusline.rs`
- `src/human.rs`
- `src/obs.rs`
- `tests/cli_revive.rs`
- `tests/hook_statusline.rs`
- `tests/cli_controls_not_disableable.rs`
- `tests/support/mod.rs`
- `Cargo.toml` — the `[[test]]` entry of the new root binary
- `.config/nextest.toml`
- `crates/viola-e2e/src/harness/boot.rs` — only if the review card keeps the harness flag
- `crates/viola-e2e/src/bin/viola-harness.rs` — only if the review card keeps the harness flag

## Open questions
- Where is the reordered strict-modes refusal recorded, on `revive`'s start arm and on the statusline arm: the
  stderr pair (or the silent exit 0) with no diagnostics line and the home left as found, or a line whose
  writing narrows the home? → blocks: plan-decision
- Does a window over its threshold with an `unknown` `resets_at` pause, and what does `release --budget` do
  and print when no pause stands? → blocks: plan-decision
- Does an override outlive its wrapper (a restart or a revive inside the crossed window)? → blocks:
  plan-decision
