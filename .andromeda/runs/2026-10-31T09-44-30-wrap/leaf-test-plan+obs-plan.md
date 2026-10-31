group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:36   class: stale
  leaf-text: «`mcp` main-thread → `process-exit{detail:"internal-error"}`, then exit 1»
  source: .andromeda/obs-plan.md:527 «The process writes no `process-exit` line (`obs::internal_error_exit_line` covers `Run` and `Cli` only) and exits 1.»
  replace-with: «`mcp` main-thread → no `process-exit` line (`obs::internal_error_exit_line` covers `Run` and `Cli` only), then exit 1 with nothing on stdout»

- leaf: .claude/rules/observability.md:28   class: omission
  leaf-text: «`detail` codes are closed kebab-case lists — never a path or pid.»
  source: .andromeda/obs-plan-detail/span-trace-coverage.md:93 «plus `detail` on a `-32602` only (a word of the closed `unsupported-version|invalid-params|release-from-driver`), absent on every other `channel-response` line, a refusal's included»
  replace-with: «`detail` codes are closed kebab-case lists — never a path or pid. `channel-response` carries `detail` on a `-32602` only (`unsupported-version` · `invalid-params` · `release-from-driver`, from `ProtocolError::cause`, on the server's and the client's line alike, closed by `schemas/diag-line.v1.json`) and none on any other `channel-response` line, a refusal's included, whose own `detail` stays on `send-refused`.»

- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «no pid, start time, path or endpoint in any line.»
  source: .andromeda/obs-plan-detail/error-capture-reporting.md:48 «`viola_channel::sanitised_fault` is the one reader of a peer's fault, for `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`).»
  replace-with: «no pid, start time, path or endpoint in any line. A peer's reply is screened before it reaches a document or a result, adding no span and no line: CLI `reply_of` and MCP `of_reply` read a `refusal` that is no `viola_core::is_code_word` as `unknown` and a string `detail` that is none as null, and `viola_channel::sanitised_fault` is the one reader of a peer's fault.»
