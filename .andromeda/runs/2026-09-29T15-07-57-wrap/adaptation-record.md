# Adaptation record — 2026-09-29T15-07-57-wrap (0-pending wrap)

Door: U35 registry migration (`references/registry-contract.md` §The U35 door), on the operator's request
("run the U35 door … the founder called it for today"). `--marker no-marker`; staged in `u35/`.

## Stage
`registry.py migrate --stage`: 6 logs (a11y 6 dated entries · design 10 · layout 4 · obs 40 · security 14 · test 20)
+ 4 keyed sections (a11y §3 9 keys · architecture §Infrastructure Patterns 5 · obs §3 10 · test §3 6 = 30 keys).

## Lift rewriters
Six `general-purpose` rewriters, one parallel batch, prompt verbatim: lifts a11y 3 · design 6 · layout 7 · obs 5 ·
security 7 · test 8.

## Verify
- Run 1: FAILED, 12 failures:
  - 6 × record heading. The letter's `{marker}` = the run-dir name `2026-09-29T15-07-57-wrap` fails the tool's
    marker grammar (`RECORD_RE`, lowercase `YYYY-MM-DD-slug`) — a letter-vs-tool mismatch.
  - 6 × lift target inside a mapped §3 section (a11y 3, obs 1, test 2).
- Operator (overseer): marker lowercased; the six §3 lifts move to `u35/k-lifts.toml`, hand-landed in their key files
  after `--apply` at the same anchors, proved by `registry.py check --all`, and named in the records.
- Run 2: FAILED, 6 × record heading. `2026-09-29t15-…` still fails (the grammar needs `-` after the date).
  Operator (overseer): `2026-09-29-t15-07-57-wrap`.
- Run 3: clean — 10 sections · 0 failures · 54 D-ids cited outside a log · 0 markers named only in a log.

## Operator review (D-id rows + lifts, in batches)
- Batch 1 (a11y-plan: 14 D-ids, 3 k-lifts): approved as proposed — 13 history-only (D-21, D-A11Y-02 · 04 · 05 · 06 ·
  07 · 09 · 11 · 15 · 16 · 17 · 18 · 19), D-A11Y-13 lift (carried by k-lift 1), k-lifts 1–3 as written. The operator
  spot-checked L517, L543, L911.
- Batch 2 (design-system + layout-templates: 6 D-ids, 13 lifts): approved as proposed — 6 history-only (design D-20 ·
  D-A11Y-02 · 03 · 06; layout D-A11Y-06 · 19); 11 lifts as written; 2 narrowed to what the log says ("a lift carries
  only what the log says, never a rewriter extrapolation" — operator): design lift 3 drops the unsourced state list
  "(default, `stale`, cocked, cocked + `stale`)"; layout lift 2 keeps only the `space-micro` inline-padding claim
  (the `--rule` box + `radius-sm` generalisation dropped; the body states those for `expired` only).
- Batch 3 (obs-plan D-01 … D-19: 16 D-ids — presented as "17 … 16 history-only", a miscount; the approved id list is
  exact): approved as proposed — 15 history-only (D-01 … D-06, D-08 … D-13, D-16, D-17, D-19); D-07 lifted by obs lift 3 (§4 Scenario 4); D-11's json-subscriber fallback clause carried by the obs
  §3 k-lift (reviewed in batch 4).
- Batch 4 (obs-plan D-21 … D-36: 14 D-ids, 5 obs lifts): approved as proposed — 10 history-only (D-21, D-22, D-25 …
  D-32); lifts D-23 (obs §3 k-lift → Logging stack), D-34 (lift 4, §4 E2), D-35 (lift 5, §6 Child / shell spawns),
  D-36 (lift 2, §4 Scenario 1); all 5 obs lifts as written; the obs record's Section names the lift-5 anchor
  "§6 Log Coverage (Child / shell spawns)" (was "→ Boundary-call wrappers").
- Batch 5 (test-plan: 4 D-ids, 6 lifts + 2 k-lifts; security-plan: 0 D-ids, 7 lifts): approved as proposed — 4
  history-only (test D-22, D-28, D-30, D-A11Y-14); all test lifts and both test k-lifts as written; security lifts 1,
  3–7 as written; security lift 2 widened to the log's whole clause (the GUI connection / SSE stream-holding half
  added beside the disk half) — "carry all of what the log says, and nothing beyond it" (operator).
- Record marker confirmed `2026-09-29-t15-07-57-wrap` (overseer1: any passing form is safe).

## Totals
54 D-ids reviewed (14 + 6 + 16 + 14 + 4): 48 history-only · 6 lift (D-A11Y-13, D-07, D-23, D-34, D-35, D-36), each
carried by an approved lift; D-11 history-only, its fallback clause carried by the obs k-lift. 36 lifts: 30 through `migrate --apply`
(design 6 · layout 7 · obs 4 · security 7 · test 6) + 6 hand-landed in §3 key files (a11y 3 · obs 1 · test 2);
3 narrowed/widened to the log (design 3, layout 2, security 2).

## Operator go
"Go: re-verify, then migrate --apply, hand-land the six key-file lifts, registry.py check --all, and re-detect at
the worktree and at HEAD after the commit." — the operator (overseer), 2026-09-29.
