# Curation — 2026-11-13-epoch-8-cleanup wrap

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A `gate.py control` clone holds no build output … fired on a workspace-building harness layer (`run --unit`, `run --e2e`), or it reads UNREPRODUCED" · + testing.md: "Two more shared helpers stand in `tests/support/` … `source_scan.rs` … `private_dir`"
  Tier 3 (.claude/docs/session-learnings.md): extended «A live start cap is the current chunk's own word» + "a print-and-exit read is no live start"
  Filters: 2 dup · 1 task-specific · 0 conflict (→ handoff) · 3 deferred (→ handoff)
  No-other-home: "Two more shared helpers stand in `tests/support/`" (fit rejected the master row: not_master type 5)
  Extended: T3/session-learnings.md: "A live start cap is the current chunk's own word, never a carried figure" + "a print-and-exit read (`--version`, `--help`) is no live start"
  CLAUDE.md size: 132/200 · T1 3.4 KB, 0 over 600 B

## Applied
1. Tier 3 extension — the founder's own word (inputs#I5 item 3, «Не считается», 2026-11-13): a print-and-exit read is
   no live start and counts against no live-start cap. Confidence 1.0 (explicit request +0.5, never-language +0.3,
   specific detail +0.2).
   Proof: inputs#I5 item 3 (the directive, `../additional/viola-overseer/wrap-epoch-8-cleanup-directive.md`); it
   clears the handoff's «Curation conflict — user review» (`mise ls claude` against `claude --version`): both stand.
2. Tier 2 testing.md, new entry — the control clone builds nothing outside the entry's package. Confidence 0.8
   (measurement +0.4, specific +0.2, no-other-home +0.2). Written first as an extension of the 2026-09-25
   remove-the-guard entry, which then read over the 1.5 KB cap (health check 4: `1 over 1.5 KB`); moved to an entry
   of its own (tx 956), the check re-read `0 over 1.5 KB`.
   Proof: `{implement run}/evidence` — control-1.txt `control: 4 holds · 0 timeout · 0 does not hold · 21 unreproduced`
   (in place: `Unable to spawn …/target/debug/viola-fake-agent because it doesn't exist on the filesystem`), then
   control-2.txt `control: 21 holds · 0 timeout · 0 does not hold · 0 unreproduced` on entries 12 and 15.
3. Tier 2 testing.md, new entry — the two new shared helpers. Confidence 0.8 (measurement +0.4: the gate entries 16
   and 19 and the census binaries 84/84; specific +0.2; no-other-home +0.2: P2's fit rejected the test-plan row,
   `not_master` type 5).
   Proof: `test-plan-fit.md` n=4 `v=not_master type=5`; `fanout-results.md` check 5 note.

## Decisions & corrections, each weighed
- The founder's word (print-and-exit read) — applied (1).
- The implement directive (no live start, no mutation run, 900 s controls, journal, operator pass, stop) — not a
  learning: a chunk-scoped directive, carried in the report.
- `gate.py control` builds what its entry builds — applied (2).
- tempfile `persist_noclobber` is `renameat2(RENAME_NOREPLACE)`; a present-name guard needs an mtime witness —
  → handoff (deferred, Filter 5 cap; confidence 0.8).
- Hand-written masked-text oracles miscounted — rejected (Filter 4: one-off, my own literals, −0.3).
- `rec.py find --text` is case-sensitive (1 row vs 4) — → handoff (deferred, Filter 5 cap; confidence 0.8; a
  pipeline tool's behaviour).
- The OS census walks only top-level test files — rejected (Filter 1, duplicate: this wrap landed it in test-plan
  `decision/os-gated-cases-in-binary-root` and the cascade wrote it into testing.md's generated body).
- Guard and hook refusals at implement — not a learning: the stamp-ahead hook refused an estimated time and the
  bash guard four calls, each at the cost of its one re-issue; the hook held the Tier-1 timestamp learning, so it is
  no recurrence.
- Hygiene's host temp path in two run-dir files — rejected (Filter 1, duplicate: the `$TMPDIR` printed-form rule
  stands in the gate contract and host-linux.md's 2026-10-10 entry).
- `fn digit_runs` CARRY (inputs#I4, inputs#I5 item 1) — not a learning: a route annotation (P5).
- The library-test scoped subscriber (P2 rejected from obs-plan as curation's) — → handoff (deferred, Filter 5 cap;
  confidence 0.8).
- `rec.py diff` labelled a rule set `retired` (tx 949) and the census's fit and pick counts — not learnings: tool
  telemetry, in the evolve records.
