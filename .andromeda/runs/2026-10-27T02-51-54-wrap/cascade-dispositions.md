# Cascade dispositions — 2026-10-27-pinned-copy-race-and-reproducible-builds

## Step 2 — the sweep
The search: one `cascade.py sweep` over the four apply writers' `## Patterns` (`apply-architecture.md` arch-01…05, `apply-security-plan.md` sec-01…03, `apply-test-plan.md` tp-01…06, `apply-obs-plan.md` none), baseline `99977116` (the parent of the oldest pre-CI commit `6c77eadd`), over the seven masters' bodies and detail sets, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaves. Main wrote no master text of its own (no `cascade-patterns.toml`). Every pattern's control fired.

Listing: `total (14 patterns) · 3 rows over 3 files` · `per class · new 0/0 · standing 2/2 · leaf 1/1 · curation 0/0 · base 0/0`.

- `.andromeda/architecture.md:445` arch-01 standing — no change: the Inherited Defaults Database line («ndjson append logs + atomic JSON snapshots (tempfile 3.27.0 `persist` through one shared helper)») speaks of the snapshots and logs, which still go through `replace_private`; the pinned copy is no state file.
- `.andromeda/architecture-detail/stack-and-technologies.md:18` arch-01 standing, edited — no change: the amended row keeps «`persist` through one shared helper … for snapshots and every other replaced file» and adds the pinned copy's `persist_noclobber` placement beside it; the line holds the old phrase once, true as now scoped (re-read whole).
- `.claude/docs/stack.md:17` arch-01 leaf — re-derived at step 3 (the leaf adds to the set).

Patterns arch-02…05, sec-01…03, tp-01…06: 0 rows each, control fired — statements about those patterns, not absence proofs; the wording-reach of P1's site sweep (`report-patterns.toml`, the report's Counts / qualifiers moved and Expected amendments) stands beside them, and the strong reader's own reach found one more stale site, `test-plan-detail/test-scope-summary.md:117`, applied this pass (fanout-test-plan--cascade.md 1).

Binds (step 2's lateral edges): test-plan §3 ↔ obs-plan §3 — no harness command changed; test-plan §3 Log format ↔ obs-plan §6 — no log field changed; a11y-plan schema ↔ the log schema — unchanged (`schemas/diag-line.v1.json` untouched). None moved.

## Leaves
Set: `cascade.py leaves --add .claude/docs/stack.md` (the step-2 `leaf` row), pass mode (this chunk does not close Epoch 6) — `total · pass · 12 group(s) · 24 leaves · 372548 B · unmapped 2`. Unmapped, no comparer: `.claude/rules/host-linux.md` (the host leaf — a statement no master states); `.claude/rules/testing-src.md` (a pointer file naming `testing.md`'s sections, none of their text repeated). One comparer per group, batch `leaf` (12 of 12 pass); `leaves --apply` total: `blocks 12 · applied 10 · held 2`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 0 | 0 | 0 | 0 |
| architecture-2 | pass | 3 | 3 (`services/viola-state.md:11`, `:20` stale, `:24`) | 0 | 0 |
| architecture-3 | pass | 3 | 2 (`stack.md:17` stale, `services/viola.md:20`) | 0 | 1 — `services/viola.md:37`: measurement B's reading stays in its master (architecture [Deployment / Distribution]); no reader of the crate note acts against it |
| architecture-4 | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+test-plan+obs-plan (CLAUDE.md) | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| obs-plan | pass | 0 | 0 | 0 | 0 |
| security-plan | pass | 2 | 2 (`rules/security.md:14`, `security-summary.md:17`) | 0 | 0 |
| security-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
| test-plan | pass | 4 | 3 (`rules/testing.md:33` stale, `:39`, `rules/verification-harness.md:40`) | 0 | 1 — `tests-summary.md:31`: Path 7's as-landed MCP case is detail that stays in `e2e-test-strategy.md`; the summary line stays true |
| test-plan+obs-plan | pass | 0 | 0 | 0 | 0 |
