# Cascade dispositions — 2026-10-19-newer-claude-cli-rows

**The search:** `cascade.py sweep --patterns-file .andromeda/runs/2026-10-20T05-48-20-wrap/cascade-patterns.toml`,
baseline `50e2f761` (the parent of the oldest pre-CI commit). It was run after the last amendment (`replace-2.md`);
trail `cascade-2026-10-19-newer-claude-cli-rows.json`.

Totals, copied from the listing: `total (7 patterns) · 49 rows over 13 files` · `per class · new 16/3 · standing
20/5 · leaf 12/8 · curation 1/1 · base 0/0`.

The patterns, each over every master and registry file for its claim's wording and mechanism:
- **C1-literal** — `for agents|input-box literal|input_box|shift\+tab to cycle`: the literal set and the singular "the
  input-box literal".
- **C2-ibsig** — `ready and turn screens|input-box-signature|long.turn`: the row's screens.
- **C3-stampset** — `2.1.287 alone|alone today|under `2.1.287` only|`2.1.287`'s (six|two|one)|2.1.302`: the stamped
  and recorded sets.
- **C4-fakeopts** — the option counts and the two screen switches.
- **C5-sets** — `Committed sets|12 under each|six under`.
- **C6-residual** — `accepted residual|scratch …|footer reading`.
- **C7-stall** — `wait_ready|stall report|readiness stall`.

## Rows

### C1-literal (new 1 · standing 6 · leaf 4)
- `architecture.md:45` ×9 — amended:
  - @c2098–2126 is the literal set;
  - @c2261–2750 is the new resting-footer sentence;
  - @c3033 is the new `input-box-signature` clause.
  - The gate's rule sentence ("a row contains an input-box literal", @c2553) is a true claim, no change.
- `architecture.md:101` ×2 — @c6385 amended in this pass ("rows that hold **an** input-box literal", `replace-2`);
  @c8588 ("holds no input-box literal for 8.0 s from the paste") no change, true.
- `architecture.md:111` — no change: `--json` for agents is another sense.
- `architecture.md:374` new — amended (`2.1.302`'s screens).
- `security-plan.md:115` — amended in this pass ("rows that hold **an** input-box literal", `replace-2`).
- `test-plan.md:508` (new bullet) and `:509` (the `SIGNATURES` bullet) — amended.
- Leaves — to step 3:
  - `.claude/docs/services/viola-agent-claude.md:11` (`input_box: ["for agents"]`, stale);
  - `.claude/docs/services/viola.md:6`.
  - `.claude/rules/api.md:29` and `.claude/docs/conventions.md:41` are `--json` for agents, another sense, no change.

### C2-ibsig (new 4 · standing 4 · leaf 1)
- `architecture.md:45` ×3:
  - @c1560 (the `quiet-period` row's "the ready and turn screens settle within `GATE_MAX_WAIT`"): no change, true,
    since that row is unchanged;
  - @c2900 and @c3067: amended.
- `architecture.md:101` @c1352, `layout-templates.md:204`, and the key file
  `registries/contracts/test-plan/5-command-implementation.md:43` — row-id lists, no change.
- `architecture.md:315`, `test-plan.md:508`, `:591` and `:962` new — amended.
- Leaf `.claude/docs/services/viola-agent-claude.md:26` (row-id list) — to step 3.

### C3-stampset (new 8 · standing 2 · leaf 1)
- Every `new` row amended: `architecture.md:45`, `:101`, `:374`; `security-plan.md:262`; `test-plan.md:379`, `:603`,
  `:936`, `:970`.
- `architecture.md:110` @c3494 and `security-plan.md:261` @c2854 ("as measured on 2.1.296 and 2.1.302") — no change,
  true (the print probe's residual).
- Leaf `.claude/docs/security-summary.md:72` — to step 3.

### C4-fakeopts (standing 3 · leaf 2 · curation 1)
- `architecture.md:315`, `test-plan.md:591` and `:962` — amended (the counts, `--long-turn-screen`).
  `--tag-turn-screen`'s own clauses stay true.
- Curation `.claude/rules/verification-harness.md:55` ("nine of them since the root package was split") — no change,
  another subject (the mutation jobs). It is not edited (preserve-verbatim).
- Leaves `.claude/rules/verification-harness.md:41` and `.claude/docs/tests-summary.md:21` — to step 3.

### C5-sets (new 1 · standing 2)
- `architecture.md:374`, `test-plan.md:379` and `:936` — amended.

### C6-residual (new 1 · standing 2 · leaf 4)
- `security-plan.md:262` new — amended (the footer reading's residual).
- `architecture.md:110` ×2 and `security-plan.md:263` (the plugin round) — no change, other residuals.
- Leaves `.claude/rules/security.md:17` and `.claude/docs/security-summary.md:71`, `:72`, `:73` — to step 3.

### C7-stall (new 1 · standing 1)
- `architecture.md:45` ×2 — no change: the product's `Gate::wait_ready`, another subject.
- `test-plan.md:563` new — amended (the test-side stall report).

## Lateral binds
- `test-plan §3 ↔ obs-plan §3` and the log-schema binds: no amended passage is in §3 or the log format. Held.

## Same-master duplicates
None stand: every restatement the patterns found is amended or dispositioned above.

## Curation homes and judgment bases
- One curation row (`verification-harness.md:55`), another subject, no route to P3.
- `playbook.md` and `drift-base.md`: 0 rows.

## Leaves

`cascade.py leaves --add` the six step-2 leaf files: `total · pass · 9 group(s) · 24 leaves · 319591 B · unmapped 2`.
One comparer ran per group, in one batch inside one snapshot pair:
- before: `tree 33340190…`, index `7f83e0cf…`, head `4eaca1d3…`;
- after: `tree 71f7c460…`, index and head unchanged;
- `git diff-tree` printed exactly nine `A` lines, one `leaf-{group}.md` each.

The apply's total line: `total · blocks 11 · applied 11`. Every finding was checked at its cited source line before
the apply.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 1 | 1 (`viola-agent-claude.md:11`, the literal set) | 0 | 0 |
| architecture-2 | pass | 0 | 0 | 0 | 0 |
| architecture-3 | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+test-plan | pass | 1 | 1 (`CLAUDE.md:42`, the stamped versions) | 0 | 0 |
| architecture+test-plan | pass | 0 | 0 | 0 | 0 |
| security-plan | pass | 1 | 1 (`security-summary.md:73`, the footer-reading residual) | 0 | 0 |
| test-plan-1 | pass | 5 | 5 (`tests-summary.md:21`: the option count, the second switch, the walked sets, the stamped sets, the resume variants) | 0 | 0 |
| test-plan-2 | pass | 3 | 3 (`verification-harness.md:38` resume sets, `:41` `--long-turn-screen`, `:42` the stall report) | 0 | 0 |

The step-2 leaf rows not changed by a finding:
- `viola.md:6` (the paste hint stands in the input-box literal's place) — true;
- `security.md:17` (verify's own residual list; the footer reading is no verify run) — true;
- `api.md:29` and `conventions.md:41` (`--json` for agents) — another sense.

Unmapped:
- `.claude/rules/host-linux.md` (the host leaf: the host's own statements, no comparer);
- `.claude/rules/testing-src.md` (a pointer list into testing.md that no changed source states).
