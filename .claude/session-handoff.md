# Session Handoff

**Last Updated:** 2026-10-19T19:47:30Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the operator pass pushed aa965127; the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-19-overseer-switch-to-viola — feat: the overseer drives viola-builder through the product viola; `question` carries `option_details`, `turn-ended` carries `background_tasks`; v1-33 proven live on claude 2.1.296

## Position
- Done: **2026-10-19-overseer-switch-to-viola**. Epoch 5 is closed.
  - The viola overseer drives viola-builder through the product binary (sha256 `231c0225…`), on claude 2.1.296 stamped 24/24.
  - The full pipeline was read live: a skill sent, the turn waited, `last`, a review dialog answered, and `/clear` between skills.
  - The takeover: the founder's own key took the wheel, the next send exited 10, and his own `release` returned it.
  - CI is green on aa965127 (ci#39305131020, 15/15).
- Next: **Newer claude CLI rows** (minted at the Epoch 6 head on the founder's word: the four rows claude 2.1.302 fails) → `/andromeda-phase`.
  - The Epoch 5 boundary skills (diagnose and audit) come first if wanted.

## Work done
- Six sources: `question` data gains `option_details` (`{description, preview}` per option, string or `null`); `turn-ended` data gains `background_tasks` (a count).
- The switch's eight acts and its 15 live starts (the founder's cap, raised 8 → 14 → 15) are recorded in `evidence/switch-readings.ndjson` and `evidence/switch-round.md`. The record: `viola-0.1.0/chunks/2026-10-19-overseer-switch-to-viola/report.md`.

## Drift resolved
- 16 amendments over five masters (architecture 5, security-plan 5, test-plan 4, obs-plan 1, a11y-plan key file 1): 12 detector proposals plus 4 raised.
- One answer card (five proposals, the option-text widening): confirmed by the founder's own word (`inputs#I21`).
- Disproved and fixed: `dialog-answered`'s `from` / `from_trust` (obs-plan §4).
- Newly recorded: the verify print probe's empty `~/.claude/projects` dir, accepted by the founder's own word.
- Whole-mode leaves (Epoch 5's close): 39 findings, 30 applied, 1 by hand, 8 declined (the stale frontmatter globs, already on the founder's batch below).
- Record: `.andromeda/runs/2026-10-19T19-16-36-wrap/`.

## Notes
- **PROVISIONAL, for the founder's own word at the Epoch 5 boundary batch (new this wrap):**
  - The obs criterion "no prompt text under the chunk folder" read as excluding `inputs/` (`I1`, `I15`, `I17` hold directive texts): the overseer's technical answer, a delegate's (`inputs#I21`).
  - The leaf path check's extra file `route-no-marker.json` (the orchestrator's own route-read trail written between the snapshots): ruled "accept, recorded" by the overseer's technical answer.
  - The light gate's `pre-push` red (2958/2958 tests passed; `llvm-cov` refused one truncated `viola` `.profraw`, a recurrence of the class open in `evidence/profraw-red.md`) ruled "not this chunk's, re-watch" by the overseer's technical answer (`inputs#I24`); the WATCH on "Newer claude CLI rows" holds it, 0/3. The Epoch 5 boundary's diagnosis and audit weigh a corrective owner.
- **The light gate's first firing (measured, fixed by the founder):** integration, e2e and `pre-push` went red on `Disk quota exceeded (os error 122)` at `viola verify`'s pinned-copy write: the `/tmp` tmpfs has `usrquota`, and parallel test homes pin a ~100 MB debug `viola` each. 22 GB of the user's `/tmp` was Claude Code's own `/tmp/claude-1000`. The founder freed it himself (`inputs#I23`; `/tmp` 24 G → 13 G), and the re-run read integration and e2e green. A gate that boots stamped homes needs that headroom.
- **PROVISIONAL, carried (same batch):**
  - The dialog-hold chunk's answer-card confirm (11 `dialog_hold_s` proposals), its `dialog-expired` wording and colour.
  - The previous wrap's answer-card confirm of security-plan §Data Protection's F1 / F2 part.
  - The cli-machine-contract P4 fork answers.
  - The end-to-end chunk's forks 1 and 2 and its E2E regex widening.
  - The unwrapped-sessions card confirm (forks 1, 3, 4).
  - The resume-and-compact fork 2, owed-ledger-rows fork 3 and paste-newline fifth-paste confirms.
  - The cli-output-discipline card confirm, Q2 / Q3 and P2 placements.
  - The mcp wrap's card confirm and Q2 / Q3.
  - The eighth gap's answer-card confirm.
  - The plugin chunk's answer-card confirm and its inputs.
  - The compact chunk's answer-card confirm and `inputs#I6`.
  - Start 2 of the plugin chunk made with `hyprctl locked` true.
