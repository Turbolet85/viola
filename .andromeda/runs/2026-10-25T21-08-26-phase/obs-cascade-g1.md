# obs · cascade read · group 1 of 5

## Carried
- (.andromeda/obs-plan-detail/obs-scope-summary.md:28) The edited `refuse_squatted` start refusal and any new `run` error path must turn an anyhow chain into a fixed message before stderr, with the full chain going only to `diagnostics/`.
  > - **Reason if not fully instrumentable:** N/A. This is the only crate that uses anyhow, so it is the error-capture boundary. Arch Stack says "context chains only at dispatch". Security Vector 6 says a chain that contains a serde source must become a fixed message before it reaches stderr. The full chain goes to `diagnostics/` only.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:27)
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:47)
  > - **Instrumentability:** instrumentable — the only crate that uses anyhow, so the error-capture boundary: a chain that contains a serde source becomes a fixed message before it reaches stderr, and the full chain goes to `diagnostics/` only.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:36) The `detail` and `refusal` values the chunk re-asserts (`squatted-name`, the budget-paused refusal) must come from viola-core's closed kebab-case types, and no log line may be emitted from viola-core.
  > - **Reason if not fully instrumentable:** It is a pure type crate with no I/O and no runtime behaviour of its own. It supplies the kebab-case values used as log fields (`refusal`, `detail`, event kinds) but emits nothing. Excluded from sections 2 to 4.
  (.andromeda/obs-plan.md:51)
  > - **Instrumentability:** not-instrumentable — a pure type crate with no I/O that supplies log-field values but emits nothing; excluded from sections 2 to 4.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:52) Any instrumentation the chunk adds to `viola-channel`'s Windows listener (`server.rs`, `server/win.rs`, `endpoint.rs`) must be sync, keeping Tokio out of the crate's normal graph.
  > - **Reason if not fully instrumentable:** N/A. Stack forbids Tokio in this crate's graph unless its feature is enabled, so instrumentation must be sync.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:51)
  > - **Instrumentability:** instrumentable
  (.andromeda/obs-plan.md:59)
  > - **Instrumentability:** instrumentable, and the instrumentation must be sync: Tokio is forbidden in this crate's graph unless its feature is enabled.
  (.andromeda/obs-plan.md:225)
  > - **Required telemetry signal:** obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:123) A remote-client or first-instance refusal of the named-pipe bind must be observed at viola's bind call site as a viola line, never through the OS or `interprocess` internals.
  > - **Reason if not fully instrumentable:** Instrument viola's call sites (bind, connect, lock, rename, kill-fallback use), not the OS internals.
  (.andromeda/obs-plan-detail/obs-scope-summary.md:122)
  > - **Instrumentability:** boundary-only
  (.andromeda/obs-plan.md:91)
  > - **Instrumentability:** boundary-only — viola's call sites, not the OS internals.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:131) The pipe name recorded as `snapshot.json`'s `endpoint` must stay out of the squat refusal's stderr pair, the exit-1 `detail` and any `ChannelError` text; it may appear only in `diagnostics/`.
  > - **Reason if not fully instrumentable:** N/A. Scrub rules per store (security Data Classifications):
  >   - `events.ndjson` is itself the product audit stream (brief §7, "the event log as the audit trail"). Its payload content is never copied into process-log lines.
  >   - Absolute paths in `snapshot.json` (`endpoint`, `pinned_bin`) and in config values are scrubbed from external output. They are allowed in `diagnostics/`.
  (.andromeda/obs-plan.md:95)
  > the absolute paths in `snapshot.json` and config values are scrubbed from external output.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:156) The CARRY case_4 capture must read lines produced by the `tracing` facade and the tracing-subscriber JSON logger as configured, and a panic in the new tests must leave the per-process hook's one line.
  > - **Backend instrumentation hooks:** Rust `tracing` facade with `#[tracing::instrument]` spans on the dispatch functions, plus a tracing-subscriber JSON structured logger (tests excerpt, Log format: `tracing-subscriber 0.3.23 fmt().json().flatten_event(true).with_current_span(false)`). Each process gets its own panic hook: a `std::panic::set_hook` replacement that writes one line. No HTTP framework is involved, so there is no auto-instrumentation.
  (.andromeda/obs-plan.md:135)
  > - **Logging stack:** `tracing` with tracing-subscriber 0.3.23, `fmt().json().flatten_event(true).with_current_span(false)`; one file per process role under `diagnostics/`, no stdout sink on any role, stderr JSON only for `mcp` when `mcp.ndjson` cannot be opened; each line one `write` on an append-mode handle; the level filter is never read from `RUST_LOG`, and no verbosity control may bypass redaction; JSON-to-file is the only mode.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:160) The squatted-endpoint refusal must keep fixed stderr strings with no path or pid, carry its exit code as a log field, and keep its own distinct exit-1 `detail` code.
  > - **Notes:**
  >   - The instance comes from `VIOLA_NAME` or an argument, never from the working directory (arch Instance identity rule).
  >   - Refusal and fault lines on stderr are fixed strings with no paths, pids, chains or upstream text (design excerpt, CLI refusals / CLI faults).
  >   - Exit codes 0, 1, 2, 10–14, 20 and 21 are the machine-readable outcome and should be a log field (arch Standard Contracts, CLI `--json` + exit codes).
  >   - Founder Direction 4: each cause behind exit 21 and exit 1 gets a distinct detail code.
  (.andromeda/obs-plan.md:161)
  > - **Required log fields:** exit-1 causes use a distinct `detail` code, with no path or pid in it.
  (.andromeda/obs-plan.md:253)
  > - **Required telemetry signal:** `release-from-driver` is its own logged event, and each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:170) The start sequence up to the channel bind and the sync channel server the chunk asserts are the wrapper's `tracing` span sites, and any span added there must be sync with no Tokio.
  > - **Backend instrumentation hooks:** `tracing` spans (`#[tracing::instrument]`) on the start sequence, the `viola-pty` seam operations, the `viola-channel` sync server handlers, vt100 readiness-gate outcomes and `viola-state` writes, all written through the tracing-subscriber JSON logger. Instrumentation must be sync: std threads, no Tokio (arch Stack, Hybrid concurrency; `cargo deny` ban).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:172) The CARRY case_4 capture must find the `send-refused` line in the `run` file sink and must make nothing visible on the terminal, and the Windows-only assertions keep the same sink and schema as every other OS.
  > - **Exporter:** A file sink only, to `diagnostics/run-<name>.ndjson` (tests excerpt, Log format). The terminal is forbidden: "`run` writes nothing to the terminal while the child runs" (arch Observability Hints).
  (.andromeda/obs-plan.md:115)
  > - **Exporter:** a file sink only, to `diagnostics/run-<name>.ndjson`; `run` writes nothing to the terminal while the child runs, so the default panic hook is replaced before the child spawns; instrumentation is sync, with no Tokio; the child's `process-exit` comes from the handle wait, never from reader EOF.
  (.andromeda/obs-plan.md:195)
  > - **Required telemetry signal:** `send` text never reaches stderr, `--json`, MCP or any process-log field outside `instances/<name>/diagnostics/`, and each deciding side's `send-refused` carries no text.
  (.andromeda/obs-plan.md:239)
  > - **Required telemetry signal:** the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:173) The `process-exit` and `send-refused` lines the chunk re-reads must carry `process="run"` and the instance's `ViolaName`.
  > - **Service identity:** as for the short-lived verbs, with `process="run"` and `instance=<ViolaName>`.
  (.andromeda/obs-plan.md:133)
  > Per line, role and instance are carried by the tests-required fields `process` (`run|hook|mcp|cli`) and `instance` (`ViolaName` or null), matching `sender` / `writer` (arch Obs-Relevant Conventions, Service identity).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:174) A panic in `run`'s start or in the new listener tests must reach the replaced panic hook, which must be installed before the child spawns, so nothing reaches the terminal and G2 reads its one line.
  > - **Notes:**
  >   - Rust's default panic hook prints to stderr, which is the child's terminal. It must be replaced before the child spawns.
