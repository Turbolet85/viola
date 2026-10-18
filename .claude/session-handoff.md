# Session Handoff

**Last Updated:** 2026-10-18T03:10:30Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-17-optional-plugin-install — viola plugin install: the MCP-only local-marketplace plugin for unwrapped drivers; the live round closed with its readings unread

## Position
- Done: **2026-10-17-optional-plugin-install** — `viola plugin install [--json]` writes the MCP-only marketplace `viola`
  (plugin `viola@viola`, `.mcp.json` on the canonical exe with `["--home", <home>, "mcp"]`, built by `serde_json`, no hooks)
  under `<home>/marketplace/` after its strict-modes check, prints the CLI's two commands, spawns no `claude`; `setup:` serves
  it. Surfaced: the live round took none of its readings; `v1-42` un-claimed with a dated note (`inputs#I8`). Wrapped in
  `.andromeda/runs/2026-10-18T02-36-56-wrap/`.
- Next: **Compact post-condition ledger row** (the first markerless entry) → `/andromeda-phase`.

## Work done
- Every local gate green; operator pass: pre-CI commit `43c64109`, CI green 15/15 (`ci#39090008812`, attempt 1). The live
  round: rig 1 stopped at its guard (`--scope local` landed at the repository root), rig 2 (`proj/` its own git repo) passed it,
  starts 1 and 2 red at the folder-trust dialog (no key typed), start 3 unspent, all four `claude plugin` commands of each rig's
  count run and removed. Record: `chunks/2026-10-17-optional-plugin-install/evidence/live-round.md`.

## Drift resolved
- 43 amendments over all seven masters and one key file (38 fan-out proposals + 5 raised by the wrap, 0 rejected); one halt,
  the answer card (security-plan's live-round residual, inside the founder's own `inputs#I2` Q3 / `#I5` Q1), confirmed by the
  Viola overseer — PROVISIONAL. 25 leaf findings: 24 by the tool, 1 by hand. Record: `fanout-results.md`,
  `cascade-dispositions.md`.

