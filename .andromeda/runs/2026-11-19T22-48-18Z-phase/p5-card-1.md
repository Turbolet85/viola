# P5 card 1 — «A fired scheduled wake keeps the wheel» (2026-11-19-a-fired-scheduled-wake-keeps-the-wheel)

**Plan:** `doc:2026-11-19-a-fired-scheduled-wake-keeps-the-wheel/plan` rev 3, rendered to
`.andromeda/cache/docs/2026-11-19-a-fired-scheduled-wake-keeps-the-wheel/plan.md`, which you can open in an editor.
- No plan stands in a chunk's folder. Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-19-a-fired-scheduled-wake-keeps-the-wheel`
  diffs the chunk's records.

**Answers it plans on:** `inputs#I3`, the founder's own F1, F2, F3 and F5/F6 and the overseer's items 5-11.

## Goal
A keyless prompt the hook files `human` keeps the wheel only when ALL of these hold:
- no human key reached viola's stdin since the last human-filed prompt;
- the last turn end counted at least one scheduled wake;
- the CLI is verified.

Such a prompt is relabelled `harness`. In every other case it takes the wheel, as today.

The driver's next send is then confirmed, and the next question dialog is held for it.

The wake count gets the 26th relied row, `stop-session-crons` (a shape-only probe). Three capped live starts plus one
spare: `WS`, `HB` and `R3W`.

## The mark rule (`inputs#I3` item 6)
- The stdin mark clears only at a human-filed prompt, never at a turn end or a `release`.
- A queued human message submitted after its turn still reads human, even across a release. A case pins it.
- The cost: an erased draft makes the next wake read human, once.
- The mouse-only submit stays the named, unmeasured false positive.

## Top acceptance
- The wake keeps the wheel and is logged `harness`; the next send is confirmed; the next dialog is held. A key, no
  count, or an unverified CLI still takes the wheel. Proven by unit, `cli_wheel`, `tui_wheel` and `R3W`.
- The queued-message false-positive cases read `human`.
- 26 relied rows: 25 ids read unverified, 26 verified. The recorded sets stamp 26/26 with no recording.
- Six remove-the-guard controls read «holds».
- Live:
  - `WS` 26/26 and the stamp kept;
  - `HB` at least one hint cell read on a fresh box;
  - `R3W` `wake-origin: harness`, `wheel-after-fire: kept`, `send-2-reply: ok`;
  - the live home's sha unmoved.

## Touchpoints
- **Product:** `src/run/wheel.rs`, `src/run/send/slot.rs`, `crates/viola-agent-claude/src/ledger.rs`.
- **Test-only:** the fake (`--stop-crons`, `--wake-after-ms`, synthetic) and the rig (`WS`, `HB`, `R3W`).
- **Count pins:** 25 → 26.
- 24 files to modify, no new files.

## Test commands
38 entries:
- the offline gates, entries 1-8;
- the live round, entries 9-32: `WS` with `--cap 1`, `HB` with `--cap 2`, `R3W` with `--cap 3`, each with `--spare 1`
  and a host read before each start;
- the whole integration and E2E layers;
- the operator pass.

Baselines, all green on the untouched tree:
- entry 3: 657 passed;
- entry 5: 60 passed;
- entry 6: 126 passed;
- entry 7: 34 passed.

## Concretization preview (matrix#v2-36)
- **Now:** «Under viola run, a scheduled wake that fires leaves the wheel with its holder: the driver's next send is
  confirmed and the next question dialog is held for the driver, while a prompt the human types still takes the
  wheel.»
- **Proposed:** «Under viola run on a CLI whose stamp counts every relied row, a prompt the hook files human with no
  human editing byte on viola's stdin since the last human-filed prompt, after a turn end that counted at least one
  scheduled wake, is logged origin harness and moves no wheel; after its turn ends the driver's next send is confirmed
  and the next question dialog is held for the driver; a prompt with a human byte, with no wake counted, or on an
  unverified CLI still takes the wheel — proven by the fake-agent rehearsal (cli_wheel, tui_wheel) AND the live R3W on
  claude 2.1.328.»
- Never weaker: it adds the conditions the founder ratified and keeps every outcome.

## Mechanical checks
- Sections are present. The 24 modify paths exist.
- The fence parses: 38 entries. `planlint` reads 0 hits. The scope lists parse: 24 items.
- Snapshots: 2 unchanged, 1 n/a.
- Every path a `run` writes is gitignored. The last rig report records no harm to the host.
- **Warning:** the plan's prose runs 452 lines, past the ~400 guide.
- **Letter deviation:** the rig keeps its frames per start (`<pid>-<ms>`), so «each arm's kept dir ABSENT» (item 8) is
  read as the label's absence in the start ledger, plus `wake-stamp.json`'s absence for `WS`.

## Beyond the plan
- The live home's re-stamp at 26 rows goes to the product re-pin operator step (no route entry), outside this chunk,
  as F3 told the founder (`inputs#I3` item 2).

Apply? (yes / review / cancel)
