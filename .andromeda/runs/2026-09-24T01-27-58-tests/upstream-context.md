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

## 2. Security Plan Excerpt

### Security Tier
- **Tier:** Minimal (0), with targeted elevations for the local privilege boundary
- **Justification:** "viola is a local-only, single-user tool. It has no accounts, no public network listener, no database, no stored credentials and no regulated data." The elevations are: IPC endpoint access control and server impersonation, paste breakout in `send.text`, GUI output encoding, GUI readability by other users and origins, v1.x brake auth, and `~/.viola/` integrity.
- **Context:** No compliance triggers. Local-only install. CI runs on windows-2025, macos-latest and ubuntu-latest, with tests against the fake agent and recorded fixtures. The real `claude` CLI and `viola verify` run only locally. macOS is CI-only in v1.

### Attack Vectors
- **Vector 1:** Local IPC channel (JSON-RPC over ndjson: `send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`, `hook.dialog`, `hook.event`). This is the highest-impact surface. Scope: `viola-channel` server/client and `viola` bin dispatch.
  Mitigation: Windows pipe DACL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, remote clients rejected. A squatted name makes `run` exit 1. Unix sockets live in a verified 0700 per-user dir, with `mode(0o600)` secondary. The server drops peers whose euid differs before reading any frame. Before writing any frame, the client checks the server's pid (and euid on Unix) plus start time against `snapshot.json`; macOS checks euid and the dir only. On mismatch, CLI/MCP report `instance-unreachable` (exit 21) and `hook` exits 0 with no body. `release` carrying `from` gets `-32602` `release-from-driver` (CLI exit 20). A frame over `MAX_FRAME` (16 MiB) gets `-32600` and the connection closes.
- **Vector 2:** `send` text and `answer` free text written into the PTY as a bracketed paste. Scope: `viola_core::validate_paste_text`, the `run` paste writer and the `answer` handler.
  Mitigation: LF, CR and TAB are allowed. Every other C0 control (ESC above all), DEL and C1 (U+0080–U+009F) is refused. The check runs on decoded `char`s, not bytes. Text is rejected, never stripped, as `not-delivered`/`control-character`, checked before the other refusal reasons. It runs on the client and again in the wrapper, which is authoritative.
- **Vector 3:** Loopback GUI (`127.0.0.1:47319`). Routes: `/`, `/assets/*`, `/health`, `/ready`, `/api/info`, `/api/sessions`, `/api/links`, SSE `/api/events`; v1.x adds `POST .../pause` and `.../unlink`. Scope: `viola-ui`.
  Mitigation:
  - **Host allowlist:** outermost layer, every route. Host must be exactly `127.0.0.1:<port>` or `localhost:<port>`, else 403 `host-not-allowed`. Missing, wrong-case, trailing-dot or wrong-port all fail.
  - **Token exchange:** `GET /?t=<token>` (32 getrandom bytes, constant-time compare). A match sets `viola_<port>` (HttpOnly, SameSite=Strict, Path=/, no Max-Age) and returns 303 to `/`. A mismatch returns 401 `unauthorized` with no cookie.
  - **Cookie gate:** `/api/*` and SSE need the cookie, else 401. `/`, `/assets/*`, `/health` and `/ready` are ungated.
  - **Methods and CORS:** any method other than GET gets 405. No CORS headers are sent.
  - **Headers on every response:** CSP `default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'; require-trusted-types-for 'script'`, plus `nosniff`, `no-referrer` and CORP `same-origin`. `no-store` on `/api/*` and on the exchange response, including its 401.
  - **`Last-Event-ID`:** each pair parses as `ViolaName` and `u64`. Any bad pair drops the whole header, and the stream starts from each file's current end.
  - **Assets and SSE:** only `/assets/*` is compressed, never SSE. Assets are embedded, never served through `ServeDir`. Event text is rendered as text only.
  - **v1.x POSTs:** cookie plus `Sec-Fetch-Site` of `same-origin` or `none`. Without that header, `Origin` must exactly match the allowed host, else 403 `cross-origin-forbidden`. A non-empty body gets 400. A bad `{name}` gets 404.
  - **Token handling:** no route re-issues the token.
