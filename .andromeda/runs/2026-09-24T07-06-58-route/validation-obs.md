# Obs validation — route draft

## Insert
- Between `Diagnostics plane` and `Observability gates`: **"Log redaction wire — skip-all span field allow-lists, redacted payload types, fixed error displays, content-bearing records routed only to instance detail files"** (epoch: `Epoch 1 — Foundation`)
  Reason: The obs plan (§3 Bootstrap phases, pii-scrubbing-wire, and §8 Integration points 1–5) requires the redaction wire to come after the logger stack and before obs-ci-gate-wire, but the draft first covers it in Epoch 6 (`Sanitised errors and never-log floor`), after four epochs of telemetry-emitting feature chunks.

## Rewrite
- `Diagnostics plane`: "per-role home-level JSON sinks" → "owner-checked per-role JSON sinks with process-start service identity"
  Reason: The draft names no chunk for service identity (§3 Bootstrap phases service-identity-wire: one viola-core name/version constant) or for the owner, 0600/0700 and no-symlink checks on diagnostics files before the first write (§3 OTel SDK init, step 4); both are part of the Foundation logger init.
- `Supply-chain and workflow gates`: "cargo-deny families plus weekly advisory run" → "cargo-deny families incl. telemetry-crate and redaction-toggle bans, plus weekly advisory run"
  Reason: §3 Bootstrap phases otel-sdk-install is a no-op record plus bans on opentelemetry-otlp, opentelemetry-stdout, sentry, tracing-appender and env-filter, and on veil `toggle` (D-19). No chunk carries it.
- `Observability gates`: whole line → "Observability gates — panic hook first, zero-panic, schema, bare-instrument, abort-panic gates, canary secret scan before any upload, print and raw-log lint bans with test-binary exemption"
  Reason: §9 Gate commands require G1 (bare `#[instrument]`) and G3 (`panic = "abort"`) next to G2 and G4, and the draft leaves both out.
- `Quality gates`: "hook perf budgets in their own job" → "hook perf budgets inside the per-OS E2E job"
  Reason: §9 Step order 1 and §10 (Obs overhead on `hook`, D-29/D-30) rule out a separate perf job. It would upload no diagnostics or collide on `diag-<os>`, and a hook panic exits 0, so only G2 over the perf home can see it.
- `viola ui loopback server`: "CSP" → "CSP, path-only request lines"
  Reason: §8 Data classification and §11 Spans/Traces require `http-request` lines to record `uri.path()` only, with no headers, so the `?t=` token and the `Cookie` header never reach the log.
- `Resumable SSE feed`: "uncompressed, no polling" → "uncompressed, no polling, logged stream lifecycle and liveness transitions"
  Reason: §3 Bootstrap phases heartbeat-tick-wire and §3 Heartbeat ticks (D-02, D-04) put the `sse-opened`/`sse-closed` drop guard and the transition-only `liveness-changed` emitter in this feed, and the draft has no chunk for them.
- `Linux and macOS parity`: "full fake-agent end-to-end suite green on both" → "full fake-agent end-to-end suite and obs gates green on both, identical diagnostics line schema"
  Reason: The multi-platform-exporter-compat trigger in §1 requires the same sink and fields on openpty and Unix sockets as on ConPTY and named pipes, and this chunk is where those Unix paths first emit lines.
