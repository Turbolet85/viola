# The live round (plan.md steps 3–11): both readings taken — 3 starts of the cap 3, start 1 red, starts 2 and 3 green

**Status (2026-10-19T06:18Z):**
- **Precedence (start 2, wrapped):** the `--plugin-dir` server alone started and answered `list`. The installed
  `viola@viola` server never started. The session logged one `session-start` and one SessionStart hook process.
- **Exit-1 (start 3, unwrapped):** a registered `viola mcp` that exits 1 does **not** block the CLI's start. The
  session reached `idle`, and the CLI logged the server as `Connection failed (CONNECTION_CLOSED)`.
- **Start 1 red:** past the trust dialog by the arrow rule, then held by a second modal, `Yes, allow external
  imports`. No key went there (`inputs#I2` F1). The founder then ruled the rig outside the repository (`inputs#I4`
  «Стенд вне репо»), and the two remaining starts were spent there on the two readings.
- Starts made: **3 of 3**. Reds: 1, never re-fired. `claude plugin` commands: rig A 4 of its four, rig B 4 of its fresh
  four.

No prompt text, assistant text, session id, `CLAUDE*` value, absolute path or `detail-*.ndjson` line is in this file.
Times are UTC, read from the clock, on 2026-10-19. Linux dev host, claude 2.1.287 by absolute path
(`~/.local/share/mise/installs/claude/2.1.287/claude --version`: `2.1.287 (Claude Code)`, read before each start).

## Setup

- **The rig:** `live-pty.py` and `live-ledger.py`, copied byte for byte from
  `viola-0.1.0/chunks/2026-10-17-optional-plugin-install/evidence/` (`cmp` exit 0 on each). `live-pty.py` was not run.
  `live-pty-arrow.py` is that chunk's `live-pty-trust.py` with only its key rule replaced. The texts of `declared_env`,
  `Screen`, `stamp` and `write_status` are byte-equal (an `ast` source-segment compare of the two files). The 210×45
  size, the 0.5 s quiet read, the 45 s window and the stop form are unchanged.
- **The key rule's controls:** `rig-controls.py` printed `controls: 10 of 10 as expected` (gate entry 2, green).
  Its first firing during the build read 9 of 10. Control 6 caught a regression: after `unreached` had closed the
  rule, a later read reported `pointer-above` again. The rule was fixed before any start.
- **The verdict words:** besides the plan's words, the rig has `unselected` (the trust row with no single pointer
  row). It means no key and red (F1: "anything else no key and red"). No start read it.
- **The build:** `cargo build --bin viola` (gate entry 3, green, 0.15 s: cargo judged it current). `target/debug/viola`
  dated 2026-10-18T21:24:23Z, sha256 prefix `957313d5fad8ce62`. The only `.rs` files newer than it are two test files
  (`tests/tui_channel_fds.rs`, `tests/support/home.rs`), which do not go into that binary.
- **The environment:** every `claude` command and the rig ran under a declared environment of 8 names (`HOME`,
  `USER`, `LOGNAME`, `SHELL`, `PATH` with the 2.1.287 dir first, `LANG`, `XDG_RUNTIME_DIR`, `TERM=xterm-256color`):
  no `CLAUDE*` name and no `VIOLA*` name. Every viola verb ran under the same environment with `--home` given, so
  `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` were removed.
- **`hyprctl locked`** read `true` before each start (05:32:36Z, 06:13:36Z, 06:15:50Z). It is recorded, and it did not
  stop the round: the rig types only into its own pty, never a desktop key (`inputs#I2` F1; plan.md step 3).
- **The prompt:** one fixed file in each rig's `priv/` (sha256 prefix `b556a602aa0144da`, the plugin chunk's file
  copied). It asks for one call of the viola `list` tool, then a reply of one fixed word.

# Rig A — `target/plugin-live-20261019T053121Z/` (the plan's form): start 1 red

- `proj/` (`git init -q`; `rev-parse --show-toplevel` reads `proj/` itself) and `priv/`, both 0700. The homes' dir
  `target/e2e-home/viola-live-plugin-20261019T053121Z/` (0700; `test -L target/e2e-home`, `findmnt` reads `tmpfs`).
- **Before (05:31Z):** `~/.claude/plugins/` 157 756 files (`priv/plugins-before.json`). `~/.claude/settings.json`
  sha256 prefix `8e7b63f4b538634a`. The repository root's `.claude/settings.local.json`: absent. `~/.claude.json`'s
  verdict for `proj/`: `no-entry`. `~/.claude/projects/`: 106 entries, none naming `proj/`. Census: 0.