- **Vector 4:** Hook stdin from Claude Code, covering all hook events and `hook statusline`. Scope: the `viola-agent-claude` parser and `viola hook`.
  Mitigation: Stdin is capped at `MAX_FRAME`; oversize fails open (exit 0, no body, a diagnostics note). Parsing is tolerant, but dialog kind and decision map to closed enums. A bad `resets_at` becomes `"unknown"`. `hook` never writes stderr or exits non-zero, even when a security check fails. `statusline_command` runs only after the strict-modes check passes. A non-null dialog decision needs a `viola verify` stamp.
- **Vector 5:** MCP over stdio (`send`, `wait`, `last`, `answer`, `list`). The driver is an LLM. Scope: `viola-mcp`.
  Mitigation: schemars schemas are advisory, since rmcp does not enforce them. Handlers parse `target` as `ViolaName`, `dialog_id` as `u64` and `response` into closed enums, and apply `validate_paste_text` to `text` and free-text answers. rmcp features are only `server` and `transport-io`. `isError` results carry only codes.
- **Vector 6:** CLI args, flags, env and stdin (`--home`, `VIOLA_DIR`, `answer` stdin/`--file`, `run <name> -- <program>`). Scope: clap in `src/main.rs` and `viola-state`.
  Mitigation: Names go through `parse_viola_name`. `--home` and `VIOLA_DIR` are canonicalized and must pass strict-modes before any snapshot or ledger read. `answer` JSON is capped at `MAX_FRAME` and parsed into closed enums. `config.json` is parsed tolerantly, with thresholds as `Percent` (0–100), port as `u16` and a `v` check. No config value, env var or flag can disable a control.
- **Vector 7:** Filesystem state under `~/.viola/`. Scope: `viola-state`.
  Mitigation:
  - **Unix modes:** dirs 0700, files 0600, pinned exe 0700. The mode is set explicitly on the temp file, and the write is discarded if it can't be set.
  - **Unix strict-modes:** refuse if a trusted dir or file is group/world-writable or not owned by euid. Also refuse if the home, `instances/<name>/` or `ui/` has `mode & 0o077 != 0`.
  - **Windows strict-modes:** refuse on:
    - an unreadable descriptor, a NULL DACL, or a volume without persistent ACLs
    - an owner other than the user or Administrators
    - any allow ACE, inherit-only included, granting write, delete, WRITE_DAC or WRITE_OWNER to a SID other than the user, SYSTEM or Administrators (`CREATOR OWNER` counts as the user)
    - a read grant to such a SID on the home, `instances/<name>/`, `diagnostics/`, `ui/` or an existing `events.ndjson` (generic bits tested)
  - **Where it runs:** `run`, `ui`, `mcp`, every `hook`, and CLI verbs that resolve an endpoint.
  - **On failure:** exit 1 for `run`, `ui` and `mcp`; exit 0 with no output for `hook`; exit 21 for CLI verbs.
  - **`--home` outside `%USERPROFILE%`:** gets a protected user + SYSTEM DACL, or is refused.
  - **Pinned exe:** re-hashed with SHA-256 (first 16 hex chars) against `<hash>`; a mismatch exits 1.
  - **Rewritten every start:** `plugin/` files and `settings.json`.
  - **ndjson reads:** lines are capped at `MAX_FRAME`; an over-long line counts as `torn_lines`. Tailing ignores symlinks and invalid names.
- **Vector 8:** Child spawning and PATH resolution (`claude agents --json`, the `run` child and its `--version` probe). Scope: `viola-agent-claude` and `viola-pty`.
  Mitigation: Output is capped at `MAX_FRAME`; a parse failure becomes `unknown`. A row `name` is never a `ViolaName` or `target`. Human-mode `list`, `wait` and `last` escape C0/C1 (keeping `\n` and `\t`). On Windows the npm shim resolves to `claude.exe`, and a `.cmd`/`.bat` PTY child makes `run` exit 1. Hook, MCP and statusline commands use the absolute pinned path. Handles are never inherited. vt100 runs under `catch_unwind`; a panic gives `not-delivered`/`input-not-ready` and passthrough continues.
