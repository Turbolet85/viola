# security · cascade read · group 3 of 3

## Carried
- (.andromeda/security-plan-detail/secret-management.md:9) A planted stripped `CLAUDE*` value must reach no error body of the three surfaces and no file the owner-only and secret-scan legs read, and the strip must keep handling names and never read a value.
  > Inherited credentials (`CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and the other `CLAUDE*` identity variables) stay only in the `viola run` wrapper's environment. They are removed from the child by the R8 rule and are never serialised into events, snapshots, diagnostics or the ledger. R8 (operator ruling 2026-09-25, Decisions Log): every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) is removed except a name in the persistent set — Windows registry `Environment` value names, Unix `config.json` `claude_env_keep` — and the identity floor (`IDENTITY_FLOOR`, the 11 names measured on the Windows host, both messaging variables among them) is removed whatever the persistent set says. The strip, the registry reader and the logs handle names only and never read a value; `process-start{subject:"claude-child"}` names the stripped (`env_stripped_known`) and kept (`env_kept`) names.
  (.andromeda/security-plan.md:372)
  > **Production:** no stored secrets in v1, and environment variables are not a configuration channel; the inherited `CLAUDE*` credentials stay only in the `viola run` wrapper's environment, removed from the child by the R8 rule (the identity floor whatever the persistent set says) and never serialised into events, snapshots, diagnostics or the ledger; the strip handles names only and never reads a value.
  (.andromeda/security-plan.md:380)
  > What CI does run is the obs-owned artifact canary scan (`viola-harness secret-scan`, `id: secret-scan`, `if: always()`, obs-plan §9 step 3). Its classes:
  > - the stripped `CLAUDE*` canaries (the twelve `claude-stripped` needles);
  > - the `cookie:` header, case-insensitive;
  > - the content canary, which fails only in home-level role files;
  > - a non-`0600` diagnostics file, Unix only.
  (.andromeda/security-plan.md:386)
  > It reads every file under `target/agent-run/` except `run --mutants`' `target/agent-run/chunk.diff`, which is skipped by exact path and excluded from the `harness-<os>` upload, so it is never uploaded. It never prints or writes the matched bytes, and the test-home uploads run only when it succeeds.
  (.andromeda/security-plan.md:476)
  > NEVER write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log or stderr, `events.ndjson`, snapshots, `diagnostics/` or fixtures (consolidated floor: Bootstrap phases, `logging-redaction-wire`).
- (.andromeda/security-plan-detail/error-handling.md:8) The cap-driving case for the status-line command's stdout must drive `viola hook statusline` to its cap and keep its stderr empty and its exit 0, and no `Display` or `Refusal` change may give the hook a body, a stderr line or a non-zero exit.
  > `hook` never writes to stderr and never exits non-zero (Hook Contract). A security check failing in `hook` (server verification mismatch, oversized stdin, strict-modes failure) still ends in exit 0 with no body, so the dialog renders for the human.
  (.andromeda/security-plan.md:418)
  > `hook` never writes to stderr and never exits non-zero: a security check failing in `hook` still ends in exit 0 with no body, so the dialog renders for the human; its hidden capture arms print nothing on every failure, their stderr always empty, their exit always 0.
  (.andromeda/security-plan.md:483)
  > NEVER write to stderr from `viola hook`, and NEVER exit non-zero from it, including on security-check failures.
  (.andromeda/security-plan.md:510)
  > NEVER let a security refusal block the human. Refusals go only to automation (`send`/`answer`), and hooks fail open.
- (.andromeda/security-plan-detail/error-handling.md:11) The cap-driving case must run the recorded command from a snapshot its readers do not replay, or the case prints nothing and drives no cap.
  > `viola hook statusline` (without `--capture`) over a replayed snapshot (`read_snapshot_or_replay`) finds no `statusline_command`, so nothing runs and nothing prints; it exits 0 with empty stderr.
- (.andromeda/security-plan-detail/error-handling.md:12) The MCP enumeration must name this unreachable result and the MCP witness must hold it to these exact fields, with no added word and no log line.
  > A `viola mcp` frame tool (`send` · `wait` · `last` · `answer`) whose server check fails (the connected peer is not the recorded wrapper, or the check cannot be made) sends no frame and returns the result of an unreachable instance: `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`; it adds no error word and writes no log line.
  (.andromeda/security-plan.md:413)
  > MCP `isError` results carry only the `structuredContent` codes from Conventions (`refusal`/`detail`, or `error: instance-unreachable | wrapper-fault | invalid-params` — `instance-unreachable` with `detail: null`, `wrapper-fault` with the JSON-RPC fault's `{code,message,data}`, `invalid-params` with a closed field word), and every `isError` text is `refused: <code>`. Any other handler fault, a reply of no known shape or a panicked call is the JSON-RPC error `-32603` `internal error` with `data: null` (chunk 2026-10-17-mcp-server-for-drivers).
- (.andromeda/security-plan-detail/error-handling.md:13) The MCP enumeration must hold the strict-modes and replayed-snapshot routes to the same body as the server-check route, so that no refusal route of a frame tool carries a path or a text of its own.
  > A `viola mcp` frame tool runs the instance's strict-modes check (`check_instance`) before its liveness pre-check; a refused check, like a snapshot that is replayed or absent (no live endpoint), sends no frame and returns the same unreachable result: `isError` `instance-unreachable` with `detail: null`, text `refused: instance-unreachable`, and no log line.
- (.andromeda/security-plan-detail/error-handling.md:14) The channel enumeration must name the `link` / `unlink` `-32603` body with its fixed message and null data, and telling the `-32602` causes apart must leave the client's exit 20 for a wrapper error reply as it is.
  > The wrapper's `link` / `unlink` methods answer a failed `record` / `remove` as `-32603` with the fixed message `internal error` and `data: null`. At `viola link` / `viola unlink`, a wrapper error reply (a `-32602` included) is a wrapper fault, exit 20, and a `Refused` reply or an `ok` of a shape other than the verb's own (`link`: a string `since` and a boolean `already_linked`; `unlink`: an object) is an internal error, exit 1.
  (.andromeda/security-plan.md:412)
  > Channel `-32603` uses a fixed `message` ("internal error") with `data: null`.
  (.andromeda/security-plan.md:421)
  > **Channel:** JSON-RPC `error` for protocol faults, including `-32600` for frames over `MAX_FRAME` and `-32602` for `release`-with-`from`, and `result.refusal` for `not-delivered`/`control-character` and `not-delivered`/`empty-text`.
- (.andromeda/security-plan-detail/error-handling.md:15) The CLI enumeration must name this exact `--json` document, and its witness must hold every frame verb's strict-modes refusal to it, with a code, a null `detail` and no path.
  > CLI `send` · `wait` · `last` · `answer` · `pause` · `release`, as `link` / `unlink`, run the instance's strict-modes check (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`) before any frame: a refusal exits 21, prints the instance pair (`human::instance_state_pair`) through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, opens nothing under the home and writes no line.
  (.andromeda/security-plan.md:409)
  > **External responses** (to channel clients, CLI `--json` and MCP drivers): sanitised.
  > - No anyhow context chains, no absolute paths, no upstream text (prompts, tool `input`, assistant output), no serde_path_to_error paths or values, and no internal type names.
  (.andromeda/security-plan.md:482)
  > NEVER echo serde_path_to_error drift reports or anyhow chains into MCP `structuredContent` or channel `error.data`, and NEVER print an anyhow chain to stderr that still contains a serde_json or serde_path_to_error source (its `Display` quotes input values such as tool `input`).
  (.andromeda/security-plan.md:487)
  > NEVER expose absolute paths in any external error.
