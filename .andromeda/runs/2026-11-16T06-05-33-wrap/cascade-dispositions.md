# Cascade dispositions — 0-pending wrap 2026-11-16T06-05-33-wrap

## Sweep
patterns: none — every `rows-land` dry run of this wrap (6 masters, 12 operations) printed `patterns: none: {n} set(s)
and retire(s) changed prose alone — no file`: each set dropped the word «provisional(ly)» or set `state` to
`standing`, a prose change with no distinctive term. No file holds a pattern, so the sweep is skipped with that line,
and step 3's leaves read each changed row's delta (`leaf-delta-{doc}.diff`).

## Leaves
Mode: pass (no chunk marker; baseline 3ac557bd). `cascade.py leaves`: 14 groups · 25 leaves · unmapped 2.
- architecture-1: 3 findings (gotchas.md:99, services/viola-agent-claude.md:25 ×2), applied 3.
- architecture-2, architecture-3, architecture-4, architecture+security-plan, architecture+design-system+layout-templates,
  architecture+test-plan+obs-plan, security-plan+obs-plan, test-plan+obs-plan, obs-plan: 0 findings.
- architecture+security-plan+design-system+layout-templates+test-plan+obs-plan (CLAUDE.md): 2 findings; :85 applied;
  :43 `anchor absent` (the leaf reads «registered provisionally with», no comma) — written by hand: drafted whole to
  `draft-CLAUDE.md`, landed by `rec.py harness land` TX 1869, the same replacement less the comma.
- design-system+layout-templates: 1 finding (design-summary.md:7), applied 1.
- security-plan: 4 findings (security-summary.md:61, :73; rules/security.md:19, :24), applied 4.
- test-plan: 1 finding (rules/testing.md:24), applied 1.
- unmapped `.claude/rules/host-linux.md` (host) — no comparer's: a host leaf states no master's claim.
- unmapped `.claude/rules/testing-src.md` — declined: it holds no «provisional» and no StopFailure, send-tolerance or
  stamp-dir statement (grep, 0 hits), so none of this wrap's changed rows reaches it.
Kept, by design: the «provisional» marks on the one-word `api-error` cause (rules/security.md:25,
docs/security-summary.md:27) — the founder's ruling 2 supersedes that word by «Granular API-error cause» and the rows
keep it until that chunk lands (adaptation:4).
total · blocks 11 · applied 10 · by hand 1 · declined 0
