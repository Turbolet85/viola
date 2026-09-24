## 1. Architecture Excerpt

### Stack (instrumentation surfaces)
- **Rust stable (host 1.95; workspace `rust-version = "1.89"`, edition 2024)**: builds the single native binary `viola` / `viola.exe`, which serves every surface (CLI, hook, MCP, GUI). The obs stack must compile at MSRV 1.89.
- **Hybrid concurrency (std threads + Tokio 1.53.1)**: std threads run `run`, `hook`, `send`, `wait` and every other CLI verb. Tokio is built only inside `mcp` and `ui`. `cargo deny` bans `tokio` anywhere in the normal dependency graph of `viola-core`, `viola-pty`, `viola-channel` (without its feature), `viola-state` and `viola-agent-claude`. Obs deps on those crates must be sync, and no exporter may pull in a Tokio runtime there.
- **axum 0.8.9 (feature `sse`) + tower-http 0.7.1 (`CompressionLayer`, with `text/event-stream` excluded)**: the view-only GET routes and the SSE feed on 127.0.0.1 (`viola ui`). The HTTP server span surface. tower-http tracing middleware can be used here.
- **clap 4.6.7 (derive)**: CLI dispatch for `run · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install`. Each subcommand is a short-lived process root, except `run`, `mcp` and `ui`.
- **portable-pty `=0.8.1` (pinned) behind viola's own `pty` seam; windows-sys 0.61.2 `TerminateProcess` kill fallback**: hosts the unmodified `claude` in ConPTY or openpty. The seam operations (spawn, read, write, resize, wait, kill) need manual instrumentation. Child exit is detected on the process handle, never on EOF.
- **vt100 0.16.2**: screen model for pre-send readiness and modal detection on `run`'s pump thread. It is never used to read content. Gate outcomes such as `input-not-ready` are candidates for manual instrumentation.
- **interprocess 2.4.4 `local_socket`**: sync API, with the Tokio flavour only in `viola-mcp`. One endpoint per `viola run` (a named pipe on Windows, a Unix domain socket elsewhere). This is the wrapper IPC boundary where cross-process context propagation happens, and it needs manual instrumentation.
- **ndjson append logs + atomically replaced JSON snapshots (no database)**: the authoritative audit trail and mutable state (wheel mirror, links, budget readings). The event log is itself a structured telemetry stream.
- **atomic-write-file 0.3.1; std `File::lock` on separate `.lock` files; std `OpenOptions::append` (one `write` per line)**: crash-safe multi-process writes. Any file-based diagnostics sink has to follow the same one-write-per-line discipline.
- **serde 1.0.229, serde_json 1.0.151, serde_path_to_error 0.1.20**: channel frames, log events, snapshots, and tolerant parsing of external payloads with path-precise drift reports. Drift reports are an error-capture touch point.
- **chrono 0.4.45**: RFC 3339 UTC timestamps with millisecond precision. Already in the tree through rmcp. Log and event timestamps must match this format.
- **thiserror 2.0.20 (per crate); anyhow 1.0.104 (bin edge only)**: typed errors per crate, with context chains only at dispatch. This is the error-capture boundary.
- **notify 8.2.0**: tails the ndjson logs for fan-out to `ui` (SSE). No broker.
- **SSE via axum `Sse::keep_alive`**: the live GUI feed `/api/events`. A long-lived streaming connection.
- **sysinfo 0.39.6 (pid + process start time) + `claude agents --json`**: process liveness that enriches the primary heartbeat-file signal.
- **rmcp 3.4.1 (features `server`, `transport-io`), minor pinned; schemars 1.2.2**: stdio MCP server (spec 2026-07-28) with tools `send · wait · last · answer · list`. stdout is reserved for MCP frames, so no telemetry may go to stdout.

### Workspace / Modules
- **viola (root bin)**: clap dispatch, the `run` PTY pump, the wheel, the budget governor and the readiness gate wiring. The only crate that uses anyhow.
- **viola-core**: normalised events, `RefusalReason`, `ViolaName`, `Percent` and the `v` constants. Depends on no other viola crate.
- **viola-pty**: the PTY seam over portable-pty `=0.8.1` plus the windows-sys kill fallback. Knows no agent.
- **viola-channel**: JSON-RPC 2.0 over ndjson on interprocess local sockets. Sync client and server, plus a Tokio client behind the `tokio` feature, which only `viola-mcp` enables.
- **viola-state**: ndjson logs, atomic snapshots, `File::lock`, torn-line healing, tailing through notify, and the sysinfo liveness check.
- **viola-agent-claude**: Claude hook parsing, dialog mapping, R8 env strip, npm-shim resolution, the capability ledger, screen signatures and statusline parsing. The only crate that knows Claude payload shapes.
- **viola-mcp**: rmcp 3.4.1 stdio server, a thin adapter over viola-channel. Uses Tokio.
- **viola-ui**: axum 0.8.9 GET routes, SSE and the Host allowlist. Uses Tokio.

