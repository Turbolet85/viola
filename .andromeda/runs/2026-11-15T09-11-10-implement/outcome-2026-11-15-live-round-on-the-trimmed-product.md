# Implement outcome — 2026-11-15-live-round-on-the-trimmed-product (run 2026-11-15T09-11-10)

**Outcome: stuck — entry 10 fired once and read RED; it is not re-fired. The session never reached `session-start`,
so there was no hold.** Starts spent: 7 of the founder's 9. The three rig faults it exposed are fixed and rehearsed
(inputs#I19), and four plan lines must move for them, so this stops for the phase door (section «After entry 10»).

Setup's approval line, quoted: `2026-11-15T09-08-58-phase/matrix-2026-11-15-live-round-on-the-trimmed-product.json · open · 910 B`
(plan rev 8). The word is snapped as inputs#I18.

## Before entry 10
- Entries 1–8 and 21 re-fired: `green 9 · red 0`, ending on the plain build `a0e247fd3a77f744`.
- The pre-fire reads: `latest: 2.1.325 · cli 2.1.325 · match` · starts 6 · state stamp `pass` for 2.1.325 · `/tmp` 21 G
  free.
- The run-dir journal was written before the fire: `.andromeda/runs/2026-11-15T09-11-10-implement/hold-journal.md`.

## Entry 10's readings (`evidence/operator-10.txt`, `evidence/live-round.md`)
- **Gate line:** `10 e2e red · exit 0 ✗ (exit 1) · exit 1 · 461.01s · 1231 B → 10.log · history: unmoved`.
- **Start readings:**
  - green: `child_cli_verified:true`, `pty_backend:"openpty"`, `snapshot_cli_verified:true`, socket dir 0700 and socket
    0600, the plugin written at this start with its hooks running the pinned copy, the child's fds 0–2 on the pty,
    none on the endpoint and none under the home, 0 `unable: ` / 0 `hint: ` bytes, no budget kind.
  - red: **no `session-start`** — `events.ndjson` holds `wheel{start}`, `wheel{human}`, `wheel{driver, release}`,
    `send-refused`, and nothing more; the run log holds no hook event. Trust step: `no-dialog` (the stripped stream held
    no trust literal).
- **Name refusals:** `already-live` and `stale-heartbeat`, no child spawned, the heartbeat live again after.
- **The box reading and the send's outcome, side by side:**

  | box reading (screen model) | send's outcome |
  |---|---|
  | `echo=False · human=True · box=erased · release=0 · driver=True · keys_after_del=0` | `send=13 · not-delivered · detail input-not-ready` (`send-refused`) |

  The rig labelled this `box_witness=x-stood`, which is wrong: `input-not-ready` means viola's readiness gate saw no
  input box. It is not a standing `x`. Neither side reads the box.
- **The hold:** `question=False` · `dialog_expired=False` · `deadline_hit` and `duration_ms` unread.
- **Killed wrapper:** `child_gone_after_kill=True` (the child gone by pid and start time within 10 s of the wrapper's
  SIGKILL); `census_held=0`.
- **Status line:** `hits: 0` on a session that never started, so not a reading. `~/.claude/settings.json` unchanged
  (mtime and sha256 equal before and after).

## The cause — a HYPOTHESIS, the screen not kept
A dialog stood over the session from its start, and `claude` fires no hook while it stands. The evidence:
- no SessionStart and no hook at all;
- `input-not-ready` at the send;
- 1770 bytes drained in 461 s;
- the rig's stripped-stream trust search found no trust literal.

Which dialog it was is unread. Two candidates: the folder-trust dialog for the fresh `proj/` cwd (its own git repo on
tmpfs), drawn in a way the stripped-stream search misses (the screen model would hold it, but the trust step still
reads the stripped stream); or another first-run dialog of 2.1.325.

Rig faults this exposes, for the operator's word (nothing changed after the red):
- (a) the trust step reads the stripped stream, not the screen model;
- (b) `send_witness` maps every `not-delivered` to «x-stood», so `input-not-ready` is misread;
- (c) the rig keeps no screen, so a red like this leaves its cause unread.

Unread because of this red: v2-03, v2-02, CARRY 3's wrapped side. Two starts remain (8 and 9 of 9). Entries 11 and 12
cost one each, and no start is left for a second wrapped attempt unless the founder raises the cap.

## After entry 10 — the three rig faults fixed (inputs#I19, the overseer's technical answer; no live start)
Entry 10 stays red as fired. Starts: still 7 of 9.
1. **The trust step reads the screen model**: the literal, the `selected` read and the ready screen come from the
   model's rows. Synthetic control on a dialog drawn with cursor moves between its words: screen `True` (and
   `selected: True`), stripped stream `False`.
