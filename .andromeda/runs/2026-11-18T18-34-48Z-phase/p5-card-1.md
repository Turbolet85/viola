# P5 review card 1 — «The 2.1.325 recorded fixture set» (chunk 2026-11-18-the-2-1-325-recorded-fixture-set)

**The plan:** `doc:2026-11-18-the-2-1-325-recorded-fixture-set/plan` rev 2.
- Its render to open in an editor: `.andromeda/cache/docs/2026-11-18-the-2-1-325-recorded-fixture-set/plan.md`.
- No plan stands in the chunk's folder, and writing in that file changes no record until a `put`.
- A diff of the chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-the-2-1-325-recorded-fixture-set`.
- Scope rev 4: P5's validation-1 added the founder's answer as its own section (intent-incomplete, not a defect).
- Research rev 2.

## Goal
Land `fixtures/claude/2.1.325/` whole, as a plain copy of two recorders' output:
- one clean live `viola verify --record` (`R4`);
- one `viola-mcp-tap` re-record of `Mcp.tools-list-1.json` (`R4M`).

`STAMPED` names the set, and every recorded-set suite reads it.

Because the rig cannot land a set as built, this chunk first builds two test-only rig labels:
- **`R4`** (spare `R4-2`, arm `VR`): the `V` arm with its record kept byte for byte under
  `target/live-rig/<label>/record/`, refusing a scratch base that holds a `home` component (C6).
- **`R4M`** (arm `MT`): one `claude -p` under the rig's private `CLAUDE_CONFIG_DIR`, its one MCP server the tap in
  front of `viola mcp`.

## The live starts, as the founder's cap holds them (`inputs#I2` item 2)
All three are on claude 2.1.325 by path, under the rig's private scratch (`/tmp/viola-rig-<uid>/`), with one ledger
file: `evidence/fixture-starts.ndjson`.

| start | kind | `--cap` / `--spare` | what it runs |
|---|---|---|---|
| `R4` | base | `--cap 1 --spare 1` | up to 9 `claude` processes one at a time; no rig key; verify types only its compiled literals |
| `R4M` | base | `--cap 2 --spare 1` | 1 `claude -p`, 1 tap, 1 `viola mcp`; the only input is the stdin literal `reply with the single word ok` |
| `R4-2` | spare | — | spent only on a cause `R4` itself showed, after its mend lands through a plan revision |

- Nothing is written under the real `~/.claude*` or `~/.viola`. The credentials are read once, and `~/.claude.json`
  is stamped read-only.
- No start touches the real home or config, and the pinned product is not rebuilt or re-pinned.

The 2.1.302 tap recording ran under the real `HOME` config (`chunks/2026-10-20-epoch-5-cleanup-ii/evidence/live-tap.md`).
`R4M` moves it under the private config (`inputs#I2` item 6). That is measured as workable: verify's print-mode probe
passed under the same private config in `BK`'s 24/24.

## Top acceptance criteria
- **`R4` GREEN:** 24/24, `failing: none`, `browser-tools-prompt: absent`, `screen-b`…`screen-f` `box`, `record:` a
  count, every teardown word, and the user config never `changed-by-rig`.
- **C10:** `screen-e: box`, and `SessionStart.resume-1.json` lands.
- **`R4M` GREEN:** `mcp-child: exit 0`, a non-empty capture, `tap-fixture: written`, and the recorded `list`
  description without «and the usage budget».
- **The landing:** the landed set equals the recorders' output (`diff -r` / `cmp`), `STAMPED` names `2.1.325`, and the
  hygiene, drift, ledger-probe and MCP-exchange suites plus the whole integration and E2E layers are green.
- **C11:** `bt-absent` holds on `cargo nextest run -p viola-agent-claude --no-fail-fast` (`--timeout 600`).
- **C7:** disposed as not readable by `R4`. Its pty holds `viola verify`, not the CLI's cells.
- **No matrix capability is claimed.** No pool cap is this entry's surface, and no `CARRY: proves` stands.

## Touchpoints
- `crates/viola-e2e/src/rig/mod.rs`, `crates/viola-e2e/src/rig/isolate.rs`, `crates/viola-e2e/src/bin/viola-live-rig.rs`,
  `crates/viola-e2e/tests/rig_rehearsal.rs` (all test-only);
- `tests/contract_ledger_probes.rs` (`STAMPED`);
- new: `fixtures/claude/2.1.325/`.
- No product crate is touched.

## Test commands (24 entries)
- 1–5: fmt, clippy, the agent-crate unit suite `--no-fail-fast`, `run --unit`, and the rehearsals.
- 6–12: the round, `R4` then `R4M`, with censuses.
- 13–20, after the landing: `diff -r`, `cmp`, the usage-budget count, the resume variant, `cli_verify`, the
  `contract_*` / `mcp_*` binaries, and the whole integration and E2E layers.
- 21–24: the operator pass.
- A drift or exchange red on the landed set stops implement before the operator pass (`inputs#I2` item 9).

