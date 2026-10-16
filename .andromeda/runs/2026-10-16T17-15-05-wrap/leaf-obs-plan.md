group: obs-plan
read: .claude/docs/obs-summary.md · 77 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »

- leaf: .claude/docs/obs-summary.md:17   class: retired
  leaf-text: «`<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson` (`ui-<port>` reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg")»
  source: .andromeda/registries/contracts/obs-plan/log-file-location.md:3 «- **Path:** `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`.»
  replace-with: «`<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`»

- leaf: .claude/docs/obs-summary.md:18   class: retired
  leaf-text: «`process` (`run|hook|mcp|ui|cli`; `ui` reserved: no emitter in 0.1.0)»
  source: .andromeda/registries/contracts/obs-plan/log-format-json-schema.md:20 «- `process` (`run|hook|mcp|cli`; `cli` is a short-lived verb with a resolved instance, writing `cli-<name>.ndjson`, obs-plan D-06) and `instance` (a `ViolaName` or null).»
  replace-with: «`process` (`run|hook|mcp|cli`)»

- leaf: .claude/docs/obs-summary.md:19   class: retired
  leaf-text: «`liveness-changed` on live↔stale↔gone transitions (flip at > 5 s) is reserved: no emitter in 0.1.0»
  source: .andromeda/registries/contracts/obs-plan/heartbeat-ticks.md:9 «- No process-log line carries a liveness transition: no process watches another instance's heartbeat.»
  replace-with: «no process logs a liveness transition (flip at > 5 s, read by `list`)»

- leaf: .claude/docs/obs-summary.md:29   class: stale
  leaf-text: «(token, `Cookie`, `?t=`, `CLAUDE*`, canary in home-level files)»
  source: .andromeda/obs-plan.md:880 «5. **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.»
  replace-with: «(`Cookie` header, `CLAUDE*` values, canary in home-level files)»

- leaf: .claude/docs/obs-summary.md:42   class: retired
  leaf-text: «· `http-request` · `panic` · `liveness-changed` · `state-recovered` · `sse-opened` · `sse-closed` · `parse-rejected` (`http-request`, `liveness-changed`, `sse-opened` and `sse-closed` reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg").»
  source: .andromeda/registries/contracts/obs-plan/log-format-json-schema.md:17 «- `process-start`, `process-exit`, `panic`: `corr` = null»
  replace-with: «· `panic` · `state-recovered` · `parse-rejected`.»

- leaf: .claude/docs/obs-summary.md:42   class: retired
  leaf-text: «`a11y-violation` is a harness-only row, not in this enum.»
  source: .andromeda/obs-plan.md:898 «as measured at chunk 2026-09-24-diagnostics-plane (`tests/contract_diag_schema.rs`). No config key, flag or env var can widen the allow-list or disable redaction (security Anti-Patterns).»
  replace-with: «»

- leaf: .claude/docs/obs-summary.md:48   class: retired
  leaf-text: «- No HTTP layer in 0.1.0: `http-request` is reserved, with no emitter (removed with the code by "Remove the browser leg")»
  source: .andromeda/registries/contracts/obs-plan/bootstrap-phases-derive-for-route-setup-project.md:25 «- **pii-scrubbing-wire:** merge with security's `logging-redaction-wire` bootstrap phase:»
  replace-with: «»
