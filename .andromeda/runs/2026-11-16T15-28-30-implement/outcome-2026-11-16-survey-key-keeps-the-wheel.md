# Outcome — 2026-11-16-survey-key-keeps-the-wheel · run 2026-11-16T15-28-30-implement (plan rev 7, revision 1)

**Outcome: stuck — S6 (entry 18) red; the operator pass is NOT run.** Plan step 10b: "Any other reading … is red
and stops the chunk before the operator pass", and the operator's direction for this run: "A red that moves a plan
entry stops you BEFORE the operator pass". The cap of 6 is spent (`ledger: starts 6`), so a seventh start is the
founder's (`inputs#I8`). The working tree is preserved. `matrix#v2-21` gets no `ref` and stays unclaimed.

## What ran, in order
1. **The rig's revision-1 edits** (`evidence/survey-rig.py`, gitignored; the product tree untouched by this run):
   - cap 6;
   - `refused: spent` and `refused: s6-not-owed`, both before the token read, uncounted;
   - `live S5` / `live S6` with per-label instance names (`survey-s5`, `survey-s6`); S6's turn comes from the
     child's argument (`viola run survey-s6 -- claude "<probe text>"`);
   - the watch (opens at the first send's reply for S5, at `session-start` for S6; closes 120 s after `turn-ended`
     or 210 s after opening);
   - key `0` once both literals stood together 1 s inside the watch, through a pure `SurveyTrack` that the
     `survey-timing` arm drives over the plan's six synthetic timelines;
   - readings `survey-seen-at`, `survey-stood-ms`, `turn-ended-before-key`, plus `watch-open-before-turn-end` and
     `sends-before-close`;
   - arms `cap-refused` (at 6), `spent-refused`, `s6-not-owed` (no S5 summary, and an S5 summary reading
     `shown: yes`), `s5-shape` (replaces `s4-shape`), `s6-shape`, `survey-timing`.
2. **Entry 12 (rehearse)** — firing 1 red: `arm s5-shape: FAIL`, `watch-open-before-turn-end: no`. The fake agent's
   `--turn-stop` fires Stop before `viola send` returns. Fixed in the rig: the s5-shape arm runs the fake agent with
   a one-step `--script` (Stop, `gate: true`) and `--control`, and the rig releases the Stop once the watch is open.
   Firing 2 **green**: every arm passed, `rehearse folders: absent`, `rehearse keys-written: 0`, `rehearse: pass`.
3. **Entry 16 (plain build)** — green, 0.17 s (fresh). `target/debug/viola` sha256 prefix `cff4a91d8b69`, read right
   before each live start.
4. **Entry 17 (S5)** — clock before: 2026-11-16T15:33:02Z. Line: `red · contains 'wheel-after-key: none' ✗ · exit 0 ·
   124.68s · history: unmoved`. Summary:
   `start: S5 · version: 2.1.325 · build: cff4a91d8b69 · token-window: ok · turn-ended: yes · shown: no ·
   prompt-in-stream: present · first-send: confirmed · watch-open-before-turn-end: yes · survey-seen-at: during-send ·
   survey-stood-ms: 535 · turn-ended-before-key: no-key · sends-before-close: 1 · keys-written: 0 · ctrl-c: 3 ·
   killed: no · folder: absent · ledger: starts 5`.
   `shown: no` → step 10b: S6 owed (`evidence/operator-17-s5.txt`).
5. **Entry 18 (S6)** — clock before: 2026-11-16T15:35:20Z. Line: `red · contains 'second-send: confirmed' ✗ · exit 0 ·
   11.37s · history: unmoved`. Summary:
   `start: S6 · version: 2.1.325 · build: cff4a91d8b69 · token-window: ok · turn-ended: yes · shown: yes ·
   prompt-literal: present · options-literal: present · options-keys: 1,2,3,0 · prompt-in-stream: present ·
   first-send: none · watch-open-before-turn-end: no-turn-end · survey-seen-at: mid-turn · survey-stood-ms: 1040 ·
   turn-ended-before-key: no · sends-before-close: 0 · turn-running-at-key: yes · key: 0 · input-held-digit: yes ·
   input-cleared: yes · survey-gone: yes · turn-started: no · follow-up: none · wheel-after-key: none ·
   second-send: refused-human-typing · keys-written: 1 · ctrl-c: 3 · killed: no · folder: absent · ledger: starts 6`
   (`evidence/operator-18-s6.txt`).
6. Step 11's record: `evidence/live-starts.md`, all six starts.

## Why S6 is red: the plan's S6 shape defeats its own assertion (code read; the start's home went with its folder)
- S6's turn starts from the child's argument with no in-flight send. So the hook files its `prompt-submitted` as
  `origin: human`, and the wrapper moves the wheel to `human` / `human-input` before any key
  (`src/run/send/slot.rs:250-252`).
