# Session Handoff

**Last Updated:** 2026-10-17T02:19:29Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-17-end-to-end-path-binary-and-harness-legs — run --e2e, boot --statusline-echo and --agents-mode, status over viola list, Path 1 as one flow and Path 6's harness leg

## Position
- Done: **2026-10-17-end-to-end-path-binary-and-harness-legs** — `run --e2e` (suite `nextest-e2e`, the integration layer
  its complement, the E2E regex widened to `^(path[0-9]*|…)_`), `boot --statusline-echo` / `--agents-mode`, `status`
  nesting `viola list --json`, `harness_session`, root `tests/path1_run_start.rs` and
  `crates/viola-e2e/tests/path6_budget_harness.rs`; `v1-02` verified. Wrapped in `.andromeda/runs/2026-10-17T01-57-16-wrap/`.
- Next: **CLI machine contract** (the first markerless entry) → `/andromeda-phase`.

## Work done
- Implement green in two fix rounds; the operator pass's pre-CI commit `ae21376`, CI green 15/15 (`ci#38992234808`).
  No live `claude` start was made.

## Drift resolved
- 20 fan-out proposals (17 test-plan, 3 architecture) applied, 0 rejected, 0 escalated; the step-2 sweep folded in
  obs-plan's heartbeat-ticks bind (an absent instance reads `degraded`); 16 leaf findings over 6 leaves. Record:
  `fanout-results.md`, `cascade-dispositions.md`.

## Notes
- **Provisional, for the founder's own word at the Epoch 5 boundary batch:**
  - this chunk: fork 1 (Path 1 on the root chain) and fork 2 (`harness_session` keeps the spawned `supervise`), the
    overseer as his delegate (`inputs#I1`); the E2E regex widening to `^(path[0-9]*|…)_`, the overseer's technical
    answer, founder-delegated (`inputs#I2`), amended into test-plan §2 / §3 and the testing.md leaf;
  - carried: the unwrapped-sessions answer-card confirm (forks 1, 3, 4; delegate viola-overseer), the
    resume-and-compact answer-card confirm of fork 2, the owed-ledger-rows fork-3 card confirm and the paste-newline
    fifth-paste card confirm.
- **WATCH retired:** `cli_send` `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` — 3 green runs
  (implement's two pre-push runs and the operator pass's re-run), CI green on all three runners.
- **Every home stamped before chunk 2026-10-16-unwrapped-sessions-on-the-board reads `cli_verified:false`** until its next
  `viola verify` (a 22-id Unix or 19-id Windows stamp is not the relied set of 23 / 20).
- **For the founder's desk (left on disk, not removed):** `target/carry5-witness/` (this chunk's CARRY 5 witness build);
  carried: `target/control-92264a7/`, `target/p4-live-20261016T211033Z/`, `target/agents-scale-214213/`,
  `target/e2e-home/viola-record-20261016T204612Z/` (tmpfs), and an earlier session scratchpad's `control/`,
  `control.tar`, `p4-rig/` (a recursive `rm` refused by the host's deny rule).
- **Epoch growth:** Epoch 5 holds 15 entries (5 markerless). The founder ruled no boundary inside Epoch 5.
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
  `src/cmd/verify.rs`; CARRY 3's three `viola-state` retry-loop mutants; this chunk's new `viola-e2e` code
  (`harness::session`, `status::derive`, the boot flags) is scored there (no chunk mutation entry).
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
- **Deferred learnings (carried):** `~/.claude/CLAUDE.md` Shell (a bounded repetition sent to the embedded grep);
  `host-linux.md` Transports (one inline python heredoc carrying document text).
- **Operator desk (the founder's word: leave them, carried):** the earlier chunks' scratch files; gate logs under the
  OS temp dir's `andromeda-gate/`; `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`; live homes
  and rig directories; `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  `~/.viola-record-20261004T142325Z`; the CLI's own transcripts. Yours to delete: `target/e2e-home.disk` and
  `target/profraw-census/`.
- **Last failed command:** none.
