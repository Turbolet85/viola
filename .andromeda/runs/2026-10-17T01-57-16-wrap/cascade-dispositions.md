# Cascade dispositions — 2026-10-17-end-to-end-path-binary-and-harness-legs

The search: `cascade.py sweep --patterns-file cascade-patterns.toml` (baseline `6042af63`, the pre-CI parent), 11 patterns
written from every amendment of this pass before the first grep — the retired regex (`old-regex`), the unbuilt-`--e2e`
clauses (`e2e-unbuilt`), the unbuilt boot flags (`flags-unbuilt`), the fixture-owned / rstest `harness_session`
(`session-fixture`), the status interim and its derivation (`status-interim`), the `last_error` enum (`last-error`), the
0.9.133 qualifier (`nextest-0133`), the verify-driven list (`verify-driven`), the `AGENT_RUN_KEEP_*` readers
(`keep-readers`), the agents-mode file (`agents-file`) and the owner title (`owner-title`). Every control fired.

Listing totals, copied: `total (11 patterns) · 50 rows over 15 files` · `per class · new 7/4 · standing 21/10 · leaf
21/5 · curation 1/1 · base 0/0`.

## Masters and registries (new 7 · standing 21)
- `registries/contracts/obs-plan/heartbeat-ticks.md:11` status-interim standing — **amended** (the test↔obs bind): `degraded` when a booted instance's item is `stale` or absent.
- `registries/contracts/test-plan/status-endpoint-shape.md:17` status-interim new; `:11`, `:17` last-error — this pass's text, re-read: no intra-line duplicate of the interim; no change.
- `registries/contracts/test-plan/5-command-implementation.md:59` last-error new, `:136` last-error edited — this pass's text; no change.
- `5-command-implementation.md:21` nextest-0133 new — this pass's text (the first measurement kept as history beside 0.9.146); `:115` nextest-0133 standing — a different measurement (the JUnit store path under llvm-cov, 0.9.133), true; no change.
- `bootstrap-phases-derive-for-route-setup-project.md:10` verify-driven edited @c1191 — this pass's text (twenty names); no change.
- keep-readers: `architecture.md:328`, `:329` edited (this pass); `architecture.md:353`, `test-plan.md:975`, `:1027`, `:1096`, `obs-plan.md:965`, `registries/contracts/architecture/ci-cd-approach.md:5` (@c587, @c1426), `:20`, `registries/contracts/obs-plan/log-file-location.md:8`, `5-command-implementation.md:35` (@c1253, @c1282, @c1345), `:39`, `:74`, `:86`, `test-data-bootstrap.md:16` (@c1259, @c1658, @c1714, @c1807, @c2559, @c2586) — each names the variables' values or CI's setting, none states the closed reader set the amendment widened; no change. `5-command-implementation.md:26` keep-readers new — this pass's text.
- agents-file: `architecture.md:367`, `test-plan.md:446` new, `5-command-implementation.md:4` edited (@c1729, @c2355) — this pass's text; no change.
- Masters with 0 rows for old-regex, e2e-unbuilt, flags-unbuilt, session-fixture and owner-title: every master site of those claims was amended in this pass (their controls fired on the pre-pass text).

## Curation homes (1)
- `.claude/rules/verification-harness.md:70` keep-readers (`## Session Additions`, 2026-10-11) — `schema-check` reads kept homes only; still true; no change, nothing routed to P3.

## Leaves (21 rows over 5 files) → cascade step 3's set
`.claude/rules/testing.md` (:20, :45), `.claude/docs/commands.md` (:27, :33, :37, :38, :71), `.claude/rules/verification-harness.md`
(:30, :33, :34, :42, :43), `.claude/docs/tests-summary.md` (:17, :20, :30), `.claude/docs/conventions.md` (:13) — each
leaf joins step 3 (`--add` where the set does not already hold it); re-derived there, never patched here.

## Leaves
Set: `cascade.py leaves --run-dir … --marker … --add` the five step-2 leaves — `total · pass · 7 group(s) · 23 leaves ·
245999 B · unmapped 2`. Unmapped: `.claude/rules/host-linux.md` (the host's own leaf; no source states it) and
`.claude/rules/testing-src.md` (a pointer list into testing.md's sections; no master statement) — no comparer, no edit.
Apply: `cascade.py leaves --apply …` — `total · blocks 16 · applied 12 · reshaped 4`.

| group | mode | findings | applied (tool) | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 1 | 1 (`conventions.md:13`) | 0 | 0 |
| architecture-2 | pass | 1 | 1 (`services/viola.md:48`) | 0 | 0 |
| architecture-3 | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan+obs-plan | pass | 6 | 4 (`commands.md:27`, `:33#3`, `:37`, `:38`) | 2 (`commands.md:33#1` boot flags, `:33#2` `--e2e` — `reshaped`: a bullet line, not a table; the replace-with adds a pipe inside code) | 0 |
| test-plan | pass | 8 | 6 (`tests-summary.md:17`, `:30`; `testing.md:45`; `verification-harness.md:30`, `:33`, `:42`) | 2 (`testing.md:20`, `verification-harness.md:28` — `reshaped`, the same pipe-in-code reason) | 0 |
| test-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| obs-plan | pass | 0 | 0 | 0 | 0 |

Each finding's source side was read at its cited line (the amended key files and `architecture.md:328`, `:367`,
`test-plan.md:398`, `:765`). Read-back: no leaf holds the retired wording (`grep -E` over CLAUDE.md, `.claude/rules/`,
`.claude/docs/`, `.claude/docs/services/` for the old regex, the interim, the fixture, the unbuilt flags and
`owed to "End-to-end path binary`: no hit).
