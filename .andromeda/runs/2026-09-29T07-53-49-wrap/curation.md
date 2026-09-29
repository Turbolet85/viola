# Curation — 2026-09-29-verify-stamped-test-homes-and-harness

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   extended host-win32.md (see Extended)
  Tier 3 (.claude/docs/session-learnings.md): + "A gate's freshness check must name a file, never a directory" (0.8)
                                              + "gate.py hygiene reads /home/<x>/ in prose as a POSIX user home" (0.8)
  Filters: 1 dup-as-recurrence (→ handoff) · 2 task-specific · 0 conflict · 1 deferred (→ handoff) · 1 homed in a master (0.6)
  No-other-home: "gate.py hygiene reads /home/<x>/ in prose as a POSIX user home"
  Extended: T2/host-win32.md: "The Bash guard refuses a heredoc whose payload carries a doubled backslash…" + "it refuses ANY command carrying one — a sed expression, an inline regex, a JSON record"
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B
```

## Applied
- **Tier 3: an `artifact` key names a file, never a directory.** Signals: explicit operator correction (+0.4) and
  verified by measurement (+0.4) = 0.8. No `paths:`-scoped rule file covers plan authoring, so Tier 3.
  Proof: plan gate 6 read `artifact STALE` on 3 of 3 runs (3466 s, 3312 s and 113 s older) while its JUnit files
  were written inside the entry window (09:33:54 / 09:34:20 and 09:38:13 / 09:38:40 +0200;
  `target/run-archive/{352,359}`). Chunk `evidence/entry-6-freshness-plan-defect.md` and
  `evidence/item8-witness-guard.md`. The operator: "a plan-target defect … retarget the freshness check to a file
  the run writes, not a directory".
- **Tier 3: hygiene's POSIX `/home/` form fires on repo-relative kept-home paths.** Signals: verified by a real
  gate failure (+0.4) and specific technical detail (+0.2) = 0.6 exactly, plus no other home this wrap (+0.2) =
  0.8. Its home is the evidence prose that no rule file scopes, so Tier 3.
  Proof: the operator-pass hygiene read `refused 1 files — P1 1` on `evidence/operator-pass.md:14 ×1 · home`. The
  line spelled the kept test homes' builder `detail-run.ndjson` as one slash path running through the home
  directory into `instances`; reworded by its parts, it read `clean`.
- **Extended T2/host-win32.md (the Bash-guard entry), an additive facet.** The entry named heredocs; the guard
  refuses any command. Signals: verified by measurement (+0.4) and repeated 3 events (+0.3) = 0.7. host-win32.md
  has no `paths:`, so it is judged by Tier 1's one-sentence and ~600 B bar, and the extension is one sentence.
  Proof: three refusals this session, "Blocked: this command carries a doubled backslash": a multi-file `sed` with
  an escaped-newline literal (implement P1, `tests/run_cli.rs`), the code-step evolve record's note (a heredoc),
  and a `ctx.py` regex argument (operator pass). Each was re-done through the Edit tool, a reworded record, or a
  Write-tool script using `chr(92)`.

## Filtered
- Recurrence, not dropped: the evolve-record refusal is the literal case host-win32.md:97 already states (a heredoc
  with a doubled backslash). → handoff Deferred learnings as `recurrence-despite-learning`.
- Homed in a master: "a stopped wrapper counts as gone only once its endpoint refuses a client". It scored 0.6
  (measurement +0.4, detail +0.2), and this wrap amended it into test-plan §3 `run` step 2, so neither
  conditional signal applies. Rejected.
- Task-specific: the multi-line `run_with(` / `, &mut` sweep pattern (one refactor's grep); "a served `Server`
  keeps its Windows pipe after `Serving` drops" (0.2, read from code, not measured).
- Deferred (the max-3 cap): the hygiene-disposition convention the operator ruled. Uncited copies are deleted;
  cited files get path-root placeholders with line counts kept; a plane-source control is renamed `.txt`. 0.7.
  → handoff.
