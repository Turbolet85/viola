# Session Handoff

**Last Updated:** 2026-10-16T14:46:23Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-14-resume-and-compact-ledger-rows — Resume and compact ledger rows: `resume-session`, the 22nd capability-ledger row, probed by `viola verify`'s Run E and stamped live on 2.1.287; `/compact` listed with no post-condition

## Position
- Done: 2026-10-14-resume-and-compact-ledger-rows.
  - `LedgerRow::ALL` holds 22 rows. `resume-session` is relied on every OS, so the relied set is 22 on Unix and 19
    on Windows.
  - verify runs five interactive runs. Run E resumes Run B's first session id (`--resume`, empty input), and seven
    spawn pairs are logged.
  - `/compact` is on `LOCAL_COMMANDS` with no post-condition, so a `send` of it reads `unconfirmable` at once.
  - The fake agent's `--resume` builds on the recorded `SessionStart.resume-1.json`.
  - Live: 12 of 12 starts, `stamped 2.1.287  22 pass  0 fail` and `local-live passed 1 failed 0`.
  - CI: green 15/15 on `784d9f9937d6` (ci#38868756782).
- Next: **Unwrapped sessions on the board** (the first markerless entry) → `/andromeda-phase`.

## Work done
- Implement runs:
  - `.andromeda/runs/2026-10-16T06-46-40-implement/`: stopped on entry 15, the preservation guard.
  - `.andromeda/runs/2026-10-16T07-29-18-phase/`: the plan revision.
  - `.andromeda/runs/2026-10-16T07-35-58-implement/`: the re-entry, then the operator pass (`784d9f9`).
- Wrap run: `.andromeda/runs/2026-10-16T14-05-30-wrap/`. The session was restarted (host restart) before this wrap,
  and the run dirs were re-read from disk.

## Drift resolved
- 54 amendments over six masters and five key files, with seven sidecar entries. Record: `fanout-results.md`,
  `answer-card.md` and `cascade-dispositions.md`.
- Two halts, four questions (`inputs#I6`, `inputs#I7`):
  - The answer card (Run E, the founder's fork 2): confirmed, PROVISIONAL (delegate viola-overseer).
  - The killed-PTY-child `child_exit_status` claim: amended to measured (overseer, founder-delegated).
  - The `/compact` post-condition row: placed at the end of Epoch 5 (the founder's own word «В конец Эпохи 5»).
  - The test-plan §4 case count: dropped (overseer, founder-delegated).
- 49 leaf findings applied over 12 leaves.

## Notes
- **Provisional, for the founder's own word at the Epoch 5 boundary batch:**
  - this chunk's answer-card confirm of the fork-2 widening (Run E: the sixth start and Run B's first session id on
    a child's argv), given as delegate viola-overseer. The ratification is his own answer to P4 fork 2 (`inputs#I1`).
  - Carried: the owed-ledger-rows fork-3 card confirm and the paste-newline fifth-paste card confirm.
- **Every home stamped before this chunk reads `cli_verified:false`** until its next `viola verify`. A 21-id (Unix)
  or 18-id (Windows) stamp is not the relied set. The tmpfs `--local-live` home was re-stamped at 22 by this chunk's
  round.
- **Epoch growth:** Epoch 5 holds 14 entries (one minted this wrap, 7 markerless). The founder ruled no boundary
  inside Epoch 5, so a split stays the operator's word.
- **For whoever runs the Epoch 5 boundary audit (`windows-mutants.yml` and the mutation run):**
  - `verify_window_without_screens_fails_every_interactive_row` now reads 42.98 s locally (Run E's settle). The
    mutants profile's kill is 45 s, about 2 s of margin; CI's is 60 s. No kill moved (test-plan bootstrap key).
  - Carried: CARRY 2's grading of `src/bin/viola-fake-agent.rs` and `src/cmd/verify.rs`, and CARRY 3's three
    `viola-state` retry-loop mutants.
- **For the founder, his next batch** (the operator's word, none ruled by him; carried):
  - the interrupted pair's wording;
  - the Windows event set;
  - a refused strict-modes check writes no line on `revive`'s start arm;
  - the checked `budget.json` read writes no line on a refusal;
  - `list`'s exit-1 shapes and per-instance no-row;
  - the five implicit-link shapes;
  - a link line with no string `ts`;
  - a `--home` session shows no user status line until `statusline-source.json` is planted;
  - `viola revive --list` opens no process log;
  - the switch from the prototype not made, `v1-33` unclaimed;
  - one CLI plan file (2026-10-08) in his user directory;
  - the three card confirms above.
- **Owed, no entry minted** (the founder, 2026-10-07T09:43Z, carried): `v1-34`'s harness-prefix row and its R8
  identity-floor row. `v1-34` stays unclaimed. Its `/compact` row now has an entry: "Compact post-condition ledger
  row".
- **For the operator's word (carried):**
  - the link rule takes a logged `driver` / `driven` as a string;
  - `docs/conventions.md` and `docs/stack.md` still list `viola link|unlink`;
  - `v1-33`'s title and `requirements.md:48` say "on Windows";
  - the ledger notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90`;
  - seven sentences of the budget-governor plan word the old hold;
  - a live `claude` session ending in this tree rewrites this file's last section.
- **Route:**
  - "Compact post-condition ledger row" is minted at the end of Epoch 5, after "Optional plugin install" (`inputs#I6`).
  - "Windows-only live measurements" gained a CARRY for `resume-session`'s Windows reading (`inputs#I2`). Its
    `BLOCKED-ON` premise was re-verified standing: the host reads Linux.
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`. The host leaf is
  `host-linux.md`.
- **Standing rules (carried):**
  - A removal the permission layer denies is not done another way: move it into `target/e2e-home.disk/`.
  - A stalled-start red is a finding about the backing: read `test -L target/e2e-home && findmnt -n -o FSTYPE -T
    target/e2e-home/`, then stop and report.
  - Read `hyprctl locked` before any step that focuses a window or types a key.
  - Before a census run (`bash scripts/profraw-census.sh 4800 48`), write the operator one line.
  - After a `cargo clean`, re-make `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
  - The Bash guard refuses a pipeline tool piped into a filter, a `cd` that moves the cwd, and a heredoc to a file.
  - A scripted `.rs` write runs rustfmt in the same call.
- **Curation:**
  - T1: the count entry is extended (a master states no test-case count).
  - T2: `testing.md` gains "a test fixture's synthetic session id has the real session-id shape".
  - T3: the hygiene entry is extended (a prose temp-dir glob).
  - Log: the wrap run dir's `curation.md`.
- **Deferred learnings:**
  - `recurrence-despite-learning: testing.md 2026-10-07 (a moved bound: list the cases by the function it decides)` —
    the third recurrence. Run E moved verify's run count. Ten readers moved outside the plan's list, one of them
    (`dialogless_home`, which builds its summary from an integer) caught only by `pre-push`, which cost a plan
    revision.
  - Carried:
    - `~/.claude/CLAUDE.md` Shell (a bounded repetition sent to the embedded grep);
    - `host-linux.md` Transports (one inline python heredoc carrying document text).
- **Operator desk (the founder's word: leave them):**
  - the earlier chunks' scratch files;
  - gate logs under the OS temp dir's `andromeda-gate/`;
  - `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`;
  - live homes and rig directories, now including this chunk's round homes `target/e2e-home/viola-record-*` and
    `target/e2e-home/viola-live-2894516/` (tmpfs);
  - `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  - `~/.viola-record-20261004T142325Z`;
  - the CLI's own transcripts, this chunk's Run B, C and D transcripts included (643 → 651).

  Yours to delete: `target/e2e-home.disk` and `target/profraw-census/`.
- **Last failed command:** none.
