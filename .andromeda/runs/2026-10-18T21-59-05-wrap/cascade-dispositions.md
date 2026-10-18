# Cascade dispositions — 2026-10-18-compact-post-condition-ledger-row

Search: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-18T21-59-05-wrap/cascade-patterns.toml`, baseline
`7f35c8ec` (the pre-CI parent), run after every body amendment of the pass (architecture, security-plan,
design-system, layout-templates, test-plan, obs-plan, a11y-plan and the architecture, test-plan and obs-plan key
files). Nine patterns, each the OLD wording or the retired mechanism of an amendment: `old-rows` (the 23 / 20 relied
counts and literals), `old-pastes` (four added / five Run B pastes), `old-framing` (five framing variants, the
two-stem list), `old-set` (33 files), `old-transcr` (Run B's two transcripts), `old-owner` (the owner title
"Compact post-condition ledger row"), `old-compact` (`/compact` with no post-condition, unconfirmable on a verified
CLI, the retired test name, the ruled limit), `old-counts` (the eight / seven / three test counts, the old cli_verify
summaries, the 24th row, thirty-eight hooks), `old-window` (the single-window sentence, the 18.5 s total). Every
control fired. Sections read beside the rows: every amended section re-read after its apply for an intra-line
duplicate.

Counts (copied from the listing): `total (9 patterns) · 25 rows over 11 files` · `per class · new 0/0 · standing 6/3 ·
leaf 19/8 · curation 0/0 · base 0/0`. `old-set` and `old-transcr`: 0 rows, controls fired.

## Master and registry rows (6)
1. `.andromeda/architecture.md:83` · old-rows · no change — `stamped 2.1.287  23 pass  0 fail` is the `agents-join` row's own historical stamp (a true dated reading).
2. `.andromeda/architecture.md:410` @c4385 · old-rows · edited line · no change — the same historical `agents-join` stamp; the line's current counts were amended to 24 / 21.
3. `.andromeda/registries/contracts/test-plan/5-command-implementation.md:8` · old-rows · edited line · no change — the regex's `elsewhere twenty\b` matched the new `elsewhere twenty-one`.
4. `.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10` @c3573 · old-pastes · edited line · no change — "Since chunk 2026-10-13-paste-newline-ledger-row Run B makes four added turns", a true dated sentence; this pass appended the five-turn sentence after it.
5. `.andromeda/architecture.md:84` · old-compact · edited line · no change — "no probe types it" now names `/remote-control` alone (a true claim).
6. `.andromeda/architecture.md:46` @c206 · old-window · edited line · no change — the amended sentence ("The confirmation window is the compiled `CONFIRM_WINDOW_FALLBACK` = 10 s …; a `/compact` send … has its own, `COMPACT_CONFIRM_WINDOW` = 40 s").

## Leaf rows (19 over 8 files) — added to step 3's set
`CLAUDE.md:42` (old-rows ×2, old-owner) · `.claude/rules/verification-harness.md:28` (old-rows ×2, old-pastes) ·
`.claude/docs/commands.md:23` (old-rows ×4, old-pastes ×2), `:24` (old-rows ×2) · `.claude/docs/security-summary.md:36`
(old-rows ×2, old-counts) · `.claude/docs/services/viola-agent-claude.md:26` (old-rows ×2), `:33` (old-owner,
old-compact) · `.claude/docs/services/viola.md:6` (old-pastes) · `.claude/docs/tests-summary.md:21` (old-rows ×2,
old-pastes, old-framing), `:26` (old-compact ×2) · `.claude/docs/gotchas.md:84` (old-owner, old-compact ×2) — each
re-derived at step 3 (`## Leaves` below).

## Leaves
Mode: pass (Epoch 5 not closed). `cascade.py leaves` set: 14 groups, 28 leaves (`total · pass · 14 group(s) · 28 leaves
· 334459 B · unmapped 2`); unmapped `.claude/rules/host-linux.md` (the host leaf; no comparer) and
`.claude/rules/testing-src.md` (pointers into testing.md only; no change). One comparer per group in one batch inside one
snapshot pair (before `tree efc118a8…`, after `tree b3af118f…`, index and head unchanged, exactly the 14
`A …/leaf-{group}.md` lines). Findings 49: a11y-plan 1 · architecture-1 11 · architecture-2 2 · architecture-3 1 ·
architecture+design-system+layout-templates 0 · architecture+security-plan 0 · architecture+test-plan+obs-plan 6 ·
design-system+layout-templates 1 · masters 3 · obs-plan 0 · security-plan 9 · security-plan+obs-plan 0 · test-plan 15 ·
test-plan+obs-plan 0. Dry run: `total · blocks 49 · applied 49`; every source side read against the amended masters;
applied 49 by the tool over 12 leaves (`git diff --stat`: 12 files, 21 insertions, 21 deletions); by hand 0; declined 0.
The write call's stdout was sent to /dev/null by mistake; its record is the cascade trail.
