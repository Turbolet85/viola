# Cascade dispositions — 2026-10-14-resume-and-compact-ledger-rows

## Step 2 — the sweep
- `cascade.py sweep --patterns-file cascade-patterns.toml`, baseline 880b0563 (the pre-CI parent).
- Twelve patterns, one per retired claim, each spanning its verbs and spellings:
  - `rows21` and `rows18`: the 21 / 18 relied counts, `/21]`, `/18]`, "first eighteen", `[&str; 21]`, "18-id";
  - `stops-newline`: the Windows step list ending at `paste-newline`;
  - `verified-win`: an 18-id stamp verifying on Windows;
  - `verify-runs`: four interactive or typed runs, A to D, five starts;
  - `spawn-six`: six spawn pairs;
  - `killed-exit`: no `child_exit_status` for a killed child;
  - `resume-norow`: `--resume` as the dependence with no row, and the owed resume payload;
  - `compact-owed`: `/compact`'s owed row and `/remote-control` as the only member with none;
  - `fixtures31`: the 2.1.287 set at 31 files;
  - `fake-resume`: the fake agent's resume launch built on `SessionStart.default`;
  - `case-count`: the local-command case count.
- Totals, copied from the listing: `total (12 patterns) · 53 rows over 17 files`; per class `new 6/5 · standing 11/4 ·
  leaf 36/11 · curation 0/0 · base 0/0`.
- Zero-row patterns (control fired, so each zero is a statement about its pattern): `stops-newline`, `fixtures31`,
  `case-count`.
- Every master row was read at its line; a line over 2 000 chars was read by `cascade.py window` at the row's offset.

### Master and registry rows (17), every one no change
- `architecture.md:91` `rows21` @c15941 — the older-stamp clause written this pass ("exactly the twenty-one ids every
  stamp written before chunk 2026-10-14-…"): true, new text.
- `architecture.md:91` `rows21` @c17339 (window 17139–17539) — the owed-ledger-rows history (`stamped 2.1.287  21 pass
  0 fail`; five starts each): a dated measurement, true.
- `architecture.md:253` `rows21` new; `rows18`; `verified-win` — the envelope sentence written this pass: true.
- `architecture.md:473` `rows21` @c3473 — the statusline rows' history (`stamped 2.1.287  21 pass  0 fail`): dated,
  true.
- `architecture.md:91` `rows18` @c1904, @c15845 and `verified-win` @c16130 — this pass's relied-set and older-stamp
  text: true.
- `architecture.md:91` `verify-runs` @c14388 (window 14188–14588) — the paste-newline history ("one round, five
  starts"): dated, true.
- `architecture.md:91` `verify-runs` @c17408 — the owed-ledger-rows history ("five starts each"): dated, true.
- `security-plan.md:617` `rows21` new and `rows18` — this pass's older-stamp and relied-set text: true.
- `test-plan.md:568` `rows21` new — this pass's oracle sentence: true.
- `registries/contracts/architecture/ci-cd-approach.md:20` `rows18` new — this pass's text: true.
- `registries/contracts/test-plan/5-command-implementation.md:8` `rows18` — this pass's boot step 4: true.
- `registries/contracts/test-plan/5-command-implementation.md:53`:
  - `rows21` @c2376, @c2548 (window 2348–2748) and `verify-runs` @c2471 — the dated twenty-one-row live firing ("five
    starts"): true.
  - `rows18` @c1318 — this pass's 19-id form: true.
- `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10` `verify-runs`:
  - @c880 — the unknown-CLI-version harness case's four runs: still true, since Run E spawns nothing with no fixture
    set (this pass says so).
  - @c2501 — "verify's four interactive runs … when it was sized": dated, true.
  - @c3720 — this pass's sentence: true.

### Leaf rows (36 over 11 files) → cascade step 3
Each row's leaf joins step 3's set (`leaves --add`):
- `CLAUDE.md:43`
- `.claude/rules/verification-harness.md:28`, `:39`
- `.claude/rules/security.md:19`
- `.claude/rules/observability.md:39`
- `.claude/docs/commands.md:26`, `:27`
- `.claude/docs/security-summary.md:38`, `:75`
- `.claude/docs/services/viola-agent-claude.md:11`, `:26`
- `.claude/docs/services/viola.md:6`, `:35`
- `.claude/docs/tests-summary.md:21`
- `.claude/docs/workflow.md:49`
- `.claude/docs/stack.md:14`

### Curation and base rows
None: 0 curation rows and 0 base rows.

### Not looked for
- The masters' own retellings in other words than these patterns. The detectors' per-doc sweeps are the control for
  reach.
- Sidecars and archives, which are history.

## Leaves
- Mode: pass (this chunk does not close Epoch 5), `cascade.py leaves --add` with every step-2 leaf. The tool reads 13
  groups and 26 leaves; two are unmapped:
  - `.claude/rules/host-linux.md` (host): the host's own leaf, no master's, no comparer;
  - `.claude/rules/testing-src.md`: a pointer file of section names, which states no master claim.
- One batch of 13 comparers ran inside one snapshot pair; `diff-tree` printed 13 `A` lines, index and head unchanged.
- `design-system+layout-templates`: its first file read 79 lines where the leaf holds 78, so it was refused.
  - The file was moved aside to `.raw-leaf-design-system+layout-templates.md` and re-spawned once in its own pair, which
    printed one `A` line.
  - The re-spawn's file reads 0 findings.
- `leaves --apply` total: `total · blocks 49 · applied 49`. Every finding was applied; none was written by hand and
  none declined.

| Group | Findings | Applied |
|---|---|---|
| architecture-1 | 10 | 10 |
| architecture-2 | 6 | 6 |
| architecture-3 | 1 | 1 |
| architecture+security-plan | 0 | 0 |
| architecture+security-plan+design-system+layout-templates | 0 | 0 |
| CLAUDE.md (all six changed masters) | 3 | 3 |
| architecture+test-plan+obs-plan | 7 | 7 |
| security-plan | 11 | 11 |
| security-plan+obs-plan | 0 | 0 |
| design-system+layout-templates | 0 | 0 |
| test-plan | 10 | 10 |
| test-plan+obs-plan | 1 | 1 |
| obs-plan | 0 | 0 |

- Each finding's source side was checked at its cited source line against the masters as amended this pass; every one
  restates a fact this pass applied.
