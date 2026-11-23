# Cascade dispositions — wrap 2026-11-23T05-33-20Z-wrap

Sweep: `cascade.py sweep` over `cascade-patterns-design-system-1.toml`, `cascade-patterns-obs-plan-1.toml` and main's
`cascade-patterns.toml` (part `sweep-pass-1.1.txt`, 188 rows over 25 files). A first firing refused main's `m-kinds`
(«15 kinds»: no master states the count, control unfired) — dropped, its two leaves added by hand to the leaves set.

- `design1-2` («v1», the tool's pattern from design-system `contract/viola-list/columns` r1, whose «in v1» the new
  promise dropped) · standing 59 · leaf 25 · curation 1 — every row another sense (the product version v1, a
  `.v1.json` schema name, a matrix id): a true claim sharing the token, no change.
- `obs1-1` («events.ndjson», the tool's pattern from obs-plan `decision/link-names-never-in-process-log` r1) · standing
  69 · leaf 23 · curation 1 — every master row a true claim of the event log's other uses, the two `edited` rows this
  wrap's own (architecture `decision/revive-in-place`, `contract/snapshot-envelope/unreadable-snapshot`; obs-plan
  `contract/run-start-events/order`, `decision/lineage-names-confined`); leaf `.claude/docs/obs-summary.md:70` (link
  names in the event log and the snapshot) read by the leaf comparers; the rest true.
- `m-start` · new 1 (test-plan `contract/run-start-events/order`, this wrap's) · leaf `.claude/rules/verification-harness.md:29`
  stale («lines 1 and 2») → the leaf comparers.
- `m-replay` · leaf `.claude/rules/events.md:30` stale (the replay's three fields) → the leaf comparers.
- `m-linkonly` · design-system 3 and layout-templates 1, each this wrap's own edit (now naming the LINKS block); leaf
  `.claude/docs/design-summary.md:56` («no link column» — true; the comparer reads whether it owes the block).
- `m-order` · layout-templates `decision/board-ordered-by-name`, this wrap's edit.
- `m-board-links` · leaf `.claude/docs/services/viola-state.md:30` stale (no type holds links) → the leaf comparers.
- Added by hand (`leaves --add`): `.claude/docs/services/viola-core.md` (line 20, «`EventKind` holds 15 kinds») and
  `.claude/docs/services/viola-state.md` (line 45, «the 15 kinds») — the close's site sweep named both.
