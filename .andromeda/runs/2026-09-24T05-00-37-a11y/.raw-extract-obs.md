## 6. Obs Plan Excerpt

### Obs Tier
- **Tier:** Standard (obs plan wording: "`Standard (1)`, with Minimal-tier exporter carve-outs")
- **Justification:** "The tests tier is Comprehensive (2), and obs sits one step below it, within the ±1 band. The security tier is Minimal (the passed variable, and "no compliance triggers"), so there is no Comprehensive driver". The carve-outs: no network OTLP exporter and no opt-in error reporter, because upstream bans outbound network calls and new listeners.

### Log Format JSON Schema (binding)
Obs Section 6 does not define its own schema. It points to tests: "The binding text is reproduced verbatim in Section 3 → Log format JSON schema, from upstream-context Section 5 Test Plan Excerpt → Test Harness Contract Summary. This section only **adds** fields and the D-01…D-06 enum values; it renames and removes nothing." The binding block reads:

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
- **Agent parsing:** lines parseable with `jq -c 'select(.event=="dialog-raised")'` or equivalent; NEVER multi-line stack traces. A panic in a hook is caught and logged as one `level:"ERROR", event:"panic"` line.
```

Obs additions to the binding schema. All were accepted on 2026-09-24 and carried to tests as amendment D-21.
- **New `event` values:** `release-from-driver`, `liveness-changed`, `state-recovered`, `sse-opened`, `sse-closed`, `parse-rejected`.
- **New `process` value:** `cli`, for short-lived verbs that have a resolved instance.
- **Null encoding:** a null `corr` or `instance` is written by leaving the key out. "absent = null". `schemas/diag-line.v1.json` rejects a literal `null` for either key.
- **Fields deliberately not emitted (D-12):**
  - `trace_id` / `span_id`
  - `service.name`: emitted once, as `service_name` on `process-start`
  - `event_type`: "this is what `event` already does, and a duplicate would amount to a rename"
- **Constraints:** `level` never includes TRACE. `message` is a static literal equal to the event name. Values are codes only, in kebab-case. No line may contain the GUI token, a `Cookie` header, `?t=`, or any stripped `CLAUDE*` value.
- **`http-request` additive fields:** `method`, `path` (from `uri.path()` only), `route`, `status`, `problem` (a Problem URN), `duration_ms`, `skipped`.

### Service Identity
- **service.name:** hardcoded `"viola"`, as `viola_core::SERVICE_NAME`. It is the binary name and is already served as `"name":"viola"` by `/health` and `/api/info`. It is never read from `OTEL_SERVICE_NAME` / `SERVICE_NAME`, because env vars are not a configuration channel.
- **service.version:** set at compile time: `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` in viola-core. The same value is used for the log `version`, the channel `sender`, the snapshot `writer` and `/health.version`.
- **deployment.environment:** N/A. viola is local-only with no hosting, and there is no env-var fallback.
- **Where identity appears in logs:** v1 builds no OTel Resource. Identity is emitted as fields on `process-start`: `service_name`, `version`, `os` (`windows|macos|linux`) and `pid`. Every line carries the role in `process` and the instance in `instance`.

### Sentry User-Feedback Widget (if applicable)
- **Widget pick:** N/A. The obs plan has no user-feedback widget. Its platform pick is "local error capture only, with no external platform". sentry 0.49.3 / sentry-tracing 0.49.3 are "**not admissible**" because they break the egress ban ("viola makes no outbound network calls") and the cargo-deny C-crate and Tokio bans.
- **Trigger surface:** N/A. Nothing appears in the UI. Errors that reach the GUI page show up as Problem URNs, each mirrored by `http-request{status, problem}`:
  - `urn:viola:problem:state-unreadable` (503, rack strip)
  - `urn:viola:problem:unauthorized` (401, access strip)
  - `not-found` (404) and `method-not-allowed` (405), defensive rack strips
  - `host-not-allowed` (403) never renders the page; the browser shows the raw Problem JSON.
- **Default state:** "there is no reporter, so there is nothing to opt into." A future reporter would need a `ureq` + `rustls` transport, the NEVER-log floor in `before_send`, off-by-default behaviour, no env-var enablement, and a security Decisions Log entry for the egress.

### Focus-Relevant Span Coverage (filtered)
(No focus-relevant spans in obs plan Section 4. a11y Phase 3 will recommend focus tracing spans that obs may add later: focus.shift / focus.trap.enter / focus.trap.exit / focus.restore.)

Frontend limits in v1: the web-spa frontend (Lit 3.3.3) is instrumented only at its boundary. "@opentelemetry/sdk-trace-web 2.11.0 is not admissible (OTLP POST impossible under GET-only/405, and it needs a bundler)". web-vitals 6.2.2 is not vendored in v1 (D-17). No browser-side span can therefore carry focus, keyboard, modal or route data. Obs Section 4 describes a "Frontend observable boundary" that Playwright reads and that a11y uses "to derive state and error surfaces; attribute names and announcements stay design/a11y-owned". It relates to state and status, not focus:
- **`<viola-atis>` tape state** (`TAPE connecting|live|stopped`, plus the `skipped N · N · N` counters): a11y correlation: status changes that need announcing. Server-side matches: `sse-opened` (→ `live`), `sse-closed{close_cause}` (→ `stopped`; `close_cause` ∈ `client-gone|server-shutdown|tail-error`), and `http-request{route:"/api/sessions", skipped}`.
- **`<viola-readback>`** (on `<viola-event-feed>` send lines and `<viola-transfer>` markers; states `open|read back|unable|unconfirmable`, with `unable · <reason> · <detail>`): a11y correlation: readback status announcements. Server-side matches, joined by the send `cursor` in `corr`: `send-issued` (open), `send-confirmed{confirmed:true}` (read back), `send-confirmed{confirmed:false}` (unconfirmable), `send-refused{refusal, detail}` (unable).
- **`ui.http_request`** for `/` and `/assets/*` (page load): a11y correlation: none for focus. The time-to-interactive proxy is read from Playwright `performance.getEntriesByType('navigation'|'resource')` and joined to server `http-request` / `sse-opened` lines by timestamp.
