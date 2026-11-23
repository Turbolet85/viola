@task_prose part=session
Every wrapped start now names a model and an effort: viola run and viola revive take --model <MODEL> and --effort <EFFORT>, each missing half taking the start's default (config.json default_model and default_effort, else the compiled DEFAULT_PAIR), and the pair is handed to the child after the plugin flag.
A --model or --effort among the words after the dashes is refused as a usage error, clap's exit 2, one fixed stderr line naming the two flags and echoing none of the caller's words; revive refuses the same way, and its --model and --effort conflict with --list.
The pair is recorded per instance as a new log-only event kind launch (kinds 16 to 17, start records 3 to 4) and an optional snapshot field launch, recovered by the replay, and re-handed on revive: the override half, else the recorded half while it re-parses, else the default half.
The list --json and MCP list wrapped item gains model and effort after role, each kept only when it re-parses; the six-column human board is unchanged.
Measured and disproved: a revive over a corrupt (replayed) snapshot cannot hand the pair, since revive refuses cwd-missing over every replayed snapshot, a standing behaviour left unmoved; what holds is that the replay recovers the pair and list --json reads it.
Nothing retired; no exit code, hint, IPC method, env var or verb added; the pair is handed, never claimed to be run on by a real claude session.
@task_file n=1 path=src/run/launch.rs
what: edited (new): the start's launch pair — the compiled default, the config.json default read, the resolve of each missing half, and the check that finds a pair in the words after the dashes with its fixed usage line
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited: the Claude child argv literals — the --model and --effort flags, the five effort words, the default pair and the launch argv
@task_file n=3 path=crates/viola-core/src/lib.rs
what: edited: the shared core words — the model word's closed shape and the normalised event kinds, gaining launch
@task_file n=4 path=crates/viola-state/src/snapshot.rs
what: edited: the instance snapshot, gaining the optional launch pair
@task_file n=5 path=crates/viola-state/src/replay.rs
what: edited: the snapshot replay from the events log, gaining the launch pair's last-line-wins rule
@task_file n=6 path=crates/viola-state/src/board.rs
what: edited: the list item read for viola list --json and the MCP list tool, gaining model and effort after role
@task_file n=7 path=crates/viola-state/tests/state_replay.rs
what: edited: the replay checks, gaining a cut snapshot replayed with the logged launch pair
@task_file n=8 path=crates/viola-state/tests/state_events.rs
what: edited: the event-kind table check, 16 to 17 kinds
@task_file n=9 path=src/run/mod.rs
what: edited: the wrapper's child launch — the child argv with the plugin flag, then the pair, then the caller's words
@task_file n=10 path=src/cmd/run.rs
what: edited: the viola run verb — its arguments, the --model and --effort value parsers, the refusal of a pair after the dashes, and the start's state writes gaining the launch line
@task_file n=11 path=src/cmd/revive.rs
what: edited: the viola revive verb — its usage gaining --model and --effort, the refusal of a pair after the dashes, and the pair handed to the resumed child
@task_file n=12 path=src/cmd/list.rs
what: edited: the viola list verb's --json document, carrying the launch pair
@task_file n=13 path=src/obs.rs
what: edited: the config.json known keys, 5 to 7
@task_file n=14 path=schemas/config.v1.json
what: edited: the config.json schema, gaining default_model and default_effort
@task_file n=15 path=src/bin/viola-fake-agent.rs
what: edited: the fake agent, taking --model and --effort and its start receipt gaining model and effort
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited: the fake agent's option checks, gaining model and effort
@task_file n=17 path=crates/viola-e2e/src/harness/boot.rs
what: edited: the harness boot readiness, reading four start records with launch third
@task_file n=18 path=tests/cli_instance_state.rs
what: edited: the instance state checks, reading the launch line and snapshot field
@task_file n=19 path=tests/hook_events.rs
what: edited: the hook event checks, their start-record count moved by one
@task_file n=20 path=tests/path1_run_start.rs
what: edited: the viola run start checks, gaining the default pair and the given pair handed
@task_file n=21 path=tests/cli_revive.rs
what: edited: the viola revive checks, gaining the recorded pair, the override and a recorded word that no longer parses
@task_file n=22 path=tests/chaos_snapshot_replay.rs
what: edited: the killed-wrapper replay checks, gaining the launch pair read by list --json
@task_file n=23 path=tests/cli_list.rs
what: edited: the viola list checks, gaining model and effort in the --json item
@task_file n=24 path=tests/run_cli.rs
what: edited: the viola run usage checks, gaining a bad effort and a pair after the dashes refused
@task_file n=25 path=tests/cli_controls_not_disableable.rs
what: edited: the check that no setting disables a control, its settings 9 to 11
@task_file n=26 path=tests/contract_bounded_readers.rs
what: edited: the bounded readers list, gaining config.json default_model and default_effort
@task_file n=27 path=crates/viola-state/src/liveness.rs
what: edited: the liveness reader, its snapshot literal gaining an empty launch
@task_file n=28 path=crates/viola-state/src/strict.rs
what: edited: the strict-modes reader, its snapshot literal gaining an empty launch
@task_file n=29 path=crates/viola-mcp/tests/handlers.rs
what: edited: the MCP tool handler checks, their snapshot literal gaining an empty launch
@task_file n=30 path=src/cmd/hook.rs
what: edited: the hook verb, its snapshot literals gaining an empty launch
@task_file n=31 path=src/run/dialog.rs
what: edited: the wrapper's dialog handling, its snapshot literal gaining an empty launch
@task_file n=32 path=src/run/wheel.rs
what: edited: the wrapper's wheel, its snapshot literal gaining an empty launch
@task_file n=33 path=src/run/send/slot.rs
what: edited: the wrapper's send slot, its snapshot literal gaining an empty launch
@task_file n=34 path=src/human/board/test_support.rs
what: edited: the human board's test support, its list item literal gaining model and effort
@task_file n=35 path=tests/hook_fail_open.rs
what: edited: the hook fail-open checks, their snapshot literal gaining an empty launch
@task_file n=36 path=tests/security_negatives_server_identity.rs
what: edited: the server identity negative checks, their snapshot literal gaining an empty launch
@task_file n=37 path=tests/cli_fake_agent.rs
what: edited: the fake agent CLI checks, the start receipt's keys gaining model and effort
@task_item n=1 part=touches
contract/viola-revive/usage is stale: revive's usage gains [--model <M>] [--effort <E>], the two conflicting with --list.
@task_item n=2 part=touches
contract/cli-verbs/run: viola run <name> -- claude now takes --model <MODEL> and --effort <EFFORT> before the dashes, and a pair among the words after the dashes is a usage refusal.
@task_item n=3 part=touches
contract/cli-verbs/revive and contract/cli-verbs/revive-list: revive hands the recorded pair, or an override half, to the resumed child; the override is refused together with --list.
@task_item n=4 part=touches
contract/viola-verb-help/usage: the per-verb usage of run and revive now shows --model and --effort.
@task_item n=5 part=touches
contract/viola-list-json/document: the wrapped item gains model and effort after role, each kept only when it re-parses.
@task_item n=6 part=touches
contract/mcp-tools/list: the MCP list item gains model and effort after role, as list --json does.
@task_item n=7 part=touches
decision/json-on-every-data-verb and contract/cli-json-mode/one-document: the list --json document carries two more item keys; still one document.
@task_item n=8 part=touches
decision/clap-derive-cli-parsing: the new flags are clap value parsers, and the pair-after-dashes refusal is a clap ArgumentConflict, exit 2.
@task_item n=9 part=touches
decision/hint-for-every-cause, decision/hint-last-on-stderr and contract/cli-refusal-line/form: the new usage refusal is clap's exit 2 with one fixed stderr line naming the two flags, no hint and no new exit code; read whether these refusal rules except usage errors.
@task_item n=10 part=touches
decision/hints-never-quote-or-locate: the fixed pair-after-dashes line echoes none of the caller's words, in line with the rule.
@task_item n=11 part=touches
contract/viola-revive/preflight-order: revive's pair-after-dashes refusal is a usage check before any preflight step.
@task_item n=12 part=touches
contract/viola-revive-json/cwd-missing and contract/viola-revive-refusals/directory-missing: revive over a replayed snapshot stays refused cwd-missing; the planned revive over a corrupt snapshot handing the pair was disproved on this, the rows unmoved.
@task_item n=13 part=touches
contract/viola-run/start-output and contract/viola-revive/start-output: the launch line writes no terminal byte, so the passed start output names no pair; read whether the rows stay.
@task_item n=14 part=touches
contract/viola-list/columns and the viola list board rows: the pair is no board column, the six columns unchanged; read whether a row must say the pair is json-only.
@task_item n=15 part=touches
contract/cli-global-flags/home: the start now reads default_model and default_effort from config.json under the viola home; a bad value takes that half of the compiled default.
@task_item n=16 part=touches
decision/cli-holds-every-action: choosing the model and effort is a flag of run and revive plus two config.json keys; no MCP tool sets the pair.
@task_fact n=1 slug=surface-cli
fact: viola run takes --model <MODEL> and --effort <EFFORT> before the dashes; each half not given takes the start's default, config.json default_model or default_effort, else the compiled DEFAULT_PAIR, and the pair is handed to the child.
@task_fact n=2 slug=surface-cli
fact: a --model or --effort, or its equals form, among the words after the dashes of viola run or viola revive is a usage refusal, exit 2, one fixed stderr line naming --model <MODEL> and --effort <EFFORT> and echoing none of the caller's words.
@task_fact n=3 slug=surface-cli
fact: viola revive takes --model and --effort, conflicting with --list; each half handed is the override, else the recorded half while it re-parses, else the start's default half.
@task_fact n=4 slug=surface-cli
fact: a model word is 1 to 64 bytes, a lowercase letter first then a-z 0-9 dot or dash; an effort is one of the five words of claude --help; any other value is clap's usage refusal, exit 2.
@task_fact n=5 slug=surface-cli
fact: the wrapped item of viola list --json and of the MCP list tool carries model and effort after role, each only when it re-parses; the human board shows neither.
@task_item n=1 part=untouched
viola send output, the readback mirror and the send refusals: no send behaviour moved.
@task_item n=2 part=untouched
viola wait and viola answer output, dialog and turn-ended rows: unchanged.
@task_item n=3 part=untouched
viola verify output and refusals: no verify step moved.
@task_item n=4 part=untouched
viola plugin install output and refusals: unchanged.
@task_item n=5 part=untouched
viola link and unlink output, the LINKS block and the fleet tree: unchanged.
@task_item n=6 part=untouched
CLI token rendering, SGR styling, colour and the no-banner, no-spinner, no-pager rules: no styling moved.
@task_item n=7 part=untouched
wheel verb output for pause and release, and the exit-21 hints and detail words: unchanged.
@task_item n=8 part=untouched
the web-surface and deep-link absences: still true.