- **Vector 9:** Supply chain (Cargo deps, GitHub Actions, v1.x release). Scope: CI and `deny.toml`.
  Mitigation: `cargo deny check` runs in CI, plus a weekly advisories run. zizmor checks the workflows. Actions are SHA-pinned, with `permissions: {}` and `contents: read`. `Cargo.lock` is committed. The only advisory ignore is RUSTSEC-2017-0008.
- **Parser surfaces for fuzz/property coverage ("tests owns the cases"):** `validate_paste_text`, the `Last-Event-ID` parser, channel ndjson framing with `MAX_FRAME`, the hook stdin parser (incl. `hook statusline`), the `claude agents --json` parser, and the vt100 feed under `catch_unwind`.
- **Error-sanitization expectations:**
  - External errors (CLI `--json`, MCP `isError`, Problem Details, channel `error.data`) contain no absolute paths, upstream text, tool `input`, serde paths or values, internal type names, anyhow chains, or token/URL/`.url` contents.
  - `-32603` has the fixed message "internal error" and `data: null`.
  - `state-unreadable` never names the path.
  - The only path disclosure is the cookie-gated `/api/info` `viola_home`.
  - Logs never contain the token, the `Cookie` header, the `?t=` query string, or `CLAUDE_CODE_MESSAGING_*` and other stripped `CLAUDE*` values.

### Anti-Patterns Rejected
**IPC and authentication**
- **Default interprocess Windows DACL**: grants read to Everyone and anonymous.
- **Logon SID in the pipe SDDL**: other logon sessions of the same user must still connect.
- **Socket in `/tmp` or at the `$TMPDIR` root**: another user can squat it (Kea CVEs).
- **`mode(0o600)` as the only Unix control**: unsupported on macOS and may be ignored.
- **Linux abstract-namespace sockets**: no filesystem permissions.
- **Writing any frame before server verification, or taking reference values from a snapshot that failed strict-modes**: impersonation, attacker-written references.
- **Default Windows client connect without SQOS, even as a fallback**: client impersonation.
- **Treating channel `from` as identity**: it is self-reported.
- **Per-instance 0600 proof file for `release`**: a same-user agent can read it.
- **Comparing token or cookie with `==`**: must be constant-time.
- **Cookie without HttpOnly and SameSite=Strict, or token left in the URL**: token exposure.

**Input**
- **Trusting schemars schemas or client checks alone**: the wrapper must re-validate.
- **`instances/<name>` joined from a non-`ViolaName` string**: path traversal.
- **Skipping bad `Last-Event-ID` pairs**: must drop the whole header.
- **Stripping controls silently, or refusing LF, CR or TAB**: breaks exact-match delivery confirmation; multi-line text is normal.
- **Control check over raw bytes**: UTF-8 continuation bytes look like C1.
- **`read_line` or `read_to_end` without `Read::take(MAX_FRAME)`**: unbounded memory.
- **serde_json `unbounded_depth`**: keep the 128 limit.
- **Free `String` for `behavior`, `dialog_id`, wheel holder or dialog kind**: must be closed enums or integers.
- **Using a `claude agents --json` row `name` as a `ViolaName` or `target`**: untrusted child output.
- **garde or validator crates**: arch chose nutype.

