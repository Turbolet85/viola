# Curation — 2026-10-20-epoch-5-cleanup

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "On Unix a PTY child dies with its master — … witness such a leak on Windows" (confidence 0.8)
                                              + testing.md: "A census for a test's leaked children keys on the test binary's file name, never its build dir" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none new
  Filters: 0 dup · 1 task-specific · 0 conflict · 0 deferred · 2 below threshold
  No-other-home: "On Unix a PTY child dies with its master …"; "A census for a test's leaked children keys on the binary's name …"; the page-cut facet
  Extended: T3/session-learnings.md: "Naming the process behind a truncated coverage profile" + "a cut on a 4 KiB page boundary inside the data records names no writer; the per-run profile count is a reading"
  CLAUDE.md size: 128/200 · T1 3.0 KB, 0 over 600 B
```

## Applied
- **testing.md — the Linux PTY hangup.** Score 0.8: verified by measurement +0.4, specific technical detail +0.2, no
  other home +0.2 (no master was amended with it; the plan's step 7 premise it disproves lives in the chunk folder).
  - Proof: `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/viola-pty-leak.md` — census `none running` after
    the old-guard and the new-guard runs under the hand-applied `try_wait -> Ok(Some(1))`; the reason is pinned by
    `tests::close_hangs_up_a_live_child`.
- **testing.md — the census key.** Score 0.8: verified by measurement +0.4 (the key choice changed the reading's
  design), specific technical detail +0.2, no other home +0.2.
  - Proof: the same evidence file. Both clones built `viola_pty-77dd4bdbf6cc1891`, and the new guard's children run
    from a tempdir copy, which a `--args target/e5c-pty-new` census never matches.
- **Extended T3 «Naming the process behind a truncated coverage profile».** Score 0.8: verified by measurement +0.4,
  specific technical detail +0.2, no other home +0.2.
  - Proof: `evidence/profraw-cause.md`. 11 of 17 refused files and P5's file end on a 4 KiB page; 8 of 17 are cut
    inside the data records with no counter; 726 profiles before the fix against 800 after over the same 200 runs.

## Each *Decisions & corrections* entry
1. The operator's direction for implement — not a learning: a one-off run direction. Its `/tmp` reading is already a
   standing handoff note.
2. The wrap directive (inputs#I4) — not a learning: this wrap's own inputs.
3. A Linux PTY child dies with its master — applied (testing.md).
4. A cut-short profile ends on a 4 KiB page boundary — extended (T3), folded with entry 5.
5. A census's profile count is a reading too — extended (T3), with entry 4.
6. The fake agent's hooks leave the foreground process group — rejected, Filter 4 at 0.6: measurement +0.4 and
   detail +0.2, with no conditional signal, since its home is test-plan §7, amended this wrap.
7. Sweep hazard, a census keyed on a target dir — applied (testing.md).
8. Sweep hazard, the run-dir spelling of a scratchpad path — rejected, Filter 2: the pipeline's own run-dir spelling
   rule states it, and hygiene's P1 reads it.
9. Guard refusal, a `cd` into a scratch clone — not curated: host-linux.md §Paths states it, and the guard held that
   rule at its one re-issue (guard-refused), so it is no recurrence.
10. Clone builds off the quota'd tmpfs — rejected, Filter 4 at 0.4: detail +0.2 and a decision, no measurement this
    chunk.
11. The `mutants` profile's walls after the fix — not a learning: a run's readings (telemetry), carried by the report
    and test-plan's key file.
