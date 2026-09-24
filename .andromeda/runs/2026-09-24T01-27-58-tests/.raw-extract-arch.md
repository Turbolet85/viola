## 1. Architecture Excerpt

### Stack (testable surfaces)
- **Rust stable (host 1.95; `rust-version = "1.89"`, edition 2024)**: one native binary, `viola` / `viola.exe`, that serves every surface.
- **std threads + Tokio 1.53.1 only in `mcp`/`ui`**: sync hot path (`run`, `hook`, `send`, `wait`). Enforced by a `cargo deny` tokio ban plus a `cargo check` of the sync crates without tokio.
- **axum 0.8.9 (`sse`) + tower-http 0.7.1**: GUI GET routes and SSE on 127.0.0.1, compressed except `text/event-stream`.
- **clap 4.6.7**: CLI surface: `run · send · wait · last · list · answer · hook · mcp · ui · verify · pause · release · link · unlink · plugin install`.
- **portable-pty `=0.8.1` behind the `pty` seam (+ windows-sys 0.61.2 `TerminateProcess` fallback)**: hosts `claude` or the fake agent in ConPTY or openpty. The seam is spawn · read · write · resize · wait · kill. Exit is detected on the process handle, never on EOF.
- **vt100 0.16.2**: screen model for the pre-send readiness gate and modal detection. Never reads content.
- **interprocess 2.4.4 `local_socket`**: wrapper channel, one endpoint per `viola run` (named pipe on Windows, Unix socket elsewhere).
- **ndjson logs + atomic JSON snapshots (atomic-write-file 0.3.1, std `File::lock` on `.lock` siblings, one `write` per line)**: crash-safe on-disk state. There is no database.
- **serde / serde_json / serde_path_to_error**: frames, events and snapshots. External payloads parse tolerantly with drift reports.
- **nutype 0.8.0**: `ViolaName` and `Percent` (0–100).
- **chrono 0.4.45**: RFC 3339 UTC in milliseconds with `Z`. A malformed external `resets_at` becomes `"unknown"`.
- **thiserror 2.0.20 per crate / anyhow 1.0.104 in the bin only**: typed errors mapped to channel refusals, CLI exit codes and MCP `isError`.
- **notify 8.2.0**: tails the logs to feed `ui` SSE.
- **sysinfo 0.39.6 + `claude agents --json`**: liveness enrichment (pid + start time; idle/busy status; unwrapped rows).
- **rmcp 3.4.1 + schemars 1.2.2**: stdio MCP server (spec 2026-07-28), tools `send · wait · last · answer · list`.
- **rustfmt / clippy `-D warnings` / cargo-deny 0.20.2 / cargo-modules 0.27.0**: lint, dependency policy (C-crate and tokio bans) and boundary review.

### Workspace / Modules
- **viola (root bin)**: clap dispatch (anyhow edge), plus the `run` pump, wheel, budget governor and readiness-gate wiring. Layout is `src/cmd/<subcommand>` and `src/run/`.
- **viola-core**: normalised event kinds, `RefusalReason`, `ViolaName`, `Percent`, `v` constants. No viola dependencies.
- **viola-pty**: pty seam over portable-pty. Knows no agent.
- **viola-channel**: JSON-RPC 2.0 over ndjson on local sockets. Sync client/server, a Tokio client behind the `tokio` feature, and the FNV-1a endpoint hash.
- **viola-state**: ndjson logs, atomic snapshots, `File::lock`, torn-line healing, tailing, pid + start-time liveness.
- **viola-agent-claude**: the only crate that knows Claude shapes. Hook parsing, dialog mapping (S3/S7/S8), the R8 `CLAUDE*` strip, npm-shim → `claude.exe` resolution, the version gate, the capability ledger and `verify` probes, screen signatures, statusline and `claude agents --json` parsing, `prompt-submitted` normalisation.
- **viola-mcp**: rmcp stdio server, a thin adapter over viola-channel (Tokio).
- **viola-ui**: axum GET + SSE with the Host allowlist. Pure disk reader (Tokio).

