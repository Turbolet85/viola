# Cascade dispositions — 2026-10-04-dialog-answers-by-dialog-id

The search: `cascade-patterns.toml` (20 patterns, written after the last body amendment of the pass), run by `cascade.py
sweep` (listing `cascade-sweep.txt`); every pattern's control fired on the pre-pass masters. Patterns cover the retired
claims by verb and phrasing: hook events seven / dialog tier joining later · `hook.dialog` unserved · four perf rows /
`pre-tool-use` untimed / perf session stamped · `2.1.283` · `build-timeout-multiplier` · verb and `from` lists without
`answer` · `run`'s stamps read without strict-modes · fixtures recorded by `verify` only · matchers not evaluated · dialog
deadline open · S3/S7/S8 "with dialog answers" · the dated-gap lists · the PermissionRequest → `permission` map · the
exactly-once dialog rule · `:78` as the dialog witness owner · `strict-modes-failed` · the non-`null` stamp gate.
Zero-row patterns (control fired): dialog-later · dialog-unserved · from-list · matcher-off · deadline-open ·
s3-with-answers — each a statement about the pattern after this pass's edits, not an absence proof.

## Master and registry rows
- `architecture.md:251` v2183 — the Ledger stamps envelope example (fenced): an illustrative version key, no change.
- `architecture.md:410` v2183 · `test-plan.md:1084` v2183 — this pass's own text naming both committed sets: true.
- `design-system.md:801` · `layout-templates.md:474` · `:478` v2183 — sample `viola verify` output lines (illustrative;
  the report's "unchanged truth" for the verify sample): no change.
- `5-command-implementation.md:45` timeout-mult — this pass's text naming the replaced flag as the reason: true.
- `security-plan.md:230` stamps-gap — re-read @c936: now `verify` alone before strict-modes, `run` through
  `read_stamps_strict`: true.
- `test-plan.md:408` fixture-source (standing) — STALE: "matches recorded `viola verify` fixtures, including forwarding
  `annotations` (S8)" while S8's fixture is relayed → amended this pass (a same-master duplicate of T3's claim).
- `test-plan.md:438` · `project-directory-structure.md:78` fixture-source — this pass's text: true.
- `security-plan.md:205` · `:207` · `:230` · `:524` dated-gaps — the edited lists, each now carrying the sixth gap / the
  narrowed second gap; re-read @c2000/2876/2898/3580 on :205: true. `security-plan.md:240` — the ConPTY third gap: a
  true claim sharing the token, no change.
- `architecture.md:169` permission-map — edited, carries the continuation exception: true.
- `architecture.md:307` exactly-once — edited: true. `a11y-plan.md:588` — `waiting:` stderr line: unrelated token.
- `strict-detail` rows: `architecture.md:253` · `:401` · `security-plan.md:207` · `:235` · `obs-plan.md:731` · `:835` ·
  `:837` — this pass's text; `design-system.md:779` · `obs-plan.md:532` · `:660` · `:832` · `:834` — exit-21 and refusal
  catalogs, true claims sharing the token: no change.
- `security-plan.md:210` stamp-gate — the accepted-risk row ("no non-`null` decision without a `viola verify` stamp"):
  still true under R2 (the six-row stamp is a stamp); no change. `:604` · `:605` — edited: true.
- `architecture.md:428` · `obs-plan.md:1081` perf-stamped — this pass's "boots unstamped": true.

## Curation homes and judgment bases
- No `curation` or `base` row on any pattern.

## Leaves (step 3 set; each re-derived from the amended source, not only the swept line)
- `.claude/docs/services/viola-agent-claude.md:23` (HookEvent 7) → 9 + `kind()` / `is_dialog()`; the `dialog` module
  bullet added.
- `.claude/rules/verification-harness.md:39` (matchers) · `:43` (2.1.283) · `:45` (four rows) · `:46` (fixture source).
- `.claude/docs/obs-summary.md:31` · `.claude/docs/tests-summary.md:21` · `:27` · `:28` · `:44`.
- `.claude/docs/commands.md:44` (the mutants flag).
- `.claude/docs/services/viola.md:30` (verb list) · `:37` wrapper modules + the answer verb bullet.
- `.claude/rules/security.md:14` (second gap narrowed; the sixth gap) · `.claude/docs/security-summary.md:18` · `:38` ·
  `.claude/docs/services/viola-state.md:22` · `.claude/rules/api.md:30`.
- `.claude/rules/events.md:20` (the kind map's continuation).
- `CLAUDE.md:15` (overview fixtures line) · `:21` (modules: `DIALOG_DEADLINE`) · `:43` (warnings: the ledger-gate dated
  exception).
- `.claude/docs/stack.md:18` (the State-file primitives row, mirrored verbatim from arch §Stack).
- `.claude/docs/services/viola-core.md:6` (`DIALOG_DEADLINE`) · `.claude/docs/gotchas.md:75` (S7 has no row until `:82`).
- Read and left: `.claude/docs/conventions.md` (no refusal-order, exit-1 verb list or kind-map statement) ·
  `.claude/docs/design-summary.md` (no catch-site list) · `.claude/docs/a11y-summary.md` (no §8 timeout statement) ·
  `.claude/rules/api.md:23` (the `answer` order with `control-character` first: true) · `.claude/docs/services/viola-channel.md`
  (method list already names `answer` / `hook.dialog`).

## Validate-side notes
- A22 rejected (Registry over-reach): arch §Stack carries no test-library row (proptest / rstest are not in it either);
  insta is registered in `crate-dependency-direction` and test-plan §2 / §4 / §7.
- T9's "6 passed on green" moved to 7 by the orchestrator's re-read of `crates/viola-e2e/src/harness/run/perf.rs:357`
  (the unit pin of the perf suite's green count), a count the report's `perf::ROWS` 4→5 moves.
- T10 keeps `run::PERF_ROWS` (a re-export of `perf::ROWS`, `run.rs:31`); T17 confirmed `MUTANTS_PROGRESS` carries
  `--build-timeout=400` for both arms (`mutants.rs:24`, `:839`); T19 confirmed `RECORDED_CLI_VERSION` = 2.1.287
  (`tests/support/fake.rs:16`); T11's `stamp: false` confirmed at `perf.rs:178`.
