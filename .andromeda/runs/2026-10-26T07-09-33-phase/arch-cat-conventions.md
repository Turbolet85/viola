# arch · Conventions — category read

## Carried
- (.andromeda/architecture-detail/conventions.md:34) A CLI verb whose server verification fails or cannot be made must exit 21 with the `instance-unreachable` document, the code a refused strict-modes check already takes for `viola list` and `viola plugin install`; no new exit code is added.
  > `21` instance unreachable (no live endpoint for the `ViolaName`); also `viola list`'s refused strict-modes check of the home, `instances/` or any one instance, which refuses the whole board (`--json` `{"v":1,"error":"instance-unreachable","detail":null}`, no log opened)
- (.andromeda/architecture-detail/conventions.md:26) For `wait` / `last` the verification failure must land on the exit-21 arm (beside a failed connect or a closed / reset connection), never on the exit-1 arm that covers other channel errors.
  > for `viola wait` / `viola last` exit 1 also covers a refusal reply, a reply of no known shape, and any channel error but a failed connect or a closed / reset connection (those are exit 21)
- (.andromeda/architecture-detail/conventions.md:26) A verification mismatch must not surface as a dispatch `Err` in any of the client verbs, since that path prints `error: internal error` and exits 1; it is a typed unreachable outcome.
  > so `send`, `wait`, `last`, `list`, `answer`, `pause`, `release`, `verify`, `revive --list` and `plugin install`), and a dispatch `Err` or a caught panic in any of them prints exactly `error: internal error` on stderr once, from the `main` catch site, no hint — no dispatch arm prints it
- (.andromeda/architecture-detail/conventions.md:27) The hook client's verification must not introduce a usage-error or any non-zero exit for `viola hook`.
  > `2` usage error (clap, a second positional to `viola send` included; never produced by `viola hook`)
- (.andromeda/architecture-detail/conventions.md:8) A failed server verification is not a refusal: no `RefusalReason` or `detail` value may be added for it outside `viola-core`'s closed sets, and the mismatch travels as the `instance-unreachable` error, not a `result.refusal`.
  > `detail` values are kebab-case strings from a closed set per reason, extended only in `viola-core`:
- (.andromeda/architecture-detail/conventions.md:15) `send`'s client-side checks stay "before any frame" with their order and exit 13 unchanged; verification must slot in before the first frame without reordering the wrapper's refusal sequence.
  > For `send`: `not-delivered`/`control-character` first (`viola_core::validate_paste_text`: LF, CR and TAB pass; every other C0, DEL and C1 is refused, never stripped; the CLI checks before any frame and the wrapper checks again)
