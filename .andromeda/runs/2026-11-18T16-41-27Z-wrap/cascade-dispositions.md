# Cascade dispositions — 2026-11-18-browser-tools-prompt-modal (wrap 2026-11-18T16-41-27Z)

## Step 2 — the sweep
The search: `cascade.py sweep` over `cascade-patterns-test-plan-1.toml` (the one rows-land dry run that wrote a file:
`tests1-5` = `stamped/pass/fail`, the old cell of test-plan `decision/ledger-row-verify-probe`) and main's own
`cascade-patterns.toml` — `everypass` (the retired claim «every relied row passes / reads pass / holds every relied
row»), `passfail` (the retired closed verdict «pass|fail», «`pass` or `fail`», «pass or fail»), `firstwords` (the old
first-screen list «trust, imports, box»). Every other rows-land dry run said `patterns: none: … changed prose alone —
no file`; those rows are read by their delta at step 3. Baseline 7e6b7fd4. Listing `sweep-pass-1.1.txt`, read whole.

Counts (copied from the tool): `total (4 patterns) · 9 rows over 6 files` · `per class · new 0/0 · standing 5/3 · leaf
4/3 · curation 0/0 · base 0/0`. `tests1-5` and `firstwords`: 0 rows, each control fired.

Rows:
- architecture.md:624 `contract/ledger-stamps/verified-version` (everypass, edited) — amended, this pass (tx 2807): «reads
  `pass`, or `absent` on `browser-tools-prompt` alone».
- test-plan.md:379 `decision/version-gate-needs-all-relied-rows` (everypass, edited) — amended, this pass (tx 2812):
  «holds every relied ledger row as counting».
- architecture.md:618 `contract/ledger-stamps/content` (passfail, edited) — amended, this pass (tx 2807), names `absent`.
- architecture.md:623 `contract/ledger-stamps/rows` (passfail, edited) — amended, this pass (tx 2807), names `absent`.
- design-system.md:184 `contract/viola-verify-output/trust-record-names-nothing` (passfail, edited) — amended, this
  pass (tx 2810): the trust-record row's own verdict, true (that row never reads `absent`).
- .claude/docs/commands.md:24 (everypass, passfail) — leaf, step 3.
- .claude/docs/security-summary.md:36 (everypass) — leaf, step 3.
- .claude/docs/design-summary.md:50 (passfail, @c3323) — leaf, step 3.
Not looked for: the masters' wording of the third modal literal (no master lists the literals, Phase 1's `modals3`
pre-pass); the curation homes and the bases hold none of these patterns (0 rows each class).

## Leaves
Set: `cascade.py leaves --add CLAUDE.md` — pass mode, epoch «Epoch 4 — Driver reliability: ledger rows and bounds»
open (markerless 6), `total · pass · 15 group(s) · 27 leaves · 434554 B · unmapped 2`. One comparer a group, batch
`leaf` 15 of 15 pass. Apply: `total · blocks 19 · applied 19` (one call, every finding written; no `--only` held).
- architecture-1 · pass · 4 findings · applied 4 (conventions.md, services/viola-agent-claude.md ×3)
- architecture-2 · pass · 1 · applied 1 (services/viola.md)
- architecture-3 · pass · 0
- architecture+security-plan · pass · 0
- masters · pass · 2 · applied 2 (CLAUDE.md lines 25 and 43, the cache-a key-name read in the Modules and Critical
  Warnings twins of the agent-crate bound)
- architecture+design-system+layout-templates · pass · 0
- architecture+test-plan+obs-plan · pass · 4 · applied 4 (commands.md); by hand: finding 24#1 kept «all
  twenty-three» — the count moved to 24, which no master states
- security-plan · pass · 4 · applied 4 (security-summary.md ×3, rules/security.md)
- security-plan+obs-plan · pass · 0
- design-system+layout-templates · pass · 1 · applied 1 (design-summary.md)
- test-plan-1 · pass · 1 · applied 1 (tests-summary.md)
- test-plan-2 · pass · 1 · applied 1 (rules/verification-harness.md)
- test-plan+obs-plan · pass · 1 · applied 1 (rules/observability.md)
- obs-plan · pass · 0 · a11y-plan · pass · 0
By hand (drafts landed by `rec.py harness land`, tx 2828–2830): the relied-row count a leaf states and no master does
(the pass mode reads master deltas, so it cannot see it; `command grep -rnoE 'twenty-three|\b23 (relied|rows|pass)|/23\]|23/23'`
over CLAUDE.md and .claude/): services/viola-agent-claude.md ×3 («twenty-three closed rows», «all twenty-three» ×2 →
twenty-four), commands.md ×3 (line 23 «all twenty-three» and «`MM` the relied count, 23; the last `[23/23]
trust-record …`» → 24 and `[24/24] browser-tools-prompt …`; line 24 «all twenty-three»), rules/verification-harness.md
×1. Left as found: the three historical clauses («exactly the twenty-three ids every stamp written before chunk
2026-10-18-…», «a twenty-three-row stamp written before chunk 2026-10-18-…») — true of the past.
Unmapped: `.claude/rules/host-linux.md` (the host leaf, no master source) and `.claude/rules/testing-src.md` — no
amended claim stands in either (`command grep` for the moved facts' tokens: none).
