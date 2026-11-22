# P5 controls — 2026-11-21-key-after-a-resize (validation check 4 (9))

Read 2026-11-22, the tree at `a32f6ef61ff830900e883b016f981020ba30aea8` with the chunk's uncommitted take-up.

## Baselines (fired through `gate.py run --entry {n}`, the trail `gate-2026-11-21-key-after-a-resize.json`)
- entry 4 `bash scripts/agent-run.sh run --integration --filter 'test(/rig_rehearsal_kr/)'` — red · exit 1 · 1.38 s:
  `error: no tests to run` — vacuous, the filter selects only the rehearsals step 6 writes (0 on the untouched tree).
- entry 21 (the ledger read) — recorded · exit 1 · 0.03 s: the ledger file absent on the untouched tree (report-only;
  it can fail).

## Controls
- **entry 4's filter control** — the entry's command with its filter operand alone changed to an existing rehearsal,
  typed as the tool types it: `/usr/bin/bash -o pipefail -c "bash scripts/agent-run.sh run --integration --filter
  'test(/rig_rehearsal_hb/)'"` on the tree → exit 0, `"ok":true`, `nextest-integration` passed 1 (archived
  `target/run-archive/567`). The layer and the filter form select a rehearsal of this file; the `KR` filter selects
  only what step 6 writes.
- **entries 18 and 19 (the readings), must-pass and must-fail** — round entries fire through no `--entry`
  (`gate.py run --docs-root . --entry 18 --project-root {clone}` printed `not run — leg round (the letter drives it)`),
  so the clone pair cannot hold and the control is typed: each entry's exact `run` text (read byte-equal against the
  plan's fence by `$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/0e3d4c6a-1cf2-4f8e-aa0c-e2230a707897/scratchpad/ctl-same.py`),
  under `/usr/bin/bash -o pipefail -c`, inside a `git clone --shared` scratch clone at HEAD
  `a32f6ef61ff830900e883b016f981020ba30aea8`, over a synthetic ledger written in the clone alone
  (`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/0e3d4c6a-1cf2-4f8e-aa0c-e2230a707897/scratchpad/ctl-reading.sh`):

  | the ledger's readings | entry 18 | entry 19 |
  |---|---|---|
  | KR `kept`, KR8 `kept` | exit 0 · `KR kr-reading=kept` · green | exit 0 · `KR8 kr-reading=kept` · green |
  | KR `lost`, KR8 `kept` | exit 0 · `KR kr-reading=lost` · red | exit 0 · `KR8 kr-reading=kept` · green |
  | KR `kept`, KR8 `lost` | exit 0 · `KR kr-reading=kept` · green | exit 0 · `KR8 kr-reading=lost` · red |
  | no summary line | exit 0 · `KR kr-reading=absent` · red | exit 0 · `KR8 kr-reading=absent` · red |

  Each reads its own version alone, passes on `kept` and fails on `lost` and on an absent summary. The ledger line
  shape is the rig's own (`crates/viola-e2e/src/rig/ledger.rs`, `Ledger::summary`: `{"v":1,"kind":"summary","label",
  "ts","reading":{…}}`).
- **The live entries' `KR` atoms** — this chunk's own print site (step 4); read printed before any graded use by
  entry 3 (the pure words) and entry 4 (the rehearsals over the fake agent), in block order (the plan's prose).