## Notes
- **PROVISIONAL, for the founder's own word at the Epoch 5 boundary batch (new this wrap):** the answer-card confirm
  (security-plan's live-round residual); the overseer's route-resolve answer `inputs#I8` (v1-42 un-claimed; the two unread
  readings and the trust-dialog method a CARRY on "Compact post-condition ledger row"); the overseer's `inputs#I5` Q2 and
  `inputs#I7` (the round closed as it stands); start 2 made with `hyprctl locked` reading `true` (no desktop key in it).
- **The founder's own words this chunk:** `inputs#I2` (four P4 answers), `#I5` (rig 2; rig 1's residual removed), `#I6` (one
  Enter on a read trust dialog — never typed: the dialog's confirm row was not selected; cap raised to 3).
- **For the founder's desk (new this wrap):** the CLI's orphaned plugin cache `~/.claude/plugins/cache/viola/viola/0.1.0/`
  (3 files) and an unattributed `~/.claude/plugins/synced/` pair; the rig and home dirs `target/plugin-live-20261017T182031Z/`,
  `target/plugin-live-20261018T014442Z/`, `target/e2e-home/viola-live-plugin-20261017T182031Z/` and
  `…-20261018T014442Z/` (tmpfs). An open question for him: which row 2.1.287's trust dialog selects by default.
- **CARRYs this wrap:** "Compact post-condition ledger row" (the unread readings, the trust-dialog method, `inputs#I8`),
  "Home and code-bearing file integrity" (the `marketplace/` tree, the P5 card row 2) and "Windows-only live measurements"
  (the two readings' Windows legs, the P5 card row 1). The Windows block re-verified standing (`uname` reads Linux).
- **PROVISIONAL / NOT ratified, carried:** the mcp wrap's Q2 (`ServeError` an open divergence) and Q3 (`viola-channel`'s
  `fake-agent = ["tokio"]`); `viola mcp`'s and every CLI verb's home through `resolved_home` (the `VIOLA_DIR` step and the
  canonicalisation); the eighth gap's answer-card confirm.
- **`v1-39` un-claimed (carried):** held by its CARRY on "Exit-cause code catalogue".
- **Provisional, for the founder's own word at the Epoch 5 boundary batch (carried):** the cli-machine-contract P4 fork answers;
  the end-to-end chunk's forks 1 and 2 and its E2E regex widening; the unwrapped-sessions card confirm (forks 1, 3, 4); the
  resume-and-compact fork 2, owed-ledger-rows fork 3 and paste-newline fifth-paste confirms; the cli-output-discipline card
  confirm, its Q2 / Q3 and its P2 placements; the mcp wrap's card confirm and Q2 / Q3.
- **Every home stamped before chunk 2026-10-16-unwrapped-sessions-on-the-board reads `cli_verified:false`** until its next
  `viola verify`.
- **Epoch growth:** Epoch 5 holds 17 entries (3 markerless). The founder ruled no boundary inside Epoch 5.
- **For the founder's desk (left on disk, carried):** `target/ci-38994749835/`, `target/carry5-witness/`,
  `target/control-92264a7/`, `target/p4-live-20261016T211033Z/`, `target/agents-scale-214213/`,
  `target/e2e-home/viola-record-20261016T204612Z/` (tmpfs), an earlier scratchpad's `control/`, `control.tar`, `p4-rig/`.
- **For the founder, his next batch (carried):** the GUI scan forms and the axum `http2` ban; `.claude/rules/a11y.md`,
  `testing.md`, `api.md` / `frontend.md` still glob `e2e-web/**` and `crates/viola-ui/**`; obs-plan §3's Log-format copy differs
  from test-plan's in two older places; `run` step 3 reads "none in 0.1.0"; Brand Identity's L41 / L45 / L16 / L21 cite cut
  material; a11y-plan's tier `Standard (1)`; the interrupted pair's wording; the Windows event set; a refused strict-modes check
  writes no line on `revive`'s arms; the checked `budget.json` read writes no line on a refusal; `list`'s exit-1 shapes and
  per-instance no-row; the five implicit-link shapes; a link line with no string `ts`; a `--home` session shows no user status
  line until `statusline-source.json` is planted; one CLI plan file (2026-10-08) in his user directory; `plugin install`'s
  no-line refusal under exemption 7 (its plan's lean, the operator's at P5).
- **For whoever runs the Epoch 5 boundary audit (carried):** `verify_window_without_screens_fails_every_interactive_row`
  reads 42.98 s against the mutants profile's 45 s kill; CARRY 2's grading of `src/bin/viola-fake-agent.rs` and
  `src/cmd/verify.rs`; CARRY 3's three `viola-state` retry-loop mutants; the end-to-end, machine-contract, output-discipline and
  mcp chunks' code; this chunk's `crates/viola-agent-claude/src/marketplace.rs`, `src/cmd/plugin.rs` and `src/human/verbs.rs`
  (no mutation run was made, the founder's cap).
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8 identity-floor
  row; `v1-34` stays unclaimed.
- **For the operator's word (carried):** the link rule takes a logged `driver` / `driven` as a string; `docs/conventions.md`
  and `docs/stack.md` still list `viola link|unlink`; `v1-33`'s title and `requirements.md:48` say "on Windows"; the ledger
  notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90`; seven sentences of the budget-governor plan word the old hold; a
  live `claude` session ending in this tree rewrites this file's last section; whether `viola-mcp` inherits
  `license.workspace`; test-plan §1's "config and code-bearing state" entity does not list the `marketplace/` tree.
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`; the host leaf is
  `host-linux.md`.
- **Standing rules (carried):**
  - A removal the permission layer denies is not done another way: a scratch home moves into `target/e2e-home.disk/`; a
    tracked directory is the founder's to remove at the desk.
  - A stalled-start red is a finding about the backing: read `test -L target/e2e-home && findmnt -n -o FSTYPE -T
    target/e2e-home/`, then stop and report.
  - Read `hyprctl locked` before any step that focuses a window or types a key with `wtype`, and stop on `true`
    (`host-linux.md` 2026-10-08).
  - Before a census run (`bash scripts/profraw-census.sh 4800 48`), write the operator one line.
  - After a `cargo clean`, re-make `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
  - The Bash guard refuses a pipeline tool piped into a filter or sent to a file, a `cd` that moves the cwd, a heredoc to a
    file, and a bare `grep` with a bounded repetition (use `/usr/bin/grep`); the write guard refuses a Write under `target/`.
  - A scripted `.rs` write runs rustfmt in the same call.
  - A live rig rebuilds `target/debug/viola` first: no gate builds it.
- **Deferred learnings:** `recurrence-despite-learning: host-linux.md Paths & the working directory` (a `cd X 2>/dev/null; …`
  prefix moved the session cwd once this wrap); carried: `~/.claude/CLAUDE.md` Shell (a bounded repetition sent to the
  embedded grep); `host-linux.md` Transports (one inline python heredoc carrying document text).
- **Operator desk (the founder's word: leave them, carried):** the earlier chunks' scratch files; gate logs under the OS temp
  dir's `andromeda-gate/`; `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`; live homes and rig
  directories; `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  `~/.viola-record-20261004T142325Z`; the CLI's own transcripts. Yours to delete: `target/e2e-home.disk` and
  `target/profraw-census/`.
- **Last failed command:** none.
