group: architecture-3
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»

- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «then `viola_mcp::serve`, the five tools over stdio until stdin closes)»
  source: .andromeda/architecture-detail/occupied-resources.md:29 «it serves the five tools over stdio until stdin closes or a stdin line passes `MAX_FRAME`, which ends the session as a close does (exit 0 once initialized, 11 before `initialize`)»
  replace-with: «then `viola_mcp::serve`, the five tools over stdio until stdin closes or a stdin line passes `MAX_FRAME` (its `\n` counted; the line refused whole, never truncated or skipped), which ends the session as a close does — exit 0 once initialized, 11 before `initialize` — with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line at WARN holding no byte of it)»

- leaf: .claude/docs/stack.md:19   class: stale
  leaf-text: «`preserve_order` keeps a printed document's declared key order (e.g. `{"v":1,"cmd":…,"ok":…}`) |»
  source: .andromeda/architecture-detail/stack-and-technologies.md:20 «serde_json's `unbounded_depth` feature is banned in `deny.toml` (Build system), so every parse of external bytes keeps serde_json's default recursion limit»
  replace-with: «`preserve_order` keeps a printed document's declared key order (e.g. `{"v":1,"cmd":…,"ok":…}`); serde_json's `unbounded_depth` feature is banned in `deny.toml` (Build system), so every parse of external bytes keeps serde_json's default recursion limit |»