- (.andromeda/architecture-detail/conventions.md:15) `answer`'s client-side check likewise stays before any frame, unchanged by the verification.
  > For `answer`: `not-delivered`/`control-character` first (`validate_paste_text` on the response's free text; the CLI checks before any frame and the wrapper checks again)
- (.andromeda/architecture-detail/conventions.md:40) The MCP `send` / `wait` / `last` / `answer` tools must report a failed verification as the `isError: true` `instance-unreachable` result with `detail: null`, the same shape as CLI `--json`.
  > An unreachable instance and a wrapper protocol fault are also `isError: true` tool results. Their `structuredContent` is `{"error":"instance-unreachable"|"wrapper-fault","detail":…}`, the same as CLI `--json` (`instance-unreachable` with `detail: null`; `wrapper-fault` with the JSON-RPC fault's `{code,message,data}`).
- (.andromeda/architecture-detail/conventions.md:40) The MCP handler's argument re-validation must stay ahead of any frame, and the verification's `isError` text must be `refused: instance-unreachable`.
  > a failure is an `isError: true` result `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`) with no frame written; every `isError` text is `refused: <code>`.
- (.andromeda/architecture-detail/conventions.md:40) A verification failure in an MCP tool must not fall into the `-32603` internal-handler-fault arm; JSON-RPC errors stay the MCP connection's own faults.
  > JSON-RPC errors are the MCP connection's own faults, an unknown tool name (`-32602`) and an internal handler fault — any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call — which is `-32603` `internal error` with `data: null`.
- (.andromeda/architecture-detail/conventions.md:24) If the strict-modes half is taken here, the existing codes-only document writer `client::refused_document` is the one that prints a refused verb's `--json` document; `strict-modes-failed` with exit 1 is `revive`'s precedent, while `list` and `plugin install` map a strict-modes refusal to exit 21.
  > under `--json` each of these revive refusals, the strict-modes one included, is instead one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`; `client::refused_document`) with stderr empty, the same exit 1 and the same log lines
- (.andromeda/architecture.md:100) The body's exit-code catalogue fixes 21 as instance unreachable; the chunk adds no code to the `0, 1, 2, 10–14, 20, 21` set.
  > `20` protocol fault from the wrapper · `21` instance unreachable, and a refused strict-modes check of `viola list` or `viola plugin install`.
- (.andromeda/architecture.md:102) The hook client's mismatch must exit 0, as every hook failure does.
  > **`viola hook` exit codes:** always 0, including on clap parse errors, channel failures and panics caught at the edge. Exit 2 is forbidden.
- (.andromeda/architecture.md:104) MCP outcomes stay in their two classes: an unreachable instance is an `isError: true` result, never a JSON-RPC error.
  > a refusal, an unreachable instance, a wrapper protocol fault and an argument that fails re-validation (`invalid-params`, no frame written) are `isError: true` results shaped like CLI `--json`; JSON-RPC errors are only the MCP connection's own faults: an unknown tool name (`-32602`) and an internal handler fault (`-32603`).
- (.andromeda/architecture.md:91) The verification must precede the first frame of every channel method a client writes: the request methods (`hook.dialog` included) and the `hook.event` notification; `list` writes no frame and is outside it.
  > Requests: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`. `list` is not a channel method. Notifications: `hook.event`.
- (.andromeda/architecture.md:41) The verification is client-local and must add no handshake frame to the channel.
  > Every `params` carries `v` and the sender's `CARGO_PKG_VERSION`, with no handshake.
- (.andromeda/architecture.md:89) Any wire change the chunk makes must be additive (no `v` bump); removing or changing a field's meaning would bump `v`.
  > Adding a field or event kind is additive and needs no bump. Removing or changing a field's meaning bumps `v`.
- (.andromeda/architecture.md:176) Server identity must come from the endpoint and the snapshot, never from a client-supplied `conn` or any `params` field.
  > A client may add `conn` (obs-plan D-10) to `params`; the server logs it and strips it before dispatch, so no method takes it as input and it is never identity.
- (.andromeda/architecture.md:106) A new verification error must be a variant of the owning crate's `<Crate>Error` enum with a fixed message (no path, pid or payload).
  > one error enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `CoreError`), each a `thiserror` enum except `PtyError`, whose `Display`/`Error` are hand-written with fixed messages
- (.andromeda/architecture.md:156) The compared start time must be the snapshot's `started_at` (sysinfo, whole seconds, RFC 3339) against the same reading of the connected server's pid; a replay rebuild carries no `endpoint`.
  > `started_at` is the wrapper's own OS start time (sysinfo, whole seconds) rendered RFC 3339, so a reader can compare it with the same field of the recorded `pid`. `endpoint` is present from the first snapshot, because the bind precedes it; only a replay rebuild lacks it.
- (.andromeda/architecture.md:157) `pid` and `started_at` are the wrapper's identity fields the verification matches.
  > `pid` and `started_at` identify the wrapper process for the liveness check.
- (.andromeda/architecture.md:152) An unreadable or newer snapshot is ignored and a replay recovers only log-derived fields (no `pid`, `started_at` or `endpoint`), so a client must not verify against a replayed state; today's outcome holds under CARRY 7's hold.
  > A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored, and the state is rebuilt by replaying `events.ndjson`, which recovers only the log-derived fields (`links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`); the replay writes no file.
- (.andromeda/architecture.md:243) The pid-plus-start-time match must agree with the liveness rule: a pid alive with another start time is not the recorded wrapper.
  > The wrapper touches `<instance dir>/heartbeat` every 1 s. When the snapshot's pid + start time no longer name a running process (the pid is dead, or alive with another start time) the session is `gone`, whatever the beat's age. Otherwise a beat at most 5 s old is `live`, and an older or absent beat is `stale`.
