group: test-plan+obs-plan
read: .claude/rules/observability.md · 47 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:16   class: stale
  leaf-text: «(`rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `notify`, `sysinfo`, `interprocess`) are OFF»
  source: .andromeda/registries/contracts/obs-plan/logging-stack.md:39 «Third-party targets (`rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `sysinfo`, `interprocess`) are `OFF` in every sink, because their records lack the required fields.»
  replace-with: «(`rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `sysinfo`, `interprocess`) are OFF»

- leaf: .claude/rules/observability.md:19   class: omission
  leaf-text: «- Codes-only role files: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`.»
  source: .andromeda/registries/contracts/obs-plan/log-file-location.md:3 «`<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson` (`ui-<port>` reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg").»
  replace-with: «- Codes-only role files: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson` (`ui-<port>` reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg").»

- leaf: .claude/rules/observability.md:32   class: retired
  leaf-text: «`instance` as explicit values into threads/tasks. The SSE stream span needs a `Drop` guard emitting `sse-closed`.»
  source: .andromeda/obs-plan.md:754 «| `sse-opened` / `sse-closed` (D-04; reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg") |»
  replace-with: «`instance` as explicit values into threads/tasks.»

- leaf: .claude/rules/observability.md:33   class: retired
  leaf-text: «- tower-http `TraceLayer` with a custom `make_span_with` recording `uri.path()` only — never `DefaultMakeSpan`, never `.include_headers(true)`.»
  source: .andromeda/obs-plan.md:225 «(`ui-<port>`, the `ui` process and the `http-request`, `liveness-changed`, `sse-opened` and `sse-closed` events below: reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg")»
  replace-with: «- The `ui` process and the `http-request`, `liveness-changed`, `sse-opened` and `sse-closed` events are reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg".»

- leaf: .claude/rules/observability.md:37   class: retired
  leaf-text: «`mcp`/`ui` main-thread → exit 1.»
  source: .andromeda/obs-plan.md:835 «A main-thread panic reaches the main-thread catch site. The process writes `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` and then exits 1.»
  replace-with: «`mcp` main-thread → `process-exit{detail:"internal-error"}`, then exit 1.»

- leaf: .claude/rules/observability.md:43   class: retired
  leaf-text: «(PTY pump, notify tail)»
  source: .andromeda/obs-plan.md:1074 «- NEVER log at `info` inside hot loops (PTY pump). Use counts on the boundary event instead.»
  replace-with: «(PTY pump)»

- leaf: .claude/rules/observability.md:43   class: retired
  leaf-text: «no line per heartbeat tick or SSE keep-alive»
  source: .andromeda/obs-plan.md:1109 «- NEVER log a line per heartbeat tick. Log transitions and lifecycle only, because the files are never rotated.»
  replace-with: «no line per heartbeat tick»
