# The live leg (plan.md step 15): wrapped claude 2.1.287 driver sessions

**Two live starts were made, each ledgered before it, the chunk's cap: one (`inputs#I2` Q2), raised to two by the
founder's own word «Ещё один старт» (`inputs#I4`, live in the overseer's dialog, 2026-10-17T14:14Z).**

- **Start 1 (12:20Z, the first build): red.** The session started viola's MCP server, but the CLI refused its
  `tools/list` result — on the negotiated protocol `2026-07-28` a list result must carry `ttlMs` and `cacheScope`,
  which that build left out — so no tool reached the model. Not re-fired. Fixed in `tools_result()`.
- **Start 2 (14:16Z, the fixed build): green.** The session listed the five tools and called two of them: `list`
  (the server's one `claude agents --json` read) and `wait` for `builder` with no `timeout_ms`, which went out as one
  `wait` frame with `timeout_ms` 50000 and was answered `timed-out` after 50 012 ms; the turn then ended with the
  fixed word. No permission dialog was raised.

No prompt text, assistant text, session id or path content is in this file or in the ledger: a reading is a code, a
count, a time or an equality. Times are UTC, read from the clock, on 2026-10-17.

## Setup (both starts)

- `hyprctl locked` read `false` before each leg; the rig types no desktop key and opens no window.
- The rig: `live-pty.py`, `live-ledger.py`, `live-drive.py`, copied byte for byte from
  `viola-0.1.0/chunks/2026-10-16-unwrapped-sessions-on-the-board/evidence/` (`cmp` exit 0 on each, read again before
  start 2 for the two run). `live-drive.py` was not run: it drives the release-check binary and its turn loop answers
  dialogs, while this leg answers none; the verbs were typed directly, against the sessions' own build.
- The rig's environment: 8 declared names (`HOME`, `LANG`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `USER`,
  `XDG_RUNTIME_DIR`), no `CLAUDE*` and no `VIOLA*` name; `~/.local/share/mise/installs/claude/2.1.287` first on
  `PATH` (`claude --version` by path: `2.1.287 (Claude Code)`, read before each start; the bare name is 2.1.296 on
  this host). The product verbs ran with `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` removed and `--home` given.
- Each start had a fresh, unstamped home (no `viola verify` run) under `target/e2e-home/` (backed by `tmpfs`) and its
  own session dirs `target/mcp-live-<stamp>/{builder,driver}`, inside the tree the founder trusted; no trust dialog
  came (each `session-start` logged). `builder` ran the fake agent (`target/harness/debug/viola-fake-agent`), not a
  `claude` start.
- The prompt: one fixed file in the private dir, the same bytes for both starts (sha256 prefix `26d2cc7a83401066`):
  call the viola `list` tool, then its `wait` tool for `builder` with no other argument, then reply with one fixed word.
- The ledger: `live-sessions.ndjson`, `live-ledger.py` its one writer, each row's `written_at` the clock's: `start` n 1
  (12:20:50Z), `census` (12:22:47Z), `start` n 2 cap 2 authority `inputs#I4` (14:15:52Z), `census` (14:17:38Z).

| | start 1 | start 2 |
|---|---|---|
| build | `target/debug/viola` at 12:20Z, sha256 prefix `52e4bdd7f874de6f` (before the fix) | `target/debug/viola` at 14:15:14Z, sha256 prefix `fef229f67d76230e`, the tree of commit `a902e6e4` (`git diff --quiet a902e6e -- crates src plugin Cargo.toml Cargo.lock`) |
| home | `viola-live-mcp-20261017T122017Z/vhome` | `viola-live-mcp-20261017T141514Z/vhome` |
| `builder` up | 12:20:38.218, `live` | 14:15:45.247, `live` |
| `driver` ledgered | 12:20:50 | 14:15:52 |
| `driver` started | 12:20:56.046; `session-start` `startup` 12:20:57.019 | 14:16:01.707; `session-start` `startup` 14:16:02.638 |
| stop (stop file) | driver 3 presses, builder 1; exit 0, not killed | driver 3 presses, builder 1; exit 0, not killed |

## Start 1 — readings

