## OTel SDK Core

### tracing 0.1.44 (fills the OTel API role; OTel SDK deferred)

- **Version:** 0.1.44 (MSRV 1.65)
- **Last release:** 2025-12-18
- **Status:** actively maintained (tokio-rs/tracing; repo pushed 2026-05-30; not archived)
- **Agent-readable:** yes. It produces JSON-per-line structured logs: spans and events rendered by tracing-subscriber 0.3.23 `fmt().json()` into `diagnostics/*.ndjson`. Configuration: the `tracing` facade in every instrumentable crate, with `#[instrument(skip_all, fields(...))]` on the Sec 4 spans.
- **Fits because:** obs-scope Sec 3 says "v1 initialises no OTel SDK exporter … Phase 2 evaluates whether [an OTel bridge] adds anything over the tests-fixed format". `tracing` is the span/event API that every Sec 2 surface already uses, and rmcp 3.4.1, axum 0.8.9 and tower-http 0.7.1 emit into it. An OTel SDK can be attached later at the root-bin edge without touching instrumented code.
- **Key detail:** The research answer is that an OTel SDK adds nothing in v1.
  - **opentelemetry / opentelemetry_sdk 0.33.0** (2026-09-18, MSRV 1.75):
    - Logs and Metrics API/SDK are stable. Traces is Beta. The OTLP Logs/Metrics exporters are RC, with stable promised in 0.33.1.
    - Default features (`trace, metrics, logs, internal-logs, percent-encoding, rand`) pull no Tokio. Tokio arrives only with `rt-tokio*` / `experimental_*_async_runtime`.
    - It is therefore admissible only at the root-bin edge, with the sync crates depending on `tracing` alone (Sec 5 Vector 9).
    - With no exporter attached it only adds dependencies and cargo-mutants surface.
  - **`Resource::builder()` auto-includes `EnvResourceDetector`**, which reads `OTEL_SERVICE_NAME` and `OTEL_RESOURCE_ATTRIBUTES`. That violates arch "environment variables are not a configuration channel", so any future use must call `Resource::builder_empty()`.
  - **tracing-opentelemetry 0.34.0** (2026-09-23, MSRV 1.75, needs `opentelemetry ^0.33.0`, Tokio only as a dev-dependency) is the sanctioned bridge. It is **deferred**: with `with_current_span(false)` and the tests' closed `event` enum, correlation runs on `corr`, not W3C trace IDs, so bridged span IDs would have no consumer.
- **Source:** https://docs.rs/crate/tracing/latest ; https://github.com/open-telemetry/opentelemetry-rust/releases ; https://docs.rs/crate/opentelemetry_sdk/latest/features ; https://docs.rs/opentelemetry_sdk/0.33.0/opentelemetry_sdk/resource/struct.Resource.html ; https://docs.rs/crate/tracing-opentelemetry/latest

## Structured Logger

### tracing 0.1.44 (instrumentation facade)

- **Version:** 0.1.44 (MSRV 1.65)
- **Last release:** 2025-12-18
- **Status:** actively maintained (tokio-rs/tracing; repo pushed 2026-05-30; not archived). No open RUSTSEC advisory exists against `tracing` 0.1.44.
- **Agent-readable:** yes. It produces JSON-per-line structured logs through tracing-subscriber's JSON formatter. Configuration: the `tracing` dependency goes in every instrumentable crate (root bin, viola-pty, viola-channel, viola-state, viola-agent-claude, viola-mcp, viola-ui). It is Tokio-free and has no C code.
- **Fits because:**
  - It covers every Sec 2 surface: cli short-lived, cli `run`, cli `hook`, ipc-internal channel, ipc-internal mcp and web-spa backend.
  - It covers every Sec 4 must-trace span, for example `run.start › … › pty.spawn` and `send.client › channel.request › run.readiness_gate › pty.paste_write › run.confirm_window`.
  - rmcp 3.4.1 (normal dependency `tracing ^0.1`), axum 0.8.9 (optional `tracing`) and tower-http 0.7.1 (optional `tracing`) already emit into it.
- **Key detail:** By default `#[instrument]` **records every function argument using `Debug`** ("By default, all arguments to the function are included as fields on the span"). That would leak send text, `hook.dialog` payloads and tool `input` (Sec 5 triggers, Vectors 1, 2 and 4). The plan should:
  - require `#[instrument(skip_all, fields(...))]` with an explicit allow-list;
  - ban the `err` and `ret` options on functions whose error `Display` can carry a serde_path_to_error source (Vector 6);
  - add a clippy or grep check for bare `#[instrument]` under the mutation and lint gate.
- **Source:** https://docs.rs/crate/tracing/latest ; https://docs.rs/tracing/0.1.44/tracing/attr.instrument.html

### tracing-subscriber 0.3.23 (features `fmt`, `json`, `chrono`, `registry`; `ansi` off for files)

- **Version:** 0.3.23 (MSRV 1.65). It meets the security floor of `>=0.3.20` (RUSTSEC-2025-0055). Version 0.3.21 was yanked.
- **Last release:** 2026-03-13
- **Status:** actively maintained. No 2026 RUSTSEC advisory against tracing-subscriber was found.
  - The `json` feature enables `serde`, `serde_json` and `tracing-serde`.
  - The `chrono` feature enables `chrono ^0.4.26`, which is already in the tree at 0.4.45.
- **Agent-readable:** yes. It produces JSON-per-line structured logs that `jq -c 'select(.event=="dialog-raised")'` can parse. Configuration:
  `fmt().json().flatten_event(true).with_current_span(false).with_span_list(false).with_ansi(false).log_internal_errors(false).with_timer(MillisUtc).with_writer(<append File MakeWriter>).with_max_level(<from config.json/flag>).init()`
