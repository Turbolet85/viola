### test-scope-summary#1.1 · values · scope-entities · ## 1. Test Scope Summary · lines 8–11
- **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets. Covers the sync client/server, the Tokio client feature, the FNV-1a endpoint hash, `MAX_FRAME` (16 MiB) and the result/refusal/error shapes
  - **Source:** arch excerpt, Workspace / Modules and Standard Contracts (Wrapper channel, Endpoint); security excerpt, Vector 1
  - **Testability:** partially-testable
  - **Reason if not fully testable:** same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name. The Windows listener's admission is asserted on CI `test (windows-2025)` (`tests/security_negatives_channel.rs`): rejecting remote pipe clients, by the pipe-type read-back beside an accept-remote and a reject-remote control, and denying access through the Windows pipe DACL (set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, read back as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`, as measured at chunk 2026-09-27-wrapper-channel), by an anonymous-token open denied on the viola listener beside the same open admitted on a default-descriptor pipe. The cross-user part cannot be driven on hosted GitHub runners, which have no second OS account: rejecting a peer with a different euid. It has to be asserted at the peer-credential check boundary. The security excerpt (data classifications) notes the Windows DACL cases are only "partially testable with stubs".
### test-scope-summary#3.1 · values · scope-entities · ## 1. Test Scope Summary · lines 32–34
- **Entity:** CLI verbs and the `--json` / exit-code contract (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install`, global `--home`, exits 0/1/2/10–14/20/21; `revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract: an exit-1 refusal of its strict-modes check or its preflight prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`; `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order, an empty chain the `no-session` document; a refusal `run`'s start makes after a passed preflight prints no document, owed to "Exit-cause code catalogue"), and its exits are 0, 1 for a refusal (the strict-modes check ahead of the log, or a preflight reading) and 2 for a clap usage error, a malformed `--id` included; without `--json` its pairs and rows are byte-identical), plus the human TTY output (k9s-style `viola list`, the readback mirror, C0/C1 escaping)
  - **Source:** arch excerpt, Standard Contracts (CLI); design excerpt, Surfaces (cli); layout excerpt, cli Signature Placements
  - **Testability:** testable
### test-scope-summary#4.1 · values · scope-surfaces · ## 1. Test Scope Summary · lines 40–44
- **Surface:** cli (`viola` verbs: `send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install · run` and `revive` launch semantics; `revive` without `--json` has the human signal: exit 0 / 1 / 2, on a refusal one `unable:` line and a last-line `hint:` on stderr with empty stdout, for `--list` one `<ts>  <cause>  <id>` row per logged session on stdout; under `--json` the exit-1 `{"v":1,"error":"<code>","detail":null}` document (its five codes) or `--list`'s `{"v":1,"ok":{"sessions":[…]}}`)
  - **Driver:** subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
  - **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`). Human mode is checked by stdout/stderr line match: `[RB] read back … cursor <n>`, `unable  <name>  <reason>  <detail>` + last-line `hint:`, `stamped <ver>  <n> pass  <n> fail`. There must be no SGR under non-TTY / `NO_COLOR` / `TERM=dumb`.
  - **Boundary:** cross-surface (verbs talk to the `run` wrapper over the IPC channel, and read disk for `list`)
  - **Notes:** traced to arch excerpt, CLI and design excerpt, Surfaces (cli). The human TTY rendering (bold NAME, amber `DIALOG`, dim `stale`) needs a pseudo-TTY on stdout. It is driven through the same outer-PTY driver as the tui surface, not by screen inspection of the child agent.
### test-scope-summary#5.2 · values · scope-surfaces · ## 1. Test Scope Summary · lines 56–61
- **Surface:** ipc-internal (wrapper channel: JSON-RPC 2.0 over ndjson on a named pipe / Unix socket; methods `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, `hook.event`)
  - **Driver:** language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
  - **Signal:** IPC response payload check. Success is `{"result":{"ok":…}}`. Refusal is `{"result":{"refusal":…,"detail":…}}` with the first-applicable ordering. Protocol faults are `{"error":{code,message,data}}` with codes -32700 / -32600 / -32601 / -32602 (`data:{supported, wrapper}`) / -32603 (`"internal error"`, `data:null`). This is cross-checked against the `events.ndjson` byte-offset `cursor`.
  - **Boundary:** single-surface for protocol tests; cross-surface when fed by the fake agent
  - **Notes:** traced to arch excerpt, Wrapper channel and security excerpt, Vector 1. Cross-user peer rejection is flagged in section 1.

### test-scope-summary#5.3 · values · scope-surfaces · ## 1. Test Scope Summary · lines 62–66
- **Surface:** ipc-internal (MCP stdio server `viola mcp`, tools `send · wait · last · answer · list`)
  - **Driver:** language-native MCP stdio test client. It spawns `viola mcp --home <tmp>` and speaks JSON-RPC over stdin/stdout, with JSON schema assertion.
  - **Signal:** MCP tool result. `isError:false` + `structuredContent` equal to the `ok` payload. `isError:true` + `{refusal, detail}` and text `"refused: <reason>"`. `isError:true` + `{"error":"instance-unreachable","detail":null}` | `{"error":"wrapper-fault","detail":{code,message,data}}` | `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target · text · after · timeout_ms · dialog_id · response`, no frame written), each text `refused: <code>`. An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is the JSON-RPC error `-32603` `internal error`, `data: null`; an unknown tool `-32602`. `tools/list` is exactly the five tools, its result carrying `ttlMs` 0 and `cacheScope` `private` (chunk 2026-10-17-mcp-server-for-drivers). A recorded live exchange (`fixtures/claude/<cli-version>/Mcp.<variant>.json`) is replayed against `viola mcp` by `mcp_recorded_exchange`, the served `tools/list` result keeping the recorded result's key and type shape, those two keys included, and its tool names in order. There must be no `release` / `pause` / `link` / `unlink` tools in `tools/list`.
  - **Boundary:** cross-surface (MCP → wrapper channel → run wrapper → fake agent)
  - **Notes:** traced to arch excerpt, MCP and security excerpt, Vector 5. Schemas are advisory, so negative inputs must be sent that violate the advertised schema.
