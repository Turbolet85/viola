# Session Handoff

**Last Updated:** 2026-10-07T11:40Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host — live rows and paste shapes on the dev host, three live sessions on claude 2.1.287, the unwrap fixed for a paste before typed text, the hint remedy chosen

## Position
- Done: **2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** (42 complete, 0 pending, 0 gated). It claimed no capability.
- Next: **Send waits out the paste hint** (`working-route.md:98`) → `/andromeda-phase`.
  - The founder's ruling of 2026-10-07T10:29Z: on a verified CLI the gate waits for the input box on a quiet
    literal-less screen, the bound 8.5 s. Live cap: one by-path re-verify of `claude` 2.1.287, 5 starts; more
    returns to him.
  - Then **A send ending in a newline is confirmed** (`:100`), then **First live test and self-drive** (`:102`),
    the last Epoch 3 entry. Epoch 3 has 19 entries and stays one epoch (R-L2).

## Work done
- Three live `claude` 2.1.287 sessions by path (3 of the cap of 10), each ledgered before its start. One product
  file changed: `hook.rs` (`unwrap_pastes` takes the second newline after a pair that typed text follows; seven
  `live_shape` cases). No ledger row, no fixture, the readiness gate untouched.
- Measured: three paste shapes, a typed `<task-notification>` at a prompt's start (filed `harness`), a real
  cross-session prompt (unescaped, three attributes), a 2.1.287 hook's `CLAUDE*` names (12, four outside the
  eleven), the paste hint (8.0 s from the last long paste) with a `send` and a `wait` inside it on both homes.
- CI green on the pre-CI commit `33d2084` (ci#37609247992, 15/15). Record: the chunk's `report.md` and `evidence/`.

## Drift resolved
- 22 amendments in four masters and one key file (architecture 15, test-plan 4, security-plan 2, obs-plan 1), three
  leaves re-derived. Two escalations resolved with the overseer in one halt.
- Four route cards: the newline send got its own entry (the overseer); the kill-leftover finding is a CARRY on
  "Self-healing state" (`:105`); the floor stays eleven and a shape no `send` relies on needs no probe (the
  founder's two live rulings of 2026-10-07T11:37Z, relayed by the overseer), both written into the masters.
- Record: `.andromeda/runs/2026-10-07T10-53-41-wrap/` (`fanout-results.md`, `cascade-dispositions.md`,
  `route-cards.md`).

## Notes
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z):** `v1-34`'s two rows, the harness-prefix row and the
  R8 identity-floor row. Each has a first measurement on 2.1.287 and no stamped probe; a probed row for either
  needs its own founder ruling (a second session addressing a verify child; the capture arm reading names).
- **Spent:** the two scratch-measurement answers of 09:43Z cover one peer message and one names list, no more.
- **Known and unfixed until `:100`:** a `send` whose text ends in a newline is delivered, reported
  `not-delivered`, and takes the wheel from the driver.
- **Standing rule (the overseer):** a removal the permission layer denies is not done through another tool; move
  the dir into `target/e2e-home.disk/` and record it.
- **Standing rule (carried):** a stalled-start red is a finding about the backing. Read
  `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/`, stop and report; never re-run for green.
- **After a `cargo clean`:** re-make the link, `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
- **Route:** `:102` carries the copied-tree question for a mutation run to the Epoch 3 boundary audit;
  `BLOCKED-ON` at `:144` stands (no interactive Windows host).
- **For the operator's word** (carried):
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32` and `v1-40` cite `:90` for the real Windows terminal's mouse report
    (now `:144`), and `v1-31`'s cite `:90` for "the dialog never renders" (now `:102`); none was rewritten;
  - `architecture-amendments.md` is over the 120 000 B whole-read bound: phase reads it through its index until
    the Epoch 3 close consolidates it.
- **Curation:** three Tier 2 writes (`host-win32.md` two, `testing.md` one).
- **Deferred learnings** (carried, plus three):
  - `recurrence-despite-learning: host-win32.md` Long single-line files (a cut-limited view answering a
    membership question);
  - `recurrence-despite-learning: host-win32.md 2026-09-28/29` (heredoc to a file; recurred this session again);
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
  - new: `inputs.py snap` takes an in-session answer with `--message-file … --origin …`, never `--source`;
  - new: a subagent's return can arrive with tag-like text neutralised (a backslash after the angle bracket),
    which here reads as the CLI's escaped form: never paste a detector's change line that holds a tag;
  - new: an extension for `host-win32.md` 2026-10-07 (a `pgrep -f` on a script name reads other projects'
    sessions; decide by cwd).
- **Operator desk (the founder's word: leave them):**
  - Run D's plan file under `~/.claude/plans/`;
  - the empty gitignored `.viola-verify-2095228/` and the ten `.viola-verify-*` dirs of 2026-10-06 at the root;
  - `~/.viola-record-20261004T142325Z`;
  - the Run B/C/D transcripts and this chunk's three live-session transcripts;
  - yours to delete: `target/e2e-home.disk`, whole. It now also holds this chunk's three scratch homes, moved
    there and not removed: `viola-hint-20261007T101352Z-v` and `viola-hint-20261007T101508Z-u` (real CLI
    content) and `viola-hint-rehearsal-20261007T101125Z`. No chunk in flight reads the stamp home in it; whether
    the hint entry's re-verify wants it kept is that entry's phase question, so ask before deleting.
- **R-S3:** Upgrade U02 and the `host-win32.md` regenerate run at the Epoch 3 boundary (`/andromeda-setup-project`).
- **Last failed command:** none.
