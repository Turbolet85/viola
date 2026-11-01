group: obs-plan
read: .claude/docs/obs-summary.md · 76 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
- leaf: .claude/docs/obs-summary.md:67   class: stale
  leaf-text: «batch-script-child|internal-error»
  source: .andromeda/obs-plan-detail/log-coverage.md:26 «`internal-error`, `socket-dir-refused` (the socket directory of `viola run` or `viola revive`'s start refused or not creatable, before any snapshot, child or bind)»
  replace-with: «batch-script-child|internal-error|socket-dir-refused»
- leaf: .claude/docs/obs-summary.md:67   class: retired
  leaf-text: «the socket-directory cause has no code (an anyhow error at the catch site) and macOS has no server check, both outside the distinct-code rule with the route entry "Unix endpoint and home hardening"»
  source: .andromeda/obs-plan-detail/obs-scope-summary.md:166 «A refused socket directory has its own code, `socket-dir-refused`, and writes its line; macOS compares no peer pid, the named carve-out from this rule, owned by no route entry»
  replace-with: «a refused socket directory has its own code, `socket-dir-refused`, and writes its line; macOS compares no peer pid, the named carve-out from the distinct-code rule, owned by no route entry, and there `server-verify-failed` means an euid mismatch alone»
- leaf: .claude/docs/obs-summary.md:67   class: stale
  leaf-text: «`instance-dead|unwrapped-name|endpoint-lost|strict-modes-failed|server-verify-failed`»
  source: .andromeda/obs-plan-detail/log-coverage.md:31 «`process-exit` exit 21: `instance-dead`, `unwrapped-name`, `endpoint-lost`, `strict-modes-failed`, `server-verify-failed`, `socket-dir-refused`.»
  replace-with: «`instance-dead|unwrapped-name|endpoint-lost|strict-modes-failed|server-verify-failed|socket-dir-refused`»
- leaf: .claude/docs/obs-summary.md:69   class: stale
  leaf-text: «three of the seven fail-open codes»
  source: .andromeda/obs-plan-detail/log-coverage.md:34 «`viola hook statusline` uses three of the eight (the rule: the `detail` codes the arm's fail-open paths write)»
  replace-with: «three of the eight fail-open codes»
- leaf: .claude/docs/obs-summary.md:72   class: omission
  leaf-text: «likewise opens no role file and writes no line»
  source: .andromeda/obs-plan-detail/log-coverage.md:42 «The check takes the read verdict of the strict-modes checks above yet writes lines under its own word»
  replace-with: «likewise opens no role file and writes no line; a refused socket directory, whose check takes the same read verdict, is no such exemption and writes under its own word: `viola run` and `viola revive`'s start one `process-exit{subject:"self", exit_code:1}`, the eight CLI frame verbs one `process-exit{exit_code:21, during:"connect"}` at WARN with no `channel-request`, `viola hook` one `hook-decision` at WARN, and only `viola mcp`'s `frame` no line, `isError` its record»
- leaf: .claude/docs/obs-summary.md:59   class: omission
  leaf-text: «NEVER log per heartbeat tick;»
  source: .andromeda/obs-plan.md:479 «On Unix a peer the wrapper's `accept_loop` refuses by euid is dropped on the accept thread, before its worker thread and before any frame, and writes no line by rule»
  replace-with: «NEVER log per heartbeat tick; NEVER log a peer the wrapper's `accept_loop` refuses by euid (Unix: dropped on the accept thread, before any frame, the client reading end-of-file);»
- leaf: .claude/docs/obs-summary.md:31   class: omission
  leaf-text: «else the gate passes vacuously)»
  source: .andromeda/obs-plan-detail/obs-scope-summary.md:191 «the socket-directory check and the peer-euid check on the hook `connect` sit in that timed path too»
  replace-with: «else the gate passes vacuously); the socket-directory check and the peer-euid check on the hook `connect` sit in that timed path too»
