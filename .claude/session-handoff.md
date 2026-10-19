# Session Handoff

**Last Updated:** 2026-10-19T07:21:28Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the pre-CI commit `2a3d5691` was pushed by the operator pass; the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-18-plugin-precedence-live-readings — the two plugin readings on claude 2.1.287: the `--plugin-dir` server alone starts and answers; an exit-1 `viola mcp` does not block the start; v1-42 verified

## Position
- Done: **2026-10-18-plugin-precedence-live-readings** — 3 of the founder's 3 starts (`inputs#I2` F2), 1 red never
  re-fired. Start 2 (wrapped): one `viola` MCP server, the `--plugin-dir` one, starts and answers `list`; the installed
  `viola@viola` never starts; one SessionStart hook process. Start 3 (unwrapped, home 0770): `idle` — an exit-1
  `viola mcp` does not block the CLI. The trust dialog passed by the arrow rule (F1; default pointer one row above the
  trust row). v1-42 verified. Wrapped in `.andromeda/runs/2026-10-19T07-00-05-wrap/`.
- Next: **Dialog hold for the driver** (Epoch 5, carries the clippy PREREQ and the migrated WATCH) → `/andromeda-phase`.

## Work done
- Implement `.andromeda/runs/2026-10-19T05-23-09-implement/` (outcome file); the rig `evidence/live-pty-arrow.py` +
  `rig-controls.py` (10 of 10); the round `evidence/live-round.md`; operator pass `evidence/operator-pass.md`: pre-CI
  commit `2a3d5691`, `ci#39209618442` green 15/15. Rig A (in-tree `proj/`) met the external-imports modal at start 1;
  the founder ruled the rig outside the repo («Стенд вне репо», `inputs#I4`) and starts 2–3 ran there.

## Drift resolved
- 7 master edits (architecture ×4, security-plan ×2, test-plan ×1; 0 fan-out proposals — all raised by Validate check
  5), 5 sidecar entries, 13 leaf findings over 8 leaves; one halt (3 questions): the answer card (F1/F2 part of the
  residual), the rig place ratified by `inputs#I4`, and the `VIOLA_DIR` home step's NOT-ratified marks LIFTED (the
  founder's «Ратифицировать», `inputs#I2`/`#I3`). Record: `fanout-results.md`, `answer-card.md`, `cascade-dispositions.md`.

## Notes
- **PROVISIONAL, for the founder's own word at the Epoch 5 boundary batch (new this wrap):** the answer-card confirm
  (the Viola overseer's fit check) of security-plan §Data Protection's F1 / F2 part.
- **The founder's own words this chunk:** `inputs#I2` (F1 «Стрелка + Enter» · F2 «3 старта» · «Ратифицировать» for
  the `VIOLA_DIR` home step), `inputs#I4` («Стенд вне репо»), each relayed verbatim by the operator.
- **Waits for a 0-pending wrap after this one (the operator, `inputs#I5`):** the founder's V63 ruling.
- **New this wrap:** `viola mcp` ends with its session writing no `process-exit{subject:"self"}` line (start 2's
  `mcp.ndjson`; no master claim, no owner — for the founder's desk). The plan-template's step "matrix records are
  never implement's" conflicts with implement's letter (the write was made; recorded as evolve telemetry).
- **WATCH (carried, re-authored):** `send_budget_paused…` case_4 — `0/3`, on "Dialog hold for the driver", non-advancing
  this chunk (no plain `cargo test` ran).
- **Every home stamped before chunk 2026-10-18-compact-post-condition-ledger-row reads `cli_verified:false`** until its
  next `viola verify` (24-row relied set).
- **CARRYs (carried):** "Home and code-bearing file integrity" (the `marketplace/` tree), "Windows-only live
  measurements" (every 2.1.287 reading's Windows leg — the two plugin readings and `local-command-compact` included).
- **PROVISIONAL / NOT ratified, carried:** the mcp wrap's Q2 (`ServeError`) and Q3 (`viola-channel`'s `fake-agent =
  ["tokio"]`); the eighth gap's answer-card confirm; the plugin chunk's answer-card confirm, `inputs#I8`, `inputs#I5`
  Q2 and `inputs#I7`; the compact chunk's answer-card confirm and `inputs#I6`; start 2 of the plugin chunk made with
  `hyprctl locked` true.
- **`v1-39` un-claimed (carried):** held by its CARRY on "Exit-cause code catalogue". **`v1-34` stays unclaimed**: its
  harness-prefix and R8 identity-floor rows are owed with no entry minted (the founder, 2026-10-07T09:43Z).
