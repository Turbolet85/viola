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
