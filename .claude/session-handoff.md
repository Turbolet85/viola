# Session Handoff

**Last Updated:** 2026-10-17T14:59:02Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-17-mcp-server-for-drivers — viola mcp: the stdio MCP server for drivers (send / wait / last / answer / list), CLI-parity results, code-only refusals

## Position
- Done: **2026-10-17-mcp-server-for-drivers** — the hidden `viola mcp` (rmcp 3.4.1 stdio, five tools, each result the CLI
  `--json` less `v`, `invalid-params` / `-32603` faults, `wait` capped at 50 000 ms), `viola-channel`'s Tokio client behind
  its `tokio` feature, `live_endpoint` moved to `viola-state`, the plugin's `.mcp.json` registering `viola`, the two gate
  scripts (`channel-tokio-free.sh`, `rmcp-features.sh`); `v1-38` verified. Wrapped in `.andromeda/runs/2026-10-17T14-19-45-wrap/`
  (two windows: P1, then P2–P7).
- Next: **Optional plugin install** (the first markerless entry) → `/andromeda-phase`.

## Work done
- Implement green; operator pass: pre-CI commit `a902e6e`, CI green 15/15 (`ci#39033242394`). Two live `claude` 2.1.287 starts
  under the founder's raised cap (`inputs#I4`): start 1 red (`tools/list` dropped without `ttlMs` / `cacheScope`), start 2 green.

## Drift resolved
- 88 amendments over all seven masters and eight key files (84 fan-out proposals + 4 raised by the wrap, 0 rejected); one halt
  of four questions on the Viola overseer's answers (`answer-card.md`): the answer card (the founder's Q1), `ServeError` an open
  divergence, the channel's test-only `fake-agent = ["tokio"]` written as built, `mcp`'s home NOT ratified. 68 leaf findings:
  63 by the tool, 4 by hand, 1 declined. Record: `fanout-results.md`, `cascade-dispositions.md`.

## Notes
- **Ratified by the founder's own word:** the eighth dated gap (MCP frame tools after the liveness-only pre-check) and MCP
  `list`'s agents read («Восьмой gap + agents», `inputs#I2` Q1); the answer-card confirm was a delegate's (the Viola
  overseer) — PROVISIONAL in each sidecar, for his own word at the Epoch 5 boundary batch.
