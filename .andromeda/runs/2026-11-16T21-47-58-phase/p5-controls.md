# P5 controls — 2026-11-16-scheduled-wake-count

## Entry 15 — the fixtures probe (no recorded fixture added or changed)
Clone: `git clone --shared` of the project at HEAD 75d636b584c3fe17a1744caf947fc044c263ef2f, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/6d10b345-bd92-40ea-822d-4a2377f8a3c3/scratchpad/ctl-fixtures`.
Fired as the entry: `gate.py run --chunk 2026-11-16-scheduled-wake-count --docs-root . --entry 15 --project-root {the clone}`.
- unmodified: ` 15 probe       green · exit 0 · 0.01s · 2 B → 15.log` (last line 0) — the same reading as the real tree
- input minted (an untracked `fixtures/claude/2.1.302/Stop.held-1.json` holding a `session_crons` element):
  ` 15 probe       red · last line '0' ✗ (last: '1') · exit 0`
Verdict: the probe reads the change it guards (red on a minted fixture, green without one).
