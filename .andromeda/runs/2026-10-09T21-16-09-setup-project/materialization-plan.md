# Materialization plan — viola · re-run, upgrade form · 2026-10-09T21:16:09Z

Setup: project viola · Development Style agent-driven (`architecture.md:458`, `:507`) · stack fragment `rust`
(`architecture.md:13`, §Stack's language row) · re-run — upgrade form: no upstream body read. The invocation's words
("Re-run on the live project for the one pending upgrade (U02). Present the card whole and wait for the answer.")
name no structural change in the masters.

Upstreams present (existence only): `input.md` · `architecture.md` · `security-plan.md` · `design-system.md` ·
`layout-templates.md` · `test-plan.md` · `obs-plan.md` · `a11y-plan.md` · `master-route.md`.

## Tier 1 — CLAUDE.md (125 lines before this run; backup `.claude/backup/CLAUDE.md.pre-setup-2026-10-09T21-16-09`, md5 `b1fa4c5dbfb42b4c65db6948e6a9526e`)

| block | disposition |
|---|---|
| `overview` | not re-derived — cascade-maintained, stands byte for byte |
| `modules` | not re-derived — cascade-maintained, stands byte for byte |
| `warnings` | not re-derived — cascade-maintained, 10 bullets, stands byte for byte |
| `pointer-table` | not re-derived — cascade-maintained, stands byte for byte (U48 `ok`: no row edit) |
| `workflow` | not re-derived — cascade-maintained, stands byte for byte |
| `architecture` | not re-derived — cascade-maintained, stands byte for byte |
| `imports` | the template's (U01 `ok`) — one line, `@.claude/session-handoff.md`; no Edit |
| `deeper-topics` | recomputed from the files that exist — the rule list differs by two files; ONE Edit, below |
| `USER:session-learnings` | never touched (9 bullets) |

`deeper-topics`, recomputed: docs — the 5 summaries (`security` · `design` · `tests` · `obs` · `a11y`), the 5 core
(`stack` · `conventions` · `commands` · `gotchas` · `workflow`), `services/` × 8 (`viola` · `viola-core` ·
`viola-pty` · `viola-channel` · `viola-state` · `viola-agent-claude` · `viola-mcp` · `viola-ui`),
`session-learnings.md` — equal to the block as it stands. Rules — `.claude/rules/` holds 11 files; the block names 9.
The two it does not name: `ci.md` (`paths:` `.github/**`) and `testing-src.md` (`paths:` `src/**/*.rs`,
`crates/*/src/**/*.rs`). Both were added by commit `211da16` (2026-10-09T17:01Z), after the last setup run
(2026-10-09T14:11:37Z), as a chunk's own stated work.

**The deeper-topics Edit** — one anchored Edit inside `GENERATED:setup:deeper-topics`, on the rule list's one line
(`CLAUDE.md:105`): `` · `ci.md` · `testing-src.md` `` appended after `` `a11y.md` ``. Every other byte of the block
and of the file stands. +0 lines.

## Tier 2 — `.claude/rules/` (11 present: 10 `preserve`, 1 `regenerate`)

**`host-linux.md` — `regenerate`**, the operator's word at the Phase 7 card ("regenerate host-linux.md",
2026-10-09): `upgrade.py apply --root . --id U04 --regenerate --run-dir {run_dir}`, after its `--dry-run`. Backup
`.claude/backup/host-linux.md.pre-setup-2026-10-09T21-16-09-setup-project` (md5 `d7d286a7b374d3c2bbd564ab3a770169`);
the leaf after the write md5 `d9fdc1447929d06a0ce7f77fa1343345`. The body above `## Session Additions` went 50 → 47
lines: one bullet the template's linux render does not carry left it — §Encoding & heredocs' `The transport
collapses a BACKSLASH PAIR …` (3 lines). The 6 lines from `## Session Additions` on are byte-identical, read back by
the tool. Every other file below is `preserve`.

`a11y.md` · `api.md` · `ci.md` · `events.md` · `frontend.md` · `host-linux.md` (the host leaf, U04 `ok` at step 1b) ·
`observability.md` · `security.md` · `testing.md` · `testing-src.md` · `verification-harness.md`. Nothing planned
beyond them. `ci.md` and `testing-src.md` are a chunk's files, not setup's render: `preserve`, as every present
leaf. Absent: none named by health checks 5 · 9 · 13.

## Tier 3 — `.claude/docs/` (19 present, each `preserve`)

Core 5: `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md`. Summaries 5:
`security-summary.md` · `design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md`. Services 8:
`viola.md` · `viola-core.md` · `viola-pty.md` · `viola-channel.md` · `viola-state.md` · `viola-agent-claude.md` ·
`viola-mcp.md` · `viola-ui.md`. `session-learnings.md` present (wrap territory). Absent: none.

## Agent harness (agent-driven)

`scripts/agent-run.sh` and `scripts/agent-run.ps1` present — `preserve`; no fresh render is made on the upgrade
form, nothing is compared. `.claude/rules/verification-harness.md` present — `preserve`.

## Code reviewer

`.claude/agents/code-reviewer.md` present — `preserve` (the pick for the language row is `rust`; not rendered).

