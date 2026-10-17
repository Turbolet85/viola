# The live leg (plan.md step 15): one wrapped claude 2.1.287 driver session

**The one start was made, ledgered before it. The session started viola's MCP server (`process-start`,
`instance` `driver`), but the CLI refused the server's `tools/list` result, so the model had no viola tool: no
`list` and no `wait` call reached the server, and the turn ended without the fixed word. The cause is read from
the CLI's own MCP log: on the negotiated protocol `2026-07-28` a list result must carry `ttlMs` and `cacheScope`,
which the build left out. Fixed after the leg (`tools_result`, unit-pinned with its remove-the-guard reading, and
asserted in `mcp_tools`). The start was not re-fired: the founder's cap for this chunk is one live start
(`inputs#I2` Q2, his dialog 2026-10-17T11:18Z; the operator's direction for this run: "a red or failed start is never
re-fired, report it instead"). The plan's live criterion (`wait` answered `timed-out` between 50 and 60 s, the fixed
word) stays unproven on a live CLI.**

No prompt text, assistant text, session id or path content is in this file or in the ledger: a reading is a code, a
count, a time or an equality. Times are UTC, read from the clock, on 2026-10-17.

## Setup

- `hyprctl locked` read `false` before the first step; the rig types no desktop key and opens no window.
- The rig: `live-pty.py`, `live-ledger.py`, `live-drive.py`, copied byte for byte from
  `viola-0.1.0/chunks/2026-10-16-unwrapped-sessions-on-the-board/evidence/` (`cmp` exit 0 on each). `live-drive.py`
  was not run: it drives the release-check binary and its turn loop answers dialogs, while this leg answers none;
  the verbs were typed directly, against the same build as the sessions.
- The build: `target/debug/viola` of this chunk's tree at 12:20Z (`cargo build -p viola`), sha256 prefix
  `52e4bdd7f874de6f` (its pinned key `0.1.0-52e4bdd7f874de6f`), before the fix below.
- The home: `target/e2e-home/viola-live-mcp-20261017T122017Z/vhome`, fresh and unstamped (no `viola verify` run),
  backed by `tmpfs`. The session dirs: `target/mcp-live-20261017T122017Z/{builder,driver}`, inside the tree the
  founder trusted; no trust dialog came (`session-start` logged).
- The rig's environment: 8 declared names (`HOME`, `LANG`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `USER`,
  `XDG_RUNTIME_DIR`), no `CLAUDE*` and no `VIOLA*` name; `~/.local/share/mise/installs/claude/2.1.287` first on
  `PATH` (`claude --version` by path: `2.1.287 (Claude Code)`; the bare name is 2.1.296 on this host).
- The ledger: `live-sessions.ndjson`, `live-ledger.py` its one writer: row `start` (`written_at` 12:20:50Z) before the
  start, row `census` (12:22:47Z) after it.

## The sessions

| instance | ledgered | started | verb | came up | stop |
|---|---|---|---|---|---|
| `builder` (not a `claude` start) | — | 12:20:38.218 | `run builder -- <the fake agent>` | `live` on `viola list --json` | stop file, 1 Ctrl-C press, exit 0, not killed |
| `driver`, the one live start | 12:20:50 | 12:20:56.046 | `run driver -- claude --model haiku --allowedTools mcp__plugin_viola_viola` | `session-start` cause `startup` at 12:20:57.019 | stop file, 3 presses, exit 0, not killed |

## Readings

- `mcp.ndjson`: one line, `process-start{subject:"self"}`, process `mcp`, instance `driver`, at 12:20:57.174, 155 ms
  after the session-start; no `channel-request`, no `agents-probe` pair, and no `process-exit` (the server's client
  ended it at the stop by a signal, not by closing its stdin).
- The CLI's MCP log for the server (`~/.cache/claude-cli-nodejs/<the driver dir>/mcp-logs-plugin-viola-viola/`, read,
  not copied): connected over stdio in 43 ms, `serverVersion` `viola` `0.1.0`, `negotiatedProtocolVersion`
  `2026-07-28`, `hasTools` true; then `tools/list failed` with two `invalid_*` issues at the paths `ttlMs` (expected a
  number, received nothing) and `cacheScope` (expected `public` or `private`), retried after 250, 500 and 1000 ms, then
  `Failed to fetch tools`. The server's directory names it `plugin-viola-viola`.
- One `viola send driver` from the fixed prompt file (`--json`): exit 0, read back, cursor 445, 36 ms.
- `viola wait driver --after 445`: exit 0, woken by `turn-ended` at 12:21:28.017 (14.8 s after the send), cursor 1962.
  Between them two `activity` lines, tool `ToolSearch` each; no `question`, `permission` or `plan` event: **no
  permission dialog was raised**, so the `--allowedTools` name was never exercised.
- `viola last driver`: exit 0, 86 words, 491 bytes; **not** the fixed word.
- The live start count: **1**, the cap.
- Census at 12:22:4xZ: no process whose executable or working directory lies under this repository, the live home
  or the session dirs.

## Against the plan's step 15 (d)

| reading | result |
|---|---|
| `mcp.ndjson` `process-start{subject:"self"}`, `instance` `driver` | read ✓ |
| a `wait` `channel-request` / `channel-response` pair, outcome `timed-out` | not read: no tool reached the model |
| the wait's duration between 50 and 60 s | not read |
| the turn's end | read ✓ (without the tools) |
| `viola last driver` equal to the fixed word | not read: the message is not the word |
| whether a permission event was raised | read: none |

## The fix, after the leg

`crates/viola-mcp/src/tools.rs` `tools_result()`: `with_ttl_ms(0)` (stale at once) and `with_cache_scope(Private)`.
`tools_result_carries_its_ttl_and_cache_scope` reads `ttlMs` 0 and `cacheScope` `"private"` on the wire; with the two
setters removed it read red (`left: Null, right: 0`) and green once they were restored. `mcp_tools` asserts both on
the rmcp client's parsed result. Neither is a live reading: whether 2.1.287 then lists and calls the five tools is
measured only by a second live start, which needs the founder's word.
