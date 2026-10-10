# Scope — 2026-10-10-budget-governor · Budget governor

**Working entry** (`viola-0.1.0/working-route.md:119`, the head of the markerless tail, in
`### Epoch 4 — Session state & governance`): Budget governor — five-hour/seven-day thresholds, budget-paused on
automated send only, lift at reset or per-instance override, unknown with age, release --budget payload — plus
four CARRY blocks (folded below; completeness check `route.py pins` → the run dir's trail: four `:119` rows, 477,
1333, 940 and 571 chars, no abstention). The entry carries no `BLOCKED-ON`, no `PREREQ` and no `WATCH`: the
chunk has no acceptance gate.

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- no mutation run, no witness run and no workflow dispatch is planned;
- the `rate_limits` ledger row, its `viola verify` probe, a re-stamp and any live `claude` start are the
  founder's, and no cap is given. The fake agent rehearses. The review card states, each option priced, what the
  entry needs of them now, as steps that wait for his word;
- the `viola revive` check-order defect is in scope, with its own acceptance and a witness that fails on the
  present order; the statusline arm's one-invocation hypothesis is measured and what it shows is folded;
- a widening goes on the review card as itself, unsettled;
- if the four CARRYs do not fit, the review card proposes the cut;
- the operator CI read at the end is the `ci.py conclusion` tool call.

**How a founder matter is planned** (`inputs#I2`): a founder ruling stands until his own word. Work that departs
from one, or that only he can open, is planned up to a stop rule that names what counts as his word; it is never
a decisive lean and never a confirm item.

**CI at take-up** (Setup 5a; the last wrap's flip through HEAD is one sha): `396c108` reads green, 15/15 checks,
`ci#38084587357`, wall 462 s. Nothing red or not-green is carried.

**As landed at HEAD `396c108`** (read at P1, re-read at P3; research.md, Files inspected):
- the reading is written and nothing reads it: `viola_state::budget` holds `write_budget` alone
  (`crates/viola-state/src/budget.rs:44`), called by `hook statusline` (`src/cmd/hook/statusline.rs:141`);
- the gate is a constant: the start writes `budget_paused: false` into the snapshot (`src/cmd/run.rs:519`) and
  appends `budget-gate{paused:false}` (`src/cmd/run.rs:534`), and nothing else appends that kind;
- `release` takes a boolean `budget` and answers the constant `budget_paused: false`
  (`src/run/wheel.rs:202-209`); the CLI verb has no `--budget` flag (`src/cmd/release.rs:20-27`);
- the refusal word exists with no producer: `RefusalReason::BudgetPaused` maps to exit 11
  (`src/cmd/send.rs:90`), no `send` path returns it, and no hint is printed for it (`src/human.rs:209`);
- `config.json` is read for `claude_env_keep` (`src/run/env.rs:34`) and `diagnostics_level` (`src/obs.rs:172`)
  only: no threshold reader exists, and no `Percent` type exists in any crate;
- `EventKind::BudgetGate` admits the three data keys `paused`, `window`, `override_until`
  (`crates/viola-core/src/lib.rs:108`), and the replay reads the last `budget-gate` line
  (`crates/viola-state/src/replay.rs:59`); the snapshot has no `budget_override_until` field
  (`crates/viola-state/src/snapshot.rs:37-66`);
- the harness has no statusline code and no `harness_session` fixture; the fake agent's `statusline-echo` mode
  and the three root helpers exist (`src/bin/viola-fake-agent.rs:38`, `tests/support/home.rs:272`, `:281`,
  `:293`).

**What research changed (P3).** Seven things the entry does not say:
- both order claims are measured: the statusline arm's refusal lasts one invocation, and `revive` over a
  group-writable home narrows the home and refuses for a later cause (items 7 and 8);
- the reordered refusal has no carrier that keeps both its log line and the home as found (item 7);
- `--budget` is not a CLI flag yet, and `Percent` does not exist (items 1 and 4);
- the clock seam gives no wall time, so the decision takes the instant as an argument (item 2);
- three behaviours have no master line (items 4 and 5);
- the harness flag has no consumer in this chunk, and is proposed as the cut (items 9 and 11);
- no capability is provable whole by this chunk (the capability section).

## What this chunk builds

### 1. The thresholds
- Source: the entry's title ("five-hour/seven-day thresholds"); architecture §Established Decisions, [Budget
  Governor] (`architecture.md:72`): defaults `five_hour` ≥ 90 %, `seven_day` ≥ 85 %.
