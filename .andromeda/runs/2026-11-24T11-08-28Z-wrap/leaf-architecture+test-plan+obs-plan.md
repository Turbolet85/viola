@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=22 class=omission
leaf-text: a `pump_child` error prints no document: the child has held the terminal
source: ctr-asykvpei «`viola adopt <SESSION_ID> --name <NAME> [--parent <NAME>] [--model <MODEL>] [--effort <EFFORT>] [--json]` takes the id in the closed session-id shape, hex of either case, and no words after `--`»
replace-with: a `pump_child` error prints no document: the child has held the terminal. `viola adopt <SESSION_ID> --name <NAME> [--parent <NAME>] [--model <MODEL>] [--effort <EFFORT>] [--json]` — resume a plain claude session under viola with an operator-given name, launching `claude --resume <SESSION_ID>`; the id in the closed session-id shape, hex of either case, no words after `--`; a `--parent` equal to `--name` is a usage error, exit 2
@leaf_edit n=2 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: `viola open <name> [--parent <name>] [--role overseer|builder|worker]
source: dec-3tojobmm «viola adopt resumes under viola, with an operator-given name, a plain claude session named by its session id, and refuses with session-live, exit 1, when a claude agents --json row carries that id; a failed read passes.»
replace-with: `viola adopt` is refused like `run` when the name is `live` or `stale`, and with `session-live`, exit 1, when a `claude agents --json` row carries the id (a failed read passes); a refusal exits 1, under `--json` the `{"v":1,"error":"<code>","detail":null}` document (`strict-modes-failed` · `session-live` · `already-live` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`). `viola open <name> [--parent <name>] [--role overseer|builder|worker]
