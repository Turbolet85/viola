# obs · Obs Scope Summary — category read

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:28) The root bin must stand as the error-capture boundary: a serde-sourced chain must be a fixed message on the way out, with the full chain only in `diagnostics/`.
  > This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:68) The proof must plant a stripped `CLAUDE*` value and find it in no sink, and a drift report must reach `diagnostics/` only, which also holds for any fold of `Refusal` into `AgentError`.
  > Its outputs are the most sensitive in the product:
  >   - Drift reports go to `diagnostics/` only (security Vector 4).
  >   - R8-stripped `CLAUDE*` values are never logged anywhere (security Vector 6).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:99) The owed case for an oversize `viola mcp` stdin line must hold to this one record, with no byte of the line in it.
  > Instrument `hook-invoked` / `hook-decision`, MCP tool-call handling and the `viola mcp` stdin cut (one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`, no byte of the line).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:107) The owed cap-driving case on the child pipe drain must keep this outcome for `claude agents --json`: `"unknown"` plus a counter, with the rows untrusted.
  > External CLI. Instrument the spawn/exit and the parse outcome. A parse failure becomes `"unknown"` plus a counter, and the output rows are untrusted.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:115) The owed cap-driving case on the status-line command's stdout must add no telemetry past duration and exit status, and must not carry the command string out of `diagnostics/`.
  > Instrument the shell-out duration and exit status only. The command string is never logged outside `diagnostics/` (security Data Classifications, `statusline_command`).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:133) No error body on any of the three surfaces may carry an `endpoint`, a `pinned_bin` or a config-value path.
  > Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:159) A failure with no instance resolved must still print only fixed text, since no process log exists to hold anything else.
  > Without an instance there is no process-log file, and human stderr carries only fixed-message lines (design excerpt, CLI faults).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:159) The frame verbs' refused instance check is a named CLI `--json` error body the enumeration must hold, exactly this document, with nothing opened and no line written.
  > `send`, `wait`, `last`, `answer`, `pause` and `release` log as `cli` once their instance check (`refuse_untrusted`: `check_instance` on the home and `instances/<name>`) has passed; a refused check exits 21 with the instance pair, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, opens nothing under the home and writes no line.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:163) Every `Display` that can reach stderr must satisfy this fixed-string rule, and the witnesses must assert it with a path and an upstream text in play.
  > Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:165) Each cause behind exit 21 and exit 1 must keep its distinct code, and a strict-modes refusal ahead of any log must stay lineless with its cause only in the fixed pair or the `--json` document.
  > Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code, except a strict-modes check refused before any log opens, which writes no line: its cause is carried only by the fixed stderr pair or the `--json` document (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink`, exit 21; `run`, `viola revive` and `viola verify`, exit 1).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:203) The channel surface's internal fault must match this exact pair, in the wrapper's answer and in its witness.
  > `-32603` must be `"internal error"` with `data:null` (security Vector 1).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:212) The owed test of a `viola mcp` stdin line past `MAX_FRAME` before `initialize` completes must be read against this clause: refused whole, stderr empty, its one record this line, and the session's end as stated here.
  > rmcp reads stdin only through viola's line cap (`CappedLines` at `MAX_FRAME`): a line of at most `MAX_FRAME` bytes, its newline counted, passes whole; a longer line is refused whole, never truncated or skipped, and ends the session as a closed stdin does, stderr empty, its one record the §6 `parse-rejected{parser:"mcp-stdin", detail:"oversize"}` line.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:215) MCP `isError` results and the log lines of rejected calls must carry only these codes, and the enumeration must name them.
  > `isError` results carry codes only: `refusal` / `detail`, `instance-unreachable`, `wrapper-fault`, `invalid-params` (its `detail` a closed field word; chunk 2026-10-17-mcp-server-for-drivers) (security Vector 5). Log lines follow the same codes-only rule for rejected calls (tests excerpt, Coverage Triggers, MCP).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:265) Any line change that tells the `-32602` causes apart must not add an `event` value without a Decisions Log entry.
  > `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:274) A `channel-response` line that tells the causes apart must do so by an added field, renaming and removing nothing.
  > obs-plan may add fields but must not rename or remove these. The harness greps on them.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:277) The never-log leg of `v1-47` must be proved by the secret-scan test, which is the enforcement this constraint names.
  > No line may contain a `Cookie` header or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:293) The `v1-47` leg that content and drift reports reach only owner-readable files must be held to these modes and this enforcement.
  > Every file is 0600 inside a 0700 directory (Windows: home DACL), and the tests secret-scan test enforces it.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:354) Channel lines must stay to this allow-list, with no `params` or result body, and `-32603` must log as the fixed message.
  >   - A scrub rule on `channel-request` / `channel-response`: method, `id` / `corr`, result class (`ok` / `refusal` / `error` code) and `sender` version only, never `params` or result bodies.
  >   - `-32603` is logged as the fixed "internal error".
- (.andromeda/obs-plan-detail/obs-scope-summary.md:363) `send` text must reach none of the three surfaces, and a refusal that names it must carry only the code.
  >   - A never-log rule for `send` text on stderr, `--json`, MCP and every process-log field outside `instances/<name>/diagnostics/`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:380) A rejected MCP call must use these codes and this closed `detail` set, and the one lineless rejection must stay `instance-unreachable` with `detail: null`.
  > Rejected tool calls are logged with codes only (`refusal` / `detail` / `instance-unreachable` / `wrapper-fault` / `invalid-params`, whose `detail` is a closed field word `target · text · after · timeout_ms · dialog_id · response`; chunk 2026-10-17-mcp-server-for-drivers), with no upstream text, absolute paths or type names. One rejected call writes no line: a frame tool (`send`, `wait`, `last`, `answer`) whose instance check (`check_instance`, before `live_endpoint`) refuses returns `instance-unreachable` with `detail: null` and writes no frame.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:387) The floor the proof plants a value against must cover these named variables and every stripped one, and no switch may turn redaction off.
  >   - A never-log floor for `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every R8-stripped `CLAUDE*` value, across all sinks and fixtures.
  >   - The R8 strip is logged as a count or variable-name list only, never values. Phase 3 must confirm names are admissible.
  >   - An anyhow chain containing a serde source becomes a fixed message before reaching stderr.
  >   - No env, flag or config may disable redaction.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:488) The `-32602` causes must stay distinct: `release-from-driver` must never collapse into a generic fault when the causes are told apart.
  >   - `release-from-driver` is its own logged event, not a generic -32602 wrapper fault.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:489) These two strict-mode documents are CLI `--json` bodies the enumeration must hold unchanged, each with a null `detail`; the revive start-refusal documents stay with "Exit-cause code catalogue".
  > `viola revive`'s strict-mode fail carries no logged `detail` as landed (no log line): its output is the stderr pair, or under `--json` the `{"v":1,"error":"strict-modes-failed","detail":null}` document (chunk 2026-10-17-cli-machine-contract), exit 1 (chunk 2026-10-10-budget-governor). `viola link` / `viola unlink`'s strict-mode fail (their instance check of the driven instance) carries no logged `detail` either (no line): its output is the fixed instance-state `unable` / `hint:` pair on stderr, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, exit 21.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:490) A server-verify fail must hold this exact code and field set, with no pid, start time, path or endpoint in it.
  >   - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (for `link` / `unlink` the driven instance's file), distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:499) The carried `InvalidParams` freight must be closed against this cause list: a `-32602` must not count as a higher-`v` rejection unless it is one.
  >   - Higher-`v` rejections (-32602; the code also carries `send`'s invalid params, `release` carrying `from`, and a `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper's own name, or whose `driver` equals `driven`, so it is not a higher-`v` count alone).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:500) The owed cap-driving cases on channel and `viola mcp` framing must keep these per-parser counts readable.
  >   - Per-parser parse-failure counts: paste validation, channel framing at `MAX_FRAME`, the `viola mcp` stdin at `MAX_FRAME`, hook stdin, `claude agents --json`, vt100 under `catch_unwind`.
