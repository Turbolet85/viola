
## 2026-10-14-resume-and-compact-ledger-rows — Run E in the verify child set; a third session-id source
**Section:** §Threat Model Summary (the `revive` and `viola verify` entry points); §Input Validation (Session id as a child argv value; Constants `MAX_FRAME`); §Data Protection (Probe captures; Interactive probe dirs; Accepted risk (verify's trusted runs, B, C, D and E)); §Security Anti-Patterns (Universal, the stamp-gated dialog decision)
**Change:**
- verify spawns five interactive children (was four). Run E runs last:
  - in `<cwd>/.viola-verify-<pid>-resume/` (0700, its guard);
  - with the run args, `--plugin-dir` and the `resume/` capture plugin, then `--resume <id>` from `resume_args(id, false)`;
  - with no `--settings` and empty input, so no key is typed;
  - it waits for one SessionStart up to `PROBE_DEADLINE`, settles, and is killed.
- An uninterrupted verify writes seven spawn pairs.
- The Session id row is retitled (`viola revive`, `viola verify`'s Run E) and gains a third id source:
  - the `session_id` of Run B's first SessionStart capture, through `ledger::resume_target`, only when `is_session_id` accepts it;
  - with none, Run E spawns nothing and `resume-session` fails;
  - `resume_args` gains its second caller.
- `MAX_FRAME`'s child output covers runs A to E.
- Probe captures gain the `resume/` root, and the interactive probe dirs gain `-resume/`.
- The accepted risk names Run E. It adds no transcript file, as measured on 2.1.287: `~/.claude/projects/` 643 → 647 → 651 over two rounds, four per round.
- The ledger has twenty-two rows (with `resume-session`). The counter is `/22` on Unix and `/19` on Windows.
- An all-`pass` 18- or 21-id older stamp reads unverified on every OS. The failing example now reads `18 pass  4 fail`.
**Why:**
- Run E and its id source widen the ratified verify child set and the session-id crossing.
- They are ratified by the founder's own answer to P4 fork 2, relayed verbatim by the operator (`inputs#I1`).
- The wrap's card confirm of the proposals inside it is PROVISIONAL (delegate viola-overseer, `inputs#I6`), superseded by his own word at the Epoch 5 boundary.
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/