2. **The send witness**: `input-not-ready` → `unread`; another `not-delivered` or a `wheel` line `human` → `x-stood`;
   confirmed → `empty` (`witness_controls() == True`).
3. **Every live session keeps its screen**: the model's frame at each decision point and the final frame, scrubbed
   as the recorder scrubs a fixture and checked clean, written live into `evidence/screens-{sub}[-{arm}]-{stamp}.json`
   and on an exception exit too. `live-verify` and `live-tap.py live` open no pty of the rig's, so they have no screen
   to keep.

**Rehearsed under the fake agent**, entries 1–8 and 21 `green 9 · red 0` (firing 2):
- entry 5: `screens=frames=9,clean=True · modal_arm=modal-refused-no/13/unread/frames=3,clean=True · witness_controls=True`,
  the hold `hold_s=60 · deadline_hit=True · duration_ms=60003`.
- entry 6: each arm's frames kept clean; the granted trust arm reads `modal-unpassed/2/0` off the screen.

**The 1770 bytes of entry 10:** discarded, so unread. What they would hold, and the mechanism that fits entry 10's
codes, are written in `evidence/live-round.md`.

**Plan lines that move — STOP for the phase door** (plan rev 8, its render's line numbers):
- **Step 3, line 71**: «… the folder-trust dialog passed by at most 2 down-arrows and 1 Enter … only while the drained
  bytes hold the compiled trust literal». The trust step now reads the screen the drained bytes build.
- **Implementation note, line 478**: «… `not-delivered` or a `wheel` line `human` says the `x` stood …». Now
  `input-not-ready` reads `unread` and only another `not-delivered` (or a `human` wheel line) reads `x-stood`.
- **The kept screens — no plan line names them:**
  - `## Codebase touchpoints → New files` enumerates the evidence files and lacks `screens-*.json`.
  - Step 3's rig discipline (line 56: each closing line holds «nothing that holds a path, prompt, assistant text, id or
    pid») governs the printed line, and the frames are files. But they keep the CLI's screen text (a dialog's words,
    the typed prompt, the assistant's text), scrubbed only of the home, the user word and the two roots. The plan says
    nowhere that a rig keeps screen content, and step 5's «the count only written» (line 128) is the nearest line.
    Whether screen content may stand in evidence is a plan line to write.

Entries 11 and 12 wait. How the last two starts are spent is the founder's.

## Files changed (work dir only)
- `evidence/operator-10.txt`, `evidence/live-round.md` — entry 10's record. `evidence/live-sessions.ndjson` — its
  `start` and `reading` rows (appended by the rig). `evidence/hold-journal.md` — the rig's journal at the send.

## Matrix
No ref. v2-02 and v2-03 stay gated and unread.

## Process hygiene
| process | started by | final state |
|---|---|---|
| entry 10's `viola run round` wrapper and its `claude` 2.1.325 child | this run (operator entry 10) | terminated — the wrapper SIGKILLed by the rig, the child gone within 10 s; `gate.py census --args viola-live-20261115T083805Z --args installs/claude/2.1.325` → none running |
| the two refused `viola run round` attempts and `viola release` / `viola send` | this run | exited on their own |
| entries 4–7's rehearsals | this run | terminated — `census_held=0` in each closing line |

## Decisions & corrections
- The operator's word: entry 10 only, no re-fire, stop after it (inputs#I18). Fired once, red, not re-fired.
- The fixes are the overseer's technical answer (inputs#I19). A synthetic control backs the trust fix, not a
  measurement of entry 10.
- A recurrence of the standing learning «a timestamp … is read from `date -u`», the second this session: an evolve `ts`
  was typed rather than copied from the clock. `evolve.py` refused it, nothing landed, and it was re-stamped from a
  fresh `date -u`.
- A hook refusal: the stamp hook refused the run-dir journal's predicted clock times. They were reworded as durations
  after the fire; the journal itself was on disk before the fire.