### Standard Contracts
**GUI HTTP**: GET only, other methods return 405. Default bind `127.0.0.1:47319`. The Host header must be `127.0.0.1:<port>` or `localhost:<port>`, else 403.
- **`GET /health`** (HTTP): 200 while serving, `{"v":1,"status":"ok","name":"viola","version":"0.1.0","ts":…}`.
- **`GET /ready`** (HTTP): 200 if viola home is readable, else 503 with the same shape and `"status":"not-ready"` (never Problem Details). `checks:{viola_home, event_tail, claude_agents}`, each `ok`·`unavailable`·`error`. A failing `claude_agents` alone does not make it not-ready.
- **`GET /api/info`** (HTTP): `{v, name, version, proto, pid, started_at, bind, viola_home, verified_cli_versions}`.
- **`GET /api/sessions`** (HTTP):
  - Envelope: `{v, generated_at, items, skipped:{unknown_kinds, unknown_fields, torn_lines}, budget}`. No pagination.
  - `budget`: `{five_hour, seven_day, read_at, paused}` or `"unknown"`. Each window is `{used_percentage, resets_at}`, and any part may be `"unknown"`.
  - Item: `{name, wrapped, liveness(live|stale), status(idle|busy|unknown), wheel?(human|driver), budget_paused?, dialog_pending, cli_version?, cli_verified?}`. Gone instances are omitted.
  - Unwrapped row (`wrapped:false`): `liveness:live`, `dialog_pending:false`, and no wheel, budget_paused or cli_* fields.
- **`GET /api/links`** (HTTP): same envelope, items `{driver, driven, since}`, derived by replaying `link`/`unlink` events.
- **`GET /api/events`** (SSE):
  - Never compressed. Keep-alive every 15 s.
  - `event:` is the kind. `data:` is the ndjson line verbatim.
  - `id:` is `<ViolaName>:<byte offset>` pairs joined by `,`, listing every tailed instance.
  - `Last-Event-ID` resumes each instance from its offset. A missing instance starts from 0. An offset past the file length replays from 0. With no header, the stream starts at each file's end.
- **`/`, `/assets/*`** (HTTP): embedded page and assets.
- **Problem Details** (schema): RFC 9457 `application/problem+json` with a `v` member. URNs `urn:viola:problem:host-not-allowed` (403), `…:method-not-allowed` (405), `…:not-found` (404), `…:state-unreadable` (503, `/api/*` only).
- **Reserved, not served in v1**: `POST /api/sessions/{name}/pause` and `POST /api/sessions/{name}/unlink`.