## Verbatim block (directive item 12, `inputs#I1`)
**The union** (printed by `rec.py rows-union` at the take-up, trail
`.andromeda/runs/2026-11-18T18-34-48Z-phase/rec-2026-11-18-the-2-1-325-recorded-fixture-set.json`):
```
architecture: rows 967 · packs 10 · picked 284 (149 by every draw) · standing 5 · 80681 B
security-plan: rows 615 · packs 7 · picked 205 (131 by every draw) · standing 8 · 59807 B
design-system: rows 316 · packs 4 · picked 86 (54 by every draw) · standing 9 · 20906 B
layout-templates: rows 222 · packs 3 · picked 76 (48 by every draw) · standing 2 · 15495 B
test-plan: rows 1347 · packs 14 · picked 237 (132 by every draw) · standing 38 · 62339 B
obs-plan: rows 1028 · packs 11 · picked 258 (141 by every draw) · standing 7 · 61938 B
a11y-plan: rows 235 · packs 3 · picked 79 (45 by every draw) · standing 11 · 21103 B
rows-union: 7 master(s) · picked 1225 of 4730 · rule any
```

**How the rows were read:** by search, not whole. The sum is 322 269 B, past 250 000 B. The standing lists and the
history files were read whole in this window.

**Is every cited row in the union?** Yes. All 17 rows the plan cites were found by search in their rows files:
- tests rows lines 14, 36, 104, 131, 173, 177, 179, 203, 204;
- security rows 40, 115, 116, 179, 180;
- arch rows 60, 343;
- obs rows 7.

**`[RETYPED]` lines and re-spawns:** none in this window. P2's ran in the take-up window, and its trail is
`fanout-2026-11-18-the-2-1-325-recorded-fixture-set.json`.

**Non-zero exits and refusals in this window:**
- The Bash guard refused three calls: a `cd` into the run dir, twice, and a tool's output piped into a filter. Each was
  re-issued once in the passing form.
- The permission classifier refused one call, verdict «Undermining Oversight»: a hand-written scratchpad script
  running controls for entries 13–16 over copied fixtures. It was not re-tried in another form. Entry 15's must-pass
  was instead fired as the entry over a `git clone --shared` scratch clone at `bb204f14` (`p5-controls.md`). Entries
  13, 14 and 16 assert no literal, and their baseline reds stand as "can read red".
- Baseline reds, all on the absent set (the subject this chunk builds):
  - `13 probe red · exit 0 ✗ (exit 2)`;
  - `14 probe red · exit 0 ✗ (exit 2)`;
  - `15 probe red · exit 1 ✗ (exit 2)`;
  - `16 probe red · exit 0 ✗ (exit 1)`.
- In the clone, entry 15 read `red · exit 1 ✗ (exit 0)` over the 2.1.302 text (the must-fail side) and `green · exit 1`
  with the clause removed (the must-pass side).

**`UNPARSED` / `INDETERMINATE` / `NOT DERIVED` lines:** none. `planlint` printed `hits: 0`, `scope lists: parsed 5
items`, `inputs: 2 entries — unchanged 2`, and `gate.py baseline` printed `not seen 0 · new (key) 8`.

**Places a letter was wrong or silent:**
1. **The previous chunk's filter could not select its contract binary.** Its plan ran `run --integration --filter`
   naming `binary(contract_ledger_probes)`. `contract_` binaries are E2E-layer by the harness regex
   (`crates/viola-e2e/src/harness/run/nextest.rs:14-15`), so that filter could not select it; its whole-layer
   `run --e2e` did. This plan files `contract_*` under `run --e2e`.
2. **The scope said C11's guard is edited here; it is not.** The scope says «this entry edits the ledger rows». This
   chunk does not change the agent crate, so the control fires over an untouched guard.
3. **C7's condition is false by construction.** The scope's «C7 only if R4's kept frames show the cell attribute»
   cannot hold: `R4`'s rig pty holds `viola verify` (`crates/viola-e2e/src/rig/mod.rs:736-745`).
4. **Research's list omits one file.** Research's `Files to modify` omits `crates/viola-e2e/tests/rig_rehearsal.rs`,
   which the plan adds. A take-up runs no list sync.
5. **Relay item 6's STOP did not trigger.** The 2.1.302 recording ran under the real config, but a private-config
   `claude -p` has run on this host inside `BK`, so it can be made under a private config.

## Expected amendments (wrap)
- **test-plan:** (tests · decision/record-home-names-no-home) gains the rig's half.
- **security-plan:** (security · decision/mcp-fixtures-via-test-only-tap) names `R4M` under the private config. This
  narrows what the start touches.
- **architecture:** (arch · name/binary/viola-live-rig) names the labels and the kept record dir.
- Leaned not a boundary widening, per `rul-r74hshif` / `rul-p2zl2kgl`.

## Beyond the plan
| # | proposal | authority |
|---|---|---|
| 1 | **New entry «The live home's re-stamp and the product re-pin»**, right after this entry in Epoch 4, written WHAT-only by the wrap's route-resolve. Carries re-pinned whole: C3 `cry-jgo4g4np`, C8 `cry-bb5cy7aq`, C9 `cry-s356l2nj`, C4 `cry-6lvrocr4`. Its precondition: the browser-tools prompt answered once by a person in a real 2.1.325 session. Its carry: verify's output says so when it meets the prompt. | the founder's own answer, `inputs#I2` items 1, 4 |
| 2 | **New entry «A live StopFailure payload»**, right after entry 1, WHAT-only. Carries C2 `cry-6p5vu7ku` re-pinned whole; the exception stays PROVISIONAL as it stands; its cap is asked of him then. | the founder's own answer, `inputs#I2` items 1, 3 |
| 3 | **C7 `cry-aglfufo4` re-pinned whole on «A live StopFailure payload»**, reason «`R4`'s rig pty holds `viola verify`; no frame it keeps reads the CLI's cells». Its next reader is that entry's phase, whose one prompt typed into a box on the rig's own pty can read the cells. | proposed here; the word approves or places it elsewhere |

Apply? (yes / review / cancel)