- `mcp.ndjson`: `process-start{subject:"self"}`, process `mcp`, instance `driver`, 12:20:57.174; nothing else.
- The CLI's MCP log for the server (`~/.cache/claude-cli-nodejs/<the driver dir>/mcp-logs-plugin-viola-viola/`, read,
  not copied): connected over stdio in 43 ms, `negotiatedProtocolVersion` `2026-07-28`; then `tools/list failed` with
  `invalid_*` issues at `ttlMs` (expected a number) and `cacheScope` (expected `public` or `private`), retried after
  250, 500 and 1000 ms, then `Failed to fetch tools`.
- `viola send driver` (`--json`): exit 0, read back, cursor 445. `viola wait driver --after 445`: `turn-ended` at
  12:21:28.017 (14.8 s after the send), two `activity` lines between, tool `ToolSearch`; no dialog event.
- `viola last driver`: 86 words; **not** the fixed word.

## Start 2 — readings

- The CLI's MCP log: connected over stdio in 46 ms, `hasTools` true; **no `tools/list failed` and no error line**.
- `viola send driver --file <the prompt> --json`: exit 0, read back, cursor 445, 36 ms (14:16:11.774 → .810).
- `driver`'s events after the send: `activity` `ToolSearch` (14:16:15.572), `activity`
  `mcp__plugin_viola_viola__list` (14:16:17.651), `activity` `mcp__plugin_viola_viola__wait` (14:17:07.704),
  `turn-ended` (14:17:10.182). **No `question`, `permission` or `plan` event: no permission dialog** — the
  server-level `--allowedTools mcp__plugin_viola_viola` covered both tools.
- `mcp.ndjson` (5 lines, process `mcp`, instance `driver` on each):
  - `process-start{subject:"self"}` at 14:16:02.796, 158 ms after the `session-start`;
  - the `list` tool's agents read: `process-start{subject:"agents-probe"}` 14:16:17.521, `process-exit` 14:16:17.642,
    `duration_ms` 121, `child_exit_status` 0;
  - `channel-request{method:"wait", corr:1, timeout_ms:50000}` at 14:16:17.676, `conn` `mcp-<pid>-<t0>-1`;
  - `channel-response{method:"wait", corr:1, outcome:"timed-out", result_class:"ok", duration_ms:50012}` at
    14:17:07.688.
- `builder`'s `run-builder.ndjson`: the same frame on the wrapper's side, `channel-request{method:"wait",
  timeout_ms:50000}` 14:16:17.676 and `channel-response{outcome:"timed-out", duration_ms:50012}` 14:17:07.688.
- `viola wait driver --after 445 --timeout-ms 300000 --json`: exit 0, `turn-ended` at 14:17:10.182, cursor 1635,
  58.4 s after the send.
- `viola last driver`: exit 0, 1 word, 10 bytes; **equal to the fixed word**.
- No `process-exit{subject:"self"}` in `mcp.ndjson` in either start: the server's client ends it at the stop by a
  signal, not by closing its stdin.
- Census at 14:17:32Z (and 12:22:4xZ after start 1): no process whose executable or working directory lies under
  this repository, either live home or either session dir's tree.

## Against the plan's step 15 (d), start 2

| reading | result |
|---|---|
| `mcp.ndjson` `process-start{subject:"self"}`, `instance` `driver` | read ✓ |
| a `wait` `channel-request` / `channel-response` pair, outcome `timed-out` | read ✓ (`corr` 1, `timeout_ms` 50000) |
| the wait's duration between 50 and 60 s | read ✓ — 50 012 ms on the wire; 50.053 s between the `list` and `wait` tool activity lines |
| the turn's end | read ✓ |
| `viola last driver` equal to the fixed word | read ✓ |
| whether a permission event was raised | read: none |
| the live start count | 2, the raised cap (`inputs#I4`) |

## The fix between the starts

`crates/viola-mcp/src/tools.rs` `tools_result()`: `with_ttl_ms(0)` (stale at once) and `with_cache_scope(Private)`.
`tools_result_carries_its_ttl_and_cache_scope` reads `ttlMs` 0 and `cacheScope` `"private"` on the wire; with the two
setters removed it read red (`left: Null, right: 0`) and green once they were restored; `mcp_tools` asserts both on
the rmcp client's parsed result. Start 2 is that fix's live reading.
