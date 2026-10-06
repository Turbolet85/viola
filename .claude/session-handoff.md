# Session Handoff

**Last Updated:** 2026-10-06T22:06Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-06-local-command-and-paste-framing-rows — the wrap commit (three ledger rows by typed probe, seventeen rows stamped on 2.1.287)

## Position
- Done: **2026-10-06-local-command-and-paste-framing-rows**:
  - the ledger holds seventeen rows: `long-paste-wrapper`, `tag-escaping`, `local-command-clear` landed by typed probe;
  - the unwrap removes the frame the CLI writes around a long paste, so a wrapped `send` is claimed;
  - verify's Run B pastes four compiled texts and waits for the input box after each added turn;
  - 2.1.287 is the one stamped set; 2.1.288 is drift-only; PATH `claude` is 2.1.289, unstamped.
- Next: **Local-command send outcomes** (`working-route.md:92`, minted at this wrap) → `/andromeda-phase`.
  - It carries three CARRYs (send's `unconfirmable` and `/clear` confirmation with `v1-29`; the readiness-gate
    reading; the missing guard control) and one PREREQ (two "seven 300 ms settles" comments).
  - Then **First live test and self-drive** (`:94`), the last Epoch 3 entry.
  - Epoch 3 has 15 entries and stays one epoch (the founder's ruling).
  - Live sessions: this chunk's cap of 12 is spent and no standing cap is left; they are the founder's to grant.

## Work done
- Three implement runs: step 0's STOP 3, a red first record round, then a rehearsal with no STOP and a green
  second round (`stamped 2.1.287  17 pass  0 fail`). The operator pass made `05b5f2b`; CI ci#37534758441 is green
  15/15, wall 481 s.
- Wrap record: `.andromeda/runs/2026-10-06T21-43-53-wrap/`.

## Drift resolved
- The fan-out returned 52 proposals; all applied, with 1 orchestrator-raised amendment; 1 escalation resolved:
  - the Run B widening (four pastes, the wait, the second transcript) is recorded as ratified by the founder's
    live answers of 2026-10-06T19:38Z and 20:49Z, confirmed by the operator at the wrap;
  - architecture ×24, test-plan ×19, layout-templates ×5, security-plan ×3, obs-plan ×1, design-system ×1.
- Cascade: 5 key files and 9 leaves re-derived (CLAUDE.md's ledger line, two rule files, six docs).

## Notes
- **Curation:**
  - a new testing.md entry: a live shape probe's record states how long each settle took;
  - the verification-harness.md 2026-09-29 entry extended: a common stall across process-spawning tests is a
    host stall.
- **Host contention (the operator's disposition: environment, relayed to overseer1, its F167):** two gate
  entries read red once while another session's build linked on the same disk; no bound or test moved.
- **Deferred learnings** (carried, plus one):
  - `recurrence-despite-learning: host-win32.md` (`rm -rf` in a compound, refused; new at this wrap);
  - `recurrence-despite-learning: host-win32.md 2026-09-25` (`pkill -f` self-match);
  - `recurrence-despite-learning: host-win32.md 2026-09-28/29` (heredoc to a file; recurred at this wrap);
  - `session-learnings.md 2026-09-29` (gate.py hygiene reads `/home/<x>/` in prose);
  - `host-win32.md` (zero-is-healthy count probe);
  - `testing.md` (bounded mutant-reachable waits);
  - `host-win32.md 2026-09-28` (the Bash guard and a heredoc to a file);
  - the "not measured here" vocabulary;
  - PID 1 as the cleanup-deadline target;
  - the PTY master close needing no held clone;
  - let a red CI run finish before folding its fix;
  - the doubled-backslash guard recurrence.
- **Operator desk (the founder's word: leave them):**
  - Run D's plan file under `~/.claude/plans/`;
  - the empty gitignored `.viola-verify-2095228/`;
  - `~/.viola-record-20261004T142325Z`;
  - the Run B/C/D transcripts;
  - new, yours to delete: `target/e2e-home/viola-test-WYNVH7` (a test home kept for a red reading).
- **R-S3:** Upgrade U02 and the `host-win32.md` regenerate run at the Epoch 3 boundary (`/andromeda-setup-project`).
- **Last failed command:** none.
