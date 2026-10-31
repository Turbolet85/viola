group: obs-plan
read: .claude/docs/obs-summary.md · 76 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
- leaf: .claude/docs/obs-summary.md:67   class: stale
  leaf-text: «exit 1 `already-live|squatted-name|pinned-hash-mismatch|batch-script-child|internal-error`»
  source: .andromeda/obs-plan-detail/log-coverage.md:26 «`stale-heartbeat` (a `stale` holder, from the same two, under `revive --json` the same-shaped document)»
  replace-with: «exit 1 `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child|internal-error`»
- leaf: .claude/docs/obs-summary.md:67   class: stale
  leaf-text: «exit 21 `instance-dead|strict-modes-failed|server-verify-failed`»
  source: .andromeda/obs-plan-detail/log-coverage.md:31 «`process-exit` exit 21: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`.»
  replace-with: «exit 21 `instance-dead|unwrapped-name|endpoint-lost|strict-modes-failed|server-verify-failed`»
- leaf: .claude/docs/obs-summary.md:67   class: stale
  leaf-text: «writes no line and carries no `detail` code»
  source: .andromeda/obs-plan-detail/telemetry-strategy.md:43 «writes no line and carries no logged `detail` code: its machine-readable form is the exit code»
  replace-with: «writes no line and carries no logged `detail` code (its machine-readable form is the exit code and, under `--json`, the `instance-unreachable` document with `detail` `strict-modes-failed` on the frame verbs and `link` / `unlink`, or `revive`'s `strict-modes-failed` document)»
- leaf: .claude/docs/obs-summary.md:67   class: omission
  leaf-text: «and `interrupted` from a `viola verify` stopped by Ctrl-C or TERM, after every spawn pair it opened is closed (chunk 2026-10-13-interrupted-verify-cleanup)»
  source: .andromeda/obs-plan-detail/log-coverage.md:30 «`batch-script-child`, `program-not-found`, `pinned-hash-mismatch`, `version-unreadable`, `strict-modes-failed`, `unclean-fixture`, beside `interrupted`.»
  replace-with: «and `interrupted` from a `viola verify` stopped by Ctrl-C or TERM, after every spawn pair it opened is closed (chunk 2026-10-13-interrupted-verify-cleanup), beside its refusals' `batch-script-child|program-not-found|pinned-hash-mismatch|version-unreadable|strict-modes-failed|unclean-fixture` on the `process-exit` it writes under a parsed `VIOLA_NAME` (a failing row is no refusal and carries no code)»
- leaf: .claude/docs/obs-summary.md:67   class: omission
  leaf-text: «exit 20 `wrapper-fault`»
  source: .andromeda/obs-plan-detail/obs-scope-summary.md:493 «the socket-directory cause has no code (`bind_endpoint`'s `create_private_dir` failure is an anyhow error at the catch site) and macOS has no server check»
  replace-with: «the socket-directory cause has no code (an anyhow error at the catch site) and macOS has no server check, both outside the distinct-code rule with the route entry "Unix endpoint and home hardening"; exit 20 `wrapper-fault`»
