# Session Handoff

**Last Updated:** 2026-10-16T23:41:23Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-16-unwrapped-sessions-on-the-board — the board reads claude agents --json, lists unwrapped sessions read-only, and revive refuses a session live elsewhere

## Position
- Done: **2026-10-16-unwrapped-sessions-on-the-board** — `viola list` joins one `claude agents --json` read (STATUS
  idle/busy, the `-- UNWRAPPED - READ-ONLY --` rack), `viola revive` refuses `session-live`, ledger row 23 `agents-join`
  (relied 23 Unix / 20 Windows). Wrapped in `.andromeda/runs/2026-10-16T22-30-02-wrap/` across two windows (P1 split at
  the context alarm, `p1-progress.md`).
- Next: **End-to-end path binary and harness legs** (the first markerless entry) → `/andromeda-phase`.

## Work done
- The operator pass's final sha `020c80b`, CI green 15/15 (`ci#38971772946`); 8 of the founder's 8 live starts spent
  (record round 6, P4 / P12 leg 2): P4 reads two writers, so `session-live` stays a refusal; P12 follows.

## Drift resolved
- 93 fan-out proposals + 5 the wrap raised (4 design-system, 1 architecture) applied, 0 rejected; 35 rode one answer
  card (forks 1, 3, 4). The step-2 sweep folded in 2 stale test-plan summary literals; 67 leaf findings over 20 leaves.
  Record: `fanout-results.md`, `answer-card.md`, `cascade-dispositions.md`.

## Notes
- **Provisional, for the founder's own word at the Epoch 5 boundary batch:**
  - this wrap's answer card: the overseer's DELEGATE confirm (2026-10-16T23:16:23Z) that 35 proposals lie inside
    the founder's own answers to P4 forks 1, 3 and 4 of this chunk (`inputs#I1`); the ratification stays his answers;
  - carried: the resume-and-compact answer-card confirm of fork 2 (delegate viola-overseer), the owed-ledger-rows
    fork-3 card confirm and the paste-newline fifth-paste card confirm.
- **Every home stamped before this chunk reads `cli_verified:false`** until its next `viola verify` (a 22-id Unix or
  19-id Windows stamp is not the relied set of 23 / 20).
- **WATCH** (on "End-to-end path binary and harness legs"): the `cli_send`
  `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` red, cause unknown; retires at 3 green runs.
- **For the founder's desk (left on disk, not removed):** `target/control-92264a7/`, `target/p4-live-20261016T211033Z/`,
  `target/agents-scale-214213/`, `target/e2e-home/viola-record-20261016T204612Z/` (tmpfs), and the implementing session
  scratchpad's `control/`, `control.tar`, `p4-rig/` (a recursive `rm` refused by the host's deny rule).
- **Epoch growth:** Epoch 5 holds 15 entries (6 markerless). The founder ruled no boundary inside Epoch 5.
- **For the founder, his next batch (carried):**
  - the GUI scan forms and the axum `http2` ban left on the overseer's P4 answers (technical, founder-delegated);
  - `.claude/rules/a11y.md`, `testing.md` and `api.md` / `frontend.md` still glob `e2e-web/**` and `crates/viola-ui/**`
    in their `paths:` frontmatter;
  - obs-plan §3's Log-format copy still differs from test-plan's in two older places (the `D-07` spelling, the
    `diag-line` `corr` rule's place);
  - `run` step 3 reads "none in 0.1.0" so the code's citations of `run` step 4 hold;
  - Brand Identity's L41 / L45 / L16 / L21 cite cut material; a11y-plan's tier `Standard (1)` is his;
  - the interrupted pair's wording; the Windows event set; a refused strict-modes check writes no line on `revive`'s
    start arm; the checked `budget.json` read writes no line on a refusal; `list`'s exit-1 shapes and per-instance
    no-row; the five implicit-link shapes; a link line with no string `ts`; a `--home` session shows no user status
    line until `statusline-source.json` is planted; `viola revive --list` opens no process log; the switch from the
    prototype not made, `v1-33` unclaimed; one CLI plan file (2026-10-08) in his user directory.
- **For whoever runs the Epoch 5 boundary audit (carried):** `verify_window_without_screens_fails_every_interactive_row`
  reads 42.98 s against the mutants profile's 45 s kill; CARRY 2's grading of `src/bin/viola-fake-agent.rs` and
  `src/cmd/verify.rs`; CARRY 3's three `viola-state` retry-loop mutants.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row; `v1-34` stays unclaimed.
- **For the operator's word (carried):** the link rule takes a logged `driver` / `driven` as a string;
  `docs/conventions.md` and `docs/stack.md` still list `viola link|unlink`; `v1-33`'s title and `requirements.md:48`
  say "on Windows"; the ledger notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90`; seven sentences of the
  budget-governor plan word the old hold; a live `claude` session ending in this tree rewrites this file's last section.
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`; the host leaf is
  `host-linux.md`.
- **Standing rules (carried):**
  - A removal the permission layer denies is not done another way: a scratch home moves into `target/e2e-home.disk/`;
    a tracked directory is the founder's to remove at the desk.
  - A stalled-start red is a finding about the backing: read `test -L target/e2e-home && findmnt -n -o FSTYPE -T
    target/e2e-home/`, then stop and report.
  - Read `hyprctl locked` before any step that focuses a window or types a key.
  - Before a census run (`bash scripts/profraw-census.sh 4800 48`), write the operator one line.
  - After a `cargo clean`, re-make `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
  - The Bash guard refuses a pipeline tool piped into a filter or sent to a file, a `cd` that moves the cwd, and a
    heredoc to a file.
  - A scripted `.rs` write runs rustfmt in the same call.
- **Deferred learnings:**
  - `recurrence-despite-learning: testing.md 2026-10-07` — its fourth recurrence (a moved count's shared-helper callers
    missed by the gate filter), landed this wrap as that entry's extension.
  - carried: `~/.claude/CLAUDE.md` Shell (a bounded repetition sent to the embedded grep); `host-linux.md` Transports
    (one inline python heredoc carrying document text).
- **Operator desk (the founder's word: leave them, carried):** the earlier chunks' scratch files; gate logs under the
  OS temp dir's `andromeda-gate/`; `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`; live homes
  and rig directories; `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  `~/.viola-record-20261004T142325Z`; the CLI's own transcripts. Yours to delete: `target/e2e-home.disk` and
  `target/profraw-census/`.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-17 03:20:16