**Wrapper channel** (JSON-RPC 2.0 over ndjson; integer ids, monotonic per connection). Every `params` carries `v` and `sender` (CARGO_PKG_VERSION), plus `from` (the caller's `VIOLA_NAME`) when set.
- **`send`** (IPC): `{text, from?}`.
  - Confirmed: `{submitted_at, cursor}`.
  - Ledger-listed local command with no measured post-condition: `{confirmed:false, detail:"unconfirmable", cursor}`.
  - `cursor` is the `events.ndjson` end offset at acceptance, before the paste.
- **`wait`** (IPC): `{after?, timeout_ms?}` returns `{event, cursor}` or `{timed_out:true}`.
  - Wakes only on `turn-ended`, `question`, `permission`, `plan`, `session-end`.
  - `after` is a byte offset. Returns the first matching line at or after it, at once if already logged.
  - A pending dialog is returned at once unless `after` is past its line.
  - If the endpoint vanishes, the caller gets `instance-unreachable`.
- **`last`** (IPC): `{}` returns `{last_assistant_message|null, ts|null}` from the newest `turn-ended`.
- **`answer`** (IPC): `{dialog_id, from?, response}` returns `{}`. `response` by kind:
  - question: `{answers:{<question>:<text>}, annotations?}`
  - permission: `{behavior:"allow"|"deny", message?}`
  - plan: `{behavior:"approve"|"revise", message?}`
- **`pause`** (IPC, CLI-only): `{}` returns `{wheel:"human"}`.
- **`release`** (IPC, CLI-only): `{budget?}` returns `{wheel, budget_paused}`.
- **`link` / `unlink`** (IPC, CLI-only): `{driver, driven}`, sent to the driven wrapper, returns `{}`. Linking an existing pair adds no log line.
- **`hook.dialog`** (IPC, from `hook`): `{kind:"question"|"permission"|"plan", data}` returns `{dialog_id, response|null}`.
  - `null` means no decision.
  - Returns `null` at once on an unverified CLI, wheel `human`, or another dialog already pending.
  - Every dialog is logged exactly once.
- **`hook.event`** (IPC notification): hook event with no id.
- **`list`** (caller-side disk read plus `claude agents --json`; not a channel method): `{items, budget, skipped}`, same shapes as `/api/sessions`.
- **Result shapes** (schema):
  - Success: `{"result":{"ok":…}}`.
  - Refusal: `{"result":{"refusal":…,"detail":<string|null>}}`.
  - Protocol fault: `{"error":{code,message,data}}`. Codes -32700, -32600, -32601, -32602 (bad params or unsupported `v`; `data:{supported, wrapper}`), -32603.
- **`RefusalReason`** (schema, kebab-case): `human-typing` (null·`manual-pause`) · `budget-paused` (`five-hour`·`seven-day`) · `unverified-cli` (null) · `not-delivered` (`input-not-ready`·`no-prompt-submitted`·`turn-running`·`unknown-dialog`) · `unknown` (`#[serde(other)]`).
  - Only the first applicable refusal is returned.
  - `send` order: human-typing → budget-paused → turn-running → input-not-ready → no-prompt-submitted.
  - `answer` order: human-typing → unverified-cli → unknown-dialog.

**MCP** (stdio, server `viola`)
- **Tools `send {target, text}` · `wait {target, after?, timeout_ms?}` · `last {target}` · `answer {target, dialog_id, response}` · `list {}`** (MCP tool):
  - `target` is a `ViolaName`. The server adds `from`.
  - Success: `isError:false`, with the `ok` payload as `structuredContent`.
  - Refusal: `isError:true`, `{refusal, detail}`, text `"refused: <reason>"`.
  - Unreachable instance or wrapper fault: `isError:true`, `{"error":"instance-unreachable"|"wrapper-fault","detail"}`.
  - No `release`, `pause`, `link` or `unlink` tools.

**CLI**
- **Verbs** (CLI):
  - `send <target>` (stdin or `--file`)
  - `wait <target> [--after] [--timeout-ms]`
  - `last <target>`
  - `answer <target> <dialog_id>` (response JSON from stdin or `--file`)
  - `pause <target>`
  - `release <target> [--budget]`
  - `link|unlink <driver> <driven>`
  - `run <name> -- <child> <args>`
  - `ui [--port]`, `verify`, `plugin install`
  - Global `--home`. Human text by default, `--json` for agents. A prompt never comes from a leading-slash argument.
- **`--json` output** (schema): `{"v":1,"ok":…}` · `{"v":1,"refusal":…,"detail":…}` · `{"v":1,"error":"instance-unreachable","detail":null}` (exit 21) · `{"v":1,"error":"wrapper-fault","detail":{code,message,data}}` (exit 20).
- **Exit codes** (schema): 0 ok · 1 internal error, or `run` refusing to start (live endpoint or `stale` heartbeat) · 2 usage · 10 human-typing · 11 budget-paused · 12 unverified-cli · 13 not-delivered · 14 unknown · 20 wrapper fault · 21 unreachable.

**Hooks**
- **`viola hook <event>`** (Claude Code exec-form hook):
  - Events: `session-start`, `user-prompt-submit`, `pre-tool-use` (matcher `AskUserQuestion|ExitPlanMode`), `permission-request`, `stop`, `session-end`, `notification`, `post-tool-use`, `post-tool-use-failure`.
  - Always exits 0, even on a clap error, channel failure or panic. Never exit 2. Never writes stderr. Exits 0 at once without `VIOLA_NAME`.
  - Decision bodies: PreToolUse `{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","updatedInput":{…}}}`; PermissionRequest `{"hookSpecificOutput":{"hookEventName":"PermissionRequest","decision":{"behavior":"allow|deny","message":"…"}}}`.
  - No decision means empty stdout. Async hooks (Notification, PostToolUse, PostToolUseFailure) print nothing.
  - Deadlines: spine hooks (SessionStart, UserPromptSubmit, Stop) exit 0 within the spine deadline; SessionEnd has ~1 s.
- **`viola hook statusline`**: writes `rate_limits` to `budget.json`, runs the user's statusline command (from the snapshot) with the same stdin, prints its stdout unchanged (empty on failure) and exits 0.

**On-disk**
- **Event line** (schema, `instances/<name>/events.ndjson`): `{"v":1,"ts","instance","kind","source":"hook|wrapper|cli","data"}`. Never truncated. Kinds and their `data`:
  - `session-start {cause(startup|clear|resume|compact|unknown), agent_session_id}`
  - `prompt-submitted {text, origin(driver|human|harness)}`
  - `turn-ended {last_assistant_message|null}`
  - `question {dialog_id, questions:[{question, options, multi_select}]}`
  - `permission {dialog_id, tool, input}`
  - `plan {dialog_id, plan}`
  - `session-end {}`
  - `activity {tool?}` (log-only)
  - `link` / `unlink {driver, driven}`
  - `wheel {holder, cause(start|human-input|manual-pause|release)}` (log-only)
  - `budget-gate {paused, window?, override_until?}` (log-only)
- **Snapshot envelope** (schema): `{"v":1,"written_at","writer","data"}`. An unsupported `v` or a parse failure triggers a rebuild by replay.
  - Replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`.
  - After a replay, `dialog_pending` is false and liveness is `live` or gone, never `stale`.
- **Instance snapshot** (schema): `{endpoint, pid, started_at, pinned_bin, statusline_command?, cli_version?, cli_verified, wheel, budget_paused, budget_override_until?, pending_dialog?{dialog_id, kind}, links, child_pid, agent_session_id?}`.
- **Other files**:
  - `heartbeat`: touched every 1 s. Older than 5 s means gone, unless pid + start time show the process alive, which makes it `stale`.
  - `budget.json`, `ledger/stamps.json` (each with a `.lock`), `config.json` (tolerant, carries `v`), `settings.json`, `diagnostics/`.
- **Endpoint** (schema): `viola-<12 hex of FNV-1a 64(ViolaName + "\0" + abs home)>`. Windows `\\.\pipe\viola-<h12>`; Unix `$TMPDIR/viola-<h12>.sock` (`/tmp` fallback). Recorded in the snapshot.
- **Versioning**: an integer `v` in every params, event, snapshot and GUI body. A higher `params.v` gets -32602 with `data.supported` and `data.wrapper`.

### Surfaces
"**Product type.** Hybrid local developer tool made of three parts:
  - a native cross-platform CLI binary;
  - a Claude Code plugin (hooks + stdio MCP server) that calls that binary;
  - a minimal view-only local web GUI served by the same binary."

Deferred surfaces:
- Mobile: "The phone view is a later version: the same web page behind authentication".
- The v1.x GUI brake: POST pause/unlink.

### Test Harness Commitment
"- Development Style: agent-driven."

(Established Decisions defer "the test framework, fake-agent harness and `viola verify` probe design (tests)".)

### Project Intent Summary
- **Core functionality:** "Lets one interactive Claude Code session drive another on the user's own subscription. It wraps the unmodified `claude` CLI in a PTY, types at turn boundaries, answers dialogs through hooks, and holds a one-driver wheel the human can take at any moment."
- **Target users:** "The founder runs v1 on their own subscription, with no accounts, no hosting and the GUI on 127.0.0.1." ... "the later public version (individual Claude Code subscribers) must not need a rewrite."
- **Critical paths hint:**
  1. `run` start: "collision check, pinned copy and plugin folder, version gate, exclusive bind of the endpoint ..., first snapshot ... and heartbeat, the start `wheel` and `budget-gate` events, and only then the child spawn."
  2. "Every `send` is confirmed. If no matching `prompt-submitted` arrives within the confirmation window, the send is reported as `not-delivered` with detail `no-prompt-submitted`."
  3. "`wait` blocks until the next driver-relevant event (`turn-ended`, `question`, `permission`, `plan` or `session-end`) and returns that event, so nothing polls".
  4. "`answer` names its target by that id. At most one dialog is pending per instance."
  5. "Any human editing key since the last turn boundary moves the wheel to the human, and `send` is refused with `human-typing`." ... "The wheel returns only through CLI `viola release`."
  6. "Once a configured threshold is crossed (defaults `five_hour` ≥ 90 %, `seven_day` ≥ 85 %), automated `send` is refused with `budget-paused`".
  7. "On an unlisted version viola still types, runs the wheel and emits events, but withholds dialog answers, and drivers get `unverified-cli`."

### CI/CD Platform
- **Platform:** GitHub Actions
- **Pipeline note:** One `ci.yml` on push and PR. Native matrix `[windows-2025, macos-latest, ubuntu-latest]`. Per OS: fmt, clippy `-D warnings`, `cargo check` of the sync crates without tokio, `cargo deny` (ubuntu only), the workspace tests against the fake agent replaying `fixtures/claude/*`, and a release build. The real `claude` and `viola verify` run only locally. No deploy stage in v1.

### Test-Relevant Conventions
- **Fake-agent substitution:** `viola run <name> -- <fake agent> <args>`. The child is spawned as given, and shim resolution applies only to the `claude` npm shim. The fake agent answers `--version` like the real CLI.
- **Per-test isolation:** each test uses its own viola home (global `--home`), so tests share no endpoint, log or `budget.json`. The home's `ledger/stamps.json` can stamp the fake agent's version. The child inherits the home via `VIOLA_DIR` (home = its grandparent).
- **Fixture location:** `fixtures/claude/<cli-version>/` holds hook payloads recorded by `viola verify` (local, live, Haiku) and replayed in CI.
- **Live vs CI split:** the real CLI is used only in local live tests and `viola verify`. CI is fake-agent only.
- **Ledger probes:** every undocumented `claude` behaviour is a ledger row in `viola-agent-claude` with a `viola verify` probe that has a post-condition check.
- **Cross-platform:** "Every OS-specific branch compiles and is tested on its CI runner".
- **Crate layout:** flat `crates/viola-<area>` plus the root bin. No separate test crate is declared.
- **Existing scopes present:** no.
