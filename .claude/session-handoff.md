# Session Handoff

**Last Updated:** 2026-10-19T09:56:16Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the operator pass pushed 3def5b5b and d146c50c; the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-19-dialog-hold-for-the-driver — feat: the configurable dialog hold (`dialog_hold_s` 60–3600, default 60) and the `dialog-expired` record; a 671 s hold answered live on claude 2.1.287

## Position
- Done: **2026-10-19-dialog-hold-for-the-driver**. Gap-list group 2 is closed.
  - The hold is `config.json`'s `dialog_hold_s`; the hook bound is 3605 s and the dialog `timeout` 3615.
  - At expiry the reply is `null` and the wrapper writes one `dialog-expired` line.
  - Live on claude 2.1.287 (Linux): 1 verify (24 pass) and 1 wrapped start; the answer came at 671.3 s.
  - CI is green on d146c50c (ci#39228538322).
- Next: **Overseer switch to viola** (Epoch 5's last entry; it carries the WATCH and this chunk's card row 2) → `/andromeda-phase`.

## Work done
- 14 sources plus the fix commit d146c50c: a held dialog's `pending_dialog` is now written before its line (macOS CI red at 3def5b5b, a pre-existing order). Record: `viola-0.1.0/chunks/2026-10-19-dialog-hold-for-the-driver/report.md`, `evidence/live-round.md` and `evidence/operator-pass.md`.

## Drift resolved
- 37 amendments over all seven masters: 34 detector proposals plus 3 raised.
- 11 widening proposals on one answer card: the `dialog_hold_s` key, the founder's own Q1 (`inputs#I2`).
- The cascade sweep found no stale master text; leaves 21 applied, 2 declined.
- Route: two CARRYs (card rows 1–2), the WATCH migrated, and one citation re-pointed. Record: `.andromeda/runs/2026-10-19T09-28-14-wrap/`.

## Notes
- **PROVISIONAL, for the founder's own word at the Epoch 5 boundary batch (new this wrap):**
  - The answer-card confirm (the Viola overseer's fit check, delegate) of the 11 `dialog_hold_s` proposals (`answer-card.md` of this wrap's run dir).
  - His wording for the `dialog-expired` kind word and its `wait` line, `dialog-expired  <name>  dialog <id>  cursor <n>` (planning forms in design-system cli pattern 3 and layout-templates; the P5 card's row 3, `inputs#I3`).
  - A Domain-status colour for `dialog-expired` (no row in design-system §Color Palette, a 0.2.0 frontend input).
- **PROVISIONAL, carried (same batch):** the previous wrap's answer-card confirm of security-plan §Data Protection's F1 / F2 part; the cli-machine-contract P4 fork answers; the end-to-end chunk's forks 1 and 2 and its E2E regex widening; the unwrapped-sessions card confirm (forks 1, 3, 4); the resume-and-compact fork 2, owed-ledger-rows fork 3 and paste-newline fifth-paste confirms; the cli-output-discipline card confirm, Q2 / Q3 and P2 placements; the mcp wrap's card confirm and Q2 (`ServeError`) / Q3 (`viola-channel`'s `fake-agent = ["tokio"]`); the eighth gap's answer-card confirm; the plugin chunk's answer-card confirm, `inputs#I8`, `inputs#I5` Q2 and `inputs#I7`; the compact chunk's answer-card confirm and `inputs#I6`; start 2 of the plugin chunk made with `hyprctl locked` true.
- **WATCH (carried, re-pinned to "Overseer switch to viola"):** `send_budget_paused…` case_4, 0/3. Non-advancing for the 2nd consecutive time (only nextest ran); a third halts.
- **recurrence-despite-learning (deferred learnings):**
  - host-linux.md 2026-10-07: this time through `gate.py census --args target/debug/viola`; extended at this wrap.
  - events.md 2026-10-16: `DialogSlot::register` wrote `pending_dialog` after the line, met as a macOS-only CI red.
- **Carried:**
  - `viola-0.1.0/vision.md:52` still reads «On the founder's Windows host…» (for route Phase A).
  - The founder's V63 ruling waits for a 0-pending wrap.
  - `viola mcp` ends with no `process-exit{subject:"self"}` line.
  - Every home stamped before chunk 2026-10-18-compact-post-condition-ledger-row reads `cli_verified:false` until its next `viola verify`.
  - `v1-39` un-claimed (its CARRY on "Exit-cause code catalogue"); `v1-34` unclaimed with no entry minted.
- **CARRYs (carried):** "Home and code-bearing file integrity" (the `marketplace/` tree), "Windows-only live measurements" (every 2.1.287 reading's Windows leg, this chunk's long hold now included).
- **Epoch growth:** Epoch 5 holds 18 entries (1 markerless). The founder ruled no boundary inside Epoch 5.
- **For the founder's desk (new):**
  - `target/e2e-home/viola-live-hold-20261019T083728Z/` (tmpfs) and `target/hold-live-20261019T083728Z/`.
  - This round's CLI writes: `~/.claude/projects/` +4 entries, `*viola-verify*` transcripts +5.
- **For the founder's desk (carried):**
  - `target/plugin-live-2026101[789]*`, `target/e2e-home/viola-live-plugin-2026101[789]*` (tmpfs; rig B's `proj/` was used again this chunk).
  - The CLI's `~/.claude.json` trust entries for the plugin rigs' `proj/`, its `~/.claude/projects/` dirs and MCP log dirs, the orphaned plugin cache `~/.claude/plugins/cache/viola/viola/0.1.0/` and an unattributed `~/.claude/plugins/synced/` pair.
  - `target/ci-38994749835/`, `target/carry5-witness/`, `target/control-92264a7/`, `target/p4-live-20261016T211033Z/`, `target/agents-scale-214213/`, `target/e2e-home/viola-record-*` (tmpfs).
  - An earlier scratchpad's `control/`, `control.tar`, `p4-rig/`.
- **For the founder, his next batch (carried):**
  - The GUI scan forms and the axum `http2` ban.
  - `.claude/rules/a11y.md`, `testing.md`, `api.md` / `frontend.md` still glob `e2e-web/**` and `crates/viola-ui/**`.
  - obs-plan §3's Log-format copy differs from test-plan's in two places.
  - `run` step 3 reads "none in 0.1.0"; Brand Identity's cut-material citations; a11y-plan's tier; the interrupted pair's wording; the Windows event set.
  - A refused strict-modes check writes no line on `revive`'s arms; the checked `budget.json` read writes no line on a refusal.
  - `list`'s exit-1 shapes; the five implicit-link shapes; a link line with no string `ts`.
  - A `--home` session shows no user status line until `statusline-source.json` is planted.
  - One CLI plan file (2026-10-08) in his user directory; `plugin install`'s no-line refusal.
- **For whoever runs the Epoch 5 boundary audit (carried):**
  - `verify_window_without_screens_fails_every_interactive_row` against the mutants profile's 45 s kill.
  - CARRY 2's grading of `src/bin/viola-fake-agent.rs` and `src/cmd/verify.rs`; CARRY 3's `viola-state` retry-loop mutants.
  - The code of the end-to-end, machine-contract, output-discipline, mcp, plugin, compact and this dialog-hold chunk (no mutation run, the founder's cap).
- **For the operator's word (carried, one recurred):**
  - The plan template's preservation guard reads red on any recorded companion edit. It recurred at this chunk's gate entry 11 and was resolved in the design (`DialogSlot::hold_for`).
  - The link rule takes a logged `driver` / `driven` as a string.
  - `docs/conventions.md` and `docs/stack.md` list `viola link|unlink`; ledger notes cite a bare `:90`.
  - Seven budget-governor plan sentences word the old hold.
  - A live `claude` session ending in this tree rewrites this file's last section.
  - Whether `viola-mcp` inherits `license.workspace`; test-plan §1's entity list and the `marketplace/` tree.
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`; the host leaf is `host-linux.md`.
- **Standing rules (carried):**
  - A removal the permission layer denies is not done another way. A scratch home moves into `target/e2e-home.disk/`; a tracked directory is the founder's to remove at the desk.
  - A stalled-start red is a finding about the backing: read `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/`, then stop and report.
  - Read `hyprctl locked` before any step that focuses a window or types a key with `wtype`, and stop on `true`.
  - Before a census run (`bash scripts/profraw-census.sh 4800 48`), write the operator one line.
  - After a `cargo clean`, re-make `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
  - The Bash guard refuses:
    - a pipeline tool piped into a filter or sent to a file;
    - a `cd` that moves the cwd;
    - a heredoc to a file;
    - a bare `grep` with a bounded repetition (use `/usr/bin/grep`).

    The write guard refuses a Write under `target/`.
  - A scripted `.rs` write runs rustfmt in the same call. A live rig rebuilds `target/debug/viola` first, and a live rig `proj/` stands outside this work tree.
- **Deferred learnings (carried):**
  - testing.md 2026-10-17 (a `binary(<tui_ stem>)` integration filter selects nothing, read as fails).
  - testing.md 2026-10-07 Ext. 2026-10-16 (a shared helper's count missed by the gate filter).
  - The auto-memory rule on a pipeline tool's stdout.
  - host-linux.md Paths (a `cd X 2>/dev/null; …` prefix).
  - `~/.claude/CLAUDE.md` Shell (bounded repetition).
  - host-linux.md Transports.
- **Operator desk (the founder's word: leave them, carried):** the earlier chunks' scratch files; gate logs under the OS temp dir's `andromeda-gate/`; `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`; live homes and rig directories; `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`; `~/.viola-record-20261004T142325Z`; the CLI's own transcripts. Yours to delete: `target/e2e-home.disk` and `target/profraw-census/`.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-19 20:31:57