**Data protection and filesystem**
- **Default umask, or a group/world-writable pinned exe**: must be 0700 dirs and 0600 files.
- **Skipping the Windows owner+DACL check, or passing a NULL DACL, a non-persistent-ACL volume or an inherit-only GENERIC grant**: other users could read or write.
- **Running `statusline_command` when the home fails strict-modes**: code execution.
- **Reusing the pinned exe without a SHA-256 re-hash, a non-crypto `<hash>` (FNV, `DefaultHasher`), or reusing `plugin/` files unrewritten**: substitution goes undetected.
- **`.cmd`/`.bat` PTY child via portable-pty**: bypasses BatBadBut escaping.
- **Inheritable channel handles, or a PTY replacement with `bInheritHandles = TRUE`**: handle leak to the child.
- **Binding to `0.0.0.0` or `::`**: plain HTTP is loopback-only.
- **Token not from getrandom**: predictable.
- **Fixtures from real prompts, or unchecked for paths and usernames**: PII leak.

**API and GUI**
- **`CorsLayer` or `Access-Control-Allow-*`**: would enable cross-origin reads.
- **axum-extra `Host` extractor, or axum `http2`**: can lose the port.
- **Host allowlist on `/api/*` only**: DNS rebinding on other routes.
- **`/api/*` or SSE without the cookie, or a state-changing route without cookie + `Sec-Fetch-Site`/`Origin`**: reachable by other local users.
- **Compressing SSE, or excluding it by content-type predicate only**: breaks SSE, 406s.
- **`ServeDir` or any filesystem asset handler**: path traversal.
- **Access decisions on client address or `X-Forwarded-For`/`Forwarded`**: there is no proxy.
- **`innerHTML`, `v-html`, `dangerouslySetInnerHTML`, Markdown-to-HTML, inline scripts or handlers, `eval`**: XSS, CSP and Trusted Types violations.
- **rmcp `transport-streamable-http-server` or `auth` features**: all 2026 advisories sit there.
- **`tower-csrf` or `axum_csrf`**: rejected in research; the Origin check is hand-written.
- **Rate limiting in v1**: not needed at this tier (256-bit token); exhaustion is an accepted risk.
- **Missing size limits (`MAX_FRAME`, POST body rejection)**: exhaustion.

**Secrets and logging**
- **Logging `CLAUDE_CODE_MESSAGING_*` or other stripped `CLAUDE*` values, the token outside the launch line and `.url` file, the `Cookie` header or the `?t=` query**: secret leak.
- **Credential in a URL past the exchange, or one token shared across launches**: Referer leak, no rotation.
- **Committing `.env`, `--home` test dirs or signing material; hardcoded credentials in source or plugin files**: secret leak.
- **Drift reports or anyhow chains in external errors, or a serde-sourced chain on stderr**: quotes input values.
- **stderr output or a non-zero exit from `viola hook`**: hooks must fail open.
- **Unescaped C0/C1 in `wait`, `last` or `list` human output**: terminal injection.
- **tracing-subscriber `<0.3.20`**: RUSTSEC-2025-0055.
- **Readable `diagnostics/`, or absolute paths in external errors**: content and PII leak.

**Code patterns and universal**
- **Upstream text treated as a command or config, or a second shell-out**: R1.
- **Screen signatures, harness prefixes or local-command lists built from runtime text**: must come only from compiled ledger rows.
- **Cookie treated as anything but an opaque random value**: it carries no claims.
- **interprocess `try_overwrite`**: TOCTOU deletion.
- **vt100 outside `catch_unwind`**: a panic kills the terminal path.
- **Actions by mutable ref**: supply-chain drift.
- **rust-cache in the release workflow; `self_update` without `signatures`**: cache poisoning, unsigned updates.
- **New listener or channel method without a Decisions Log entry**: unmapped surface.
- **A security refusal blocking the human**: refusals are for automation only.
- **Hook, MCP or statusline commands resolving `viola` via PATH**: code execution by whatever is first on PATH.
- **Non-null `hook.dialog` without a verify stamp, or a non-`verify` process writing `stamps.json`**: the stamp gates `allow`.
- **Any process other than the wrapper writing `snapshot.json`**: it controls endpoint and exec.
- **Config, `VIOLA_*` env or a flag disabling any control**: env is not a config channel.

