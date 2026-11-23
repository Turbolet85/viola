@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_edit n=1 leaf=.claude/docs/conventions.md line=40 class=stale
leaf-text: `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` and `viola revive <name> --list` send no channel frame;
source: ctr-zecfbixa «`viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` and `viola revive <name> --list` send no channel frame; `--list` conflicts with `--id`, `--fork`»
replace-with: `viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` and `viola revive <name> --list` send no channel frame (`--list` conflicts with `--id`, `--fork`, `--model`, `--effort` and the trailing arguments);
@leaf_edit n=2 leaf=.claude/docs/conventions.md line=40 class=omission
leaf-text: and `viola run --role <WORD>` its role, a code word.
source: ctr-mmtpr6ws «viola run --model <MODEL> names the model handed to the child, a model word; a value failing that shape is a usage error, exit 2; with none given the model is the config.json default, else the compiled»
replace-with: and `viola run --role <WORD>` its role, a code word; `viola run --model <MODEL>` names the model handed to the child, a model word, and `viola run --effort <EFFORT>` its effort, one of the five effort words, a value failing its shape a usage error, exit 2, and with none given each is the `config.json` default, else the compiled default.
@leaf_edit n=3 leaf=.claude/docs/conventions.md line=45 class=omission
leaf-text: any other value reads false with one `parse-rejected` config-json `malformed` line).
source: ctr-sfd57bms «A string of the model word shape, default the model half of DEFAULT_PAIR; a bad value takes that half with one parse-rejected config-json malformed line, and no value disables a control.»
replace-with: any other value reads false with one `parse-rejected` config-json `malformed` line), `default_model` (a string of the model word shape, default the model half of `DEFAULT_PAIR`) and `default_effort` (one of the five effort words, default the effort half of `DEFAULT_PAIR`), each bad value taking that half with one `parse-rejected` config-json `malformed` line, and no value disabling a control.
@leaf_edit n=4 leaf=.claude/docs/services/viola-agent-claude.md line=6 class=stale
leaf-text: it is the one product home of the two flag literals.
source: dec-wm2dwyha «The Claude agent crate is the one product home of the --resume, --fork-session, --model and --effort flag literals, and holds the program name and the --settings flag that verify reads.»
replace-with: it is the one product home of the four flag literals `--resume`, `--fork-session`, `--model` and `--effort` (`MODEL_FLAG`, `EFFORT_FLAG`), and holds `DEFAULT_PAIR`, the one compiled default model and effort pair, with the five effort words of `claude --help`; no environment variable sets the pair.
@leaf_edit n=5 leaf=.claude/docs/services/viola-core.md line=20 class=omission
leaf-text: and, last, `lineage` (`data` keys `parent`, `role`, each a string or `null`; `source: wrapper`, one line at every start, log-only);
source: ctr-xw4xo2fx «`{model, effort}`, both strings, with `source: wrapper`.»
replace-with: `lineage` (`data` keys `parent`, `role`, each a string or `null`; `source: wrapper`, one line at every start, log-only) and, last, `launch` (`data` keys `model`, `effort`, both strings; `source: wrapper`, one line at every start after `lineage`, log-only);
