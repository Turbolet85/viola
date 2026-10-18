group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:40   class: omission
  leaf-text: «approved by the operator at P5), opens no role file and writes no line»
  source: .andromeda/obs-plan.md:990 «or `viola plugin install`'s refused check of its home or a `marketplace/` tree dir through `check_path` (chunk 2026-10-17-optional-plugin-install, its plan's lean, approved by the operator at P5)»
  replace-with: «approved by the operator at P5), or `viola plugin install`'s refused `check_path` of its home or a `marketplace/` tree dir (run before its `cli` role file opens; exit 21, the stderr pair or under `--json` the `instance-unreachable` document; its plan's lean, approved by the operator at P5), opens no role file and writes no line»

- leaf: .claude/rules/observability.md:39   class: omission
  leaf-text: «and `config-json` threshold lines into `mcp.ndjson`.»
  source: .andromeda/obs-plan.md:665 «`plugin install` writes that pair only after its strict-modes tree check has passed (`process-exit` exit 0, no detail); a refused check opens no log and writes no line (exit 21: the stderr pair, or under `--json` the `instance-unreachable` document)»
  replace-with: «and `config-json` threshold lines into `mcp.ndjson`. `viola plugin install` runs its strict-modes tree check before its log; once it passes, under a parsed `VIOLA_NAME` it writes the self pair (`process-exit` exit 0, no detail) into `cli-<name>.ndjson`, and its span `cli.plugin_install` is no dispatch span: it is on `write_tree`, opened after the `cli` log opens, so the tree check and its refusal run outside it; no path in any line.»
