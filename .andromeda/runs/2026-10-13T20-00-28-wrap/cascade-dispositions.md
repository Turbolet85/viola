# Cascade dispositions — 2026-10-13-interrupted-verify-cleanup

**The search.** `cascade.py sweep --patterns-file cascade-patterns.toml` (baseline `d9f01b1b`), nine patterns derived
from every amendment of this pass, each a retired claim's wording or mechanism:
- `exit-path`: «every exit path» and its restatements;
- `owed-move`: the owed `--settings` / `PROGRAM` move;
- `matrix-count`: the Windows matrix count;
- `detail-count`: the `process-exit.detail` count;
- `exit1-verify`: verify's exit-1 causes;
- `spawn-status`: «every verify spawn's `process-exit` carries `child_exit_status`»;
- `runb-ending`: Run B's Ctrl-C ending;
- `verify-summary`: «the summary closes the run»;
- `stop-new`: the new wording, read to check this pass's own text.

Every control fired. Listing: `total (9 patterns) · 40 rows over 15 files`; `per class · new 20/9 · standing 13/7 ·
leaf 6/5 · curation 1/1 · base 0/0`.

## Rows
- `architecture.md:100` exit-path standing → no change: «all gone when verify ends» is about verify's child
  processes, which a stopped run also kills and reaps (true).
- `architecture.md:449` exit-path standing → no change: a CI report file «removed on every clean run», not verify.
- `.claude/rules/security.md:19` exit-path leaf → re-derived (step 3, the security-plan group).
- `.claude/docs/security-summary.md:75` exit-path leaf, and the same leaf's runb-ending row → re-derived (step 3).
- `test-plan.md:1226` matrix-count standing → no change: «all nine jobs» is run 38036448183's reading, a past fact.
- `registries/contracts/architecture/ci-cd-approach.md:2` matrix-count standing, edited @c3023 → no change on that
  match: «all nine jobs» of run 38036448183 (a past reading). The same line's present-tense count was amended at @~1240.
- `.claude/rules/ci.md:31` matrix-count curation → P3: a dated correction note («nine `mutants (…)` checks since the
  root package was split by file») in a preserve-verbatim `## Session Additions`; the count is now ten. Curation's
  in-place extension, never a cascade edit.
- `.claude/rules/verification-harness.md:44`, `.claude/docs/commands.md:48`, `.claude/docs/tests-summary.md:42`
  matrix-count leaf → re-derived (step 3).
- `design-system.md:915` detail-count standing → no change: «11px», a font size, a pattern false positive.
- `architecture.md:406` spawn-status standing, edited → amended (the stopped child's pair and the interrupted self
  exit).
- `obs-plan.md:620` spawn-status standing → no change: `run`'s own `version-probe` pair, which observes no stop (true).
- `obs-plan.md:739`, `obs-plan.md:871` spawn-status standing, edited → amended.
- `architecture.md:91`, `security-plan.md:131` runb-ending standing, edited → amended (a stopped run is ended by a kill
  with no key).
- `layout-templates.md:602` verify-summary standing, edited → amended (the summary closes a completed run).
- `architecture.md:70` stop-new standing → no change: «The wait is not interrupted when the wheel moves», the send gate,
  a true unrelated claim.
- stop-new new rows: architecture `:91`, `:139`, `:371`, `:406`, `:413`; security-plan `:278`, `:283`; design-system
  `:811`, `:826`; layout-templates `:496`, `:500`, `:592`; test-plan `:656`, `:1163`; obs-plan `:739`, `:835`;
  a11y-plan `:90`, `:838`; registries `ci-cd-approach.md:2`, `crate-dependency-direction.md:11` → this pass's own
  text, re-read: each states the stop as the report carries it, with no intra-line duplicate of a retired claim.
- `owed-move` and `exit1-verify` read 0 rows with their controls fired: the owed clause and the «a failing row or a
  refusal» wording no longer stand in any master or registry file.

## Leaves
`cascade.py leaves` (pass mode: this chunk does not close Epoch 5) gave 13 groups and 28 leaves. Two leaves are
unmapped: `.claude/rules/host-linux.md` (the host's own leaf, which no master states) and `.claude/rules/testing-src.md`
(a pointer list). Neither gets a comparer, and neither changes. One comparer per group ran in a single batch inside a
snapshot pair: 13 `A` lines, index and head unchanged.

`leaves --apply`: `total · blocks 19 · applied 17 · held 2`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 1 | 0 | 0 | 1 — `services/viola-agent-claude.md:14`, the consumer list gains no fact a reader acts on; the detail stays in architecture `:371` |
| architecture-2 | pass | 5 | 5 | 0 | 0 |
| architecture+security-plan | pass | 1 | 1 | 0 | 0 |
| architecture+security-plan+design-system+layout-templates | pass | 1 | 1 | 0 | 0 |
| masters (CLAUDE.md) | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan+obs-plan | pass | 2 | 2 | 0 | 0 |
| security-plan | pass | 4 | 4 | 0 | 0 |
| security-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| design-system+layout-templates | pass | 1 | 0 | 0 | 1 — `design-summary.md:55`, a component-name list; the interrupted form stays in design-system `:811` |
| test-plan | pass | 2 | 2 | 0 | 0 |
| test-plan+obs-plan | pass | 1 | 1 | 0 | 0 |
| obs-plan | pass | 1 | 1 | 0 | 0 |
| a11y-plan | pass | 0 | 0 | 0 | 0 |

Each applied finding's source side was read at its cited line: architecture `:91`, `:139`, `:406`; the
crate-direction key `:11`; the CI/CD key `:2`; the directory tree `:33`; design-system `:826`; obs-plan `:739`,
`:835`; security-plan `:131`, `:283`; test-plan `:1163`. The step-2 leaf rows are all covered: `security.md:19`,
`security-summary.md:75`, `verification-harness.md:44`, `commands.md:48` and `tests-summary.md:42` were re-derived.
