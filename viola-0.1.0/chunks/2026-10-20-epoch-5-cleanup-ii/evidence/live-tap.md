# The live round (plan.md step 18): 2 starts of the cap 2 — start 1 red, start 2 green, recorded

**Status (2026-10-20T18:35Z):**
- **Start 1** (the cap 1, `inputs#I2` Q1 «Один старт», Q2 «2.1.302»; `inputs#I4`): exited 1 in 252 ms with no recording.
  The CLI spawned no MCP server. Not re-fired.
- **The founder's new word** (`inputs#I5` Q1, dialog 15): «Ещё один, промпт через stdin», "one more start, stdin form".
  The cap rose to 2.
- **Start 2** (the stdin form, the same rig, home and build): exit 0 in 3.2 s, stderr empty, the reply the fixed word,
  and one exchange recorded. The fixture `fixtures/claude/2.1.302/Mcp.tools-list-1.json` was written from it.
- **Measured on start 2:** on protocol `2026-07-28`, claude 2.1.302 sends **`server/discover`**, then `tools/list`, each
  with its own `_meta` (protocol version, client info, client capabilities). It sends no `initialize` and no
  `notifications/initialized`, which are the frames plan.md step 15 named. The class document was adapted to the
  frames the CLI sent (below).

No prompt text, assistant text, session id, `CLAUDE*` value or path content is in this file or in the ledger. Times are
UTC, read from the clock. Linux dev host.

## Setup (both starts)

- `hyprctl locked`: `false` before each start; the rig types no desktop key.
- `live-ledger.py`: copied byte for byte from `viola-0.1.0/chunks/2026-10-17-mcp-server-for-drivers/evidence/`
  (`cmp` exit 0).
- The rig `live-tap.py` (this dir), `setup` at stamp `20261020T162750Z`:
  - the fresh, unstamped home `target/e2e-home/viola-live-tap-20261020T162750Z/vhome` (made first by start 2's server;
    `tmpfs` behind the `target/e2e-home` link, 15G free of 32G);
  - the project dir `…/proj` beside it: `git init -q` exit 0, `rev-parse --show-toplevel` reads `proj/` itself, it
    resolves outside the work tree, and no `CLAUDE.md`, `.claude/CLAUDE.md` or `CLAUDE.local.md` stands at it or any
    ancestor (0 found);
  - the private dir `target/mcp-live-tap-20261020T162750Z/priv/` (0700) with the `--mcp-config` file `mcp.json` (0600):
    one server, `viola`, `type` `stdio`, `command` the tap's absolute path, `args` `record <priv>/capture.ndjson <abs
    target/debug/viola> --home <base>/vhome mcp`, `env` empty.
- The CLI: `~/.local/share/mise/installs/claude/2.1.302/claude --version` by path: `2.1.302 (Claude Code)`, read
  before each start.
- The environment: 8 declared names (`HOME`, `LANG`, `LOGNAME`, `PATH` with the 2.1.302 dir first, `SHELL`, `TERM`,
  `USER`, `XDG_RUNTIME_DIR`), no `CLAUDE*` and no `VIOLA*` name.
- **The build, both starts:** `target/debug/viola` sha256 prefix `5a63d1c72218cf00` (the plain build, no `fake-agent`)
  and `target/debug/viola-mcp-tap` `276835b941422a52`, read right before each start.
  - The setup line's `e8d82882501399d5` was a `fake-agent` build a test run had linked there at 16:27Z. It was never
    started.
  - Before start 2, the gate block's `fake-agent` runs had relinked both names to other builds. Each name was pointed
    back at cargo's own artifact of the ledgered build (`ln -f` from `target/debug/deps/`, the operation cargo itself
    performs), and both hashes were read equal to the ledger's.
  - Between the starts, only one source file under these binaries changed: `ledger.rs`'s `#[cfg(test)]` module
    declaration (a `#[path]` and a comment, `inputs#I5` Q2), which compiles out of both builds.
- The prompt: the fixed synthetic text, sha256 prefix `9f8de96f7d6e43aa`.

## The ledger (`live-sessions.ndjson`, `live-ledger.py` its one writer)

