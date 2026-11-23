@row_set n=1 row=decision/revive-resumes-in-place cell=rule seen=1 detector=none
value: viola revive appends to the gone instance's events log with the earlier bytes unchanged, replays no launch spec but the recorded model and effort, and its first record is wheel with holder driver and cause start, as every start's.
basis: fct-pblfonop
because: A revive now hands the recorded model and effort pair on the resumed child's argv, so «replays no launch» narrows to the launch spec alone.
@row_set n=2 row=decision/wrapper-events-log-only cell=rule seen=4 fact=F3 detector=D-arch-resources
value: Event kinds the wrapper appends (`wheel`, `lineage`, `launch`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.
basis: fct-ydz7awsz
because: The new wrapper-appended `launch` kind is log-only and not in the wait wake kinds, so the row's list of log-only kinds must name it.
@row_set n=3 row=decision/snapshot-replay-log-fields-only cell=rule seen=3 detector=D-arch-resources
value: A snapshot rebuilt by replay recovers only links, agent_session_id, wheel, lineage and launch from the log; a field no line gave is absent, never a default.
basis: fct-ydz7awsz
because: The replay now recovers the launch pair from the last `launch` line, so the closed list of recovered fields gains it.
@row_set n=4 row=contract/snapshot-envelope/unreadable-snapshot cell=promise seen=3 detector=D-arch-resources
value: A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `lineage`, `launch`; the replay writes no file.
basis: fct-ydz7awsz
because: The replay over an unreadable snapshot now also recovers `launch`, so the promised list of recovered fields gains it.
@row_set n=5 row=contract/instance-snapshot/data cell=promise seen=4 detector=D-arch-resources
value: `data` is `{endpoint?, pid, started_at, pinned_bin, cli_version?, cli_verified, wheel, pending_dialog?, links, child_pid?, agent_session_id?, cwd?, lineage?, launch?}`.
basis: fct-xb6c4igy
because: The snapshot gains the optional `launch` field with no `v` move, so the promised shape of `data` must carry it.
@row_set n=6 row=contract/list-payload/items cell=promise seen=3 detector=D-arch-resources
value: Each `items` entry is `{name, wrapped, liveness, status, wheel?, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?}`.
basis: fct-xb6c4igy
because: The `list --json` and MCP `list` item gains `model?` and `effort?` after `role`, so the promised item shape must carry them.
@row_set n=7 row=decision/claude-flag-literals-in-agent-crate cell=rule seen=2 detector=D-arch-resources
value: The Claude agent crate is the one product home of the --resume, --fork-session, --model and --effort flag literals, and holds the program name and the --settings flag that verify reads.
basis: fct-ydz7awsz
because: The `--model` and `--effort` literals now live in the agent crate beside `--resume` and `--fork-session`, so the row's set of flag literals it owns must name them.
@row_set n=8 row=contract/viola-cli-arguments/revive cell=promise seen=1 detector=D-arch-resources
value: viola revive takes <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>], or <name> --list, each with [--json].
basis: fct-ydz7awsz
because: `ReviveArgs` gains `--model` and `--effort`, so the promised argument shape of viola revive must carry them.
@row_set n=9 row=contract/cli-verbs/revive cell=promise seen=1 detector=D-arch-resources
value: `viola revive <name> [--id <ID>] [--fork] [--model <MODEL>] [--effort <EFFORT>] [-- <child args>]` and `viola revive <name> --list` send no channel frame; `--list` conflicts with `--id`, `--fork`, `--model`, `--effort` and the trailing arguments.
basis: fct-ydz7awsz
because: viola revive takes `--model` and `--effort`, each conflicting with `--list`, so the verb's promised form and its conflicts must carry them.
@row_set n=10 row=contract/viola-revive-exit-codes/2 cell=promise seen=1 detector=D-arch-resources
value: A clap usage error: a malformed --id, --model or --effort, or a model or effort flag in the words after the dashes, included.
basis: fct-ydz7awsz
because: revive's own check now refuses a bad pair value or a pair after `--` as clap's exit 2, so the code's named causes gain them.
@row_add n=1 row=decision/launch-pair-handed-not-verified area=established-decisions fact=F1 detector=D-arch-decisions
name: Launch pair handed, never verified
rule: Every start, run and revive, hands the claude child a model and effort pair on its command line; the pair is never read back or verified as the one the session runs on.
why: Whether a session started or resumed with the pair runs on it is unmeasured on the real CLI.
basis: fct-z6wqhaac
because: Every start now hands a model and effort pair, and the master must say it is handed only, never verified, so later work does not rely on the session running on it.
@row_add n=2 row=contract/child-argv/launch-pair area=occupied-resources fact=F2 detector=D-arch-resources
boundary: flags passed to the child
item: launch pair
promise: `--model <M> --effort <E>` follows `--plugin-dir` and precedes the caller's arguments at every start and every resume.
basis: fct-ydz7awsz
because: The child's argv now carries the pair after the plugin flag and before the caller's words, a new item of the child-argv contract.
@row_add n=3 row=contract/normalised-event-kinds/launch area=conventions detector=D-arch-resources
boundary: normalised event kinds
item: launch
promise: `launch` is a normalised event kind.
basis: fct-ydz7awsz
because: `EventKind::Launch` is a new normalised kind, and each kind is one row of this closed list.
@row_add n=4 row=contract/wrapper-event-appends/launch area=standard-contracts fact=F3 detector=D-arch-resources
boundary: wrapper event appends
item: `launch`
promise: The wrapper appends `launch` with `source: wrapper` once at every start, after `lineage`; it is log-only and never wakes `wait`.
basis: fct-ydz7awsz
because: The start state now writes a `launch` line after `lineage` at every start, a new wrapper append others read in order.
@row_add n=5 row=contract/event-data/launch area=standard-contracts fact=F4 detector=D-arch-resources
boundary: event data
item: `launch`
promise: `{model, effort}`, both strings, with `source: wrapper`.
basis: fct-ydz7awsz
because: The new `launch` kind's data keys are `model` and `effort`, a new item of the event-data contract.
@row_add n=6 row=contract/instance-snapshot/launch area=standard-contracts fact=F5 detector=D-arch-resources
boundary: instance snapshot
item: launch
promise: `launch` is `{model, effort}`, the pair the start handed, omitted when absent with no snapshot `v` move; a replay rebuilds it from the log, the last `launch` line replacing the held value and a line with a non-string member reading none.
basis: fct-xb6c4igy
because: The snapshot gains the optional `launch` field and the replay its last-line rule, a new item of the instance-snapshot contract.
@row_add n=7 row=contract/config-json/default-model area=occupied-resources fact=F6 detector=D-arch-resources
boundary: config.json
item: default_model
promise: A string of the model word shape, default the model half of DEFAULT_PAIR; a bad value takes that half with one parse-rejected config-json malformed line, and no value disables a control.
basis: fct-xb6c4igy
because: `default_model` is a new config.json key in the schema, and each key is one row of the config contract.
@row_add n=8 row=contract/config-json/default-effort area=occupied-resources fact=F6 detector=D-arch-resources
boundary: config.json
item: default_effort
promise: One of the five effort words, default the effort half of DEFAULT_PAIR; a bad value takes that half with one parse-rejected config-json malformed line, and no value disables a control.
basis: fct-xb6c4igy
because: `default_effort` is a new config.json key in the schema, and each key is one row of the config contract.
@row_add n=9 row=contract/viola-cli-arguments/run-model area=occupied-resources fact=F7 detector=D-arch-resources
boundary: viola CLI arguments
item: run --model
promise: viola run --model <MODEL> names the model handed to the child, a model word; a value failing that shape is a usage error, exit 2; with none given the model is the config.json default, else the compiled default.
basis: fct-ydz7awsz
because: `RunArgs` gains `--model`, a new CLI flag of viola run.
@row_add n=10 row=contract/viola-cli-arguments/run-effort area=occupied-resources fact=F7 detector=D-arch-resources
boundary: viola CLI arguments
item: run --effort
promise: viola run --effort <EFFORT> names the effort handed to the child, one of the five effort words; any other value is a usage error, exit 2; with none given the effort is the config.json default, else the compiled default.
basis: fct-ydz7awsz
because: `RunArgs` gains `--effort`, a new CLI flag of viola run.
@row_add n=11 row=decision/revive-pair-from-record area=occupied-resources fact=F8 detector=D-arch-decisions
name: Revive hands the recorded pair
rule: Each half of the model and effort pair a revive hands is its --model or --effort override, else the recorded half while it still parses, else the start's default.
state: provisional
basis: fct-z6wqhaac
because: viola revive now resolves the pair it hands from override, record and default, a rule a delegate's word set, so it lands provisional.
@row_add n=12 row=decision/recorded-pair-on-child-argv area=occupied-resources detector=D-arch-decisions
name: Recorded launch pair on the child's command line
rule: A model and effort pair read from the instance's record may be handed on the revived claude child's command line, each word re-parsed to its closed shape first.
why: A ratified crossing, beside the logged session id on the child's command line.
basis: fct-z6wqhaac
because: The recorded pair now crosses onto the revived child's argv, a second ratified crossing the master must hold beside the session id's.
@row_add n=13 row=decision/launch-pair-refused-in-caller-words area=conventions fact=F9 detector=none
name: Model and effort flags refused in the caller's words
rule: A model or effort flag in the caller's words after the dashes, plain or equals form, is refused on run and revive as a usage error, exit 2, with one fixed line naming the two flags and echoing none of the caller's words.
basis: fct-ydz7awsz
because: run and revive now refuse a pair after `--` with the one fixed line, a convention every start honours.
@row_add n=14 row=contract/list-item/model area=standard-contracts fact=F10 detector=D-arch-resources
boundary: list payload item
item: model
promise: Present only when the recorded model re-parses as a model word at the board; follows role.
basis: fct-ydz7awsz
because: The wrapped list item, CLI and MCP, gains `model?` after `role`, a new item key.
@row_add n=15 row=contract/list-item/effort area=standard-contracts fact=F10 detector=D-arch-resources
boundary: list payload item
item: effort
promise: Present only when the recorded effort re-parses as a code word at the board; follows model.
basis: fct-ydz7awsz
because: The wrapped list item, CLI and MCP, gains `effort?` after `model`, a new item key.
@row_add n=16 row=decision/default-launch-pair area=occupied-resources fact=F11 detector=D-arch-resources
name: Compiled default launch pair
rule: The compiled default model and effort pair is one named constant, DEFAULT_PAIR, in the Claude agent crate, and the effort words are the five of claude --help; no environment variable sets the pair.
basis: fct-ydz7awsz
because: The chunk lands `DEFAULT_PAIR` and `EFFORT_LEVELS` in the agent crate as the one source of the default pair and the effort words.
@row_add n=17 row=contract/model-word/format area=conventions detector=D-arch-resources
boundary: Model word
item: format
promise: A model word is 1 to 64 bytes, a lower-case letter first, then lower-case letters, digits, dots or hyphens.
basis: fct-ydz7awsz
because: viola-core gains the closed model word shape the run flag, the config key and the board all check against.
