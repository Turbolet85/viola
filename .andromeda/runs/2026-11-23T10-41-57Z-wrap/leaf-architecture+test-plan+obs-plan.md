@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=19 class=omission
leaf-text: `viola run <name> [--parent <NAME>] [--role <WORD>] -- claude [args]` — wrap a live session
source: ctr-mmtpr6ws «viola run --model <MODEL> names the model handed to the child, a model word; a value failing that shape is a usage error, exit 2; with none given the model is the config.json default, else the compiled default.»
replace-with: `viola run <name> [--parent <NAME>] [--role <WORD>] [--model <MODEL>] [--effort <EFFORT>] -- claude [args]` — wrap a live session; `--model` (a model word) and `--effort` (one of the five effort words) name the pair handed to the child as `--model <M> --effort <E>`, a bad value a usage error, exit 2, with none given the `config.json` `default_model` / `default_effort`, else the compiled default
@leaf_edit n=2 leaf=.claude/docs/commands.md line=22 class=stale
leaf-text: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` — restart a dead instance in this terminal
source: ctr-vopiabqz «viola revive takes <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>], or <name> --list, each with [--json].»
replace-with: `viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` — restart a dead instance in this terminal (each half of the pair handed is its `--model` / `--effort` override, else the recorded half while it still parses, else the start's default)