- A human key under a human wheel makes no move and writes no line (`src/run/wheel.rs:679`, `key_under_the_human`).
  So **`wheel-after-key: none` is vacuous in S6**: it cannot tell the survey rule's keep from a key the classifier
  took.
- The second send is refused `human-typing` because the holder is human (`src/run/send.rs:111`), whatever the survey
  key did. On this product, S6 as planned could not read `second-send: confirmed` under any build.
- **This is not shown to be a product fault.** The CI half (entry 6's tables) held in the previous run, and the
  product code is unchanged since.
- **Not measured:** the order of the wheel line against the key's offset. Inference from code only.

## Why S5 read no survey (unmeasured hypothesis)
The two literals stood together during the first send for at most 535 ms and were gone at its return. That fits the
bundle's `typed_over` close: the send types over the box, and the survey closes. No start is left to confirm it.

## What this means for v2-21
The live half (a survey key under `viola run`, then no `wheel{human,human-input}`, then a confirmed driver send) is
**unproven after 6 starts**. Every start shape tried fails one of two ways:
- with a send before the survey, the send closes the survey (S4, S5);
- with no send, the argument-started prompt hands the wheel to the human first (S6).

The CI half holds by entry 6's tables (previous run). A live proof would need a start in which the driver holds the
wheel at the moment the survey stands, with no typed send closing it. One candidate (not tried, the founder's):
- start under S6's shape;
- after the turn and the human-input wheel move, issue `viola release` (a human verb, by the rig as the human);
- wait for a later survey and key it, then the second send.

That candidate needs a survey shown after a release, which the forced survey may not repeat.

**For the founder** (a seventh start is his): (a) one more start in a shape that leaves the wheel with the driver when
the survey stands; (b) re-plan the live proof (`/andromeda-phase` offers the revision); or (c) let the CI tables carry
the claim, with the live half owed to «The 2.1.325 recorded fixture set» alongside the survey's ledger row.

## Gates in this run
- 12 green (firing 2; firing 1 red, recorded above, fixed in the rig before the re-fire) · 16 green · 17 red (plan
  route: `shown: no` → S6) · 18 red (stop).
- **Entries 1–11 and 19 were NOT run in this run.** The product tree is unchanged since the previous run's green
  block (`.andromeda/runs/2026-11-16T13-33-50-implement/outcome-2026-11-16-survey-key-keeps-the-wheel.md`:
  `entries 21 · green 14 · red 0`, `pre-push` `"ok":true`). The stop came first.
- Operator entries 20–22 not run (the stop).

## Deviations (with justification)
1. **The s5-shape arm holds the fake agent's Stop behind a `--script` + `--control` gate** and releases it once the
   watch is open. Plan step 1 asks the arm to show that the watch was open before `turn-ended` landed. Under
   `--turn-stop` the fake agent ends the turn before `viola send` returns, so the arm could not show it. The rig
   only; no product or fixture file (the script is written in the rehearsal's own `/tmp` folder).
2. **Readings beyond the plan's three:** `watch-open-before-turn-end: yes|no|no-turn-end|pending` (the s5-shape
   showing) and `sends-before-close: {n}` (the s6-shape's `send-issued` check). Both are closed words or numbers.
3. **`survey-seen-at` when `turn-ended` landed before the watch opened** uses the watch's opening as the turn's end
   time (the event's own time is not read).

Scope: `scope: clean — changed 4 · listed 4 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) ·
absorbed 0 · excluded 246`. No guard entry exists in the plan.

## P3 smoke
S5 and S6 booted `viola run` on the plain build through `pump_child`; S5's `first-send: confirmed`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| rehearse arms (fake agent, `viola run`) | this run (entry 12, 2 firings) | terminated — `rehearse folders: absent` |
| `viola run survey-s5` + claude 2.1.325 | this run (entry 17) | terminated — Ctrl-C ×3, `killed: no`, folder absent |
| `viola run survey-s6` + claude 2.1.325 | this run (entry 18) | terminated — Ctrl-C ×3, `killed: no`, folder absent |
| the 15 census rows (overseers, prototype builders and worker, product `viola wait`, `~/.viola` mcp) | NOT DERIVED — not this run's | left running: the host's own sessions — their owners |

Census after S6: `census: 15 running`. None carries `viola survey probe`, `survey-s5` or `survey-s6`. No
`viola-survey*` name stands under `/tmp`.

## Decisions & corrections
- The operator's direction (this run's arguments): the rig edits, the rehearsal, the build, then S5, and S6 only if
  S5 reads `shown: no`; cap 6, each start ledgered with the clock time before it; stop before the operator pass on a
  red that moves a plan entry; no scheduled wake or timer. All followed; no token-window refusal met.
- A red met and fixed in the rig: rehearse firing 1, `arm s5-shape: FAIL` (`watch-open-before-turn-end: no`).
- No guard refusal in this run.
