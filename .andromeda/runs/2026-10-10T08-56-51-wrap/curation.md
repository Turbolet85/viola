# Curation — 2026-10-10-windows-mutation-grade

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + testing.md: "No test sends a kill at a process it did not start … the one standing case, `cleanup_waits_its_deadline_for_a_target_that_outlives_its_kill`, stays and is no precedent" (confidence 0.8), with a one-line pointer in testing-src.md
      Proof: the operator's wrap note 1 (`inputs#I6`), an explicit curation request (+0.5) in "never" language (+0.3); first given at the P5 review (`inputs#I3`), where it removed a planned kill at pid 4. The named case was read in source: `crates/viola-e2e/src/harness/cleanup.rs`, the `fn` at line 300 on this tree (the note cites 295, its line before this chunk), `cfg(target_os = "linux")`, `ProcessId::of(1)`.
    + verification-harness.md: "Forecast a `windows-mutants` job's wall from its count of viable mutants, never from its file's size …" (confidence 0.8)
      Proof: verified by measurement (+0.4), the plan's wall-time note falsified in run 38036448183: 26 m 11 s for the job forecast at about 52 min, and 41 to 49 min for three jobs forecast at 16 min or less (`evidence/windows-dispatch.md`, "The wall-time forecast"); a specific technical detail (+0.2): median test time 102 s and 95 s per caught mutant in `viola-run-env` and `viola-panic-frames`, 9 left-out mutants at 101 s each. No-other-home (+0.2): the fact stands in no master, playbook rule or route annotation.
  Tier 3 (.claude/docs/session-learnings.md): none
  Extended: T2/testing.md: "2026-09-24: Keep a `#[cfg(unix)]`-only function to a minimal OS reader …" + "the exception is a stub whose whole body would be `Ok(())` … (`viola_state::fs::restrict`)" (confidence 0.9)
      Proof: owed by the plan (Implementation notes, "Owed at the wrap, curation"; the operator's note at the P4 fork, `inputs#I2`, +0.5); verified by measurement (+0.4): `cargo mutants --list` on 27.1.0 over the final tree holds one `restrict` line, the Unix function's at `fs.rs:19:5`, and the mutant `fs.rs:18:5` that the Epoch 3 audit graded missed on Windows is no longer generated (`evidence/survivors.md`, row 16).
  Corrected (exempt from the cap):
    ~ verification-harness.md, the 2026-09-24 entry's 2026-10-04 correction: the jobs are `mutants (<label>)`, nine of them.
      Proof: the workflow's job name is `mutants (${{ matrix.label }})` over nine items (the report, Counts); found by the cascade sweep (`cascade-dispositions.md`, the curation row at `verification-harness.md:56`).
    ~ ci.md, the 2026-10-04 entry "A dispatched run joins its sha's checks": nine `mutants (…)` checks, 15 → 24.
      Proof: entry 26 read `verdict: green · checks 9/24` on `dd5161d55743` (`evidence/windows-dispatch.md`).
  Filters: 2 dup · 0 task-specific · 0 conflict · 0 deferred · 3 below the threshold
    dup: "cargo-mutants' summary line counts a host-excluded mutant as missed; judge by the harness document" (now in verification-harness.md's generated body, written by this wrap's cascade) · "syn and proc-macro2 in viola-e2e" (stack.md, the same cascade).
    below the threshold: "a caught mutant at 10 s of test time cannot be told from a kill-line stop in a job log" (one mention, 0.2) · "a wrap note's line number may be the line before the chunk" (one mention, 0.2) · "an exception to a founder ruling is a dated sentence beside it, never a rewording" (a direction for this chunk, one mention, 0.4; the operator's auto-memory already holds the standing form).
    recurrence-despite-learning (to the handoff): `ci.md` 2026-10-09, `gh run list --commit` with a short sha, met again at this session's start.
  No-other-home: "Forecast a `windows-mutants` job's wall from its count of viable mutants …"
  CLAUDE.md size: read at P7 from `health.py`.
