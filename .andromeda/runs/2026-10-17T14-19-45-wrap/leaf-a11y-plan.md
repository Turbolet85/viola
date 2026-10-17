group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/docs/a11y-summary.md:53   class: omission
  leaf-text: «a failed, oversize, malformed or timed-out read fails open, and no key is blocked, refused or delayed by it.»
  source: .andromeda/a11y-plan.md:285 «The MCP `wait` tool (chunk 2026-10-17-mcp-server-for-drivers) clamps its `timeout_ms` to `WAIT_CAP` 50 000 ms, which is also its default when the caller passes none (`viola_mcp::wait_timeout`)»
  replace-with: «a failed, oversize, malformed or timed-out read fails open, and no key is blocked, refused or delayed by it. The MCP `wait` tool (chunk 2026-10-17-mcp-server-for-drivers) clamps its `timeout_ms` to `WAIT_CAP` 50 000 ms, also its default when the caller passes none (`viola_mcp::wait_timeout`), so an MCP `wait` with no timeout is answered `timed-out` after 50 s: a bound on the driver only (the hidden, plugin-run `viola mcp` serves a driving Claude session, never a person), no key is blocked, refused or delayed by it, and the wheel is untouched.»
