# Session Handoff

**Last Updated:** 2026-10-14T07:06:24Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-13-owed-ledger-rows — Owed ledger rows: `statusline-override`, `statusline-shell`, `statusline-rate-limits`, relied on Unix, stamped live on 2.1.287

## Position
- Done: 2026-10-13-owed-ledger-rows.
  - `LedgerRow::ALL` holds 21 rows. The relied set (`LedgerRow::relied()`) is 21 on Unix and 18 on Windows, where the
    status line is not wrapped. The stamp verdict and verify's `[NN/MM]` count read it.
  - Run B gains one Unix probe `--settings`, and there is a hidden `hook statusline --capture --shell` arm.
  - Two live rounds, ten starts, both `stamped 2.1.287  21 pass  0 fail`.
  - CI: red on the pre-CI commit (two of this chunk's cases), fixed, then green on `1c4f2258ef7c` (ci#38520999442).
- Next: **Resume and compact ledger rows** (minted this wrap, the first markerless entry) → `/andromeda-phase`.

## Work done
- The chunk: implement run `.andromeda/runs/2026-10-13T23-05-48-implement/`, wrap run
  `.andromeda/runs/2026-10-14T06-37-54-wrap/`.

## Drift resolved
- Amendments over six masters (architecture, security-plan, test-plan, obs-plan, design-system, layout-templates) and
  four key files; seven sidecar entries.
  - 54 proposals came from the detectors and 9 were raised by main from the plan's expected amendments.
  - Halt: one answer card (the founder's fork-3 widening) and one question (the fixture handle). Both are confirmed.
- 47 leaf findings applied over 13 leaves. The record is in `fanout-results.md`, `answer-card.md` and
  `cascade-dispositions.md`.

## Notes
- **Provisional, for the founder's own word at the Epoch 5 boundary batch.**
  - This chunk's answer-card confirm of the fork-3 widening (Run B's probe `--settings` and the statusline capture
    arm) was given by the operator as delegate viola-overseer. The ratification is the founder's own answer to P4
    fork 3 (`inputs#I1`).
  - The paste-newline chunk's fifth-paste card confirm is carried the same way.
  - Both are recorded so in the owning sidecars.
- **The fixture handle.**
  - The two `StatusLine` fixtures keep `workspace.repo.owner` (this repository's own remote owner).
  - The decision is the founder's own word (`inputs#I3`, «Принять и пушить»). Its landing in test-plan §7/§11,
    security-plan §Data Protection and architecture Repository is the overseer's application of it.
- **Every Unix home stamped before this chunk reads `cli_verified:false`** until its next `viola verify` (an 18-id
  stamp is not the relied set). The tmpfs `--local-live` home was re-stamped at 21 by this chunk's round.
- **Epoch growth:** Epoch 5 holds 13 entries (one minted this wrap; 7 markerless). The founder ruled no boundary
  inside Epoch 5; a split stays the operator's word.
- **For whoever dispatches `windows-mutants.yml` next (the Epoch 5 boundary audit):**
  - CARRY 2's grading of `src/bin/viola-fake-agent.rs` and `src/cmd/verify.rs`, and CARRY 3's three `viola-state`
    retry-loop mutants, are still owed there (carried).
  - This chunk adds no Windows-gated product file: the harness's 18-row literal is `cfg(not(unix))`.
- **For the founder, his next batch (the operator's word, none ruled by him; carried):**
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
  - the two card confirms above.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row. `v1-34` stays unclaimed.
- **For the operator's word (carried):**
  - the link rule takes a logged `driver` / `driven` as a string;
  - `docs/conventions.md` and `docs/stack.md` still list `viola link|unlink`;
  - `v1-33`'s title and `requirements.md:48` say "on Windows";
  - the ledger notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90`;
  - seven sentences of the budget-governor plan word the old hold;
  - a live `claude` session ending in this tree rewrites this file's last section.
- **Route:**
  - "Resume and compact ledger rows" is minted directly after "Owed ledger rows", carrying its CARRY 2 and 3
    verbatim and CARRY 1's cost frame (the founder's fork 1, `inputs#I1`; card `inputs#I2`; `inputs#I4`).
  - The `claude agents --json` join row moved onto "Unwrapped sessions on the board" (fork 2).
  - "Windows-only live measurements" gained a CARRY for the statusline rows' Windows leg; its `BLOCKED-ON` premise
    was re-verified as standing.
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`; the host leaf is
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
  - Tier 2 `verification-harness.md`: a fake-agent side effect goes after the turn's screen write.
  - Tier 2 `testing.md`: read a new fixture class's keys for identity values; and an in-place extension of the
    2026-10-07 entry (a per-OS set's member). Log: the wrap run dir's `curation.md`.
- **Deferred learnings:**
  - `recurrence-despite-learning: testing.md 2026-10-07 (a moved bound: list the cases by the function it decides)`.
    Research missed `merge_stamp`'s literal row-set envelope oracle when `LedgerRow::ALL` grew. This is the second
    recurrence, after the paste-newline chunk.
  - carried:
    - `docs/session-learnings.md` "gate.py hygiene reads `/home/<x>/` in prose…";
    - `~/.claude/CLAUDE.md` Shell (a bounded repetition sent to the embedded grep);
    - `host-linux.md` Transports (one inline python heredoc carrying document text).
- **Operator desk (the founder's word: leave them):**
  - the earlier chunks' scratch files;
  - gate logs under the OS temp dir's `andromeda-gate/`;
  - `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`;
  - live homes and rig directories, now including this chunk's round homes `target/e2e-home/viola-record-20261013T232122Z/`
    and `target/e2e-home/viola-live-2266945/` (tmpfs);
  - `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  - `~/.viola-record-20261004T142325Z`;
  - the CLI's own transcripts, this chunk's record round's Run B, C and D transcripts included.

  Yours to delete: `target/e2e-home.disk` and `target/profraw-census/`.
- **Last failed command:** none.
