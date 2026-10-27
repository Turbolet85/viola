group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:25   class: omission
  leaf-text: «`Read::take(MAX_FRAME)` (16 MiB) before `read_line` / `read_to_end` on every external reader;»
  source: .andromeda/security-plan.md:142 «MCP stdin (`viola mcp`'s transport, the driver's JSON-RPC lines) — each line capped at `MAX_FRAME`, its `\n` counted, by the reader viola hands rmcp, which reads stdin lines unbounded;»
  replace-with: «`Read::take(MAX_FRAME)` (16 MiB) before `read_line` / `read_to_end` on every external reader; `viola mcp`'s stdin reaches rmcp (which reads lines unbounded) only through `viola_mcp::stdin::CappedLines` at `MAX_FRAME`, its `\n` counted: a longer line releases no byte, is refused whole, never truncated or skipped, and ends the session as a stdin close does, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line holding no byte of it;»

- leaf: .claude/rules/security.md:25   class: omission
  leaf-text: «serde_json default depth (never `unbounded_depth`).»
  source: .andromeda/security-plan.md:455 «NEVER enable serde_json's `unbounded_depth` feature; `deny.toml` bans it (§Dependency Security).»
  replace-with: «serde_json default depth (never `unbounded_depth`; `deny.toml` bans the feature, its firing proven by `scripts/deny-probes.sh`).»