- (.andromeda/architecture.md:263) Clients keep connecting to the snapshot-recorded endpoint; the verification is added on that connection.
  > every other process connects to the `endpoint` recorded in the instance snapshot.
- (.andromeda/architecture.md:39) On Windows the verifying client must keep the SQOS Identification open, never interprocess's default connect.
  > endpoint names are a short hash of the instance; on Windows the client connects only through the SQOS Identification open, never interprocess's default connect.
- (.andromeda/architecture.md:45) A hook whose verification fails loses its event, not the turn.
  > A lost endpoint loses the event, not the turn.
- (.andromeda/architecture.md:405) The hook client's mismatch is an internal failure that must fail open with no body.
  > Any internal failure in `hook` produces exit 0 with no body, so the dialog renders for the human.
- (.andromeda/architecture.md:412) The hook's verification path must write nothing to stderr and nothing but a decision body to stdout.
  > `hook` writes only its decision body to stdout and nothing to stderr;
- (.andromeda/architecture.md:410) The CLI and hook verification must be std-thread blocking code; only the MCP tools' path may use the Tokio client.
  > New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime. CLI `wait` is a blocking read on `viola-channel`'s sync client.
- (.andromeda/architecture.md:291) If the verification lives in `viola-channel`, its sync half must keep the crate's normal graph tokio-free, the Tokio client's half behind the `tokio` feature.
  > `viola-channel` left that list at chunk 2026-10-17-mcp-server-for-drivers: it declares `tokio = ["dep:tokio", "interprocess/tokio"]` (its Tokio client, enabled by `viola-mcp`) and `fake-agent = ["tokio"]`, so its package-scoped mutation build compiles and tests the Tokio client; its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh`.
- (.andromeda/architecture.md:420) The per-connection proof is the security specialist's to define per OS; this plan only requires that no other OS user's endpoint is trusted.
  > Each endpoint, whether a named pipe or a Unix socket file, must not be connectable by another OS user. The security specialist owns how this is enforced per OS and any per-connection proof beyond it.

## Bindings
- **exit 21 / `instance-unreachable` ↔ security-plan client-side server verification row and security.md's eight dated exceptions**: the mismatch outcome the security side requires ("CLI/MCP report unreachable") is this category's exit 21 and its `--json` document, (.andromeda/architecture-detail/conventions.md:34), (.andromeda/architecture.md:100)
- **MCP `instance-unreachable` result ↔ security.md's eighth exception (`viola mcp`'s frame tools)**: the Tokio-client tools report a failed check as the `isError: true` unreachable result, never `-32603`, (.andromeda/architecture-detail/conventions.md:40), (.andromeda/architecture.md:104)
- **`viola hook` always 0 ↔ body §Established Decisions [Hook Contract] and §Cross-cutting Patterns fail open**: the hook client's mismatch exits 0 with nothing on stdout or stderr and no frame, (.andromeda/architecture.md:102), (.andromeda/architecture.md:405), (.andromeda/architecture.md:412)
- **strict-modes refusal code ↔ "Home and code-bearing file integrity" (the strict-modes half)**: if the half is taken here, its code follows `list` / `plugin install` (exit 21) or `revive` (exit 1 `strict-modes-failed`) through `client::refused_document`, a choice for P4, (.andromeda/architecture-detail/conventions.md:24), (.andromeda/architecture-detail/conventions.md:34)
- **snapshot `pid` / `started_at` ↔ §Standard Contracts Session liveness and the security-plan's sysinfo start-time check**: the compared fields and their whole-second sysinfo reading, (.andromeda/architecture.md:156), (.andromeda/architecture.md:243)
- **replay ignoring an unreadable or newer snapshot ↔ the scope's CARRY 7 hold (the founder's to rule)**: a replay has no `pid`, `started_at` or `endpoint`, so today's outcome (absent, exit 21 / fail open) stands, (.andromeda/architecture.md:152)
- **Tokio containment ↔ `viola-channel`'s `tokio` feature and `scripts/channel-tokio-free.sh`**: the sync clients verify with blocking code, the MCP client's half under the feature, (.andromeda/architecture.md:410), (.andromeda/architecture.md:291)
