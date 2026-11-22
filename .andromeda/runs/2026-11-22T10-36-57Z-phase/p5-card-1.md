# P5 card 1 — «Paste write bounded» (2026-11-22-paste-write-bounded)

**The plan:** `doc:2026-11-22-paste-write-bounded/plan` rev 4. Its render is
`.andromeda/cache/docs/2026-11-22-paste-write-bounded/plan.md`, to open in an editor. No plan stands in a chunk's
folder. Writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-22-paste-write-bounded` is a diff of the
chunk's records. The research is `doc:2026-11-22-paste-write-bounded/research` rev 3, and the scope rev 3 is closed.
The inputs are `inputs#I1` (the directive) and `inputs#I2` (the operator's answers and the overseer's technical
answers). Both are unchanged; check 10 reads 0 unparsed.

## Goal
The paste ends within a compiled 5 s `PASTE_BOUND`.
- A paste the child does not take within it is cut: the bracket is closed and no Enter is written. The send ends
  `not-delivered` / `paste-cut` (exit 13) with its cursor, and the wheel is unmoved.
- Every later send is refused `paste-cut` until a human editing key, a `prompt-submitted`, a `session-start` or a
  `session-end` clears the mark.
- The paste moves onto a PTY writer thread of its own, path (B), so the human's keys wait behind at most one chunk
  and the closing marker.
- The longest delivered send stays at 8.5 + 5 + 40 = 53.5 s, under the 55 s MCP deadline.

## Top acceptance criteria
- **viola-pty:**
  - a paste into a child that stops reading returns `PasteCut` within its bound;
  - the cut leaves the start marker, a text prefix and the closing marker, with no CR;
  - a human key waits behind at most one chunk;
  - a paste the child takes is written whole with its CR;
  - the pump returns with no writer thread left.

  These are five cases and four controls: `paste-bound`, `key-behind`, `cut-no-enter`, `writer-join`.
- **send:**
  - a cut ends `paste-cut` with its cursor and no wheel record;
  - every later send is refused `paste-cut`, with nothing issued, until one of the four clears;
  - a failed paste stays `input-not-ready`.

  These are the cases of step 16 and the control `cut-mark`.
- **cross-process, over the fake agent with the new `--stop-reading`:**
  - a 65 536-byte send exits 13 with exactly `{"v":1,"refusal":"not-delivered","detail":"paste-cut"}` in at least
    5 s and under 12 s;
  - the next send is refused `paste-cut` with no `send-issued`;
  - a send during the block is refused within `PASTE_BOUND` + 2 s (35.1 s today);
  - a hook event's line lands within `PASTE_BOUND` + 2 s (today it waits for the child).
- **bound:** the arithmetic test, 53.5 s < 55 s; `paste-cut` is a viola-core word with its hint and its struck line
  pinned by literal.
- **layers:** the unit, integration and e2e layers, the lint, and CI green on the operator pass's HEAD.

## Touchpoints
16 files to modify, no new file:
- `crates/viola-pty/src/pump.rs` and `lib.rs`
- `crates/viola-agent-claude/src/screen.rs`
- `crates/viola-core/src/lib.rs`
- `crates/viola-mcp/src/tools.rs`
- `src/human/hints.rs` and `mirror.rs`
- `src/run/send.rs`, `send/slot.rs` and `wheel.rs`
- `src/cmd/run.rs`
- `src/cmd/verify/typed/run.rs`
- `src/bin/viola-fake-agent.rs`, `/input.rs` and `/test_cases_opts.rs`
- `tests/cli_send.rs`

## Test commands
25 entries:
- 1–2: the formatter and clippy;
- 3–7: viola-pty whole, then its four control entries, each filtered to its one case;
- 8–10: viola-core, viola-agent-claude, viola-mcp;
- 11–12: the root binaries' units and the `cut-mark` control entry;
- 13: the four cross-process cases;
- 14: the error-bodies census;
- 15: the host read;
- 16–18: the harness unit, integration and e2e layers;
- 19–25 (operator): boot, status, cleanup, pre-push, hygiene, push, CI.

