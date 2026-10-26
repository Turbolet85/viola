group: obs-plan
read: .claude/docs/obs-summary.md · 76 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »

- leaf: .claude/docs/obs-summary.md:67   class: omission
  leaf-text: «exit 21 `instance-dead|strict-modes-failed|server-verify-failed`).»
  source: .andromeda/obs-plan-detail/log-coverage.md:29 «`server-verify-failed` is written by the six CLI frame verbs on a server that is not the recorded wrapper, or a check that cannot be made (Linux and Windows)»
  replace-with: «exit 21 `instance-dead|strict-modes-failed|server-verify-failed`; `server-verify-failed` on Linux and Windows only: the six CLI frame verbs write `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN into `cli-<name>.ndjson` with no frame sent, `viola hook` fails open with `hook-decision{detail:"server-verify-failed"}`, MCP returns `instance-unreachable` with no line, no line holds a pid, start time, path or endpoint, and macOS compares no peer pid (the recorded gap)).»
