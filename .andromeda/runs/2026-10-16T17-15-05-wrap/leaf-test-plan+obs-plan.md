group: test-plan+obs-plan
read: .claude/rules/observability.md · 47 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:16   class: stale
  leaf-text: «Third-party targets (`rmcp`, `axum`, `tower_http`, `hyper`, `portable_pty`, `sysinfo`, `interprocess`) are OFF»
  source: .andromeda/registries/contracts/obs-plan/logging-stack.md:38 «Third-party targets (`rmcp`, `portable_pty`, `sysinfo`, `interprocess`) are `OFF` in every sink, because their records lack the required fields.»
  replace-with: «Third-party targets (`rmcp`, `portable_pty`, `sysinfo`, `interprocess`) are OFF»

- leaf: .claude/rules/observability.md:19   class: retired
  leaf-text: «`<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson` (`ui-<port>` reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg").»
  source: .andromeda/registries/contracts/obs-plan/log-file-location.md:3 «- **Path:** `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`.»
  replace-with: «`<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`.»

- leaf: .claude/rules/observability.md:25   class: retired
  leaf-text: « `a11y-violation` is NOT a product event.»
  source: .andromeda/obs-plan.md:720 «This section only **adds** fields and the D-01, D-03, D-05 and D-06 enum values; it renames and removes nothing.»
  replace-with: «»

- leaf: .claude/rules/observability.md:33   class: retired
  leaf-text: «- The `ui` process and the `http-request`, `liveness-changed`, `sse-opened` and `sse-closed` events are reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg".»
  source: .andromeda/obs-plan.md:729 «- `process`: `run|hook|mcp` (+ `cli`, D-06)»
  replace-with: «»
