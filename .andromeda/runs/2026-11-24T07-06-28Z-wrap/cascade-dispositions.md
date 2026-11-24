# Cascade dispositions — 2026-11-24-viola-up (wrap 2026-11-24T07-06-28Z)

## The sweep (step 2)
- The landings' own pattern files: none. Every `rows-land` dry run printed `patterns: none: {n} set(s) and retire(s)
  changed prose alone — no file` (or `the call sets and retires no row`), so no `cascade-patterns-{doc}-{k}.toml`
  exists; step 3's leaves read each changed row's delta.
- Main's own patterns, `cascade-patterns.toml` (the claims' own phrasings over every master, the key files, the
  curation homes, the bases and the leaves): `main-termpair` (the disproved terminal pair — «process-start /
  process-exit{subject:"terminal"} pair», «pair of the foot terminal window», «and for the terminal viola open
  spawns»), `main-setup` (the old `setup:` list), `main-nowindow` («revive never places a window», «no window step»,
  «replays no workspace»), `main-everybound` («every time bound»), `main-cwdonly` (the cwd «only in the snapshot»
  rows). The first firing refused `main-termpair` (control unfired on its first phrasing, the masters' text read
  «pair of the foot terminal window»); its phrasings were widened and the sweep re-fired.
- Listing `sweep-pass-1.1.txt`: `total (5 patterns) · 6 rows over 5 files` · `per class · new 0/0 · standing 2/1 ·
  leaf 4/4 · curation 0/0 · base 0/0`.
- Rows:
  - `.andromeda/security-plan.md:17` `dec-to5gfx4r` (decision/cwd-only-in-snapshot) — amended (tx 4593): the line
    now names the third standing place, up's terminal argv; «only» still governs a closed list. No change.
  - `.andromeda/security-plan.md:57` `dec-okfbhtnd` (decision/spawn-cwd-snapshot-only) — amended (tx 4593), as above.
    No change.
  - `.claude/rules/observability.md:42` (leaf) — stale: «`viola open` writes one `process-start` /
    `process-exit{subject:"terminal"}` pair»; to step 3 (the rule the directive names; corrected to the code).
  - `.claude/docs/obs-summary.md:70` (leaf) — stale, the same claim; to step 3.
  - `.claude/rules/frontend.md:25` (leaf) — stale: `setup:` holding `open, run, revive, verify, plugin install`; to
    step 3.
  - `.claude/docs/a11y-summary.md:53` (leaf) — stale: «Every time bound viola holds»; to step 3.
- `main-nowindow`: 0 rows, its control fired — a statement about this pattern.
- Curation homes and bases: 0 rows.

## Leaves (step 3)
- Set: `cascade.py leaves` pass mode (`epoch · «Epoch 5 — The fleet: lineage, launch and windows» · markerless 2 · gated
  2 · open`), `total · pass · 17 group(s) · 27 leaves · 491121 B · unmapped 2`; the four swept leaves in their groups.
- Batch `leaf`: 17 comparers, `VERDICT: 17 of 17 pass`.
- Decision: every finding's source row read against the landed rows (the read-backs above); all 42 applied by
  `cascade.py leaves --apply` (`total · blocks 42 · applied 42`), none by hand, none declined. Per group: a11y-plan 3 ·
  architecture-1 4 · architecture-2 1 · architecture-3 3 · architecture-4 0 · architecture+design-system+layout-templates
  5 · architecture+security-plan 4 · architecture+test-plan+obs-plan 2 · design-system+layout-templates 2 · masters 0
  (CLAUDE.md, gotchas.md: no finding) · obs-plan 5 · security-plan-1 1 · security-plan-2 3 · security-plan+obs-plan 0
  · test-plan-1 4 · test-plan-2 3 · test-plan+obs-plan 2.
- The rule the directive names, `.claude/rules/observability.md:42`, is CORRECTED to the code (finding
  `leaf-test-plan+obs-plan.md/.claude/rules/observability.md:42#1`): `viola open` and `viola up` write one
  `process-start{subject:"terminal"}`, with no `process-exit`, per terminal; the terminal outlives the verb.
- Unmapped: `.claude/rules/host-linux.md` (host — the host's shell recipes, no master source: no comparer's);
  `.claude/rules/testing-src.md` (path-scoped test rules derived from test-plan; no statement of it names `up`,
  revive's `--origin`, the terminal pair, the help row or the time bounds — read for them by name: no change).
