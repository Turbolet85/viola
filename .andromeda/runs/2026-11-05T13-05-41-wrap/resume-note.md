# Resume note — wrap of 2026-11-05-linux-only-code-and-ci (window 3, 2026-11-06)

P2 is COMPLETE (drift = 0 reached in window 3). Resume per Setup 2a at **P3 curation**; nothing is committed.

## Done in window 2 (2026-11-06)
- P1 resume reads, the re-fan, every batch through apply-p3, Q1–Q3, masters TXs 242–338 (see the earlier note's list,
  kept in `fanout-results.md` §Apply rulings).

## Done in window 3 (2026-11-06)
- Cascade step 2: the sweep re-fired (same 30 pattern files; `total (551 patterns) · 655 rows over 62 files`);
  `cascade-dispositions.md` written; routine fixes `edits-main-7.md` (TX 339; two replacement facts, originals retired
  TX 340–341).
- Entry writers (`--batch entry`, 7/7 pass), 84 entries landed TX 342–348, read back.
- Leaves, WHOLE mode (Epoch 7 closes): `--batch leaf` 16/16 pass; 289 findings applied by `cascade.py leaves --apply`,
  4 declined and 3 tool-refused rows written by hand (TX 374–379); `## Leaves` in cascade-dispositions.md.
- Remarks + Q4 (handle inheritance, the overseer's technical answer 2026-11-06 16:25Z): `edits-main-8.md` (TX 380).
- Residual census (cascade miss): `edits-main-9.md` (TX 381), fct-ahjpxago retired (TX 382), 8 leaf lines by hand
  (TX 383–390); main's entries TX 391–394, read back. §Residual census in cascade-dispositions.md.
- P2 evolve checkpoint landed (TX 395–396). The two `evolve-reconcile-*.py` scripts in this run dir move to the session
  scratchpad before P7's hygiene read (their records already landed).

## Owed to P3 (curation rows, preserve-verbatim homes)
- The sweep's 26 curation rows (cascade-dispositions.md §Curation rows) and the residual census's: `rules/testing.md:56`,
  `:70`; `docs/session-learnings.md` :50–55, :175.

## Owed to P5 (CARRYs on «Version done-check»)
- The `thiserror::Error` census family (architecture fct-gctodeyz, conventions fct-nhsg4k7g, obs error-capture «9»;
  the report read 10; rul-in3bj4pn).
- obs-plan fct-yb5jpaot (`process-start` row): target text `pty_backend` (`openpty`) alone, `sideload_fallback` gone (Q3).
- Q4: measure the no-leak property on Linux — under the fake agent the wrapped child's `/proc/<child pid>/fd` holds no
  channel socket or listener; then re-word fct-p7rzex2i off «recorded, not established».
- Code remnants (cascade-dispositions.md §Code remnants): `tests/cli_output_plain.rs:104,116`, `viola-agent-claude/src/lib.rs:2`,
  `viola-e2e` `session.rs:52` and the `EXE_SUFFIX` joins; also `COVERAGE_FLOORS` «per OS» and the `COVERAGE_IGNORE`
  Windows-path comment; `plain_drive` (fct-k6htnrfj) and `host.rs`'s `windows` key still in the tree.
- The P5 card's rows already on the report: the RUSTSEC-2026-0441 CARRY and `v1-01`'s owner.

## Pipeline friction already logged (TX 396)
- key-file targets not handed to apply writers; line removal inside a fact / fence; `|` in a code span in a table; the
  `[&str; 3]` entity read; the sweep's output over the cap; the cascade miss; 4 declined leaf findings.