There is **no round and no live entry**, so there is no round dry-run to show. Five remove-the-guard controls are
implement's one-shot `gate.py control` runs, each on its own filtered entry with a bounded case.

**Baselines on the untouched tree**, 14 entries `new = true`:
- green: 3, 8, 9, 10, 11, 14 (and 15 recorded).
- vacuous red, exit 4 with 0 selected, until the chunk's cases exist: 4, 5, 6, 7, 12, 13.
- **red on an existing property:** 18, the e2e layer. `contract_fake_agent_drift` read verify's
  `[20/29] agents-join` failing on 2.1.287 beside its two expected fails, while the host read **CONTENDED** (870 of
  900 s stalled on IO, escher's build and mutation tier alive). This is the WATCH's own signature (cry-dkm6eurv), now
  met in a contract binary. It is recorded and not re-run (`inputs#I2` item 10). **Owner: the WATCH this chunk
  folded.** Plan step 19 now reads it in both layers. If it reads red again on a QUIET host, that is a finding and
  implement stops.

## Concretization preview
**None.** This chunk claims no capability (`inputs#I2` item 4). The plan carries the ledger note `matrix#v2-24 notes`,
which the wrap writes at P7.3. Its status, ref and acceptance are untouched. Coverage stays 23/37.

## Corrections to what was shown before
- **The residual shown to the operator in the P3 block was wrong.** The block said a client built before the word
  reads `paste-cut` as `unknown`, exit 14. In fact exit codes go by reason alone (`src/cmd/send.rs:65-69`), and a
  reader keeps an unknown detail word as opaque (`architecture · contract/refusal-reason/detail`). Such a client reads
  **`not-delivered` / `paste-cut`, exit 13, the word shown, no hint line**. The real residual is milder than the one he
  accepted. Until the re-pin, the pinned product neither emits nor reads the word.
- **The hint's wording is a planning form, PROVISIONAL.** The hints are the founder's words. His answer fixed what it
  says (a human must look at the session), not its letters. The plan's form is
  `<name> did not take the whole text in time; a human must look at the session before the next send`.

## The block, verbatim (`inputs#I1` item 13)

**The union's per-master lines and its `rows-union` line, as printed:**
```
architecture: rows 978 · packs 10 · picked 177 (107 by every draw) · standing 26 · 53509 B
security-plan: rows 632 · packs 7 · picked 81 (50 by every draw) · standing 10 · 26026 B
design-system: rows 324 · packs 4 · picked 99 (67 by every draw) · standing 3 · 21526 B
layout-templates: rows 223 · packs 3 · picked 50 (41 by every draw) · standing 14 · 10389 B
test-plan: rows 1366 · packs 14 · picked 304 (164 by every draw) · standing 33 · 74955 B
obs-plan: rows 1034 · packs 11 · picked 286 (185 by every draw) · standing 34 · 69092 B
a11y-plan: rows 236 · packs 3 · picked 84 (61 by every draw) · standing 18 · 22481 B
rows-union: 7 master(s) · picked 1081 of 4793 · rule any
```
The rows files sum to 277 978 B, past the 250 000 B bound, so P4 read them **by search**, never whole.

**Is every cited row in the union? Yes:** cited 35 · in a rows file 35 · only in a standing list 0 · not in union 0
(`cited_rows.py` over the plan, all seven masters). No existing row outside the P3 block's §3 was found reached.

The plan's new rows follow from the operator's answers. They are named here so none surfaces later as a fork:
- architecture: a decision row for the cut mark (L1);
- obs-plan: a row saying the cut mark opens no span and writes no line, as the wheel's typed mark does;
- design-system: `contract/refusal-hints/not-delivered-paste-cut` (W1);
- test-plan: `contract/fake-agent-cli/stop-reading` and the fake agent's option count moved by one (`inputs#I2`
  item 9).