- (.andromeda/security-plan-detail/error-handling.md:16) The CLI enumeration must account for `revive --json`'s `strict-modes-failed` as a code with no path, while the shape of that start-refusal document stays with the exit-cause catalogue.
  > `viola run` and `viola revive`'s start run the home tree's strict-modes check (`check_home_tree`, over the instance and the pinned copy's key) before the wrapper log opens: a refusal exits 1 with the home pair for a home-level refusal or the instance pair for an instance-level one, and opens no log, writes no line and starts no child (`revive --json` prints `strict-modes-failed`).
- (.andromeda/security-plan-detail/error-handling.md:17) The chunk's read of every product `Display` must hold these two enums to their fixed messages, with no path or payload.
  > `viola_state::stamps::StampsError` (`StrictModes` · `State`, the error `update_stamps` returns) and `viola_state::pin::PinError::StrictModes` carry fixed `Display` messages with no path or payload: `the capability stamps failed the strict-modes check`, a write failure its own fixed message, and `the pinned copy failed the strict-modes check`; `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`.
  (.andromeda/security-plan.md:411)
  > The `Display` impls on `PtyError` (hand-written), `ChannelError`, `StateError`, `AgentError`, `McpError` and `CoreError` (`thiserror`) use fixed messages. Variant fields that hold paths or payloads are excluded from `Display`.
- (.andromeda/security-plan-detail/error-handling.md:18) The re-read of the owner-readable detail leg must read this rule too: a detail file that fails its check takes no detail line, and that line must then reach no other file, stderr or any of the three surfaces.
  > A `diagnostics/` folder, role file or detail file that fails `strict::check_diagnostics` (a symbolic link, or a mode or grant another user may use) is a failed open, checked before anything is narrowed: the role writer ends at `std::io::sink` and the detail line is dropped.
  (.andromeda/security-plan.md:416)
  > Full error detail goes only to `instances/<name>/diagnostics/` (files 0600; format owned by obs) or, for short-lived CLI verbs, to stderr. stderr never carries upstream text, tool `input`, drift-report values or any other value on the NEVER-log floor (Bootstrap phases, `logging-redaction-wire`).
  (.andromeda/security-plan.md:486)
  > NEVER create `diagnostics/` files readable by other users.
- (.andromeda/security-plan-detail/security-anti-patterns.md:39) No terminal colour fact may stand in an error body of the three surfaces, and the enumeration must keep that visible.
  > let a colour fact change a control or reach a log line, error or refusal, without its own Decisions Log entry.
  (.andromeda/security-plan.md:454)
  > NEVER hand rmcp a `viola mcp` stdin reader that does not cap each line at `MAX_FRAME`, nor truncate or skip an over-long line instead of ending the session.
  (.andromeda/security-plan.md:465)
  > NEVER spawn a `.cmd`/`.bat` PTY child through portable-pty. Resolve to a real `.exe` (portable-pty 0.8.1 bypasses std's BatBadBut escaping).
  (.andromeda/security-plan.md:472)
  > NEVER skip request size limits: `MAX_FRAME` on the channel and on each `viola mcp` stdin line.

## Bindings
- **Secret Management's planted value ↔ Error Handling's external responses**: the witness per surface proves the two together, the planted stripped value and the path and upstream text absent from the same body, `(.andromeda/security-plan-detail/secret-management.md:9)` and `(.andromeda/security-plan.md:410)`.
- **Secret Management's floor ↔ Error Handling's detail files**: full detail may go to `diagnostics/`, and a stripped value may not go even there, so moving a chain out of an error body must not carry a planted value into a detail file, `(.andromeda/security-plan.md:416)` and `(.andromeda/security-plan-detail/secret-management.md:9)`.
- **Error Handling's channel bodies ↔ Error Handling's MCP `wrapper-fault`**: the JSON-RPC fault's `{code,message,data}` is what the driver reads, so the channel's fixed messages and data bind the MCP surface too, `(.andromeda/security-plan.md:413)` and `(.andromeda/security-plan-detail/error-handling.md:14)`.
- **Security Anti-Patterns' colour-fact clause ↔ Error Handling's external responses**: a body of a closed code and a fixed message holds no colour fact, and the enumeration reads both rules over the same bodies, `(.andromeda/security-plan-detail/security-anti-patterns.md:39)` and `(.andromeda/security-plan.md:410)`.
