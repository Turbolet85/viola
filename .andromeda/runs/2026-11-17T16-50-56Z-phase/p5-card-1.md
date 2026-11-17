# P5 review card 1 (revision 1) — 2026-11-17-first-readings-on-the-standing-rig «First readings on the standing rig»

The plan's render to open in an editor: `.andromeda/cache/docs/2026-11-17-first-readings-on-the-standing-rig/plan.md`
(rev 8). No plan stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the
chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-first-readings-on-the-standing-rig`.

The changed lines only (inputs#I6 item 7). Everything else stands as approved on the take-up's card 2.

**Why:** implement stopped before the operator pass on entry 14 — `R3` read `user-config: changed`, a host writer
rewriting `~/.claude.json` about once a minute (`evidence/user-config-writer.md`). Word: inputs#I5 (yours), inputs#I6
(the founder's answers, the overseer's technical word).

**Research:** lists synced to the revision's delta (+0 −2), rev 4; kept: none dropped beyond the two; not re-derived
since the take-up's rev 3. The two dropped files are `harness/session.rs` and `bin/viola-harness.rs`; neither was
edited as built. No file outside the lists. The read stayed at the witness's site (research `## Revision 1`, its
**Read:** line).

## Steps 1–11: done, with as-built notes folded in
- `INSTANCE_DEADLINE` is `pub`.
- The rehearsals wait on their own 18 s `WITHIN`. The 12 s one is the root tests', sized under the root `mutants` 15 s
  kill. `rig_rehearsal` is on neither 20 s line (`ci` 30 s × 4; `mutants` `package(viola-e2e)` 15 s × 2).