- Verified (the arch and security extracts; `architecture.md:399`, `:464`; `security-plan.md:237`): the
  thresholds are read from `config.json` as `Percent` (0 to 100), with those defaults, through the `MAX_FRAME`
  cap, `v` = 1, unknown keys counted.
- [premise-corrected: research.md finding 2 — `grep -rn Percent --include=*.rs crates src` → 0 hits] `Percent`
  is minted here, in `viola-core`, as a bounded newtype over a whole number from 0 to 100.
- [premise-corrected: research.md findings 7 and 8 — `architecture.md:399` gives the rule, the masters give no
  nesting] The keys are top-level `five_hour` and `seven_day`, as the masters spell them. A value that is not a
  whole number from 0 to 100 (`101`, `-1`, `"90"`) falls back to that key's default and is reported once as
  `parse-rejected{parser:"config-json", detail:"malformed"}`; the start is not refused. No value disables the
  gate (`test-plan.md:641`, Vector 6, a case in `tests/cli_controls_not_disableable.rs`).
- Verified (`test-plan.md:582`): `five_hour` 89 is open and 90 is paused; `seven_day` 84 is open and 85 is
  paused.

### 2. The gate: evaluated from `budget.json` at three points
- Source: `architecture.md:72`. The wrapper evaluates its gate from `budget.json`, read under
  `budget.json.lock`, at start, when a `send` arrives and at each `turn-ended`. It never polls. A crossed
  threshold, a passed `resets_at` or an expired override takes effect at the next of those points, and only then
  is `budget-gate` appended.