- (.andromeda/obs-plan.md:264) Where §1 differs from §3, §6 or §12, those sections must win for this chunk until a wrap brings the difference current, the diagnostics root and the channel join key included.
  > Until a wrap brings a difference current, §3, §6 and §12 win where they differ: diagnostics root (D-08, D-22), `event` enum extensions (D-01…D-05, D-21), `hook-invoked` `corr` timing (D-07), the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`), and the MSRV floor (D-22: 1.96).
- (.andromeda/obs-plan.md:312) Content a sanitised body drops must go only to the instance detail file, never to a home-level line.
  > Every role's codes-only process log goes to its home-level role file under `diagnostics/`, and content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.
- (.andromeda/obs-plan.md:418) The carried `-32602` freight must leave this metric correct or leave this over-count stated; the line's change must be read against it.
  > `viola.compat.v_rejected` over-counts: `-32602` also answers `send`'s invalid params, `release` carrying `from` and a bad `link` / `unlink` name or pair, until the line tells the causes apart.
- (.andromeda/obs-plan.md:451) Every `detail` a sanitised surface or log line carries must be a member of this closed catalog.
  > **`detail` code catalog** (closed, kebab-case, never a path or pid; Founder Direction 4):
- (.andromeda/obs-plan.md:496) The chunk's `thiserror` sweep must keep this mechanism: fixed-message `Display` on each named type, and anyhow at the root-bin edge and its reporter only.
  > The mechanism is typed errors with fixed-message `Display` impls — hand-written on `PtyError` (`viola-pty` takes no thiserror), thiserror 2.0.20 on `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError`. anyhow 1.0.104 is used at the root-bin dispatch edge and its catch-site reporter (`viola::obs::report_internal_error`) only
- (.andromeda/obs-plan.md:509) The three scrubbing layers must hold across all three surfaces, and a serde-sourced chain must become exactly `internal error` before stderr, `--json`, MCP `isError` or channel `error.data`.
  > 1. fixed `Display` strings on every thiserror type (no fields holding paths or payloads);
  > 2. at dispatch, an anyhow chain whose source is a `serde_json::Error` / `serde_path_to_error::Error` is replaced by the fixed message `internal error` before it reaches stderr, `--json`, MCP `isError` or channel `error.data`;
  > 3. the full chain is written as a one-line JSON array `chain:[...]` to `instances/<name>/diagnostics/detail-<process>.ndjson` only, and only when an instance resolves.
- (.andromeda/obs-plan.md:523) A short-lived verb's internal fault must keep this exact line and stderr text, with the chain only in `detail-cli.ndjson`.
  > short-lived `cli`: when an instance resolves, `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` to `cli-<name>.ndjson` (`obs::internal_error_exit_line` covers `Cli` as well as `Run`), the chain in `detail-cli.ndjson` only; then exactly `error: internal error\n` on stderr in one `write_all`, with no hint, and exit 1;
- (.andromeda/obs-plan.md:537) The MCP witness must read `instance-unreachable` from the `isError` code alone, since that outcome writes no line.
  > MCP rejects are `isError` codes only; a logged reject's line is codes-only, and `instance-unreachable` — a frame tool's refused instance check (made before the liveness pre-check), the liveness pre-check's, or a frame tool's server-verification mismatch on Linux and Windows — is logged by no line, its outcome kept inside `mcp.tool_call`, the `isError` code its only record.
- (.andromeda/obs-plan.md:570) The secret-scan leg of `v1-47` must run over every `diagnostics/` file in CI before artifact upload, with the canary absent from every home-level file.
  > **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload; the canary never appears in a home-level `diagnostics/*.ndjson`;
- (.andromeda/obs-plan.md:576) A field added to tell the `-32602` causes apart must be named in the §6 catalog first and pass G4, or it is a build failure.
  > **Default-deny posture:** a field is logged only if it is named in this plan's Section 6 catalog. Unknown fields are a build failure (§10), caught by gate G4 (§9) against `schemas/diag-line.v1.json`.
- (.andromeda/obs-plan.md:724) This ban must hold on all four named outlets, which the chunk's three surfaces and stderr cover.
  > NEVER print an anyhow chain that still contains a serde_json / serde_path_to_error source to stderr, `--json`, MCP `isError` or channel `error.data`.
- (.andromeda/obs-plan.md:734) The floor must hold at every level and in every file, detail files included, so a planted `CLAUDE*` value must not appear even in `detail-<process>.ndjson`.
  > NEVER log `CLAUDE*` values or a `Cookie` header, at any level, in any file, detail files included.
- (.andromeda/obs-plan.md:735) No `detail` code a surface or a line carries may hold a path or a pid.
  > NEVER put paths or pids inside `detail` codes (Founder Direction 4). Paths appear only in instance detail files.

## Bindings
- **Fixed messages and scrubbing layers ↔ §7 Error Capture & Reporting**: §1's serde-chain rule and §7's three layers state one requirement, and §7's list of error types is what the `thiserror` sweep must cover, (.andromeda/obs-plan-detail/obs-scope-summary.md:28) (.andromeda/obs-plan.md:509) (.andromeda/obs-plan.md:496)
- **`Refusal` enum fold ↔ architecture [Error Handling]**: the fold or ratification the architecture names must leave `AgentError` a fixed-message type on §7's list, with drift reports held to `diagnostics/`, (.andromeda/obs-plan.md:496) (.andromeda/obs-plan-detail/obs-scope-summary.md:68)
- **MCP codes ↔ security plan Vector 5**: the codes-only `isError` rule is sourced to Vector 5, and both must name the same closed set, (.andromeda/obs-plan-detail/obs-scope-summary.md:215) (.andromeda/obs-plan-detail/obs-scope-summary.md:380)
- **Channel `-32603` ↔ security plan Vector 1**: the exact `"internal error"` / `data:null` pair is sourced to Vector 1, and the channel line's scrub rule must agree with it, (.andromeda/obs-plan-detail/obs-scope-summary.md:203) (.andromeda/obs-plan-detail/obs-scope-summary.md:354)
- **Never-log floor ↔ security plan Vector 6 and test-plan secret scan**: Vector 6 sets the floor and the secret-scan test proves it, which is the `v1-47` leg to read at P3, (.andromeda/obs-plan-detail/obs-scope-summary.md:277) (.andromeda/obs-plan-detail/obs-scope-summary.md:387) (.andromeda/obs-plan.md:570)
- **`-32602` causes ↔ §5 Metric Coverage and §6 Log Coverage**: telling the causes apart on `channel-response` moves `viola.compat.v_rejected`, and any added field must enter the §6 catalog and `schemas/diag-line.v1.json`, (.andromeda/obs-plan.md:418) (.andromeda/obs-plan.md:576) (.andromeda/obs-plan-detail/obs-scope-summary.md:499)
- **Owed `viola mcp` stdin case ↔ test-plan §4 viola-mcp and §6 Contract suite**: the test plan names this entry the owner of the case, and §1 fixes the record and streams it must assert, (.andromeda/obs-plan-detail/obs-scope-summary.md:212) (.andromeda/obs-plan-detail/obs-scope-summary.md:99)
- **Strict-modes documents ↔ "Exit-cause code catalogue"**: the `--json` documents here stay in this chunk's enumeration, and the revive start-refusal documents stay with that entry, (.andromeda/obs-plan-detail/obs-scope-summary.md:489)