- **Provisional, for the founder at the Epoch 5 boundary batch (carried):** the cli-machine-contract P4 fork answers; the
  end-to-end chunk's forks 1 and 2 and its E2E regex widening; the unwrapped-sessions card confirm (forks 1, 3, 4); the
  resume-and-compact fork 2, owed-ledger-rows fork 3 and paste-newline fifth-paste confirms; the cli-output-discipline
  card confirm, Q2 / Q3 and P2 placements; the mcp wrap's card confirm and Q2 / Q3.
- **Epoch growth:** Epoch 5 holds 18 entries (2 markerless). The founder ruled no boundary inside Epoch 5.
- **For the founder's desk (new):** `target/plugin-live-20261019T053121Z/`, `target/plugin-live-20261019T061245Z/`,
  `target/e2e-home/viola-live-plugin-20261019T053121Z/`, `target/e2e-home/viola-live-plugin-20261019T061245Z/` (tmpfs);
  the CLI's `~/.claude.json` trust entries for both rigs' `proj/`; one `~/.claude/projects/` dir and the CLI's MCP log
  dirs under `~/.cache/claude-cli-nodejs/` for rig B's `proj/`.
- **For the founder's desk (carried):** the CLI's orphaned plugin cache `~/.claude/plugins/cache/viola/viola/0.1.0/`
  (now naming rig B's home) and an unattributed `~/.claude/plugins/synced/` pair; `target/plugin-live-2026101[78]*`,
  `target/e2e-home/viola-live-plugin-2026101[78]*`, `target/ci-38994749835/`, `target/carry5-witness/`,
  `target/control-92264a7/`, `target/p4-live-20261016T211033Z/`, `target/agents-scale-214213/`,
  `target/e2e-home/viola-record-*` (tmpfs), an earlier scratchpad's `control/`, `control.tar`, `p4-rig/`.
- **For the founder, his next batch (carried):** the GUI scan forms and the axum `http2` ban; `.claude/rules/a11y.md`,
  `testing.md`, `api.md` / `frontend.md` still glob `e2e-web/**` and `crates/viola-ui/**`; obs-plan §3's Log-format copy
  differs from test-plan's in two places; `run` step 3 reads "none in 0.1.0"; Brand Identity's cut-material citations;
  a11y-plan's tier; the interrupted pair's wording; the Windows event set; a refused strict-modes check writes no line on
  `revive`'s arms; the checked `budget.json` read writes no line on a refusal; `list`'s exit-1 shapes; the five
  implicit-link shapes; a link line with no string `ts`; a `--home` session shows no user status line until
  `statusline-source.json` is planted; one CLI plan file (2026-10-08) in his user directory; `plugin install`'s no-line refusal.
- **For whoever runs the Epoch 5 boundary audit (carried):** `verify_window_without_screens_fails_every_interactive_row`
  against the mutants profile's 45 s kill (with a sixth paste — measure it); CARRY 2's grading of
  `src/bin/viola-fake-agent.rs` and `src/cmd/verify.rs`; CARRY 3's `viola-state` retry-loop mutants; the end-to-end,
  machine-contract, output-discipline, mcp, plugin and compact chunks' code (no mutation run, the founder's cap).
- **For the operator's word (carried):** the link rule takes a logged `driver` / `driven` as a string; `docs/conventions.md`
  and `docs/stack.md` list `viola link|unlink`; `v1-33`'s title and `requirements.md:48` say "on Windows"; ledger notes cite
  a bare `:90`; seven budget-governor plan sentences word the old hold; a live `claude` session ending in this tree
  rewrites this file's last section; whether `viola-mcp` inherits `license.workspace`; test-plan §1's entity list and the
  `marketplace/` tree; the plan template's preservation guard reads red on any recorded companion edit.
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
  - A scripted `.rs` write runs rustfmt in the same call. A live rig rebuilds `target/debug/viola` first, and a live rig
    `proj/` stands outside this work tree.
- **Deferred learnings:** carried: testing.md 2026-10-17 (a `binary(<tui_ stem>)` integration filter selects nothing,
  read as fails); testing.md 2026-10-07 Ext. 2026-10-16 (a shared helper's count missed by the gate filter); the
  auto-memory rule on a pipeline tool's stdout; host-linux.md Paths (a `cd X 2>/dev/null; …` prefix), `~/.claude/CLAUDE.md`
  Shell (bounded repetition), host-linux.md Transports.
- **Operator desk (the founder's word: leave them, carried):** the earlier chunks' scratch files; gate logs under the OS temp
  dir's `andromeda-gate/`; `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`; live homes and rig
  directories; `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  `~/.viola-record-20261004T142325Z`; the CLI's own transcripts. Yours to delete: `target/e2e-home.disk` and `target/profraw-census/`.
- **Last failed command:** none.
