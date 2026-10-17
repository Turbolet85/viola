group: obs-plan
read: .claude/docs/obs-summary.md · 76 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »

- leaf: .claude/docs/obs-summary.md:17   class: omission
  leaf-text: «`<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`; content-bearing»
  source: .andromeda/obs-plan.md:213 «stderr JSON only for `mcp`, and only when `mcp.ndjson` cannot be opened (D-09).»
  replace-with: «`<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson` (`mcp` always writes `mcp.ndjson`, its home and `diagnostics/` checked by strict-modes before the sink opens; stderr JSON only when that file cannot be opened, D-09); content-bearing»

- leaf: .claude/docs/obs-summary.md:71   class: omission
  leaf-text: «the `agents-probe` spawn pair of its `claude agents --json` read (chunk 2026-10-16-unwrapped-sessions-on-the-board) into `cli-<name>.ndjson`.»
  source: .andromeda/obs-plan.md:656 «`viola mcp`'s `list` tool runs the same board read (through `list::board_document`, chunk 2026-10-17-mcp-server-for-drivers) and its `state-recovered` lines go into `mcp.ndjson`»
  replace-with: «the `agents-probe` spawn pair of its `claude agents --json` read (chunk 2026-10-16-unwrapped-sessions-on-the-board) into `cli-<name>.ndjson`. `viola mcp`'s `list` tool runs the same board read (through `list::board_document`, chunk 2026-10-17-mcp-server-for-drivers) and writes its `state-recovered`, `parse-rejected{parser:"state-entry"}`, per-key `config-json` and `agents-probe` lines into `mcp.ndjson`.»

- leaf: .claude/docs/obs-summary.md:72   class: omission
  leaf-text: «(bounded exemption 7, with the statusline arm's; exemption 6 retired)»
  source: .andromeda/obs-plan.md:989 «or `viola mcp`'s refused check of its home or `diagnostics/` through `viola_state::strict::check_path` (chunk 2026-10-17-mcp-server-for-drivers, its plan's lean, approved by the operator at P5)»
  replace-with: «(bounded exemption 7, with the statusline arm's, `viola list`'s refused scan and `viola mcp`'s refused check of its home or `diagnostics/`, which exits 1 with nothing on stdout or stderr; exemption 6 retired)»