- (.andromeda/obs-plan-detail/obs-scope-summary.md:196) Any span the listener work adds on the channel server must be a `tracing` span correlated through `corr`, with no trace-context field added to channel `params`.
  > - **Backend instrumentation hooks:** language-native `tracing` spans on client call and server dispatch. Propagation uses the `corr` field in log lines (JSON-RPC `id` for `channel-request` / `channel-response`; `dialog_id`; send `cursor`), per the tests excerpt Log format. Channel `params` declare no trace-context field (arch Standard Contracts, `send`: "No trace-context field is declared"). Readers skip unknown fields (arch Mixed-version tolerance), so an additive envelope field would be wire-compatible, but none is required by upstream.
  (.andromeda/obs-plan.md:264)
  > Until a wrap brings a difference current, §3, §6 and §12 win where they differ: diagnostics root (D-08, D-22), `event` enum extensions (D-01…D-05, D-21), `hook-invoked` `corr` timing (D-07), the channel join key (D-25: `(conn, corr)`, each side logs its own `instance`), and the MSRV floor (D-22: 1.96).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:198) A listener or bind failure on the server side must be logged in the wrapper's own file, `run-<name>.ndjson`, never in a client's.
  > - **Exporter:** The log file of whichever process is on each side (`run-<name>`, `hook-<name>`, `mcp`, or none for short-lived CLI verbs without an instance).
  (.andromeda/obs-plan-detail/obs-scope-summary.md:199)
  > - **Service identity:** Each side logs its own `process`. The peer version is available as `params.sender` (`CARGO_PKG_VERSION`).
