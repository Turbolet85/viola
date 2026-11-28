# P5 controls — 2026-11-28-console-actions, revision 1

## Entry 14 — the console-key schema arm's `count` (must-pass reading; check 4 (9): the literal is the plan's own, step 11, not read from an output)
- The tree as the stopped run left it: ` 14 probe       red · last line 'count,key' ✗ (last: 'key') · exit 0 · 0.0s · 4 B → 14.log` (this run's `gate.py run --entry 14`, its trail `gate-2026-11-28-console-actions.json`).
- A copy carrying step 11's field: `schemas/diag-line.v1.json` with `"count": {"$ref": "#/$defs/count"}` added to the
  `console-key` field arm's `properties`, placed as `$TMPDIR/{scratchpad}/ctl-count/schemas/diag-line.v1.json`, fired
  as the entry, `gate.py run --chunk 2026-11-28-console-actions --docs-root . --entry 14 --project-root
  $TMPDIR/{scratchpad}/ctl-count`: ` 14 probe       green · exit 0 · 0.0s · 10 B → 14.log`. That root is no git
  repository, so the run's `delta` and `walk-class` lines read INDETERMINATE; the probe reads one file and neither.
The copy stands in the scratchpad (`ctl-count/`), not removed by a recursive delete.