- **Fits because:** This is the tests-bound logger, verbatim (Sec 3 Logging stack; Founder Direction 1), with one init in `main` for every Sec 2 surface. Research found four settings that the tests line alone does not settle:
  1. **Timestamp precision.** Neither built-in timer matches the arch and tests `timestamp` (RFC 3339 UTC, milliseconds, `Z`).
     - The default `SystemTime` timer writes 6 fractional digits (`{:06}`) plus `Z`, which is microseconds.
     - `ChronoUtc::rfc_3339()` calls `to_rfc3339()`, which gives automatic nanosecond precision and a `+00:00` offset.
     - Fix: a custom `FormatTime` of about 5 lines that writes `chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)`, the same call arch mandates for `ts`. `ChronoUtc::new("%Y-%m-%dT%H:%M:%S%.3fZ".into())` also works.
  2. **`log_internal_errors` defaults to `true`.** The docs say "If writing to the writer fails, the error message is printed to stderr as a fallback". That breaks the Hook contract ("hook never writes to stderr") and the `run` terminal rule. Set `.log_internal_errors(false)` for every role.
  3. **`with_span_list` defaults to `true`.** The tests line disables only `current_span`, so a `spans:[…]` array holding span fields would still be appended. Set `.with_span_list(false)`. It renames and removes no required field, and it closes a leak path for span fields.
  4. **The default writer is stdout** (`W = fn() -> Stdout`). `with_writer` must always be explicit: on `mcp` stdout carries MCP frames, and on `hook` it carries the decision body.
- **Key detail:** `on_event` formats each event into a thread-local `String`, then makes **one `io::Write::write_all` call per event**.
  - **Writer.** Use a `MakeWriter` returning a `std::fs::File` opened with `OpenOptions::new().append(true).create(true)`, plus `OpenOptionsExt::mode(0o600)` on Unix and directory mode `0o700`. Each line is then one append write. That meets the arch "one `write` per line" rule and works for concurrent `hook` writers (POSIX `O_APPEND`, Windows append-only handle).
  - **ANSI sanitization.** `with_ansi_sanitization` defaults to `true` in 0.3.23 (protection against terminal injection). Keep it.
  - **Required fields on every line.** With `flatten_event(true)` and `with_span_list(false)`, only event fields are emitted. Wrap `tracing::event!` in a crate-local macro that always attaches:
    - `event`, the Display of a closed kebab-case enum in viola-core, which enforces the tests' closed enum at compile time;
    - `corr`;
    - `process`;
    - `instance`.
  - **Tests.** Point `with_writer` at an in-memory `Vec<u8>` and parse the lines with serde_json. This needs no extra dependency and keeps obs code under the cargo-mutants gate.
  - **Alternatives researched and not adopted:**
    - **tracing-appender 0.2.5** (2026-04-17, MSRV 1.63):
      - `non_blocking` spawns a worker thread over `crossbeam-channel` and **drops lines by default** when its buffer fills.
      - It needs a `WorkerGuard` flush at exit.
      - Both are unsafe for the `hook` role (≤1.0 s `max` gate, must exit 0) and for the zero-unlogged-panics rule.
      - Rotation is unused in v1.
      - It pulls in the `time` crate; RUSTSEC-2026-0009 affects `>=0.3.6,<0.3.47`.
    - **json-subscriber 0.3.0** (2026-07-08, MSRV 1.75). It allows custom top-level fields, but the tests bind the emitter `tracing-subscriber 0.3.23 fmt().json()…` (Founder Direction 1), and it pulls in `uuid`. It is the fallback if the wrapper-macro approach is not enough.
- **Source:** https://docs.rs/crate/tracing-subscriber/latest ; https://docs.rs/tracing-subscriber/0.3.23/tracing_subscriber/fmt/struct.Layer.html ; https://docs.rs/tracing-subscriber/0.3.23/tracing_subscriber/fmt/format/struct.Json.html ; https://docs.rs/tracing-subscriber/latest/src/tracing_subscriber/fmt/time/datetime.rs.html ; https://docs.rs/tracing-subscriber/latest/src/tracing_subscriber/fmt/time/chrono_crate.rs.html ; https://docs.rs/tracing-subscriber/0.3.23/src/tracing_subscriber/fmt/fmt_layer.rs.html ; https://rustsec.org/advisories/RUSTSEC-2025-0055.html ; https://docs.rs/crate/tracing-appender/latest ; https://rustsec.org/advisories/RUSTSEC-2026-0009.html ; https://github.com/mladedav/json-subscriber

### tracing-subscriber 0.3.23 `tracing-log` feature (log-crate bridge)

- **Version:** tracing-subscriber 0.3.23 with the default feature `tracing-log`
- **Last release:** 2026-03-13
- **Status:** actively maintained (tokio-rs/tracing)
- **Agent-readable:** yes. It turns `log`-crate records into the same JSON-per-line output. Configuration: `SubscriberBuilder::init()` installs `LogTracer` automatically when `tracing-log` is on.
- **Fits because:** A crates.io check showed that **portable-pty 0.8.1 and notify 8.2.0 log through the `log` crate, not `tracing`**. interprocess 2.4.4 and sysinfo 0.39.6 emit nothing. Without this bridge, two failure classes would be silently dropped:
  - `pty.spawn` and ConPTY failures (Sec 4 path 1; Sec 5 chaos "child exit detected on the process handle without EOF");
  - notify watcher errors (Sec 4 path 7 `state.tail_notify`).
- **Key detail:** Bridged `log` records carry no `event`, `corr` or `process` fields, and their messages are third-party text. Choose one of two treatments:
  - filter them to `WARN+` with a per-target directive taken from config, not from `RUST_LOG`;
  - or wrap them in the crate-local macro at the seam boundary.

  The standalone `tracing-log` crate was last released as 0.2.0 on 2023-10-25. It ships as a dependency of tracing-subscriber 0.3.23 and should not be listed directly.
- **Source:** https://docs.rs/crate/tracing-subscriber/latest/features ; https://crates.io/api/v1/crates/portable-pty/0.8.1/dependencies ; https://crates.io/api/v1/crates/notify/8.2.0/dependencies

## Stdout OTel Exporter (Minimal floor)

### tracing-subscriber 0.3.23 JSON file sink (`MakeWriter` over a std append `File`)