## Hooks

`.claude/settings.json` — **U02 `behind`** (`write inline · bash inline`): Phase 5 step 2 acts.

- Backup first: `.claude/backup/settings.json.pre-setup-2026-10-09T21-16-09`.
- The PreToolUse entry matching `Edit|MultiEdit|Write|NotebookEdit` (the write guard, an inline `bash -c` command) is
  REPLACED by the matrix's entry: `"timeout": 5`, `"command": "bash ~/.claude/skills/andromeda-tools/hooks/write-guard.sh"`.
- The PreToolUse entry matching `Bash` (the Bash guard, an inline `bash -c` command) is REPLACED by the matrix's
  entry: `"timeout": 5`, `"command": "bash ~/.claude/skills/andromeda-tools/hooks/bash-guard.sh"`.
- The PostToolUse entry matching `Edit|MultiEdit|Write` (the stdin prologue + `rustfmt "$f"` on `*.rs`,
  `"timeout": 30`) is the matrix's Rust render already: it stands byte for byte. No linter row (clippy is a gate),
  no type-checker row (Rust has none).
- `env` (`PYTHONUTF8` · `PYTHONIOENCODING`) present as the render sets it: stands. No other key or entry exists in
  the file, so nothing user-managed is in reach of the write.
- The install's scripts read at this step: `~/.claude/skills/andromeda-tools/hooks/write-guard.sh` (1179 B) and
  `bash-guard.sh` (5110 B), both present and non-empty. The formatter row is resolved from the existing settings
  (hooks-matrix precedence 3; obs-plan §3 is never read on the upgrade form).

`rustfmt.toml` — U05 `ok`: no write.

## Gitignore · gitattributes

`.gitignore` — U07 `ok` (every base ignore decided by the root `.gitignore`, depth 2 included): no write.
`.gitattributes` — U06 `ok` (carries `* text=auto eol=lf`): no write.

## Code-graph pipeline

Planes by manifest: rust, ts. `scripts/code-graph.py` · `code-graph-views.sql` · `scip_pb2.py` · `requirements.txt` ·
`code-graph-cookbook.md` present; U03 `ok`: no write. `.andromeda/cache/` present (`.refresh-done`: `rust ok 30s
4370/22729` · `ts ok 0s 7/1`; `tree.db.commit` = `f0a0dd16ceb3979ba18f3b0b486e5ad739dc24ad`, equal to HEAD).

## Seeded operational artifacts

`.andromeda/state.yaml` · `.claude/session-handoff.md` · `.andromeda/drift-base.md` · `.andromeda/playbook.md` ·
`.claude/docs/session-learnings.md` — all present, none written.

## Upgrade

HEAD at Setup 5b: `f0a0dd16ceb3979ba18f3b0b486e5ad739dc24ad`. `route.py cursor`: `records 47 · complete 47 · pending 0
· gated 0` · `half-promote 0 of 47 stamped lines vs 47 master records`.

Path set at Setup 5b (`git status --porcelain=v1 -z --untracked-files=all`), all expected-transient bookkeeping:

```
 M .andromeda/friction-log.ndjson
 M .andromeda/runs/2026-10-09T20-50-14-wrap/evolve-2026-10-09-inner-cr-and-crlf-in-a-sent-text.json
 M .claude/session-handoff.md
```

The listing, verbatim (`upgrade.py detect --root .`, read once at step 1b):

```
upgrade v1.8 · 30b07c54
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write inline · bash inline
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · ok · setup · .claude/rules/host-{os}.md · every template line present above `## Session Additions`
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · ok · noted · .claude/docs/workflow.md · .claude/docs/workflow.md carries `it never commits`
U10 · ok · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh carries `ensure_fresh_artifa…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 33 markerless entries · 0 introducers behind markup or after t…
U35 · ok · hand · masters' logs + keyed contracts · ok: infra K, test K, test L, obs K, obs L, a11y K, a11y L, security…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
U48 · ok · setup · CLAUDE.md pointer rows · 4 of 4 indexes named
upgrade: for setup 1 (U02) · awaiting a door 0 · noted 1 (U36) · INDETERMINATE 0 · 17 detectors of 45 registry entries
```

Rows that act: **U02** (Phase 5 step 2, the two entries above). No setup-class row reads `INDETERMINATE`. U11 and
U12 read `ok` — Phase 7.5 step 1 has nothing to apply. U04 reads `ok` — step 8a does not run, no host re-seed
(`upgrade.py host` prints `host: linux`; the leaf is `host-linux.md`). U36 is `noted`: the card names it;
`regenerate {leaf}` stays the operator's word.

## Consistency (step 9)

Two writes planned outside the run dir and the backups: `.claude/settings.json` (two PreToolUse entries replaced by
the matrix's calls, every other byte standing) and `CLAUDE.md` (one line of the `deeper-topics` rule list, +0 lines:
125 stays 125, limit 200). A third came at the card, on the operator's word: `.claude/rules/host-linux.md`
regenerated above `## Session Additions` (Tier 2, above). Every other artifact stands. No `## Adoption` (upgrade form).