- (.andromeda/obs-plan-detail/obs-scope-summary.md:200) Any new or changed `ChannelError` on the listener path must keep a `-32603` reply exactly `"internal error"` with `data:null`, and no channel line may log `from` as authenticated or carry `params` or result bodies.
  > - **Notes:**
  >   - JSON-RPC ids are "integers, monotonic per connection" (arch Standard Contracts, JSON-RPC frame shapes). `corr` for `channel-request` / `channel-response` is therefore not unique across connections or processes, and a merged-log join needs `process`, `instance` and a connection discriminator. This is a gap for Phase 3; the tests contract allows additive fields.
  >   - `-32603` must be `"internal error"` with `data:null` (security Vector 1).
  >   - `from` is self-reported and must not be logged as an authenticated identity (security Vector 1).
  (.andromeda/obs-plan.md:123)
  > `-32603` is `"internal error"` with `data:null`, and the self-reported `from` is never logged as an authenticated identity.
  (.andromeda/obs-plan.md:190)
  > - **Required telemetry signal:** channel lines never carry `params` or result bodies, and `from` is logged as unauthenticated.
- (.andromeda/obs-plan.md:311) The squatted-endpoint start refusal the chunk edits must keep writing exactly two fixed stderr lines through `src/human.rs` `refuse`, with no path or pid, before any child spawns, and nothing else of its own on the terminal.
  > with one exception before any child is spawned: a start refusal (`.cmd`/`.bat` child, live or stale name, tampered pinned copy, squatted endpoint) writes exactly two fixed stderr lines (e.g. `unable: <name>'s command is a .cmd or .bat script` / `hint: pass the real executable, not a .cmd or .bat shim`, no path or pid) through `src/human.rs` `refuse` → `write_refusal`, one `write_all` on the locked stderr, a closed pipe swallowed
- (.andromeda/obs-plan.md:312) The squat refusal's process line and the CARRY case's `send-refused` line must stay codes-only in the home-level `run` role file, with any content-bearing detail only in the instance's `detail-<role>.ndjson`.
  > - Every role's codes-only process log goes to its home-level role file under `diagnostics/`, and content-bearing detail goes only to the instance's `diagnostics/detail-<role>.ndjson`.
- (.andromeda/obs-plan.md:12) The remote-client and first-instance outcomes and the case_4 line must be readable by an agent from the log files, never from a human-only view.
  > _**Agent-driven invariant:** every telemetry signal described in this
  > plan must be readable by an agent from a snapshot or pasted log
  > excerpt.

## Bindings
- **obs-scope-summary (OS APIs boundary-only) ↔ span-trace / log coverage (the viola-seam line)**: because the named pipe is instrumented only at viola's bind call site, a remote-client or first-instance refusal reaches the log only as a viola line in the wrapper's own file, built from a fixed-text error. (.andromeda/obs-plan-detail/obs-scope-summary.md:123) ↔ (.andromeda/obs-plan-detail/obs-scope-summary.md:198)
- **obs-scope-summary (root bin anyhow boundary, store scrub) ↔ error-capture (fixed `Display`, `-32603`)**: the squat refusal's stderr pair and any `ChannelError` carry no pipe name, path or pid, the full chain going to `diagnostics/` only. (.andromeda/obs-plan-detail/obs-scope-summary.md:28) ↔ (.andromeda/obs-plan-detail/obs-scope-summary.md:131) ↔ (.andromeda/obs-plan-detail/obs-scope-summary.md:200)
- **obs-scope-summary (exit-1 detail code) ↔ log coverage (the closed `detail` catalogue) ↔ the body's harness contract**: the `squatted-name` cause keeps its distinct exit-1 `detail` while its two fixed stderr lines stay unchanged. (.andromeda/obs-plan-detail/obs-scope-summary.md:160) ↔ (.andromeda/obs-plan.md:161) ↔ (.andromeda/obs-plan.md:311)
- **obs-scope-summary (run file sink, logging stack) ↔ SLO / CI (zero flakiness, `RUST_LOG` never read)**: the case_4 capture is fixed through the `run` file sink and the configured JSON logger, with no stdout or terminal sink and no env-read level. (.andromeda/obs-plan-detail/obs-scope-summary.md:172) ↔ (.andromeda/obs-plan-detail/obs-scope-summary.md:156) ↔ (.andromeda/obs-plan.md:135)
- **obs-scope-summary (panic hook) ↔ metric / CI (G2 zero panics on the Windows leg)**: a panic in the new listener tests or `run`'s start must leave the replaced hook's one line, so G2 counts it. (.andromeda/obs-plan-detail/obs-scope-summary.md:174) ↔ (.andromeda/obs-plan-detail/obs-scope-summary.md:156)
- **obs-scope-summary (sync `viola-channel`) ↔ security plan (dependency bans, `cargo deny`)**: listener instrumentation adds no Tokio to the crate's normal graph and no C-building crate. (.andromeda/obs-plan-detail/obs-scope-summary.md:52) ↔ (.andromeda/obs-plan.md:225)
