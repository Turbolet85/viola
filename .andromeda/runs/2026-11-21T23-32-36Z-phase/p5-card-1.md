# P5 review card 1 — 2026-11-21-key-after-a-resize («Key after a resize»)

Written 2026-11-22T08:16:59Z. Plan `doc:2026-11-21-key-after-a-resize/plan` rev 3 · scope rev 3 · research rev 1. The
operator's answers K1–K4 and the overseer's items 5–10: `inputs#I2`, appended to this run's `founder-forks.md` §9.

**To read the plan in an editor:** `.andromeda/cache/docs/2026-11-21-key-after-a-resize/plan.md` (its render, 532
lines). No plan stands in a chunk's folder; writing in that file changes no record until a `put`.
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-21-key-after-a-resize` is a
diff of the chunk's records.

## Goal

The question was only ever a Windows hypothesis, so it is measured on Linux now, not assumed. A new unwrapped rig arm
`KR` runs a real `claude` in the live rig's own pty, under its scratch folder and private config, with no prompt sent
and no product file changed:
- **Control:** one `x` and one DEL with no resize, to learn the box row's typed and hint attribute words.
- **12 rounds:** each resizes the rig's own pty (210×45 ↔ 180×40), types one `x` 0, 20 or 100 ms after the resize
  returned, reads kept / lost from the box row's cell attributes (closed words), and types one DEL.

It runs on 2.1.330 (`KR`) and 2.1.328 (`KR8`), with one spare.
- **Both `kept`:** `v2-27` closes on the readings (F0).
- **Any `lost`:** a red, never retried and never a spare's. Implement stops with the summary's closed words, and the
  driver-paste fix (F1) comes back to the operator as a plan revision.

## Top acceptance criteria

1. **(e2e, matrix#v2-27)** `KR kr-reading=kept` and `KR8 kr-reading=kept`, the round's reading entries 18–19: 12 of
   12 rounds `kept` per version. The readings are recorded as closed words in
   `.andromeda/cache/chunks/2026-11-21-key-after-a-resize/evidence/live-round.md`.
2. **(security)** Each live start types exactly 13 `x` and 13 DEL and resizes only its own pty under
   `--grant box,resize`. Its summary and ledger hold closed words and counts only; its frames stay under the
   gitignored `target/live-rig/`; hygiene reads `clean`.
3. **(security)** The kept rig products and the live home's stamp read the same before and after the round.
4. **(tests)** The rig's inline cases (every kept / lost / unread and reading word) and both `KR` rehearsals over the
   fake agent pass; the whole integration layer, `pre-push` and CI read green.

## Touchpoints

Five files, all in `crates/viola-e2e`, no product file:
- `src/rig/keys.rs` — the `resize` grant word;
- `src/rig/screen.rs` — `Model::resize`, vt100's in-place `set_size`;
- `src/rig/mod.rs` — `Arm::KR`, the round's pure parts, `arm_kr`, the summary keys, `finish`'s rule;
- `src/bin/viola-live-rig.rs` — the doc lines;
- `tests/rig_rehearsal.rs` — two rehearsals.

## Test commands — 29 entries

- **Phase A, 1–5:** fmt; clippy; the rig lib cases; the `KR` rehearsal filter; the unit layer.
- **The round, 6–19** (`round` / `live` only):
  - host read; the 2.1.330 and 2.1.328 installs by path; `mise ls`; census; the rig build and hash;
  - kept products' sha before; live stamp sha before;
  - **`KR` (`--cap 1`)**, **`KR8` (`--cap 2`)**, each `--spare 1`;
  - kept products after; live stamp after;
  - the `KR` reading; the `KR8` reading.
- **After the span:**
  - 20: census (`operator`);
  - 21: the ledger read (report-only);
  - 22: the whole integration layer;
  - 23–29: the operator pass — boot / status / cleanup smoke, `pre-push`, hygiene, the push, `ci.py`.

**The two starts are ordered independent** (`inputs#I2` item 7). A whole start exits 0 whether it reads `kept` or
`lost`; the `kept` / `lost` reading is no live atom. So a `lost` on `KR` still lets `KR8` run, and the first red
reading entry (18, then 19) stops the round after both. A start that exits 1 (`refused shape`, `confounded`,
`dirty-box`, `unverified`, the token window, a host stall) stops the round at its own entry. The spare `KR-2` /
`KR8-2` is then a plan revision's entry on the overseer's word — never a re-fire, never on a `lost`.