- **(a) The install:** `target/debug/viola --home <ihome> plugin install`, human mode (05:31:59Z): exit 0, stderr
  empty. Its first line read `viola@viola  written  <dir>`, `<dir>` equal to `<ihome>/marketplace` as canonical. The
  tree read 5 dirs at 0700 and 3 files at 0600 (`.claude-plugin/marketplace.json`, `viola/.claude-plugin/plugin.json`,
  `viola/.mcp.json`). `viola/.mcp.json` registers the one server `viola`, keys `args` and `command`: `command` equals
  the canonical absolute path of `target/debug/viola`, and `args` equals `["--home", <canonical ihome>, "mcp"]`. No
  `hooks` key, file or dir is anywhere in the tree.
- **Command 1**, `claude plugin marketplace add <dir> --scope local --json` from `proj/` (05:32:10Z): exit 0,
  outcome `ok`, stderr empty. **The guard passed:** `proj/.claude/settings.local.json` exists, the repository root's
  is absent, and `~/.claude/settings.json` is unchanged.
- **Command 2**, `claude plugin install viola@viola --scope local --json` (05:32:17Z): exit 0, outcome `ok`, scope
  `local`. `proj/.claude/settings.local.json` holds `extraKnownMarketplaces` (`viola`) and `enabledPlugins`
  (`viola@viola`).

## Start 1 (n 1 of 3), the precedence reading — RED at a second modal

- Ledgered first, `written_at` 05:32:36Z. From `proj/`: `live-pty-arrow.py <priv> s1 -- target/debug/viola --home
  <whome> run driver -- <claude 2.1.287> --model haiku --allowedTools mcp__plugin_viola_viola`.
- **The rig's keys:** `absent` (0.5 s) → `pointer-above` (1.1 s; `default_offset` **-1**, so the default pointer is one
  row above the trust row) → one down arrow → `selected` (1.6 s) → one Enter (05:32:45.020Z) → **`other-modal`**
  (2.2 s): the `Yes, allow external imports` row on screen. No key after it. arrows 1, enter 1.
