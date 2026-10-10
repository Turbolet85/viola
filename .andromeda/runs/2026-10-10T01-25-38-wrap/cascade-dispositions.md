# Cascade dispositions — 2026-10-09-epoch-3-cleanup-ii

The sweep ran after the last body edit of the pass (`cascade.py sweep`, the pattern set in
`cascade-patterns.toml`, the trail `cascade-2026-10-09-epoch-3-cleanup-ii.json`). Baseline `14f1fb5f`, the parent
of the pre-CI commit. Its last lines, copied:

`total (12 patterns) · 66 rows over 18 files`
`per class · new 1/1 · standing 45/8 · leaf 18/10 · curation 2/1 · base 0/0`

## What was searched
Twelve patterns, each with its control fired on the pre-pass masters and key files:
- the retired count and ordinal: `9 root waits` · `ninth`;
- the claim's own words: `waits on a child` · `WITHIN` · `wait_endpoint_gone`;
- the helper directory, for any second list of the helper files or any statement of per-file copies:
  `tests/support` · `per-file`;
- the wall reading and its population: `78 m` · `711`;
- the whole-member score's form and its owed-mutant wording: `--package viola-e2e` · `whole member` /
  `whole-member` / `whole unit` · `not measured here`.

Also read by hand, outside the tool: `unscored`, `no score`, `not yet scored`, `2.5 h`, `165 of 718` over the seven
masters, the registries and the leaves, 0 hits; the same expression fires on `viola-0.1.0/working-route.md` (the
chunk's own frozen line), so the zero is the pattern's and it is alive.

Not searched by pattern: `events.rs` and `cli.rs` alone (most uses name `crates/viola-state/src/events.rs` or the
CLI; the helper files are reached through `tests/support`).

Every `tests/support` hit was read in a bounded window by offset (seven of them stand on lines over 2 000
characters: `architecture.md:47`, `:424`; `test-plan.md:623`, `:627`, `:1079`; the key file
`5-command-implementation.md:33`; `test-data-bootstrap.md:17`).

## Rows, by class
**new (1 row, 1 file)** — the key file `bootstrap-phases-derive-for-route-setup-project.md:16`, this pass's own
sentence. No disposition owed.

**standing (45 rows, 8 files)**
- `nine-waits`: 0 rows. Both sites that held the count were amended (`architecture.md:407`, the key file
  `5-command-implementation.md:33`).
- `architecture.md:407` and `5-command-implementation.md:33` (`waits-child`, `within`, `endpoint-gone`,
  `support-dir`): the two amended lines, where the swept words stand in the new wording. Each re-read whole for a
  second copy of the retired count or ordinal on the line: none ("the ninth wait" is gone from the key file).
- `bootstrap-phases-…:10` (`within`): "`WITHIN` = 7 s stays the bound of every other wait". It states the bound, no
  count. No change.
- `design-system.md:909` (`ninth`): "a ninth hex value". A true claim sharing the word. No change.
- `tests/support` in `architecture.md:47`, `:384`, `:385`, `:408`, `:424` (×3), `:425`; `security-plan.md:603`;
  `test-plan.md:623` (×2), `:627` (×2), `:628`, `:658`, `:690`, `:972`, `:1079`, `:1086`; the key files
  `5-command-implementation.md:33` (five more sites), `bootstrap-phases-…:42`, `test-data-bootstrap.md:11`, `:12`,
  `:13`, `:17`: each names one support file or the directory for another fact (the home keepers, the outer PTY, the
  piped driver, the hygiene checker, the canary constant, the fixture chain). None lists the helper files and none
  says a test file carries its own copy. No change.
- `test-plan.md:456`: the amended helper list. Its own text.
- `per-file` in `security-plan.md:323`, `:418`: a per-file SHA-256 of the vendored ConPTY. Another subject.
- `bootstrap-phases-…:16` (`e2e-wall`): the amended line; `78 m` stands as measured at its chunk, the new wall
  beside it. `:13` (`e2e-711`): "0 Timeout grades over 711 Linux viola-e2e mutants", a reading of its own chunk
  that this chunk's 0 timeouts over 718 does not contradict. No change.
- `test-plan.md:1212` (`package-score`, `whole-member`, `not-measured`): the amended Mutation gate paragraph. Its
  own text.
- `test-plan.md:437`; `5-command-implementation.md:24`, `:48`, `:49`, `:66`; the architecture key file
  `ci-cd-approach.md:5` (`package-score`, `whole-member`): each states the form (`run --mutants --package
  <member>`, the boundary tier, verdict `package`). None says a score is missing or names a wall. No change.

**leaf (18 rows, 10 files)**
- `.claude/docs/tests-summary.md:42` (`whole-member`, `not-measured`): re-derived. The Mutation row now carries the
  first whole-member `viola-e2e` score beside the owed-mutant rule.
- `.claude/rules/testing.md:48` (`whole-member`): the `run --mutants` forms, unchanged by the pass. The same
  file's §Framework integration bullet was re-derived from test-plan §2 with no row: it now names the shared
  helpers in `tests/support/` and that a root test file defines no copy of its own. `:34` (`support-dir`): the home
  base keeper. No change.
- `.claude/rules/verification-harness.md:6`, `:43`, `:44`: its `paths:` entry, the root fixture chain and the
  `--package` form. It carries neither the count of waits nor a helper list. Re-derived: no change.
- `.claude/docs/gotchas.md:111` (`CHILD_WITHIN`, viola-pty's own bound, another constant), `:117` (the piped
  driver), `:123` (`wait_endpoint_gone` named with no count or ordinal). Re-derived: no change.
- `.claude/docs/commands.md:35`, `:46`: the `--package` form. Re-derived: no change.
- `.claude/docs/conventions.md:13`, `.claude/docs/services/viola-pty.md:14`: one support file named for another
  fact. No change.
- `.claude/docs/design-summary.md:68` (`ninth`), `.claude/docs/security-summary.md:68`, `.claude/docs/stack.md:42`
  (`per-file`): the true claims of the standing rows above. No change.
- `CLAUDE.md`: no row. Its generated blocks were read against the amended architecture row and test-plan
  sections: none carries the Watch report, the helper files or a mutation score. No change.

**curation (2 rows, 1 file)** — `.claude/rules/testing.md:71` (`tests/support/verify.rs`, the stamped-home
fixture's verify step) and `:84` ("the whole unit suite"), both in its Session Additions. True claims of other
subjects; preserve-verbatim, nothing routed to P3.

**base (0 rows)** — neither judgment base holds a swept word.

## Leaves re-derived
`.claude/docs/tests-summary.md` and `.claude/rules/testing.md` changed. Read whole against the amended passages
and standing: `CLAUDE.md`, `.claude/rules/testing-src.md` (a pointer list into `testing.md`'s section names, none
renamed). Read at their sweep rows only, by window, and standing: `.claude/rules/verification-harness.md`,
`.claude/docs/gotchas.md`, `.claude/docs/commands.md`. Not opened: `.claude/docs/services/viola-state.md`, whose
header names architecture §Occupied Resources (Filesystem); the sweep printed no row in it for any of the twelve
patterns, so it carries neither the Watch bound nor the count.
