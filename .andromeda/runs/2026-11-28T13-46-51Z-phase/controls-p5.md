# P5 controls — 2026-11-28-console-actions

## Entry 12 — the console-action schema arm (must-pass reading; check 4 (9): the literal is the plan's own, not read from an output)
Fired as the entry, `gate.py run --chunk 2026-11-28-console-actions --docs-root . --entry 12 --project-root $TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/a5bddab8-dcc1-4c25-a776-a7c56fd5d598/scratchpad/ctl-schema`, over a `git clone --shared` of the project at HEAD `a07542e9a4bf497b6fa6b2afc26157af3c5b4ba7`:
- unmodified clone: ` 12 probe       red · last line 'verb,exit_code' ✗ (last: '') · exit 0 · 0.0s · 0 B → 12.log` — what the real tree read.
- the clone carrying step 7's arm (`{"if": event const console-action, "then": {verb enum, exit_code integer, required [verb, exit_code]}}` appended to `schemas/diag-line.v1.json`'s `allOf`, left uncommitted): ` 12 probe       green · exit 0 · 0.0s · 15 B → 12.log`.
The clone stands in the scratchpad (`ctl-schema/`), not removed by a recursive delete; named on the card.
