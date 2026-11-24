@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=91
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: no `viola ui` subcommand: viola has no web or graphical frontend;
source: ctr-3g45airy «contract/cli-verbs/open · CLI verbs · open — `viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>] [--workspace N] [--model M] [--effort E] [--json] -- claude`.»
replace-with: `viola open <name> [--parent <name>] [--role overseer|builder|worker] [--cwd <dir>] [--workspace N] [--model M] [--effort E] [--json] -- claude` — open a session in its own terminal window (refused like `run` when the name is `live` or `stale`); exit 0 once the session went live (the human line `<name>  live  <placement>`, under `--json` `{"v":1,"ok":{"name","parent","window","placement"}}`, window `{address, workspace}` or null), 1 for a refusal (`strict-modes-failed` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `terminal-not-found` · `not-live` · `internal-error`, under `--json` the `{"v":1,"error":"<code>","detail":null}` document), 2 for a usage error (a clap error, any word after `--` but `claude` alone, or a parent equal to the opened name). No `viola ui` subcommand: viola has no web or graphical frontend;
