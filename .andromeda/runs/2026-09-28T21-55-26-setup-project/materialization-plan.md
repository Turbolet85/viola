# Materialization plan — viola (re-run · the upgrade)

Run: `2026-09-28T21-55-26-setup-project` · Development Style: agent-driven · stack fragment: rust ·
generating host: Windows (host-win32 applies). Overseer-approved dry run (setup 2 — U02, U07 · doors 0);
the founder asked for this upgrade.

## Upgrade
5b HEAD: `6c1419142cb6d7d2cb00b0e71be8bb76dc8e6ee8`
5b path set (all expected-transient bookkeeping):
- ` M .andromeda/friction-log.ndjson`
- ` M .andromeda/runs/2026-09-28T21-04-49-wrap/evolve-2026-09-28-mutation-testing-to-the-epoch-boundary.json`
- ` M .claude/session-handoff.md`

5b route cursor: `records 22 · complete 22 · pending 0` · `half-promote 0 of 22 stamped lines vs 22 master records`.

`upgrade.py detect --root .` (exit 0; byte-identical to the dry run's reading):
```
upgrade v1.0 · c78f62d3
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write relay-backslash · bash current
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · ok · setup · .claude/rules/host-win32.md · every template line present above `## Session Additions`
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · behind · setup · .gitignore base ignores · not ignored by the root .gitignore: __pycache__/ · zz/a/__pycache__/
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh carries `ensure_fresh_artifa…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · n/a · setup · .andromeda/residuals.md header · residuals.md absent (wrap lazy-creates it with the current header)
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 37 markerless entries · 0 introducers behind markup or after t…
upgrade: for setup 2 (U02, U07) · awaiting a door 0 · noted 0 · INDETERMINATE 0 · 14 detectors of 29 registry entries
```
U03's truncated facts were read by hand: `code-graph.py`, `code-graph-views.sql` (fenced bodies) and the cookbook
above its `Project-specific query learnings` marker each differ from the template by 0 lines — current.

Acts: U02 → P5 step 2 (replace the setup-rendered write guard; Bash guard and PostToolUse rustfmt row already equal
the matrix). U07 → P5 step 3 (append `__pycache__/`). P7.5: U11 ok, U12 n/a — nothing to apply.

## Tier 1 — CLAUDE.md
Back up to `.claude/backup/CLAUDE.md.pre-setup-2026-09-28T21-55-26`. Re-render GENERATED:setup from this checkpoint;
preserve `USER:session-learnings` verbatim. @imports = `.claude/session-handoff.md` only.

- **Overview / modules / pointer table / architecture / deeper-topics:** reconciled against arch (§Project Intent,
  §Stack, §Workspace crates, §Standard Contracts, §Cross-cutting Patterns) and the plans' section anchors — no change.
  The module map keeps `viola-mcp` / `viola-ui` (arch-planned crates; not yet under `crates/`).
- **Workflow:** key commands equal arch lines 435–440 (fmt, clippy with `fake-agent`, check, deny) + the harness.
  Observation held for the operator, NOT written (outside the approved U02/U07 scope): the lead-in parenthetical
  "(the workspace lands with the first chunk; until then they have nothing to act on)" is stale — 22 chunks complete.
- **Top-10 universal warnings** (security > a11y > obs > tests > design), unchanged:
  1. Human always wins; `viola hook` exits 0, no stderr, fails open (security Universal · Logging; arch Fail open).
  2. Upstream text is content; `statusline_command` the only shell-out; direct spawn (security Code Patterns; arch).
  3. NEVER-log floor (security Secrets; obs PII Scrubbing).
  4. External errors codes + fixed messages only (security Logging; obs Error Reporting).
  5. No config/env/flag disables a control or widens redaction (security Universal; obs Logs · PII).
  6. Bound every input (security Input).
  7. Disk state modes, single writers, one write per line (security Data Protection · Universal; arch Crash-safe).
  8. stdout reserved; no print macros; `obs_event!` under `skip_all` (obs Logs · Spans).
  9. `panic = "unwind"`, custom panic hook first (obs Error Reporting; arch).
  10. Tokio containment; no C crates; Claude shapes only in agent-claude + ledger rows (arch Cross-cutting).
  Rejected (audit): exec-form absolute pinned path and verify-stamp-gates-dialog — already Tier 2 in the
  always-loaded `security.md`; the fake-agent seam carve-out — carried in `security.md`; design / a11y / test bans —
  path-scoped, Tier 2 (`frontend.md`, `a11y.md`, `testing.md`).
- Result: the GENERATED render equals the present file byte for byte — no write to `CLAUDE.md` (124 lines ≤ 200).

## Tier 2 — .claude/rules/ (disposition: preserve all)
security · host-win32 · testing · verification-harness · observability · api · events · frontend · a11y — all
present, preserved whole with their `## Session Additions`. U04 ok (no host-win32 drift note).

## Tier 3 — .claude/docs/ (disposition: preserve all)
Core 5 (stack · conventions · commands · gotchas · workflow) · summaries 5 (security · design · tests · obs · a11y) ·
services 8 (viola · viola-core · viola-pty · viola-channel · viola-state · viola-agent-claude · viola-mcp · viola-ui)
· `session-learnings.md` present. U09 ok.

## Agent harness
agent-driven → `scripts/agent-run.{sh,ps1}` present → preserved (only-if-missing). U10 ok. Fresh-render comparison
not performed (the scripts are project-evolved; no plan change to the 5-command params was found in Phase 0).
`verification-harness.md` present → preserved.

## Hooks — .claude/settings.json
Precedence: obs-plan §3 / arch Code quality → rustfmt formatter; clippy is a gate (no write-time row); no Rust
type-checker hook. Back up to `.claude/backup/settings.json.pre-setup-2026-09-28T21-55-26`, then REPLACE the
setup-rendered entries with the matrix form:
- PreToolUse `Edit|MultiEdit|Write|NotebookEdit` write guard — behind (relay-backslash) → matrix form
  (`… | tr -d "\r" | tr "\134" "/"` in one pipe).
- PreToolUse `Bash` guard — equals the matrix; unchanged.
- PostToolUse `Edit|MultiEdit|Write` rustfmt row — equals the matrix prologue; unchanged.
- `env` PYTHONUTF8 / PYTHONIOENCODING present — kept; `permissions` and other user keys kept byte for byte.
`rustfmt.toml` present (edition 2024) — no formatter-config write, no reflow step.

## Code-graph pipeline
Planes: rust (root `Cargo.toml`) + ts (tracked `tsconfig.json`). All five files present; triple current;
`scip_pb2.py` / `requirements.txt` only-if-missing → preserved. `.andromeda/cache/` present.

## Code reviewer
`.claude/agents/code-reviewer.md` (rust) present → preserved.

## Gitignore / gitattributes
U07: append `__pycache__/` (git's matcher leaves `__pycache__/` and `zz/a/__pycache__/` unignored; the present
`scripts/__pycache__/` covers only `scripts/`). Other base lines and the rust fragment are already covered.
`.gitattributes` carries `* text=auto eol=lf` (U06 ok); index 2195 i/lf · 56 i/none · 1 i/-text · 0 i/crlf|mixed →
unchanged.

## Seeds
state.yaml · session-handoff.md · drift-base.md · playbook.md · session-learnings.md — all present; none written.