### Standard Contracts
- **`GET /health`** (HTTP endpoint): GUI liveness, returning `{"v":1,"status":"ok","name":"viola","version":…,"ts":…}`.
- **`GET /ready`** (HTTP endpoint): readiness, 200 or 503, with `checks` `{viola_home, event_tail, claude_agents}` valued `ok · unavailable · error`. A `claude_agents` failure alone does not make it not-ready.
- **`GET /api/info`** (HTTP endpoint): service info `{v, name, version, proto, pid, started_at, bind, viola_home, verified_cli_versions}`.
- **`GET /api/sessions`** (HTTP endpoint): the session list envelope plus a top-level `budget` and `skipped` counts (`unknown_kinds`, `unknown_fields`, `torn_lines`).
- **`GET /api/links`** (HTTP endpoint): the link list `{driver, driven, since}`.
- **`GET /api/events`** (HTTP SSE endpoint): `event:` is the normalised kind, `id:` is a composite cursor `<ViolaName>:<byte offset>,…`, and `data:` is the ndjson line verbatim. Keep-alive every 15 s, never compressed. Resumes via `Last-Event-ID`.
- **`/`, `/assets/*`** (HTTP endpoint): the view page and embedded static files. Any non-GET method returns 405. Errors are RFC 9457 Problem Details with URNs `urn:viola:problem:{host-not-allowed|method-not-allowed|not-found|state-unreadable}`.
- **Reserved `POST /api/sessions/{name}/pause`, `POST /api/sessions/{name}/unlink`** (HTTP endpoint, v1.x only, not served in v1): the brake routes.
- **`send`** (IPC method): `{text, from?}` returns `{submitted_at, cursor}` or `{confirmed:false, detail:"unconfirmable", cursor}`. Blocks for the readiness gate, the paste and the confirmation window. Every `params` carries `v`, `sender` (`CARGO_PKG_VERSION`) and optionally `from` (the caller's `VIOLA_NAME`). No trace-context field is declared.
- **`wait`** (IPC method): `{after?, timeout_ms?}` returns `{event, cursor}` or `{timed_out:true}`. A long-blocking call.
- **`last`** (IPC method): `{}` returns `{last_assistant_message, ts}`.
- **`answer`** (IPC method): `{dialog_id, from?, response}` returns `{}`.
- **`pause`** / **`release`** (IPC methods): wheel control, returning `{wheel}` / `{wheel, budget_paused}`. CLI-only.
- **`link`** / **`unlink`** (IPC methods): `{driver, driven}` returns `{}`. Sent to the driven instance's wrapper.
- **`hook.dialog`** (IPC method): sent by `hook`, `{kind, data}` returns `{dialog_id, response|null}`. The hook blocks up to the dialog deadline.
- **`hook.event`** (IPC notification, no id): the hook event path for SessionStart, UserPromptSubmit, Stop and the async hooks.
- **JSON-RPC frame shapes** (schema): success is `result.ok`, a refusal is `result.refusal` + `detail`, and a protocol fault is `error` with codes -32700, -32600, -32601, -32602 (unsupported `v`) or -32603. JSON-RPC ids are integers, monotonic per connection.
- **`list`** (CLI/MCP read, not a channel method): the caller reads `instances/*/` and `claude agents --json` directly and returns `{items, budget, skipped}`.
- **MCP tools `send · wait · last · answer · list`** (stdio JSON-RPC 2.0): refusals, `instance-unreachable` and `wrapper-fault` all come back as `isError: true` tool results with `structuredContent`. Only faults on the MCP connection itself are JSON-RPC errors. The server adds `from` from its own `VIOLA_NAME`.
- **Hook contract** (Claude Code hook stdin/stdout): exec-form `viola hook <event>`. A decision is exit 0 with a `hookSpecificOutput` body, and no decision is exit 0 with empty stdout. `hook` never exits 2 and never writes to stderr. Registered events: SessionStart, UserPromptSubmit, PreToolUse (matcher `AskUserQuestion|ExitPlanMode`), PermissionRequest, Stop, SessionEnd, Notification, PostToolUse, PostToolUseFailure.
- **`viola hook statusline`** (process contract): records `rate_limits` to `budget.json`, then runs the user's statusline command through the shell. This is the only shell-out in viola.
- **ndjson event line** (schema, `instances/<ViolaName>/events.ndjson`): `{"v":1,"ts":…,"instance":…,"kind":…,"source":"hook|wrapper|cli","data":{}}`. Kinds are `session-start`, `turn-ended`, `prompt-submitted`, `question`, `permission`, `plan`, `session-end`, `activity`, `link`, `unlink`, `wheel` and `budget-gate`.
- **Snapshot envelope / instance snapshot** (schema): `{"v":1,"written_at":…,"writer":<CARGO_PKG_VERSION>,"data":{…}}`. The instance `data` holds `endpoint, pid, started_at, pinned_bin, cli_version, cli_verified, wheel, budget_paused, pending_dialog, links, child_pid, agent_session_id`.
- **CLI `--json` output + exit codes** (schema): `{"v":1,"ok"|"refusal"|"error":…}`. Exit codes: 0 ok · 1 internal error or start collision · 2 usage · 10 `human-typing` · 11 `budget-paused` · 12 `unverified-cli` · 13 `not-delivered` · 14 `unknown` · 20 wrapper protocol fault · 21 instance unreachable. `viola hook` always exits 0.
- **`RefusalReason`** (schema, kebab-case): `human-typing` (`null`/`manual-pause`) · `budget-paused` (`five-hour`/`seven-day`) · `unverified-cli` · `not-delivered` (`input-not-ready`/`no-prompt-submitted`/`turn-running`/`unknown-dialog`) · `unknown`.
- **Heartbeat** (file contract): the wrapper touches `<instance dir>/heartbeat` every 1 s. A beat older than 5 s means gone, or `stale` if the pid + start-time check says the process is still alive.

### Surfaces
> **Product type.** Hybrid local developer tool made of three parts:
>   - a native cross-platform CLI binary;
>   - a Claude Code plugin (hooks + stdio MCP server) that calls that binary;
>   - a minimal view-only local web GUI served by the same binary.

Deferred surfaces: "[Mobile] N/A in v1. Rationale: the phone view is the same web page behind authentication, in a later version." v1.x adds the GUI brake and then public distribution. There is no hosting (local-only v1). Windows is live-supported, and macOS and Linux are CI-tested.

### Observability Hints
**1. Cross-cutting Patterns (there is no dedicated Logging subsection; these are the logging-relevant items, quoted verbatim)**
- "**Diagnostic output channels.** `hook` writes only its decision body to stdout and nothing to stderr (Hook Contract). While the child runs, `run` writes nothing to the terminal except the child's own output, because the terminal carries the child's screen. Its diagnostics go to the instance's `diagnostics/`. `mcp` writes only MCP frames to stdout. Its diagnostics go to stderr, or to the instance's `diagnostics/` when `VIOLA_DIR` is set. `ui` and short-lived CLI verbs may use stderr. The logger and the format are owned by obs."
- "**Config management.** Precedence: CLI flags override `<viola home>/config.json`, which overrides built-in defaults." "`config.json` is parsed tolerantly (unknown keys skipped and counted) and carries `v`." "There are no secrets in v1, and environment variables are not a configuration channel. The `VIOLA_*` variables are wrapper-to-child plumbing set by `viola run` only." "Viola home: a global `--home <dir>` flag on every subcommand overrides the default `<user home>/.viola/`. Resolution order: `--home`, then the grandparent of `VIOLA_DIR` when it is set, then the default."
- From the Hook Contract decision: "`hook` never writes to stderr. Its diagnostics go only to the instance's `diagnostics/` directory (format owned by obs), and a failure to write them is ignored."
- From Hook Transport: "If the endpoint cannot be reached or written within the deadline, the hook records the failure in `diagnostics/` and exits 0."
- "**Tokio containment.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` and `viola-ui` may build a runtime."
- "**Crash-safe disk writes.** Append-only logs use one `write` per line. Mutable files are replaced atomically. Every guarded file has its own `.lock` sibling. Readers always tolerate a torn last line."
- "**Mixed-version tolerance.** Every channel `params`, event, snapshot and config carries `v`, and channel `params` also carry the sender's `CARGO_PKG_VERSION`. Readers skip unknown kinds and fields and surface the counts."
- "**Local endpoint trust boundary.** … v1 trusts every process of the same OS user and no other."
- Build policy constraint: `cargo deny check` bans "C-building crates such as `cc`, `libsqlite3-sys` and `openssl-sys", along with `tokio` in the sync crates' graph, evaluated for the Windows, macOS and Linux targets.

**2. Inherited Defaults**
- Inherited Defaults names no logging library, OTel SDK, error-reporting platform or crash-reporting toggle. The closest obs-adjacent entries:
  - "Errors: thiserror 2.0.20 per crate, anyhow 1.0.104 only in the root bin; typed CLI exit codes; `viola hook` always exits 0."
  - "Timestamps: chrono 0.4.45, RFC 3339 UTC with milliseconds and `Z`, field `ts` / `*_at`."
- Established Decisions [Deferred] says: "Deferred to specialists and later versions: … logging and observability (obs) …"

**3. Observability Design**
(No dedicated Observability Design section in arch — Phase 3 will derive defaults from Cross-cutting Patterns + Inherited Defaults + stack research.)

### Project Intent Summary
- **Core functionality:** "Lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified `claude` CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment."
- **Target users:** Growth model: "Modular monolith: one binary with compiler-enforced internal crates (`pty · channel · state · agent-claude · mcp · ui` around `core`), and no daemon." Users: "The founder runs v1 on their own subscription, with no accounts, no hosting and the GUI on 127.0.0.1", and "the later public version (individual Claude Code subscribers) must not need a rewrite."
- **Critical paths hint:** arch describes these flows in Established Decisions and Project Intent:
  1. `viola run` start sequence: "collision check, pinned copy and plugin folder, version gate, exclusive bind of the endpoint …, first snapshot … and heartbeat, the start `wheel` and `budget-gate` events, and only then the child spawn."
  2. Confirmed send: "Every `send` is confirmed. If no matching `prompt-submitted` arrives within the confirmation window, the send is reported as `not-delivered` with detail `no-prompt-submitted`." It is gated first by the vt100 readiness gate (`input-not-ready`).
  3. `wait` "blocks until the next driver-relevant event (`turn-ended`, `question`, `permission`, `plan` or `session-end`) and returns that event."
  4. Dialog answering: a sync hook sends `hook.dialog`, the wrapper assigns `dialog_id` and logs it, the driver `answer`s, and the hook emits its decision. On deadline expiry the hook exits 0 with no body and the dialog renders for the human.
  5. Human takeover: "Any human editing key since the last turn boundary moves the wheel to the human, and `send` is refused with `human-typing`." The wheel returns only through `viola release`.
  6. Budget governor: "Once a configured threshold is crossed (defaults `five_hour` ≥ 90 %, `seven_day` ≥ 85 %), automated `send` is refused with `budget-paused`." Evaluated at start, on `send`, and at each `turn-ended`.
  7. GUI view: the page loads `/api/sessions` + `/api/links`, then tails the live SSE `/api/events`.

### CI/CD Platform
- **Platform:** GitHub Actions (one workflow `.github/workflows/ci.yml`, native matrix `windows-2025`, `macos-latest`, `ubuntu-latest`; `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2.9.2`).
- **Pipeline note:** On push and PR, each OS runs fmt check, clippy `-D warnings`, `cargo check` of the sync crates without Tokio, `cargo deny check` (on ubuntu only), the workspace tests against the fake agent replaying `fixtures/claude/*`, and a release build. The real `claude` CLI and `viola verify` never run in CI. There is no deploy stage in v1.

### Obs-Relevant Conventions
- **Diagnostics location:** `<viola home>/instances/<ViolaName>/diagnostics/` (default viola home `~/.viola/`) is "reserved for `hook`, wrapper and in-session `mcp` diagnostics; file format owned by obs". It is located via the `VIOLA_DIR` env var.
- **Output channel rules:** `hook` writes nothing to stderr and only its decision to stdout. `run` writes nothing to the terminal while the child runs. `mcp` stdout carries MCP frames only (diagnostics go to stderr, or to `diagnostics/` when `VIOLA_DIR` is set). `ui` and short-lived CLI verbs may use stderr.
- **Audit event log:** `instances/<ViolaName>/events.ndjson`. v1 never rotates or truncates it, because byte offsets serve as cursors (`wait` `after`, `send` `cursor`, SSE ids). Bounding its size is an open item.
- **ndjson line discipline:** one complete JSON object plus `\n` per single `write` call. Multi-line text travels as an escaped JSON string.
- **Timestamps:** field `ts` on every event and frame, `*_at` for other instants. RFC 3339 UTC with milliseconds and a `Z` suffix (chrono `to_rfc3339_opts(SecondsFormat::Millis, true)`).
- **JSON / enum naming:** snake_case field names. Enum values on the wire are kebab-case (event kinds, refusal reasons, details, wheel holders).
- **Service identity:** the binary name is `viola`. Version comes from compile-time `CARGO_PKG_VERSION`, which is carried as `sender` in every channel `params`, as `writer` in snapshots, and in `/api/info` and `/health` (`"name":"viola"`). The instance identity is `ViolaName` (ASCII `[a-z0-9-]`, 1–32 chars) via `VIOLA_NAME`. The endpoint name is `viola-<12-hex FNV-1a 64>`.
- **Instance identity rule:** "Every hook, MCP call and CLI verb finds its instance through `VIOLA_NAME` (or an explicit `ViolaName` argument), never through the working directory."
- **Env var prefix:** `VIOLA_` in SCREAMING_SNAKE_CASE (`VIOLA_NAME`, `VIOLA_DIR`, `VIOLA_BIN`). Env vars are not a configuration channel in v1.
- **Unknown-record counting:** readers skip unknown kinds and fields and surface `skipped` counts (`unknown_kinds`, `unknown_fields`, `torn_lines`). Unparseable external values become `"unknown"`, never an error.
- **Log level / span naming conventions:** none declared in arch.
- **Existing scopes present:** no.

## 2. Security Plan Excerpt

### Security Tier
- **Tier:** Minimal, stated as "Minimal (0), with targeted elevations for the local privilege boundary". The elevations cover four things: IPC endpoint access control and server impersonation, GUI cross-user and cross-origin readability, GUI output encoding, and the integrity of `~/.viola/`.
- **Justification:** "viola is a local-only, single-user tool. It has no accounts, no public network listener, no database, no stored credentials and no regulated data […] which rules out Standard's user-account and HTTPS concerns and Hardened's compliance drivers."
- **Compliance triggers:** None. There is no payment, health, children's or third-party personal data. All logged content is the operator's own Claude session content, kept on their own machine.
- **Obs-relevant framing from the plan:**
  - "Logging format and fields belong to obs."
  - Obs owns the logger and the format. Security skips its own Logging section at Minimal tier, and its only contribution is a consolidated NEVER-log floor (the `logging-redaction-wire` bootstrap phase).
  - `diagnostics/` file format is owned by obs.
  - Error reporting integration: none. viola makes no outbound network calls. "If obs ever adds an external reporter, the Logging bans below apply before transmission."

### Logging-Sensitive Vectors
- **Vector 1:** Local IPC wrapper channel (JSON-RPC 2.0 over ndjson; named pipe `\\.\pipe\viola-<h12>` or Unix socket; methods `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, `hook.event`). Logging implications:
  - `hook.event` notifications (UserPromptSubmit, Stop, PostToolUse), `wait`/`last` results and `hook.dialog` carry prompt text, `last_assistant_message` and tool data. That content may only go into the contract payloads. It never goes into log or error lines or into channel `error.data`.
  - Channel `-32603` uses the fixed message "internal error" with `data: null`.
  - The `from` parameter is self-reported. It must not be treated as an authenticated identity.
- **Vector 2:** CLI `send` text written into the PTY (from stdin, `--file` or MCP `send.text`). Logging implications:
  - Sent text is user content. It never goes to stderr, CLI `--json` error output or MCP `isError` results. Only `instances/<name>/diagnostics/` may hold it.
  - A refusal is reported as the code `not-delivered` with detail `control-character`, never by echoing the text.
- **Vector 3:** Loopback HTTP GUI (`viola ui` on `127.0.0.1:47319`; SSE `/api/events` streams raw event lines including prompts, assistant output and permission `input`). Logging implications:
  - The per-launch token travels in `GET /?t=<token>` until the 303 redirect. Any HTTP request or span logging in `viola-ui` must record the path without its query string.
  - The `Cookie` header (`viola_<port>`) must never be logged.
  - Problem Details use fixed `detail` strings. `urn:viola:problem:unauthorized` never contains the token, the launch URL, or the `.url` path or contents. `urn:viola:problem:state-unreadable` must not name the unreadable path.
  - The only intended path disclosure is `viola_home` in `/api/info`, which is behind the cookie.
- **Vector 4:** Hook stdin payloads from Claude Code (`viola hook <event>`, plus `hook statusline`, which shells out `statusline_command`). Logging implications:
  - `hook` never writes to stderr and never exits non-zero, even when a security check fails (server verification mismatch, oversized stdin, strict-modes failure). It fails open with exit 0 and no body, and writes a note only to `diagnostics/`.
  - serde_path_to_error drift reports can quote tool `input` and go only to `diagnostics/`.
- **Vector 5:** MCP over stdio (tools `send`, `wait`, `last`, `answer`, `list`; the driver is an LLM). Logging implication: MCP `isError` results carry only the `structuredContent` codes (`refusal`/`detail`, or `error: instance-unreachable | wrapper-fault`). They never carry upstream text, absolute paths, internal type names or drift-report content.
- **Vector 6:** CLI arguments, flags and environment. Logging implications:
  - `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every other R8-stripped `CLAUDE*` value must never appear in any log, stderr, diagnostic, snapshot, `events.ndjson` or fixture.
  - An anyhow chain printed to human-mode stderr must never still contain a serde_json or serde_path_to_error source, because their `Display` quotes input values. Map it to a fixed message before it joins the chain, and write the full error only to `diagnostics/` when an instance is resolved.
- **Vector 7:** Filesystem state under `~/.viola/`. Logging implications:
  - `events.ndjson` and `instances/<name>/diagnostics/` files are 0600 inside 0700 directories. On Windows they are covered by the home DACL read rule (user, SYSTEM and Administrators only).
  - `events.ndjson` is never rotated or truncated today, and its retention is an open arch item. Any rotation scheme must keep 0600/0700 permissions and keep `Last-Event-ID` offsets valid.
  - Unbounded growth, which can exhaust disk, is an accepted availability risk.
- **Vector 8:** Child process spawning (`claude agents --json`, the `--version` probe). Logging implication: output rows are untrusted, so human-mode `list` output (every field, including unwrapped session names) and human-mode `wait`/`last` output must escape C0/C1 controls (keeping `\n` and `\t`) before printing to a terminal.
- **Vector 9:** Supply chain. Logging implication: if obs selects tracing-subscriber, the version must be `>=0.3.20` (RUSTSEC-2025-0055, terminal escape injection).

### Anti-Patterns Rejected
The plan's `## Security Anti-Patterns (NEVER do these)` section is its rejection list. The items below are the ones with a telemetry, logging, diagnostics or error-output bearing. Bans limited to IPC ACLs, input parsing, CSP/CORS wiring, supply-chain CI and PTY spawning are not included.

- **Logging CLAUDE session credentials:** writing `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any R8-stripped `CLAUDE*` value into any log, stderr, `events.ndjson`, snapshot, `diagnostics/` or fixture — rejected because these are inherited credentials held only in the wrapper's memory (consolidated NEVER-log floor).
- **Printing or logging the GUI token, the `Cookie` header, or the query string of a GUI request URI** — rejected because `/?t=<token>` carries the credential. The only exceptions are the one-time `viola ui` launch line on stderr and the 0600 `ui/<port>.url` file. No other stderr line may carry the token.
- **Echoing serde_path_to_error drift reports or anyhow chains into Problem Details `detail`, MCP `structuredContent` or channel `error.data`** — rejected because they leak upstream content and internal paths.
- **Printing an anyhow chain to stderr that still contains a serde_json or serde_path_to_error source** — rejected because its `Display` quotes input values such as tool `input`.
- **Writing to stderr from `viola hook`, or exiting non-zero from it (including on security-check failures)** — rejected because of the Hook Contract: hooks must fail open to the human. Diagnostics go to `diagnostics/` only.
- **Printing `wait`/`last` human-mode output, or `list` rows from `claude agents --json`, to a terminal without escaping C0/C1 controls** — rejected because of terminal escape injection (the same class as RUSTSEC-2025-0055).
- **Using tracing-subscriber `<0.3.20`** — rejected because of RUSTSEC-2025-0055.
- **Creating `diagnostics/` files readable by other users** — rejected because diagnostics hold user content, tool `input` and drift reports. They must be 0600, or protected by the Windows home DACL.
- **Exposing absolute paths in any external error** (the only exception is `/api/info`'s cookie-gated `viola_home`) — rejected because paths carry the OS username, the only PII in scope.
- **Putting user content, tool `input`, the `statusline_command` string or drift reports (paths and values) into stderr log or error lines, CLI `--json` error output, MCP `isError`, Problem Details or channel `error.data`** — rejected because these may go only to `instances/<name>/diagnostics/`, apart from the by-design contract payloads.
- **Internal type names in external error output** — rejected. `thiserror` `Display` impls on `PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `UiError` and `CoreError` use fixed messages and leave out any fields holding paths or payloads.
- **Adding a listener (TCP port, WebSocket, `http` hooks, MCP HTTP transport) without a Security Decisions Log entry mapping it to the plan's IPC/GUI controls** — rejected because loopback TCP is reachable by every local user. This applies to any metrics or telemetry endpoint too.
- **Binding anything to `0.0.0.0` or `::`** — rejected. Plain HTTP is acceptable only on loopback.
- **Making an access decision based on the client address or `X-Forwarded-For` / `Forwarded`** — rejected because there is no proxy. Access rests on the Host allowlist and the cookie.
- **Committing `fixtures/claude/*` recorded from real (non-synthetic) prompts, or without first checking them for home paths and usernames** — rejected because fixtures are committed to git.
- **Committing `.env` files, local `--home` test directories or signing material** — rejected because recorded state must never land in git.
- **Letting `config.json`, a `VIOLA_*` environment variable or a CLI flag switch off any security control** — rejected because environment variables are not a configuration channel, and a same-session agent can influence `VIOLA_DIR`. This also rules out a debug or verbose switch that disables redaction.
- **Letting a security refusal block the human** — rejected. Refusals go only to automation (`send`/`answer`), and hooks fail open.
- **External error reporter sending data before the Logging bans are applied** — rejected. v1 has no outbound network calls, so any future reporter must apply the NEVER-log floor before transmission.

### Data Classifications
The plan does not assign Critical/High/Medium/Low labels, except "low sensitivity" for operational metadata. Where sensitivity is marked "(sensitivity inferred)", the level was derived from the plan's description. The obs handling below is stated explicitly by the plan unless marked "(handling inferred from sensitivity)".

- **GUI per-launch token, launch URL, `ui/<port>.url` contents, `viola_<port>` cookie value / `Cookie` header** (Critical, sensitivity inferred) — obs handling: never-log. Also strip the query string from any HTTP request path that is logged. Sole exceptions: the one-time stderr launch line and the 0600 `.url` file. Appears in: `viola-ui` process memory, `<viola home>/ui/<port>.url`, `GET /?t=<token>` exchange, browser cookie.
- **Inherited credentials: `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET`, other R8-stripped `CLAUDE*` variables** (Critical, sensitivity inferred; transient, held in process memory only) — obs handling: never-log anywhere, including `diagnostics/`, snapshots, `events.ndjson` and fixtures. Appears in: the `viola run` wrapper process environment (stripped from the child).
- **v1.x signing credentials (Azure Artifact Signing, zipsign private key)** (Critical, sensitivity inferred) — obs handling: never-log; held only as CI secrets in the v1.x release workflow. Appears in: the v1.x release workflow (does not exist in v1).
- **User content: prompt text and assistant output** (`prompt-submitted.data.text`, `turn-ended.data.last_assistant_message`) (High, sensitivity inferred; persistent, never rotated) — obs handling: never-log in log or error lines (stderr, CLI `--json` errors, MCP `isError`, Problem Details, channel `error.data`). Permitted only in the by-design contract payloads and in `instances/<name>/diagnostics/` (0600). Appears in: `instances/<name>/events.ndjson`, SSE `/api/events`, MCP/CLI `wait`/`last`, channel `hook.event`, wrapper memory.
- **User content: tool-call arguments (`permission.input`, including shell commands and file bodies), plan text, dialog questions and answers** (High, sensitivity inferred; may incidentally contain secrets or personal data) — obs handling: same as prompt and assistant content (never-log outside contract payloads; `diagnostics/` only). Appears in: `events.ndjson`, SSE, channel `hook.dialog`, `answer` free text.
- **serde_path_to_error drift reports (paths and values; can quote tool `input`)** (High, sensitivity inferred) — obs handling: `diagnostics/` only. Never in stderr, `--json`, MCP, Problem Details or `error.data`. Appears in: `viola-agent-claude` hook parser, tolerant parsers.
- **`statusline_command` (user-written shell string; the only shell-out)** (High for integrity, sensitivity inferred) — obs handling: never in log or error lines; `diagnostics/` only. Appears in: `snapshot.json`, `instances/<name>/settings.json`, `hook statusline`.
- **Config values: `config.json` budget thresholds and GUI port; snapshot `endpoint` / `pinned_bin`; `ledger/stamps.json` verified CLI versions; `hooks.json` / `.mcp.json` exec paths** (Low confidentiality, high integrity; sensitivity inferred) — obs handling: OK to log structured, except that values containing absolute paths follow the operational-metadata rule below (handling inferred from sensitivity). Appears in: `~/.viola/` state files.
- **Operational metadata: absolute paths carrying the OS username (the only PII in scope), internal type names** (Medium, sensitivity inferred) — obs handling: scrub-required in external output (CLI `--json` errors, MCP `isError`, Problem Details, channel `error.data`). Paths are permitted in `diagnostics/`. The only intended external path disclosure is the cookie-gated `/api/info` `viola_home`. Appears in: error paths across all crates, `/api/info`.
- **Operational metadata: `pid`, `child_pid`, `agent_session_id`, `started_at`, `budget.json` usage percentages and `resets_at`, `verified_cli_versions`, `bind`** (Low; "low sensitivity" per the plan) — obs handling: OK to log structured, and explicitly permitted in `diagnostics/`. Appears in: snapshots, `budget.json`, `/api/info`, `/api/sessions`.
- **Unwrapped session identifiers / rows from `claude agents --json`** (Low but untrusted, sensitivity inferred) — obs handling: OK to log structured, but C0/C1 controls must be escaped in any human-mode terminal output. Appears in: CLI `list`, the MCP `list` tool, `/api/sessions`.
- **Protocol codes and refusal details** (`-32600`, `-32602`, `release-from-driver`, `not-delivered` / `control-character`, `instance-unreachable`, `wrapper-fault`, Problem Details URNs) (Low, sensitivity inferred) — obs handling: OK to log structured (handling inferred from sensitivity). Appears in: channel, MCP, CLI, `viola-ui`.
- **Recorded fixtures (`fixtures/claude/<cli-version>/`)** (Medium, sensitivity inferred) — obs handling: scrub-required. Probe prompts must be synthetic, and home paths, usernames and tool `input` must be reviewed before committing. Appears in: the repository, recorded by `viola verify`.
- **Payment, health or multi-user PII:** none. There is no database, no accounts and no payment, identity or health SDK.

## 3. Design System Excerpt

### Surfaces
- **web-spa: `viola ui`** (desktop-browser SPA, loopback only at `http://127.0.0.1:47319`). A single page with no routes, built from Lit 3.3.3 light-DOM `viola-*` elements that are vendored and embedded via `include_bytes!`, with no JS build step. It runs on Windows 10/11 (Edge/Chrome) first, plus macOS and Linux (Chromium, Firefox, Safari 17.5+). Headless-browser checks run in CI on ubuntu. It shows session strips, transfer markers and an SSE-fed event tape. Telemetry hint: browser OTel SDK + web-vitals is the candidate, but it must be vendored and same-origin. The CSP is `default-src 'none'; script-src 'self'; style-src 'self'; font-src 'none'; require-trusted-types-for 'script'`, with no CDN assets, inline script or `innerHTML`. `/ready` (`ok` / `unavailable` / `error`) and `/health` are probe routes for tests and obs, not shown on the page.
- **cli: `viola` binary** (cli; Rust stable, clap 4.6.7). Used by a human on a TTY and by LLM drivers via `--json` / MCP. Results go to stdout; context lines, refusals, `hint:` lines, errors and the `viola ui` launch line go to stderr. `--json` prints one JSON document on stdout with a typed exit code. `viola run` prints nothing while the child `claude` TUI runs. `viola hook` never writes stderr and always exits 0. `viola mcp` writes only MCP frames to stdout. Stack traces never print, and full detail goes only to `instances/<name>/diagnostics/`. Telemetry hint: stdout/stderr only, no frontend.

### Loading / Error / Empty State Patterns
- **Initial read (before `/api/sessions` answers, typically under 100ms)**: page-replace text. Each rack shows `sessions: no reading yet` and the ATIS fields show `unknown`. Skeletons and spinners are banned. Telemetry hook: a span from page load to the first `/api/sessions` response (TTI proxy).
- **Failed reading (`unknown`)**: inline word in the field cell. Examples: STATUS `unknown` when `claude agents --json` is missing or fails; CLI version `unknown`; budget `5H unknown  7D unknown  read unknown`. `n/a` means absent by contract on unwrapped rows, not a failure. Telemetry hook: a counter for `unknown` per field.
- **Empty rack**: inline `no wrapped sessions — start one with  viola run <name> -- claude`. The CLI prints `no wrapped sessions  start one: viola run <name> -- claude`. Telemetry hook: a counter for empty-state occurrences.
- **Empty tape**: inline `TAPE live since 19:40:02Z — no events yet`. The SSE stream starts at each file's current end. Telemetry hook: a span from SSE open to the first event.
- **Tape connection (SSE `EventSource`)**: an inline ATIS cell showing `TAPE connecting`, then `TAPE live since <time>`, or `TAPE stopped · viola ui not answering` (boxed). Telemetry hook: a span around connecting → open, and a counter for transitions to closed with a cause attribute.
- **Readback (send lifecycle)**: an inline 16px box plus a word at the far right of tape send lines and transfer markers. Changes are instant. The four states are:
  - `open`, from CL-1 `send-issued`.
  - `read back`, when the matching `prompt-submitted` with origin `driver` arrives, or `session-start` cause `clear` for `/clear`.
  - `unable · <reason> · <detail>`, from CL-1 `send-refused`.
  - `unconfirmable`, from an `ok` send with `confirmed:false`.

  The CLI mirrors these as `[  ]` / `[RB]` / `[/ ]`. Until CL-1 lands, the page shows only `read` boxes and prints `unknown` in the marker's last-send cell. Telemetry hook: a send → readback latency span, a counter per terminal state, and the refusal reason and detail as attributes.
- **Stale liveness**: an instant, inline lamp-off dimming of the whole strip; the strip keeps its slot. The CLI row is SGR-dimmed with `stale` in the LIVE column. Telemetry hook: a counter for live → stale transitions.
- **Budget expired / paused**: inline ATIS words. `5H 62 % expired` appears once `resets_at` has passed. A boxed `budget-paused` replaces `gate open`, and the reading age `read 4m ago` is always shown. Telemetry hook: a counter for gate transitions and a gauge for reading age.
- **Skipped counts (unknown kinds / unknown fields / torn lines)**: inline ATIS `skipped 0 · 0 · 0`, boxed when nonzero and never auto-cleared. The tape shows `skipped  1 unknown kind` in place. Telemetry hook: counters for unknown kinds, unknown fields and torn ndjson lines.
- **Tape line cap (2000 DOM lines)**: an inline notice `older lines trimmed from view: N — the full tape is events.ndjson`. Telemetry hook: a counter for trim events.
- **Tape follow rule**: a static inline line `N new lines below` when the reader is not at the bottom. Telemetry hook: none specified.
- **CLI `viola wait`**: a static stderr line `waiting: <name>` (TTY only), then one result line: `turn-ended …`, `question … dialog N …` or `timed out  <name>  30000 ms`. There is no spinner. Telemetry hook: a span around the wait with the outcome as an attribute.
- **CLI `viola verify`**: step-counter lines `[03/14] S3 … pass` / `fail`, then a summary `stamped 2.1.280  14 pass  0 fail`. Telemetry hook: a span per step and pass/fail counters.

### User-Facing Error Surfaces
- **401 `unauthorized` access strip** (in-page, full-width strip in the rack position). Appears for: a missing or invalid GUI session. It prints `UNAUTHORIZED  this page has no session for 127.0.0.1:47319`, then `open the launch line printed by "viola ui" at start, or restart viola ui`. Recovery affordance: reopen the launch line or restart; there is no retry control or input field. It never shows the token, launch URL, `?t=` query, `ui/<port>.url` path or cookie, and never reads `location.search`. Feedback widget candidate: no, because v1 has no page controls.
- **503 `state-unreadable` rack strip** (in-page). Appears for: viola home could not be read. It prints `unable · state-unreadable  viola home could not be read`. Recovery affordance: none. Feedback widget candidate: no.
- **404 `not-found` / 405 `method-not-allowed` rack strips** (in-page, defensive only; the page makes GET requests to its own routes only). They print `unable · not-found` / `unable · method-not-allowed`. Recovery affordance: none. Feedback widget candidate: no.
- **403 `host-not-allowed`** (dedicated error page: the browser shows the raw Problem JSON). Appears for: a failed Host check, so the page never loads. Recovery affordance: none. Feedback widget candidate: no.
- **`Problem::cross-origin-forbidden` (403)**: reserved for the v1.x brake controls and not used in v1.
- **Tape stopped** (in-page: a boxed ATIS cell `TAPE stopped · viola ui not answering` plus a strip at the top of the tape). Appears for: the SSE connection closing or the server not answering. Strips keep their last values and the ATIS age keeps counting. It is announced through a polite `aria-live` region. Recovery affordance: none. Feedback widget candidate: no.
- **Readback refusal** (in-page, inline on the tape send line and the transfer-marker box). Appears for: a typed refusal shown as `unable · <reason> · <detail>`, announced via `aria-live`.
  - Reasons: `human-typing`, `budget-paused`, `unverified-cli`, `not-delivered`, `unknown` (its detail is opaque text).
  - Details: `manual-pause`, `five-hour`, `seven-day`, `input-not-ready`, `no-prompt-submitted`, `turn-running`, `unknown-dialog`, `control-character`.

  Recovery affordance: none. Feedback widget candidate: no.
- **CLI refusals** (stderr line `unable  <name>  <reason>  <detail>` followed by one `hint:` line keyed by the reason). Exit codes:
  - 10 `human-typing`
  - 11 `budget-paused`
  - 12 `unverified-cli`
  - 13 `not-delivered`
  - 14 `unknown` (no hint)
  - 21 `instance-unreachable`
  - 1 `unable: <name> is already live`, followed by `hint: viola list`

  Under `--json` a refusal is `{"v":1,"refusal":…,"detail":…}` on stdout, with no hint. Hints never quote sent or upstream text. Recovery affordance: the hint names the next command (`viola release` / `wait` / `verify`). Feedback widget candidate: no.
- **CLI faults** (stderr line). Exit 1 prints `error: internal error` (fixed message, no chain, no paths, no hint). Exit 20 prints `error: wrapper fault  <code>`, or `{"v":1,"error":"wrapper-fault","detail":{…}}` under `--json`. There are no stack traces, and detail goes only to `instances/<name>/diagnostics/`. Errors never contain upstream text, paths, pids or anyhow chains with serde sources. Recovery affordance: none. Feedback widget candidate: no.
- **CLI Git Bash path warning** (stderr line `warning: argument looks like a Git Bash rewritten path`). Recovery affordance: none. Feedback widget candidate: no.
- **Redaction rules for every error surface**: never display or log the GUI token, launch URL, `?t=` query, `.url` path or cookie values. The one exception is the single unstyled `viola ui` stderr launch line. Never render `viola_home` from `/api/info`, because it holds the OS username, the only PII in scope.

## 4. Layout Templates Excerpt

### Layout Types per Surface
- **web-spa:** bay-steady-state (≥1024), bay-narrow (760–1023), bay-first-reading-empty, bay-degraded, tape-line-expanded. All five are states of one route (`/`), not separate routes. Component regions in fixed vertical order: header (`<viola-atis>`), WRAPPED rack (`<viola-session-row>` strips plus `<viola-transfer>` markers), UNWRAPPED · READ-ONLY rack, TAPE (`<viola-event-feed>`, the only bounded scroll region, `<details>` tape lines, capped at 2000 DOM lines). Cross-cutting elements: `<viola-readback>` signature (on tape send lines and outbound markers), state strips (first-reading / empty / 503 `state-unreadable` / `TAPE stopped` / 401 access strip), tape-end zone (`#tape-end` anchor plus `N new lines below`). Custom elements total: `viola-atis`, `viola-session-row`, `viola-transfer`, `viola-readback`, `viola-event-feed`. Lit 3.3.3 light-DOM, no build step. Data sources: `/api/sessions`, `/api/links`, SSE `/api/events`. There are no spinners, skeletons, toasts or "Loading…" indicators. Waiting is shown as words: `unknown`, `open`, `sessions: no reading yet`, `TAPE connecting`, `read Nm ago`.
- **cli:** viola-list (human TTY / piped-NO_COLOR / `--json`), viola-send (readback mirror), viola-wait / viola-last, wheel-handoff-dialog verbs (pause, release, link, unlink, answer), viola-verify (step counter), viola-help, viola-run (passthrough, prints nothing), viola-ui (one launch line on stderr). Shared components: BAY context header (only on `viola list`), strip rows, refusal lines with a `hint:` line (stderr, typed exit codes 10–14, 20 wrapper fault, 21 instance-unreachable), and the exit code as the terminator. There are no spinners or progress bars. Results go to stdout; `waiting:`, the issue line, refusals, hints and errors go to stderr.

### Error Boundary Placement
(No explicit error boundary placement in layouts — Phase 3 will recommend defaults per layout category, typically section-level for dashboards / page-root for forms.)

Related region-scoped degraded-state signals from the layouts. These are data and connection error states, not component-crash boundaries:
- **bay-degraded (web-spa):** the error strip sits in the rack or tape position it affects. `unable · state-unreadable  viola home could not be read` (HTTP 503) replaces rack content in the rack position. `TAPE stopped · viola ui not answering` sits at the top of the tape and is paired with a boxed ATIS header cell. The 401 access strip uses the same rack-position anatomy, and its content is deferred to implementation. Strips keep their last values and the last received tape lines stay.
- **Header counters:** `skipped a · b · c` (unknown kinds · unknown fields · torn lines) is boxed and printed in `color-text-primary` when nonzero, and it is never cleared automatically. An unknown event kind is printed in place in the tape as `skipped  1 unknown kind`.

## 5. Test Plan Excerpt

### Tests Tier
- **Tier:** Comprehensive (recorded upstream as `Comprehensive (2)`)
- **Justification:** "Five drivers beyond the Standard baseline push the tier to Comprehensive:" the creator brief requires multi-platform CI (Founder Direction 4, D3); crash-safe self-healing state needs chaos tests; hook deadlines and SSE keep-alive need performance-budget tests; multi-version CLI fixtures need compat tests; the founder mandates mutation testing from chunk 1. "The one Comprehensive component that does not apply is compliance verification: the security excerpt, Context, says 'No compliance triggers'."

### Test Harness Contract Summary
- **5-command names:** `boot`, `run`, `status`, `cleanup`, `logs`
  - Invoked as `scripts/agent-run.sh <command>` / `scripts/agent-run.ps1 <command>`. These are thin shims over `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]`.
  - Every harness command exits 0 on success, 1 on failure and 2 on usage error. Each prints exactly one JSON document on stdout, starting `{"v":1,"cmd":"<command>","ok":<bool>,…}`.
  - Internal subcommands (not part of the 5-command surface): `supervise`, `gate`.
  - `logs` streams ndjson on stdout:
    - event lines are wrapped as `{"src":"events","instance":<name>,"offset":<byte offset>,"record":<line>}`
    - process-log lines are wrapped as `{"src":"diag","file":<basename>,"record":<line>}`
    - torn lines are emitted as `{"src":…,"torn":true,"offset":n}` and never dropped
    - it is filterable by `--instance`, `--kind`, `--process run|hook|mcp|ui` and `--after <byte offset>`
    - retention is the whole lifetime of the session home
- **Status JSON shape:** (`agent-run status` document, verbatim from tests Section 3)

```json
{
  "v": 1, "cmd": "status", "ok": true,
  "state": "ready | degraded | down",
  "pid": "<viola ui pid from /api/info, or null without --ui>",
  "uptime_ms": "<now - /api/info.started_at, or null>",
  "last_error": "null | <harness code, e.g. ready-503, list-exit-21, sessions-mismatch>",
  "list": "<viola list --json document>",
  "ui": { "ready": "<GET /ready body>", "sessions": "<GET /api/sessions body>" },
  "api_sessions_equal_list": true
}
```

  "`state` is `ready` when every booted instance has `liveness:"live"` and `/ready` is 200. It is `degraded` when any item is `stale`, `/ready` is 503, or `api_sessions_equal_list` is false. It is `down` when `list` exits non-zero. `ok` is true only for `ready`."
  - `status.last_error` is a closed enum: `null`, `ready-503`, `list-exit-<code>`, `sessions-<http status>`, `sessions-mismatch`, `ui-unreachable`.
  - Nested product shapes, verbatim from tests Section 1:
    - `GET /health`: `{"v":1,"status":"ok","name":"viola","version":"0.1.0","ts":<RFC3339 ms Z>}`
    - `GET /ready`: `{"v":1,"status":"ok"|"not-ready", …, "checks":{"viola_home":"ok"|"unavailable"|"error","event_tail":…,"claude_agents":…}}`. It returns 503 when not-ready. If only `claude_agents` fails, it still returns 200.
    - `GET /api/sessions` / `viola list --json`: `{v, generated_at, items:[{name, wrapped, liveness:"live"|"stale", status:"idle"|"busy"|"unknown", wheel?:"human"|"driver", budget_paused?, dialog_pending, cli_version?, cli_verified?}], skipped:{unknown_kinds, unknown_fields, torn_lines}, budget:{five_hour:{used_percentage,resets_at}, seven_day:{…}, read_at, paused}|"unknown"}`
- **Log format JSON schema:** tests Section 3 has no fenced JSON schema block. The contract is the bulleted spec below, copied verbatim:

```markdown
- **Format:** JSON-per-line. There are two streams:
  - **Event stream:** product events in `instances/<name>/events.ndjson`, exactly the arch Event line `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`.
  - **Process logs:** emitted by tracing-subscriber 0.3.23 `fmt().json().flatten_event(true).with_current_span(false)`, writing to `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>}.ndjson`. Files are 0600, dirs 0700, with one `write` per line.
- **Required fields:**
  - For process logs: `timestamp` (RFC 3339 UTC with ms and `Z`), `level` (`DEBUG|INFO|WARN|ERROR`), `target`, and `fields.message` (or the flattened `message`).
  - `event`, a closed kebab-case enum. A new value needs a Decisions Log entry, like the §3 closed enums. Each value fixes what its `corr` holds:
    - `channel-request`, `channel-response`: `corr` = the JSON-RPC request `id`
    - `dialog-raised`, `dialog-answered`: `corr` = `dialog_id`
    - `hook-invoked`, `hook-decision`: `corr` = `dialog_id` for dialog hooks, otherwise null
    - `send-issued`, `send-confirmed`, `send-refused`: `corr` = the send `cursor`
    - `process-start`, `process-exit`, `http-request`, `panic`: `corr` = null
  - `process` (`run|hook|mcp|ui`) and `instance` (a `ViolaName` or null).
  - `corr` is copied unchanged as a JSON number or string. It is never renamed (for example to `correlation_id`).

  obs-plan may add fields but must not rename or remove these. The harness greps on them.
- **Constraints:**
  - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
  - No line may contain the GUI token, a `Cookie` header, `?t=`, or any stripped `CLAUDE*` value. This is enforced by the secret-scan test in §6.
- **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.
```

  Related contract points from tests Section 3:
  - No standalone product pid file exists. Wrapper `pid`, `started_at` and `child_pid` are in `<home>/instances/<name>/snapshot.json`.
  - Liveness is in `<home>/instances/<name>/heartbeat`, touched every 1 s. More than 5 s old means gone, unless pid + start time show `stale`.
  - The UI pid comes from `GET /api/info` (`pid`, `bind`) and `<home>/ui/<port>.url`.
  - The secret-scan test also requires `diagnostics/` to be 0600.

### Critical Paths (must-trace)
- **Path 1: `run` start sequence:** "Collision check, pinned copy and plugin folder, version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn." Expected order in `events.ndjson`: `wheel{cause:"start"}` → `budget-gate` → `session-start`. A duplicate `run`, a squatted endpoint or a pinned-exe hash mismatch exits 1.
- **Path 2: confirmed `send` with CL-1 events:** "Driver `send` is accepted, then `send-issued (cursor, from)` is logged, one bracketed paste + Enter is written, and the matching `prompt-submitted{origin:"driver"}` confirms it." If no match arrives in the window, the result is `not-delivered`/`no-prompt-submitted` and `send-refused` is logged. A local command yields `unconfirmable`.
- **Path 3: `wait` / `last` event-driven readback (§6 live flow):** "`wait` wakes only on `turn-ended` / `question` / `permission` / `plan` / `session-end`, returns at once if the event is already logged at or after `after`, and times out with `{timed_out:true}`." Killing the wrapper mid-wait gives exit 21 / MCP `instance-unreachable`.
- **Path 4: dialog → `answer` (question / permission / plan):** "The hook raises `hook.dialog`, exactly one dialog is pending per instance, and the driver answers by `dialog_id`." A second concurrent dialog gets a `null` response. An unknown `dialog_id` exits 13 with `unknown-dialog`.
- **Path 5: human takes the wheel, automation is refused, `release` returns it:** "A human editing key in the `run` terminal since the last turn boundary moves the wheel to the human." A driver `send` is refused with `human-typing` (exit 10). The wheel returns only through CLI `viola release`.
- **Path 6: budget governor:** "`viola hook statusline` writes `rate_limits` to `budget.json` and passes the user's statusline stdout through unchanged." Crossing `five_hour` ≥ 90 % or `seven_day` ≥ 85 % logs `budget-gate{paused:true, window}` and refuses `send` with `budget-paused` (exit 11). `release --budget` overrides it.
- **Path 7: unverified CLI version gate:** "On a version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers, and drivers get `unverified-cli`." `answer` exits 12.
- **E1: R5 unwrapped session hooks are a silent no-op.**
- **E2: R8 `CLAUDE*` env strip** (14 inherited variables).
- **E3: `link` / `unlink` transfer markers** (`/api/links`, `<viola-transfer>` readback).
- **E4: SSE `Last-Event-ID` resume.**
- **E5: snapshot corruption → replay recovery.**

### Coverage Triggers Summary
- **IPC channel (Vector 1)** (security-vector-coverage). obs implication: log `channel-request` / `channel-response` keyed by JSON-RPC `id`, and record protocol-fault codes (-32600/-32602/-32603) plus server pid/start-time verification failures (CLI exit 21 / MCP `instance-unreachable`).
- **Paste injection (Vector 2)** (security-vector-coverage). obs implication: telemetry on `send-refused` with detail `control-character`, emitted at both client and wrapper, with the rejected text never logged.
- **Loopback GUI (Vector 3)** (security-vector-coverage). obs implication: `http-request` log lines carrying status (403 `host-not-allowed` / 401 / 405 / 303) that never include the token, `Cookie` or `?t=`.
- **Hook stdin (Vector 4)** (security-vector-coverage). obs implication: fail-open hook paths (oversize stdin, malformed JSON, channel failure, panic, missing `VIOLA_NAME`) logged to `hook-<name>.ndjson` only, with exit 0 and empty stderr, and a panic as a single `event:"panic"` line.
- **MCP (Vector 5)** (security-vector-coverage). obs implication: rejected tool calls logged with codes only, never with payload values.
- **CLI and env (Vector 6)** (security-vector-coverage). obs implication: strict-modes and name-validation refusal events. No env, flag or config may disable a control, including logging controls.
- **Filesystem (Vector 7)** (security-vector-coverage). obs implication: telemetry for strict-modes refusal, pinned-exe hash mismatch, `skipped.torn_lines` counts and ignored symlinks.
- **Child spawn (Vector 8)** (security-vector-coverage). obs implication: `process-start` / `process-exit` for PTY children, a vt100-panic → `input-not-ready` event, and `claude agents --json` parse failures surfaced as `unknown`.
- **Supply chain (Vector 9)** (CI gate: cargo deny, tokio-free sync crates, zizmor, SHA-pinned Actions). obs implication: obs dependencies must satisfy the tokio ban on sync crates and the C-crate / cargo-deny bans.
- **Parser surfaces** (property-test / fuzz). Covers `validate_paste_text`, `Last-Event-ID`, channel framing at `MAX_FRAME`, hook stdin incl. statusline, `claude agents --json`, vt100 feed under `catch_unwind`, and `prompt-submitted` normalisation. obs implication: parse-failure / skipped-record counters per parser.
- **Crash-safe state** (chaos). Cases: kill mid-write / torn ndjson line, corrupt or unsupported-`v` snapshot → replay, child exit without EOF (ConPTY), endpoint vanishing during `wait`, stale-heartbeat vs live-pid. obs implication: fault telemetry for torn-line healing, snapshot replay recovery, exit detected on the process handle, and heartbeat staleness transitions.
- **Hook deadlines / SSE keep-alive / heartbeat** (perf-budget). obs implication: latency histograms for hook invocations (spine hooks, SessionEnd, pre-tool-use), the SSE keep-alive interval (15 s) and the heartbeat staleness flip (>5 s).
- **Cross-surface coordination** (CLI `list --json` == `/api/sessions`, identical CLI/MCP payloads, CL-1 `send-issued`/`send-refused` readback). obs implication: send-lifecycle events correlated by `corr` = send `cursor` across CLI, MCP, wrapper and web.
- **Multi-OS** (multi-platform: windows-2025, macos-latest, ubuntu-latest; browser suite ubuntu only). obs implication: logging and instrumentation must work on named pipe vs Unix socket, ConPTY vs openpty, and Windows DACL vs Unix modes.
- **Multi-version CLI** (compat). obs implication: counters for unknown event kinds / fields (`skipped`), the `RefusalReason` `unknown` fallback (exit 14), and higher-`v` rejections (-32602).
- **Fake agent vs recorded `viola verify` fixtures** (contract-test-against-sandbox). obs implication: hook-sequence and decision-body logs stable enough to snapshot-compare, and fixtures free of real prompts, paths and usernames.
- **Error sanitization / secret logging** (security-vector-coverage). obs implication: no absolute paths, serde values, anyhow chains or tool `input` in CLI `--json`, MCP `isError`, Problem Details or channel `error.data`. `-32603` must be exactly `"internal error"` / `data:null`. All logs and `diagnostics/` must be free of the token, `Cookie`, `?t=` and `CLAUDE*` values, and `diagnostics/` must be 0600.
- **Mutation testing** (discipline; cargo-mutants scoped to the chunk diff; a surviving mutant is red). obs implication: obs instrumentation code falls under the mutation gate like all product code.
- **No screen parsing / no blanket approval / no flakes** (discipline). obs implication: E2E verdicts rely on hook events, `events.ndjson`, channel payloads and DOM attributes, so these must be emitted deterministically. Waits key on logged events and byte offsets, never sleeps.

### Quality Gates Summary
- **Zero-flakiness statement:** "flaky tests are NOT tolerated. The nextest `retries = 0`, and Playwright `retries: 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` and `test.skip` are not allowed as a parking place. Agent-driven dev cannot distinguish a flake from a real bug; retry policies mask actual failures."
- **Coverage thresholds:**
  - Values: ≥ 85% line, ≥ 80% branch (enforced as region coverage `--fail-under-regions 80`), ≥ 95% function.
  - Scope: enforced per OS job via `cargo llvm-cov nextest`, rolled up across all workspace crates.
  - Exclusions: `viola-fake-agent`, `crates/viola-e2e`, `tests/support`, `fuzz/`.
  - Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates.
- **Mutation gate:** `missed == 0` and `timeout == 0` in `mutants.out/outcomes.json` for the chunk diff.
- **Performance gates:**
  - Method: hyperfine `max` sample, `--warmup 3 --runs 30`, no reruns.
  - `viola hook session-end` must have `max < 1.0 s`.
  - Spine hooks (`session-start`, `user-prompt-submit`, `stop`) and `pre-tool-use` on an unverified CLI must have `max <` the arch spine deadline, provisionally 1.0 s.
  - SSE keep-alive is emitted at `advance(15 s)` and not before.
  - Heartbeat: 4.9 s → live, 5.1 s → gone/stale.
- **Build failure conditions:**
  - a test failure at any level on any OS
  - coverage below threshold (per OS)
  - a perf `max` regression
  - any missed or timed-out mutant
  - a lint, clippy, cargo-deny, zizmor or cargo-modules failure

## 6. Creator Brief Excerpt

_Source: `.andromeda/input.md` (which folds in `refs/viola-brief.md` and `refs/viola-prior-art.md` at full fidelity; refs/ read directly as well, no additional files). Quotes are verbatim._

### Must-Work Scenarios

- First live test (brief §6): "the overseer sends `/andromeda-new-session` to the builder, waits for `Stop`, and reads the dashboard. When that passes, the foundation stands."
- Wheel take-over (brief §6, PASSED on Pulse): "the founder typing into the window took the wheel, `send` was refused, `release` returned it."
- R2 One wheel (brief §3.3): "A submitted prompt the bridge did not send means the human took the wheel, and automation pauses by itself — no button needed."
- Dialog answering (brief §3.3 R7): "`PreToolUse` answers `AskUserQuestion` and approves a plan, `PermissionRequest` answers a permission prompt (a suggestion included) or sends a plan back with feedback, and the answered dialog never renders".
- Waking without polling (brief §4.1 S5): "A background `wait` that exits on the next `Stop` completed when the driven turn ended, and the driving session's harness woke on its exit."
- Identity stripping (brief §3.3 R8 / §4.1 S6): "Started from inside a Claude session — the normal case once an overseer launches a builder — the child would inherit that session's id, its Remote Control bridge and its messaging socket. The wrapper removes them".
- Shell path rewrite (brief §6): "a leading-slash argument is taken for a path: `/andromeda-new-session` arrived as `C:/Program Files/Git/andromeda-new-session` (42 bytes sent for 22) … The product must not depend on that: take the text from stdin or a file, and warn when an argument carries a rewritten-path prefix."
- Windows exit detection (brief §4.1): "ConPTY did not close the output stream when the child exited, so exit is detected on the process, never on EOF."
- Budget governor (brief O8): "The statusline's input JSON carries the plan limits as official fields, `rate_limits.five_hour.used_percentage` and `rate_limits.seven_day.used_percentage` … record the figures, then hand over to the user's script unchanged."
- GUI (brief D5): "a minimal GUI: the active sessions and the links between them."

### Rigor Hints

- R4 (brief §3.3): "The bridge carries, logs (ndjson on disk, so it survives restarts on either side) and holds the wheel. It never decides what to answer."
- Brief §7: "Security is a first-class concern: the bridge types into sessions that can run commands. Localhost only by default, authentication for anything remote, and the event log as the audit trail."
- Brief §3.4: "CI on all three OSes against a FAKE AGENT — a small program that behaves like Claude Code (prints, waits for input, calls the hook commands). Tests spend no tokens and do not flake; the real CLI runs only in local live tests."
- Brief §3.4: "an agent can verify it through a headless browser, which is how the founder's projects verify UI by default."
- Brief §4: "The docs lag the installed build. … Design against the installed CLI, measured."
- Brief §4.1: "Timing: the screen lags the hooks. … Hooks are the source of truth; the screen is an eventually consistent view."
- Brief O6: "Limits — the largest paste the input box takes whole; hook latency per tool call (the spike's hooks start a Python interpreter each time; the product's hooks call the native binary)."
- Brief §7 (policy): "Advertised usage limits for Pro and Max plans assume ordinary, individual usage of Claude Code and the Agent SDK — so a budget governor belongs in the first version, not a later one."
- Brief Appendix A: "A step that moves the rate of caught errors is reverted."

### Obs Anti-Patterns (creator's explicit asks)

- Brief §4 (transcripts): "the entry format *\"is internal to Claude Code and changes between versions, so scripts that parse these files directly can break on any release\"* — usable for reading on demand, never as the bridge's contract."
- Brief §3.2: "Events, not screen-scraping. … Nobody parses the rendered screen for content."
- Brief §7 (policy): "developers may not collect, store, or intermediate Claude.ai credentials or session tokens"; input.md Extracted: "never touches credentials".
- Brief §2 D2: "It drives the official `claude` CLI under the user's own login. No API key."
- Prior art §4: "Overclaiming. The GUI shows only what the hooks and the wrapper observed."
- Prior art §4: "Fragile shared state. … viola's on-disk state parses defensively and heals itself from day one."
- Brief §3.4: "No central daemon: each wrapper owns its channel, state lives on disk".

### Founder Directions (overseer, founder-delegated — received during this obs run, 2026-09-24; BINDING for Phase 1 onward)

_Delivered by the overseer session on the founder's behalf mid-run. Recorded verbatim; they rank above derived defaults._

1. "Align to tests: adopt test-plan §3 log fields AND its fixed event-name list verbatim; tests owns them."
2. "Structured JSON logs from every process (run/hook/mcp/ui) under diagnostics/; hook never writes stderr; the harness `logs` command merges them with events.ndjson."
3. "Normal mode, not inverted: viola is not a telemetry product."
4. "From the cross-plan audit: release-from-driver must log as its own event, not a generic wrapper fault -32602; each cause behind exit 21 (dead instance, strict-mode fail, server-verify fail) and exit 1 (already live, squatted name, SHA-256 mismatch, .cmd/.bat child) logs a distinct detail code, never a path or pid."
5. "send-issued / send-refused (CL-1) are logged events."

