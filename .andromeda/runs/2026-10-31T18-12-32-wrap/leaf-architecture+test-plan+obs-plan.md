group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»
- leaf: .claude/docs/commands.md:13   class: stale
  leaf-text: «under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document»
  source: .andromeda/architecture.md:230 «its refused strict-modes check prints the `instance-unreachable` document with `detail` `strict-modes-failed` (exit 21) through `client::unreachable_document`»
  replace-with: «under `--json` the `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` document»
- leaf: .claude/docs/commands.md:13   class: omission
  leaf-text: «1 for an internal error, 2 for a clap usage error (`viola plugin` alone)»
  source: .andromeda/architecture.md:228 «`{"v":1,"error":"internal-error","detail":null}` (exit 1): a `cli` verb at the `main` catch site with `--json` among the arguments before a `--` (`json_asked`), printed on stdout»
  replace-with: «1 for an internal error (under `--json` the `{"v":1,"error":"internal-error","detail":null}` document on stdout, stderr empty), 2 for a clap usage error (`viola plugin` alone)»
- leaf: .claude/docs/commands.md:21   class: stale
  leaf-text: «a refused strict-modes check or a preflight refusal prints the exit-1»
  source: .andromeda/architecture.md:229 «`viola revive --json` refusing its strict-modes check, its preflight or the start it shares with `run`»
  replace-with: «a refused strict-modes check, a preflight refusal or a refusal of the start it shares with `run` prints the exit-1»
- leaf: .claude/docs/commands.md:21   class: stale
  leaf-text: «(`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`)»
  source: .andromeda/architecture.md:229 «one of `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`»
  replace-with: «(`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`)»
- leaf: .claude/docs/commands.md:21   class: retired
  leaf-text: «and a refusal `run`'s start makes after a passed preflight prints no document»
  source: .andromeda/architecture.md:229 «A `pump_child` error prints no document: the child has held the terminal.»
  replace-with: «and a `pump_child` error prints no document: the child has held the terminal»
- leaf: .claude/docs/commands.md:23   class: omission
  leaf-text: «(and the same pair, exit 1, when `update_stamps` refuses `ledger/stamps.json`)»
  source: .andromeda/architecture-detail/occupied-resources.md:42 «when `update_stamps` refuses `ledger/stamps.json`, with `detail:"strict-modes-failed"` on its `process-exit` under a parsed `VIOLA_NAME`»
  replace-with: «(and the same pair, exit 1, when `update_stamps` refuses `ledger/stamps.json`, with `detail:"strict-modes-failed"` on its `process-exit` under a parsed `VIOLA_NAME`)»
- leaf: .claude/docs/commands.md:33   class: stale
  leaf-text: «`logs [--session] [--instance] [--kind] [--process]`»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:79 «[--process run|hook|mcp|cli] [--after <byte offset>]»
  replace-with: «`logs [--session] [--instance] [--kind] [--process] [--after]`»
