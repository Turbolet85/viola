group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «(under `--json` the `instance-unreachable` document, `detail: null`), `hook` exits 0 with no body, MCP returns `instance-unreachable` with `detail: null`»
  source: .andromeda/security-plan-detail/authentication-authorization.md:27 «under `--json` the `instance-unreachable` document, `detail` `strict-modes-failed`); `viola hook` exits 0 with no body; MCP returns `instance-unreachable` with `detail` `strict-modes-failed`»
  replace-with: «(under `--json` the `instance-unreachable` document, `detail` `strict-modes-failed`), `hook` exits 0 with no body, MCP returns `instance-unreachable` with `detail` `strict-modes-failed`»
- leaf: .claude/rules/security.md:12   class: stale
  leaf-text: «the CLI frame verbs exit 21 `instance-unreachable`, MCP returns `instance-unreachable` with `detail: null` and writes no line»
  source: .andromeda/security-plan-detail/authentication-authorization.md:27 «MCP returns `instance-unreachable` with `detail` `server-verify-failed` and writes no line»
  replace-with: «the CLI frame verbs exit 21 `instance-unreachable` (under `--json`, `detail` `server-verify-failed`), MCP returns `instance-unreachable` with `detail` `server-verify-failed` and writes no line»
- leaf: .claude/rules/security.md:12   class: omission
  leaf-text: «an `endpoint` present; else exit 21)»
  source: .andromeda/security-plan-detail/authentication-authorization.md:73 «none is exit 21 through `Unreachable::without_endpoint`, `instance-dead` with the not-running hint or `unwrapped-name` with the unwrapped-name hint»
  replace-with: «an `endpoint` present; else exit 21, `viola_state::liveness::without_endpoint`'s `symlink_metadata`, which opens nothing and writes no line, reading `instance-dead` for an instance directory and `unwrapped-name` for anything else)»
- leaf: .claude/rules/security.md:12   class: omission
  leaf-text: «`viola_state::liveness::live_endpoint` (else `instance-unreachable`)»
  source: .andromeda/security-plan-detail/authentication-authorization.md:27 «else `{"error":"instance-unreachable","detail":"<cause>"}`, the cause `without_endpoint`'s reading, `instance-dead` or `unwrapped-name`»
  replace-with: «`viola_state::liveness::live_endpoint` (else `instance-unreachable` with `detail` `without_endpoint`'s reading, `instance-dead` or `unwrapped-name`)»
- leaf: .claude/rules/security.md:26   class: stale
  leaf-text: «a refused strict-modes check or a preflight refusal prints exactly `{"v":1,"error":"<code>","detail":null}` on stdout (stderr empty, exit 1), `<code>` one of the closed `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`,»
  source: .andromeda/security-plan-detail/input-validation.md:39 «`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`»
  replace-with: «a refused strict-modes check or any refusal of the start it shares with `run` prints exactly `{"v":1,"error":"<code>","detail":null}` on stdout (stderr empty, exit 1), `<code>` one of the closed `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`,»
- leaf: .claude/rules/security.md:26   class: retired
  leaf-text: «; a refusal `run`'s start makes after a passed preflight prints no document (owed to the route entry "Exit-cause code catalogue")»
  source: .andromeda/security-plan-detail/input-validation.md:40 «an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error` and returns the same error»
  replace-with: «; an anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints `internal-error` and returns the same error, and a `pump_child` error prints no document»
- leaf: .claude/rules/security.md:40   class: omission
  leaf-text: «MCP `isError` results carry codes only (`refusal`/`detail`, or `instance-unreachable`, `wrapper-fault`, `invalid-params`)»
  source: .andromeda/security-plan-detail/error-handling.md:23 «Exit 21's `detail` is a closed set of five causes, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`»
  replace-with: «MCP `isError` results carry codes only (`refusal`/`detail`, or `instance-unreachable` with `detail` its closed cause, `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, as in CLI `--json`'s exit-21 document, or `wrapper-fault`, `invalid-params`)»
- leaf: .claude/rules/security.md:41   class: omission
  leaf-text: «Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves.»
  source: .andromeda/security-plan-detail/error-handling.md:28 «`{"v":1,"error":"internal-error","detail":null}` prints on stdout in place of `error: internal error`, stderr empty; `run`, `revive`'s start, `mcp` and `hook` stay silent there»
  replace-with: «Full detail goes only to `instances/<name>/diagnostics/detail-*.ndjson` (0600) when an instance resolves. Where the `cli` catch site prints its internal-error line and `json_asked` reads `--json` among the arguments before a `--`, `{"v":1,"error":"internal-error","detail":null}` prints on stdout in place of `error: internal error`, stderr empty; `run`, `revive`'s start, `mcp` and `hook` stay silent there.»
- leaf: .claude/docs/security-summary.md:65   class: stale
  leaf-text: «a preflight refusal under `--json` is the same document with its closed code (`already-live` · `no-session` · `session-live` · `cwd-missing`)»
  source: .andromeda/security-plan-detail/input-validation.md:39 «`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`»
  replace-with: «any refusal of the start it shares with `run` under `--json` is the same document with its closed code (`already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`)»
