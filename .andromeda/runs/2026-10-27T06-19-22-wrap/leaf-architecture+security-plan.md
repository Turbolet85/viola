group: architecture+security-plan
read: .claude/rules/api.md · 41 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:35   class: omission
  leaf-text: «rmcp features exactly `server` + `transport-io`; every call returns»
  source: .andromeda/security-plan.md:142 «each line capped at `MAX_FRAME`, its `\n` counted, by the reader viola hands rmcp, which reads stdin lines unbounded; a longer line releases no byte, is refused whole, never truncated or skipped»
  replace-with: «rmcp features exactly `server` + `transport-io`; `viola mcp`'s stdin reaches rmcp only through the crate-private `viola_mcp::stdin::CappedLines` at `MAX_FRAME` per line, its `\n` counted (rmcp 3.4.1's stdio transport reads each line unbounded, so the bound sits in the reader viola hands rmcp, never its codec): a line of exactly the cap passes whole, a longer line releases no byte, is refused whole, never truncated or skipped, and reads as end of stream, ending the session as a closed stdin does, after which `serve` writes one WARN `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` holding no byte of it and exits 0 once initialized, or `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes; every call returns»
