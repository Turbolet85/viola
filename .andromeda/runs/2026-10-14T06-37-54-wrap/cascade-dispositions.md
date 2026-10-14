# Cascade dispositions — 2026-10-13-owed-ledger-rows

The search: `cascade-patterns.toml`, eleven patterns over the seven masters, every `.andromeda/registries/**` file,
the three curation homes, the two judgment bases and the leaves (`cascade.py sweep`, baseline `14e62329`, the pre-CI
commit's parent; listing `cascade-sweep.txt`). Each pattern is a retired claim's own wording or mechanism:
- `rows18`: the eighteen-row count and its `/18` / `18 pass` spellings;
- `all-len`: the `LedgerRow::ALL` row set read as the relied set or `MM`;
- `no-row`: the statusline shapes "with no row" / "owed";
- `owed-entry`: "Owed ledger rows" as an owner;
- `capture-only`: the capture arm "called only by" the probe plugins;
- `only-both`: the fake agent's settings command "only with both";
- `user-statusline`: Run B running the user's own status line;
- `verdict-every`: the verdict over "every landed row";
- `probe-dir`: the probe dir's four subdirs;
- `files29`: the 2.1.287 set's 29 files;
- `scrub-users`: fixtures scrubbed of usernames.

The sections read: architecture [CLI Version Compatibility], [Hook Contract], [Plugin Scope], §Standard Contracts
(stamps envelope, list, hook statusline), §Occupied Resources, §Cross-cutting Patterns; security-plan §Threat Model
Summary, §Input Validation, §Data Protection, §Error Handling, §Security Anti-Patterns; test-plan §1–§5, §7, §11;
obs-plan §1–§4, §10; design-system pattern 5; layout-templates `viola verify`.

Totals, copied from the listing: `total (11 patterns) · 67 rows over 22 files` · `per class · new 6/4 · standing 30/9 ·
leaf 30/11 · curation 1/1 · base 0/0`. `probe-dir` and `files29`: 0 rows, each control fired.

## Master and registry rows (36: new 6, standing 30)
- `architecture.md:91` rows18 ×4 — amended: the new "first eighteen" (Windows relied set), `[18/18] paste-newline`
  (the Windows step line), and the paste-newline chunk's dated `18 pass  0 fail` reading (history, kept).
- `architecture.md:253` rows18 ×2 (new) — amended: the stamps-envelope verdict, the Windows count.
- `architecture.md:473` rows18 — no change: the paste-newline chunk's dated `18 pass` reading, kept as history.
- `security-plan.md:617` rows18 ×2 (new) — amended: the Windows relied set, counter `/18`.
- `test-plan.md:657` rows18 — amended: the Windows step line `[18/18]`.
- `test-plan.md:753` rows18 — no change: a dated live reading (history).
- `registries/…/ci-cd-approach.md:20`, `…/5-command-implementation.md:8` rows18 (new) — amended: the Windows relied
  count.
- `registries/…/bootstrap-phases-derive-for-route-setup-project.md:10` rows18 — no change: "eighteen verify-driven
  binaries" counts nextest binaries, not ledger rows.
- `architecture.md:91`, `security-plan.md:617`, `design-system.md:811`, `…/crate-dependency-direction.md:6` all-len —
  amended: `LedgerRow::ALL` now names the twenty-one-row set, beside `relied()` as the verdict and `MM`.
- `architecture.md:48` no-row — no change: "with no row read" is the screen gate, another claim.
- `architecture.md:473` no-row ×3 — amended: the one dependence still without a row is `viola revive`'s.
- `design-system.md:605` no-row — no change: a layout "row gap".
- `obs-plan.md:739` no-row — no change: "no row text" in a spawn line.
- `a11y-plan.md:173` no-row — no change: the unwrapped rows owed to "Unwrapped sessions on the board", true.
- `architecture.md:291` owed-entry (new) — amended: the row moved FROM "Owed ledger rows".
- `architecture.md:473`, `test-plan.md:1094` owed-entry ×2 — amended: "directly after "Owed ledger rows"" places the
  new entry.
- `architecture.md:69` capture-only — amended: both callers named.
- `architecture.md:91` @2881, `:383`, `:412`; `security-plan.md:282` capture-only — no change: the print probe's
  `hook --capture` arm, which reads no `VIOLA_*`; both forms share it.
- `architecture.md:411` capture-only — no change: `update_stamps` "called only by `viola verify`", another claim.
- `security-plan.md:231` capture-only ×2 — amended: the event arm's caller and the new statusline arm's.
- `architecture.md:100` user-statusline (new) — amended.
- `architecture.md:327` user-statusline — no change: the source rule for `--home`.
- `security-plan.md:291` user-statusline — amended in this pass (a duplicate of architecture 7's claim: Run B on
  Unix runs the probe capture, not the user's status line).
- `a11y-plan.md:823`, `registries/…/a11y-plan/keyboard-test-harness.md:8` user-statusline — no change: the
  pass-through runs the user's command, true.
- `security-plan.md:565`, `test-plan.md:1099`, `test-plan.md:1312` scrub-users — amended: the kept `workspace.repo`.

## Curation and bases
- `.claude/rules/testing.md:72` capture-only (curation) — no change: "called only by `cfg(unix)` cases", another claim.
- bases: 0 rows.

## Leaf rows (30 over 11 files) → step 3
Each leaf named joins step 3's set by `--add`: `CLAUDE.md`, `.claude/docs/commands.md`,
`.claude/docs/security-summary.md`, `.claude/docs/services/viola-agent-claude.md`, `.claude/docs/services/viola.md`,
`.claude/docs/tests-summary.md`, `.claude/docs/workflow.md`, `.claude/rules/observability.md`,
`.claude/rules/security.md`, `.claude/rules/testing.md`, `.claude/rules/verification-harness.md`.

## Leaves
Mode: pass (this chunk does not close its epoch). Set: `leaves-set.txt` (13 groups, 26 leaves; `unmapped 2`). Comparer
batch path check: before `tree cd3ce4b78d0bff848357f1c4a0bcc9d83b0a94eb index f93413140f2811bfdb65f7fe54ecbbe1e11f8b97
head 1c4f2258ef7c20650f7fd099f7e5ba5a1f110966`, after `tree 1654030a15a7a3203a0036b643d55ad6cff93e74`, exactly the 13 `A`
lines, index and head unchanged. Every receipt's count matched its file's blocks.

Per group — findings · applied · by hand · declined:
- architecture-1: 8 · 8 · 0 · 0 (`services/viola-agent-claude.md`)
- architecture-2: 5 · 5 · 0 · 0 (`services/viola.md` 4, `workflow.md` 1)
- architecture-3: 0
- architecture+security-plan: 1 · 1 · 0 · 0 (`rules/api.md`)
- architecture+security-plan+design-system+layout-templates: 0
- architecture+…+obs-plan (CLAUDE.md): 5 · 5 · 0 · 0 — read before the write: the directory line, the
  `statusline` module, the `hook statusline` `--capture` form, the relied set of twenty-one rows, the owed dependences
- architecture+test-plan+obs-plan: 4 · 4 · 0 · 0 (`docs/commands.md`)
- security-plan: 10 · 10 · 0 · 0 (`security-summary.md` 6, `rules/security.md` 4)
- security-plan+obs-plan: 0
- design-system+layout-templates: 0
- test-plan: 12 · 12 · 0 · 0 (`tests-summary.md` 7, `rules/testing.md` 1, `rules/verification-harness.md` 4)
- test-plan+obs-plan: 1 · 1 · 0 · 0 (`rules/observability.md`)
- obs-plan: 1 · 1 · 0 · 0 (`obs-summary.md`)

Total, copied from the tool: `total · blocks 47 · applied 47` (`leaves-apply.txt`).

Unmapped, placed by hand: `.claude/rules/host-linux.md` (the host leaf, setup's own statements, no comparer's) and
`.claude/rules/testing-src.md` (a pointer file naming `testing.md` sections, no statement of a master) — declined,
no master statement to compare.