| row | `written_at` | |
|---|---|---|
| `start` n 1, cap 1, authority `inputs#I2`, `inputs#I4` | 16:28:32Z | before start 1 |
| `census` | 16:29:37Z | `processes_left` 0, outcome `no-recording` |
| `start` n 2, cap 2, form 2, authority `inputs#I5` Q1 (the founder's own word) | 18:31:58Z | before start 2 |
| `census` | 18:34:23Z | `processes_left` 0, outcome `recorded` |

## Start 1 (form 1: the plan's argv)

`claude -p --strict-mcp-config --mcp-config <priv>/mcp.json <the prompt>`, from `proj/`, a direct spawn, stdin null.

- Began 16:28:36.66Z, ended 16:28:36.91Z: exit 1, 0 bytes on stdout, no capture file, no home made, no `mcp.ndjson`.
- The CLI's stderr was captured and **not recorded**, a rig defect, fixed for start 2. No CLI log was left for `proj/`.
- Measured afterwards, with no session (`claude --help` by the same path): `--mcp-config <configs...>`, "Load MCP servers
  from JSON files or strings (space-separated)". The option is variadic.
- **HYPOTHESIS (not measured):** the prompt after the config path was read as a second `--mcp-config` value. Start 2's
  form, with nothing after the config path, started cleanly, which is consistent with it but does not measure it.

## Start 2 (form 2: the prompt on stdin)

`claude -p --strict-mcp-config --mcp-config <priv>/mcp.json` with the prompt on stdin; nothing follows the config path.

| began | ended | exit | stdout | stderr | reply |
|---|---|---|---|---|---|
| 18:32:04.50Z | 18:32:07.74Z | 0 | 3 bytes | 0 bytes (`stderr-2.txt`, 0600, not committed) | the fixed word |

- **The capture:** 0600, 4 lines, 2 per direction, in order: client `server/discover` → its response → client
  `tools/list` → its response.
- The server's codes-only log `mcp.ndjson` (2 lines, process `mcp`): `process-start{subject:"self"}`, then
  `process-exit{subject:"self", exit_code:0}`. The CLI closed the server's stdin at its end, with no signal.
- **The `server/discover` result** (the keys only): `resultType` `complete`, `supportedVersions` (5, ending
  `2026-07-28`), `capabilities.tools`, `ttlMs` 0, `cacheScope` `private`, `_meta` `io.modelcontextprotocol/serverInfo`
  `{name: viola, version}`.
- **The `tools/list` result** (the keys only): `resultType` `complete`, `ttlMs` 0, `cacheScope` `private`, `tools` with
  `send`, `wait`, `last`, `answer`, `list` in order, each with `name`, `description` and `inputSchema`.
- **Identity read** before the commit (testing.md 2026-10-14). Every string outside the tool texts was read:
  - each request's `_meta` `io.modelcontextprotocol/clientInfo` names the CLI product only: `claude-code`,
    `Claude Code`, `2.1.302`, its product description and its public website URL;
  - the two JSON-RPC ids are `server-discover-probe-1` and `0`, neither session-shaped;
  - no home, user word, host word or email.
- Census (`gate.py census --args mcp-live-tap --args viola-live-tap --args viola-mcp-tap`, 18:34Z): none running.

## The fixture (`viola-mcp-tap fixture`, the scrub and its closed refusal)

- `fixtures/claude/2.1.302/Mcp.tools-list-1.json`: exit 0 (no refusal code). It holds:
  - `v` 1;
  - `client`, the two client frames verbatim and in order. `cmp` of each frame, re-serialised with sorted keys,
    against the capture's client lines: equal;
  - `results`, keyed by method: `server/discover` and `tools/list`.
- A grep for `/home/`, `/Users/`, the host user word and `@`: 0 hits.
- **Adapted to the measured handshake** (a deviation from plan.md step 15, in-intent; the wrap amends):
  - the class document holds the client's frames up to and including its first `tools/list` request, with each
    request's result keyed by its method, where the plan named `initialize`, `notifications/initialized`, `tools/list`
    and two results;
  - the schema, the tap's unit case and the hygiene walk's planted exchange follow it;
  - the replay takes the `tools/list` response by its request's id, not by position.

## Staging before start 1 (steps 15–17 green against a synthetic capture; no `claude`)

- A scratchpad driver sent the tap the `initialize` form's three frames over the same build: tap exit 0, capture 0600, 5
  lines.
- Its fixture, placed for the reading and then removed: replay `"ok":true` (0.562 s); hygiene, drift and ledger-probe
  contracts 96 passed. The drift and ledger-probe contracts are unchanged by a set holding an `Mcp.` file.
