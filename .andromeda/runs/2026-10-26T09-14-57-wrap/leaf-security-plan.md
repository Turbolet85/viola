group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/security.md:12   class: omission
  leaf-text: «the CLI frame verbs exit 21 `instance-unreachable`, MCP returns»
  source: .andromeda/security-plan-detail/authentication-authorization.md:27 «the CLI frame verbs exit 21 `instance-unreachable` (the CLI refusal, below) and write one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` WARN line»
  replace-with: «the CLI frame verbs exit 21 `instance-unreachable` and write one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` WARN line, MCP returns»
