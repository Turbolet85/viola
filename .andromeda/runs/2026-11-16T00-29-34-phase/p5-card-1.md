# P5 review card 1 — 2026-11-16-wait-wake-condition («`wait` wake condition»)

**The plan to read:** `.andromeda/cache/docs/2026-11-16-wait-wake-condition/plan.md` (rev 4, rendered from the
store).
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `rec.py put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-16-wait-wake-condition` is a
  diff of the chunk's records.

## Goal
Document and pin `wait`'s wake condition as built. F1 is option 1, the delegate's PROVISIONAL answer (`inputs#I5`);
the founder's word is owed.
- `wait` wakes on the event kind alone. So every `turn-ended` returns a parked wait, whatever its `background_tasks`
  and whatever its message asks.
- The CLI's Notification (`activity`) and a `<task-notification>` harness turn (`prompt-submitted`, origin `harness`)
  never return a parked wait.
- What a driver does with a held or scheduled turn end is its own policy. The V150 miss lives in the overseer's
  waiter (`inputs#I3`), and the product's `wait` gives no false wake.
- No product branch, field or event kind is added.

The carried CI red (ci#43773402286 on 7fd86141, `path1_run_start_one_home_in_order` TIMEOUT at the `ci` 20 s kill)
is closed by option (a):
- one `viola verify` stamp per built binary, under a file lock;
- copied into each test's own fresh home, never linked;
- the kill line unchanged.

## Top acceptance criteria
1. `viola wait --json` returns a turn end holding 2 background tasks with its count and message
   (`path3_wait_wakes_on_a_turn_end_holding_background_tasks`). The wrapper's method does the same for counts 0, 1, 3
   and null.
2. A wait armed at a task-free end's cursor stays parked through a Notification and a harness turn, and returns on
   the next turn end (`path3_wait_armed_at_a_task_free_turn_end_parks_through_a_notification_and_a_harness_turn`,
   with a unit twin).
3. MCP `wait` wakes on the same kinds by construction: one wrapper method; the standing parity witness runs in CI.
4. Each case's remove-the-guard control reads `holds`: `held-turn`, `held-turn-e2e`, `no-false-wake`,
   `no-false-wake-e2e`.
5. No stamped-home copy shares an inode with the stamp or another copy. A test that tampers its copy (pinned byte,
   `stamps.json`, mode, lock) leaves the stamp and another copy hash-equal. Controls `copy-not-link` and `one-stamp`
   read `holds`.
6. The class's wall time and slowest test, before (at `df3bad61`) and after, at the class's own load, recorded in
   `evidence/class-wall.json`: `after_max_test_s` < 20, `after_wall_s` < `before_wall_s`, same test count.
7. CI green on the pushed HEAD, `test (ubuntu-latest)` running the red test under the unchanged 20 s kill.

## Touchpoints
- New: `fixtures/fake-scripts/wait-held-turn.json`.
- Modify:
  - `src/run/wait.rs` — test module only;
  - `tests/cli_wait_last.rs`;
  - `tests/support/home.rs`;
  - `tests/support/verify.rs`;
  - `tests/cli_verify/stamps.rs`;
  - `.config/nextest.toml` — a comment.
- No smoke: no boot-path touchpoint changes.

## Test commands (15 entries, baselines on `df3bad61`)
- 1–3 lint: seen green at the last wrap.
- 4 wait unit module: green, 43/43.
- 5 viola-core: green, 39/39.
- 6 the two booted cases by name: red, exit 4, not written yet.
- 7 the two stamp tests by name: red, exit 4, not written yet.
- 8 wait + the stamped-home writers (5 binaries): green, 99/99 in 51.44 s.
- 9 the class record: red, absent. Must-pass `True` and must-fail `False` were read in a clone.
- 10 no recorded fixture changed: green 0. Its control (a minted fixture) is red, last line 1.
- 11 `run --unit` and 12 `pre-push`: seen green.
- 13–15 operator: hygiene · push · `ci.py conclusion --wait 1200`.
- Controls: `.andromeda/runs/2026-11-16T00-29-34-phase/p5-controls.md`. Six one-shot `gate.py control`s at implement
  (step 9).

## Concretization preview — matrix#v2-17 (the never-weaken checkpoint)
- **Current acceptance:** «A waiting driver is woken by a question asked at a turn end that still holds a background
  task and is not woken by a notification right after a task-free turn end; the wake condition is documented; each
  case has a test that fails before the fix.»
- **Proposed acceptance:**
  - «A waiting driver's `wait` — CLI `wait --json` and MCP `wait`, one wrapper method — returns on every `turn-ended`,
    a turn end holding background tasks (`background_tasks` ≥ 1) whose last message asks the driver a question
    included, carrying the count and the message unchanged.
  - A wait armed at a task-free turn end's cursor is not returned by the CLI's Notification (`activity`) or a
    background task's `<task-notification>` harness turn (`prompt-submitted`, origin `harness`), only by the next
    wake kind.
  - The wake condition is documented in the masters, naming the driver's side: what to do with a held or scheduled
    turn end is the driver's policy.
  - Each case has a test over the fake agent. «A test that fails before the fix» is read as each case's
    remove-the-guard control (the wake set without `turn-ended`; the wake set with `activity` and
    `prompt-submitted`), because `wait` as built already holds both cases. This reading is PROVISIONAL, the delegate's
    answer (`inputs#I5`), with the founder's word owed.»
- **What moved, said in so many words:** the cap read a product defect to fix. P3 measured none. The «fails before»
  test becomes the control's red with the guard removed, not a red on today's tree. Nothing else is narrowed: both
  clauses are still asserted end to end, over the fake agent.

## The run's record (`inputs#I1` item 7)

### The row read's tool lines, verbatim
```
architecture: rows 943 · packs 10 · picked 203 (95 by every draw) · standing 17 · 55619 B
security-plan: rows 599 · packs 6 · picked 182 (117 by every draw) · standing 5 · 46878 B
design-system: rows 314 · packs 4 · picked 107 (64 by every draw) · standing 10 · 25777 B
layout-templates: rows 221 · packs 3 · picked 57 (29 by every draw) · standing 0 · 11713 B
test-plan: rows 1328 · packs 14 · picked 229 (134 by every draw) · standing 48 · 52988 B
obs-plan: rows 1021 · packs 11 · picked 227 (91 by every draw) · standing 50 · 57660 B
a11y-plan: rows 235 · packs 3 · picked 40 (22 by every draw) · standing 8 · 11445 B
rows-union: 7 master(s) · picked 1045 of 4661 · rule any
```

### How the rows were read
- **By search.** The union sums 262 080 B, past 250 000 B. Every term the steps and criteria rest on was grepped in
  each rows file.
- **Every cited row is in the union.** Each handle the plan cites was found in a rows file by that search:
  - architecture — mechanism-not-policy, the wake-kind contracts, notification, background-tasks,
    wrapper-events-log-only;
  - test-plan — the wake, non-waking-records, fake-script, fresh-home, state-files, verify-sole-stamp-writer, the two
    kill rows, rstest chain, bounded-remove-the-guard-witness and cli-mcp-identical-payloads rows;
  - security-plan — stop-background-tasks-count-only, upstream-text-is-content;
  - obs-plan — channel-response-outcome.

  Standing lists were read whole; the steps keep their lines.

### Batches
- **`[RETYPED]` lines:** one, `obs-pick-p2.d3 · [RETYPED] line 27: decision/no-telemetry-from-screen-content is no row
  of the pack — read as decision/no-telemetry-from-screen, the pack's row 27`.
- **Re-spawns:** none. All batches passed first try: task 7, pick 153 in 3 parts, history 7. `draws-dead:0`.

### Non-zero exits, refusals and UNPARSED lines
- **Bash-guard refusals (5), each re-issued once in the passing form:**
  - a `cd` into the skills dir (P1);
  - two `cd` into the run dir to read the standing lists (P2);
  - a `cd` into the run dir to search the rows files (P4);
  - a pipe of `gate.py run` into `grep` (P5).
- **P3, host shell:**
  - `bc` absent, so the timing script was redone in shell integers;
  - `timeout command grep` exited 127, re-run with `/usr/bin/grep`;
  - the first bundle grep passed 120 s and was moved to the background.
- **P4:** the stamp-ahead hook refused an estimated `Generated:` stamp, re-read from `date -u`. planlint check 2 had
  1 HIT (a `matrix#v2-17 notes` wrap amendment on a planned cap); it was moved into the P5 claim's `--note-file`.
- **P5:** `rec.py put` of plan rev 3 printed `UNPARSED: Expected newline or end of document after a statement (at
  line 17, column 65)` and exit 3, and landed the rev anyway. The cause was my doubled `''` inside a TOML-literal
  baseline. `gate.py run --dry-run` and `gate.py baseline` printed the same line. Fixed as a basic string; rev 4
  parses.
- **Baseline reds as predicted:** entries 6 and 7 exit 4 (no tests yet), entry 9 exit 1 (record absent).
- **No `INDETERMINATE` or `NOT DERIVED` line** was printed by any tool.

### Places a letter was wrong or silent
- `tests/support/home.rs` and `tests/cli_wait_last.rs`, first named at P3, were judged the scope's own surfaces, and
  the row read was not re-run. The delegate agreed (`inputs#I5`).
- **`tests/cli_verify/stamps.rs`** was first named at P4, after that agreement (the stamp-isolation tests' file). I
  judged it the same surface, the stamped-home test support, and did not re-run the row read. **This judgment is not
  yet covered by the delegate's word.**
- **`rec.py put` lands a plan whose fence does not parse.** It exits 3 with an `UNPARSED` note, rather than refusing.
  The letters do not say which is meant.
- **The stamp's row count.** The fake-agent stamp at 2.1.287 on the HEAD build reads `21 pass  0 fail`.
  `verification-harness.md` and history say `LedgerRow::relied()` is 24 on every build. The fixture judges
  `pass  0 fail` only, so nothing here turns on it; it is named for the wrap.
- **Setup 5a's CI read** reached only `df3bad61` (green). The open red on `7fd86141` arrived by the entry's CARRY,
  not by that read. The promotion letter's CI-fold rule covers only 5a's range.

## Beyond the plan
1. **Candidate for the founder: carry `session_crons` as a count** (F1 option 2, not taken).
   - What a driver gains: it sees from the turn end that the CLI has a wake scheduled, so it does not treat that turn
     end as the session's last word.
   - What crosses: one count, no content.
   - Where it would land: `crates/viola-agent-claude/src/hook.rs`; `turn-ended` data keys 3 → 4; architecture,
     security-plan and obs-plan rows with Decisions Log entries.
   - Owner: the founder's own word. It stands in
     `.andromeda/runs/2026-11-16T00-29-34-phase/founder-forks.md`. No route entry is minted.
2. **The V150 fix in the overseer's waiter** (`pwait.sh`, `inputs#I3`): wake on every turn end, or read the message,
   in place of re-parking on `background_tasks` ≠ 0.
   - Owner: the overseer, in its own repository. It is outside viola's route and lands in no viola entry.

Apply? (yes / review / cancel)
