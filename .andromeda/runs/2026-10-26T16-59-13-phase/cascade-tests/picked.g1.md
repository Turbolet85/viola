### test-scope-summary#1.1 · values · scope-entities · ## 1. Test Scope Summary · lines 8–12
- **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets. Covers the sync client/server, the Tokio client feature, the FNV-1a endpoint hash, `MAX_FRAME` (16 MiB) and the result/refusal/error shapes
  - **Source:** arch excerpt, Workspace / Modules and Standard Contracts (Wrapper channel, Endpoint); security excerpt, Vector 1
  - **Testability:** partially-testable
  - **Reason if not fully testable:** same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name. The Windows listener's admission is asserted on CI `test (windows-2025)` (`tests/security_negatives_channel.rs`): rejecting remote pipe clients, by the pipe-type read-back beside an accept-remote and a reject-remote control, and denying access through the Windows pipe DACL (set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, read back as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`, as measured at chunk 2026-09-27-wrapper-channel), by an anonymous-token open denied on the viola listener beside the same open admitted on a default-descriptor pipe. The cross-user part cannot be driven on hosted GitHub runners, which have no second OS account: rejecting a peer with a different euid. It has to be asserted at the peer-credential check boundary. The security excerpt (data classifications) notes the Windows DACL cases are only "partially testable with stubs".
  - **As landed (server verification):** the pid/start-time server verification is asserted on Linux and Windows: the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`) by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame and the verb exits 21, beside a control that reaches the recorded wrapper; §6 Security control negatives), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`. These negatives compile out on macOS, where the client reads no peer pid; the macOS cases are owed (§6 Exit-cause matrix).
### test-scope-summary#2.1 · values · scope-entities · ## 1. Test Scope Summary · lines 18–27
- **Entity:** `viola-agent-claude`, covering:
  - hook parsing and dialog mapping (S3/S7/S8)
  - the R8 `CLAUDE*` strip
  - npm-shim → `claude.exe` resolution
  - the version gate, the capability ledger and the `verify` probes
  - screen signatures, statusline parsing, `claude agents --json` parsing
  - `prompt-submitted` normalisation (M2 harness prefixes; M3/M4 `<pasted_content` / `<\pasted_content`)
  - **Source:** arch excerpt, Workspace / Modules and Test-Relevant Conventions (Ledger probes); creator excerpt, Must-Work (M1–M6, S3/S7/S8, R8)
  - **Testability:** partially-testable
  - **Reason if not fully testable:** parsers, mapping, normalisation, the env strip (using a stub child that dumps its env) and the version gate are testable against the fake agent and against `fixtures/claude/<cli-version>/`. The real `claude` CLI's live behaviour cannot be reached in CI (arch excerpt, CI/CD: "The real `claude` CLI runs only locally, never in CI"; `viola verify` runs in CI only against the fake agent). That includes CLI-native modals (M1), real harness-injected turns (M2), real `claude agents --json` output and new-version drift. Locally it is agent-runnable through `viola verify` (Haiku, spends tokens).