**Control ids:** none. The plan has no remove-the-guard control (no product guard is added), so no `gate.py control`
fires.

**The whole-round dry-run** (`gate.py run --chunk 2026-11-21-key-after-a-resize --live-legs --dry-run`, as printed):
```
  6 probe       not run — dry-run · python3 ~/dev/projects/additional/andromedaV3/andromeda-ov… (103 chars)
  7 probe       not run — dry-run · test -x "$HOME/.local/share/mise/installs/claude/2.1.330/c… (134 chars)
  8 probe       not run — dry-run · test -x "$HOME/.local/share/mise/installs/claude/2.1.328/c… (134 chars)
  9 probe       not run — dry-run · mise ls claude
 10 probe       not run — dry-run · cargo run -q -p viola-e2e --bin viola-live-rig -- census
 11 build       not run — dry-run · cargo build -q -p viola-e2e --bin viola-live-rig && sha256… (102 chars)
 12 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (584 chars)
 13 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (210 chars)
 14 e2e         not run — dry-run · cargo run -q -p viola-e2e --bin viola-live-rig -- live KR … (224 chars)
 15 e2e         not run — dry-run · cargo run -q -p viola-e2e --bin viola-live-rig -- live KR8… (225 chars)
 16 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (592 chars)
 17 probe       not run — dry-run · f=.andromeda/cache/chunks/2026-11-21-key-after-a-resize/ev… (258 chars)
 18 probe       not run — dry-run · python3 -c 'import json;r=[d for d in map(json.loads,open(… (297 chars)
 19 probe       not run — dry-run · python3 -c 'import json;r=[d for d in map(json.loads,open(… (300 chars)
not run — excluded · 15: 1-5,20-29
round: DRY-RUN · legs 2 · entries 29 · green 0 · red 0 · recorded 0 · timeout 0 · indeterminate 0 · not-run 29
```

**The kept products** (the set derived from `ls target/live-rig/` at P4, `inputs#I2` item 9). Their sha is written
once before the round (entry 12) and must read equal after it (entry 16):

| product | its files |
|---|---|
| `HF` | `floor-stamp.json` |
| `HG` | `floor-stamp-2.json` and `HG/` |
| `XS` | `cross-stamp.json` |
| `XS-2` | `cross-stamp-2.json` and `XS-2/` |
| `CS` | `context-stamp.json` |
| `PQ` | `question-stamp.json` and `PQ/` |
| `WS` | `wake-stamp.json` |
| `R4` · `R4-2` · `R4M` | their dirs |

The live home's stamp `~/.viola/ledger/stamps.json` is read for its sha alone before (entry 13) and after (entry 17).
No other byte of `~/.viola` is read and nothing is written there.

## What a kept frame can hold of yours (`inputs#I2` item 6)

Each kept frame is the CLI's own screen. That can include:
- its banner: version, a model word, possibly your plan or account word;
- the scratch cwd's path under your home directory;
- its footer hints.

It holds no status line (the seed sets none). The frames are 0600 under the gitignored `target/live-rig/<run>/screens/`
and never land. No frame is read into a message, a log, the evidence or the tree. The evidence file is written from
the ledger's typed fields alone: closed words, counts and delays. Hygiene's read of the evidence dir is entry 27.

## Concretization preview — `v2-27` («A key typed right after a resize», e2e)

- **Today:** «Whether a key typed right after a terminal resize reaches real claude on Linux is measured and recorded;
  if it is lost, a fix makes it arrive and a test holds it.»