- **Route (this wrap):**
  - The case_4 WATCH is converted to a CARRY on "Windows endpoint admission" (the founder's word): run it under plain `cargo test` ×3 or fix its log capture.
  - A new WATCH on "Newer claude CLI rows": the open `pre-push` coverage-merge red (a `viola hook`'s truncated `.profraw`, killer unknown; `evidence/profraw-red.md`), 0/3.
  - The 3600 s ceiling's first live reading moved to "Version done-check" (card row 2). The Windows readings of 2.1.296's stamp and the two keys are on "Windows-only live measurements" (card row 1).
- **The overseer's driving facts (outside this repo):** the overseer pins viola-builder to claude 2.1.296 through a PATH shim, and its own home `~/.viola` holds a 2.1.302 stamp at 20/4 beside 2.1.296's 24/24. Its answering process carries no `VIOLA_NAME`, so its answers record no `from`.
- **recurrence-despite-learning (deferred learnings):**
  - host-linux.md §Paths: the Bash guard refused a `cd` into a chunk or run dir twice (implement step 12, this wrap's fan-out read).
  - host-linux.md 2026-09-28 Ext.: a `cat >> file <<EOF` append refused at the site sweep.
  - The auto-memory rule on a pipeline tool's stdout: a gate tool piped into `tail` refused at the leaf apply.
- **Carried:**
  - `viola-0.1.0/vision.md:52` still reads «On the founder's Windows host…» (for route Phase A).
  - The founder's V63 ruling waits for a 0-pending wrap.
  - `viola mcp` ends with no `process-exit{subject:"self"}` line.
  - Every home stamped before chunk 2026-10-18-compact-post-condition-ledger-row reads `cli_verified:false` until its next `viola verify`.
  - `v1-39` un-claimed (its CARRY on "Exit-cause code catalogue"); `v1-34` unclaimed with no entry minted.
- **CARRYs (carried):** "Home and code-bearing file integrity" (the `marketplace/` tree); "Windows-only live measurements" (every Linux-only reading's Windows leg).
- **For the founder's desk (new):**
  - Five probe homes of this wrap's quota diagnosis moved into `target/e2e-home.disk/` (`wrap-probe-*`, `wrap-probe2-*`, `wrap-probe3-*`, `wrap-par-*`, `wrap-par2-*`): a recursive `rm` is refused by the host's deny rule.
  - The fake agent's `target/harness/debug/sessions/` registry holds 16 930 files (one per fake start, never removed).
  - This chunk's CLI writes: `~/.claude/projects/` 111 → 120 entries, `*viola-verify*` files 70 → 78 (two verifies, 2.1.296 and 2.1.302).
  - `~/.viola` is now the product's live home (viola-builder's instance, a `config.json` with `dialog_hold_s` 3600, the two stamps).
- **For the founder's desk (carried):**
  - `target/e2e-home/viola-live-hold-20261019T083728Z/` (tmpfs) and `target/hold-live-20261019T083728Z/`.
  - `target/plugin-live-2026101[789]*`, `target/e2e-home/viola-live-plugin-2026101[789]*` (tmpfs).
  - The CLI's `~/.claude.json` trust entries for the plugin rigs' `proj/`, its `~/.claude/projects/` dirs and MCP log dirs, the orphaned plugin cache `~/.claude/plugins/cache/viola/viola/0.1.0/` and an unattributed `~/.claude/plugins/synced/` pair.
  - `target/ci-38994749835/`, `target/carry5-witness/`, `target/control-92264a7/`, `target/p4-live-20261016T211033Z/`, `target/agents-scale-214213/`, `target/e2e-home/viola-record-*` (tmpfs).
  - An earlier scratchpad's `control/`, `control.tar`, `p4-rig/`.
- **For the founder, his next batch (carried):**
  - The GUI scan forms and the axum `http2` ban.
  - `.claude/rules/a11y.md`, `testing.md`, `api.md` / `frontend.md` still glob `e2e-web/**` and `crates/viola-ui/**` (this wrap's whole-mode comparers flagged the same 7 globs again).
  - obs-plan §3's Log-format copy differs from test-plan's in two places.
  - `run` step 3 reads "none in 0.1.0"; Brand Identity's cut-material citations; a11y-plan's tier; the interrupted pair's wording; the Windows event set.
  - A refused strict-modes check writes no line on `revive`'s arms; the checked `budget.json` read writes no line on a refusal.
  - `list`'s exit-1 shapes; the five implicit-link shapes; a link line with no string `ts`.
  - A `--home` session shows no user status line until `statusline-source.json` is planted.
  - One CLI plan file (2026-10-08) in his user directory; `plugin install`'s no-line refusal.
- **For whoever runs the Epoch 5 boundary audit (carried, + this chunk):**
  - `verify_window_without_screens_fails_every_interactive_row` against the mutants profile's 45 s kill.
  - CARRY 2's grading of `src/bin/viola-fake-agent.rs` and `src/cmd/verify.rs`; CARRY 3's `viola-state` retry-loop mutants.
  - The code of the end-to-end, machine-contract, output-discipline, mcp, plugin, compact, dialog-hold and this switch chunk (no mutation run, the founder's cap).
- **For the operator's word (carried):**
  - The plan template's preservation guard reads red on any recorded companion edit.
  - The link rule takes a logged `driver` / `driven` as a string.
  - `docs/conventions.md` and `docs/stack.md` list `viola link|unlink`; ledger notes cite a bare `:90`.
  - Seven budget-governor plan sentences word the old hold.
  - A live `claude` session ending in this tree rewrites this file's last section.
  - Whether `viola-mcp` inherits `license.workspace`; test-plan §1's entity list and the `marketplace/` tree.
  - `cascade.py leaves --apply` reads a backticked `a|b` inside a list item as a table cell (`reshaped`).
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
  - Between a fan-out's two snapshots, no `{tools_dir}` call with `--run-dir` (a read verb writes its trail too).
- **Deferred learnings (carried):**
  - testing.md 2026-10-17 (a `binary(<tui_ stem>)` integration filter selects nothing, read as fails).
  - testing.md 2026-10-07 Ext. 2026-10-16 (a shared helper's count missed by the gate filter).
  - host-linux.md Paths (a `cd X 2>/dev/null; …` prefix).
  - `~/.claude/CLAUDE.md` Shell (bounded repetition).
  - host-linux.md Transports.
- **Operator desk (the founder's word: leave them, carried):** the earlier chunks' scratch files; gate logs under the OS temp dir's `andromeda-gate/`; `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`; live homes and rig directories; `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`; `~/.viola-record-20261004T142325Z`; the CLI's own transcripts. Yours to delete: `target/e2e-home.disk` and `target/profraw-census/`.
- **Last failed command:** none.