### test-scope-summary#8.1 · procedure · paths · ## 1. Test Scope Summary · lines 118–125
- **Path:** `run` start sequence. Collision check, pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn. A second `run` of the same name, or a squatted endpoint, exits 1.
  - **Surfaces involved:** tui (`viola run`) + ipc-internal + cli (`list --json`)
  - **Verification signal:**
    - `events.ndjson` order is `wheel{cause:"start"}` → `budget-gate` → the fake agent's `session-start{source:"hook"}` (landed with "Hooks to normalised events").
    - `snapshot.json` has `pinned_bin`, `pid`, `started_at`, `child_pid` (`endpoint` from "Wrapper channel" on); `plugin/` is rewritten, and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written.
    - `viola list --json` shows `{wrapped:true, liveness:"live"}`.
    - A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1.
  - **Source:** arch excerpt, Project Intent, Critical paths hint 1; security excerpt, Vector 7 (pinned exe)
### test-scope-summary#8.2 · procedure · paths · ## 1. Test Scope Summary · lines 126–135
- **Path:** confirmed `send` with CL-1 events. Driver `send` is accepted, then `send-issued (cursor, from)` is logged, one bracketed paste + Enter is written, and the matching `prompt-submitted{origin:"driver"}` confirms it. If no match arrives in the window, the result is `not-delivered`/`no-prompt-submitted` and `send-refused` is logged. A text is validated as received and then typed with every CR LF pair and every other CR as one LF and without its trailing CR and LF characters, every one of them in any order (the typed text holds no CR, and nothing else changes), and that typed text is what is classified, pasted and matched: a `send` whose text ends in LF, CR or CRLF, or holds a CR or a CR LF inside, is confirmed like any other and the driver keeps the wheel (chunk 2026-10-07-a-send-ending-in-a-newline-is-confirmed for LF, chunk 2026-10-09-epoch-3-cleanup for CR and CRLF at the end, chunk 2026-10-09-inner-cr-and-crlf-in-a-sent-text for a CR or a CR LF inside). A text whose typed text is empty is refused `not-delivered`/`empty-text` (exit 13) right after `control-character` and before anything is issued, by the client before any frame and by the wrapper. `send` consumes the ledger's compiled local-command list by exact equality of the typed text (nothing else trimmed, no case folding, never a leading-slash test; chunk 2026-10-06-local-command-send-outcomes), so a listed command followed by LF, CR or CRLF is that command. A listed command with no measured post-condition, or any listed command on an unverified CLI, yields `ok` `{confirmed:false, detail:"unconfirmable", cursor}` at once, with no confirmation window.
 `/clear` on a verified CLI waits for its post-condition only (a `session-start` with cause `clear` and a new session id) and is never claimed by a `prompt-submitted`, and so does `/compact` (a `session-start` with cause `compact` and the session's own unchanged id) inside its own window, `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL); a missed post-condition is `not-delivered`/`no-prompt-submitted`. A local command is never presumed delivered. Multi-line text arrives as one prompt.
  - **Surfaces involved:** cli + ipc-internal (MCP `send`) + tui (fake agent receipt)
  - **Verification signal:**
    - CLI `--json` exit 0 with `{submitted_at, cursor}`, where `cursor` equals the pre-paste `events.ndjson` end offset.
    - The log shows `send-issued` then `prompt-submitted`.
    - The fake agent reports one prompt with its inner LFs intact and each inner CR or CR LF as one LF, no CR, no trailing LF and no ESC.
    - With the fake agent suppressing UserPromptSubmit, the CLI exits 13 with detail `no-prompt-submitted`, `send-refused` is logged.
    - An MCP `send` gives `isError:false` with the same `structuredContent`.
  - **Source:** arch excerpt, Critical paths hint 2; creator excerpt, R3/S1, M3/M4, M5, Founder Direction 5
### test-scope-summary#9.1 · procedure · paths · ## 1. Test Scope Summary · lines 141–151
- **Path:** human takes the wheel, automation is refused, `release` returns it. A human editing key in the `run` terminal since the last turn boundary moves the wheel to the human. A driver `send` is refused with `human-typing`. `pause` forces it manually. The wheel returns only via CLI `viola release`, and `release` carrying `from` is rejected.
  - **Surfaces involved:** tui (outer-PTY keystrokes) + cli + ipc-internal (channel + MCP)
  - **Verification signal:**
    - The log shows `wheel{holder:"human", cause:"human-input"}`.
    - `send --json` exits 10 with detail `null`. After `pause` the detail is `manual-pause`.
    - MCP `send` gives `isError:true` `"refused: human-typing"`.
    - `release` gives `{wheel:"driver"}` and the next `send` exits 0.
    - `release` with `from` gives `-32602` `release-from-driver` (exit 20).
    - A harness-injected turn (a prompt starting `<agent-message from=`, `<task-notification>`, `<\cross-session-message` or `<cross-session-message`) does not flip the wheel.
    - The human is never blocked: typing still reaches the fake agent.
  - **Source:** arch excerpt, Critical paths hint 5; creator excerpt, R2, M2 and §6 passed run ("the founder typing into the window took the wheel, `send` was refused, `release` returned it")
### test-scope-summary#10.1 · procedure · triggers · ## 1. Test Scope Summary · lines 166–173
- **Trigger type:** chaos-test
  - **Source:** arch excerpt, Stack (ndjson + atomic snapshots "crash-safe") and On-disk (replay recovery); creator excerpt, Risk Tolerance ("parses defensively and heals itself from day one")
  - **Required test type:**
    - stop the wrapper and shorten the last ndjson line with `File::set_len` (as landed, in place of a kill mid-write): the reader counts it in `torn_lines` and rewrites nothing, and the next append heals it and logs one `state-recovered` line
    - corrupt `snapshot.json` or give it an unsupported `v`: the replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`, with `dialog_pending:false`. As landed, the replay recovers `links` from the logged `link` / `unlink` lines as `{driver, driven, since}` items (chunk 2026-10-10-session-links), the replay writes no file, and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive); the replay gives no `cwd`, a field no event carries
    - child exits without EOF (ConPTY): exit detected on the handle
    - endpoint vanishes during `wait`: `instance-unreachable`
    - stale-heartbeat / live-pid distinction
### test-strategy#1.1 · values · strategy · ## 2. Test Strategy · lines 8–11
- **Level:** Unit
  - **Coverage focus:** Pure functions, isolated module logic
  - **Tier-specific depth (Comprehensive, viola):** Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes, except the two real-cargo-mutants harness tests, compiled out on macOS (§10 Mutation gate).
  - **Runner:** nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported, and under it the root-bin cases `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`, `obs::tests::log_config_rejection_writes_one_warn_line` and `case_2_sigterm` are known-cause reds, recorded with no owner.
### test-strategy#1.2 · values · strategy · ## 2. Test Strategy · lines 12–14
- **Level:** Integration
  - **Coverage focus:** Cross-module + IPC + disk state + process boundaries (no DB exists)
  - **Tier-specific depth (Comprehensive, viola):** Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
### test-strategy#1.3 · values · strategy · ## 2. Test Strategy · lines 15–17
- **Level:** E2E
  - **Coverage focus:** Critical user-facing paths via surfaces
  - **Tier-specific depth (Comprehensive, viola):** Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
### test-strategy#1.5 · values · strategy · ## 2. Test Strategy · lines 21–23
- **Level:** Performance / Load
  - **Coverage focus:** Budget assertions on perf-critical paths
  - **Tier-specific depth (Comprehensive, viola):** Required (performance-budget trigger present). hyperfine 1.20.0 gates the hook deadlines on `max` (the `--perf` arm, the five hyperfine rows — `pre-tool-use` included, on an unstamped perf session — and the per-OS `perf` CI job are built; §10). The heartbeat staleness flip at >5 s is checked by `viola_state::liveness::classify`, a pure function fed the beat's age (no clock).
### test-strategy#1.6 · values · strategy · ## 2. Test Strategy · lines 24–26
- **Level:** Chaos / Fault
  - **Coverage focus:** Failure injection (process / disk / clock)
  - **Tier-specific depth (Comprehensive, viola):** Required (chaos trigger present). Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
### test-strategy#3.1 · values · strategy · ## 2. Test Strategy · lines 56–58
| Test-scope Sec 5 trigger | Primary layer(s) | Plan location |
|---|---|---|
| IPC vector (V1) | integration + E2E | §5 Module ↔ IPC; §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial) |
### test-strategy#3.3 · values · strategy · ## 2. Test Strategy · lines 60–60
| Hook stdin (V4) | integration (assert_cmd hook) | §5 hook boundary; §6 Security sweep |
### test-strategy#3.4 · values · strategy · ## 2. Test Strategy · lines 61–61
| MCP (V5) | E2E (rmcp client) | §6 Scenarios 2, 5, 7 + Security sweep |
### test-strategy#3.5 · values · strategy · ## 2. Test Strategy · lines 62–62
| CLI and env (V6) | unit + integration | §4 viola-core/root bin; §5 |
### test-strategy#3.6 · values · strategy · ## 2. Test Strategy · lines 63–63
| Filesystem (V7) | integration + chaos | §5 Module ↔ disk; §6 Scenario 1, E5 |
### test-strategy#3.10 · values · strategy · ## 2. Test Strategy · lines 67–67
| chaos-test | chaos | §6 Chaos suite |
### test-strategy#3.11 · values · strategy · ## 2. Test Strategy · lines 68–68
| performance-budget | perf | §10 Performance budgets |
### test-strategy#3.12 · values · strategy · ## 2. Test Strategy · lines 69–69
| cross-surface-coordination | E2E | §6 Scenarios 2, E3 + Cross-surface parity |
### test-strategy#3.13 · values · strategy · ## 2. Test Strategy · lines 70–70
| multi-os-compat | CI matrix | §9 Matrix builds |
### test-strategy#3.16 · values · strategy · ## 2. Test Strategy · lines 73–73
| error sanitization / secret logging | integration + E2E | §6 Error sanitization and secret scan; §3 logs |
