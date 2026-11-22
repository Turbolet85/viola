# P5 controls — 2026-11-21-key-after-a-resize, revision 1 (validation check 4 (9))

Read 2026-11-22, plan rev 4. Round entries fire through no `--entry` (the take-up's `p5-controls.md` read `not run —
leg round (the letter drives it)`), so each control is typed: each entry's exact `run` text, parsed from the plan's
fence by `tomllib` (`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/b93393e5-6547-4341-9b9c-13a2e2bac49b/scratchpad/ctl.py`),
run under `/usr/bin/bash -o pipefail -c` inside a `git clone --shared` scratch clone at HEAD
`a32f6ef61ff830900e883b016f981020ba30aea8`, over synthetic inputs written in the clone alone (the scratch removed
after).

## Entries 19 and 20 (the readings), must-pass and must-fail

| the ledger's readings | entry 19 | entry 20 |
|---|---|---|
| KR `kept`, KR8 `unverified`, KR8-2 `kept` | exit 0 · `KR kr-reading=kept` · green | exit 0 · `KR8-2 kr-reading=kept` · green |
| KR `kept`, KR8 `kept`, KR8-2 `lost` | green | exit 0 · `KR8-2 kr-reading=lost` · red |
| KR `kept`, KR8 `kept`, KR8-2 `unverified` | green | exit 0 · `KR8-2 kr-reading=unverified` · red |
| KR `kept`, KR8 `kept`, KR8-2 start with no summary | green | exit 0 · `KR8-2 kr-reading=absent` · red |
| KR `lost`, KR8-2 `kept` | exit 0 · `KR kr-reading=lost` · red | green |

Entry 20 reads `KR8-2`'s line alone: a `KR8` line, `kept` or `unverified`, never stands in for it. Entry 19 reads
`KR`'s line alone. The ledger line shape is the rig's own (`crates/viola-e2e/src/rig/ledger.rs`, `Ledger::summary`).

## Entries 13 and 17 (the rig's out dirs), must-pass and must-fail

Entry 13 fired over a synthetic `target/live-rig/` holding three `<pid>-<start>` dirs (the two KR names among them)
and `HG/` (no match), then the dirs changed, then entry 17:

| after the start | entry 13 | entry 17 |
|---|---|---|
| one new dir, none gone | exit 0 | exit 0 · `rig-dirs: one new, none gone` · green |
| no new dir | exit 0 | exit 1 · red |
| two new dirs | exit 0 | exit 1 · red |
| one new, one listed dir gone | exit 0 | exit 1 · red |

## The live entry's `KR` atoms
This chunk's own print site (steps 4, 8), read printed before any graded use by entry 3 (the pure words) and entry 4
(the rehearsals over the fake agent), in block order (the plan's prose). Entry 15's other atoms are the rig's own keys,
as the stopped run printed them (`evidence/round-082630Z.txt` entries 14–15).