- **Version:** tracing-subscriber 0.3.23 (`json` feature) plus a std `OpenOptions::append` writer
- **Last release:** 2026-03-13
- **Status:** actively maintained (tokio-rs/tracing)
- **Agent-readable:** yes. It writes JSON-per-line structured logs to `diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>}.ndjson`, ready to paste through `scripts/agent-run.sh logs`. Configuration: `.with_writer(move || file.try_clone().unwrap_or_else(|_| sink_file()))` on a file opened with `append(true).create(true)`, mode 0600, inside a 0700 directory.
- **Fits because:** It fills the Minimal-floor role, an always-on local machine-parseable sink, for every Sec 2 role. Sec 2 forbids a stdout sink on every role:
  - short-lived CLI: stdout carries the `--json` results;
  - hook: stdout carries the decision body;
  - mcp: stdout carries MCP frames;
  - run: the terminal carries the child's screen.

  There is no rotation in v1 (Sec 5 audit-log-retention), and the schema is the same on every OS (Sec 5 multi-platform-exporter-compat).
- **Key detail:** The floor is a **file sink, not stdout**. The canonical OTel floor, **opentelemetry-stdout 0.33.0** (2026-09-18, MSRV 1.75; depends on `chrono`, `opentelemetry` and `opentelemetry_sdk`), is not admissible:
  - it writes only to stdout, which all four roles reserve;
  - upstream calls it "intended solely for educational and debugging purposes … the format of the output may change … unsuitable for production".
- **Source:** https://docs.rs/tracing-subscriber/0.3.23/src/tracing_subscriber/fmt/fmt_layer.rs.html ; https://crates.io/crates/opentelemetry-stdout ; https://docs.rs/opentelemetry-stdout/latest/opentelemetry_stdout/

## Per-Surface Instrumentation Library

### cli (short-lived verbs): tracing 0.1.44 `#[instrument(skip_all)]` + std panic hook (no framework auto-instrumentation)

- **Version:** tracing 0.1.44 and tracing-subscriber 0.3.23; `std::panic::set_hook` (Rust std, workspace MSRV 1.89)
- **Last release:** 2025-12-18
- **Status:** actively maintained (tokio-rs/tracing)
- **Agent-readable:** yes. When an instance resolves, it writes JSON-per-line to `diagnostics/`. Otherwise there is no process-log file, and stderr carries only fixed-message lines. Configuration follows the Sec 3 init order: panic hook → clap → home/instance → open the 0600 append file → subscriber → `process-start`.
- **Fits because:** It covers the Sec 2 surface "cli (short-lived verbs)" and the client halves of Sec 4 paths 2–5 (`send.client`, `wait.block`, `answer.client`, `release`). `exit_code` (0, 1, 2, 10–14, 20, 21) and distinct exit-1 and exit-21 `detail` codes go on `process-exit` (Sec 5 creator-explicit; Founder Direction 4). clap 4.6.7 has no tracing integration, so the spans go on the dispatch functions.
- **Key detail:** Both panic crates were rejected, so the hook is hand-written:
  - tracing-panic 0.1.2 was last released 2024-04-19, outside the recency window;
  - human-panic 2.0.8 writes a human-readable report to stderr and a TOML file.

  The hook formats the `PanicHookInfo` payload and location into one JSON line through the same writer (`level:"ERROR", event:"panic"`, `corr:null`), and it never calls the default hook. Because the payload can quote user data, it goes to `diagnostics/` only.
- **Source:** https://docs.rs/tracing/0.1.44/tracing/attr.instrument.html ; https://crates.io/crates/tracing-panic ; https://crates.io/crates/human-panic

### cli (`viola run` wrapper): tracing 0.1.44 spans on the pty seam, channel server and vt100 gate, plus the `log` bridge for portable-pty

- **Version:** tracing 0.1.44 and tracing-subscriber 0.3.23 (with `tracing-log`); portable-pty `=0.8.1`, which emits through `log ^0.4`
- **Last release:** 2025-12-18
- **Status:** actively maintained (tokio-rs/tracing)
- **Agent-readable:** yes. It writes JSON-per-line to `diagnostics/run-<name>.ndjson` and nothing to the terminal. Configuration is the same init. Replace the panic hook **before** `pty.spawn`, because the default hook would print into the child's screen.
- **Fits because:** It covers the Sec 2 surface "cli (`viola run`)" and these Sec 4 paths:
  - path 1: `run.start › run.collision_check › run.pin_copy › run.version_gate › channel.bind › state.snapshot_write › state.heartbeat_start › pty.spawn`;
  - path 2: `run.readiness_gate`, `pty.paste_write` and `run.confirm_window`;
  - path 5: `run.wheel_transition`;
  - path 6: `run.budget_eval`.

  It also covers the Sec 5 chaos trigger: child exit is detected on the process handle, not on EOF.
- **Key detail:** All instrumentation stays on std threads. `tracing` and `tracing-subscriber` have no Tokio dependency, so viola-pty, viola-channel, viola-state and viola-agent-claude pass the `cargo deny` Tokio ban (Sec 5 Vector 9). vt100 has no tracing support, so gate outcomes such as `input-not-ready` are manual events. The PTY byte stream is never a log field.
- **Source:** https://crates.io/api/v1/crates/portable-pty/0.8.1/dependencies ; https://docs.rs/crate/tracing/latest

### cli (`viola hook <event>` / `hook statusline`): tracing-subscriber 0.3.23 with stderr-proof configuration + `std::panic::catch_unwind`

- **Version:** tracing-subscriber 0.3.23 and tracing 0.1.44; `std::panic::catch_unwind` (std)
- **Last release:** 2026-03-13
- **Status:** actively maintained (tokio-rs/tracing)
- **Agent-readable:** yes. It writes JSON-per-line to `diagnostics/hook-<name>.ndjson`. Concurrent hook processes share that file, and each line is one `write_all` on an append handle. Configuration:
  - `.log_internal_errors(false)` is mandatory;
  - `.with_writer(append_file_or_sink)`;
  - if the directory or file cannot be opened, install `std::io::sink`, so the hook stays silent and exits 0.
