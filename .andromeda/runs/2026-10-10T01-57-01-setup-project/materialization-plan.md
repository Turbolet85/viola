# Materialization plan — viola · re-run, upgrade form · 2026-10-10T01:57:01Z

Setup: project viola · Development Style agent-driven (`architecture.md:458`, `:507`) · stack fragment `rust`
(`architecture.md:13`, §Stack's language row) · re-run — upgrade form: no upstream body read. The invocation's words
("Re-run on the live project for the one pending upgrade (U03). Present the card whole and wait for the answer.")
name no structural change in the masters.

Upstreams present (existence only): `input.md` · `architecture.md` · `security-plan.md` · `design-system.md` ·
`layout-templates.md` · `test-plan.md` · `obs-plan.md` · `a11y-plan.md` · `master-route.md`.

## Tier 1 — CLAUDE.md (126 lines; backup `.claude/backup/CLAUDE.md.pre-setup-2026-10-10T01-57-01`, md5 `37ce280dc1431ab778b827f6ed40a474`)

| block | disposition |
|---|---|
| `overview` | not re-derived — cascade-maintained, stands byte for byte |
| `modules` | not re-derived — cascade-maintained, stands byte for byte |
| `warnings` | not re-derived — cascade-maintained, 10 bullets, stands byte for byte |
| `pointer-table` | not re-derived — cascade-maintained, stands byte for byte (U48 `ok`: no row edit) |
| `workflow` | not re-derived — cascade-maintained, stands byte for byte |
| `architecture` | not re-derived — cascade-maintained, stands byte for byte |
| `imports` | the template's (U01 `ok`) — one line, `@.claude/session-handoff.md`; no Edit |
| `deeper-topics` | recomputed from the files that exist — equal to the block as it stands; no Edit |
| `USER:session-learnings` | never touched (10 bullets) |

`deeper-topics`, recomputed: docs — the 5 summaries (`security` · `design` · `tests` · `obs` · `a11y`), the 5 core
(`stack` · `conventions` · `commands` · `gotchas` · `workflow`), `services/` × 8 (`viola` · `viola-core` ·
`viola-pty` · `viola-channel` · `viola-state` · `viola-agent-claude` · `viola-mcp` · `viola-ui`),
`session-learnings.md`. Rules — `.claude/rules/` holds 11 files and the block names the same 11 (`security.md` ·
`host-linux.md` · `testing.md` · `verification-harness.md` · `observability.md` · `api.md` · `events.md` ·
`frontend.md` · `a11y.md` · `ci.md` · `testing-src.md`). Both lists equal the block: **no Edit, so CLAUDE.md is not
written by this run** (Phase 1 step 3). The backup above was taken at Setup 5 before that was known; it is ignored by
git and never staged.

## Tier 2 — `.claude/rules/` (11 present, each `preserve`)

`a11y.md` · `api.md` · `ci.md` · `events.md` · `frontend.md` · `host-linux.md` (the host leaf, U04 `ok`: every
template line present above `## Session Additions`; `upgrade.py host` prints `host: linux`) · `observability.md` ·
`security.md` · `testing.md` · `testing-src.md` · `verification-harness.md`. Nothing planned beyond them. `ci.md` and
`testing-src.md` are a chunk's files, not setup's render: `preserve`, as every present leaf. Absent: none named by
health checks 5 · 9 · 13.

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

`.claude/settings.json` — U02 `ok` (`write current · bash current · PostToolUse on the stdin prologue`): no write.
The file parses as JSON. `rustfmt.toml` — U05 `ok`: no write.

## Gitignore · gitattributes

`.gitignore` — U07 `ok` (every base ignore decided by the root `.gitignore`, depth 2 included): no write.
`.gitattributes` — U06 `ok` (carries `* text=auto eol=lf`): no write.

## Code-graph pipeline

Planes by manifest: rust, ts. The five files are present, so nothing is seeded: `scripts/code-graph.py` ·
`code-graph-views.sql` · `scip_pb2.py` · `requirements.txt` · `code-graph-cookbook.md`. `.andromeda/cache/` present
(`.refresh-done`: `rust ok 27s 4367/22796` · `ts ok 0s 7/1`; `tree.db.commit` =
`d778ececb7ddfe2d09a84b7fa188759b3ab42984`, equal to HEAD).

**U03 `behind` — Phase 6's drift rule acts.** Each of the triple read against its installed template with the
health tool's own reader (`health.template_region`, `health.behind`):

| file | lines behind | facts |
|---|---|---|
| `scripts/code-graph.py` | 27 | live 387 lines, 16 903 B, md5 `9f982a7fed402835edfaa32d85a5ca3d`; the template's fence under `## Template` 407 lines, 17 973 B, md5 `a2aeacc7727d7f54e784edf65a450f6c`; both LF, no CR |
| `scripts/code-graph-views.sql` | 0 | byte-equal to the template's fence (4 000 B, md5 `26bb460cbec6531e26e633e9d73bc395`) |
| `scripts/code-graph-cookbook.md` | 0 | equal above its `Project-specific query learnings` marker |

The live `code-graph.py` is the copy the first setup commit wrote (`13b7ee3`, its only commit): no project edit
stands in it. The template file was last written 2026-10-10T00:36Z, after the previous setup run (2026-10-09T21:16Z)
read this row `ok`.

- Backup, taken at this step: `.claude/backup/code-graph.py.pre-setup-2026-10-10T01-57-01`
  (md5 `9f982a7fed402835edfaa32d85a5ca3d`, the live file's).
- **Proposed update, for the card; written only on the operator's "yes":** `scripts/code-graph.py` becomes the
  template's fence body, byte for byte (target md5 `a2aeacc7727d7f54e784edf65a450f6c`). The difference is three
  hunks, +27 / −7 lines, kept whole in this run dir as `code-graph.py.diff`:
  1. `PLANES` — the rust plane gains `"config": {"cargo": {"features": "all"}}`, and both planes' `argv` lambdas take
     a fifth argument `cfg`; rust appends `--config-path <cfg>` when one is given, ts ignores it.
  2. `_build_plane` — when a plane carries `config`, it is written as a transient `indexer.json` beside the SCIP dump
     in `.andromeda/cache/{plane}/` and removed when the indexer returns. The indexer runs with it first; if that
     run fails or leaves no SCIP file, it runs once more without it and the plane's result carries the note
     ` (default features - the all-features index failed)`.
  3. The `tree-refresh[...]` print and the `.refresh-done` line carry that note when it is set.
- The write, on "yes": three anchored Edits on the live file, one per hunk, then `md5sum` read against the target.
  A mismatch is a HALT before Phase 7.5's re-detect.
- No views change, so no `built.views` rebuild. The setup commit moves HEAD, so on each plane the first query after
  it regenerates that plane (`built.head` and `tree.db.commit` both hold `d778ece`; a re-run stamps nothing) — the
  rust plane then indexes with every Cargo feature on.
- What "every feature" is in this workspace, by the manifests: `fake-agent` (the root package and five member
  crates) and `test-support` (`viola-channel`). Not measured here: whether the all-features index succeeds on this
  workspace, its duration, and the node and edge counts it gives. The script's own fallback covers a failure.
- Nothing in `.github/`, `.claude/settings.json` or another script names `code-graph.py`; `commands.md` and
  CLAUDE.md name its two subcommands, which do not change.

## Seeded operational artifacts

`.andromeda/state.yaml` · `.claude/session-handoff.md` · `.andromeda/drift-base.md` · `.andromeda/playbook.md` ·
`.claude/docs/session-learnings.md` — all present, none written.

## Upgrade

HEAD at Setup 5b: `d778ececb7ddfe2d09a84b7fa188759b3ab42984`. `route.py cursor`: `records 48 · complete 48 · pending 0
· gated 0` · `half-promote 0 of 48 stamped lines vs 48 master records`.

Path set at Setup 5b (`git status --porcelain=v1 -z --untracked-files=all`), all expected-transient bookkeeping or an
untracked path under `.andromeda/runs/`:

```
 M .andromeda/friction-log.ndjson
 M .andromeda/runs/2026-10-10T01-25-38-wrap/evolve-2026-10-09-epoch-3-cleanup-ii.json
 M .claude/session-handoff.md
?? .andromeda/runs/2026-10-10T01-25-38-wrap/health-2026-10-09-epoch-3-cleanup-ii.json
```

The listing, verbatim (`upgrade.py detect --root .`, read once at step 1b):

```
upgrade v1.8 · 30b07c54
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · behind · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph…
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
U14 · ok · hand · working-route markerless introducers · 32 markerless entries · 0 introducers behind markup or after t…
U35 · ok · hand · masters' logs + keyed contracts · ok: infra K, test K, test L, obs K, obs L, a11y K, a11y L, security…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
U48 · ok · setup · CLAUDE.md pointer rows · 4 of 4 indexes named
upgrade: for setup 1 (U03) · awaiting a door 0 · noted 1 (U36) · INDETERMINATE 0 · 17 detectors of 45 registry entries
```

Rows that act: **U03** (Phase 6 step 1's drift rule: backed up, proposed at the card, written on "yes"). No
setup-class row reads `INDETERMINATE`. U11 and U12 read `ok` — Phase 7.5 step 1 has nothing to apply. U04 reads `ok`
— step 8a does not run, no host re-seed. U36 is `noted`: the card names it; `regenerate {leaf}` stays the
operator's word.

## Consistency (step 9)

One write is planned outside the run dir and the backups, and only on the operator's "yes": `scripts/code-graph.py`
(three hunks, 387 → 407 lines). CLAUDE.md is not written (126 lines stand, limit 200). Every other artifact stands.
Declined, the script stands as it is, U03 stays `behind` on the card and the dashboard, and the run commits nothing
(the manifest then holds only ignored backups). No `## Adoption` (upgrade form).