- **PROVISIONAL / NOT ratified, for the founder's next batch:** this wrap's Q2 (`ServeError` recorded as an open divergence)
  and Q3 (`viola-channel`'s `fake-agent = ["tokio"]` written as built), both the overseer's; `viola mcp` resolving its home
  through `resolved_home` (Q4) joins the carried NOT-ratified CLI-verbs `VIOLA_DIR` / canonicalisation item.
- **CARRYs and entries this wrap:** minted "Dialog hold for the driver" and "Overseer switch to viola" after "Compact
  post-condition ledger row" (the founder's Q3 / Q4 via the operator-approved P5 card); CARRYs on "Optional plugin install"
  (does a registered `viola mcp` that exits 1 block the CLI's start), "Server verification before any frame" and "Home and
  code-bearing file integrity" (the eighth gap), and "Home and code-bearing file integrity" (the `pin_exe` race, 6/24 vs 0/24,
  `inputs#I6`). The Windows block re-verified standing (`uname` reads Linux).
- **`v1-39` un-claimed (carried):** `revive --json` after a passed preflight prints no document; held by its CARRY on
  "Exit-cause code catalogue" (status kept `implemented`).
- **Provisional, for the founder's own word at the Epoch 5 boundary batch (carried):** the cli-machine-contract P4 fork answers;
  the end-to-end chunk's forks 1 and 2 and its E2E regex widening; the unwrapped-sessions card confirm (forks 1, 3, 4); the
  resume-and-compact fork 2, owed-ledger-rows fork 3 and paste-newline fifth-paste confirms; the cli-output-discipline card
  confirm, its Q2 / Q3 and its P2 placements; this wrap's card confirm and Q2 / Q3.
- **Every home stamped before chunk 2026-10-16-unwrapped-sessions-on-the-board reads `cli_verified:false`** until its next
  `viola verify`.
- **Epoch growth:** Epoch 5 holds 17 entries (4 markerless). The founder ruled no boundary inside Epoch 5.
- **For the founder's desk (left on disk, carried):** `target/ci-38994749835/`, `target/carry5-witness/`,
  `target/control-92264a7/`, `target/p4-live-20261016T211033Z/`, `target/agents-scale-214213/`,
  `target/e2e-home/viola-record-20261016T204612Z/` (tmpfs), an earlier scratchpad's `control/`, `control.tar`, `p4-rig/`.
- **For the founder, his next batch (carried):** the GUI scan forms and the axum `http2` ban; `.claude/rules/a11y.md`,
  `testing.md`, `api.md` / `frontend.md` still glob `e2e-web/**` and `crates/viola-ui/**`; obs-plan §3's Log-format copy differs
  from test-plan's in two older places; `run` step 3 reads "none in 0.1.0"; Brand Identity's L41 / L45 / L16 / L21 cite cut
  material; a11y-plan's tier `Standard (1)`; the interrupted pair's wording; the Windows event set; a refused strict-modes check
  writes no line on `revive`'s arms; the checked `budget.json` read writes no line on a refusal; `list`'s exit-1 shapes and
  per-instance no-row; the five implicit-link shapes; a link line with no string `ts`; a `--home` session shows no user status
  line until `statusline-source.json` is planted; one CLI plan file (2026-10-08) in his user directory.
- **For whoever runs the Epoch 5 boundary audit (carried):** `verify_window_without_screens_fails_every_interactive_row`
  reads 42.98 s against the mutants profile's 45 s kill; CARRY 2's grading of `src/bin/viola-fake-agent.rs` and
  `src/cmd/verify.rs`; CARRY 3's three `viola-state` retry-loop mutants; the end-to-end, machine-contract and output-discipline
  chunks' code; this chunk's `crates/viola-mcp/`, `crates/viola-channel/src/tokio_client.rs` and `src/cmd/mcp.rs` (no
  mutation run was made, the operator's direction).
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8 identity-floor
  row; `v1-34` stays unclaimed.
- **For the operator's word (carried):** the link rule takes a logged `driver` / `driven` as a string; `docs/conventions.md`
  and `docs/stack.md` still list `viola link|unlink`; `v1-33`'s title and `requirements.md:48` say "on Windows"; the ledger
  notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90`; seven sentences of the budget-governor plan word the old hold; a
  live `claude` session ending in this tree rewrites this file's last section; whether `viola-mcp` inherits
  `license.workspace` (build-system's Licence bullet omits it, unread by this wrap).
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`; the host leaf is
  `host-linux.md`.
- **Standing rules (carried):**
  - A removal the permission layer denies is not done another way: a scratch home moves into `target/e2e-home.disk/`; a
    tracked directory is the founder's to remove at the desk.
  - A stalled-start red is a finding about the backing: read `test -L target/e2e-home && findmnt -n -o FSTYPE -T
    target/e2e-home/`, then stop and report.
  - Read `hyprctl locked` before any step that focuses a window or types a key.
  - Before a census run (`bash scripts/profraw-census.sh 4800 48`), write the operator one line.
  - After a `cargo clean`, re-make `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
  - The Bash guard refuses a pipeline tool piped into a filter or sent to a file, a `cd` that moves the cwd, a heredoc to a
    file, and a bare `grep` with a bounded repetition (use `/usr/bin/grep`).
  - A scripted `.rs` write runs rustfmt in the same call.
- **Deferred learnings (carried):** `~/.claude/CLAUDE.md` Shell (a bounded repetition sent to the embedded grep);
  `host-linux.md` Transports (one inline python heredoc carrying document text).
- **Operator desk (the founder's word: leave them, carried):** the earlier chunks' scratch files; gate logs under the OS temp
  dir's `andromeda-gate/`; `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`; live homes and rig
  directories; `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  `~/.viola-record-20261004T142325Z`; the CLI's own transcripts. Yours to delete: `target/e2e-home.disk` and
  `target/profraw-census/`.
- **Last failed command:** none.
