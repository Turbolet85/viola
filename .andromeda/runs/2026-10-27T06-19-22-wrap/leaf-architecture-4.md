group: architecture-4
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»
read: .claude/rules/frontend.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:25   class: omission
  leaf-text: «`viola mcp` writes only MCP frames, and exits 1 with nothing on stdout or stderr when its home or `diagnostics/` fails `check_path`»
  source: .andromeda/architecture-detail/occupied-resources.md:29 «it serves the five tools over stdio until stdin closes or a stdin line passes `MAX_FRAME`, which ends the session as a close does (exit 0 once initialized, 11 before `initialize`)»
  replace-with: «`viola mcp` writes only MCP frames, ends its session at a stdin line past `MAX_FRAME` as at a closed stdin, writing nothing of its own (exit 0 once initialized, 11 before `initialize`), and exits 1 with nothing on stdout or stderr when its home or `diagnostics/` fails `check_path`»