- **Readings:** no `session-start` in 90 s. `events.ndjson`: `wheel{holder:"driver", cause:"start"}`, `budget-gate`,
  `wheel{holder:"human", cause:"human-input"}` (the rig's keys moved the wheel, `src/run/wheel.rs:42`). No
  `mcp.ndjson` in either home: no viola MCP server started. `~/.claude.json`'s verdict for `proj/` turned `trusted`,
  the CLI's own write on the accepted dialog. No release and no send were made, since the start had no session.
- **The stop** (stop file 05:35:05Z): 3 presses, exit 0, not killed, 4 117 bytes drained. The CLI fired SessionEnd
  (`hook-invoked{hook_event:"session-end"}`, `events.ndjson` `session-end`), with no SessionStart before it.
  `claude-child` exit 0, `self` exit 0. Census: 0. Not re-fired.
- **The cause, inferred, not read:** `proj/` lies inside this repository's work tree, so the repository's `CLAUDE.md`
  is an ancestor memory of the session. Its one import, `@.claude/session-handoff.md`, resolves outside `proj/`'s git
  root, the external import the second modal asks about. The rig keeps no screen text, so the dialog's subject is not
  recorded.
- **The founder's word (`inputs#I4`, relayed by the operator, 2026-10-19):** «Стенд вне репо»: a fresh `proj/` outside
  this work tree (the e2e-home tmpfs, no ancestor `CLAUDE.md`, its own `git init`), a fresh count of four `claude
  plugin` commands, the remaining 2 starts for the two readings, the arrow + Enter rule kept, and the old `proj/`'s
  plugin removed first with commands 3 and 4.

## Rig A's close

- **Command 3**, `claude plugin uninstall viola@viola --scope local --json` (06:12:20Z): exit 0, outcome `ok`, scope
  `local`, `keptData` false. **Command 4**, `claude plugin marketplace remove viola --scope local --json`
  (06:12:21Z): exit 0, outcome `ok`. `proj/.claude/settings.local.json`: both keys empty.
- **`~/.claude/plugins/` against rig A's before** (157 756 files both): added 0, removed 0, changed 4, all the same
  size: `cache/viola/viola/0.1.0/.mcp.json` and `.orphaned_at` (the CLI's orphaned cache, rewritten by its own
  install and uninstall), and `synced/<id>/.last-complete-round` and `.marketplaces.json` (unattributed, since other
  live sessions run on this host).
- `~/.claude/settings.json`: unchanged. The repository root's `.claude/settings.local.json`: absent. `~/.claude/projects/`:
  106, no new dir (start 1 opened no session).

# Rig B — `proj/` outside the work tree (`inputs#I4`): starts 2 and 3 green

- The rig's `priv/` is `target/plugin-live-20261019T061245Z/priv/` (0700). The homes and `proj/` are under
  `target/e2e-home/viola-live-plugin-20261019T061245Z/` (0700, `tmpfs`). `proj/` resolves through the link to the
  e2e-home base outside the repository, `git init -q`, and `rev-parse --show-toplevel` reads `proj/` itself. Every
  ancestor was read for `CLAUDE.md`, `.claude/CLAUDE.md` and `CLAUDE.local.md`: none present. `~/.claude.json`:
  `no-entry` for `proj/` and each of its ancestors up to the temp dir.
- **Before (06:12Z):** `~/.claude/plugins/` 157 756 files. settings.json `8e7b63f4b538634a`, the root's local file
  absent, `~/.claude/projects/` 106 (none naming `proj/`). Census: 0.
- **(a) The install** (06:13:04Z): exit 0, the printed `<dir>` equal to the canonical `<ihome>/marketplace`, and the tree
  read as in rig A (5 dirs at 0700, 3 files at 0600, `command` the canonical exe, `args` `["--home", <canonical ihome>,
  "mcp"]`, no hooks anywhere).
- **Command 1** (06:13:25Z): exit 0, outcome `ok`. **The guard passed**, the same three readings. **Command 2**
  (06:13:26Z): exit 0, outcome `ok`, scope `local`, the two keys as in rig A.

## Start 2 (n 2 of 3), the precedence reading — GREEN

- Ledgered first, `written_at` 06:13:36Z. From `proj/`: the same command as start 1, home `whome`.
- **The rig's keys:** `pointer-above` (0.8 s; `default_offset` **-1**) → one down arrow → `selected` (1.3 s) → one
  Enter (06:13:46.315Z) → `absent` (2.8 s). arrows 1, enter 1. No second modal.
- **The wheel:** `wheel{holder:"human", cause:"human-input"}` at 06:13:45.810Z, from the rig's keys. Then `viola --home
  <whome> release driver --json` (06:13:56Z): exit 0, `{"v":1,"ok":{"wheel":"driver","budget_paused":false}}`, and
  `wheel{holder:"driver", cause:"release"}` was logged.
- **The send:** `send driver --file <priv>/prompt.txt --json` (06:14:10Z): exit 0, `ok` with `submitted_at` and
  `cursor` 726. `send-issued`, `prompt-submitted` and `send-confirmed` followed. `wait driver --after 726` returned
  `turn-ended` (06:14:16.768Z). `last driver --json`: `last_assistant_message` equals the fixed word (compared by a
  script; the text is not recorded).
- **Readings, per home:**

| reading | `whome` (the `--plugin-dir` plugin) | `ihome` (the installed `viola@viola`) |
|---|---|---|
| `process-start{subject:"self"}` with process `mcp` in `diagnostics/mcp.ndjson` | **1** | **0** (no `diagnostics/` dir at all) |
| the `agents-probe` pair (`process-start` / `process-exit`, child exit 0) during the `list` call | **gained** | none |

- **Which server answered:** the `--plugin-dir` server, `whome`'s. The CLI's own MCP log for the session
  (`~/.cache/claude-cli-nodejs/<proj>/mcp-logs-plugin-viola-viola/`, read for its outcome words only) names **one**
  server, `plugin-viola-viola`, with one connection (`Successfully connected (transport: stdio) in 62ms`) and one
  `Calling MCP tool: list` / `Tool 'list' completed successfully`. Both plugins register the same server name, and
  the CLI started one.
- **One hook process per event:** `events.ndjson` holds 1 `session-start` (cause `startup`). `hook-driver.ndjson` holds
  1 `hook-invoked{hook_event:"session-start"}` and 1 `hook-invoked` each for `user-prompt-submit` and `stop`, plus 2 for
  `post-tool-use` and 6 for `statusline`.
- No `permission` event was raised. `events.ndjson` in all: 3 `wheel`, `budget-gate`, `session-start`, `send-issued`,
  `prompt-submitted`, `send-confirmed`, 2 `activity`, `turn-ended`.
- **The stop** (06:15:17Z): 3 presses, exit 0, not killed, 12 760 bytes drained. `claude-child` exit 0, `self` exit 0.
  The `mcp` server's `mcp.ndjson` holds no `process-exit{subject:"self"}` line: it ended with the session without
  logging its own exit. `~/.claude.json`: `proj/` `trusted` (the CLI's write). Census: 0.

## Start 3 (n 3 of 3), the exit-1 reading — GREEN

- `chmod 0770 <ihome>` (06:15:39Z). **The control:** `target/debug/viola --home <ihome> mcp </dev/null`: exit 1, stdout
  0 bytes, stderr 0 bytes. `<ihome>` still 0770, and `<ihome>/diagnostics/` still absent (`mcp.ndjson` 0 lines before
  and after).
- Ledgered first, `written_at` 06:15:50Z. From `proj/`, unwrapped: `live-pty-arrow.py <priv> s3 -- <claude 2.1.287>
  --model haiku`, so only the installed plugin registers `viola`.
- **The rig's keys:** `absent` at the first read (1.6 s) and throughout. No trust dialog showed (`proj/` was trusted at
  start 2). arrows 0, enter 0, `default_offset` null.
- **The agents read**, 20 s after the first `absent` read (06:16:29Z): one `claude agents --json` by path, from
  `proj/`, killed at 5 s (it exited 0 on its own), stdout to `priv/agents.json`. 12 rows. **The start's `child_pid` has
  a row, status `idle`.** The registered `viola mcp` exiting 1 did not block the start.
- **The CLI's side:** its MCP log for `plugin-viola-viola` (outcome words only): `Starting connection`, a respawn
  after the version-negotiation probe closed the server, then `Connection failed after 101ms (CONNECTION_CLOSED)`
  (one `error` entry).
- **After:** `<ihome>` still 0770, and its `diagnostics/` still absent. The refusal wrote nothing.
- **The stop** (06:16:46Z): 3 presses, exit 0, not killed, 4 460 bytes drained. Census: 0. Then `chmod 0700 <ihome>`
  (read back 700).

## Rig B's close (step 9, step 10)

- **Command 3** (06:17:04Z): exit 0, outcome `ok`, scope `local`, `keptData` false. **Command 4** (06:17:04Z): exit
  0, outcome `ok`. `proj/.claude/settings.local.json`: `enabledPlugins` and `extraKnownMarketplaces`, both empty.
- **`~/.claude/plugins/` against rig B's before** (157 756 files both): added 0, removed 0, changed 4, all the same
  size, the same four paths as rig A: the orphaned cache's `.mcp.json` and `.orphaned_at`, and the unattributed
  `synced/<id>/` pair. The cache's `.mcp.json` now names rig B's `<ihome>`.
- `~/.claude/settings.json`: sha256 prefix `8e7b63f4b538634a`, unchanged. The repository root's
  `.claude/settings.local.json`: absent.
- `~/.claude.json`'s verdict: rig A's `proj/` `no-entry` → `trusted`; rig B's `proj/` `no-entry` → `trusted` (the
  CLI's own writes on the two accepted dialogs).
- `~/.claude/projects/`: 106 → **107**, one dir naming rig B's `proj/` (the transcripts of starts 2 and 3).
- Census (06:16:46Z, after the last stop): 0. Ledger row `round-closed`, 06:17:54Z.
- **The residual, left as found for the founder's desk:** the orphaned `~/.claude/plugins/cache/viola/viola/0.1.0/`
  (rewritten, still 3 files); the unattributed `synced/` pair; the two `~/.claude.json` trust entries; rig B's
  `~/.claude/projects/` dir; the CLI's MCP log dirs under `~/.cache/claude-cli-nodejs/` for rig B's `proj/`. Neither
  viola nor the rig wrote under `~/.claude` or into `~/.claude.json`. The rig and home dirs stay on disk:
  `target/plugin-live-20261019T053121Z/`, `target/plugin-live-20261019T061245Z/`, and on tmpfs
  `target/e2e-home/viola-live-plugin-20261019T053121Z/` and `target/e2e-home/viola-live-plugin-20261019T061245Z/`.

| reading (plan.md steps 6–7) | result |
|---|---|
| which `viola` MCP server or servers a wrapped session holding both plugins started | **the `--plugin-dir` server only**: `whome` 1 `process-start{subject:"self"}`, `ihome` 0 |
| which answered its `list` call | **the `--plugin-dir` server**: `whome`'s `mcp.ndjson` gained the `agents-probe` pair |
| the wrapped session's `session-start` | **1** in `events.ndjson`, **1** `hook-invoked{hook_event:"session-start"}` in `hook-driver.ndjson` |
| whether a registered `viola mcp` that exits 1 blocks an unwrapped start | **no**: the start's row read `idle`; the control exit 1 with both streams empty; `<ihome>` stayed 0770 and its `mcp.ndjson` was never written |
| the rig's keys per start | s1: 1 arrow, 1 Enter, offset -1, then `other-modal` · s2: 1 arrow, 1 Enter, offset -1 · s3: none |
| the live start count, at most 3 | **3** (1 red, not re-fired) |
