group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:22   class: omission
  leaf-text: «it serves until stdin closes, and exits 1»
  source: .andromeda/architecture-detail/occupied-resources.md:29 «it serves the five tools over stdio until stdin closes or a stdin line passes `MAX_FRAME`, which ends the session as a close does (exit 0 once initialized, 11 before `initialize`), and exits 1»
  replace-with: «it serves until stdin closes or a stdin line passes `MAX_FRAME` (refused whole, never truncated or skipped, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line holding no byte of it), which ends the session as a close does (exit 0 once initialized, 11 before `initialize`), and exits 1»

- leaf: .claude/docs/commands.md:60   class: stale
  leaf-text: «`deny-probes: 13/13 banned, control clean`: the twelve bans one probe each»
  source: .andromeda/test-plan-detail/ci-integration.md:15 «last line `deny-probes: 14/14 banned, control clean`: the thirteen bans one probe each (the serde_json `unbounded_depth` ban's row `serde_json-unbounded-depth` firing `error[feature-banned]`)»
  replace-with: «`deny-probes: 14/14 banned, control clean`: the thirteen bans one probe each (the serde_json `unbounded_depth` ban's row `serde_json-unbounded-depth` among them)»
