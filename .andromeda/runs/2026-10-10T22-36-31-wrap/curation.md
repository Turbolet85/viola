# Curation — wrap of 2026-10-10-budget-governor

Scope: this wrap's conversation (the resume from Phase 2), the report's Decisions & corrections, and the candidates
the earlier session of this wrap wrote into `resume-point.md` before its conversation was cleared. The first
implement run's conversation was gone before either; what only it held is not curated beyond those candidates.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A missing test for a clause a chunk landed is folded into the next chunk, never pinned on a later route entry." (confidence 0.7)
  Tier 2 (.claude/rules/*):                   + testing.md: "`JsonFields` records an `f64` log field as its Debug text … give that case a visitor of its own with `record_f64`" (confidence 0.8)
                                              + ci.md: "In the operator pass, read the Windows-target lint on the host before the push …" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 0 task-specific · 0 conflict · 3 below threshold · 0 deferred by the cap
  No-other-home: "`JsonFields` records an `f64` log field as its Debug text" · "read the Windows-target lint on the host before the push"
  CLAUDE.md size: 127/200 · T1 2.3 KB, 0 over 600 B
```

## Applied

1. Tier 1, `CLAUDE.md`: a missing test is folded into the next chunk.
   Proof: the operator's answer in this wrap's dialog (`inputs#I11`, question 3), which set aside the recommended
   owner ("The board: viola list") for the head, in these words: "A missing test is folded into the next chunk; the
   booted pair of The board is not needed for it, a closed gate in cli_budget is enough." Signals: an explicit
   correction (+0.4), rule language (+0.3). The entry keeps his first clause; "never pinned on a later route entry"
   is the wrap's wording of what the answer did.
2. Tier 2, `rules/testing.md`: the shared test visitor turns an `f64` field into text.
   Proof: one red run in the first implement run, and the local visitor it led to (`NumberFields`,
   `src/run/send.rs` 2022-2041; report, Deviations 4). Signals: proven by a real gate failure (+0.4), a specific
   technical detail (+0.2), no other durable home (+0.2: in no master, on no route entry).
3. Tier 2, `rules/ci.md`: the Windows-target lint is read before the push.
   Proof: this chunk's first push read green after the lint was read on the host (`evidence/operator-pass.md`); the
   previous chunk's first push read red on `lint` and `test` on `windows-2025` (`ci#38080061631`) for what that
   lint sees. Signals: verified by measurement (+0.4), a specific technical detail (+0.2), no other durable home
   (+0.2: `testing.md` holds the command, not its place in the pass).

## Filtered

- dup (carried by a master this wrap amended, and by the leaf re-derived from it): a guard test for a strict-modes
  check on the home reads the mode while the process still runs (test-plan §6, the exit-cause list); a reader's
  unit cases may plant a malformed state file only by altering the writer's own bytes (test-plan §11,
  `rules/testing.md`'s body).
- below threshold (one signal each, a specific technical detail): `pre-push` prints no coverage figure and the three
  numbers are in `target/agent-run/artifacts/llvm-cov-summary.json`; the CI read outlives the Bash tool's
  foreground bound and goes in the background; a foreground `sleep` is refused by the harness (the harness itself
  refuses the act).
- not a curation candidate, written to the operator's memory instead: the stamp-ahead hook's new cases (rstest
  literals with the day's date; an example calendar date in a master), an extension of the existing memory.
- for the playbook, appended at Phase 2 on the operator's word: "Held items landed on the operator's word".

## Recurrences despite a standing learning (for the handoff's list)

- `host-linux.md` Paths: a `cd` outside a subshell opened most Bash calls of this wrap (the cwd was already the
  repository root, so no guard refused).
- `host-linux.md` Transports: one inline python heredoc for an edit of a leaf (`docs/stack.md`).
- the handoff's own deferred learning on long single-line files: multi-KB master lines were read through a scratch
  script again; `cascade.py window` wraps its output and cannot feed the Edit tool an exact string.
- from the earlier session's list: `host-linux.md 2026-09-28/29` (a `cat` heredoc with a file target, once).
- a time written into a record without a read of the clock (`inputs#I11`'s origin text).
