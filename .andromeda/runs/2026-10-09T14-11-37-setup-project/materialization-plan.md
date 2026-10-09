# Materialization plan — viola · re-run, upgrade form · 2026-10-09T14:11:37Z

Setup: project viola · Development Style agent-driven (`architecture.md:458`, `:507`) · stack fragment `rust`
(`architecture.md:13`, §Stack's language row) · re-run — upgrade form: no upstream body read. The invocation's words
("Re-run on the live project for the one pending upgrade (U48). Present the card whole and wait for the answer.")
name no structural change in the masters.

Upstreams present (existence only): `input.md` · `architecture.md` · `security-plan.md` · `design-system.md` ·
`layout-templates.md` · `test-plan.md` · `obs-plan.md` · `a11y-plan.md` · `master-route.md`.

## Tier 1 — CLAUDE.md (124 lines before this run; backup `.claude/backup/CLAUDE.md.pre-setup-2026-10-09T14-11-37`, md5 `24b344c42d36d8fc848a970a6ad48d68`)

| block | disposition |
|---|---|
| `overview` | not re-derived — cascade-maintained, stands byte for byte |
| `modules` | not re-derived — cascade-maintained, stands byte for byte |
| `warnings` | not re-derived — cascade-maintained, 10 bullets, stands byte for byte |
| `pointer-table` | not re-derived — cascade-maintained; the U48 row edit below, every other byte stands |
| `workflow` | not re-derived — cascade-maintained, stands byte for byte |
| `architecture` | not re-derived — cascade-maintained, stands byte for byte |
| `imports` | the template's (U01 `ok`) — one line, `@.claude/session-handoff.md`; no Edit |
| `deeper-topics` | recomputed from the files that exist — equal to the block as it stands; no Edit |
| `USER:session-learnings` | never touched (8 bullets) |

**The U48 row edit** — four anchored Edits inside `GENERATED:setup:pointer-table`, each directly after the section
its row names, in the template's spelling (`claude-md-template.md:44`, `:48`–`:50`):

| line | row | inserted after | text inserted |
|---|---|---|---|
| 51 | Directory tree · resource registry | `§Infrastructure Patterns` | `` (keyed: `.andromeda/registries/architecture-contracts.toml`, one file per key)`` |
| 59 | Test harness | `§3` | `` (keyed: `.andromeda/registries/test-plan-contracts.toml`, one file per key)`` |
| 61 | Obs pipeline · event catalog · CI gates | `§3` | `` (keyed: `.andromeda/registries/obs-plan-contracts.toml`)`` |
| 62 | A11y harness · per-SC map · ARIA catalog | `§3` | `` (keyed: `.andromeda/registries/a11y-plan-contracts.toml`)`` |

The four indexes stand in `.andromeda/registries/` (`a11y-plan-contracts.toml` · `architecture-contracts.toml` ·
`obs-plan-contracts.toml` · `test-plan-contracts.toml`, beside `contracts/`).

`deeper-topics`, recomputed: docs — the 5 summaries (`security` · `design` · `tests` · `obs` · `a11y`), the 5 core
(`stack` · `conventions` · `commands` · `gotchas` · `workflow`), `services/` × 8 (`viola` · `viola-core` ·
`viola-pty` · `viola-channel` · `viola-state` · `viola-agent-claude` · `viola-mcp` · `viola-ui`),
`session-learnings.md`; rules — `security.md` and `host-linux.md` (always loaded) · `testing.md` ·
`verification-harness.md` · `observability.md` · `api.md` · `events.md` · `frontend.md` · `a11y.md`.

## Tier 2 — `.claude/rules/` (9 present, each `preserve`)

`a11y.md` · `api.md` · `events.md` · `frontend.md` · `host-linux.md` (the host leaf, U04 `ok` — current) ·
`observability.md` · `security.md` · `testing.md` · `verification-harness.md`. Nothing planned beyond them. Absent:
none named by health checks 5 · 9 · 13.

## Tier 3 — `.claude/docs/` (19 present, each `preserve`)

Core 5: `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md`. Summaries 5:
`security-summary.md` · `design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md`. Services 8:
`viola.md` · `viola-core.md` · `viola-pty.md` · `viola-channel.md` · `viola-state.md` · `viola-agent-claude.md` ·
`viola-mcp.md` · `viola-ui.md`. `session-learnings.md` present (wrap territory). Absent: none.

## Agent harness (agent-driven)

`scripts/agent-run.sh` (executable) and `scripts/agent-run.ps1` present — `preserve`; no fresh render is made on the
upgrade form, nothing is compared. `.claude/rules/verification-harness.md` present — `preserve`.

## Code reviewer

`.claude/agents/code-reviewer.md` present — `preserve` (the pick for the language row is `rust`; not rendered).

## Hooks

`.claude/settings.json` — U02 `ok` (write current · bash current · PostToolUse on the stdin prologue): no write.
`rustfmt.toml` — U05 `ok`: no write.

## Gitignore · gitattributes

`.gitignore` — U07 `ok` (every base ignore decided by the root `.gitignore`, depth 2 included): no write.
`.gitattributes` — U06 `ok` (carries `* text=auto eol=lf`): no write.

## Code-graph pipeline

Planes by manifest: rust, ts. `scripts/code-graph.py` · `code-graph-views.sql` · `scip_pb2.py` · `requirements.txt` ·
`code-graph-cookbook.md` present; U03 `ok`: no write. `.andromeda/cache/` present (`.refresh-done`: `rust ok 25s
4347/22500` · `ts ok 0s 7/1`; `tree.db.commit` = `e304994ae0413c5cf5bf679a5b3d19abded0e472`).

## Seeded operational artifacts

`.andromeda/state.yaml` · `.claude/session-handoff.md` · `.andromeda/drift-base.md` · `.andromeda/playbook.md` ·
`.claude/docs/session-learnings.md` — all present, none written.

## Upgrade

HEAD at Setup 5b: `e304994ae0413c5cf5bf679a5b3d19abded0e472`. `route.py cursor`: `records 45 · complete 45 · pending 0
· gated 0` · `half-promote 0 of 45 stamped lines vs 45 master records`.

Path set at Setup 5b (`git status --porcelain=v1 -z --untracked-files=all`), all expected-transient bookkeeping or
untracked `.andromeda/runs/**`:

```
 M .andromeda/code-metrics.ndjson
 M .andromeda/friction-log.ndjson
 M .andromeda/runs/2026-10-08T09-10-03-wrap/evolve-2026-10-08-first-live-test-and-self-drive.json
 M .claude/session-handoff.md
?? .andromeda/runs/2026-10-08T09-45-26-evolve-diagnose/   (7 files: proposals.md · q-chains.json · q-health.json · q-level.json · q-retractions.json · q-typed.json · q-untyped.json)
?? .andromeda/runs/2026-10-08T10-08-51-code-audit/        (55 files, every one under that directory)
```

The listing, verbatim (`upgrade.py detect --root .`, read once at step 1b):

```
upgrade v1.7 · 970f0065
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
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
U14 · ok · hand · working-route markerless introducers · 30 markerless entries · 0 introducers behind markup or after t…
U35 · ok · hand · masters' logs + keyed contracts · ok: infra K, test K, test L, obs K, obs L, a11y K, a11y L, security…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
U48 · behind · setup · CLAUDE.md pointer rows · unnamed: architecture · test-plan · obs-plan · a11y-plan (4 of 4)
upgrade: for setup 1 (U48) · awaiting a door 0 · noted 1 (U36) · INDETERMINATE 0 · 17 detectors of 43 registry entries
```

Rows that act: **U48** (Phase 1 step 3, the four Edits above). No setup-class row reads `INDETERMINATE`. U11 and U12
read `ok` — Phase 7.5 step 1 has nothing to apply. U04 reads `ok` — step 8a does not run, no host re-seed. U36 is
`noted`: the card names it; `regenerate {leaf}` stays the operator's word.

## Consistency (step 9)

One write planned outside the run dir and the backup: `CLAUDE.md`, four row Edits, +0 lines (124 stays 124, limit
200). Every other artifact stands. No `## Adoption` (upgrade form).
