# P5 review card 1 — 2026-11-16-scheduled-wake-count «Scheduled-wake count on a turn end»

The plan's render to open in an editor: `.andromeda/cache/docs/2026-11-16-scheduled-wake-count/plan.md` (plan rev 2).
No plan stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-scheduled-wake-count`.

## Goal
A Stop's `session_crons` crosses as its length alone, named `scheduled_wakes` on `turn-ended` (the CLI's name mapped
once, in the adapter's Stop arm), beside `background_tasks` and by its rule: `0` for the CLI's empty array, `null`
when absent / `null` / not an array (a non-array also one drift entry), `null` on an API-error end. It reaches
`events.ndjson`, CLI `wait --json` and MCP `wait` — not `last`, `list`, the snapshot or a log line. `wait`'s wake rule,
the wrapper's re-validation, the human output, the ledger and the recorded fixtures are untouched.

## Top acceptance criteria
- Counts: empty `0` · synthetic two `2` · three non-objects `3` · absent/null/non-array `null` (+ one drift entry) ·
  API-error end `null` — matrix#v2-35
- A driver reads it: CLI `wait --json` `scheduled_wakes` `2` then `0`; MCP `wait` `structuredContent` unchanged — matrix#v2-35
- `wait` wakes exactly as before, whatever the count — matrix#v2-35
- No text planted in the array's elements reaches any file under a booted home — matrix#v2-35
- `scheduled_wakes` a known `turn-ended` key (`unknown_fields` 0); no human output changes

## Touchpoints (8, all modify)
`crates/viola-core/src/lib.rs` · `crates/viola-agent-claude/src/hook.rs` · `src/run/wait.rs` · `tests/hook_events.rs` ·
`tests/cli_wait_last.rs` · `tests/cli_wheel.rs` · `crates/viola-state/tests/state_events.rs` ·
`crates/viola-mcp/tests/handlers.rs`

## Test commands (20 entries)
fmt · clippy · 12 nextest entries (6 whole-scope green on the untouched tree, 6 by-name red on the absent cases:
`exit 4, no tests to run`, the red the plan predicts) · the fixtures probe (green; its clone control red on a minted
fixture, `p5-controls.md`) · `run --unit` · `pre-push` · operator: hygiene · push · `ci.py --wait 1200`. Controls
(one-shot `gate.py control`, ids ≤ 16): `wake-count` · `wake-drift` (entry 5, `--timeout 300`) · `wake-key` (entry 9,
300) · `wake-content` (entry 10, 600, the absence case forced on). No mutation run, no live leg, no smoke (no
boot-path touchpoint).

## Concretization preview — matrix#v2-35 (the never-weaken check)
- **Present:** «A driver reading the turn end of a session with scheduled wakes sees how many there are, a session
  with none reads none, no wake's content reaches an event or log line, and wait wakes exactly as it did before.»
- **Proposed:** «A Stop's `session_crons` array crosses as its length alone, as `turn-ended` data `scheduled_wakes`,
  which a driver reads in CLI `wait --json` and MCP `wait`: a two-element array reads `2`
  (`path3_wait_carries_the_scheduled_wake_count_of_a_turn_end`, `handlers_wait_carries_a_scheduled_wake_count_unchanged`);
  the CLI's empty array — every recorded Stop — reads `0`, and an absent, `null` or non-array field and an API-error
  end read `null` (`turn_ended_scheduled_wakes_counts_the_session_crons_array`,
  `turn_ended_scheduled_wakes_is_null_when_absent_or_not_an_array`); no text planted in the array's elements reaches
  any file under a booted home (`hook_a_stop_holding_scheduled_wakes_carries_their_count_and_no_content`); `wait`
  returns a turn end whatever its count and its wake-kind cases pass unchanged
  (`wait_a_turn_end_wakes_whatever_its_scheduled_wakes`). That the live CLI fills the array when a wake is scheduled
  is read-from-source (the 2.1.287 · 2.1.302 · 2.1.325 bundles), not measured.»
- **What moves:** «sees how many there are» is proven on synthetic Stop payloads, the count of the array the CLI
  sends; «a real wake reads 1» is NOT claimed — unmeasured, owned by the CARRY below (`inputs#I2` item 2,
  PROVISIONAL). «none reads none» is concretized as `0` for the CLI's empty array and `null` where the field is absent.

