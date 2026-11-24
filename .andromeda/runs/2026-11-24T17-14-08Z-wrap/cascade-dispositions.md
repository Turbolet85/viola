# Cascade dispositions — wrap 2026-11-24T17-14-08Z-wrap · chunk 2026-11-24-viola-focus

## Sweep (step 2)
- No pattern file: each `rows-land` dry run printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no
  file` (architecture 1, security-plan 2, obs-plan 1). The sweep is skipped with that line; step 3's leaves read each
  changed row's delta.
- Verbatim cross-master check, by hand: `rec.py find --regex '(?i)not on Hyprland|no compositor answering,
  provisional|unreachable[^.]{0,40}provisional'` over all seven masters → 0 hits each. The amended claim
  (obs-plan `contract/process-exit-detail/exit-1-focus`) is cited nowhere in its old wording. The state sets change no
  wording.
- Binds (test-plan §3 ↔ obs-plan §3/§6, a11y schema): untouched — no harness command, log field or schema moved.

## Leaves (step 3)
`cascade.py leaves` (pass mode): 11 groups · 21 leaves · unmapped 2 (`.claude/rules/host-linux.md`, the host leaf;
`.claude/rules/testing-src.md`, no source changed for it) — no comparer for either, and nothing to compare.
- 9 groups: 0 findings.
- architecture+obs-plan: 1 finding, applied (`.claude/docs/commands.md:23`, the focus details to his F-E words).
- obs-plan: 2 findings — `.claude/rules/observability.md:28` applied; `.claude/docs/obs-summary.md:67` held and
  written BY HAND (the comparer's replace-with dropped the parenthesis of `unreachable`): the leaf drafted whole and
  landed by `rec.py harness land` (tx 4856), reading «`unsupported` (no `hyprctl` reachable) spawns nothing,
  `unreachable` reads no compositor answering)».
- Tool total: `blocks 3 · applied 2 · held 1`; declined 0.
