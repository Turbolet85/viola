# Session Handoff

**Last Updated:** 2026-10-18T22:32:35Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-18-compact-post-condition-ledger-row — `/compact` confirmed by its same-session SessionStart, the 24th ledger row `local-command-compact`, its own 40 s window

## Position
- Done: **2026-10-18-compact-post-condition-ledger-row** — on a verified CLI a `send` of `/compact` is confirmed by a
  `session-start` of cause `compact` with the remembered (same) id, inside `COMPACT_CONFIRM_WINDOW` 40 s PROVISIONAL;
  the 24th row `local-command-compact` (relied 24 Unix / 21 Windows), probed by Run B's sixth paste, stamped live on
  2.1.287 (`24 pass  0 fail`, one round, 6 of the founder's 12 starts). Wrapped in `.andromeda/runs/2026-10-18T21-59-05-wrap/`.
- Next: **Plugin precedence live readings** (new, written by this wrap from the P5 card's row 1) → `/andromeda-phase`.

## Work done
- Implement `.andromeda/runs/2026-10-18T21-07-38-implement/` (outcome + journal); operator pass: pre-CI commit `3d5ae39`,
  `ci#39169321430` green 15/15. Two recorded out-of-list test edits (`tests/support/home.rs`, `tests/tui_channel_fds.rs`),
  accepted by the overseer (`inputs#I6`); the plan's preservation guard reads red on exactly those two (noted at the
  light gate, the operator's direction `inputs#I7`).

## Drift resolved
- 77 amendments over all seven masters and six key files (75 fan-out proposals + 2 raised, 0 rejected); one halt, the
  answer card (the sixth Run B paste and its transcript residual, inside the founder's own fork-1 answer `inputs#I3`),
  confirmed by the Viola overseer — PROVISIONAL. 49 leaf findings applied over 12 leaves. Record: `fanout-results.md`,
  `cascade-dispositions.md`, `answer-card.md`.

## Notes
- **PROVISIONAL, for the founder's own word at the Epoch 5 boundary batch (new this wrap):** the answer-card confirm
  (14 proposals: Run B's six pastes, the three-file transcript residual); the overseer's `inputs#I6` (the two out-of-list
  test edits, entry 15 red on them).
- **The founder's own words this chunk:** `inputs#I3` (F1 sixth Run B paste · F2 cap 12 starts · F3 CARRY 2 to the new
  entry · F4 no plugin start), relayed verbatim by the operator.
- **New this wrap:** the transcript residual is 5 per real-CLI verify (Run B 3 incl. the compaction's `-backup-` file, C 1,
  D 1) — written into security-plan / architecture; whether the CLI fires a SessionEnd for `/compact` is unmeasured; the
  record dir `target/e2e-home/viola-record-20261018T212421Z/` (tmpfs) is for the founder's desk; the tmpfs `run
  --local-live` home re-stamps at its next firing (P5 card row 2, no entry owns it).
- **WATCH (new):** `send_budget_paused…` case_4 red once under plain `cargo test` — pinned on "Plugin precedence live readings".
- **Every home stamped before this chunk reads `cli_verified:false`** until its next `viola verify` (24-row relied set).
- **CARRYs (carried):** "Plugin precedence live readings" (CARRY 2 whole, `v1-42`), "Home and code-bearing file
  integrity" (the `marketplace/` tree, the P5 card row 2 of the plugin chunk), "Windows-only live measurements" (every
  2.1.287 reading's Windows leg, `local-command-compact` included).
- **PROVISIONAL / NOT ratified, carried:** the mcp wrap's Q2 (`ServeError`) and Q3 (`viola-channel`'s `fake-agent = ["tokio"]`);
  `viola mcp`'s and every CLI verb's home through `resolved_home`; the eighth gap's answer-card confirm; the plugin
  chunk's answer-card confirm, `inputs#I8`, `inputs#I5` Q2 and `inputs#I7`; start 2 made with `hyprctl locked` true.
- **`v1-39` un-claimed (carried):** held by its CARRY on "Exit-cause code catalogue". **`v1-34` stays unclaimed**: its
  harness-prefix and R8 identity-floor rows are owed with no entry minted (the founder, 2026-10-07T09:43Z).
- **Provisional, for the founder at the Epoch 5 boundary batch (carried):** the cli-machine-contract P4 fork answers; the
  end-to-end chunk's forks 1 and 2 and its E2E regex widening; the unwrapped-sessions card confirm (forks 1, 3, 4); the
  resume-and-compact fork 2, owed-ledger-rows fork 3 and paste-newline fifth-paste confirms; the cli-output-discipline
  card confirm, Q2 / Q3 and P2 placements; the mcp wrap's card confirm and Q2 / Q3.
- **Epoch growth:** Epoch 5 holds 18 entries (3 markerless). The founder ruled no boundary inside Epoch 5.
- **For the founder's desk (carried):** the CLI's orphaned plugin cache `~/.claude/plugins/cache/viola/viola/0.1.0/` and an
  unattributed `~/.claude/plugins/synced/` pair; `target/plugin-live-*`, `target/e2e-home/viola-live-plugin-*`,
  `target/ci-38994749835/`, `target/carry5-witness/`, `target/control-92264a7/`, `target/p4-live-20261016T211033Z/`,
  `target/agents-scale-214213/`, `target/e2e-home/viola-record-*` (tmpfs), an earlier scratchpad's `control/`,
  `control.tar`, `p4-rig/`; an open question: which row 2.1.287's trust dialog selects by default.
- **For the founder, his next batch (carried):** the GUI scan forms and the axum `http2` ban; `.claude/rules/a11y.md`,
  `testing.md`, `api.md` / `frontend.md` still glob `e2e-web/**` and `crates/viola-ui/**`; obs-plan §3's Log-format copy
  differs from test-plan's in two places; `run` step 3 reads "none in 0.1.0"; Brand Identity's cut-material citations;
  a11y-plan's tier; the interrupted pair's wording; the Windows event set; a refused strict-modes check writes no line on
  `revive`'s arms; the checked `budget.json` read writes no line on a refusal; `list`'s exit-1 shapes; the five
  implicit-link shapes; a link line with no string `ts`; a `--home` session shows no user status line until
  `statusline-source.json` is planted; one CLI plan file (2026-10-08) in his user directory; `plugin install`'s no-line refusal.
- **For whoever runs the Epoch 5 boundary audit (carried):** `verify_window_without_screens_fails_every_interactive_row`
  against the mutants profile's 45 s kill (now with a sixth paste — measure it); CARRY 2's grading of
  `src/bin/viola-fake-agent.rs` and `src/cmd/verify.rs`; CARRY 3's `viola-state` retry-loop mutants; the end-to-end,
  machine-contract, output-discipline, mcp and plugin chunks' code; this chunk's `ledger.rs`, `slot.rs`, `typed.rs` and
  the fake agent's compact replay (no mutation run, the founder's cap).
- **For the operator's word (carried):** the link rule takes a logged `driver` / `driven` as a string; `docs/conventions.md`
  and `docs/stack.md` list `viola link|unlink`; `v1-33`'s title and `requirements.md:48` say "on Windows"; ledger notes cite
  a bare `:90`; seven budget-governor plan sentences word the old hold; a live `claude` session ending in this tree
  rewrites this file's last section; whether `viola-mcp` inherits `license.workspace`; test-plan §1's entity list and the
  `marketplace/` tree; the plan template's preservation guard reads red on any recorded companion edit (this chunk).
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`; the host leaf is `host-linux.md`.
- **Standing rules (carried):**
  - A removal the permission layer denies is not done another way: a scratch home moves into `target/e2e-home.disk/`; a
    tracked directory is the founder's to remove at the desk.
  - A stalled-start red is a finding about the backing: read `test -L target/e2e-home && findmnt -n -o FSTYPE -T
    target/e2e-home/`, then stop and report.
  - Read `hyprctl locked` before any step that focuses a window or types a key with `wtype`, and stop on `true`.
  - Before a census run (`bash scripts/profraw-census.sh 4800 48`), write the operator one line.
  - After a `cargo clean`, re-make `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
  - The Bash guard refuses a pipeline tool piped into a filter or sent to a file, a `cd` that moves the cwd, a heredoc to a
    file, and a bare `grep` with a bounded repetition (use `/usr/bin/grep`); the write guard refuses a Write under `target/`.
  - A scripted `.rs` write runs rustfmt in the same call. A live rig rebuilds `target/debug/viola` first.
- **Deferred learnings:** `recurrence-despite-learning`: testing.md 2026-10-17 (a `binary(<tui_ stem>)` integration filter
  selects nothing, read as fails); testing.md 2026-10-07 Ext. 2026-10-16 (a shared helper's count missed by the gate
  filter); the auto-memory rule on a pipeline tool's stdout (sent to /dev/null once this wrap); carried: host-linux.md
  Paths (a `cd X 2>/dev/null; …` prefix), `~/.claude/CLAUDE.md` Shell (bounded repetition), host-linux.md Transports.
- **Operator desk (the founder's word: leave them, carried):** the earlier chunks' scratch files; gate logs under the OS temp
  dir's `andromeda-gate/`; `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`; live homes and rig
  directories; `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  `~/.viola-record-20261004T142325Z`; the CLI's own transcripts. Yours to delete: `target/e2e-home.disk` and `target/profraw-census/`.
- **Last failed command:** none.
