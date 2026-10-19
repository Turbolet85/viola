# Codebase Research — 2026-10-18-plugin-precedence-live-readings

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, 17 Session Additions; applied:
  2026-10-10 (the stamped CLI's install dir first on `PATH`, `claude --version` by path before each start),
  2026-09-29 Ext. 2026-10-07 (a stalled-start red on the tmpfs backing is reported, never re-run), 2026-10-09 (an
  earlier build's hash proves nothing about the code it loads: the round rebuilds and records its build), 2026-09-25
  (never pipe a harness step that leaves a child running). `.claude/rules/host-linux.md` (always loaded): 2026-10-08
  (`hyprctl locked` before any keyed step), 2026-10-07 (a process named `viola` is judged by `/proc/<pid>/exe`).
- **Platform issues consulted:** none — no runner-only bullet (`ci#39172977857` green at P3) and no CI-reading entry
  outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's direction for this take-up (the founder asleep; plan through P4;
  his forks one dialog answered «hold for the founder»; the WATCH folded and its form named for the wrap). One host
  read outside the repository is NOT snapshotted: `~/.claude.json` (the CLI's own config, read only, through
  `$TMPDIR/{scratchpad}/trust_ancestors.py`, which prints only `trusted` / `untrusted` / `no-entry` per ancestor
  path and two counts). It holds the account's credentials, so `inputs.py snap` would commit a verbatim copy of
  them; the reading is recorded below (Measured facts 3) as printed, and the plan's live round re-reads it.

## Files inspected
- `viola-0.1.0/chunks/2026-10-17-optional-plugin-install/evidence/live-round.md` (full) — the failed round: rig 1's
  guard (`:62`–`:70`), rig 2's two red starts (`:140`–`:162`, `:206`–`:226`), the founder's one-Enter ruling and cap 3
  (`:184`–`:189`), the open default-row question (`:234`–`:237`), the close and its residual (`:248`–`:279`).
- `viola-0.1.0/chunks/2026-10-17-optional-plugin-install/plan.md` (`:182`–`:261`) — step 7, the round's shape this chunk
  repeats: the rig, the declared environment, the dirs, before/after, (a) install + guard, (b) the precedence start
  with its `send` / `wait` and its readings, (c) the exit-1 start with its control, (d) the removal, the record.
- `viola-0.1.0/chunks/2026-10-17-optional-plugin-install/evidence/live-pty-trust.py` (`:1`–`:50`, the key sites
  `:193`–`:201`, `:284`–`:290`) — the rig's screen model, the `selected` / `unselected` verdict and its one `\r`.
- `src/cmd/mcp.rs` (full) — `mcp` checks `home_passes` before the log (`:22`), `home_passes` runs `check_path` on the
  home and `diagnostics/` (`:43`–`:50`), the refusal is exit 1 with nothing printed.
- `plugin/.mcp.json` (full) — the `--plugin-dir` server: `command` `@@VIOLA_BIN@@`, `args` `["mcp"]`, no `--home`.
- `src/run/mod.rs` (`:120`–`:130`) — the wrapped child is handed `VIOLA_NAME` and `VIOLA_DIR` (`:125`–`:126`).
- `src/run/wheel.rs` (`:1`–`:75`) — any editing key read from `run`'s stdin moves the wheel to the human
  (`(Input::Human, Wheel::Driver, _)`, `:42`); only `viola release` returns it (`:45`).
- `crates/viola-agent-claude/src/screen.rs` (greps `:37`–`:40`, `:417`–`:425`, `:499`–`:500`) — the measured modal
  literals and the synthetic rows ` ❯ No, exit` / `   Yes, I trust this folder`.
- `crates/viola-agent-claude/src/ledger.rs` (`:1405`–`:1444`) — `signature_rows` keeps a literal row as rendered.
- `fixtures/claude/2.1.287/Screen.modal.json` (full) — the recorded 2.1.287 modal: one kept row, `   Yes, I trust this folder`.
- `crates/viola-e2e/src/harness/boot.rs` / `mod.rs` (greps) — the harness's own build dir is `target/harness`.
- The relays `chunks/2026-10-18-compact-post-condition-ledger-row/inputs/I3-relay-2.md.txt`, `I4-relay-3.md.txt`,
  `I7-relay-1.md.txt` and `chunks/2026-10-17-optional-plugin-install/inputs/I8-relay-1.md.txt` (full) — the moves and
  answers the folded CARRYs cite.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- Rust plane regenerated for this query (6435 nodes / 35753 edges); trace `.andromeda/runs/2026-10-18T22-39-54-phase/tree-query-2026-10-18-plugin-precedence-live-readings.json`, 12 rows.
- **home_passes** — 1 product caller, `mcp` at `src/cmd/mcp.rs:22`; 5 test call sites in `src/cmd/mcp.rs` — the exit-1
  mechanism is one function on one path; no change planned.
- **board_document** — called by `mcp` at `src/cmd/mcp.rs:35` (the `list` tool's provider) — the read the precedence
  `list` call makes; no change planned.
- **marketplace_files** — 1 product caller, `write_tree` at `src/cmd/plugin.rs:114`; 3 unit tests in
  `crates/viola-agent-claude/src/marketplace.rs` — the installed `.mcp.json`'s one writer; no change planned.
- **mcp** — dispatched once, `dispatch` at `src/cmd/mod.rs:191`.

## Patterns detected
- **The attended live round in `evidence/`** (`chunks/2026-10-17-optional-plugin-install/plan.md:182`–`:261`): a rig
  copied byte for byte, a declared 8-name environment, a start ledger written before each start, codes and counts only
  in the record, the rig and home dirs left for the founder's desk.
- **A key typed only on a read screen** (`live-pty-trust.py:193`–`:201`, `:284`–`:285`): the verdict is computed from
  the rig's in-memory screen, the key written once, only on `selected`, only within its window.
- **The CLI's own removal** (`live-round.md:250`–`:261`): `claude plugin uninstall` then `marketplace remove` restore
  `installed_plugins.json` and `known_marketplaces.json`; the cache stays orphaned.

## Measured facts
1. **The trust dialog's default row is not its confirm row (two screen models).** The rig's own model read
   `unselected` at every quiet read of start 2 (`live-round.md:216`–`:217`). Independently, `viola verify`'s Run A
   recorded the 2.1.287 modal through the vt100 model (`fixtures/claude/2.1.287/Screen.modal.json`, committed at
   `012fc50`): its one kept row is `   Yes, I trust this folder` with three leading blanks and no `❯`, and
   `signature_rows` keeps a literal row as rendered (`ledger.rs:1420`–`:1430`). Which row holds the pointer is not
   recorded (non-literal rows are written `""`), and the row carries no number, so a digit key has no recorded basis.
2. **The two servers log into two homes.** The installed `.mcp.json` passes `--home <ihome>`; `resolved_home` takes
   `--home` first, so that server logs into `<ihome>/diagnostics/mcp.ndjson`. The `--plugin-dir` server has `args`
   `["mcp"]` (`plugin/.mcp.json`) and reaches the wrapped home through `VIOLA_DIR` (`src/run/mod.rs:126`). Both inherit
   `VIOLA_NAME` from the wrapped child (`:125`), so `instance` is equal on both and only the file attributes a line.
3. **Only the repository root is trusted.** `$TMPDIR/{scratchpad}/trust_ancestors.py` over `~/.claude.json`, for the
   ancestors of the plugin round's `proj/` (2026-10-18T22:47Z): `~/dev/projects/viola` `trusted`; `~` `untrusted`;
   every other ancestor `no-entry`; 0 entries under `/tmp`; 16 entries in all. A rig outside the work tree has no
   trusted ancestor, and a rig inside it takes the repository root as its project (rig 1), so the "already trusted
   root" method means the repository's own `.claude/settings.local.json`.
4. **A key on the wrapped start moves the wheel.** The precedence start runs under `viola run`, so a key the rig types
   reaches the child through `run`'s stdin and is a human editing key (`wheel.rs:42`; not on the closed non-editing
   list, `wheel.rs:4`–`:5`). The driver's `send` then refuses `human-typing` (exit 10) until `viola release driver`
   (`wheel.rs:45`). The exit-1 start is unwrapped, so a key there touches no wheel.
5. **The exit-1 refusal writes nothing.** `home_passes` refuses a 0770 home before `viola_obs_init` (`mcp.rs:22`–`:27`):
   exit 1, empty stdout and stderr, no role file, so `<ihome>/diagnostics/mcp.ndjson` gains no line and the home's
   mode stays as found. Any viola process that opens a log in `<ihome>` after the chmod (an install under a name, the
   precedence server) would narrow it to 0700 first (the 2026-10-10 trap), so the order is precedence start, stop,
   chmod 0770, control, exit-1 start.

## Conventions to follow
- **Codes and counts only in `evidence/`**: no prompt or assistant text, session id, `CLAUDE*` value, absolute path or
  `detail-*.ndjson` line (`live-round.md:24`–`:26`; security-plan §Secret Management; obs-plan §8).
- **Rebuild first**: `cargo build --bin viola` and record the build's sha256 prefix before any start (`live-round.md:36`–`:38`).
- **The 2.1.287 binary by absolute path**, its install dir first on `PATH`, `--version` read by path before each start
  (verification-harness.md 2026-10-10).

## New files to create
- `viola-0.1.0/chunks/2026-10-18-plugin-precedence-live-readings/evidence/` — the round's record, the start ledger and the rig scripts

## Files to modify
- none

## Sweeps
- none — no name sweep ran (no product symbol changes)

## Open questions
- The method past the folder-trust dialog (a pointer move + Enter on a read screen · a `~/.claude.json` trust record ·
  the repository root's local scope · no live round) → blocks: plan-decision (the founder's P4 fork, held under
  `inputs#I1`).
- The live `claude` start cap and the `claude plugin` command count for the round → blocks: plan-decision (the
  founder's P4 fork, held under `inputs#I1`).
- Whether the confirm row is one pointer move from the default row → blocks: implementation-scope (read by the rig on
  screen at start 1, only under the key method).
