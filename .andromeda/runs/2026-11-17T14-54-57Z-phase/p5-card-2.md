# P5 review card 2 — 2026-11-17-first-readings-on-the-standing-rig «First readings on the standing rig»

This card answers the overseer's review of card 1 (`.andromeda/runs/2026-11-17T14-54-57Z-phase/p5-card-1.md`),
snapped `inputs#I3`. It was a technical review with no founder fork in it. The plan is now rev 4, research rev 3.
The plan's render to open in an editor is `.andromeda/cache/docs/2026-11-17-first-readings-on-the-standing-rig/plan.md`.
No plan stands in a chunk's folder; writing in that file changes no record until a `put`. A diff of the chunk's
records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-first-readings-on-the-standing-rig`.

## What the review changed

### 1. `R3-326` on a transport-only wrapper, read from the product's code
The review's hypothesis does not hold at its premise. A transport-only wrapper still registers its hooks:
- `pin_and_plugin` writes `plugin/<key>/` and the child takes `--plugin-dir` before and apart from the version gate (`src/cmd/run.rs:281-290`).
- So `session-start`, `prompt-submitted` and `turn-ended` are written on 2.1.326 too, if that CLI runs `--plugin-dir` hooks. That is unmeasured.
- `viola send` delivers there. With no signature set the gate reads a quiet screen `Ready` (`crates/viola-agent-claude/src/screen.rs:190-191`).
- A non-local text confirms by its `prompt-submitted` (`src/run/send/classify.rs:27-33`).
- Only dialog decisions and the screen-signature reads are lost.

The review's remedy stands as the fallback for the unmeasured part. Step 6 now does this:
- **No refusal:** a wake arm never refuses on a missing `session-start`. It records `session-start: seen|absent` and goes on.
- **A bound for each turn:** every `turn-ended` wait has a named bound, `WAKE_TURN_BOUND` 120 s. Where it runs out the arm records `send-<n>-turn: unended`.
- **Either sign of the fired wake:** the wrapper's `prompt-submitted` with no send in flight, or a capture line past send 1's mark. The summary prints `wake-sign: event|capture|none`.
- **The capture keys always print:** `capture-lines`, `capture-source` and `capture-keys` come from `capture.ndjson` whatever `wake` reads, the stand bound included. They read `unread` only on an empty file.
- **A new rehearsal (step 9):** the test waits for `send-1.mark`, then appends one synthetic `loop_wakeup` capture line. The case reads `wake: fired` and `wake-sign: capture`. Without the line, the start reads `unfired`.
- **A new control:** `wake-capture`, on entry 4, `--timeout 600`. With the capture sign's condition set to `false`, the case reads `unfired` and fails (predicted).

### 2. Send 1's own capture lines
- **The mark:** the arm takes the line count of `capture.ndjson` at send 1's `turn-ended`, or at its 120 s bound where none comes. It writes the count to `<scratch root>/send-1.mark`.
- **What it prints:** `send-1-capture`, the count of those lines, and `send-1-source`, their `source` words in order. A typed prompt reads `other` under the ratified closed set, and that is a reading.
- **The fired wake's line** is the first capture line past the mark.

### 3. Headroom under the watchdog
Every bound is named. The per-send cost is `SEND_BOUND` 90 + `SUBMIT_BOUND` 20 + `WAKE_TURN_BOUND` 120 = 230 s, and the stop is 10 s + 2 s.

| Arm | Worst case | Watchdog | Headroom |
|---|---|---|---|
| `R3B` | first frame 60 + `session-start` 60 + 2 × 230 + stop 12 = 592 s | 900 s | 308 s |
| `R3` / `R3-326` | 60 + 60 + 230 + stand 180 (was 300) + fired turn 120 + 12 = 662 s | 900 s | 238 s |
| `IM` | 72 s | 600 s (unchanged) | 528 s |

The wake legs pass `--deadline 900`, with an entry `timeout` of 1200 s. `R3`'s stand is now 180 s: a wake asked at 1 minute needs no 300 s.

Everything else stands as on card 1, and the overseer accepted it as written: the readiness deadline and its control, the three binaries on the 20 s line, the capture verb and its two
controls, the imports arm's no-key stop, R2's explanation, the order `R3B` → `R3` → `R3-326` → `IM`, the cap, no
capability claimed.

## The plan now
**Steps (11):**
1. `BootOptions.ready_deadline`.
2. The never-ready case at 2 s, under a 10 s wall.
3. The three binaries on the `ci` 20 s line.
4. The capture verb.
5. The capture hook and the imports seed.
6. The wake arms: event or capture sign, the send-1 mark, 120 s turn bounds, no refusal on a missing `session-start`.
7. The imports arm and the no-key stop.
8. Labels and verbs (`--stand` defaults to 180).
9. Four rehearsals over the fake agent.
10. `holds-text.md`.
11. The live round `R3B` → `R3` → `R3-326` → `IM`.

**Acceptance criteria (11 in the plan):**
- the nextest step;
- the `ready-deadline`, `capture-keys`, `capture-source` and `wake-capture` controls each read `holds`;
- the rehearsals;
- release-check;
- `R3B` first;
- `R3` / `R3-326` with every key (the capture keys whatever `wake` reads), each inside the 900 s watchdog;
- `IM`;
- the cap ledgered;
- `holds-text.md`.

**Touchpoints:** unchanged. New: `crates/viola-e2e/src/rig/capture.rs`, `crates/viola-e2e/src/rig/wake.rs`. Modify: the eleven files listed on card 1.

**Test commands:** 20 entries, as on card 1, with three changes:
- entry 13 (`R3B`) adds `--deadline 900`;
- entries 14 and 15 (`R3`, `R3-326`) add `--stand 180 --deadline 900` and assert `wake-sign:`, `capture-lines:` and `send-1-source:`;
- entry 15 also asserts `session-start:`.

All three now have `timeout = 1200`.

The baselines are unchanged: entries 3–8 are byte-equal to the runs baselined at card 1.

| Entry | Baseline |
|---|---|
| 3 | green, 29 passed |
| 4 | green, 9 passed |
| 5 | green, never-ready 20.459 s |
| 6 | green, 20.356 s |
| 7 | red, as predicted, with its must-pass control green (`controls-entry7.md`) |
| 8 | green |

**Controls (4):**

| Control | Entry | Kill |
|---|---|---|
| `ready-deadline` | 6 | `--timeout 120` |
| `capture-keys` | 3 | `--timeout 300` |
| `capture-source` | 3 | `--timeout 300` |
| `wake-capture` | 4 | `--timeout 600` |

Each is predicted to read its case FAILED with no TIMEOUT line. All four are unmeasured until implement fires them.

**Capabilities:** none claimed. v2-19, v2-32 and v2-35 get ledger notes as Expected amendments.

**Expected amendments (wrap):** as on card 1.

## The directive's item 12 block — what changed since card 1
- **Re-runs:** the union, the rows reads, the cited-row check (13 of 13 in the union), the `[RETYPED]` lines (3) and the re-spawns (0) are as card 1 states them. Nothing was re-run for this card.
- **New reads:** `src/cmd/run.rs` (280-330, 560-600), `crates/viola-agent-claude/src/screen.rs` (171-204), `src/run/send/classify.rs` (14-33) and `src/run/gate.rs` (150-200), all at P5. The first three are entered in research rev 3.
- **New non-zero exits and refusals:** one `grep` with no hit (exit 1, while locating `cli_verified` users).
- **`UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:** none.
- **Where the letter was wrong or silent:** the letter names no form for a review that changes the plan without a fork. It was handled as review-then-fold: re-run step 1's checks, then the next card.

## Beyond the plan
- **Re-pin R4:** R4, the live `viola verify --record`, and the record-home rule's live half (cry-ksqqok4x) leave this chunk. They are re-pinned on the existing entry «The 2.1.325 recorded fixture set» as a carry, at the wrap's route-resolve, on the founder's own words (inputs#I2 item 1). The overseer accepted this at review (inputs#I3).
- **An owner for R2's open question:** whether the CLI's placeholder hint is told apart by a cell attribute is unmeasured. The owner is «The 2.1.325 recorded fixture set», carried by the wrap; the overseer accepted it at review (inputs#I3).
