# Materialization plan — viola · re-run, upgrade form

Development Style: agent-driven · primary language: rust (root `Cargo.toml`) · host: linux
Invocation: "the upgrade re-run at 0 pending: bring migration U68 (the PreToolUse wake guard row in
.claude/settings.json) to this project." — no structural change named → upgrade form (no upstream body read).

## Tier 1
- overview · modules · warnings · pointer-table · workflow · architecture: not re-derived — cascade-maintained,
  standing byte for byte (warnings: 9 bullets).
- imports: the template's (U01 ok) — no Edit.
- deeper-topics: recomputed from the files that exist — docs (5 core + 5 summaries + session-learnings, services
  viola · viola-core · viola-pty · viola-channel · viola-state · viola-agent-claude · viola-mcp) and rules (security ·
  host-linux · testing · verification-harness · observability · api · events · frontend · a11y · ci · testing-src)
  match the block as it stands — no Edit.
- pointer-table: U48 ok — no Edit. → CLAUDE.md is not written.

## Tier 2
Present, each `preserve`: a11y · api · ci · events · frontend · host-linux · observability · security · testing ·
testing-src · verification-harness. Host leaf: host-linux.md, this host's (U04 `noted`: 9 template lines missing
above `## Session Additions` — card only, regenerate on the operator's word). Absent: none.

## Tier 3
Present, each `preserve`: stack · conventions · commands · gotchas · workflow · the 5 summaries · session-learnings ·
services/{viola, viola-core, viola-pty, viola-channel, viola-state, viola-agent-claude, viola-mcp}. U36 `noted`
(security-summary.md lacks `cascade re-derives it`) — card only. Absent: none.

## Agent harness
`scripts/agent-run.sh` present — preserve, not compared (upgrade form). verification-harness.md present — preserve.

## Hooks
`.claude/settings.json`: U68 behind (wake absent) → add the matrix's PreToolUse `ScheduleWakeup|CronCreate` entry
(`bash ~/.claude/skills/andromeda-tools/hooks/wake-guard.sh`, timeout 5). The write and Bash guard rows and the
PostToolUse rustfmt row already equal the matrix's render (U02 ok) — kept byte for byte; `env` kept. Backup first to
`.claude/backup/settings.json.pre-setup-2026-11-16T12-24-46`. rustfmt.toml present (U05 ok).

## Code-graph pipeline
Seeded files all present; U03 lines behind 0 — nothing written.

## Code reviewer
`.claude/agents/code-reviewer.md` present — preserve.

## Gitignore
U07 ok · U56 ok — nothing appended. `.gitattributes`: U06 ok — unchanged.

## Upgrade
HEAD at 5b: 9ed65d7b88bd61a21b1814e113cddda142a4b2ca
Path set at 5b:
- .andromeda/db/log/2026-11-16T06-29-26-2026-11-16-local-command-delivery.ndjson (M)
- .andromeda/runs/2026-11-16T08-47-46-wrap/evolve-2026-11-16-local-command-delivery.json (M)
- .claude/session-handoff.md (M)

```
upgrade v1.26 · 41830636
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-gra… · lines behind: code-graph.py 0 · code-graph-views.sql 0 · code-graph-cookbook.md 0
U04 · noted · setup · .claude/rul… · 9 template line(s) missing above `## Session Additions` — regenerate on the operator's word
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag (rows)
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts … · scripts/agent-run.sh carries `ensure_fresh_artifacts`
U11 · n/a · hand · code-metrics.ndjson … · past U61 the audit's records are `metric` rows: no ledger file, no terminator
U12 · n/a · hand · .andromeda/residuals.md header · past U61 the residuals are `residual` rows: no file, no header
U13 · ok · hand · .andromeda/{doc}-amendme… · masters in the store — every sidecar entry on the form held at write (U60)
U14 · ok · hand · working-route markerless i… · 23 markerless entries · 0 introducers behind markup or after the em-dash
U35 · ok · hand · masters' lo… · masters in the store — the logs and the keyed contracts out of the body, a key regrown refused held at write (U60)
U36 · noted · noted · .claude/docs/{security,design,te… · .claude/docs/security-summary.md lacks `cascade re-derives it`
U48 · ok · hand · CLAUDE.md p… · masters in the store — CLAUDE.md's pointer rows naming `rec.py contracts {plan}` held at write (U60)
U56 · ok · setup · .gitignore base ignor… · a run's prompts/ ignored by the root .gitignore (.andromeda/runs/*/prompts/)
U57 · ok · hand · the friction family in the store — `.andromeda/db/{moved, log/*.ndjson, reco… · friction: in the store
U58 · ok · hand · the spine in the store — `version` · `epoch` · `chunk` · `carry` · `capability`… · spine: in the store
U59 · ok · hand · a chunk's and a version's documents and files in the store — `doc` · `section` ·… · docs: in the store
U60 · ok · hand · the masters in the store — the seven `.andromeda/{doc}.md` bodies, their cate… · masters: in the store
U61 · ok · hand · the bases in the store — `.andromeda/residuals.md`, `playbook.md`, `drift-base.… · bases: in the store
U62 · ok · hand · the run dirs sealed — every `.andromeda/runs/{run}/` but the sealer's own as `.a… · runs: in the store
U63 · ok · hand · the harness files as the store's render — CLAUDE.md, `.claude/rules/**`, `.cl… · harness: in the store
U65 · ok · hand · the masters as rows — every `section` and `fact` of the seven ma… · masters: 7 master(s) stand as rows
U68 · behind · setup · .claude/settings.json hooks — the PreToolUse wake guard (`ScheduleWakeup|CronCreat… · wake absent
upgrade: for setup 1 (U68) · blocks phase 0 · awaiting a door 0 · noted 2 (U04, U36) · INDETERMINATE 0 · 27 detectors of 62 registry entries
```
