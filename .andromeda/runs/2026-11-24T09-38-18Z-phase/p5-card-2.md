# P5 review card 2 — «`viola adopt`» (2026-11-24-viola-adopt)

**Plan:** `doc:2026-11-24-viola-adopt/plan` rev 4. Its render is `.andromeda/cache/docs/2026-11-24-viola-adopt/plan.md`.
No plan stands in a chunk's folder, writing in that file changes no record until a `put`, and `rec.py diff --root .
2026-11-24-viola-adopt` diffs the chunk's records.
**The review it folds:** `.andromeda/runs/2026-11-24T09-38-18Z-phase/relay-1.md`, snapped as `inputs#I2`
(origin: the overseer, delegate for the operator). T1–T6 each stand on their lean, as the review gave them. W1–W4
stand on arm 1, PROVISIONAL, his own word owed. Card 1 (`p5-card-1.md`) stands for everything not named below.
**Checks re-run whole on rev 4:**
- the `gate.py run --dry-run` fence parses, 22 entries;
- `gate.py baseline`: seen green 5 · new 9 · leg 8;
- planlint: hits 0, with the 11 «unchanged» claims read consistent;
- research's lists: `scope lists: parsed 10 items`;
- `inputs.py verify`: 2 entries — I1 unchanged, I2 n/a (a message), none uncited.

## What changed from card 1
### (1) What hosts `adopt` in `scripts/r5-adopt.sh` (plan step 12)
- **The pty:** util-linux `script -qfec <cmd> /dev/null` (`/usr/bin/script`, util-linux 2.42.4 on the host), started
  by the script's one `host_adopt` function as its background child. `-e` returns the child's exit code, and no
  typescript is kept.
  - `<cmd>` is the `viola --home <home> adopt …` argv, each word quoted by `printf %q` after the id's shape check,
    under `SHELL=/bin/sh`.
  - Its input is a FIFO the script makes and holds open on its fd 3, so no end-of-input and no key ever reach the
    wrapper.
  - `host_adopt` records `script`'s pid at start.
- **Closed, on every exit path:** one `close_host`, run by the `trap` on EXIT, INT and TERM and by the normal end.
  It acts only on pids this run recorded:
  1. SIGTERM to the wrapper — the snapshot's `pid`, or as a second way the one child of the recorded `script` whose
     `/proc/<pid>/exe` is the given `viola` — while that exe check holds; its claude child ends with the PTY.
  2. SIGTERM to `script`.
  3. Each waited up to 10 s, then SIGKILL only while its recorded start time is unchanged.
  4. Close fd 3, then remove the FIFO by its one name.

  Nothing is removed recursively.
- **Live and rehearsal share the code:** R5a, R5b and `--rehearse` all go through the same `host_adopt` and
  `close_host`; only `PATH` (the fake as `claude`) and the home differ. Each verdict line now carries `ended=<yes|no>`
  — yes only when both recorded pids are gone after `close_host` — so entry 9's rehearsal reads the hosting and its
  close:
  - `r5a: green resumed=yes lineage=yes pair=haiku/low turn=ended ended=yes`;
  - `r5b: green refused=session-live started=none ended=yes`;
  - then `r5: rehearsed — not a live reading` last.

### (2) R5a's prompt, pair and spend — parameters of his grant, beside the home (plan step 12, entry 21)
| Parameter | Value |
|---|---|
| Home | his stamped `~/.viola` (no verify starts; leaves `adopt-probe` on his board) or a fresh scratch home (spends the automatic verify's starts) |
| Prompt | `Reply with the single word: adopted` — one compiled literal, one send, nothing else typed |
| Model / effort | `haiku` / `low`, the cheapest the CLI takes (`--effort`: low · medium · high · xhigh · max), handed on the resumed child's argv as every start hands them (architecture · decision/launch-pair-handed-not-verified: whether the session runs on them is unmeasured; the line reads the pair the snapshot recorded). Entry 21 takes them as `R5_MODEL` / `R5_EFFORT`, so his grant can name another pair |
| Spend | one model turn on that pair. Its input is the adopted session's whole transcript plus the prompt — a few thousand input tokens for a one-exchange probe session, which is why the probe should be one short exchange — and about one word out. His own plain start and its exchange are his, outside the script. R5b spends no model turn |

- Entry 21's expect is split around the `pair=` field his grant varies: `contains r5a: green resumed=yes lineage=yes`
  · `contains turn=ended ended=yes`.
- Entry 22 adds `ended=yes`. Entry 9 asserts the full rehearsal lines above.
- The gated entry 14 (`^r5[ab]: green `) is unchanged.

## Unchanged from card 1
- The goal, steps 1–11, the touchpoints, entries 1–8, 10–20, the baselines and the controls table.
- The concretization preview of `matrix#v2-13`: still claimed GATED on R5. Its proposed reading gains `pair=<granted>`
  and `ended=yes` on the `r5a` line and `ended=yes` on the `r5b` line, the outcome unweakened.
- Directive item 11's block.

## Beyond the plan
- **A route note for «Switching a session's model and effort» (v2-30)** (standing — `.andromeda/runs/2026-11-24T09-38-18Z-phase/p5-card-1.md`):
  v2-30's acceptance names «after … an adopt the session runs on the pair its instance recorded». This chunk records
  adopt's launch pair, like every start, and claims no part of v2-30.
- **W1–W4 PROVISIONAL rows at the wrap** (standing — `.andromeda/runs/2026-11-24T09-38-18Z-phase/p5-card-1.md`): each
  widening lands as built, marked not ratified, his own word owed (`rul-r74hshif` / `rul-p2zl2kgl`).

Apply? (yes / review / cancel)
