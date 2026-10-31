group: obs-plan
read: .claude/docs/obs-summary.md · 76 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
- leaf: .claude/docs/obs-summary.md:29   class: omission
  leaf-text: «`secret-scan` step before any diagnostics-bearing upload»
  source: .andromeda/obs-plan-detail/ci-integration.md:65 «`v1-47`'s secret-scan reading is after each job's end-to-end run, not after every test: `AGENT_RUN_KEEP_HOMES` is `1` on the job»
  replace-with: «`secret-scan` step before any diagnostics-bearing upload, read after each job's end-to-end run and not after every test (`v1-47`'s reading, PROVISIONAL)»
- leaf: .claude/docs/obs-summary.md:42   class: omission
  leaf-text: «`channel-response` (corr = JSON-RPC id)»
  source: .andromeda/obs-plan-detail/log-coverage.md:13 «`detail` (`unsupported-version` / `invalid-params` / `release-from-driver`, closed by `schemas/diag-line.v1.json`: the three `-32602` faults»
  replace-with: «`channel-response` (corr = JSON-RPC id; `detail` only on a `-32602`, on both sides: `unsupported-version|invalid-params|release-from-driver`, closed by `schemas/diag-line.v1.json`; a refusal's own `detail` stays on `send-refused`, and the word `release-from-driver` here does not replace the event)»
- leaf: .claude/docs/obs-summary.md:47   class: omission
  leaf-text: «Default-deny: only fields in the obs-plan §6 catalog; veil `#[derive(Redact)]` on payload types (`toggle` banned)»
  source: .andromeda/obs-plan-detail/error-capture-reporting.md:48 «`viola_channel::sanitised_fault` is the one reader of a peer's fault, for `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`).»
  replace-with: «Default-deny: only fields in the obs-plan §6 catalog; veil `#[derive(Redact)]` on payload types (`toggle` banned); a peer's fault reads only through `viola_channel::sanitised_fault` and CLI `reply_of` / MCP `of_reply` screen a reply by `viola_core::is_code_word`, adding no span and no line»
- leaf: .claude/docs/obs-summary.md:49   class: omission
  leaf-text: «Merged phase with security's `logging-redaction-wire`»
  source: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md:41 «`product_error_writers_are_each_named` holds every product error writer to a name, and `thiserror_messages_interpolate_nothing` holds `thiserror` messages free of interpolation.»
  replace-with: «Merged phase with security's `logging-redaction-wire`; `tests/contract_error_bodies.rs` holds each error body of CLI `--json`, MCP results and the seven channel faults to a writer and a witness that stand, every product error writer to a name and `thiserror` messages free of interpolation»
- leaf: .claude/docs/obs-summary.md:67   class: omission
  leaf-text: «batch-script-child|internal-error`»
  source: .andromeda/obs-plan-detail/error-capture-reporting.md:18 «the `mcp` role writes no self `process-exit` line: `internal_error_exit_line` (`src/obs.rs`) writes for `Run` and `Cli` only.»
  replace-with: «batch-script-child|internal-error` (`internal-error` on `run` and `cli` only: the `mcp` role writes no self `process-exit` line)»
- leaf: .claude/docs/obs-summary.md:70   class: omission
  leaf-text: «(holding neither name; a bad name or pair is `-32602`, no line)»
  source: .andromeda/obs-plan-detail/span-trace-coverage.md:196 «holds neither the `driver` nor the `driven` name nor any param (on a `-32602` the `channel-response` also carries `detail:"invalid-params"`)»
  replace-with: «(holding neither name nor any param; a bad name or pair is `-32602`, no line of its own, its `channel-response` also carrying `detail:"invalid-params"`)»
