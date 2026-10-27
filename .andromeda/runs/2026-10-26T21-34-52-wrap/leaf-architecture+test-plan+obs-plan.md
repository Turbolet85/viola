group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»
- leaf: .claude/docs/commands.md:23   class: omission
  leaf-text: «exit 1 on a failing row;»
  source: .andromeda/architecture-detail/occupied-resources.md:41 «`verify` exits 1 with the home-state pair, before its `cli` log and any step line, when `check_home_tree` refuses its home, `ledger/`, `bin/` or `plugin/`, or `update_stamps` refuses»
  replace-with: «exit 1 on a failing row; exit 1 with the home-state `unable:` / `hint:` pair, no step line, no summary and no log line when `check_home_tree` refuses its home, `ledger/`, `bin/` or `plugin/` before its `cli` log opens (and the same pair, exit 1, when `update_stamps` refuses `ledger/stamps.json`);»
