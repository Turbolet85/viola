# Outcome — 2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin

**Run:** 2026-11-19T06-49-43Z-implement · plan rev 12 · normal (pending) mode · re-entry at step 19

**Outcome:** stuck. The plan's own stop fired (step 21; the directive, inputs#I12: «A RED live entry stops
implement BEFORE the operator pass and before any rebuild … a RED `L1` leaves `L2` unfired»). `L1` read RED on one
atom, `lacks 'disabled-mcp: unread'`. `L2` was not fired. No operator entry was fired and no report was written.

## Gates
- Block firing 1: entries 1–8 green on the first firing. `entries 27 · green 8 · red 0 · recorded 0 · timeout 0 ·
  not-run 19`.
- The R1 case ran green inside entries 6 and 7. That closes nothing: the red stays open.
- The `bt-line` control was not fired again (plan step 19; it held at `evidence/control-1.txt`).
- The host read before the round:
  - 06:53:19Z: `CONTENDED - 10 s stalled on IO: 06:38:56Z..06:39:06Z`. Held, and the operator was told.
  - 06:54:02Z: `QUIET - 4 s stalled on IO`, re-read on the operator's word (inputs#I13: «re-read the host now; if
    it reads QUIET, fire the live round …»).
- The round (`evidence/round-065404Z.txt`):
  - `round: STOPPED at 14 (red · lacks 'disabled-mcp: unread' ✗) · legs fired 1/2 · entries 27 · green 4 · red 1
    (14) · recorded 1 · timeout 0 · indeterminate 0 · not-run 21`.
  - Entries 9, 10, 12 and 13 green; 11 recorded. P0 printed `present`.

## L1: what the start itself showed (the whole summary in `evidence/live-starts.md`)
- **Verify passed:** `verify-exit: 0`, `rows: 24`, `pass-count: 24/24`, `failing: none`,
  `browser-tools-prompt: absent`, `trust-record: pass`, `stamp-others: kept`, `keys: none`, `stop: none`,
  `child: exit 0`.
- **Release build:** `viola-build: d025ad3477a068ab`, the same hash entry 12 read.
- **Stamps:** the live home's 2.1.328 entry is the one this verify wrote, all 24 rows passing. The 2.1.325 entry and
  the other entries are kept.
- **The red:** `projects-new: 0`, `projects-new-keys: none`, `disabled-mcp: unread`, `needle-claude-json:
  absent/absent`, `needle-settings: absent/absent`, `claude-json-keys: same`, `settings-keys: same`.
  - The rig found no `projects` key present after the start and absent before it.
  - So it found no Run B entry under `<repository root>/.viola-verify-` to read `disabledMcpServers` from.
  - Why this happened was not measured. The founder's two allowed reads are spent, and nothing wider was read.
- **What L1 left:**
  - one new `~/.viola/bin/` key, `0.1.0-d025ad3477a068ab`;
  - `~/.viola/ledger/` names unchanged;
  - repository root `.viola-verify-*` reads 0 after;
  - no SIGKILL, so no leftovers to name.
- **The cap:** `L1` spent; `L2` unspent; `L1-2` unspent (only on a cause `L1` showed, after a plan revision).

## Files changed
None this run. The tree is the stopped run's (`2026-11-19T06-21-00Z-implement`), preserved. `gate.py scope`:
`scope: clean — changed 10 · listed 9 · recorded 1 (… in-intent 1 …)`.

## Deviations
- The scope record's line: ``- `crates/viola-agent-claude/src/ledger/test_cases_stamps.rs` · in-intent · serves
  step 1 · self``.
- The round was fired after the CONTENDED read, on the operator's word and a QUIET re-read (inputs#I13).

## What stuck, and Next
The `disabled-mcp` reading of plan step 9 (C6's `/mcp disable` half) rests on a new `projects` entry for Run B's
cwd. Live, on claude 2.1.328 under the real config, none appeared.

Next is the operator's and overseer's call. One option is a plan revision through `/andromeda-phase`, which would:
- decide what reads `projects-new: 0`;
- decide whether `L1-2` is spent on it;
- decide whether `L2` fires.

The re-pin waits, and the operator pass is not run.

## Guard pass lines
None: the block holds no `gate.py guard` entry.

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate block entries 1–8 (cargo, nextest, the harness, fake agents, rig rehearsals) | this run | terminated (the census below finds none) |
| round entries 9–14; `L1`'s `viola verify` and its `claude` 2.1.328 children | a leg | terminated (`census: 0` in L1's summary; the round's census found no survivors) |
| 7 `claude` sessions (the overseers and builders, `up.sh`) | NOT DERIVED: not this run's | left running: not this run's |

The census (`gate.py census --comm viola-live-rig --comm viola-fake-agent --comm claude --args
projects/viola/target/debug/viola --args projects/viola/target/release/viola`) read `census: 7 running`. All 7 are
the host's `claude` sessions; no rig, fake agent or viola build of this tree was running.

## Decisions & corrections
- The directive's host stop fired on a CONTENDED read. The operator's word (inputs#I13): «re-read the host now; if
  it reads QUIET, fire the live round … That one stall was this repository own gates, no neighbour.»
- The plan's live-red stop was honoured: `L2` unfired, no operator entry, no rebuild.