## PROVISIONAL (delegate overseer, `inputs#I2`; the founder away — `founder-forks.md` F1 · F2)
- F1 the wake-firing step's placement: (c), its live half only, one CARRY; the classification step NOT placed.
- F2 the live probe: not run; «a real wake reads 1» unmeasured.
- The field's name `scheduled_wakes` is the overseer's technical answer (`inputs#I2` item 3), not provisional.
- Escalation the wrap meets: the new count crossing the hook→event boundary («Boundary widening», `rul-r74hshif` ·
  `rul-p2zl2kgl`) is answered by the founder's own ruling B, relayed verbatim — given at a route-adaptation dialog,
  not a P4 fork; the wrap reads which form of the escalation it takes (plan, Expected amendments).

## The directive's block (item 9), verbatim where a tool printed it
- The union, per master and its line:
  - `architecture: rows 947 · packs 10 · picked 154 (73 by every draw) · standing 16 · 43643 B`
  - `security-plan: rows 603 · packs 7 · picked 94 (64 by every draw) · standing 15 · 27075 B`
  - `design-system: rows 315 · packs 4 · picked 55 (12 by every draw) · standing 16 · 13558 B`
  - `layout-templates: rows 222 · packs 3 · picked 38 (20 by every draw) · standing 2 · 7079 B`
  - `test-plan: rows 1331 · packs 14 · picked 251 (148 by every draw) · standing 44 · 59026 B`
  - `obs-plan: rows 1024 · packs 11 · picked 159 (74 by every draw) · standing 39 · 40614 B`
  - `a11y-plan: rows 235 · packs 3 · picked 35 (8 by every draw) · standing 14 · 11640 B`
  - `rows-union: 7 master(s) · picked 786 of 4677 · rule any`
- Rows read WHOLE: the sum 202 635 B, under 250 000; each rows file, standing list and history file read once.
- Every row the plan cites is in its master's union rows file (each handle checked against it; three handles first
  written under a twin area's sheet were corrected before the first put).
- `[RETYPED]` lines: `tests-pick-p3 · [RETYPED] line 74: decision/hook-stdin-test-layer is no row of the pack — read as decision/hook-stdin-test-layers, the pack's row 74`;
  `tests-pick-p3.d2 · [RETYPED] line 21: decision/hook-stdin-test-layer is no row of the pack — read as decision/hook-stdin-test-layers, the pack's row 21`.
- Re-spawns: one — `tests-pick-p13 · [ABSENT] .andromeda/runs/2026-11-16T21-47-58-phase/tests-pick-p13.md` →
  `pick-p2-r1`: `VERDICT: 1 of 1 pass · failed: none` (yes 0 of 100). No dead draw.
- Non-zero exits and refusals: the six by-name baseline entries exit 4 (`error: no tests to run`, predicted); the
  bash guard refused three calls (two `cd` moves, one tool output piped into a filter), each re-issued once in the
  guard's form; the stamp-ahead hook refused the plan's estimated `Generated` stamp once, re-written from `date -u`.
  No hook refused a ScheduleWakeup or CronCreate call: none was made, no wake or timer set.
- `UNPARSED` / `INDETERMINATE` / `NOT DERIVED`: none printed.
- Where a letter was wrong or silent:
  - the test-plan history agent wrote amendment ids (`amd-…`) in `marker=`: `check` printed 40 `not a marker` rows
    (14 shown, `… +26 rows in …/fanout-2026-11-16-scheduled-wake-count.json`) and passed; each item names its chunk in
    its prose — read and accepted;
  - research's list rule is silent on a pass-through surface's companion test: the MCP handler test was added at P4
    and research re-put (rev 3);
  - the escalate pattern `rul-bznresq5` keys a founder's answer to a P4 fork; ruling B came from a route-adaptation
    dialog — the letter is silent on that form, left to the wrap;
  - `gate.py show` reads only the trail's last run, so the baseline readings were read from the entries' own logs
    (`$TMPDIR/andromeda-gate/2026-11-16-scheduled-wake-count/phase-2026-11-16T21-47-58/{n}.log`).

## Beyond the plan
- Route (wrap route-resolve, PROVISIONAL delegate overseer `inputs#I2` items 1-2): ONE CARRY on the existing entry
  «Standing live rig with a screen model» — read live whether a FIRED scheduled wake's prompt-submit payload carries the
  schema's optional `source` (`schedule_wakeup` · `loop_wakeup`), on which CLI versions, the raw payload shape captured
  as closed words (the hook drops it today); and read «a real wake reads 1» for this chunk's count (one start, two
  sends, the wrapper stopped before the wake is due). The carry says the CLASSIFICATION step (a ledger row with its
  verify probe, the widening of how a harness prompt is told from a typed one, its place on the route) is NOT placed
  and waits for the founder's own word, and names `inputs#I2`. No new route entry.

Apply? (yes / review / cancel)
