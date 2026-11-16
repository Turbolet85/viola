# P5 controls — 2026-11-16-granular-api-error-cause

## Entry 14 — the fixtures probe (no recorded fixture added or changed)
Clone: `git clone --shared` of the project at HEAD bde97ba87929274a879458c8a329146daefd6055, at
`$TMPDIR/claude-1000/-home-turbolet-dev-projects-viola/f500cc8d-b3a2-4ed9-b554-b0f197eb828d/scratchpad/ctl-fixtures`.
Fired as the entry: `gate.py run --chunk 2026-11-16-granular-api-error-cause --docs-root . --entry 14 --project-root {the clone}`.
- unmodified: ` 14 probe       green · exit 0 · 0.01s · 2 B → 14.log` (last line 0) — the same reading as the real tree
- input minted (an untracked `fixtures/claude/2.1.325/StopFailure.default.json`): ` 14 probe       red · last line '0' ✗ (last: '1') · exit 0`
Verdict: the probe reads the change it guards (red on a minted fixture, green without one).