**`[RETYPED]` lines, as printed:**
```
security-pick-p4.d2 · [RETYPED] line 46: decision/live-rig-mb-arm-keeps-only-closed-words is no row of the pack — read as decision/live-rig-mb-arm-keeps-no-output, the pack's row 46
tests-pick-p6 · [RETYPED] line 51: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 51
tests-pick-p6.d3 · [RETYPED] line 67: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 67
tests-pick-p9 · [RETYPED] line 30: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 30
```

**The re-spawn:** `layouts-pick-p3 · [ABSENT]` in pick-p1 was re-spawned once as `pick-p1-r1`, and read
`VERDICT: 1 of 1 pass`. No dead draw.

**Every non-zero exit, refusal and UNPARSED / INDETERMINATE / NOT DERIVED line:**
- **P3 measurement:**
  - run 1: `viola run` exited 1, with the detail line `local socket name length exceeds capacity of sun_path`;
  - run 3: send A ended `input-not-ready` because the child's ticks defeated the gate's quiet period;
  - the first capacity probe looped without end on a canonical slave, and I stopped it with TaskStop.
- **P4:** `gate.py baseline` ran past the 2-minute foreground limit, so it went to the background and its output was
  read whole.
- **P5:** the Bash guard refused one call, `Blocked: a pipeline tool's output piped into a filter`; I re-fired it
  bare. The baseline firings read red on entries 4, 5, 6, 7, 12 and 13 (`exit 4`, no tests to run) and 18 (`exit 1`,
  above).
- No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed anywhere in the run.

**Every place a letter was wrong or silent:**
1. My P3 block's exit-14 residual (corrected above).
2. Research rev 1's modify list named `tests/contract_error_bodies.rs`, which a new `PtyError` variant does not
   touch. It missed `src/run/wheel.rs` and `src/bin/viola-fake-agent/test_cases_opts.rs`. Synced at P4 and P5
   (research rev 3).
3. My first sweep line miscounted (8 files where the list held 10), corrected before the first put.
4. validation.md (9) is silent on the baseline word for a report-only entry (`expect = []`). Entry 15 is written
   `green — recorded …`.
5. The fan-out contract's §The batch 1a says the next part starts only after the previous part's re-spawns. It is
   silent on whether a picker's ABSENT draw is re-spawned, or left dead and read on its surviving draws. I re-spawned
   it under the general rule.
6. The rows-file standing list arch S3 says "each new fake-agent behaviour is a named test seam variable". That
   conflicts with the argv-option precedent (arch history n=23, test-plan `contract/fake-agent-cli/*`) and with
   `inputs#I2` item 9. The argv option wins, and S3 is kept by adding no seam variable.

**Checks:**
- 1: sections present.
- 2: 16 modify paths exist; no master among them; the v2-24 ledger note is on a verified cap.
- 3: `PASTE_BOUND`, `stop-reading`, `paste-cut`, `pastecut-smoke` are free in architecture.
- 4 (0): the fence parses. 4 (9): baselines written.
- 5: 0 placeholders.
- 6: 428 prose lines, 7% over 400 (a note).
- 7: graph consulted, two queries, both fresh.
- 8: `Observed::read` → `human_key` per editing read and `Ctx::refuse` read at HEAD.
- 9: none. 10: 0 unparsed. 11: every process a test ends is one it started. 12: in order.
- validation-1: aligned with the scope and the working entry.
- planlint: 0 hits.

## Beyond the plan
| # | Proposal the word approves | Where it lands |
|---|---|---|
| 1 | The WATCH cry-dkm6eurv met its signature again at P5, the e2e baseline on a CONTENDED host. Its "retires: a recurrence" clause is now met. The wrap's route-resolve decides its fate: retire it as recurred, with the reading made a finding owned by a named entry, or keep watching. The plan makes no route edit. | the wrap's route-resolve |
| 2 | The desk: `target/pwb-xdg-3`, `-4`, `-5` (each a stale socket and lock from P3's measuring wrapper) and the scratchpad runs `run1`–`run5` stand for removal by hand. This chunk deletes nothing. | the founder's desk |

The word on this card is snapped. When the plan is approved the phase ENDS and nothing else starts (`inputs#I2`
item 12).

**Apply? (yes / review / cancel)**
