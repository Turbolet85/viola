# Cascade dispositions — 2026-10-31-sanitised-error-surfaces

## The search
One `cascade.py sweep` over the twelve apply writers' `## Patterns` (45 patterns, ids `a11y-*`, `arch1-*`, `arch2-*`, `ds-*`, `lt-*`, `obs1-*`, `obs2-*`, `sec1-*`, `tp1-*`, `tp2-*`, `tp3-*`), over all seven masters, every file under `.andromeda/registries/`, the leaves, the three curation homes and the two judgment bases, against the baseline `7bf5c32a`, after the last amendment of this pass. Every pattern's control fired. The trail is `cascade-2026-10-31-sanitised-error-surfaces.json` in this run dir.

total (45 patterns) · 16 rows over 12 files
per class · new 1/1 · standing 7/6 · leaf 8/5 · curation 0/0 · base 0/0

## Rows
- `.andromeda/a11y-plan.md:257` · `a11y-03` standing edited · amended: «one document with no hint» now carries its exception on the same line.
- `.andromeda/a11y-plan-detail/cognitive-accessibility.md:19` · `a11y-03` standing edited · amended: the same.
- `.andromeda/design-system.md:263` · `ds-01` standing edited · amended: the one-document rule carries its exception on the same line.
- `.andromeda/design-system-detail/surface.md:145` · `ds-04` standing edited · amended: the clap text kept on the row, as answered at Q5.
- `.andromeda/architecture-detail/conventions.md:15` · `ds-05` standing edited, markup · amended: a reader's opaque rule kept for a code word, as answered at Q8.
- `.andromeda/architecture-detail/conventions.md:76` · `sec1-02` standing edited · amended: `wrapper-fault` now reads through `sanitised_fault` on the same line.
- `.andromeda/test-plan-detail/e2e-test-strategy.md:259` · `tp1-02` standing edited · amended: the line states that no helper runs after every E2E test.
- `.andromeda/security-plan-detail/error-handling.md:31` · `tp1-02` new · amended: this pass's own statement that no such check runs.
- `.claude/docs/conventions.md:15` · `arch1-03` leaf · re-derived in step 3: `CoreError`.
- `.claude/docs/services/viola-mcp.md:23` · `arch1-06`, `obs2-01`, `sec1-04`, `tp1-01` leaf · re-derived in step 3: exit 11.
- `.claude/docs/commands.md:22` · `arch2-04` leaf · re-derived in step 3: exit 11 before `initialize`.
- `.claude/docs/services/viola.md:45` · `arch2-04` leaf · re-derived in step 3: the same.
- `.claude/docs/services/viola-agent-claude.md:11` · `arch2-06` leaf · re-derived in step 3: `Refusal`.

## Added to step 3's set from the report
`.claude/docs/stack.md`, `.claude/docs/services/viola-core.md` (`CoreError`) and `.claude/rules/observability.md` (`mcp`'s `process-exit`), named by the report's Spec claims disproved; their words stand outside these patterns, so the comparers read them whole.

## The lateral binds
- `test-plan §3 ↔ obs-plan §3`, harness commands: no harness verb moved (report, Harness / gate surface), no change.
- `test-plan §3 Log format ↔ obs-plan §6`: `channel-response`'s `detail` is a field obs-plan §6 already named, now closed; no required test field renamed, no change on the §3 side.
- `a11y-plan schema ↔ the log schema`: the a11y contract emits no violation rows and no a11y field moved (`fanout-a11y-plan.md`'s note), no change.

## Leaves
Mode: pass, 15 groups, 27 leaves (`total · pass · 15 group(s) · 27 leaves · 395967 B · unmapped 2`). `total · blocks 63 · applied 49 · held 14`. By hand: none. Every finding's source quote was read at its cited line, 63 of 63. Every `retired` and `stale` finding is applied; an `omission` is applied only where a reader of the leaf would otherwise act against the source.

Unmapped: `.claude/rules/host-linux.md` (the host's own, no comparer's) · `.claude/rules/testing-src.md` (`command grep -n -E "Refusal\b|CoreError|exit 11|process-exit|sanitis|fault_detail|opaque"` reads no line).

| group | findings | applied | declined, why |
|---|---|---:|---|
| a11y-plan | 0 | 0 | |
| architecture-1 | 10 | 10 | |
| architecture-2 | 11 | 10 | `viola-mcp.md:36#1`: a witness's name in a test list; the master holds it |
| architecture-3 | 3 | 3 | |
| architecture-4 | 0 | 0 | |
| architecture+design-system+layout-templates | 5 | 1 | `frontend.md:25#1` (the `mcp` role, no frontend's) · `:25#3`, `:25#4`, `:26#1` (`api.md` and `conventions.md` carry them; the detail stays in design-system) |
| architecture+security-plan | 10 | 9 | `api.md:35#1`: `:21` and `:28` state it |
| architecture+test-plan+obs-plan | 1 | 1 | |
| design-system+layout-templates | 3 | 1 | `design-summary.md:50#2`, `:50#3`: the screen and the closed body stay in `surface.md`; no reader acts against it from a summary |
| masters | 3 | 3 | |
| obs-plan | 6 | 3 | `obs-summary.md:29#1` (the reading stands at its owner, `ci-integration.md`) · `:47#1` (no line added; `api.md` holds the reader) · `:49#1` (the census stands in `api.md` and `security.md`) |
| security-plan | 5 | 4 | `security.md:40#1`: the line stays true and the file stays lean |
| security-plan+obs-plan | 0 | 0 | |
| test-plan | 3 | 2 | `testing.md:40#1`: the file's rule stands; the `-32603` limit stays in test-plan §6 |
| test-plan+obs-plan | 3 | 2 | `observability.md:40#1`: no line added, `api.md` and `security.md` hold the screen |