- **Fits because:** It covers the Sec 2 surface "cli (`viola hook`)" and two Sec 4 paths:
  - path 4: `hook.handle › channel.request(hook.dialog) › … › hook.decision_emit`;
  - path 6: `hook.statusline › state.budget_write › statusline.shell_out`.

  It also covers two Sec 5 triggers:
  - Vector 4: fail-open paths are logged only to `hook-<name>.ndjson`, with exit 0 and empty stderr;
  - error-budget: a panic produces exactly one `event:"panic"` line.
- **Key detail:** The tracing-subscriber default **`log_internal_errors(true)` falls back to `eprintln` when the writer fails**. That is the only way a correctly configured file subscriber can still write to stderr, so it must be disabled for the hook role. Init costs one `open` and one append, which fits the `max < 1.0 s` hyperfine gate (Sec 5 perf-budget).
- **Source:** https://docs.rs/tracing-subscriber/0.3.23/tracing_subscriber/fmt/struct.Layer.html

### ipc-internal (`viola-channel` JSON-RPC over interprocess local sockets): manual tracing 0.1.44 spans + `corr` field propagation

- **Version:** tracing 0.1.44; interprocess 2.4.4, which emits no `log` and no `tracing` (checked on crates.io)
- **Last release:** 2025-12-18
- **Status:** actively maintained (tokio-rs/tracing)
- **Agent-readable:** yes. It writes `channel-request` / `channel-response` JSON lines into each side's process log, joined by `corr`. Configuration: a `channel.request` span on the client and a server dispatch span. Each records:
  - `method`;
  - `corr`, the JSON-RPC `id`;
  - the result class (`ok` / `refusal` / `error` code);
  - the `sender` version.

  It **never** records `params` or result bodies.
- **Fits because:** It covers the Sec 2 surface "ipc-internal (`viola-channel`)" and two Sec 5 triggers:
  - Vector 1: the scrub rule, `-32603` logged as the fixed "internal error", and `from` marked unauthenticated;
  - cross-surface-trace-propagation.

  No packaged library exists. There is no OTel instrumentation for interprocess or named pipes, and channel `params` declare no trace-context field.
- **Key detail:** JSON-RPC ids are monotonic **per connection**, so `corr` alone cannot join a merged log (the Sec 2 gap). No library provides a discriminator. The plan should add an extra field, for example a per-connection `conn` (pid plus a connection counter). W3C `traceparent` is the wrong fix: nothing consumes it, while the tests contract allows added fields.
- **Source:** https://crates.io/api/v1/crates/interprocess/2.4.4/dependencies ; https://docs.rs/tracing/0.1.44/tracing/attr.instrument.html

### ipc-internal (`viola mcp` stdio server): rmcp 3.4.1 native `tracing` + explicit non-stdout writer

- **Version:** rmcp 3.4.1 (MSRV 1.88; normal dependency `tracing ^0.1`)
- **Last release:** 2026-09-23
- **Status:** actively maintained (modelcontextprotocol/rust-sdk)
- **Agent-readable:** yes. rmcp's internal transport and handler events, plus viola's per-tool spans (`send · wait · last · answer · list`), go to `diagnostics/mcp.ndjson` as JSON-per-line when `VIOLA_DIR` is set. Otherwise they go to stderr, still as JSON-per-line (arch). Configuration: always set `.with_writer(file_or_stderr)`, never the default writer.
- **Fits because:** It covers the Sec 2 surface "ipc-internal (`viola mcp`)" and Sec 5 Vector 5 (rejected tool calls are logged with codes only). The community pattern for rmcp stdio servers is `tracing_subscriber::fmt().with_writer(std::io::stderr)`, because "if it goes to stdout, it must be JSON-RPC".
- **Key detail:** rmcp's own `tracing` events carry its crate targets and lack the tests' required fields (`event`, `corr`, `process`, `instance`). Filter them to `rmcp=warn` with config-sourced directives, not `RUST_LOG` (Sec 3). rmcp's debug and trace events can include MCP frame content, such as send text and `last_assistant_message`.
- **Source:** https://docs.rs/crate/rmcp/latest ; https://dev.to/ejb503/from-println-disasters-to-production-building-mcp-servers-in-rust-imf

### web-spa backend (`viola ui`, axum 0.8.9): tower-http 0.7.1 `TraceLayer` with a custom `make_span_with`

- **Version:** tower-http 0.7.1 (feature `trace`; MSRV 1.65)
- **Last release:** 2026-08-31
- **Status:** actively maintained (tower-rs/tower-http)
- **Agent-readable:** yes. It writes `http-request` JSON-per-line entries to `diagnostics/ui-<port>.ndjson`. Configuration:
  `TraceLayer::new_for_http().make_span_with(|req| info_span!("ui.http_request", method=%req.method(), path=%req.uri().path())).on_response(|res, latency, _| event!(… event="http-request", status=res.status().as_u16(), duration_ms=latency.as_millis(), problem=…))`
- **Fits because:** It covers the Sec 2 surface "web-spa (`viola ui`)" and Sec 4 path 7, with a `ui.http_request` span for each of `/`, `/api/sessions`, `/api/links`, `/ready` and `/health`. It also covers Sec 5 Vector 3: the path attribute is scrubbed to `uri.path()` and `Cookie` is excluded. Tokio is allowed in viola-ui.
- **Key detail:** **`DefaultMakeSpan` records `uri = %request.uri()`, which is the full URI including the query string.** On `GET /?t=<token>` that writes the GUI token into the log, which breaks the security anti-pattern and fails the tests secret-scan test.
  - **Never use `DefaultMakeSpan`, and never call `.include_headers(true)`** (that would log `Cookie`).
  - **SSE lifecycle.** TraceLayer's response hooks miss the `/api/events` lifecycle (open › `Last-Event-ID` resume › keep-alive › close with `close_cause`), because the response counts as complete once the headers are sent. Use a manual span with a `Drop` guard inside the stream.
  - **axum's own events.** axum's optional `tracing` feature emits `axum::rejection` events at TRACE. Keep the `axum` target at `WARN+`.
  - **Rejected: axum-tracing-opentelemetry 0.39.1** (2026-08-30):
    - its **MSRV of 1.91 is above the workspace `rust-version = "1.89"`**, and arch Stack says "The obs stack must compile at MSRV 1.89";
    - its main value is extracting and injecting W3C `traceparent`, which the browser never sends (Sec 3: "Browser → ui: no propagation header in v1");
    - it needs the deferred OTel SDK and an exporter;
    - its optional `metrics-prometheus` feature adds a `GET /metrics` route, which would need a Security Decisions Log entry.
