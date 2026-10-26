# Cascade dispositions — 2026-10-26-explicit-session-links

## The search (Cascade step 2)
One call, `cascade.py sweep` (cascade v1.18), baseline 8fc3d1df (the parent of the oldest pre-CI commit 743057f9), over
the pass's patterns: the apply writers' `## Patterns` (`apply-a11y-plan.md` a11y-01…08, `apply-architecture.p1.md`
arch1-01…10, `apply-architecture.p2.md` arch2-01…08, `apply-design-system.md` ds-01…10, `apply-layout-templates.md`
lt-01…06, `apply-obs-plan.p1.md` obs1-01…10, `apply-obs-plan.p2.md` obs2-01…02, `apply-security-plan.md` sec-01…08,
`apply-test-plan.p1.md` tp1-01…08, and `.raw-apply-test-plan.p2.md` tp2-01…10 — the patterns of the edits that writer's
first file wrote; its re-spawn's file holds `(none)`, its one block a clause drop) and main's own
`cascade-patterns.toml` (m-notbuilt · m-six · m-twosites · m-nocaller · m-owner: the retired claim's own phrasings —
"not built / not served / answers `-32601` / held", "six frame verbs", "two product call sites", "no caller", the
pending owner's title), every pattern's control fired.
Listing: `total (85 patterns) · 34 rows over 15 files` · `per class · new 0/0 · standing 6/5 · leaf 28/10 · curation 0/0 ·
base 0/0`. Sections read: each standing row's window (`cascade.py window`, layout-templates.md:329 at @c600, @c1380,
@c1700, @c2050; design-system-detail/surface.md:21 at @c560) and the three security lines at their matches.

## Rows
Standing (6 rows, 5 files):
- design-system-detail/surface.md:21 ds-01 (edited, markup) — amended: the line apply edited; `viola list`'s exit-21
  clause stands true beside the added link / unlink clause.
- design-system-detail/surface.md:21 ds-04 (edited) — amended: the same line, its fixed-form list now naming link / unlink.
- layout-templates.md:329 ds-01 (edited, markup, @c1380) — no change, true: `viola list`'s exit-21 entry of the fixed-pair
  list, whose link / unlink entry the apply wrote at @c2050 (read).
- security-plan.md:114 sec-01 (edited) — amended: "the snapshot read without the strict-modes check is a dated gap … except
  for CLI `link` / `unlink`" (read @c390).
- security-plan-detail/threat-model-summary.md:56 sec-03 (edited) — amended: the same exception (read @c446).
- security-plan-detail/bootstrap-phases.md:15 sec-08 (edited) — amended: the same exception (read @c456).
Leaf (28 rows, 10 files) — each re-derived at step 3 (`cascade.py leaves --add`), never edited from this listing:
- CLAUDE.md:27 (m-notbuilt) — "`link` / `unlink` are not served" — stale.
- .claude/rules/api.md:20 (arch1-04, tp1-01, tp1-02, m-notbuilt) — "not built: the wrapper answers both `-32601`" — stale.
- .claude/rules/api.md:29, :30 (ds-09) — the six frame verbs' server check — stale (eight).
- .claude/rules/events.md:26 (arch1-04, arch1-08, tp1-01, tp1-02, m-notbuilt) — "only the implicit link is built … not
  built" — stale.
- .claude/rules/frontend.md:25 (a11y-03, ds-01, ds-10, m-owner) — the help rows without `handoff:`, the pending owner, the
  fixed-form list — stale.
- .claude/rules/observability.md:40 (obs2-01, tp1-05, m-six) — "the six CLI frame verbs" — stale.
- .claude/docs/a11y-summary.md:29 (a11y-02, lt-02, m-owner) — the `handoff:` row owed — stale.
- .claude/docs/obs-summary.md:70 (m-notbuilt) — E3's explicit methods not built — stale.
- .claude/docs/tests-summary.md:32 (tp1-01, m-notbuilt, m-owner) — the explicit steps not built — stale.
- .claude/docs/services/viola-channel.md:26 (ds-09) — the six verbs' server check — stale.
- .claude/docs/services/viola.md:25 (m-nocaller) — "`remove` has no product caller" — stale.
Curation homes and judgment bases: 0 rows. Every row dispositioned.

## Leaves
`cascade.py leaves` (pass mode; `--add` the 10 leaf files of step 2), `total · pass · 14 group(s) · 27 leaves · 366319 B · unmapped 2`; one comparer per group (batch `leaf`, 14 of 14 pass); `leaves --apply --dry-run` then the write: `total · blocks 40 · applied 40`.
- a11y-plan · pass · findings 2 · applied 2 · by hand 0 · declined 0
- architecture-1 · pass · findings 5 · applied 5 · by hand 0 · declined 0
- architecture-2 · pass · findings 5 · applied 5 · by hand 0 · declined 0
- architecture-3 · pass · findings 3 · applied 3 · by hand 0 · declined 0
- architecture+design-system+layout-templates · pass · findings 3 · applied 3 · by hand 0 · declined 0
- architecture+security-plan · pass · findings 9 · applied 9 · by hand 0 · declined 0
- architecture+test-plan+obs-plan · pass · findings 0 · applied 0 · by hand 0 · declined 0
- design-system+layout-templates · pass · findings 1 · applied 1 · by hand 0 · declined 0
- masters · pass · findings 2 · applied 2 · by hand 0 · declined 0
- obs-plan · pass · findings 2 · applied 2 · by hand 0 · declined 0
- security-plan · pass · findings 1 · applied 1 · by hand 0 · declined 0
- security-plan+obs-plan · pass · findings 0 · applied 0 · by hand 0 · declined 0
- test-plan · pass · findings 2 · applied 2 · by hand 0 · declined 0
- test-plan+obs-plan · pass · findings 5 · applied 5 · by hand 0 · declined 0
- unmapped `.claude/rules/host-linux.md` (host) — the host's own leaf, no master source: no comparer, no edit.
- unmapped `.claude/rules/testing-src.md` — no master provenance header; the step-2 sweep read 0 rows in it: no edit.
Every finding's source line was checked at its cited line (each a line this pass wrote); each omission names a fact a reader of that leaf would otherwise act against (the verbs' exit codes, refusal order and spans, the served methods' answers).