### test-scope-summary#3.1 · values · scope-entities · ## 1. Test Scope Summary · lines 33–36
- **Entity:** CLI verbs and the `--json` / exit-code contract (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install`, global `--home`, exits 0/1/2/10–14/20/21; `revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract: an exit-1 refusal of its strict-modes check or its preflight prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`; `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order, an empty chain the `no-session` document; a refusal `run`'s start makes after a passed preflight prints no document, owed to "Exit-cause code catalogue"), and its exits are 0, 1 for a refusal (the strict-modes check ahead of the log, or a preflight reading) and 2 for a clap usage error, a malformed `--id` included; without `--json` its pairs and rows are byte-identical), plus the human TTY output (k9s-style `viola list`, the readback mirror, C0/C1 escaping)
  - **Source:** arch excerpt, Standard Contracts (CLI); design excerpt, Surfaces (cli); layout excerpt, cli Signature Placements
  - **Testability:** testable
  - **As landed (`link` / `unlink`):** of the eight CLI frame verbs only `link` / `unlink` run `viola_state::strict::check_instance` on the driven instance, before any role file or log opens (their refusal: §5 On-disk); a `driver` equal to `driven` is refused by the wrapper alone, never client-side (CLI exit 20).
### test-scope-summary#4.1 · values · scope-surfaces · ## 1. Test Scope Summary · lines 42–46
- **Surface:** cli (`viola` verbs: `send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install · run` and `revive` launch semantics; `revive` without `--json` has the human signal: exit 0 / 1 / 2, on a refusal one `unable:` line and a last-line `hint:` on stderr with empty stdout, for `--list` one `<ts>  <cause>  <id>` row per logged session on stdout; under `--json` the exit-1 `{"v":1,"error":"<code>","detail":null}` document (its five codes) or `--list`'s `{"v":1,"ok":{"sessions":[…]}}`)
  - **Driver:** subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
  - **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`). Human mode is checked by stdout/stderr line match: `[RB] read back … cursor <n>`, `unable  <name>  <reason>  <detail>` + last-line `hint:`, `stamped <ver>  <n> pass  <n> fail`. There must be no SGR under non-TTY / `NO_COLOR` / `TERM=dumb`.
  - **Boundary:** cross-surface (verbs talk to the `run` wrapper over the IPC channel, and read disk for `list`)
  - **Notes:** traced to arch excerpt, CLI and design excerpt, Surfaces (cli). The human TTY rendering (bold NAME, amber `DIALOG`, dim `stale`) needs a pseudo-TTY on stdout. It is driven through the same outer-PTY driver as the tui surface, not by screen inspection of the child agent.
### test-scope-summary#5.2 · values · scope-surfaces · ## 1. Test Scope Summary · lines 58–64
- **Surface:** ipc-internal (wrapper channel: JSON-RPC 2.0 over ndjson on a named pipe / Unix socket; methods `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, `hook.event`)
  - **Driver:** language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
  - **Signal:** IPC response payload check. Success is `{"result":{"ok":…}}`. Refusal is `{"result":{"refusal":…,"detail":…}}` with the first-applicable ordering. Protocol faults are `{"error":{code,message,data}}` with codes -32700 / -32600 / -32601 / -32602 (`data:{supported, wrapper}`) / -32603 (`"internal error"`, `data:null`). This is cross-checked against the `events.ndjson` byte-offset `cursor`.
  - **Boundary:** single-surface for protocol tests; cross-surface when fed by the fake agent
  - **Notes:** traced to arch excerpt, Wrapper channel and security excerpt, Vector 1. Cross-user peer rejection is flagged in section 1.
  - **As landed (`link` / `unlink`):** both are served by the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven` is `-32602` with `data: null` and no line written, and a failed `record` / `remove` is `-32603`; `link` answers `ok` `{since, already_linked}` and `unlink` an empty `ok` object.

### test-scope-summary#5.3 · values · scope-surfaces · ## 1. Test Scope Summary · lines 65–69
- **Surface:** ipc-internal (MCP stdio server `viola mcp`, tools `send · wait · last · answer · list`)
  - **Driver:** language-native MCP stdio test client. It spawns `viola mcp --home <tmp>` and speaks JSON-RPC over stdin/stdout, with JSON schema assertion.
  - **Signal:** MCP tool result. `isError:false` + `structuredContent` equal to the `ok` payload. `isError:true` + `{refusal, detail}` and text `"refused: <reason>"`. `isError:true` + `{"error":"instance-unreachable","detail":null}` | `{"error":"wrapper-fault","detail":{code,message,data}}` | `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target · text · after · timeout_ms · dialog_id · response`, no frame written), each text `refused: <code>`. An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is the JSON-RPC error `-32603` `internal error`, `data: null`; an unknown tool `-32602`. `tools/list` is exactly the five tools, its result carrying `ttlMs` 0 and `cacheScope` `private` (chunk 2026-10-17-mcp-server-for-drivers). A recorded live exchange (`fixtures/claude/<cli-version>/Mcp.<variant>.json`) is replayed against `viola mcp` by `mcp_recorded_exchange`, the served `tools/list` result keeping the recorded result's key and type shape, those two keys included, and its tool names in order. There must be no `release` / `pause` / `link` / `unlink` tools in `tools/list`.
  - **Boundary:** cross-surface (MCP → wrapper channel → run wrapper → fake agent)
  - **Notes:** traced to arch excerpt, MCP and security excerpt, Vector 5. Schemas are advisory, so negative inputs must be sent that violate the advertised schema.
### test-scope-summary#6.1 · values · strategy · ## 1. Test Scope Summary · lines 75–101
- **5-command requirements:**
  - `boot`:
    - Builds `viola` and creates a fresh temp home, passed as `--home` (arch excerpt, Per-test isolation).
    - Starts one or more `viola run <name> -- <fake agent> <args>` wrappers.
    - Readiness, per instance: `instances/<name>/snapshot.json` exists with `pid`, `started_at`, `child_pid` and `endpoint`. The `heartbeat` mtime is under 5 s old. The log has a start `wheel` and a `budget-gate` event, followed by `session-start` once "Verify-stamped test homes and harness" (the `boot` step-4 owner) joins it to harness readiness.
    - On readiness failure it exits non-zero with a JSON reason on stdout. A `run` refusal to start (exit 1: a `live` or `stale` name, a tampered pinned copy, or a `.cmd`/`.bat` child) is surfaced verbatim.
  - `run`:
    - Invokes, in order:
      1. the workspace unit and integration tests (`cargo test`)
      2. fake-agent E2E against `fixtures/claude/*`
      3. none in 0.1.0: the browser suite left with the browser leg, and the mutation step keeps its number 4
      4. mutation testing scoped to the chunk diff (Founder Direction 1: "surviving mutants are red") — only under `run --mutants`, never in the default or `--all`; its cadence is the epoch-boundary code audit
    - Exit 0 only if every suite passes and no mutant survives. Non-zero otherwise, with a JSON summary on stdout (suite, passed, failed, survived).
    - Local-only mode adds `viola verify` against the real CLI. It must never run in CI (arch excerpt, CI/CD).
  - `status`:
    - Queries `viola list --json --home <tmp>`.
    - The agent polls per item: `name`, `wrapped`, `liveness`, `status`, `wheel`, `budget_paused`, `dialog_pending`, `cli_verified`.
    - It also polls the envelope `skipped{unknown_kinds, unknown_fields, torn_lines}` and `budget`.
  - `cleanup`:
    - Stops every wrapper started by `boot`. Exit is confirmed on the process handle, not EOF.
    - Deletes the temp home and verifies the endpoint is gone: a CLI verb against the name returns exit 21, and the Unix socket file or pipe no longer exists.
    - Must be idempotent: a second run exits 0 and reports nothing to clean.
  - `logs`:
    - Prints, as JSON lines, the structured per-process logs of `run` / `hook` / `mcp` / `cli` in the §3 log format (Founder Direction 3; the exact field set is not in the distilled excerpt, so Phase 3 pins it).
    - Also prints `instances/<name>/events.ndjson` and the hook trace under `diagnostics/`.
    - Filterable by instance and kind.
    - Retention is the lifetime of the temp home: events are never truncated (arch excerpt, Event line), and `cleanup` deletes the home.
### test-scope-summary#7.1 · values · data · ## 1. Test Scope Summary · lines 107–115
- **Test data strategy:**
  - Self-bootstrapping per test: a fresh `--home` temp dir.
  - The fake agent is the child (`viola run <name> -- <fake agent>`; it answers `--version` like the real CLI). It replays hook payloads from `fixtures/claude/<cli-version>/`, which `viola verify` records locally and CI replays.
  - `budget.json` is seeded by piping synthetic statusline stdin into `viola hook statusline`, and `claude agents --json` by a stub on PATH.
  - Property / fuzz generators cover the parser surfaces the security excerpt lists.
  - Constraints:
    - Fixtures must not come from real prompts and must be checked for absolute paths and usernames (security excerpt, Anti-Patterns: "Fixtures from real prompts, or unchecked for paths and usernames").
    - Open conflict for Phase 3: the arch excerpt (Per-test isolation) says the home's `ledger/stamps.json` "can stamp the fake agent's version". The security excerpt rejects "a non-`verify` process writing `stamps.json`". Stamps must therefore be produced by running `viola verify` against the fake agent, or the conflict must be resolved upstream. Resolved in §12 (initial entry, ratified by User review 1): the first option is used.
    - No developer-seeded state.
### test-scope-summary#8.1 · procedure · paths · ## 1. Test Scope Summary · lines 121–128
- **Path:** `run` start sequence. Collision check, pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn. A second `run` of the same name, or a squatted endpoint, exits 1.
  - **Surfaces involved:** tui (`viola run`) + ipc-internal + cli (`list --json`)
  - **Verification signal:**
    - `events.ndjson` order is `wheel{cause:"start"}` → `budget-gate` → the fake agent's `session-start{source:"hook"}` (landed with "Hooks to normalised events").
    - `snapshot.json` has `pinned_bin`, `pid`, `started_at`, `child_pid` (`endpoint` from "Wrapper channel" on); `plugin/` is rewritten, and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written.
    - `viola list --json` shows `{wrapped:true, liveness:"live"}`.
    - A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1.
  - **Source:** arch excerpt, Project Intent, Critical paths hint 1; security excerpt, Vector 7 (pinned exe)
### test-scope-summary#9.2 · procedure · paths · ## 1. Test Scope Summary · lines 155–163
- **Path:** budget governor. `viola hook statusline` writes `rate_limits` to `budget.json` (only when the payload holds a `rate_limits` object) and passes the user's statusline stdout through unchanged (on Unix only: on Windows `run` writes no `settings.json` override and the arm runs no command). Crossing `five_hour` ≥ 90 % or `seven_day` ≥ 85 % refuses automated `send` with `budget-paused`, and `release --budget` overrides it.
  - **Surfaces involved:** cli (`hook statusline`, `send`, `release`) + ipc-internal
  - **Verification signal:**
    - `budget.json` holds the fed percentages. A malformed `resets_at` becomes `"unknown"`.
    - Statusline stdout equals the user command's stdout (empty on failure), exit 0. The equality is Unix only; on Windows the arm runs no command and prints nothing.
    - A `budget-gate{paused:true, window}` event is logged.
    - `send` exits 11 with detail `five-hour` / `seven-day`.
    - `release --budget` gives `{budget_paused:false}`, with `override_until` when a pause stood.
  - **Source:** arch excerpt, Critical paths hint 6; creator excerpt, O8 / §7
### test-scope-summary#10.1 · procedure · triggers · ## 1. Test Scope Summary · lines 169–176
- **Trigger type:** chaos-test
  - **Source:** arch excerpt, Stack (ndjson + atomic snapshots "crash-safe") and On-disk (replay recovery); creator excerpt, Risk Tolerance ("parses defensively and heals itself from day one")
  - **Required test type:**
    - stop the wrapper and shorten the last ndjson line with `File::set_len` (as landed, in place of a kill mid-write): the reader counts it in `torn_lines` and rewrites nothing, and the next append heals it and logs one `state-recovered` line
    - corrupt `snapshot.json` or give it an unsupported `v`: the replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`, with `dialog_pending:false`. As landed, the replay recovers `links` from the logged `link` / `unlink` lines as `{driver, driven, since}` items (chunk 2026-10-10-session-links), the replay writes no file, and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive); the replay gives no `cwd`, a field no event carries
    - child exits without EOF (ConPTY): exit detected on the handle
    - endpoint vanishes during `wait`: `instance-unreachable`
    - stale-heartbeat / live-pid distinction