- **Source:** https://docs.rs/tower-http/0.7.1/src/tower_http/trace/make_span.rs.html ; https://docs.rs/tower-http/latest/tower_http/trace/struct.DefaultMakeSpan.html ; https://oneuptime.com/blog/post/2026-02-06-instrument-rust-axum-opentelemetry/view ; https://docs.rs/crate/axum-tracing-opentelemetry/latest

### web-spa frontend (embedded Lit 3.3.3): web-vitals 6.2.2 (vendored) + native Performance API, read into DOM attributes

- **Version:** web-vitals 6.2.2 (ships `dist/web-vitals.js` as ESM and `dist/web-vitals.iife.js`)
- **Last release:** 2026-09-14
- **Status:** actively maintained (GoogleChrome/web-vitals; repo pushed 2026-09-14)
- **Agent-readable:** yes. It produces a paste-friendly snapshot through the headless browser, with no network egress:
  - DOM state words and `data-*` attributes on the `viola-*` elements: `TAPE connecting|live|stopped`, readback `open|read back|unable|unconfirmable`, and `data-vital-*`;
  - `performance.getEntriesByType('resource'|'navigation')`.

  Playwright reads these through `page.evaluate` and serialises them to JSON. Configuration: vendor the ESM file into `/assets/`, embedded with `include_bytes!` like the Lit assets.
- **Fits because:** Sec 1 marks the frontend as "boundary-only (in v1)". The Sec 2 web-spa surface has no egress: routes are GET-only, a new listener is banned, and the CSP is `default-src 'none'; script-src 'self'`. The design-excerpt hooks are measurable from resource timing plus the server-side `http-request` `duration_ms` lines (Sec 4 path 7):
  - the TTI proxy (page load → first `/api/sessions`);
  - the SSE connecting → open span.
- **Key detail:** **@opentelemetry/sdk-trace-web 2.11.0** (2026-08-31; depends on `@opentelemetry/core` and `sdk-trace-base` 2.11.0; peer `@opentelemetry/api >=1.0.0 <1.10.0`) is not admissible in v1:
  - its only export path is an OTLP POST, while every non-GET request returns 405 and no ingest listener is allowed;
  - it needs a bundler, while Lit is vendored with **no JS build step**.

  This answers the Sec 2 question of whether a browser SDK path is admissible: none is in v1. Revisit only with the v1.x authenticated remote view and a Decisions Log entry. web-vitals is **optional**; add it only if a tests perf gate needs Core Web Vitals.
- **Source:** https://registry.npmjs.org/web-vitals/latest ; https://www.npmjs.com/package/web-vitals ; https://registry.npmjs.org/@opentelemetry/sdk-trace-web/latest ; https://github.com/open-telemetry/opentelemetry-js/releases

## CI Integration Pattern

### actions/upload-artifact v7.0.1 (captures diagnostics and harness `logs` output)

- **Version:** v7.0.1
- **Last release:** 2026-04-10
- **Status:** actively maintained (actions/upload-artifact; not archived)
- **Agent-readable:** yes. It uploads JSON-per-line files as a paste-friendly artifact: the per-OS `agent-run logs` ndjson, the `agent-run status` JSON, and the raw `diagnostics/*.ndjson`. Configuration: `- if: failure()` (or `always()`) `uses: actions/upload-artifact@<SHA of v7.0.1>` with `path: target/e2e-home/**/diagnostics/*.ndjson`, `name: diag-${{ matrix.os }}` and `retention-days: 7`.
- **Fits because:** It covers the Sec 5 multi-platform-exporter-compat trigger: the same file sink and schema across windows-2025, macos-latest and ubuntu-latest in the arch's single `ci.yml` matrix. The tests "no flakes" rule means a failing job's `diagnostics/` must be retrievable without a re-run. No OTLP collector job is needed.
- **Key detail:**
  - Pin the action by commit SHA; the tests plan already requires SHA-pinned Actions and zizmor.
  - Set retention below the 90-day default.
  - Run the secret-scan test **before** the upload.
- **Source:** https://github.com/actions/upload-artifact ; https://oneuptime.com/blog/post/2026-01-25-github-actions-artifacts/view

### cargo-nextest 0.9.146 (JUnit XML test output)

- **Version:** cargo-nextest 0.9.146
- **Last release:** 2026-09-21
- **Status:** actively maintained (nextest-rs/nextest)
- **Agent-readable:** yes. It produces JUnit XML (`[profile.ci.junit] path = "junit.xml"` in `.config/nextest.toml`). Its libtest-json output is experimental and "not currently full-fidelity", so prefer JUnit.
- **Fits because:** The tests plan already runs `cargo llvm-cov nextest` with `retries = 0`. JUnit plus the `diagnostics/` artifact gives an agent the failing test ID and its correlated `event` lines (Sec 5 multi-platform-exporter-compat).
- **Key detail:** Upload `target/nextest/ci/junit.xml` alongside the diagnostics artifact.
- **Source:** https://nexte.st/docs/machine-readable/junit/ ; https://nexte.st/docs/machine-readable/libtest-json/

## Service Identity Convention

### Rust/Cargo 1.95.0 compile-time `env!("CARGO_PKG_VERSION")` + fixed `"viola"` name (arch-mandated)