### Data Classifications
- **Prompts and assistant output** (high): in `instances/<name>/events.ndjson`, wrapper memory, SSE, and `wait`/`last`. Never rotated; retention is open (must keep 0600 and valid offsets). Testability hint: testable (temp `--home` + fake agent).
- **Tool args, plans, dialog Q&A** (high; may hold secrets): `permission {tool, input}`, `plan` and `question` in `events.ndjson`, SSE and `hook.dialog`. Drift reports go only to `diagnostics/` (0600). Testability hint: testable (recorded hook fixtures).
- **Config and code-bearing state** (integrity-high): `config.json`, `settings.json`, `snapshot.json` (`statusline_command`, `endpoint`, `pinned_bin`, `pid`, `started_at`), `ledger/stamps.json`, plugin `hooks.json`/`.mcp.json`, and the pinned exe. Module: `viola-state`, `run`, `verify`. Testability hint: testable for Unix modes, hash mismatch and rewrite; partially testable with stubs for the Windows DACL cases (per-OS CI matrix).
- **GUI per-launch token** (high, secret): process memory, `ui/<port>.url` (0600, removed on graceful shutdown) and the `viola_<port>` cookie. Module: `viola-ui`. Testability hint: testable.
- **Inherited credentials** (high, transient): `CLAUDE_CODE_MESSAGING_TOKEN`/`_SOCKET` and other `CLAUDE*` values, held only in the wrapper env and stripped from the child. Testability hint: partially testable with a stub child.
- **Operational metadata** (low; PII is the OS username in paths): `/api/info` (cookie-gated), snapshot pids and ids, `budget.json`, `/api/sessions`. Testability hint: testable.
- **Payment, health or multi-user PII**: none (no database, no accounts).

## 3. Design System Excerpt

### Surfaces
- **web-spa** (`viola ui`, loopback page at `http://127.0.0.1:47319`; Lit 3.3.3 light-DOM `viola-*` custom elements, hand-written `@layer` CSS, no JS build step, headless-browser verified; CI headless GUI checks run on ubuntu, where the fonts resolve to DejaVu Sans Condensed + DejaVu Sans Mono) — a single view-only page, the "strip bay". It shows a sticky ATIS/budget header, WRAPPED and UNWRAPPED · READ-ONLY session racks, transfer markers and the event tape (SSE, most recent at the bottom). v1 has no controls.
- **cli** (Rust clap 4.6.7 terminal binary; human TTY at expression 0.2, and `--json` / non-TTY / `NO_COLOR` / `TERM=dumb` / `viola run` passthrough at 0.0) — flat verbs (`run · send · wait · last · list · answer · verify · pause · release · link · unlink · ui · plugin install`). `viola list` prints a k9s-style fixed-column board. Results go to stdout. Context lines, refusals, `hint:` lines, errors and the `viola ui` launch line go to stderr. `--json` prints one JSON document on stdout with typed exit codes (0, 1, 2, 10–14, 20, 21). `viola hook` and `viola mcp` have no human design surface.

### Layout Categories
(The design system has no explicit Layout Categories section. The categories below come from its Navigation Pattern and Component Patterns.)
- **Sticky status header (ATIS / fuel-state)** — used in: web-spa (`<viola-atis>`), cli (the `BAY` line at the top of `viola list`)
- **List / fixed-column table (session racks of strips, grouped WRAPPED vs UNWRAPPED · READ-ONLY, ordered by name, stable slots)** — used in: web-spa (`<viola-session-row>` in `role="table"` racks), cli (`viola list`)
- **Inline handoff marker lines (transfer markers in the rack gap)** — used in: web-spa (`<viola-transfer>`)
- **Log / feed with expandable detail (tower tape of `<details>` lines, capped at 2000 lines, follows the bottom)** — used in: web-spa (`<viola-event-feed>`, `role="log"`)
- **Status / error strips (401 access strip, 503 state-unreadable, tape stopped)** — used in: web-spa
- **Empty and initial-read states (printed phraseology, never spinners or skeletons)** — used in: web-spa (`sessions: no reading yet`, `no wrapped sessions — start one with  viola run <name> -- claude`, `TAPE live since … — no events yet`), cli (`no wrapped sessions  start one: viola run <name> -- claude`)
- **Single result / verb-output lines (readback mirror, wait/last, wheel/link verbs, verify step counter)** — used in: cli
- **Reserved v1.x controls (brake buttons `I HAVE CONTROL` / `UNLINK`, Raised-3 confirmation)** — used in: web-spa (not built in v1)

