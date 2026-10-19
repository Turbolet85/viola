# Cascade dispositions — 2026-10-18-plugin-precedence-live-readings

The search: `cascade-patterns.toml` (6 patterns — PREC, EXIT1, OWEDTITLE, VDIR, RIGDIR, RESID: the old wording of every
amended passage and the mechanism each asserted — "which `viola` MCP server … starts and uses" / "both plugins" / "same
name", "exits 1 blocks the CLI's start", the owner title "Plugin precedence live readings", "NOT ratified" / "word
owed", "a rig dir under `target/` that is its own git repo", the residual's own phrasing), run by `cascade.py sweep`
after the last body edit (trail `cascade-2026-10-18-plugin-precedence-live-readings.json`). Listing: `total (6 patterns)
· 22 rows over 15 files` · `per class · new 1/1 · standing 10/6 · leaf 8/6 · curation 2/2 · base 1/1`. Sections read:
architecture §Established Decisions ([Plugin Scope], [Deployment / Distribution]), §Occupied Resources (`VIOLA_DIR`),
§Cross-cutting Patterns (Config management); security-plan §Input Validation (Home resolution), §Data Protection;
test-plan §5.

| row | disposition |
|---|---|
| `architecture.md:87` PREC standing @c20164 | no change — "plugin precedence is no row ([Plugin Scope])" stays true |
| `test-plan.md:189` PREC standing | no change — "a second `run` of the same name … exits 1", another claim sharing the token |
| `obs-plan.md:922` PREC standing | no change — a CI artifact upload of "the same name", unrelated |
| `.claude/rules/host-linux.md:50` PREC curation | no change — a process of "the same name", unrelated; curation home untouched |
| `architecture.md:95` EXIT1 new @c3129 | amended — this pass's own text ("does not block the CLI's start") |
| OWEDTITLE | 0 rows (control fired) — the owner title "Plugin precedence live readings" no longer stands in any master, leaf, curation home or base |
| `architecture.md:68` VDIR standing @c2157 | no change — the link verbs' hold ("the boundary widening they carry is not ratified"), not the home step |
| `design-system.md:420` VDIR standing | no change — the width source's PROVISIONAL, not the home step |
| `design-system.md:529` VDIR standing | no change — `--help`'s served verbs PROVISIONAL, not the home step |
| `layout-templates.md:61` VDIR standing | no change — the same `--help` PROVISIONAL |
| `test-plan.md:819` VDIR standing | no change — the link verbs' hold |
| `.claude/docs/session-learnings.md:66` VDIR curation | no change — a learning quoting the phrase class; curation home untouched |
| `.andromeda/playbook.md:66` VDIR base ×3 | no change — the "Held items" rule's own text, still the rule; no propose-channel item |
| `.claude/rules/api.md:31` VDIR leaf ×3 | leaf — re-derived in step 3 |
| `.claude/rules/security.md:23` VDIR leaf ×3 | leaf — re-derived in step 3 |
| `.claude/docs/conventions.md:44` VDIR leaf ×2 | leaf — re-derived in step 3 |
| `.claude/docs/security-summary.md:64` VDIR leaf ×3 | leaf — re-derived in step 3 |
| `.claude/docs/services/viola-state.md:6` VDIR leaf ×2 | leaf — re-derived in step 3 |
| `.claude/docs/services/viola.md:20` VDIR leaf ×2 | leaf — re-derived in step 3 |
| `security-plan.md:262` RIGDIR standing edited | amended — the line now reads the rig outside the work tree; "its own git repo" stands as current truth (no intra-line duplicate of the retired "under `target/`") |
| `.claude/docs/security-summary.md:73` RIGDIR leaf ×2 | leaf — re-derived in step 3 |
| `security-plan.md:262` RESID standing edited mixed ×3 | amended — the heading phrase and "orphaned plugin cache" / "trust entry" are this pass's current truth |
| `.claude/docs/security-summary.md:73` RESID leaf ×2 | leaf — re-derived in step 3 |

Rows dispositioned: 22 of 22 (amended 3 · no change 11 · leaf 8 rows over 6 leaves, `security-summary.md:73` holding one RIGDIR and one RESID row; the OWEDTITLE line is its 0-row statement, no row).

## Leaves

`cascade.py leaves` (pass mode, baseline `bc2e3bf7`): `total · pass · 8 group(s) · 24 leaves · 310720 B · unmapped 2`;
one comparer per group, one batch inside a snapshot pair (before `tree 3add8160…`, after `tree 968cde79…`, index and
head unchanged; exactly the eight `A … leaf-{group}.md` lines). Apply: `total · blocks 13 · applied 13`, every leaf read
back. Unmapped: `.claude/rules/host-linux.md` (the host leaf — no master statement, no comparer) and
`.claude/rules/testing-src.md` (a pointer list into testing.md — no master statement, no comparer).

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 1 | 1 (`conventions.md:44` retired) | 0 | 0 |
| architecture-2 | pass | 4 | 4 (`viola-state.md:6`, `viola.md:20` retired; `viola-mcp.md:14`, `:23` omission — the two readings a reader of that crate's notes acts on) | 0 | 0 |
| architecture-3 | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan | pass | 2 | 2 (`api.md:31` stale ×2) | 0 | 0 |
| architecture+security-plan+test-plan | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan | pass | 1 | 1 (`commands.md:22` omission — the exit-1 reading beside the `mcp` refusal it describes) | 0 | 0 |
| security-plan | pass | 5 | 5 (`security.md:23` stale ×2, `security-summary.md:64` stale ×2, `:73` stale) | 0 | 0 |
| test-plan | pass | 0 | 0 | 0 | 0 |

Closure: the VDIR and RIGDIR patterns re-swept after the apply — `VDIR · … leaf 0`; `RIGDIR · leaf 1` is the leaf's new
current wording (`security-summary.md:73`, "its own git repo outside this work tree"); `total (2 patterns) · 9 rows
over 8 files`.