- **Version:** Rust/Cargo 1.95.0 (host toolchain per arch; workspace `rust-version = "1.89"`; `version.workspace = true`)
- **Last release:** 2026-04-16
- **Status:** actively maintained (rust-lang/rust, six-week release train)
- **Agent-readable:** yes, as a JSON-per-line field. `service.version` is the same value already carried as channel `sender`, snapshot `writer`, and `/health` / `/api/info` `version`. Add it as an extra `version` field on `process-start` only. Configuration: `pub const VERSION: &str = env!("CARGO_PKG_VERSION");` in viola-core.
- **Fits because:** It follows the Sec 2 and Sec 3 service identity:
  - `service.name` is fixed as `"viola"`;
  - `service.version` is set at compile time;
  - `deployment.environment` is N/A (local-only).

  Resolving these at runtime from env vars (`OTEL_SERVICE_NAME`, `SERVICE_NAME`) is **excluded**, because arch says env vars are not a configuration channel. OTel semconv requires only `service.name`.
- **Key detail:** `env!` resolves per crate. Every crate should inherit `version.workspace = true` and read the single viola-core constant, or `sender`, `writer` and log lines can drift apart. If an OTel Resource is ever built, use `Resource::builder_empty()`, because the default `EnvResourceDetector` reads `OTEL_*` env vars.
- **Source:** https://github.com/rust-lang/rust/releases/tag/1.95.0 ; https://doc.rust-lang.org/cargo/reference/environment-variables.html ; https://opentelemetry.io/docs/specs/semconv/resource/service/

## Network OTLP Exporter

### tracing-subscriber 0.3.23 JSON file sink + harness `agent-run logs` (fills the exporter role in v1)

- **Version:** tracing-subscriber 0.3.23 (`json`), consumed by `scripts/agent-run.{sh,ps1} logs`
- **Last release:** 2026-03-13
- **Status:** actively maintained (tokio-rs/tracing)
- **Agent-readable:** yes. It produces JSON-per-line structured logs. The harness `logs` command merges `events.ndjson` (`src:"events"`, byte `offset`) with `diagnostics/*.ndjson` (`src:"diag"`) and writes ndjson to stdout for `jq` or pasting. Configuration: the file sink above.
- **Fits because:** Per the Sec 6 tier text, a Minimal-tier shape replaces the Standard network exporter default. The reasons:
  - security says "viola makes no outbound network calls";
  - a local receiver would need a Security Decisions Log entry;
  - stdout is reserved on `hook`, `mcp` and `run`.

  The Sec 3 paste-to-AI path is the harness `logs` / `status` commands.
- **Key detail:** **opentelemetry-otlp 0.33.0** (2026-09-18, MSRV 1.75; exporters RC) is not admissible:
  - **its default features include `reqwest-blocking-client`, and `reqwest::blocking` spawns an internal Tokio current-thread runtime**;
  - `grpc-tonic`, `hyper-client` and `reqwest-client` pull in `tokio` directly, which breaks the `cargo deny` Tokio ban if it reaches a sync crate;
  - its endpoint is configured through the `OTEL_EXPORTER_OTLP_ENDPOINT` env var (`:4318` for HTTP, `:4317` for gRPC), which arch forbids.

  If it is ever admitted, use only this shape:
  - `default-features = false, features = ["http-json", "trace", "logs"]`;
  - `ui` and `mcp` roles only;
  - endpoint `127.0.0.1:4318` set in code;
  - `Resource::builder_empty()`;
  - the JSON files remain the primary sink.
- **Source:** https://docs.rs/tracing-subscriber/0.3.23/tracing_subscriber/fmt/format/struct.Json.html ; https://docs.rs/crate/opentelemetry-otlp/latest/features ; https://docs.rs/reqwest/latest/reqwest/blocking/index.html ; https://github.com/seanmonstar/reqwest/issues/1233

## Error Reporting Platform

### thiserror 2.0.20 typed errors + std panic hook → one-line `event:"panic"` records (local error capture)

