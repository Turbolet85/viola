proposals:
  - detector: D-retired
    severity: warning
    section: '§4 → Scenario: `wait` / `last` event-driven readback'
    change: 'A home or instance tree the instance check refuses ends a CLI `wait` / `last` before any role file opens (exit 21, the instance pair, or exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` under `--json`; no line), and MCP returns `instance-unreachable` with `detail` `strict-modes-failed` and no line; the server-verification clause reads the same way, MCP returning `instance-unreachable` with `detail` `server-verify-failed` and no line.'
    sidecar: 'The `wait` / `last` scenario states the refused instance check as the `strict-modes-failed` exit-21 document and names the MCP cause words; `detail: null` retired (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Retired** names the exit-21 document `detail: null`; **Symbols / APIs** says the document now carries the cause and that a refused `check_instance` is `StrictModes` and a failed `server_verified` is `ServerVerify` in MCP `frame`, with no line on any of them.'
    basis: '.andromeda/obs-plan.md:361'
  - detector: D-retired
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'In the `cli` per-role anchor for `viola link` / `viola unlink`, the refusal reads: exit 21, the fixed instance-state pair, or `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` under `--json`.'
    sidecar: 'OTel SDK init: the `link` / `unlink` refused-check document carries `detail` `strict-modes-failed`; `detail: null` retired (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Retired** names the exit-21 document `detail: null`; **Symbols / APIs** says `refuse_untrusted`, the check `link` / `unlink` share with the frame verbs, writes `StrictModes` through `unreachable_document`.'
    basis: '.andromeda/registries/contracts/obs-plan/otel-sdk-init.md'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§3 → Logging stack'
    change: 'In the `run-<name>.ndjson` shared-file item, a second `viola run <name>` or a `viola revive <name>` on a live or `stale` name appends its `process-start` and its exit-1 `process-exit` next to the holder''s lines: `detail:"already-live"` for a live holder or a fresh replayed beat, `detail:"stale-heartbeat"` for a `stale` holder.'
    sidecar: 'Logging stack: the collision line of a stale holder is `stale-heartbeat`, `already-live` kept for a live holder or a fresh replayed beat (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Retired** names `already-live` as a stale holder''s refusal; **Symbols / APIs** says `collision_check` prints and logs `stale-heartbeat` for a `stale` holder and `already-live` for a live holder and a fresh replayed beat.'
    basis: '.andromeda/registries/contracts/obs-plan/logging-stack.md'
  - detector: D-retired
    severity: warning
    section: '§3 → OTel SDK init'
    change: 'In the `run` per-role anchor for `viola revive`''s start arm, the logged preflight refusals read (`already-live`, `stale-heartbeat`, `no-session`, `session-live`, `cwd-missing`).'
    sidecar: 'OTel SDK init: `stale-heartbeat` joins the preflight exit-1 refusals logged by `viola revive`''s start arm (chunk 2026-10-31-exit-cause-code-catalogue).'
    rationale: 'Report **Retired** names `already-live` as a stale holder''s refusal, and this list gave no other code for one; **Symbols / APIs** lists `stale-heartbeat` among `viola revive --json` codes and says `collision_check` logs it.'
    basis: '.andromeda/registries/contracts/obs-plan/otel-sdk-init.md'
    dependent-of: D-retired
note: 'No detector or **Retired** item covers `.andromeda/obs-plan.md:523`, so it is not a proposal, but the report (`catch-site`, **Symbols / APIs**) says it must change: under `--json` the short-lived `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of `error: internal error`, stderr then empty, and the line stands byte for byte without `--json`; the orchestrator should confirm the cascade applies it. Other stale claims found are in `.andromeda/obs-plan-detail/` (`obs-scope-summary.md`, `span-trace-coverage.md`, `log-coverage.md`, `error-capture-reporting.md`, `pii-scrubbing-compliance.md`), which are the cascade''s. D-obs-instrumentation, D-obs-stack and D-obs-pii found no drift.'
