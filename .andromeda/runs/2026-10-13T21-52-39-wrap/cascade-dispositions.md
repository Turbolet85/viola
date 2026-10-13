# Cascade dispositions — 2026-10-13-paste-newline-ledger-row

## The search
- The patterns are `cascade-patterns.toml`, eight of them, derived from every amendment of this pass:
  - `rows17`: the seventeen-row count in every spelling — `seventeen`, `17 rows`, `/17]`, `[&str; 17]`,
    `17 pass  0 fail`, `MM` is 17;
  - `pastes-count`: Run B's added-paste count, by its verbs and nouns — two/three added, framing or compiled pastes,
    turns or paste rows; four bracketed pastes; four compiled literals; four pastes;
  - `owner-pnlr`: the retired owner, "Paste newline ledger row";
  - `variants-two`: the two-variant `--framing` list, and four framing variants or payloads;
  - `dialogless13`: the dialog-less summary `13 pass  4 fail`;
  - `row18`: the hypothetical "18th row";
  - `lf-unrowed`: the LF-inside-a-paste shape as unrowed — "has no ledger row and no `viola verify` probe" · "no ledger
    row or `viola verify` probe covers";
  - `relied-three`: "three relied-on".
- Command: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-13T21-52-39-wrap/cascade-patterns.toml`, baseline
  `19483111` (the parent of the pre-CI commit `16b5f5f4`). Every pattern's control fired over the pre-pass masters.
- The listing's own counts: `total (8 patterns) · 30 rows over 14 files` ·
  `per class · new 3/3 · standing 11/5 · leaf 16/9 · curation 0/0 · base 0/0`.
- `row18` and `lf-unrowed` read 0 rows, their controls fired: both retired claims are gone from the masters.
- Sections read beside the listing: the architecture `:91` / `:472` / `:49` lines by window
  (`cascade.py window`), security-plan §Data Protection `:283` / `:291` (Run B's transcript count, unchanged — no
  edit), and the registries' test-plan and architecture keys the amendments touched.

## Master and registry rows (14)
- `architecture.md:91` rows17 standing, edited, ×2: @c7702 is `stamped 2.1.287  17 pass  0 fail` in the dated
  re-verify record of chunk 2026-10-08 — no change, a true record. @c11644 is this pass's own clause (a stamp
  holding the seventeen ids written before this chunk reads unverified) — amended text, true.
- `security-plan.md:617` rows17 new: this pass's own clause (the seventeen of an older stamp) — amended, true.
- `test-plan.md:566` rows17 standing: the seventeen `typed_text` cases of 2026-10-09 — a true claim sharing the
  token, no change.
- `registries/contracts/test-plan/5-command-implementation.md:53` rows17, two rows, ×3 between them: the seventeen-row
  live home reading `row-missing` until it is re-stamped (this pass's own) and the dated live firing at seventeen rows
  of chunk 2026-10-08 with its `17 pass  0 fail` — amended and records, no further change.
- `security-plan.md:131` pastes-count standing, edited, ×2: "Run B's first three added pastes are the founder's
  live answer of 2026-10-06" and "the card named the three compiled pastes" — the dated record of the earlier
  answer, kept by this pass's amendment; true.
- `security-plan.md:600` pastes-count standing: the four harness prefixes (compiled literals) — unrelated true
  claim, no change.
- `test-plan.md:565` pastes-count standing: "two pastes under one id" (the hook-mapping cases) — unrelated, no
  change.
- `test-plan.md:656` pastes-count new: "the three paste rows pass on the fake agent's echo" — this pass's own,
  true.
- `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10` pastes-count standing,
  edited: "Run B's three added turns make it ten 300 ms waits" — the dated 2026-10-06 sizing record, kept as the
  proposal asked, with the new four-turn sentence after it; no change.
- `architecture.md:49` owner-pnlr standing, edited: the entry the row landed on ("the working-route entry 'Paste
  newline ledger row', placed at the head of Epoch 5 …") — true history, amended.
- `architecture.md:291` new, `architecture.md:472` ×2 standing edited, `security-plan.md:248` standing edited,
  `test-plan.md:1093` standing edited — owner-pnlr: each reads "moved … from 'Paste newline ledger row'" beside the
  new owner "Owed ledger rows" — amended, true.

## Leaf rows (16, over 9 files) — each adds its leaf to step 3's set
- `CLAUDE.md:43` — rows17, owner-pnlr ×2, relied-three: the Critical Warnings line, `GENERATED:setup:warnings`.
- `.claude/rules/verification-harness.md:28` — rows17: boot step, seventeen rows.
- `.claude/rules/security.md:19` — pastes-count: Run B's four pastes.
- `.claude/docs/commands.md:26` — rows17 ×2: seventeen rows, `[NN/17]`.
- `.claude/docs/security-summary.md:38` — rows17, dialogless13.
- `.claude/docs/services/viola-agent-claude.md:25`, `:26` — owner-pnlr, rows17.
- `.claude/docs/services/viola.md:6` — rows17.
- `.claude/docs/tests-summary.md:21` — rows17, pastes-count, owner-pnlr, variants-two.
- `.claude/docs/workflow.md:49` — rows17.

## Curation homes and judgment bases
- No row in either class (`curation 0/0 · base 0/0`).

## Leaves
`cascade.py leaves` in pass mode: this chunk does not close Epoch 5, which has markerless entries left. The nine
leaf rows above were passed by `--add`. The tool's line was `total · pass · 10 group(s) · 25 leaves · 272533 B ·
unmapped 2`. Ten comparers ran in one batch inside one snapshot pair; `git diff-tree` printed exactly the ten `A`
lines, with index and head unchanged. `leaves --apply` total: `blocks 20 · applied 20`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 3 | 3 | 0 | 0 |
| architecture-2 | pass | 3 | 3 | 0 | 0 |
| architecture-3 | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+design-system+layout-templates | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+design-system+layout-templates+test-plan (CLAUDE.md) | pass | 2 | 2 | 0 | 0 |
| architecture+test-plan | pass | 3 | 3 | 0 | 0 |
| security-plan | pass | 3 | 3 | 0 | 0 |
| design-system+layout-templates | pass | 0 | 0 | 0 | 0 |
| test-plan | pass | 6 | 6 | 0 | 0 |

- Each finding's source side was read at its cited master or key-file text; all twenty are this pass's amendments.
  The two `omission` findings (`viola-agent-claude.md:26`, `commands.md:27`) are applied: a reader of either leaf
  would otherwise take an older seventeen-row stamp as verified.
- Every sweep `leaf` row is covered: `CLAUDE.md:43`, `verification-harness.md:28`, `security.md:19`,
  `commands.md:26`, `security-summary.md:38`, `viola-agent-claude.md:25`–`:26`, `viola.md:6`, `tests-summary.md:21`
  (all four of its patterns) and `workflow.md:49`.
- Unmapped, no comparer: `.claude/rules/host-linux.md` (the host's own leaf; no master states it) and
  `.claude/rules/testing-src.md` (a pointer list into `testing.md`, which states no master claim).
