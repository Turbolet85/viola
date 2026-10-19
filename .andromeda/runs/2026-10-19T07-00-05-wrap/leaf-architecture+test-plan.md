group: architecture+test-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:22   class: omission
  leaf-text: «exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` fails `viola_state::strict::check_path`»
  source: .andromeda/architecture.md:95 «A registered `viola mcp` that exits 1 (its home refused by `home_passes`) does not block the CLI's start: the session reaches `idle` and the CLI logs that server's connection as failed»
  replace-with: «exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` fails `viola_state::strict::check_path` (that exit does not block the CLI's start: the session reaches `idle` and the CLI logs the server's connection as failed, measured on claude 2.1.287 on the Linux dev host; its Windows leg owed to "Windows-only live measurements")»