- Verified (the arch extract's first constraint; research.md finding 1): the read of `budget.json` is a new
  `viola-state` function beside `write_budget`, under the same lock sibling, through `Read::take(MAX_FRAME)`,
  never writing. An absent, over-cap, unparseable or newer file reads as no reading. Whether the wrapper may
  refuse a driver on that file without a ruling is item 10.
- The wrapper mirrors its budget-pause state, any override included, to the instance snapshot
  (`budget_paused`, `budget_override_until`), where `list` and `ui` will read it (`architecture.md:72`, `:211`,
  `:255`). `budget_override_until` is a new, additive, optional snapshot field.
- The gate declares the span `run.budget_eval` with `trigger` (`start` · `send` · `turn-ended`), `paused` and
  `window` (the obs extract's first constraint). A gate change gets no process-log event of its own: the
  `budget-gate` record is its carrier (the obs extract's fourth constraint).
- [premise-corrected: research.md finding 3 — `viola_core::Clock::now()` is an `Instant`, `viola-core` has no
  chrono, no crate depends on mock_instant] The decision (reading, thresholds, override, the wall-clock instant
  → open, or paused with its window) is a pure function in the root bin that takes the instant as an argument.
  The product passes the wall clock at each of the three points; a unit case passes a literal. The override's
  expiry is a unit case (`test-plan.md:582`, `:845`).
- A window whose `resets_at` has passed does not pause: the reading is of a window that has ended
  (`architecture.md:72`, "the pause lifts when `resets_at` passes").

### 3. The refusal: `budget-paused` on an automated send only
- Source: the entry's title; `architecture.md:72`, `:129-142`. An automated `send` is refused `budget-paused`
  with detail `five-hour` or `seven-day`, client exit 11, through the same refusal path as `human-typing`. Human
  keys are never blocked and a running turn finishes.
- Order (`test-plan.md:590`): control-character → empty-text → human-typing → budget-paused → turn-running →
  input-not-ready → no-prompt-submitted; the wrapper returns only the first that applies. The second read after
  the readiness gate's wait names `human-typing` and `turn-running` only: no second budget read (the arch
  extract's third constraint).
- The refused send appends `send-refused` with no cursor and writes one process-log line with the fields
  obs-plan names (`window`, `used_percentage`, `resets_at`, `paused:true`; the schema admits all four,
  research.md finding 6).
- The human-mode refusal prints the struck-box `unable` line and one `hint:` line, the design's text
  (`design-system.md:760`); the hint never contains `release` (`test-plan.md:862`, `:996`).
- Verified (the arch extract's third constraint): `wait`, `last`, `answer`, `pause` and `hook.dialog` are not
  gated.
- When both windows are over their thresholds, the detail names one: `five-hour` first. No master line orders
  them; the review card lists it.

### 4. The lift: at the window's reset, or by a per-instance override
- Source: the entry's title ("lift at reset or per-instance override", "release --budget payload");
  `architecture.md:72`. The pause lifts when the crossed window's `resets_at` passes, or on
  `viola release --budget`, which goes to one instance's wrapper and lifts the pause for that instance only,
  until the crossed window's `resets_at` passes, so the next over-threshold reading does not re-pause it.
  `release --budget` leaves the wheel where it is; a plain `release` returns the wheel and lifts no pause.
- The override is logged as `budget-gate{paused:false, override_until}` and stands in the snapshot as
  `budget_override_until`; the record's `override_until` is later than its `ts` and equals the snapshot's
  (`test-plan.md:858-860`).
- [premise-corrected: research.md finding 4 — `ReleaseArgs` holds `name` and `json` only] The CLI flag
  `--budget` is built here: the contract lists it (`architecture.md:285`, `:362`) and the verb does not have it.
- The result of `release --budget` is `{budget_paused:false}` beside the wheel (`test-plan.md:853`); its human
  line names the instance, `budget gate lifted until` with the override's end as a UTC clock time, and the
  wheel holder (the example line at `design-system.md:803`; `layout-templates.md:456`).
- Verified as additive (the arch extract's fourth constraint: an added field needs no `v` bump): the machine
  result carries the override's end as `override_until`, so the human line is derivable from it (requirements
  `v1-13`).
- Answered at P4 (`inputs#I3`, question 3; no master line says, research.md finding 9): when no pause stands,
  `release --budget` records nothing: no override, no `budget-gate` line, no snapshot change. Its result is
  `{wheel, budget_paused:false}` with no `override_until`, exit 0. Its human line has no words in any master:
  the wording goes on the review card for the founder, and the plan holds a planning form only.
- An override covers the instance's whole gate until its end, whichever window is crossed next
  (`architecture.md:72`: "so the next over-threshold reading does not re-pause it"). The review card lists it.
- `release` carrying a string `from` stays refused `-32602` with or without `budget`, before any lift (the
  security extract's fifth constraint; landed, `src/run/wheel.rs:197-200`).

### 5. Unknown, with its age
- Source: the entry's title ("unknown with age"); `architecture.md:72`: with figures missing, viola uses the
  newest reading from any wrapped session and shows its age; with no reading at all it records `unknown` and
  does not block.
- Verified (the arch extract; `architecture.md:216`): an `unknown` window, an `unknown` `used_percentage` and an
  absent `budget.json` never pause; `read_at` is the age's source. The surfaces that show the reading and its
  age (`list`, `/api/sessions`, the ATIS) are other entries' (below).
- Answered at P4 (`inputs#I3`, question 2; no master line says, research.md finding 9): a window is judged
  only when both its figures are known. A window over its threshold with an `unknown` `resets_at` does not
  pause, so no override ever has an undefined end. The cost goes on the review card: a drift in the CLI's
  `resets_at` shape opens the gate for that window.

### 6. The `budget-gate` writer, and what the replay reads
- Source: the first CARRY (chunk 2026-10-10-self-healing-state, the operator's direction at its wrap): the log
  replay reads `budget_paused` and `budget_override_until` from the last `budget-gate` line, and no writer writes
  `override_until` or `window` yet; this entry lands that writer; the reader already admits the three keys.
- Re-verified at P1: `crates/viola-state/src/replay.rs:59`; `crates/viola-core/src/lib.rs:108`;
  `src/cmd/run.rs:534` is the only product append of the kind.
- `budget-gate` is `{paused, window?, override_until?}` with `window` `five-hour` · `seven-day`, appended by the
  wrapper once at start and on every change (`architecture.md:303`, `:308`). The start order
  `wheel{cause:"start"}` → `budget-gate` → `session-start` stands (`test-plan.md:228`, `:728`): the start
  evaluation appends exactly one `budget-gate`, as record two, whatever it reads.
- The CARRY states: test-plan §6 Scenario E5's budget pause is owed here.
- Verified (the tests extract; `test-plan.md:950`): E5 as a whole is not buildable here: it still needs `link`
  ("Session links") and `list --json` ("The board: viola list"), and two of its signals are set against what
  landed.
- [premise-corrected: research.md finding 13 — a revive over a readable snapshot does not take the replay, and
  over an unreadable one is refused `cwd-missing`] The budget leg is not shown through `revive`. It is a root
  case: the real wrapper writes a pause and then an override, and `viola_state::replay::replay` over that
  instance directory reads `budget_paused` and the `budget_override_until` the override wrote, with zero on all
  three skip counts.
- Answered at P4 (`inputs#I3`, question 4; no master line says, research.md finding 9): an override does not
  outlive its wrapper. Every start, a revived one included, evaluates from `budget.json` alone, so a restart
  inside the crossed window pauses again. No snapshot read is added to the start.

### 7. The `viola revive` check order (a defect in shipped code)
- Source: the second CARRY (chunk 2026-10-10-statusline-pass-through, the operator's direction at its wrap: a
  defect in shipped code goes into the next chunk, as an item with its own acceptance and witness) and
  `inputs#I1` note 3.
- Verified by measurement (research.md, Measured at P3, runs C and D): over a home at 0770 with the instance
  directory owner-only, `viola revive` left the home at 0700 and refused `no-session`; over a home at 0700 with
  the instance directory at 0770 it refused `strict-modes-failed`. The mechanism is the role file's opening
  (`src/obs.rs:77`), reached from `open_wrapper_log` at `src/cmd/revive.rs:101`, ahead of `preflight` at `:102`.
- Acceptance, as the CARRY words it: a `viola revive` over a group-writable home is refused
  `strict-modes-failed`, exit 1, no child started.
- Witness, as the CARRY words it: a case that fails on today's order and passes on the fixed one
  (`tests/cli_revive.rs` widens the instance directory today, not the home).
- The fix moves the instance check ahead of the log's opening on `revive`'s start arm. The refusal prints the
  state-files pair that stands (run D; no third wording, research.md finding 15).
- [premise-corrected: research.md finding 10 — the statusline arm's way of logging a refused check is what
  narrows the home (run A1)] The arm is no model for the refusal's log line. The `process-exit` line for
  `strict-modes-failed` on this arm is contracted in four masters; obs-plan §3 step 4 says no diagnostics line
  for a refused home; `revive --list` is the landed form with no line.
- Answered at P4 (`inputs#I3`, question 1, the operator's note: "A refusal that lasts one invocation is no
  refusal."): a refused check writes no log line and opens nothing under the home. `revive` prints its
  state-files pair and exits 1; the home's mode stays as found. The contracted line is dropped on this arm, an
  amendment for the wrap in the four masters that state it.
- The masters name this entry as the order's owner (architecture §Occupied Resources, the instance files;
  security-plan §Authentication & Authorization; `.claude/rules/security.md`).

### 8. The statusline arm's refusal: measured, and folded as a fix
- Source: the second CARRY and `inputs#I1` note 3.
- Verified by measurement (research.md, Measured at P3, runs A1, A2 and B): the CARRY's hypothesis holds. The
  refused invocation opened its role file, the home read 0700 afterwards, and the next invocation over the same
  home wrote `budget.json`.
- So the arm's fix is an item of this chunk, with its own acceptance and witness. Acceptance: a `viola hook
  statusline` over a group-writable home runs no command, prints nothing, leaves `budget.json` as it was and
  exits 0, on the first invocation and on the next one, and the home's mode is unchanged after both. Witness: a
  `cfg(unix)` case of two invocations over one widened home that fails on today's arm.
- Answered at P4 with item 7 (`inputs#I3`, question 1): the refused arm writes no line. The contracted
  `hook-decision{detail:"strict-modes-failed"}` is the line whose writing narrows the home, and it is dropped.
  The operator's note binds the review card: it states for the founder that a refused check on this arm then
  leaves no trace anywhere, and says where an operator would learn of it.
- Verified (research.md, Graph impact, `viola_obs_init`): the other verbs that open a role file before any
  check are `client::start` (`send`, `wait`, `last`, `answer`, `pause`, `release`), `hook`'s nine events,
  `verify` and `run`. None runs an instance check today. They stay with the Epoch 6 entry "Home and
  code-bearing file integrity"; this chunk fixes `revive`'s start arm and the statusline arm.

### 9. Path 6, as far as this chunk's surfaces reach
- Source: test-plan §6 Path 6 (`test-plan.md:835-863`); the fourth CARRY names this entry's Path 6 test.
- Path 6's cli and ipc signals land as root cases on a planted source: a reading fed through `viola hook
  statusline`, the refused `send` with its detail and exit, the `budget-gate` record, the boundary values, the
  malformed `resets_at`, `release --budget` and the next `send`, and the human-mode hint.
- [premise-corrected: research.md finding 12 — the root fixture gives one wrapper per home] Step 7's
  cross-process form (two instances in one session) is not built here. A unit case holds its rule: two gates
  over one `budget.json`, an override on one.
- Its `/api/sessions`, `list --json` and Playwright signals wait for the entries that build those surfaces. The
  landing is recorded as partial.

## What waits for the founder's word (planned as steps behind a stop rule; the rule is lifted by the revision below)

### 10. The `rate_limits` ledger row, its probe, and what the gate trusts
- Source: the third CARRY (chunk 2026-10-10-statusline-pass-through; the founder's word on that chunk, live,
  2026-10-10, relayed verbatim by the operator: a hand reading of three live starts and no row): the
  `rate_limits` ledger row and its `viola verify` probe are owed here, where the first `send` refusal relies on
  the shape.
- Verified (research.md finding 17; that chunk's `evidence/live-statusline.md:6-7`, `:95-100`): the hand
  reading says what the CARRY says: "the payload's `rate_limits` object with `five_hour` and `seven_day`, each a
  whole-number `used_percentage` and a whole-number `resets_at` that reads as epoch seconds (read by hand on
  2.1.287 on the Linux dev host)".
- The CARRY's "hypothesis: `rate_limits` arrives from the second payload of a session on" stays a hypothesis
  (one live start, `evidence/live-statusline.md:122`, `:136`). Nothing here leans on it: the gate reads
  `budget.json`, whenever it was written.
- The CARRY states as not measured: an account without `rate_limits`, and the `spend_limit` window.
- Bound by `inputs#I1` note 2: the row, its probe, a re-stamp and any live `claude` start are the founder's.
  None is built or run on this plan's own authority. The plan builds and rehearses under the fake agent, and
  the review card states, each option priced, what the entry needs of them now and how many live starts, as
  steps that wait for his word.
- Verified (the arch and security histories): a `send` refusal that relies on the shape is outside the
  founder's ruled limit ("a shape no `send` relies on needs no probe"), and an eighteenth row turns every home
  stamped at seventeen unverified until a re-stamp. What the gate does on a CLI build with no row for the shape
  is his. It goes on the card, never leaned.
- **Widenings, on the card as themselves, unsettled** (`inputs#I1` note 4), each confirmed at P3:
  - the wrapper refuses a driver on `budget.json`, a file a hook arm writes, that the trusted-file list does not
    hold and that no ruling covers (the security extract's third constraint; the stamps read of 2026-09-28 is
    the same class and was the founder's);
  - what `revive`'s start arm and the statusline arm do on a refused check (item 7's question): either answer
    departs from a sentence in the masters.

## The revision: the operator pass no longer waits (`inputs#I6`)
- Source: the operator's word at this revision's invocation (`inputs#I6`, verbatim in the snapshot), given after
  /andromeda-implement ended at the plan's step 15 with steps 1 to 14 built. Basis, the implement run dir's
  trail `gate-2026-10-10-budget-governor.json`: its two `run` calls each read `entries 16 · green 13 · red 0`
  with `not-run 3` (the three operator-leg entries), and its last `scope` call reads `scope: clean — changed
  31 · listed 31`.
- Verified against the snapshot: the operator pass (the hygiene entry, the pre-CI commit, the push, the CI read)
  no longer waits for the founder's answer. The stop rule item 10 was planned behind is lifted.
- Verified against the snapshot and against the tree: the four held items land in their planning forms, as
  built:
  - the `budget-paused` refusal flows with no `rate_limits` ledger row (the rung, `src/run/send.rs:347-350`;
    `git diff --quiet 396c108ba64c -- crates/viola-agent-claude src/cmd/verify.rs src/cmd/verify fixtures` exits
    0: no row, no probe and no fixture moved);
  - the wrapper refusing a driver on `budget.json` lands as built (`read_budget` and its two `check_path` reads,
    `crates/viola-state/src/budget.rs:77-82`), and the wrap records it as NOT ratified, the founder's word owed,
    never as ratified;
  - a refused check writes no line (items 7 and 8; `evidence/red-green.md`, readings (a) and (b));
  - the two human lines as the plan's step 9 words them (`write_budget_open`, `src/human.rs:216-219`; the date
    ahead of the time for an end on another UTC date, `override_end`, `src/human.rs:192-203`).
- Verified against the snapshot: each of the four is carried on the handoff for the founder, and an answer of
  his that differs comes later as a revision or a new chunk.
- Verified against the snapshot: the revision changes the plan's step 15 and nothing else: no other step, gate,
  criterion or touchpoint, and no built file.
- What the word does not change (`inputs#I1` note 2 stands): no ledger row, no `viola verify` probe, no re-stamp
  and no live `claude` start is built or run by this chunk.
- The word is the operator's. It ratifies none of the four: the playbook's rule "Boundary widening — what
  ratifies it" (`.andromeda/playbook.md:48-50`) keeps the ratification of the `budget.json` widening with the
  founder, and no line of this scope reads any of the four as answered by him.
- Left open by the word, for this chunk's wrap: the third CARRY placed the `rate_limits` row and its probe on
  this entry, and they are not built here, so they need a route owner the word does not name.
- Confirmed at the revision's P5 review (`inputs#I7`, the operator's word): the revision is applied as it is.
  The hold is not restored for the `budget.json` item, which lands as built and not ratified, the founder's
  word owed. The plan's sentence on the owed `rate_limits` row stays. The seven sentences elsewhere in the plan
  that still word the old hold stay as they are, governed by the closing clause of step 15.

## What this chunk may not fit (the cut)

### 11. The cut, proposed at the review card
- `inputs#I1` note 5: four CARRYs ride this entry; if they do not fit one builder window, the card proposes the
  cut, each left-out part to its real first consumer.
- [premise-corrected: research.md finding 11 — no `harness_session` fixture, no `path_` binary, `run --e2e`
  unbuilt and pinned on "The board: viola list", the `boot` key's `--ui` implication on an unbuilt surface] The
  proposed cut is the fourth CARRY: `agent-run boot --statusline-echo` and the harness-session form of Path 6
  go to the route entry "The board: viola list", which lands the first `path_` E2E binary and owns Path 6's
  `list --json` signal. In this chunk the flag would have no consumer. No `statusline-source-unresolved`
  refusal is built by either entry: its premise is gone.
- [premise-corrected: `inputs#I6` — the operator's later word lifts the hold on the operator pass] Confirmed at
  the P5 review (`inputs#I4`, the operator's word): the cut stands as proposed and the plan's leans are
  confirmed. That word's last clause, the four founder items going to him before the operator pass, is
  superseded by the revision (above): they are carried to him on the handoff, and the pass does not wait.

## What is owed elsewhere
- The `budget` field of `list`, the board's `driver · budget-paused` cell and the BAY line: "The board: viola
  list". The `/api/sessions` envelope `budget` and the ATIS: the Epoch 8 entries. This chunk writes the
  snapshot fields they read.
- The MCP form of the refusal (`architecture.md:318-319`): "MCP server for drivers".
- `link`, and E5's link leg: "Session links".
- The Windows statusline half (no override, no command, so no reading from a live Windows session):
  "Windows-only live measurements". The gate itself reads `budget.json` on every OS.
- The home strict-modes check at every entry point and the Windows case of `check_instance`: "Home and
  code-bearing file integrity".
- The settings-override and Unix-shell ledger rows and the real-CLI readings not taken: "Paste newline ledger
  row".
- The `spend_limit` window: architecture knows two windows.

## The capability
- `v1-35` (requirements.md:53) names this entry's subject. Its acceptance has two clauses this chunk cannot
  prove: readings recorded "without touching the user's statusline configuration or output" holds on Unix only
  (the Windows half is cut to "Windows-only live measurements"), and "missing readings show as `unknown` with
  their age" needs a showing surface (`list`, the ATIS). Advanced here, not claimed.
- `v1-13` (requirements.md:25) names `release --budget`'s result payload beside `link`'s: a leg here, not the
  whole.
- `v1-45` (home integrity) is advanced by items 7 and 8 and proven by a later entry.

## Boundaries
- No mutation run, no witness run, no workflow dispatch; no ledger row, verify child, re-stamp or live `claude`
  start without the founder's word (`inputs#I1`).
- No new env var or channel method; no config key beyond the two thresholds, and neither can disable the gate.
  `--budget` is the contract's flag, built here. No poll: the gate reads at its three points only.
- The human always wins: no human key is blocked, refused or delayed by the gate, and a running turn finishes.
- `viola release` stays a human verb: no driver-facing hint, MCP tool or doc suggests it.
- Universal invariants that bound every item: refusals carry codes and fixed messages; 0700 dirs and 0600 files
  set explicitly; every replaced file through `replace_private`; every external reader through
  `Read::take(MAX_FRAME)`; only the wrapper writes `snapshot.json`; one `write` per ndjson line and
  `events.ndjson` never truncated; stdout is reserved; logging only through `obs_event!`; Tokio only in
  `viola-mcp` / `viola-ui`; each relied-on CLI behaviour is a ledger row with a probe.
- A planned test ends only processes it started.
