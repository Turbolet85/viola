# Cascade step 2 — sweep dispositions, wrap of 2026-10-10-session-links

**The search.** `cascade.py sweep` over `cascade-patterns.toml` (16 patterns), run twice: once after every master
body was applied (66 rows over 16 files), and again after the leaves were re-derived and one more master bullet
(test-plan §6 Scenario E3, As landed) was added. This file dispositions the SECOND listing, the tree as it is
committed. Its last lines, copied:

    total (16 patterns) · 70 rows over 18 files
    per class · new 10/3 · standing 42/8 · leaf 18/10 · curation 0/0 · base 0/0

The baseline was `f02d5486`, the parent of the pre-CI commit. Every pattern's control fired on the pre-pass text.
Each pattern is keyed on what a retired or extended claim SAID. Sections read whole beside the listing:
architecture [Links], [Budget Governor], the Channel methods rows, the Instance snapshot bullets, the Snapshot
envelope paragraph and the start chain of [Session Liveness] (by offset where the line is over 2 000 chars);
security-plan's row "Own state files on read" (by offset, 5 459 chars after the pass); obs-plan §4's three
scenarios and edge flows E3 and E5; test-plan §6 Path 6, E3 and E5; the two key files amended.

**What was NOT looked for:** bare `unlink` or `link` as a subject token (57 pre-pass master lines hold `unlink`;
the `served` pattern reads the ones that also say served, landed or built, and the seven detectors each read
their master's explicit-verb sentences and left them standing as the design); `budget-paused` or `closed gate`
as bare tokens; a count of the root cases of `tests/cli_links.rs` (stated once, in the test-plan key file, by
this pass); the unit-suite and CI test totals (no master states them).

## Rows, by pattern

- **replays-empty**, **kinds-land**, **owner-links**, **needs-link**, **reader-list**, **kind-count**: 0 rows
  in the second listing. Each control fired on the pre-pass text (architecture.md:247, test-plan.md:644 and :951,
  architecture.md:50), so the retired wording is gone from the masters, the key files and the leaves. In the
  first listing these patterns held 6 leaf rows (`rules/events.md:30` twice, `services/viola-state.md:28` and
  :45, `services/viola-core.md:20`, `tests-summary.md:30`), each re-derived at step 3.
- **no-case** · architecture.md:72 standing, edited → amended; the hit is this pass's own residual clause "no
  case drives `hook.dialog` under a closed gate", true by the read of the case's body (the dialog is raised
  before the gate closes).
- **in-the-diff** · architecture.md:72, a11y-plan.md:823 standing, edited → amended; "read in the diff" stays
  beside the case that now holds the clause. · `docs/a11y-summary.md:59` leaf → re-derived.
- **fourteen** · architecture.md:91, security-plan.md:617, test-plan.md:562 (the fourteen older stamp ids),
  test-plan.md:566 (fourteen labelled table cases) standing → no change, other subjects, read at the first
  listing. · `docs/security-summary.md:38` leaf → no change, the older stamp ids. The kill-line count is gone
  from the key file (now "fifteen", with its rule and the name list).
- **start-steps** · obs-plan.md:606 standing, edited → amended; "Two start steps … by design" stands for the two
  the operator answered, and the third (the link set's restore) follows as landed.
- **appends** · architecture.md:71 standing, edited → no change at the hit (the [Links] design sentence, true of
  the implicit link); the bullet gained its as-landed passage. · architecture.md:309 standing, edited → amended.
  · `rules/events.md:26` leaf → re-derived.
- **never-wake** · architecture.md:70 (the `wheel` event, log-only), :172 ×2 (`activity`), design-system.md:174
  (`activity`) standing → no change, other kinds, read by window. · architecture.md:278, :309 standing, edited →
  amended (`link` joins the lines that never wake; confirmed by the read of `LinkSet::change`, which appends
  through `append_event` and touches no wait feed). · `rules/events.md:17` leaf → re-derived.
- **derived** · architecture.md:71, :262 standing, edited → no change at the hit (true), :262 gained the as-landed
  facts. · `rules/events.md:26` leaf → re-derived.
- **one-caller** · architecture.md:247, security-plan.md:122, test-plan.md:952 standing → no change: each says the
  replay has one product reader, `viola revive`, which holds (the wrapper's restore is not a replay).
  · security-plan.md:241 standing, edited → amended (the rule's second caller named). · obs-plan.md:731 new →
  this pass's text. · `rules/events.md:30`, `services/viola-state.md:28` leaf → re-derived, both true.
- **unknown-kinds** · architecture.md:50, security-plan.md:241 standing, edited → amended (`link` / `unlink`
  are known kinds, returned and not counted). · architecture.md:220, design-system.md:195,
  layout-templates.md:401, test-plan.md:188, :202, :549, :1029, obs-plan.md:344, :719, :732,
  `registries/contracts/test-plan/5-command-implementation.md:75` standing → no change: each names the count or
  its envelope, none names `link` as an unknown kind (windows read at test-plan.md:549 and :1029 and
  obs-plan.md:344 and :732). · `rules/api.md:42`, `rules/events.md:34`, `services/viola-state.md:6` leaf → no
  change, true.
- **served** (the direction "no master says `link` or `unlink` is served") · architecture.md:53 standing → no
  change: [API Style] lists the methods whose requests carry an `id`, a design list. · architecture.md:94, :353,
  security-plan.md:84, test-plan.md:108, a11y-plan.md:83-84 standing → no change: the reserved v1.x brake
  route, "not served in v1". · architecture.md:261-262, :287-288 standing, wrap → no claim: the match spans two
  lines. · architecture.md:71 ×2, :262, :288, :309, test-plan.md:379, :644, :925, :952, obs-plan.md:729 ×2 new →
  this pass's text, each saying the explicit verbs and methods are NOT built, or naming the kinds. No row says
  either method or verb is served. · `CLAUDE.md:28`, `rules/api.md:21`, `rules/events.md:26` ×2,
  `docs/obs-summary.md:71` and :71-72, `docs/services/viola.md:25`, `docs/tests-summary.md:32` ×2 leaf →
  re-derived, each "not built" or "not served". · `docs/services/viola-ui.md:27` leaf → no change, the reserved
  route.

## Leaves re-derived at step 3

`CLAUDE.md` (Modules: `viola-state`, the root bin) · `.claude/rules/events.md` (the log-only list, the link set
bullet, the replay sentence) · `.claude/rules/api.md` (the methods bullet) · `.claude/rules/observability.md`
(the no-span list) · `.claude/docs/services/viola-state.md` · `services/viola-core.md` · `services/viola.md`
(one new bullet) · `docs/tests-summary.md` (Path 6, E1–E5) · `docs/a11y-summary.md` · `docs/obs-summary.md` (one
new bullet). Read and left: `rules/security.md` (no bullet restates the reader census), `rules/testing.md`,
`docs/security-summary.md`, `docs/conventions.md` and `docs/stack.md` (the verb and subcommand lists are
architecture §Conventions and §Stack, which this pass did not amend).

No hit in a curation home or a judgment base.
