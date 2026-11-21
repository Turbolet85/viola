# P5 card 4 — «Harness-prefix and identity-floor ledger rows» (2026-11-20-harness-prefix-and-identity-floor-ledger-rows)

This card replaces card 3. **The plan:** `doc:2026-11-20-harness-prefix-and-identity-floor-ledger-rows/plan`,
**rev 6**. Its render is at `.andromeda/cache/docs/2026-11-20-harness-prefix-and-identity-floor-ledger-rows/plan.md`.
- No plan stands in the chunk's folder.
- Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-harness-prefix-and-identity-floor-ledger-rows`
  is a diff of the chunk's records.

## 1. What rev 6 changed — the overseer's one review item on card 3, and nothing else

The two name-diff entries are now keyed `leg = 'round'` (`inputs#I1` item 10). They are the `ls` diff of
`target/live-rig/HF/record/2.1.328` against `fixtures/claude/2.1.328`, and its `grep -c` twin. They assert the state
between `S1` and the landing, which step 27 (the landing) ends. As unkeyed entries, the wrap's light gate would have
re-run them red, as at the 27th-row chunk.

For each of the two entries:
- `leg = 'round'` added;
- `new = true` and `baseline` removed: a leg entry is exempt, and after P5 `baseline` stands only on new non-leg
  entries;
- the `run` and `expect` are byte-equal to rev 5;
- the `note` names the reason and its source (`atom from` diff's own `<` / `>` lines, and from `grep -c`'s count
  line). It also keeps the P5 readings: red on the untouched tree; green over the pass input and red over a
  stray-name input in the control clone (`.andromeda/runs/2026-11-20T18-17-18Z-phase/p5-controls.md`).

No step, criterion, touchpoint, cap, predicate, version or other entry moved.

## 2. Checks, all re-run on rev 6

- `gate.py run --dry-run`: 38 entries. Entries 26 and 27 now read `not run — leg round (the letter drives it)`.
- `planlint`: `hits: 0`.
  - check 4 (5): `leg entries with a content atom: 11 · note without atom from: 0`;
  - 4 (17): `0 of 21`;
  - 12 (c): `0 of 21`;
  - 12 (d): 3 claims listed, each read and holding.
- `gate.py baseline`: `seen green 11 · not seen 0 · new (key) 10 · leg 17`. Every new non-leg entry (3, 4, 11, 12,
  13, 14, 23, 24, 25, 28) carries its `baseline`.
- `gate.py scope --lists-only`: `scope lists: parsed 23 items (new 0 · modify 22 · derived 1)`.
- `inputs.py verify`: exit 0, 4 entries unchanged.
- Cited rows: `cited 56 · missing 0`.
- **Size, WARN, unchanged:** about 780 lines of prose outside the fence.
- **Validation-1:** aligned.

## 3. Standing from card 3

These stand as card 3 printed them (`.andromeda/runs/2026-11-20T18-17-18Z-phase/p5-card-3.md`):
- the forks' answers (F-P4-1 (a), F-P4-2 (c), `inputs#I3`);
- rev 5's fold;
- the concretization previews of v2-25 and v2-26;
- card 1's verbatim run block (`p5-card-1.md` §4).

No new refusal, `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line.

## Beyond the plan

none

---
Apply? (yes / review / cancel)
