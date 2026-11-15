# Curation — 2026-11-15-mutation-and-coverage-baselines-green (wrap 2026-11-15T20-06-27-wrap)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   corrected testing.md 2026-09-24 (extended 2026-10-06): «the `mutants` profile's 10 s kill» → 15 s · corrected verification-harness.md 2026-10-04: a test socket binds under a short `/tmp` root, never under `TMPDIR`
  Tier 3 (.claude/docs/session-learnings.md): + "A «no product source changed» probe counts non-test hunks, not files"
  Filters: 0 dup · 1 task-specific · 0 conflict · 0 deferred · 3 under threshold (each exactly 0.6)
  No-other-home: "A «no product source changed» probe counts non-test hunks, not files"
  CLAUDE.md size: 133/200 · T1 3.4 KB, 0 over 600 B

## Applied
- Correction, Tier 2 `testing.md` 2026-09-24 entry, its Extended 2026-10-06 clause (cap-exempt; tx 1615).
  Proof: `.config/nextest.toml` `[profile.mutants]` `slow-timeout = { period = "5s", terminate-after = 3 }` (gate entry 9
  green); the report's Counts / qualifiers moved; test-plan `decision/mutants-kill-below-cargo-mutants-timeout` landed
  15 s at tx 1610.
- Correction, Tier 2 `verification-harness.md` 2026-10-04 entry (cap-exempt; tx 1616).
  Proof: the report's Spec claims disproved — `git grep -n 'viola-test-chan-' -- '*.rs'`: 8 hits, every one now under a
  short `/tmp` root; the cold baseline's `sun_path` red under the 52 B scratch `TMPDIR` (13.log) and its green re-fire
  (13.2.log, 3286 passed).
- Tier 3, new entry (tx 1617). Score: verified by a real gate failure +0.4 (entry 15 red on `last: '6'` while every
  hunk sat in `#[cfg(test)]`) · specific technical detail +0.2 · reached no other durable home +0.2 (no master row, no
  route annotation, no rule or detector, no hook) = 0.8.
  Proof: `evidence/fold-test-modules.md` — the hunk probe `outside 0` in place and `outside 1` with a planted non-test
  line.

## Every *Decisions & corrections* entry, its disposition
Implement's:
- The operator's word «Fold it in now (widening)» (inputs#I4) — not a learning: a chunk decision, carried by the
  report's scope record and the architecture row decision/test-sockets-under-short-tmp-root.
- Sweep hazard: a sun_path fix keyed on one casualty misses the class — rejected, Filter 4: 0.6 (measurement +0.4,
  detail +0.2; the master now holds the rule, so no-other-home does not fire).
- Sweep hazard: a stale-bound sweep over `crates/*/src/` test modules reaches product files — rejected, Filter 4: 0.6
  (measurement +0.4, detail +0.2; no conditional signal: «API-error turn end» does not need it).
- A file-level product-change probe cannot tell a `#[cfg(test)]` edit from a product line — applied, Tier 3 (above).
- Guard refusals (`cd` into a references dir; a `cat` heredoc to a file) — not a learning: each cost its one
  re-issue (`guard-refused`), the guard already refuses the act.
- Host load 18–164 through every timing reading — rejected, Filter 2: task-specific (a host reading).
After implement's:
- The P5 card's `vs-*` hygiene row held on the delegate's word (inputs#I5) — not a learning: a ruling on one row.
- `inputs.py snap` refused a `why` carrying the glob — rejected, Filter 4: 0.6 (a tool refusal +0.4, detail +0.2; the
  tool refuses the act itself).
- The wrap directive (inputs#I6) — not a learning: this wrap's instructions, carried by the report.
This window:
- The leaf comparer silent on `gotchas.md` (grouped under architecture alone) — not curated: a pipeline gap, recorded
  as evolve friction (`contract.cascade-miss`).
