@row_set n=1 row=contract/viola-revive/usage cell=promise seen=1 detector=D-design-cli-wording
value: `viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` or `viola revive <name> --list`, each with `[--json]`; `--list` conflicts with `--id`, `--fork`, `--model`, `--effort` and the trailing arguments.
basis: fct-ydz7awsz (`ReviveArgs --model` / `--effort`, conflicting with `--list`); fct-pblfonop (layout-templates · contract/viola-revive/usage)
because: viola revive gains --model and --effort, which conflict with --list, so the usage the row states no longer covers the verb's arguments.
@row_set n=2 row=decision/hint-for-every-cause cell=except seen=1 detector=D-design-cli-wording
value: No hint for the `unknown` refusal, for `wrapper fault`, for `internal error` or for the usage refusal of a `--model` / `--effort` after the dashes.
basis: fct-ydz7awsz (`checked()` refuses a pair in the program words, exit 2, the one line `PAIR_AFTER_DASHES`); fct-ydz7awsz (no new closed word, hint)
because: The chunk adds a refusal that prints one fixed stderr line and no hint, so the row's list of hintless refusals must name it.
@row_add n=1 row=contract/viola-run/launch-pair area=surface-cli fact=F1 detector=none
boundary: viola run start
item: the launch pair
promise: `viola run` takes `--model <MODEL>` and `--effort <EFFORT>` before the dashes; each half not given takes config.json `default_model` or `default_effort`, else the compiled `DEFAULT_PAIR`, and the pair is handed to the child.
basis: fct-ydz7awsz (`RunArgs --model <MODEL>` / `--effort <EFFORT>`, `start()` reads the default and resolves, `child_launch`); fct-xb6c4igy (`default_model`, `default_effort`)
because: viola run gains a launch pair at the CLI boundary, and no row of the cli surface states its flags or how a missing half is filled.
@row_add n=2 row=contract/run-revive-usage/pair-after-dashes area=surface-cli fact=F2 detector=D-design-cli-wording
boundary: viola run and viola revive usage
item: a launch pair after the dashes
promise: A `--model` or `--effort`, or its equals form, among the words after the dashes of `viola run` or `viola revive` is a usage refusal, exit 2, one fixed stderr line naming `--model <MODEL>` and `--effort <EFFORT>` and echoing none of the caller's words.
basis: fct-ydz7awsz (`checked()` refusing a pair in the program words, `ArgumentConflict`, exit 2, `PAIR_AFTER_DASHES`; revive's own `checked()` with the same line); fct-dgfm4vwr
because: The chunk adds a new CLI refusal wording with its exit code, which no master row states.
@row_add n=3 row=contract/viola-revive/launch-pair area=surface-cli fact=F3 detector=none
boundary: viola revive start
item: the launch pair
promise: Each half handed to the resumed child is the `--model` / `--effort` override, else the recorded half while it re-parses, else the start's default half.
basis: fct-ydz7awsz (`cmd::revive::given_pair`); fct-ckuwa7cb
because: viola revive now hands a launch pair to the resumed child, and no row states which half wins.
@row_add n=4 row=contract/run-revive-arguments/model area=surface-cli fact=F4 detector=none
boundary: viola run and viola revive arguments
item: --model
promise: A model word is 1 to 64 bytes, a lowercase letter first, then a-z, 0-9, dot or dash; any other value is clap's usage refusal, exit 2.
basis: fct-ydz7awsz (`is_model_word`, `parse_model`); fct-objkygma (the value parsers on run and revive)
because: The new --model flag takes a closed shape at the CLI boundary, which no row states.
@row_add n=5 row=contract/run-revive-arguments/effort area=surface-cli fact=F4 detector=none
boundary: viola run and viola revive arguments
item: --effort
promise: An effort is one of the five effort words of `claude --help`; any other value is clap's usage refusal, exit 2.
basis: fct-ydz7awsz (`EFFORT_LEVELS`, `is_effort_level`, `parse_effort`); fct-dgfm4vwr (`run_with_a_bad_effort_is_usage`)
because: The new --effort flag takes a closed list at the CLI boundary, which no row states.
@row_add n=6 row=contract/viola-list-json/launch-pair area=surface-cli fact=F5 detector=none
boundary: viola list --json
item: model and effort
promise: The wrapped item of `viola list --json` and of the MCP list tool carries `model` and `effort` after `role`, each only when it re-parses; the human board shows neither.
basis: fct-ydz7awsz (`board::Item.model?` / `effort?` after `role`, re-parsed); fct-aahvlfhn (the six columns unchanged)
because: The list item gains two fields at the --json and MCP boundary while the human board stays as it is, and no row of the cli surface states either.
