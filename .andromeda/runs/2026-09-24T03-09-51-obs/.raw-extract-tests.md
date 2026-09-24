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