- **Proposed** (the P3 block's §6, accepted by the operator as K4, `inputs#I2` item 4): «On Linux, real `claude`
  2.1.330 and 2.1.328 each run unwrapped in the live rig's own pty under its scratch home. After each of 12 resizes
  alternating 210×45 ↔ 180×40, one `x` typed 0, 20 or 100 ms after the resize returned reads `kept` (the box row's
  attribute word equal to the no-resize control's typed word) in 12 of 12 rounds per version; the readings are recorded
  as closed words in the chunk's evidence, and the arm's rehearsal over the fake agent holds the mechanics in CI. Any
  `lost` reading takes the fix half to the driver's paste (F1, a resize polled between the gate's ready and the paste
  sends the send back into the gate; human keys never held), as a plan revision on the operator's word, the human-key
  loss recorded as the CLI's.»

It is narrowed, not weakened, on the operator's own word: the fix half is the driver's paste only, since a human key
cannot be held behind a resize (`architecture · decision/human-key-never-held-behind-resize`). Claimed after the word
with `--note-file`, `chunk = 2026-11-21-key-after-a-resize`.

## Checks (P5 step 1, re-run whole after the baseline edit)

- **The block parses** (`gate.py run --dry-run`, 29 entries, no env refusal).
- **planlint** `hits: 0` (it hit once at P4 — check 2, a `matrix#v2-27 notes` amendment on an unverified cap — and
  the line was removed).
- **Lists:** `scope lists: parsed 5 items (new 0 · modify 5 · derived 0)`.
- **Snapshots:** `inputs: 2 entries — unchanged 2`.
- **Baselines:** `gate.py baseline`: seen green 5 · not seen 2 · leg 22. Entry 4 `red — exit 1, error: no tests to
  run` (vacuous; its filter control selected 1 existing rehearsal and passed). Entry 21 `recorded · exit 1` (the
  ledger absent).
- **Entries 18–19 control** (`p5-controls.md`): each exact `run`, typed in a scratch clone over a synthetic ledger,
  green on `kept`, red on `lost` and on an absent summary.
- **No row outside the P3 block's §4 is reached** (`inputs#I2` item 9).
- **Validation-1:** aligned (measure first; the fix only after the reading).
- **Host reach:** no recorded harm (the last rig round's census `0`, every rig process terminated).
- **Non-zero exits met at P5:**
  - entry 4's baseline exit 1 (vacuous, above);
  - entry 21's exit 1 (no ledger yet);
  - `gate.py run --docs-root . --entry 18 --project-root {clone}` printed `not run — leg round (the letter drives it)`
    — a round entry fires through no `--entry`, so its control was typed (`p5-controls.md`).
- **Where a letter was silent:** the validation letter names no form for controlling a `round` entry. It was
  controlled in the typed form its letter allows when the clone pair cannot hold.

**The P2 union, as printed:**
```
architecture: rows 979 · packs 10 · picked 175 (107 by every draw) · standing 34 · 55217 B
security-plan: rows 630 · packs 7 · picked 104 (58 by every draw) · standing 14 · 33555 B
design-system: rows 324 · packs 4 · picked 110 (71 by every draw) · standing 37 · 24820 B
layout-templates: rows 223 · packs 3 · picked 61 (31 by every draw) · standing 14 · 13088 B
test-plan: rows 1362 · packs 14 · picked 243 (118 by every draw) · standing 39 · 63895 B
obs-plan: rows 1034 · packs 11 · picked 180 (97 by every draw) · standing 65 · 44509 B
a11y-plan: rows 236 · packs 3 · picked 84 (55 by every draw) · standing 15 · 22648 B
rows-union: 7 master(s) · picked 957 of 4788 · rule any
```

**The rows files, read by search.** Their sum is 257 732 B, past 250 000 B, so P4 read them by search, never
whole. Every row the plan cites stands in its rows file: 18 of 18, each handle read by `grep "^{handle} "` in its
master's `-rows.md`. Two `[RETYPED]` notes stand at the picker checks, each read as its pack's row:
- `security-pick-p4.d2` line 48 → `decision/live-rig-mb-arm-keeps-no-output`;
- `tests-pick-p9.d2` line 67 → `decision/synthetic-fixture-text`.

No re-spawn ran.

## Beyond the plan

| proposal | owner / place |
|---|---|
| The driver-paste fix F1, should any reading be `lost` | a plan revision of THIS chunk, on the operator's word after the reading (`inputs#I2` item 4) |
| `cry-7mgt7ybs` (the paste's missing time bound) re-pinned whole | the wrap's route-resolve, its own entry «Paste write bounded» still the operator's |
| `cry-m267ghbx` (the `CHILD_WITHIN` comment) re-pinned whole | the wrap's route-resolve (no `viola-pty` edit here) |
| `cry-lqe3bfwx` retired as met in `1b22388f` | the wrap: `src/bin/viola-fake-agent/input.rs:74-85`, held by `rig_rehearsal_reads_the_box_empty_after_x_and_del` (`crates/viola-e2e/tests/rig_rehearsal.rs:417`) |
| `cry-234cbfoa` (the rehearsal red) re-pinned whole unless met | the wrap, acted on only at its next red |
| `cry-teayoana` (the advisory ignores' re-read) | the Epoch 4 boundary, the founder's |
| The H2 leaves re-worded with the reading: `.claude/docs/gotchas.md:109`, `.claude/docs/services/viola-pty.md:31` | the wrap's leaf cascade |

**Apply? (yes / review / cancel)**
