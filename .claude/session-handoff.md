# Session Handoff

**Last Updated:** 2026-10-07T19:33Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-07-a-send-ending-in-a-newline-is-confirmed — a send ending in a newline is confirmed: send types the text without its trailing LF on the founder's ruling, the driver keeps the wheel, the 9f2bebe coverage red closed by its measured mechanism

## Position
- Done: **2026-10-07-a-send-ending-in-a-newline-is-confirmed** (44 complete, 0 pending, 0 gated). It claimed no
  capability.
- Next: **First live test and self-drive** (`working-route.md:102`) → `/andromeda-phase`. It is the last Epoch 3
  entry; Epoch 3 has 19 entries and stays one epoch (R-L2).

## Work done
- `send` validates a text as received, then types it without its trailing LF characters
  (`viola_agent_claude::hook::typed_text`). The local-command list, the exact match, `text_bytes` and the paste
  read that one typed text. A send ending in a newline is now confirmed `ok` and the driver keeps the wheel.
- The coverage red on `9f2bebe` (ci#37627485806 attempt 1) is closed by its measured mechanism: the root bin's
  start test killed its own instrumented child during that child's exit-time profile write. The child is now
  `whoami`. `scripts/profraw-census.sh` is the witness: 4 800 loaded runs, one whole profile each.
- CI green on the final sha `d047fec` (ci#37644414657, 15/15, first attempt) and on the pre-CI commit `0093ffe`
  (ci#37643226001). Record: the chunk's `report.md` and `evidence/`.

## Drift resolved
- 23 amendments in four masters and one key file (architecture 10, security-plan 4, test-plan 6, obs-plan 3),
  nine leaves re-derived. The three security-plan proposals marked `escalate` were resolved on two recorded
  words, the founder's ruling and the overseer's word at this wrap; no card was raised.
- Route: one CARRY and one WATCH on "First live test and self-drive" (`:102`).
- Record: `.andromeda/runs/2026-10-07T19-12-23-wrap/` (`fanout-results.md`, `cascade-dispositions.md`,
  `directive.md`, `curation.md`).

## Notes
- **For the founder:**
  - recorded as his: the remedy, strip every trailing LF, his live ruling of 2026-10-07T15:21Z, relayed by the
    overseer. The masters and four sidecar entries name it so, the security-plan amendment included;
  - the consequence he was shown now stands in architecture and security-plan: a listed local command followed by
    newlines is that command (`/clear` and a newline clears the session). Proved at the unit layer only;
  - still open, his: the `input-not-ready` hint line's wording (carried, untouched).
- **Wording of its time, not edited** (the overseer's word at this wrap): the chunk plan's Goal, step 1 and
  "Listed for the founder", `research.md` and `scope.md` still say provisional.
- **Unmeasured, pinned on `:102`:** on a live CLI, a `send` ending in newlines; a text of only newlines; a
  trailing CR; `/clear` followed by a newline. Carried from before: a `send` under the real CLI's paste hint.
- **A limit, stated in the report and in test-plan §10:** the red is closed by mechanism. The refused profile's
  writer, pid 10799, is not provable from the run, and ci#36529038462 and ci#36481260151 are not claimed closed.
  The WATCH on `:102` retires on a recurrence or on 3 green runs (0 so far).
- **Observed, cause not measured:** the final run's Windows leg was slower than the run before it (the start test
  6.3 s against 0.6 s; two paste-hint cases 10.27 s against 8.05 s, past nextest's 10 s slow line; all passed).
- **A correction of implement's report:** eleven census directories stand under `target/profraw-census/`, not
  nine. They are gitignored tallies.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s two rows, the harness-prefix row
  and the R8 identity-floor row. A probed row for either needs its own founder ruling.
- **Standing rule (the overseer):** a removal the permission layer denies is not done through another tool; move
  the dir into `target/e2e-home.disk/` and record it.
- **Standing rule (carried):** a stalled-start red is a finding about the backing. Read
  `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/`, stop and report; never re-run for green.
- **Before a census run:** `bash scripts/profraw-census.sh 4800 48` loads 48 workers for about 2.5 minutes on a
  host other builders share. Write the operator one line first; a notice, not a question.
- **After a `cargo clean`:** re-make the link, `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
- **Route:** `:102` carries the copied-tree question for a mutation run to the Epoch 3 boundary audit;
  `BLOCKED-ON` at `:144` stands (no interactive Windows host; the dev host is Linux, read at this wrap).
- **For the operator's word** (carried):
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32` and `v1-40` cite `:90` for the real Windows terminal's mouse report
    (now `:144`), and `v1-31`'s cite `:90` for "the dialog never renders" (now `:102`); none was rewritten;
  - `architecture-amendments.md` is over the 120 000 B whole-read bound (138 208 B now): phase reads it through
    its index until the Epoch 3 close consolidates it.
- **Curation:** one Tier 2 write (`testing.md`) and one extension of a Tier 3 entry (`session-learnings.md`).
- **Deferred learnings** (carried, two met again):
  - `recurrence-despite-learning: host-win32.md 2026-09-28/29` (heredoc to a file) — met again at this chunk's
    implement;
  - a time written into a record ahead of the clock (read `date -u`) — met again: an inputs origin label typed
    ten minutes ahead;
  - `recurrence-despite-learning: host-win32.md` Long single-line files (a cut-limited view answering a
    membership question);
  - `recurrence-despite-learning: host-win32.md` (`rm -rf` in a compound, refused);
  - `recurrence-despite-learning: host-win32.md 2026-09-25` (`pkill -f` self-match);
  - `recurrence-despite-learning: host-win32.md` Exit codes (a push read through `| tail`, with `PIPESTATUS`
    read beside it);
  - `session-learnings.md 2026-09-29` (gate.py hygiene reads `/home/<x>/` in prose);
  - `host-win32.md` (zero-is-healthy count probe);
  - `testing.md` (bounded mutant-reachable waits);
  - the "not measured here" vocabulary;
  - PID 1 as the cleanup-deadline target;
  - the PTY master close needing no held clone;
  - let a red CI run finish before folding its fix;
  - the doubled-backslash guard recurrence;
  - `grep` on the Linux dev host is ugrep, and a counted-context `-o -E` extraction is refused as too complex;
  - `inputs.py snap` takes an in-session answer with `--message-file … --origin …`, never `--source`;
  - a subagent's return can arrive with tag-like text neutralised: never paste a detector's change line that
    holds a tag;
  - an extension for `host-win32.md` 2026-10-07 (a `pgrep -f` on a script name reads other projects' sessions;
    decide by cwd);
  - the Bash tool's `find` is an embedded finder that refuses a GNU-style `-newermt` timestamp; the ISO form
    works.
- **Operator desk (the founder's word: leave them):**
  - Run D's plan file under `~/.claude/plans/`;
  - the empty gitignored `.viola-verify-2095228/` and the ten `.viola-verify-*` dirs of 2026-10-06 at the root;
  - `~/.viola-record-20261004T142325Z`;
  - the Run B/C/D transcripts and the two earlier chunks' live-session transcripts;
  - the previous round's home, `target/e2e-home/viola-reverify-20261007T124408Z/`, on the tmpfs behind the
    `target/e2e-home` link and gone at a reboot;
  - yours to delete: `target/e2e-home.disk`, whole, and the eleven directories under `target/profraw-census/`.
    No chunk in flight reads either.
- **R-S3:** Upgrade U02 and the `host-win32.md` regenerate run at the Epoch 3 boundary (`/andromeda-setup-project`).
- **Last failed command:** none.
