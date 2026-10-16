group: obs-plan
read: .claude/docs/obs-summary.md · 77 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »

- leaf: .claude/docs/obs-summary.md:9   class: stale
  leaf-text: «four process roles, seven instrumentable crates and seven must-trace paths»
  source: .andromeda/obs-plan.md:31 «- 6 instrumentable crates;»
  replace-with: «four process roles, six instrumentable crates and six must-trace paths»

- leaf: .claude/docs/obs-summary.md:17   class: omission
  leaf-text: «`<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`»
  source: .andromeda/registries/contracts/obs-plan/log-file-location.md:3 «(`ui-<port>` reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg").»
  replace-with: «`<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson` (`ui-<port>` reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg")»

- leaf: .claude/docs/obs-summary.md:18   class: omission
  leaf-text: «`process` (`run|hook|mcp|ui|cli`)»
  source: .andromeda/registries/contracts/obs-plan/log-format-json-schema.md:8 «(`ui-<port>`, the `ui` process and the `http-request`, `liveness-changed`, `sse-opened` and `sse-closed` events below: reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg")»
  replace-with: «`process` (`run|hook|mcp|ui|cli`; `ui` reserved: no emitter in 0.1.0)»

- leaf: .claude/docs/obs-summary.md:19   class: stale
  leaf-text: «`liveness-changed` on live↔stale↔gone transitions (flip at > 5 s); SSE keep-alive every 15 s»
  source: .andromeda/registries/contracts/obs-plan/heartbeat-ticks.md:9 «Reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg".»
  replace-with: «`liveness-changed` on live↔stale↔gone transitions (flip at > 5 s) is reserved: no emitter in 0.1.0; short-lived readers (`list`) expose `liveness` in `--json`»

- leaf: .claude/docs/obs-summary.md:21   class: stale
  leaf-text: «over `viola list --json` + `/ready` + `/api/sessions`»
  source: .andromeda/registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md:5 «- `agent-run status` is the aggregate health snapshot: `state` and `last_error` closed enum.»
  replace-with: «over `viola list --json`: the aggregate health snapshot, `state` and `last_error` closed enum»

- leaf: .claude/docs/obs-summary.md:29   class: stale
  leaf-text: «listed in obs-plan §8 item 6»
  source: .andromeda/obs-plan.md:884 «5. **Verification:** the tests secret-scan test (token, `Cookie`, `?t=`, `CLAUDE*` values, 0600) over every `diagnostics/` file, run in CI before artifact upload.»
  replace-with: «listed in obs-plan §8 item 5»

- leaf: .claude/docs/obs-summary.md:37   class: stale
  leaf-text: «same value as `sender`, `writer`, `/health.version`»
  source: .andromeda/registries/contracts/obs-plan/service-identity.md:4 «so the log `version`, channel `sender` and snapshot `writer` cannot drift apart.»
  replace-with: «same value as `sender` and `writer`»

- leaf: .claude/docs/obs-summary.md:42   class: omission
  leaf-text: «`process-start` · `process-exit` · `http-request` · `panic` · `liveness-changed` · `state-recovered` · `sse-opened` · `sse-closed` · `parse-rejected`.»
  source: .andromeda/registries/contracts/obs-plan/log-format-json-schema.md:8 «(`ui-<port>`, the `ui` process and the `http-request`, `liveness-changed`, `sse-opened` and `sse-closed` events below: reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg")»
  replace-with: «`process-start` · `process-exit` · `http-request` · `panic` · `liveness-changed` · `state-recovered` · `sse-opened` · `sse-closed` · `parse-rejected` (`http-request`, `liveness-changed`, `sse-opened` and `sse-closed` reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg").»

- leaf: .claude/docs/obs-summary.md:48   class: retired
  leaf-text: «HTTP spans record `uri.path()` only; no headers»
  source: .andromeda/obs-plan.md:749 «| `http-request` (reserved: no emitter in 0.1.0; removed with the code by "Remove the browser leg") | `method`, `path`, `route`, `status`, `problem`, `duration_ms`, `skipped` |»
  replace-with: «No HTTP layer in 0.1.0: `http-request` is reserved, with no emitter (removed with the code by "Remove the browser leg")»

- leaf: .claude/docs/obs-summary.md:49   class: stale
  leaf-text: «never in the PTY pump or notify tail loops»
  source: .andromeda/obs-plan.md:1074 «- NEVER log at `info` inside hot loops (PTY pump). Use counts on the boundary event instead.»
  replace-with: «never in the PTY pump loop»

- leaf: .claude/docs/obs-summary.md:51   class: stale
  leaf-text: «nor the passwd home (obs-plan §8 item 6)»
  source: .andromeda/obs-plan.md:884 «5. **Verification:** the tests secret-scan test (token, `Cookie`, `?t=`, `CLAUDE*` values, 0600) over every `diagnostics/` file, run in CI before artifact upload.»
  replace-with: «nor the passwd home (obs-plan §8 item 5)»

- leaf: .claude/docs/obs-summary.md:52   class: stale
  leaf-text: «outside every upload path (obs-plan §8 item 6)»
  source: .andromeda/obs-plan.md:884 «5. **Verification:** the tests secret-scan test (token, `Cookie`, `?t=`, `CLAUDE*` values, 0600) over every `diagnostics/` file, run in CI before artifact upload.»
  replace-with: «outside every upload path (obs-plan §8 item 5)»

- leaf: .claude/docs/obs-summary.md:60   class: stale
  leaf-text: «NEVER log per heartbeat tick or SSE keep-alive»
  source: .andromeda/obs-plan.md:1109 «- NEVER log a line per heartbeat tick. Log transitions and lifecycle only, because the files are never rotated.»
  replace-with: «NEVER log per heartbeat tick»