Navigation: web-spa is one page with no routes. Its vertical order is ATIS header → WRAPPED rack → UNWRAPPED · READ-ONLY rack → TAPE. Width breakpoints: ≥1024px is the primary layout. At 760–1023px each strip wraps to two lines. Below 760px is reserved for a later phone view. Horizontal scroll never happens.

### Brand Identity Anchors
- **Arrival amber** (`--attention` / #D97706) — drives selector for: the cocked strip's `.band` and its `DIALOG` word (`viola-session-row[data-dialog="pending"]`, shifted by `translateX(12px)`), and the CLI `DIALOG` word in `viola list`. It is used for `dialog_pending` only.
- **Departure blue** (`--handoff` / #5B8DB8) — drives selector for: transfer-marker arrows and names (`→ builder`, inbound `← overseer`) in `<viola-transfer>`, and the names in `link` / `unlink` tape lines. It is never used in the CLI.
- **Readback box states** (`viola-readback[data-rb="open|read|refused|unconfirmable"]`, `.rb` 16px square, buff #E6D8AE fill with `RB` in graphite #3B3A36 when read) — drives selector for: the send-line and transfer-marker readback. Each state has a printed word: `open` / `read back` / `unable · <reason> · <detail>` / `unconfirmable`. The CLI mirror is `[RB]` / `[  ]` / `[/ ]`.
- **Session-row state attributes** (`data-liveness="stale"`, `data-dialog="pending"`, `data-wrapped="false"`, `data-live` on SSE arrivals) — drives selector for: stale (lamp-off #9C9278 ink on anthracite), cocked, unwrapped (dashed rail outline, no band) and newly arrived tape lines or markers. All state lives only in `data-*` attributes. `style="…"` and `styleMap` are never used.
- **ARIA roles** (`role="table"` rack, `role="row"` strip, `role="cell"`, `role="columnheader"` captions NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, `role="log"` tape, visually hidden `role="status" aria-live="polite"` region) — drives selector for: role-based E2E queries. The live region announces only strips turning cocked, readback refusals and `TAPE stopped`. The first tab stop is the skip link `skip to tape`. `document.title` becomes `DIALOG <name> · viola` (plus `+N`) while a strip is cocked and `viola` otherwise.
- **Callsign typography** (NAME cell, Bahnschrift stack 600 15px, never text-transformed) — drives selector for: the session-name text match. A `ViolaName` is lower-case `[a-z0-9-]` and appears exactly as typed. Only unwrapped names may end in an ellipsis. In the CLI the NAME column is bold.

## 4. Layout Templates Excerpt

### Layout Types per Surface

- **web-spa:** bay-steady-state (≥1024), bay-narrow (760–1023), bay-first-reading-empty, bay-degraded, tape-line-expanded (one route `/`; screens are bay states; <760 not laid out in v1)
- **cli:** viola-list (TTY), viola-list (piped/NO_COLOR/TERM=dumb), viola-list-json, viola-send, viola-wait/last, wheel-handoff-dialog-verbs (pause, release, link, unlink, answer), viola-verify, viola-help, viola-run, viola-ui (launch line deferred)

### Signature Placements

- **web-spa tape send line:** signature element = `<viola-readback>` (`data-rb`: `open` / `read` "RB" `read back` / `refused` strike `unable · <reason> · <detail>` / `unconfirmable`) at far right of every `send` line in `<viola-event-feed>` (rationale: "readback on the right, read as one unit."). Opens on SSE `send-issued`, fills on matching driver `prompt-submitted` (or `session-start` cause `clear`), strikes on `send-refused`; `turn-ended` never touches it; all instant. Pre-CL-1 only `read` renders.
- **web-spa outbound transfer marker:** signature element = same `<viola-readback>` at far right of outbound `<viola-transfer>` (`→ <driven>  since <time>`) in WRAPPED rack gap (rationale: "it prints only the word `unknown` with no box" until a send is seen). Flips same frame as tape box; inbound `← <driver>` has no box.
- **web-spa cocked strip:** signature element = `<viola-session-row data-dialog="pending">` shifted `translateX(--cock-offset)` in WRAPPED rack (rationale: "when a strip's `dialog_pending` turns true"). Shows `DIALOG <kind>`; `document.title` = `DIALOG <name> · viola` (+N). Never in tape, unwrapped, or CLI.
- **web-spa header:** signature element = sticky `<viola-atis>` at top, fixed cells `VIOLA`, `BAY <addr>`, `5H`/`7D <n> %`, `read <age> ago`, `gate open`|`budget-paused`, `TAPE live since…`|`connecting`|`stopped · viola ui not answering`, `skipped a · b · c` (rationale: "It never truncates").
- **web-spa regions:** signature element = labels `WRAPPED`, `UNWRAPPED · READ-ONLY`, `TAPE` in fixed order; captions `NAME LIVE STATUS WHEEL DIALOG CLI` always drawn (rationale: "the only wayfinding"). Tab order: `skip to tape`, tape summaries oldest→newest, `N new lines below` → `#tape-end`; strips not focusable.
- **web-spa state strips:** signature element = fixed copy in rack/tape position: `sessions: no reading yet`, `no wrapped sessions — start one with  viola run <name> -- claude`, `no unwrapped sessions`, `TAPE live since <time> — no events yet`, `unable · state-unreadable  viola home could not be read` (503), `TAPE stopped · viola ui not answering` (rationale: "Never: 'Loading…', a spinner"). Missing = `unknown`, absent-by-contract = `n/a`. >2000 lines → `older lines trimmed from view: N …`.
- **cli viola send:** signature element = first-column mirror: `[  ] open` issue line (stderr, TTY only), then `[RB] read back … cursor <n>` (stdout, exit 0) | `[/ ] unable <name> <reason> <detail>` (stderr, exit 10–14 + `hint:`) | `[  ] unconfirmable` (stdout, exit 0) (rationale: "appended, never redrawn"). No colour; `--json` = typed document, no glyph.
- **cli viola list:** signature element = `BAY` header, caption row, name-ordered rows, `-- UNWRAPPED - READ-ONLY --` separator (rationale: "same six captions, words and slot order as the web racks"). TTY: bold NAME, amber `DIALOG`, dim `stale` row; piped = no SGR; `--json` shape = `/api/sessions`.
- **cli refusals:** signature element = stderr `unable  <name>  <reason>  <detail>` + last-line `hint:` (rationale: "The exit code is the terminator"). Exits: 10 human-typing, 11 budget-paused, 12 unverified-cli, 13 not-delivered, 14 unknown, 20 wrapper fault, 21 instance-unreachable, 1 already live. Results stdout, diagnostics stderr. `verify` ends `stamped <ver>  <n> pass  <n> fail`.

## 5. Creator Brief Excerpt

Sources: `.andromeda/input.md` (which folds in `refs/`) plus the current `refs/viola-brief.md` (§4.2 M1–M7 are newer than the folded copy). Quotes are verbatim.

### Must-Work Scenarios

- R2: "Exactly one driver at a time. A submitted prompt the bridge did not send means the human took the wheel, and automation pauses by itself — no button needed."
- R3: "Type only at a turn boundary, as one bracketed paste followed by Enter. Never send Esc". S1: "A bracketed multi-line paste plus Enter arrived as ONE prompt, newlines inside."
- R5: "a session started without the wrapper makes every hook a silent no-op." R8: "A wrapper strips its parent's session identity" (S6: 14 `CLAUDE*` variables removed; the child registered as a session of its own).
- S3/S7/S8 dialogs: AskUserQuestion answered by "`PreToolUse` … `permissionDecision: allow` with `updatedInput.answers`" (free text and `annotations` too); permission by "`PermissionRequest` … `decision.behavior: allow`"; plan "APPROVE: only `PreToolUse` … `PermissionRequest` `allow` is IGNORED for this tool … REVISE: `PermissionRequest` `deny` with a `message`".
- S5: "A background `wait` that exits on the next `Stop` completed when the driven turn ended".
- Windows: "`claude` on PATH is an npm shim … the wrapper must resolve it. ConPTY did not close the output stream when the child exited, so exit is detected on the process, never on EOF."
- §6 first live test: "the overseer sends `/andromeda-new-session` to the builder, waits for `Stop`, and reads the dashboard" — passed run: "the founder typing into the window took the wheel, `send` was refused, `release` returned it."
- §6: "`/andromeda-new-session` arrived as `C:/Program Files/Git/andromeda-new-session` … take the text from stdin or a file, and warn when an argument carries a rewritten-path prefix."
- M1: "CLI-native modals bypass every hook … A turn boundary per the hooks therefore does not prove the input box is ready." M2: "Harness-injected turns fire UserPromptSubmit" (`<agent-message from=`, `<task-notification>`; the prototype "paused automation twice"). M3/M4: long pastes arrive wrapped in `<pasted_content id="…">`; "A literal `<pasted_content` arrived as `<\pasted_content` … Prompt matching must normalise both." M5: "Local commands fire no UserPromptSubmit." M6: "Hooks found through PATH run a different binary from the wrapper after a rebuild."
- O8 / §7: budget from statusline `rate_limits.five_hour|seven_day.used_percentage`, "then hand over to the user's script unchanged"; "a budget governor belongs in the first version".

### Risk Tolerance Hints

- §7: "Security is a first-class concern: the bridge types into sessions that can run commands. Localhost only by default … the event log as the audit trail."
- §4: "The docs lag the installed build. … Design against the installed CLI, measured." Timing: "Hooks are the source of truth; the screen is an eventually consistent view."
- D3: "cross-platform from the start: Windows, macOS, Linux. Windows is … the first target and the hardest one." O5: "every measurement in §4.1 is Windows-only so far."
- prior-art §4: "viola's on-disk state parses defensively and heals itself from day one." / "The GUI shows only what the hooks and the wrapper observed."

### Test Anti-Patterns (creator's explicit asks)

- §3.4: "CI on all three OSes against a FAKE AGENT — a small program that behaves like Claude Code (prints, waits for input, calls the hook commands). Tests spend no tokens and do not flake; the real CLI runs only in local live tests."
- §3.4: "an agent can verify it through a headless browser, which is how the founder's projects verify UI by default."
- §3.2 / prior-art §4: "Nobody parses the rendered screen for content." / "Viola parses no screen for content (R7)."
- S8: "a first run that seemed to show otherwise was an instrument defect — the spike's hook did not forward `annotations`; fixed and re-measured".
- §4.2: "A long real session surfaced what the fake agent and the short demos could not."
- prior-art §4: "Blanket approval. Auto-yes flags and skip-permissions defaults remove the one checkpoint a human relied on."

### Founder Directions for this tests run (2026-09-24, verbatim; binding from Phase 1 on)

Delivered by the overseer on the founder's behalf, during this run:

1. "Mutation testing is part of the headless contract from chunk 1: cargo-mutants scoped to each chunk diff, surviving mutants are red; include it in the P2 research catalog so the plan can cite it."
2. "Every layer agent-runnable; the fake agent + `viola verify` fixtures are the contract."
3. "Structured JSON logs from every process (run/hook/mcp/ui) in the §3 log format, a `logs` command in the harness, hook trace in diagnostics/ (hook never writes stderr)."
4. "3-OS CI (windows/ubuntu/macos)."
5. "Design cross-lane CL-1: send-issued (cursor, from) and send-refused (refusal + detail) events are expected; cover them in critical paths."