- `r3b` reads `send-n-wakes: 0`: the fake replays `session_crons: []`.
- The `r3` capture rehearsal ends by its watchdog (`--deadline 10`).
- An over-long capture input writes `{"unread":true,"oversize":true,"ms":n}`.
- `R3`'s `screen-ms` 25411 measured no named frame. It is not carried as a reading.
- Step 11 records the four starts as read. `R3`'s red stands as its disposition: recorded with its measured cause,
  not re-fired and not re-worded to pass (inputs#I6 item 4). Its entry's `note` now says so, and so do entries 13,
  15 and 16's notes (FIRED, green, not re-fired).

## New steps (implement re-enters at 12)
12. **The witness by the rig's own marks** (`isolate.rs`; `start` / `finish` in `mod.rs`). The stamp works as now:
    held → `unchanged`, absent both times → `absent`. When the stamp moved, the rig reads the file ONCE:
    `take(MAX_FRAME + 1)`, a link followed only under `HOME`, absent or unreadable → `unread`. The read searches for
    two needles:
    - the PATH needle: the canonical scratch root;
    - the ID needle: `agent_session_id` from the first `session-start` a wrapped arm (`R2` · `R3B` · `R3`) reads,
      kept only when it passes `is_session_id`.

    An unwrapped arm, or one with no `session-start`, has no id needle: the path needle alone decides, and the
    summary prints `needles: 1`. A needle found → `changed-by-rig` (RED, exit 1); none found → `changed-by-others`.
    Only the word is kept. Unit cases: stamp held · moved with no mark · path mark · id mark · an id-shaped text with
    no id needle · absent or unreadable · a link out of the home.
13. **Rehearsed on a stand-in** the test owns. The no-mark rewrite now reads `changed-by-others`. A new `hold` case
    writes the start's scratch-root path into the stand-in and reads `changed-by-rig`, `needles: 1`, exit 1. `r3b`
    and `r3` read `needles: 2`: the fixture's SessionStart id is session-shaped.
14. **R3's `left-idle` frame** kept at the first screen change after send 1's mark, so a `screen-ms` names its
    frame. Plus `send-1-wakes` on `R3`.
15. **The `R3D` label** maps to `Arm::R3`. `admit` counts it as base, so its entry passes `--cap 5 --spare 3`: the
    four spent starts plus `R3D`, the three spares unspent.
16. **`R3D`, one start, no spare.**
    - What it sends: `/loop Reply with the single word ok, then schedule the next wake in 60 seconds.` on 2.1.325.
    - The dynamic form is UNMEASURED here: `R3B` tried `interval` first, it read 1, and the arm stopped there.
    - Why we expect it to schedule: in the 2.1.325 bundle, ScheduleWakeup is «Schedule when to resume work in /loop
      dynamic mode», Stop's `session_crons` counts ScheduleWakeup wakes, and the schema names its fired `source`
      `loop_wakeup`.
    - If the model calls no ScheduleWakeup: `wake: unfired` at the stand bound, `send-1-wakes` as read (0 or
      `null`) — a reading, never a lost start.
    - Bounds as card 2 revised them: stand 180 s, watchdog 900 s, worst case 662 s, entry `timeout` 1200 s.
17. **`ready-deadline`, made tool-fireable:** new entry 20,
    `cargo build -q --workspace --features viola/fake-agent && cargo nextest run -p viola-e2e -E 'test(boot_that_never_gets_ready_times_out_and_stops_its_supervisor)'`
    (baseline green, 5.78 s). The workspace build gives the tool's clone its `viola-fake-agent`, and the default
    profile has no kill line.
    - Not `run --integration`: its `ci` profile would kill the guard-removed case (20.340 s) as a TIMEOUT.
    - The hand-fired `holds` stays as that firing's record (`evidence/control-2.txt`).

## Test Commands — changed
- Entries 17 (rig census, `round`) · 18 (plain `cargo build --bin viola`, `round`) · 19 (`R3D`, `live`):
  `live R3D --cli 2.1.325 --wake dynamic --stand 180 --deadline 900 --cap 5 --spare 3 …`. It asserts `send-1-wakes:`,
  `needles: 2`, `lacks changed-by-rig`, `lacks user-config: unread` and a whole teardown. Fired by `--live-legs
  --entry 19`.
- Entry 20 as above (non-leg, after the round's span). The operator entries move to 21–24, unchanged.
- Controls:

  | Control | Entry | Timeout | Status |
  |---|---|---|---|
  | `user-config-mark` | 3 | `--timeout 300` | new: mark search removed → the «path mark present» case fails |
  | `ready-deadline` | 20 | `--timeout 1200` | restated (was entry 6) |
  | `capture-keys` · `capture-source` · `wake-capture` | 3 · 3 · 4 | as approved | already read `holds` |

## Acceptance — changed
- `ready-deadline` reads `holds` fired by the tool on entry 20.
- `R3` / `R3-326`: `R3`'s red is recorded with its cause and not re-fired or re-worded; its `screen-ms` is not a
  reading.
- Up to five base starts (`R3D` after the four) and three spares.
- The witness's unit cases are in `rig::`, and the `user-config-mark` control reads `holds`.
- The rehearsals read `changed-by-rig` / `needles: 1` on the stand-in mark, `changed-by-others` on the no-mark
  rewrite, and `needles: 2` on `r3b` / `r3`.
- `R3D`: fired once, clock first, no key, its keys named, a `user-config` word other than `changed-by-rig` and
  `unread`.

## Expected amendments — added
- security-plan ~ `decision/live-rig-isolated-child`: the mark witness. Pattern rul-bznresq5, the founder's own
  answer «Искать в файле следы рига» relayed verbatim (inputs#I6 item 1) — the wrap's answer card.
- test-plan ~ `decision/standing-live-rig`: the witness, `needles`, R3's frame and wake count, `R3D`.

No capability is claimed (unchanged). Validation: step 1 clean — fence 24 entries, planlint 0 hits, `scope sync:
PASS`, `scope lists: parsed 11`, inputs 6 entries (I4 uncited, information only).

## Beyond the plan
- **Re-pin R4:** R4, the live `viola verify --record`, and the record-home rule's live half (cry-ksqqok4x) leave this chunk. They are re-pinned on the existing entry «The 2.1.325 recorded fixture set» as a carry, at the wrap's route-resolve, on the founder's own words (inputs#I2 item 1). The overseer accepted this at review (inputs#I3). (standing — `.andromeda/runs/2026-11-17T14-54-57Z-phase/p5-card-2.md`)
- **An owner for R2's open question:** whether the CLI's placeholder hint is told apart by a cell attribute is unmeasured. The owner is «The 2.1.325 recorded fixture set», carried by the wrap; the overseer accepted it at review (inputs#I3). (standing — `.andromeda/runs/2026-11-17T14-54-57Z-phase/p5-card-2.md`)
- **A carry on «A fired scheduled wake keeps the wheel» (v2-36's owner entry):** a fired CronCreate-scheduled `/loop`
  wake's UserPromptSubmit carries no `source` key on 2.1.325 and 2.1.326 (`R3`, `R3-326`), plus the dynamic form as
  `R3D` reads it. The wrap pins it at its route-resolve; whether that entry is withdrawn or re-scoped is that entry's
  call, on the founder's word. No ledger note: v2-36 is `planned`.

Apply? (yes / review / cancel)