- **Version:** thiserror 2.0.20 (MSRV 1.71; in Stack), with `std::panic::set_hook` / `catch_unwind`, and anyhow 1.0.104 at the bin edge only
- **Last release:** 2026-08-08
- **Status:** actively maintained (dtolnay/thiserror)
- **Agent-readable:** yes. Errors land in machine-readable form in two places:
  - JSON-per-line records in `diagnostics/*.ndjson`: `level:"ERROR", event:"panic"`, or `process-exit` with `exit_code` and `detail`;
  - the CLI `--json` document `{"v":1,"error":…}`.

  Configuration: the panic hook installed first in `main`, plus fixed-message `Display` impls on `PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `UiError` and `CoreError`.
- **Fits because:** It covers two Sec 5 triggers:
  - error-budget-SLO: zero unlogged panics, one single-line record each, and `hook` still exits 0;
  - Vector 6: an anyhow chain with a serde source is replaced by a fixed message before it reaches stderr, and the full chain goes only to `diagnostics/`.

  Security fixes "Error reporting integration: none".
- **Key detail:** **sentry 0.49.3 / sentry-tracing 0.49.3** (2026-09-21, MSRV 1.88) has a `before_send` scrubbing hook but is not admissible:
  - the egress ban applies;
  - its **default transport is `reqwest` + `native-tls` + `tokio ^1.44`**. `native-tls` links OpenSSL on Linux, which hits the cargo-deny ban on C-building crates (`openssl-sys`), and Tokio hits the ban on Tokio in sync crates.

  The `ureq` + `rustls` transport avoids both dependency bans but not the egress ban. Any future reporter must use `ureq`+`rustls`, apply the NEVER-log floor in `before_send`, be opt-in and off by default, and never be enabled by an env var.
- **Source:** https://github.com/dtolnay/thiserror ; https://doc.rust-lang.org/std/panic/fn.set_hook.html ; https://docs.rs/crate/sentry/latest ; https://crates.io/api/v1/crates/sentry/0.49.3/dependencies

## Frontend Telemetry

### web-vitals 6.2.2 (optional, vendored; DOM-attribute sink only)

- **Version:** 6.2.2 (ships `dist/web-vitals.js` as ESM and `dist/web-vitals.iife.js`)
- **Last release:** 2026-09-14
- **Status:** actively maintained (GoogleChrome/web-vitals)
- **Agent-readable:** yes. It produces a paste-friendly snapshot through the headless browser. The `onLCP`, `onINP`, `onCLS` and `onTTFB` callbacks write `data-vital-*` attributes on `<viola-atis>`, and Playwright reads them and serialises them to JSON. Nothing goes over the network. Configuration: vendor the file into `/assets/` (same-origin, which satisfies `script-src 'self'`), then `import {onLCP,onINP} from '/assets/web-vitals.js'`.
- **Fits because:** It matches the design-excerpt candidate "web-vitals … vendored and same-origin" (Sec 2 web-spa) and satisfies both the CSP and the no-egress rule. It is **optional**: native `performance` entries plus the server `http-request` lines already cover the Sec 4 path 7 TTI proxy and the SSE open span.
- **Key detail:** Setting `dataset` attributes is compatible with `require-trusted-types-for 'script'`. **@opentelemetry/sdk-trace-web 2.11.0** (2026-08-31) is not admissible in v1: exporting needs an OTLP POST, which is ruled out by the 405 on non-GET requests and the ban on new listeners, and the SDK needs a bundler while the project has no JS build step.
- **Source:** https://registry.npmjs.org/web-vitals/latest ; https://www.npmjs.com/package/web-vitals ; https://registry.npmjs.org/@opentelemetry/sdk-trace-web/latest

## Heartbeat / Tick Pattern

### sysinfo 0.39.6 (pid + start-time liveness) + the existing 1 s heartbeat file, logging transitions only

- **Version:** sysinfo 0.39.6 (in Stack; crates.io lists `rust-version` 1.95)
- **Last release:** 2026-07-09
- **Status:** actively maintained (GuillaumeGomez/sysinfo; repo pushed 2026-09-22; not archived)
- **Agent-readable:** yes. Liveness can be queried three ways:
  - the `<instance>/heartbeat` file's mtime;
  - `liveness` in `list --json` and `/api/sessions`;
  - a JSON-per-line event written **only on a transition** (live → stale → gone).

  Configuration: the wrapper touches `heartbeat` every 1 s. Readers compare an mtime age above 5 s with the sysinfo pid + start-time check, and emit an event when the state changes.
- **Fits because:** It covers the Sec 3 heartbeat ticks and two Sec 5 triggers:
  - perf-budget: the flip happens after 5 s (4.9 s is still live, 5.1 s is stale or gone);
  - chaos: a stale heartbeat while the pid is still live.

  No 2025–2026 Rust package targets heartbeat telemetry, so a custom implementation is the norm. A log line every second would grow a file that is never rotated.
- **Key detail:** **crates.io lists `rust-version = 1.95` for sysinfo 0.39.6, above the workspace `rust-version = "1.89"`.** That is an inconsistency in the arch Stack for Phase 3 to flag: either raise the workspace MSRV or pin an older sysinfo.
  - **`ui` tick:** the `Sse::keep_alive` 15 s interval. Log SSE lifecycle events, not each keep-alive.
  - **`mcp`, `hook` and short-lived CLI verbs:** no tick.
  - **Transition event:** it needs either an extension to the `event` enum or an extra `detail` field (Sec 3 gap b).
- **Source:** https://crates.io/crates/sysinfo ; https://github.com/GuillaumeGomez/sysinfo

## Performance Budget Histograms

[trigger-driven; pulled in by perf-budget-instruments from obs-scope Sec 5; not standard for Standard but required for trigger coverage]

### hyperfine 1.20.0 (`--export-json`, `max` gate) + `duration_ms` log fields aggregated by jq

- **Version:** hyperfine 1.20.0 (MSRV 1.88; a CI tool, not a product dependency)
- **Last release:** 2025-11-18
- **Status:** actively maintained (sharkdp/hyperfine; repo pushed 2026-04-30; not archived)
- **Agent-readable:** yes. `--export-json` writes a paste-friendly JSON snapshot with per-run `times[]`, `max`, `mean` and exit codes. Configuration: `hyperfine --warmup 3 --runs 30 --export-json perf/hook-session-end.json 'viola hook session-end < fixture.json'`, then `jq '.results[0].max < 1.0'` sets the exit code.
- **Fits because:** It covers the Sec 5 perf-budget-instruments trigger ("Aggregation into p95/p99 is done by the harness or `jq` over the ndjson, not by an in-process metrics pipeline"). It also covers the tests gate "`session-end` `max < 1.0 s`". In-process OTel histograms or `metrics` 0.24.6 are **not recommended**, because there is no reader or exporter to flush them to.
- **Key detail:** hyperfine does not compare runs itself, so the `jq` threshold on `max` is the assertion that sets the exit code. In the product, quantiles come from the `duration_ms` field:
  - lines to read: `hook-decision`, `process-exit` (`process="hook"`), `channel-response` and `http-request`;
  - computation: `jq -s 'map(.duration_ms)|sort'`.

  criterion 0.8.2 (2026-02-04, MSRV 1.86) is optional for micro-benchmarks.
- **Source:** https://github.com/sharkdp/hyperfine ; https://bencher.dev/hyperfine/ ; https://crates.io/crates/criterion

## Chaos / Fault Injection Telemetry

[trigger-driven; pulled in by chaos-instrumentation from obs-scope Sec 5; not standard for Standard but required for trigger coverage]

### tracing 0.1.44 fault-class events (product-side), with faults induced externally by the tests harness

- **Version:** tracing 0.1.44, through the crate-local event macro with an extra `detail` fault code
- **Last release:** 2025-12-18
- **Status:** actively maintained (tokio-rs/tracing)
- **Agent-readable:** yes. Each fault the product detects or heals becomes one JSON-per-line record in `diagnostics/*.ndjson`, tagged with a kebab-case `detail` fault code (the counterpart of OTel's `chaos.type`) and joinable by `instance` and `offset`. Configuration: emit at the heal and detect call sites in viola-state, viola-pty and viola-channel.
- **Fits because:** It covers the Sec 5 chaos-instrumentation trigger. The fault classes are:
  - torn line healed (`file`, `offset`);
  - corrupt or unsupported-`v` snapshot → replay recovery;
  - exit detected on the process handle without EOF (ConPTY);
  - endpoint vanished during `wait` (exit 21 / `instance-unreachable`);
  - stale heartbeat while the pid is still live.

  The harness induces the faults: it kills the wrapper mid-write, truncates `events.ndjson` mid-line, corrupts `snapshot.json`, and kills the endpoint during `wait`. The emitter must be Tokio-free, and `tracing` is.
- **Key detail:** No maintained, Tokio-free failpoint crate exists for 2025–2026:
  - **fail-parallel 0.6.0** (2026-07-10) has a **required normal dependency on `tokio ^1.40`**, which violates the Tokio ban on the sync crates;
  - **fail 0.5.1** (last release 2022-10-08), **failpoints 0.2.0** (2022-01-17) and **fault-injection 1.0.10** (2023-08-05) are outside the recency window;
  - the fail-rs family is configured through the `FAILPOINTS` env var, which conflicts with arch's rule against env-var configuration.

  The fault-code values need either a Sec 3 gap-b Decisions Log entry or an extra `detail` field.
- **Source:** https://docs.rs/tracing/0.1.44/tracing/ ; https://crates.io/crates/fail-parallel ; https://github.com/tikv/fail-rs ; https://crates.io/crates/failpoints ; https://docs.rs/fault-injection

## Audit Log Retention Library

[trigger-driven; pulled in by audit-log-retention from obs-scope Sec 5; not standard for Standard but required for trigger coverage]

### tracing-subscriber 0.3.23 append-only file sink, no rotation (retention = lifetime of the session home)

- **Version:** tracing-subscriber 0.3.23 plus a std `OpenOptions::append` writer (no rotation library)
- **Last release:** 2026-03-13
- **Status:** actively maintained (tokio-rs/tracing)
- **Agent-readable:** yes. It keeps append-only JSON-per-line `diagnostics/*.ndjson` files for the lifetime of the session home, read by `agent-run logs`. Configuration: files 0600 inside 0700 directories on Unix; on Windows, the home directory's DACL.
- **Fits because:** It covers the Sec 5 audit-log-retention trigger ("No rotation of `events.ndjson` or `diagnostics/*.ndjson` in v1"). Byte offsets serve as the `wait after`, `send cursor` and `Last-Event-ID` cursors, so **`events.ndjson` must never be rotated by a generic library**. The security tier is Minimal with no compliance triggers, so neither encryption at rest nor provenance is required.
- **Key detail:** The candidates for future rotation of `diagnostics/` only:
  - **logroller 0.1.12** (2026-07-01, MSRV 1.71; depends on `chrono`, `flate2`, `regex` and `thiserror 1`; optional `xz2`). Phase 5 must check three things before adopting it:
    - that files are created 0600 (not verified in this research);
    - that `xz2` stays off (it pulls in `lzma-sys`, a C-building crate) and `flate2` keeps its pure-Rust backend;
    - that a duplicate `thiserror 1.x` alongside the workspace's `thiserror 2` is acceptable.
  - **file-rotate 0.8.0** (2025-02-27) is older.
  - **tracing-appender 0.2.5**'s `Rotation` offers HOURLY, DAILY and NEVER only, with no cap on retained files.
- **Source:** https://docs.rs/tracing-subscriber/0.3.23/src/tracing_subscriber/fmt/fmt_layer.rs.html ; https://crates.io/crates/logroller ; https://github.com/alasdairpan/logroller/ ; https://docs.rs/crate/tracing-appender/latest

## PII Scrubbing Library

[trigger-driven; pulled in by logging-sensitive triggers (security Vectors 1–9) from obs-scope Sec 5; not standard for Standard but required for trigger coverage]

### veil 0.3.0 (`#[derive(Redact)]` Debug redaction) with the `toggle` feature banned

- **Version:** 0.3.0 (depends on `once_cell` and `veil-macros =0.3.0`; one feature, `toggle`)
- **Last release:** 2025-12-22
- **Status:** actively maintained (primait/veil)
- **Agent-readable:** yes. Redacted `Debug` output stays inside the normal JSON-per-line fields. Configuration: `#[derive(Redact)] struct SendParams { #[redact] text: String, from: Option<ViolaName> }`.
- **Fits because:** It adds a second layer of protection for the Sec 5 logging-sensitive triggers:
  - Vector 1: `params` and result bodies;
  - Vector 2: send text;
  - Vector 3: the GUI token and cookie;
  - Vector 4: drift reports and tool `input`;
  - Vector 6: `CLAUDE*` values stripped under R8.

  A stray `?value` capture then prints `[REDACTED]`. Regex-based PII scrubbers do not fit, because viola's sensitive data is known by type rather than detected by pattern.
- **Key detail:** veil's non-default **`toggle` feature lets the `VEIL_DISABLE_REDACTION` env var or `veil::disable()` turn redaction off**. That violates the security anti-pattern "no env, flag or config may disable redaction". Ban it in cargo-deny: `[[bans.features]] crate = "veil" deny = ["toggle"]`.
  - **Primary enforcement stays structural:**
    - `#[instrument(skip_all)]`;
    - codes-only fields;
    - `uri.path()` only;
    - the tests secret-scan test over `diagnostics/`.
  - **Other redaction crates:** redact 0.1.11 (2025-07-16) is a lighter alternative. secrecy 0.10.3 (2024-10-09) is outside the recency window.
  - **Tier-filter note:** these categories were deliberately not researched:
    - Multi-Sink Exporter: it is Comprehensive-only, and no Sec 5 trigger maps to it.
    - Compliance Trace Fields: it requires the Hardened security tier, but the tier is Minimal with "Compliance triggers: None".
    - A telemetry-exposing MCP Server Pattern: `self-observing-inversion` is not triggered (Founder Direction 3). rmcp 3.4.1 is covered above as a product surface, and the agent-facing query surface is `scripts/agent-run.{sh,ps1} logs|status`.
- **Source:** https://docs.rs/veil/latest/veil/ ; https://github.com/primait/veil ; https://crates.io/crates/redact ; https://embarkstudios.github.io/cargo-deny/checks/bans/cfg.html
