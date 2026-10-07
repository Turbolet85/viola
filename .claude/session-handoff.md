# Session Handoff

**Last Updated:** 2026-10-07T08:37Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-07-test-homes-off-the-contended-volume — test homes off the contended volume, the dev host's homes on a keeper-checked tmpfs link, proved in a natural contended window

## Position
- Done: **2026-10-07-test-homes-off-the-contended-volume** (41 complete, 0 pending, 0 gated). It claimed no capability.
- Next: **Live rows and paste shapes on the dev host** (`working-route.md:96`) → `/andromeda-phase`.
  - Unattended, `claude` 2.1.287 by path, live-session cap 10.
  - **Open, the founder's:** what `send` does while the paste hint stands. `:96` times the hint again on the live
    CLI and brings the card; nothing about the gate changes before it.
  - Epoch 3 has 17 entries and stays one epoch (R-L2); `:98` First live test and self-drive is its last.

## Work done
- On the Linux dev host `target/e2e-home` is a link to an owner-only tmpfs directory under `/tmp`; two test-side
  keepers re-make and check its target; G2's walk descends a linked scope. No product crate changed.
- Proof, one natural window (08:08:52Z, another project's build): the control write stalled 10.999 s median on the
  shared volume, 0.285 s at most through the link, `binary(cli_verify)` 29 passed, 0 failed.
- CI green on the pre-CI commit `230f5dc` (ci#37592258366, 15/15). Record: the chunk's `report.md` and `evidence/`.

## Drift resolved
- 13 amendments in four masters (architecture 2, security-plan 1, test-plan 7, obs-plan 3), six leaves re-derived.
- The boundary widening (the test side creates and deletes outside the working directory through the link) is
  recorded as the founder's, live, 2026-10-07T07:25Z, relayed by the overseer; no halt was taken at the wrap.
- Record: `.andromeda/runs/2026-10-07T08-21-13-wrap/` (`fanout-results.md`, `cascade-dispositions.md`).

## Notes
- **Standing rule (the overseer, at this wrap):** a stalled-start red is a finding about the backing. Read
  `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/`, stop and report; never re-run for green.
- **After a `cargo clean`:** re-make the link, `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`; a reboot needs
  nothing (the keepers re-make the target). A kept home now lives in memory and does not survive a reboot.
- **Route:** `:98` carries the copied-tree question for a mutation run to the Epoch 3 boundary audit; `:96` carries
  a stale row list in test-plan's `--local-live` bullet (14 of 17 ids named).
- **For the operator's word** (carried):
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32` and `v1-40` cite `:90` for the real Windows terminal's mouse report
    (now `:140`), and `v1-31`'s cite `:90` for "the dialog never renders" (now `:98`); none was rewritten;
  - `architecture-amendments.md` is now 122 839 B, over the 120 000 B whole-read bound: phase reads it through
    its index until the Epoch 3 close consolidates it.
- **Curation:** three Tier 2 writes (`host-win32.md` new; `verification-harness.md` and `testing.md` extended).
- **Deferred learnings** (carried, plus one):
  - `recurrence-despite-learning: host-win32.md` Long single-line files (a cut-limited view answering a
    membership question);
  - `recurrence-despite-learning: host-win32.md 2026-09-28/29` (heredoc to a file; recurred this session);
  - `recurrence-despite-learning: host-win32.md` (`rm -rf` in a compound, refused; recurred this session);
  - `recurrence-despite-learning: host-win32.md 2026-09-25` (`pkill -f` self-match);
  - `session-learnings.md 2026-09-29` (gate.py hygiene reads `/home/<x>/` in prose);
  - `host-win32.md` (zero-is-healthy count probe);
  - `testing.md` (bounded mutant-reachable waits);
  - `host-win32.md 2026-09-28` (the Bash guard and a heredoc to a file);
  - the "not measured here" vocabulary;
  - PID 1 as the cleanup-deadline target;
  - the PTY master close needing no held clone;
  - let a red CI run finish before folding its fix;
  - the doubled-backslash guard recurrence;
  - `grep` on the Linux dev host is ugrep, and a counted-context `-o -E` extraction is refused as too complex;
  - new: a time written into an evidence file ahead of the clock (the stamp hook refused it; read `date -u`).
- **Operator desk (the founder's word: leave them):**
  - Run D's plan file under `~/.claude/plans/`;
  - the empty gitignored `.viola-verify-2095228/`;
  - `~/.viola-record-20261004T142325Z`;
  - the Run B/C/D transcripts;
  - yours to delete: `target/e2e-home.disk`, whole (the 13 entries, 545M, that were in `target/e2e-home` before
    the link, the kept test home `viola-test-WYNVH7` among them).
- **R-S3:** Upgrade U02 and the `host-win32.md` regenerate run at the Epoch 3 boundary (`/andromeda-setup-project`).
- **Last failed command:** none.
