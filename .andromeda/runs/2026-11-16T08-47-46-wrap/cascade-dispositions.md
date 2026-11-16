# Cascade dispositions — 2026-11-16-local-command-delivery

## The search
- Pattern files: every `rows-land` dry run of this pass (TX 1926–1932) printed `patterns: none: … changed prose alone
  — no file`; no `cascade-patterns-{doc}-{k}.toml` stands. Main's own patterns, `cascade-patterns.toml`, spanning the
  seven masters, their key files, the leaves, the curation homes and the bases:
  - `m-exact` — the retired mechanism: a local command recognised by exact equality of its typed text
    (`local[- ]commands?…exact|exact…local[- ]commands?|exact equality`, case-insensitive);
  - `m-rows21` — the relied ledger row count 21 (now 22): `21 (relied|capability|ledger)|relied…21|twenty-one|[NN/21]`;
    the tool's forms rule compiled `+21` bare (`forms +twenty-one +21`), so the pattern reads every `exit 21`.
  - Dropped before the sweep, their controls UNFIRED over the masters at c80fe742 (the tool's refusal, first call):
    `m-pastes` (Run B's six pastes) and `m-localcount` (three local commands / `/remote-control, /clear, /compact`) —
    no master held either wording at the baseline; the leaves holding them are read by step 3's comparers.
- Listing: `sweep-pass-1.1.txt` (rows 1/1, read whole). `total (2 patterns) · 228 rows over 26 files`;
  `per class · new 2/2 · standing 180/7 · leaf 44/17 · curation 2/2 · base 0/0`.

## Dispositions
- `new` 2 — no change: architecture `decision/local-commands-by-ledger` (TX 1926's own new text, «exactly a name on
  the ledger's local-command list» — the by-name rule, correct) · test-plan `contract/fake-agent-input/echo-command`
  (TX 1931, «the exact verify echo probe command» — the fake agent's own literal match, correct).
- `standing` 180 (architecture 29 · security-plan 22 · design-system 27 · layout-templates 19 · test-plan 31 ·
  obs-plan 42 · a11y-plan 10) — no change, one class: every row is an `exit 21` site (the CLI exit code), the bare
  `21` the forms rule added; none states a ledger row count. `m-exact` found no standing master row.
- `curation` 2 — no change: `.claude/rules/ci.md:31` and `.claude/docs/session-learnings.md:269` count CI checks
  (`15 → 21`), not ledger rows.
- `base` 0.
- `leaf` 44 over 17 files — to step 3 (`cascade.py leaves` reads them itself). The stale ones seen: `gotchas.md:84`
  (`m-exact`, «send consumes them by ex[act …]»); `security-summary.md:36` («The ledger has twenty-one rows»),
  `services/viola-agent-claude.md:26` («twenty-one closed rows»), `services/viola.md:6` and `workflow.md:49` («all
  twenty-one») — the relied count is 22. `commands.md:24` («a twenty-one-row stamp written before chunk 2026-10-1…»)
  is history — no change. The other `leaf` rows are `exit 21` sites — no change.

## Leaves
- Set: `cascade.py leaves --add .claude/rules/security.md --add CLAUDE.md`, pass mode, baseline c80fe742 —
  `total · pass · 13 group(s) · 25 leaves · 397687 B · unmapped 2`; epoch «Epoch 3 — Driver reliability: turns and
  sends» open (markerless 3), so no whole mode.
- Batch `leaf`: 13 comparers, `VERDICT: 13 of 13 pass · failed: none`, `[PATH_CHECK] PASS`.
- Decision: every file read whole; each finding's source row read in this pass's TX read-backs (1926–1932); the
  `gotchas.md:84` #1 join read against the leaf's following clause. `cascade.py leaves --apply` (all ids):
  `total · blocks 37 · applied 37`. By hand 0 · declined 0.
  - architecture-1 · pass · 6 findings (viola-agent-claude.md 5 · viola-channel.md 1) · applied 6.
  - architecture-2 · pass · 5 findings (services/viola.md) · applied 5.
  - architecture-3 · pass · 1 finding (rules/events.md) · applied 1.
  - architecture+design-system+layout-templates · pass · 0 findings.
  - architecture+security-plan+design-system+layout-templates+test-plan+obs-plan · pass · 5 findings (gotchas.md 3
    · CLAUDE.md 2, its `GENERATED:setup:warnings` and `:architecture` blocks) · applied 5.
  - architecture+security-plan · pass · 1 finding (rules/api.md) · applied 1.
  - architecture+test-plan+obs-plan · pass · 2 findings (commands.md) · applied 2.
  - design-system+layout-templates · pass · 2 findings (design-summary.md) · applied 2.
  - obs-plan · pass · 0 findings.
  - security-plan · pass · 7 findings (security-summary.md 2 · rules/security.md 5) · applied 7.
  - security-plan+obs-plan · pass · 0 findings.
  - test-plan · pass · 8 findings (tests-summary.md 4 · rules/verification-harness.md 4) · applied 8.
  - test-plan+obs-plan · pass · 0 findings.
- Unmapped 2: `.claude/rules/host-linux.md` (the host leaf — no master states it, no comparer's) ·
  `.claude/rules/testing-src.md` (section pointers into testing.md only, repeating no master text) — no change.
- By hand 1 (the sweep's `leaf` row its comparer, architecture-3, raised no finding on): `.claude/docs/workflow.md:49`
  «all twenty-one on every build» → «all twenty-two», and the fake agent's interactive modes gain `--transcript`
  (test-plan `decision/stamping-verify-passes-transcript`, TX 1931) — drafted whole, `rec.py harness land`, tx 1946.
  `commands.md:24` (history) left as found.
