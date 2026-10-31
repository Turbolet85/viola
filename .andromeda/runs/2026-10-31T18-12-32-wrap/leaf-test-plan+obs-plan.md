group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
- leaf: .claude/rules/observability.md:21   class: stale
  leaf-text: «the fixed instance-state pair or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document»
  source: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:25 «exit 21, the fixed instance-state pair, or `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` under `--json`»
  replace-with: «the fixed instance-state pair or under `--json` the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document»
- leaf: .claude/rules/observability.md:28   class: omission
  leaf-text: «`detail` codes are closed kebab-case lists — never a path or pid.»
  source: .andromeda/obs-plan-detail/log-coverage.md:31 «`process-exit` exit 21: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`.»
  replace-with: «`detail` codes are closed kebab-case lists — never a path or pid. `schemas/diag-line.v1.json` closes `process-exit`'s at 19 values; exit 21's closed causes are `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed` (`during` `call` for `endpoint-lost` alone; `instance-dead`, `unwrapped-name` and `server-verify-failed` are `during` `connect`), and `stale-heartbeat` is exit 1's cause alone (a frame verb against a stale instance reads `instance-dead`).»
- leaf: .claude/rules/observability.md:31   class: stale
  leaf-text: «`#[instrument(skip_all, name = "<area>.<operation>", fields(..))]` only — never bare»
  source: .andromeda/obs-plan.md:327 «Every span is declared with `#[instrument(skip_all, name = "...", fields(...))]` or `info_span!` with an explicit field allow-list»
  replace-with: «`#[instrument(skip_all, name = "<area>.<operation>", fields(..))]` or `info_span!` with an explicit field allow-list only — never bare»
- leaf: .claude/rules/observability.md:36   class: stale
  leaf-text: «then exit 1 with exactly `error: internal error` (no hint)»
  source: .andromeda/obs-plan.md:523 «or under `--json` the `{"v":1,"error":"internal-error","detail":null}` document on stdout with stderr empty, and exit 1»
  replace-with: «then exit 1 with exactly `error: internal error` (no hint), or under `--json` (read from argv before any `--`) the `{"v":1,"error":"internal-error","detail":null}` document on stdout with stderr empty»
- leaf: .claude/rules/observability.md:36   class: omission
  leaf-text: «`run` → `process-exit{detail:"internal-error"}`»
  source: .andromeda/obs-plan-detail/error-capture-reporting.md:20 «an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document and returns the same error»
  replace-with: «`run` → `process-exit{detail:"internal-error"}` (the catch site prints nothing; under `viola revive --json` an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document before it returns, `pump_child`'s prints none)»
- leaf: .claude/rules/observability.md:36   class: omission
  leaf-text: «then exit 1 with nothing on stdout.»
  source: .andromeda/obs-plan-detail/error-capture-reporting.md:57 «`unreachable_document` (`src/cmd/client.rs`) is the one writer of the document, `refuse_untrusted` and `src/cmd/plugin.rs` through it»
  replace-with: «then exit 1 with nothing on stdout. Exit 21's `--json` document carries one cause word of `viola_core::UnreachableCause` in `detail`, written by the one `unreachable_document` (`src/cmd/client.rs`; `refuse_untrusted` and `src/cmd/plugin.rs` through it), and the type writes no log line of its own.»
- leaf: .claude/rules/observability.md:38   class: stale
  leaf-text: «(a twelfth `detail` value in `schemas/diag-line.v1.json`)»
  source: .andromeda/obs-plan-detail/log-coverage.md:29 «`schemas/diag-line.v1.json` closes the `process-exit` `detail` at 19 values»
  replace-with: «(a member of the `process-exit` `detail` enum `schemas/diag-line.v1.json` closes at 19 values)»
- leaf: .claude/rules/observability.md:38   class: omission
  leaf-text: «prints the same pair and exits 1 through its own `process-exit`»
  source: .andromeda/obs-plan-detail/span-trace-coverage.md:232 «`verify` maps that refusal (`StampsError::StrictModes`) to the same home pair, and under a parsed `VIOLA_NAME` its `process-exit` carries `detail:"strict-modes-failed"`.»
  replace-with: «prints the same pair and exits 1 through its own `process-exit`, which under a parsed `VIOLA_NAME` carries `detail:"strict-modes-failed"`; every other refusal past the tree check writes its own code there (`batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `unclean-fixture`), a failing row is no refusal and carries no code, and the fixture's `<file>` enters no code»
- leaf: .claude/rules/observability.md:40   class: stale
  leaf-text: «`already-live`, `no-session`, `session-live` (the thirteenth `process-exit` `detail` value), `cwd-missing`»
  source: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:20 «the preflight's exit-1 refusals (`already-live`, `stale-heartbeat`, `no-session`, `session-live`, `cwd-missing`)»
  replace-with: «`already-live` (a live holder or a fresh replayed beat), `stale-heartbeat` (a stale holder), `no-session`, `session-live`, `cwd-missing`»
- leaf: .claude/rules/observability.md:40   class: stale
  leaf-text: «MCP's frame tools return `instance-unreachable` with no line, as the liveness pre-check's and as their own refused `check_instance` of the home and the instance directory (made before `live_endpoint`, `detail: null`, no frame)»
  source: .andromeda/obs-plan-detail/span-trace-coverage.md:33 «a refusal returns `instance-unreachable` with `detail` `strict-modes-failed`, no frame and no line»
  replace-with: «MCP's frame tools return `instance-unreachable` with `detail` `server-verify-failed` and no line, as the liveness pre-check's does with `without_endpoint`'s reading (`instance-dead` or `unwrapped-name`) and as their own refused `check_instance` of the home and the instance directory does (made before `live_endpoint`, `detail` `strict-modes-failed`, no frame)»
