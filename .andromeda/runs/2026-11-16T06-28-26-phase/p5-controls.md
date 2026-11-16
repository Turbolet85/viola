# P5 controls — 2026-11-16-local-command-delivery

## Entry 11 — the `fixtures/claude` probe (validation check 4 (9), an inline guard's known-positive control)
- Clone: `$TMPDIR/{scratchpad}/ctl-fixtures`, `git clone --shared` of the project, detached at `c80fe742dbf9`.
- Unmodified, fired through `gate.py run --docs-root . --entry 11 --project-root <clone>` at 2026-11-16T08:01:39Z:
  `11 probe green · exit 0 · 0.01s · 2 B → 11.log` (last line 0) — the clone reads what the tree reads.
- Input: one untracked file minted in the clone, `fixtures/claude/2.1.302/SessionStart.control-1.json`; fired the
  same way at 08:01:40Z: `11 probe red · last line '0' ✗ (last: '1') · exit 0` — the probe reads a new fixture red.
