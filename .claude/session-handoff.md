# Session Handoff

**Last Updated:** 2026-10-07T13:15Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-07-send-waits-out-the-paste-hint — send waits out the paste hint, the gate waits for the input box to an 8.5 s bound, send reads the wheel and the turn again after the wait, 2.1.287 re-verified by path

## Position
- Done: **2026-10-07-send-waits-out-the-paste-hint** (43 complete, 0 pending, 0 gated). It claimed no capability.
- Next: **A send ending in a newline is confirmed** (`working-route.md:100`) → `/andromeda-phase`.
  - Then **First live test and self-drive** (`:102`), the last Epoch 3 entry. Epoch 3 has 19 entries and stays one
    epoch (R-L2).

## Work done
- On a verified CLI the readiness gate waits for the input box on a quiet screen with no literal, to an 8.5 s
  bound (`GATE_MAX_WAIT`). A `send` under the paste hint is delivered once the input box returns. After the wait
  `send` reads the wheel and the running turn again and refuses with nothing typed.
- One live round, fired once: `claude` 2.1.287 by path, `stamped 2.1.287  17 pass  0 fail`, settles 1 103 ms and
  617 ms against the 8 500 ms bound. Five starts, the whole of the cap, each ledgered before the fire.
- CI green on the pre-CI commit `e574e73` (ci#37623727247, 15/15). Record: the chunk's `report.md` and `evidence/`.

## Drift resolved
- 21 amendments in four masters and two key files (architecture 9, test-plan 7, a11y-plan 3, obs-plan 2), five
  leaves re-derived. No escalation; no card of the founder's arose.
- One route card: a CARRY on "First live test and self-drive" (`:102`).
- Record: `.andromeda/runs/2026-10-07T12-57-41-wrap/` (`fanout-results.md`, `cascade-dispositions.md`,
  `directive.md`).

## Notes
- **For the founder** (he was away for this chunk; nothing here was answered in his place):
  - open: the `input-not-ready` hint line's wording. It is unchanged and still advises `viola wait`; it is no
    longer printed for the paste-hint cause and stays for a modal, a screen that never goes quiet, a poisoned
    screen and a hint past the bound. A rewording is his.
  - a fact, the overseer's technical answer of 2026-10-07T12:10Z, not his ruling: after the gate's wait `send`
    reads the wheel and the running turn again. The masters record it so.
  - `send`'s longest block is now 18.5 s (he was told so on the hint card).
- **Unmeasured:** a `send` under the real CLI's paste hint since the build. Delivery after the wait is measured
  under the fake agent's hold only. It is pinned on `:102`.
- **A limit, stated in the report and in test-plan §6:** the end-to-end keystroke case cannot tell `send`'s first
  wheel read from its second; the second read's proof is the unit cases and their remove-the-guard controls.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s two rows, the harness-prefix row
  and the R8 identity-floor row. A probed row for either needs its own founder ruling.
- **Known and unfixed until `:100`:** a `send` whose text ends in a newline is delivered, reported
  `not-delivered`, and takes the wheel from the driver.
- **Standing rule (the overseer):** a removal the permission layer denies is not done through another tool; move
  the dir into `target/e2e-home.disk/` and record it.
- **Standing rule (carried):** a stalled-start red is a finding about the backing. Read
  `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/`, stop and report; never re-run for green.
- **After a `cargo clean`:** re-make the link, `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
- **Route:** `:102` carries the copied-tree question for a mutation run to the Epoch 3 boundary audit;
  `BLOCKED-ON` at `:144` stands (no interactive Windows host; the dev host is Linux).
- **For the operator's word** (carried):
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32` and `v1-40` cite `:90` for the real Windows terminal's mouse report
    (now `:144`), and `v1-31`'s cite `:90` for "the dialog never renders" (now `:102`); none was rewritten;
  - `architecture-amendments.md` is over the 120 000 B whole-read bound: phase reads it through its index until
    the Epoch 3 close consolidates it.
- **Curation:** three Tier 2 writes (`testing.md` two, `verification-harness.md` one).
- **Deferred learnings** (carried, plus two):
  - `recurrence-despite-learning: host-win32.md` Long single-line files (a cut-limited view answering a
    membership question);
  - `recurrence-despite-learning: host-win32.md 2026-09-28/29` (heredoc to a file);
  - `recurrence-despite-learning: host-win32.md` (`rm -rf` in a compound, refused);
  - `recurrence-despite-learning: host-win32.md 2026-09-25` (`pkill -f` self-match);
  - `session-learnings.md 2026-09-29` (gate.py hygiene reads `/home/<x>/` in prose);
  - `host-win32.md` (zero-is-healthy count probe);
  - `testing.md` (bounded mutant-reachable waits);
  - the "not measured here" vocabulary;
  - PID 1 as the cleanup-deadline target;
  - the PTY master close needing no held clone;
  - let a red CI run finish before folding its fix;
  - the doubled-backslash guard recurrence;
  - `grep` on the Linux dev host is ugrep, and a counted-context `-o -E` extraction is refused as too complex;
  - a time written into an evidence file ahead of the clock (read `date -u`);
  - `inputs.py snap` takes an in-session answer with `--message-file … --origin …`, never `--source`;
  - a subagent's return can arrive with tag-like text neutralised: never paste a detector's change line that
    holds a tag;
  - an extension for `host-win32.md` 2026-10-07 (a `pgrep -f` on a script name reads other projects' sessions;
    decide by cwd);
  - new: the Bash tool's `find` is an embedded finder that refuses a GNU-style `-newermt` timestamp; the ISO form
    works (deferred by the cap of three);
  - new: `recurrence-despite-learning: host-win32.md` Exit codes (a push read through `| tail`, with
    `PIPESTATUS` read beside it).
- **Operator desk (the founder's word: leave them):**
  - Run D's plan file under `~/.claude/plans/`;
  - the empty gitignored `.viola-verify-2095228/` and the ten `.viola-verify-*` dirs of 2026-10-06 at the root;
  - `~/.viola-record-20261004T142325Z`;
  - the Run B/C/D transcripts, the previous chunk's three live-session transcripts and this round's four;
  - this round's home, `target/e2e-home/viola-reverify-20261007T124408Z/`, left where it is: it sits on the
    tmpfs behind the `target/e2e-home` link and is gone at a reboot;
  - yours to delete: `target/e2e-home.disk`, whole. This chunk read nothing in it and no chunk in flight does.
- **R-S3:** Upgrade U02 and the `host-win32.md` regenerate run at the Epoch 3 boundary (`/andromeda-setup-project`).
- **Last failed command:** none.
