group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:21   class: omission
  leaf-text: «`viola revive <name> --list` prints its logged sessions»
  source: .andromeda/architecture.md:301 «`revive` takes `<name> [--id <ID>] [--fork] [-- <child args>]`, or `<name> --list`, each with `[--json]`; under `--json` a refused strict-modes check or a preflight refusal is the exit-1»
  replace-with: «`viola revive <name> --list` prints its logged sessions; both arms take `[--json]`: a refused strict-modes check or a preflight refusal prints the exit-1 `{"v":1,"error":"<code>","detail":null}` document on stdout, stderr empty (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`), `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order (an empty chain the `no-session` document), and a refusal `run`'s start makes after a passed preflight prints no document»
