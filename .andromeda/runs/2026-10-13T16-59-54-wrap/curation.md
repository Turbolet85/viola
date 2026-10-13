CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "a `+=` → `-=` mutant on an unsigned sum from zero reads red by its overflow panic"
                                              corrected in place: verification-harness.md 2026-09-27 (the filter's full path)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 1 rejected at 0.6
  CLAUDE.md size: 128/200 · T1 2.6 KB, 0 over 600 B

## Applied
- **Correction** — `.claude/rules/verification-harness.md` Session Additions 2026-09-27, edited in place with
  `[corrected 2026-10-13: …]`. It was "an inline unit test is `tests::name` … write `test(=tests::name)`"; now
  `<module>::tests::name`, the regex form `test(/(^|::)tests::name$/)`, and a red counts only with the case's own panic
  line. Exempt from the cap; directed by the operator (`inputs#I4`: "Amend the three leaves your report names").
  Proof: implement `journal.md` (the 16:14Z entry) — 19 of 29 hand-applied mutant specs whose filter was
  `test(=tests::name)` read `nextest-exit-4` for both runs; re-run under the regex form, all 19 read red/green (report
  **Spec claims disproved**).
- **Tier 2, testing.md** — the `-=` overflow kill form. Score: verified by measurement +0.4 (a real kill read this
  way) · specific technical detail +0.2 · no other durable home +0.2 (in no master, route annotation, ledger note or
  base; the report and `survivors.md` are not homes) = 0.8 > 0.6. Routed by class: test authoring → `testing.md`.
  Proof: `evidence/survivors.md`, row `board.rs:164:29` `+=` → `-=` — red `attempt to subtract with overflow` at
  `board.rs:164:9`, green restored.

## Decisions & corrections — each entry's disposition
- The implement run's direction (no live claude, no mutation run, hand-applied mutants, file-redirected tool output,
  rustfmt in the same call, journaled waits, the operator pass) — not a learning: the run's own direction, already
  held by the plan and `inputs#I1`.
- The operator's wrap word (`inputs#I4`): the mint → P5; the three leaves → two re-derived by the cascade
  (`gotchas.md:111`, `services/viola-pty.md:33`), the third corrected here; the Bash guard sentence → not a learning:
  the guard now refuses the act itself and its message names the form (`host-linux.md` "Exit codes" already says
  redirect and read the file; the user memory holds it).
- The filter sweep hazard → **corrected** in place (above).
- The `-=` overflow kill form → **applied**, testing.md (above).
- A relative path between two Windows drives → **rejected at 0.6**: verified by measurement, as the fact changed the
  design (+0.4), plus specific detail (+0.2); the no-other-home signal does not apply, because this wrap amended the
  fact into architecture §Occupied Resources → Repository (`target/viola-answers-*/`).
- The clock-stamp recurrence → **→ handoff** as `recurrence-despite-learning: a time written into a record ahead of
  the clock` (carried; the stamp-ahead hook caught it at its one re-issue).
- Hand-applied mutant discipline (byte copy, restore, confirm) → not a new learning: testing.md 2026-09-25 (the
  remove-the-guard run) carries it; the one new facet (a red needs the case's panic line) rides the correction.
