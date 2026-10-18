# The live round (plan.md step 7): CLOSED as it stands — 2 starts red at the trust dialog, start 3 unspent

**Status (2026-10-18T02:22Z), closed on the Viola overseer's direction (`inputs#I7`; the founder asleep, his forks
PROVISIONAL):**
- Rig 1 (below) stopped at its guard with no start.
- Rig 2 (`inputs#I5`: `proj/` its own git repo) passed the guard and installed the plugin at `proj/`'s local scope.
- **Start 1 red:** a modal, no key.
- **Start 2 red** (`inputs#I6`: one Enter on a read trust dialog): the trust row was on screen but not selected, so
  no key.
- **Start 3 unspent.** The removal (commands 3-4) ran after the last start.
- Starts made: 2 of 3. `claude plugin` commands: rig 1 used 3 of its four; rig 2 used all 4 of the fresh count.

**None of the plan's readings was taken:** which server starts and answers `list`, the one `session-start`, and
whether an exit-1 `viola mcp` blocks the start. v1-42 stays `planned`. The residual is under *The close*, at the end.

# Rig 1 (2026-10-17): STOPPED at the (a) guard, no start made

**The round stopped after its first `claude plugin` command, by the plan's own stop rule (step 7 (a), Implementation
notes): the local scope did not land in the rig's `proj/.claude/`. The CLI wrote the repository root's
`.claude/settings.local.json` instead. The two removal commands of (d) ran at once; no start was made (0 of the
founder's 2), and 3 of the four `claude plugin` commands were used (`inputs#I2` Q3, Q4; `inputs#I4`).** The finding is
brought to the founder before the round is made again.

No prompt text, assistant text, session id or path content beyond the rig's relative paths is in this file. Times are
UTC, read from the clock, on 2026-10-17. Linux dev host, claude 2.1.287 by absolute path
(`~/.local/share/mise/installs/claude/2.1.287/claude --version`: `2.1.287 (Claude Code)`).

## Setup

- The rig: `live-pty.py` and `live-ledger.py`, copied byte for byte from
  `viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/` (`cmp` exit 0 on each). `live-pty.py` was not run:
  no start was made.
- The rig dir `target/plugin-live-20261017T182031Z/` (`proj/`, `priv/`, both 0700) and the homes'
  dir `target/e2e-home/viola-live-plugin-20261017T182031Z/` (0700, on `tmpfs`: `findmnt` read `tmpfs`). Only `ihome/`
  was made, by the verb itself; `whome/` and `lhome/` were never made.
- The build: `target/debug/viola` was found stale (16:15:14 local, sha256 prefix `fef229f67d76230e`, the earlier
  chunk's build) and rebuilt from this tree by `cargo build --bin viola` at 18:20:41Z, sha256 prefix
  `ee27584342cef430`; `viola plugin --help` read the `install` sub-verb.
- `hyprctl locked` read `false` (18:20:31Z). The `claude plugin` commands ran from `proj/` under a declared
  environment of 8 names (`HOME`, `USER`, `LOGNAME`, `SHELL`, `PATH` with the 2.1.287 dir first, `LANG`,
  `XDG_RUNTIME_DIR`, `TERM=xterm-256color`): no `CLAUDE*` and no `VIOLA*` name. The viola verb ran with `VIOLA_NAME`,
  `VIOLA_DIR` and `VIOLA_BIN` removed and `--home` given.

## Before (18:21Z)

- `~/.claude/plugins/`: 157 812 files listed (relative path, size, sha256) into `priv/plugins-before.json`.
- `~/.claude/settings.json` sha256 prefix `8e7b63f4b538634a`.
- The repository root's `.claude/settings.local.json`: absent.
- Census (executable under the repository's `target/`, or cwd under the rig or home dirs): 0 processes.

## (a) The install, and the guard

- `target/debug/viola --home <ihome> plugin install --json` (18:21:29Z): exit 0, stdout
  `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`. The tree read 5 directories at `drwx------` (the
  home, `marketplace/`, `marketplace/.claude-plugin/`, `marketplace/viola/`, `marketplace/viola/.claude-plugin/`) and 3
  files at `-rw-------`; `.mcp.json` registered `viola` with `command` the rebuilt exe's absolute path and `args`
  `["--home", <ihome as canonical, through the tmpfs link's target>, "mcp"]`.
- Command 1, `claude plugin marketplace add <ihome>/marketplace --scope local --json` (18:21:37Z): exit 0, stderr
  empty, a JSON object with keys `command`, `marketplace`, `message`, `outcome`.
- **The guard, read right after it:**

| guard | reading |
|---|---|
| `proj/.claude/settings.local.json` exists | **no** — `proj/` stayed empty (no `.claude/`) |
| the repository root's `.claude/settings.local.json` still absent | **no** — created at 18:21:37Z (237 bytes, `-rw-r--r--`), its one key `extraKnownMarketplaces` holding `viola` |
| `~/.claude/settings.json` sha256 unchanged | yes (`8e7b63f4b538634a`) |

- **The finding:** on 2.1.287, `--scope local` run from a cwd inside a git work tree writes the local settings file of
  the work tree's root (here this repository's `.claude/settings.local.json`), not of the cwd. The plan's rig premise
  ("from a rig dir under `target/`", `inputs#I2` Q3) placed `proj/` inside the repository's work tree. The file is
  ignored by git (`.gitignore:10`; `git status --short .claude/` listed only the handoff).
- Command 2 (`claude plugin install viola@viola --scope local`) was **not made**.

## (d) The removal, at once (the stop rule)

- Command 3, `claude plugin uninstall viola@viola --scope local --json` (18:21:54Z): exit 1, outcome `failed`,
  `failureCode` `not_installed` — command 2 was never made.
- Command 4, `claude plugin marketplace remove viola --scope local --json` (18:21:55Z): exit 0, outcome `ok`.

## After (18:22Z)

- `~/.claude/plugins/`: 157 812 files; the diff against `priv/plugins-before.json` reads **0 added, 0 removed, 0
  changed**.
- `~/.claude/settings.json` sha256 prefix `8e7b63f4b538634a`: unchanged.
- **The residual:** the repository root's `.claude/settings.local.json` stays, 35 bytes, rewritten at 18:21:55Z to
  `extraKnownMarketplaces` holding an empty object. It was absent before the round. It is left on disk for the
  founder's word (the standing removal rule); it is git-ignored and holds no plugin enablement.
- `proj/`: still empty.
- Census: 0 processes.

## Against the plan's readings

| reading | result |
|---|---|
| which `viola` MCP server or servers a wrapped session holding both plugins started, and which answered `list` | **not read** — no start |
| the wrapped session's one `session-start` | **not read** — no start |
| whether a registered `viola mcp` that exits 1 left an unwrapped start reaching `idle` | **not read** — no start |
| the residual under `~/.claude/plugins/` after the removal, as a diff | read: empty diff; the residual is the repository root's `.claude/settings.local.json` |
| the live start count, at most 2 | **0** |

# Rig 2 (2026-10-18): the guard passed, start 1 red, start 2 held

The founder's own answers (`inputs#I5`, live in the overseer's dialog, 2026-10-18T01:42Z): Q1 «proj/ — свой
git-репо», a fresh rig with `git init` in `proj/`, a FRESH count of four `claude plugin` commands, the two starts
untouched, the same first guard; Q3 «Удалить сейчас». Q2 (the overseer's technical answer): the operator pass waits
for the round.

## Setup

- **Q3, his ruling:** the repository root's `.claude/settings.local.json` (rig 1's residual) was removed at
  2026-10-18T01:44:31Z (`rm` exit 0, read absent after).
- The tree's end state was re-read before the rig: the guard at `src/cmd/plugin.rs:52` is in place; no `.rs` file
  under `src`, `crates` or `tests` is newer than `target/debug/viola` (18:20:41Z, sha256 prefix `ee27584342cef430`,
  the build rig 1 used).
- The rig dir `target/plugin-live-20261018T014442Z/` (`proj/`, `priv/`, 0700), with `git init -q` in `proj/`
  (`rev-parse --show-toplevel` reads `proj/` itself). The homes' dir is `target/e2e-home/viola-live-plugin-20261018T014442Z/`
  (0700, `tmpfs`).
- `hyprctl locked` read `false` before the rig and again before start 1. `claude --version` by path read
  `2.1.287 (Claude Code)` before start 1. The rig's declared environment is rig 1's: 8 names, no `CLAUDE*` / `VIOLA*`.
- The prompt: one fixed file in `priv/` (sha256 prefix `b556a602aa0144da`): call the viola `list` tool once, then reply
  with one fixed word. It was never sent, because no session started.

## Before (01:44Z)

`~/.claude/plugins/`: 157 812 files (`priv/plugins-before.json`). `~/.claude/settings.json` sha256 prefix
`8e7b63f4b538634a`. The repository root's `.claude/settings.local.json`: absent. Census: 0 processes.

## (a) The install, and the guard

- `target/debug/viola --home <ihome> plugin install --json` (01:44:59Z): exit 0, the `ok` document. The tree read 5
  directories at `drwx------` and 3 files at `-rw-------`.
- **Command 1** of the fresh count, `claude plugin marketplace add <ihome>/marketplace --scope local --json`
  (01:45:06Z): exit 0, outcome `ok`, stderr empty.
- **The guard: passed.** `proj/.claude/settings.local.json` exists (237 bytes), the repository root's is absent, and
  `~/.claude/settings.json`'s sha256 is unchanged.
- **Command 2**, `claude plugin install viola@viola --scope local --json` (01:45:13Z): exit 0, outcome `ok`, scope
  `local`. `proj/.claude/settings.local.json` then held the keys `extraKnownMarketplaces` (`viola`) and
  `enabledPlugins` (`viola@viola`). The root's file is still absent, and `settings.json` is unchanged.

## (b) Start 1, the precedence reading — RED

- Ledgered first: `start` n 1, cap 2, authority `inputs#I2` Q4 / `inputs#I5` Q1, `written_at` 01:45:33Z.
- From `proj/`, under `live-pty.py`: `target/debug/viola --home <whome> run driver -- <claude 2.1.287> --model haiku
  --allowedTools mcp__plugin_viola_viola`, started 01:45:41.794Z.
- **Readings:**
  - `run-driver.ndjson`: `process-start` self, the `version-probe` pair (exit 0), `process-start` `claude-child`.
    Nothing more until the stop.
  - `events.ndjson`: `wheel{cause:start}` and `budget-gate` at 01:45:41.917Z, then **no `session-start`** in 60 s of
    waiting (read again at 01:47:02Z: still 2 lines).
  - `<whome>/diagnostics/mcp.ndjson` and `<ihome>/diagnostics/mcp.ndjson`: **both absent**. No `viola mcp` started
    from either plugin.
  - The CLI wrote no `~/.cache/claude-cli-nodejs/` dir for `proj/`.
  - `~/.claude.json` (read only): the repository root's project entry has `hasTrustDialogAccepted: true`; `proj/` has
    **no project entry**.
- **The stop** (stop file at 01:47:19Z): 3 Ctrl-C presses, exit 0, not killed, 1 808 bytes drained in all. The
  wrapper logged `wheel{cause:human-input}`, then `process-exit` `claude-child` with child exit status **1** and
  `process-exit` self exit 0.
- **Census** (01:47:27Z): 0 processes. Ledger row `census`, `live_starts` 1, 01:47:41Z.
- **Reading:** a modal held the start before the CLI's SessionStart hook ran. The plan's rule makes that a red reading,
  since the rig types no key. **Inference, not read** (the rig keeps no screen): it is the trust dialog. With
  `git init`, `proj/` became its own project root, and that root has no trust record, while the trusted root is the
  repository. Not re-fired (`inputs#I2` Q4).

## (c) Start 2 — HELD, not made

Start 2 runs unwrapped from the same `proj/`, so it would meet the same modal. Whether a registered `viola mcp` that
exits 1 blocks the start could then not be told apart from the trust prompt, so the start would be spent with no
reading. Holding it is a judgement brought to the founder, not a plan rule. Starts used: **1 of 2**.

## The state left standing, for the founder's word

- `viola@viola` stays installed at `proj/`'s local scope only (`proj/.claude/settings.local.json`). Commands 3 and 4
  (the uninstall and the marketplace removal) are not yet run: **2 of the fresh four used**.
- `~/.claude/plugins/` against the before listing (01:47Z): added `cache/viola/viola/0.1.0/.claude-plugin/plugin.json`
  (180 B) and `cache/viola/viola/0.1.0/.mcp.json` (253 B). Changed: `installed_plugins.json` (444 → 825 B) and
  `known_marketplaces.json` (554 → 918 B). Changed, same size: two files under one `synced/<id>/` dir
  (`.last-complete-round`, `.marketplaces.json`). Their time (01:45:42Z) coincides with start 1; other live sessions run
  on this host, so they are recorded unattributed. Removed: none.
- `~/.claude/settings.json`: unchanged. The repository root's `.claude/settings.local.json`: absent.


## The founder's second answer, and the rig's extension

`inputs#I6`, the founder's own words, live in the overseer's dialog, 2026-10-18:
- Q1 «Одна клавиша в стенде»: the rig may type ONE Enter into its own claude child's pty to accept the folder-trust
  dialog, only after it reads the screen and finds that dialog. Anything else on screen: no key, the start is red.
  Never a desktop key.
- Q2 «Поднять до 3»: the cap rises to 3 starts (1 used, 2 left: the precedence reading again, then the exit-1 reading).
  A red is still recorded and never re-fired. Plugin commands 3 and 4 run after the last start.

**`live-pty-trust.py`** is `live-pty.py` plus two things, and nothing else changed (the declared environment's
function is byte-equal):
- an in-memory screen (210×45, never written): printable text, CR / LF / BS / TAB, the CSI cursor moves and erases,
  the alternate screen as a clear; colours, modes and OSC strings are dropped;
- at most one `\r` into the pty it owns. It is written only when the output has been quiet for 0.5 s, within 45 s of
  the start, and the screen reads `selected`: one row holds the measured modal literal `Yes, I trust this folder`
  (`crates/viola-agent-claude/src/screen.rs` `SIGNATURES.modals`) with the selection pointer `❯` on that same row, and
  no row holds `Yes, allow external imports`.
- Its verdicts are `selected` · `unselected` (the literal without the pointer on its row) · `other-modal` · `absent`.
  Only the verdict word reaches the status file.
- Controls, before the start (a scratchpad script loading the rig by path): 8 synthetic streams, all as expected.
  They covered the selected dialog whole and fed in 3-byte chunks (split UTF-8 and CSI), the pointer on `No, exit`
  (`unselected`), the dialog erased by EL 2 or ED 2 (`absent`), another modal beside it (`other-modal`), an input box
  alone, and a frame redrawn by cursor-up and EL (`absent`).

## Start 2 (n 2 of 3), the precedence reading again — RED, no key typed

- Ledgered first: `start` n 2, cap 3, home `whome2` (fresh, so its readings stand clear of start 1's), rig
  `live-pty-trust.py`, authority `inputs#I6` Q1, Q2, `written_at` 02:17:32Z.
- Before it: `claude --version` by path read `2.1.287 (Claude Code)`; census 0. **`hyprctl locked` read `true`.** The
  start went ahead on this basis: the standing rule (`host-linux.md` 2026-10-08) stops a step that focuses a window or
  types a key with `wtype`. This start does neither, since its one possible key goes into the rig's own pty and the
  founder ruled "never a desktop key". The plan reads the lock "though the rig types no desktop key".
- From `proj/`: `target/debug/viola --home <whome2> run driver -- <claude 2.1.287> --model haiku --allowedTools
  mcp__plugin_viola_viola`, started 02:17:51.473Z.
- **The rig's read:** `unselected` at every quiet read until the 45 s window closed (02:18:36.510Z). The trust
  dialog's confirm row was on screen, but the pointer was not on its row. **No key typed** (`written_before_stop` 0).
- **Readings:**
  - `run-driver.ndjson`: `process-start` self, the `version-probe` pair (exit 0), `process-start` `claude-child`.
  - `events.ndjson`: `wheel{cause:start}` and `budget-gate` (02:17:51.631Z), then no `session-start` in 75 s.
  - `mcp.ndjson` in `whome2` and in `ihome`: **both absent**.
  - `~/.claude.json`: still no project entry for `proj/`.
- **The stop** (stop file 02:19:29Z): 3 presses, exit 0, not killed. The wrapper logged `wheel{cause:human-input}`,
  then `claude-child` child exit status 1 and self exit 0. Drained **1 808 bytes**, the same count as start 1.
- Census, read right after the stop (02:19:31Z): 0 processes. Ledger row `census`, `live_starts` 2, 02:19:59Z.
- Not re-fired (`inputs#I6` Q2).

## Start 3 (the exit-1 reading) — HELD, not made

It would start unwrapped from the same `proj/` into the same dialog. The rig would type nothing again, so the start
would be red with no exit-1 reading: the CLI started no MCP server in either start while the dialog stood (no
`mcp.ndjson` in any home). Holding it is a judgement brought to the founder, not a rule.

**The open fact:** what the dialog's selected row is on 2.1.287. The rig reads that the confirm row is not it, and
keeps no screen text. The repository's synthetic verify rows (`crates/viola-agent-claude/src/ledger.rs`
`clean_typed`) draw ` ❯ No, exit` above `   Yes, I trust this folder`. If the default selection is `No, exit`, one
Enter would decline trust and exit, so the founder's one-key form cannot accept it.

## The state left standing

- `viola@viola` stays installed at `proj/`'s local scope only; commands 3 and 4 are unrun.
- `~/.claude/plugins/`: as at 01:47Z (the install's two cache files, `installed_plugins.json`,
  `known_marketplaces.json`, and the unattributed `synced/` pair).
- `~/.claude.json`: no project entry for `proj/` (trust never accepted).
- The repository root's `.claude/settings.local.json`: absent.


## The close (`inputs#I7`, the overseer: close as it stands, start 3 unspent, no key, no `~/.claude.json` write, no root local scope)

- **Command 3** (rig 2's count), `claude plugin uninstall viola@viola --scope local --json`, from `proj/`
  (02:22:00Z): exit 0, outcome `ok`, scope `local`, `keptData` false, stderr empty.
- **Command 4**, `claude plugin marketplace remove viola --scope local --json`, from `proj/` (02:22:01Z): exit 0,
  outcome `ok`, stderr empty.
- **`~/.claude/plugins/` against rig 2's before listing** (157 812 files then, 157 815 now):
  - **added:** `cache/viola/viola/0.1.0/.claude-plugin/plugin.json` (180 B), `cache/viola/viola/0.1.0/.mcp.json`
    (253 B; sha256 prefix `1680874dac41d647`, equal to `<ihome>/marketplace/viola/.mcp.json`), and
    `cache/viola/viola/0.1.0/.orphaned_at` (13 B, written by the CLI at the uninstall, 02:22:00Z);
  - **changed, same size:** `synced/<id>/.last-complete-round` and `synced/<id>/.marketplaces.json`, one dir, recorded
    unattributed: their time coincides with a start, and other live sessions run on this host;
  - **removed:** none;
  - `installed_plugins.json` and `known_marketplaces.json` are back to their before bytes.
- **The residual:** the CLI's orphaned plugin cache, `~/.claude/plugins/cache/viola/viola/0.1.0/` (3 files), whose
  `.mcp.json` names the rebuilt exe and `<ihome>`. It is the CLI's own write, left as found for the founder's word,
  since the direction asked for a record, not a removal. Plus the unattributed `synced/` pair.
- `proj/.claude/settings.local.json`: keys `enabledPlugins` and `extraKnownMarketplaces`, both empty.
- `~/.claude/settings.json`: sha256 prefix `8e7b63f4b538634a`, unchanged. The repository root's
  `.claude/settings.local.json`: absent. `~/.claude.json`: no project entry for `proj/` (never written).
- Census (02:22Z): 0 processes. Ledger row `round-closed`.
- The rig and home dirs stay on disk for the founder's desk: `target/plugin-live-20261017T182031Z/`,
  `target/plugin-live-20261018T014442Z/`, `target/e2e-home/viola-live-plugin-20261017T182031Z/` and
  `target/e2e-home/viola-live-plugin-20261018T014442Z/` (on `tmpfs`).

| reading (plan.md step 7) | result |
|---|---|
| which `viola` MCP server or servers a wrapped session holding both plugins started, and which answered `list` | **not read** — starts 1 and 2 never passed the trust dialog |
| the wrapped session's one `session-start` | **not read** |
| whether a registered `viola mcp` that exits 1 left an unwrapped start reaching `idle` | **not read** — start 3 unspent |
| the residual under `~/.claude/plugins/` after the removal, as a diff | read: the orphaned `cache/viola/viola/0.1.0/` (3 files) and an unattributed `synced/` pair |
| the live start count, at most the raised cap of 3 | **2** |
