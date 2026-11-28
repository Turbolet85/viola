# Cascade dispositions — wrap 2026-11-28T08-40-41Z-wrap (chunk 2026-11-28-console-scaffold)

## Step 2 — the sweep
Skipped: no `rows-land` dry run of this pass wrote a pattern file. Each printed `patterns: none: {n} set(s) and
retire(s) changed prose alone — no file` (architecture 5, design-system 4, layout-templates 3, test-plan 14,
obs-plan 11 and main's 2, a11y-plan 6) or `patterns: none — the call sets and retires no row` (security-plan; main's
test-plan, design-system and layout-templates). No main pattern file was written. Step 3's leaves read each changed
row's delta (`leaf-delta-{doc}.diff`, seven masters). The binds (test-plan §3 ↔ obs-plan §3 / §6): the process field
reads the five roles in both (test-plan `contract/log-line/process`, obs-plan `contract/log-line/process`,
`contract/diag-line/process`, `contract/process-log-line/process`); the two new event field items stand in obs-plan
§6 alone (test-plan's two adds rejected, check 2).

## Leaves
`cascade.py leaves` pass mode: 17 groups · 27 leaves · unmapped 2 (`.claude/rules/host-linux.md` — the host's leaf, no
master source; `.claude/rules/testing-src.md` — no master names it, left). Comparers: 17 of 17 pass. `leaves --apply`:
`total · blocks 36 · applied 35 · held 1`.
- Applied 35 (14 leaves): a11y.md 1 · conventions.md 2 · stack.md 1 · frontend.md 4 · api.md 3 · commands.md 1 ·
  design-summary.md 3 · CLAUDE.md 4 · obs-summary.md 6 · security-summary.md 2 · security.md 2 · tests-summary.md 1 ·
  testing.md 2 · observability.md 3.
- Declined 1: `leaf-architecture-3.md/.claude/docs/services/viola.md:34#1` (stale) — it would say `role_of` files
  `console` under its own role; `role_of` takes the `Cli` fallback for `console` (plan step 4, the report's Symbols /
  APIs), only its obs process is `console`, so the leaf line stands true as written.
- Groups with no finding: architecture-2 · architecture-4 · security-plan+obs-plan · test-plan-2.
