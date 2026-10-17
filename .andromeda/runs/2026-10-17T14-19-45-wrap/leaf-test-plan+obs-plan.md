group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:19   class: omission
  leaf-text: «Codes-only role files: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`.»
  source: .andromeda/registries/contracts/obs-plan/logging-stack.md:27 «`mcp`: `<home>/diagnostics/mcp.ndjson`, with stderr JSON only when the file cannot be opened (D-09);»
  replace-with: «Codes-only role files: `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`; `mcp` always writes `mcp.ndjson` (D-09), stderr JSON only when that file cannot be opened, never stdout (it carries MCP frames).»

- leaf: .claude/rules/observability.md:39   class: omission
  leaf-text: «with no name it opens no log.»
  source: .andromeda/registries/contracts/obs-plan/logging-stack.md:10 «and from the `list` tool the `agents-probe` pair and the board's `state-recovered` / `parse-rejected` lines»
  replace-with: «with no name it opens no log. `viola mcp`'s `list` tool makes the same board read (through `list::board_document`) and writes its `agents-probe` pair, `state-recovered`, `parse-rejected{parser:"state-entry"}` and `config-json` threshold lines into `mcp.ndjson`.»

- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «or on `viola hook statusline` opens no role file and writes no line»
  source: .andromeda/obs-plan.md:989 «or `viola mcp`'s refused check of its home or `diagnostics/` through `viola_state::strict::check_path` (chunk 2026-10-17-mcp-server-for-drivers, its plan's lean, approved by the operator at P5)»
  replace-with: «or on `viola hook statusline`, or `viola mcp`'s refused `viola_state::strict::check_path` of its home or `diagnostics/` (run before its `mcp` role file opens; exit 1 with nothing on stdout or stderr; its plan's lean, approved by the operator at P5), opens no role file and writes no line»
