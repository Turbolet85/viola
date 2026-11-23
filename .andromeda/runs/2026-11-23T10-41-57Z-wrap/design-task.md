@task_prose part=session
Every wrapped start now names a model and an effort: viola run takes --model and --effort, and where either is missing that half comes from config.json default_model and default_effort, else from viola's compiled default pair.
A model or effort word given after the -- is refused as a usage error, clap's exit 2, with one fixed stderr line that names the two flags and echoes none of the caller's words.
viola revive takes --model and --effort too, which conflict with --list; without them revive hands the child the pair it recorded, and a recorded half that no longer parses takes the default half.
The pair is recorded as a new log-only launch event line and a launch field in the instance snapshot. The list --json and MCP list item gain model and effort after role, and the human board keeps its six columns.
No new exit code, hint, closed word or verb. The planned check that a revive over a corrupt snapshot still hands the pair was disproved: revive still refuses cwd-missing over every replayed snapshot.
@task_file n=1 path=src/run/launch.rs
what: added: the start's launch pair, which reads the config.json default pair, resolves each missing half and finds a pair in the caller's words after the dashes, plus the fixed usage line that names the two flags
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited: the Claude agent crate's flag literals, which gain --model and --effort, the five effort words, the default pair and the launch argv
@task_file n=3 path=crates/viola-core/src/lib.rs
what: edited: the core event kinds and closed word shapes, which gain the model-word shape and the launch event kind
@task_file n=4 path=crates/viola-state/src/snapshot.rs
what: edited: the instance snapshot, which gains an optional launch field
@task_file n=5 path=crates/viola-state/src/replay.rs
what: edited: the snapshot replay from the events log, which gains the launch line's last-line-wins rule
@task_file n=6 path=crates/viola-state/src/board.rs
what: edited: the board read behind viola list and its --json items, which gain model and effort after role, each re-parsed before it is shown
@task_file n=7 path=crates/viola-state/tests/state_replay.rs
what: edited: the replay tests, which gain a cut snapshot replayed with the logged launch pair
@task_file n=8 path=crates/viola-state/tests/state_events.rs
what: edited: the event-kind table tests, which gain the seventeenth kind
@task_file n=9 path=src/run/mod.rs
what: edited: the wrapper's child argv builder, which hands the pair after the plugin flag and before the caller's words
@task_file n=10 path=src/cmd/run.rs
what: edited: the viola run verb, its arguments --model and --effort, its refusal of a pair after the dashes and its start records
@task_file n=11 path=src/cmd/revive.rs
what: edited: the viola revive verb, its --model and --effort override, conflicting with --list, and the recorded pair it hands
@task_file n=12 path=src/cmd/list.rs
what: edited: the viola list verb's --json document, which carries the launch pair
@task_file n=13 path=src/obs.rs
what: edited: the known config.json keys, five to seven
@task_file n=14 path=schemas/config.v1.json
what: edited: the config.json schema, which gains default_model and default_effort
@task_file n=15 path=src/bin/viola-fake-agent.rs
what: edited: the fake agent, which takes --model and --effort and reports them in its start receipt
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited: the fake agent's option tests
@task_file n=17 path=crates/viola-e2e/src/harness/boot.rs
what: edited: the harness boot readiness, which reads four start records
@task_file n=18 path=tests/cli_instance_state.rs
what: edited: the instance-state integration tests
@task_file n=19 path=tests/hook_events.rs
what: edited: the hook event integration tests, their start-record count
@task_file n=20 path=tests/path1_run_start.rs
what: edited: the run-start integration tests, which hand the default and the given pair
@task_file n=21 path=tests/cli_revive.rs
what: edited: the viola revive integration tests, the recorded pair, the override and a stale recorded word
@task_file n=22 path=tests/chaos_snapshot_replay.rs
what: edited: the snapshot replay chaos tests, which read the pair after a killed wrapper
@task_file n=23 path=tests/cli_list.rs
what: edited: the viola list integration tests, which read model and effort in the --json items
@task_file n=24 path=tests/run_cli.rs
what: edited: the viola run usage tests, a bad effort and a pair after the dashes as exit 2
@task_file n=25 path=tests/cli_controls_not_disableable.rs
what: edited: the check that no setting disables a control, its settings list nine to eleven
@task_file n=26 path=tests/contract_bounded_readers.rs
what: edited: the bounded-reader boundary list, which gains the config.json default pair
@task_file n=27 path=crates/viola-state/src/liveness.rs
what: edited: the liveness read, its snapshot literal
@task_file n=28 path=crates/viola-state/src/strict.rs
what: edited: the strict-modes check, its snapshot literal
@task_file n=29 path=crates/viola-mcp/tests/handlers.rs
what: edited: the MCP handler tests, their snapshot literal
@task_file n=30 path=src/cmd/hook.rs
what: edited: the hook verb, its test snapshot literals
@task_file n=31 path=src/run/dialog.rs
what: edited: the dialog reader, its test snapshot literal
@task_file n=32 path=src/run/wheel.rs
what: edited: the wheel state, its test snapshot literal
@task_file n=33 path=src/run/send/slot.rs
what: edited: the send slot, its test snapshot literal
@task_file n=34 path=src/human/board/test_support.rs
what: edited: the human board's test support, its board item literal
@task_file n=35 path=tests/hook_fail_open.rs
what: edited: the hook fail-open tests, their snapshot literal
@task_file n=36 path=tests/security_negatives_server_identity.rs
what: edited: the server-identity security tests, their snapshot literal
@task_file n=37 path=tests/cli_fake_agent.rs
what: edited: the fake agent's integration tests, its start receipt key list
@task_item n=1 part=touches
name/command/run: the run verb gains --model and --effort.
@task_item n=2 part=touches
name/command/revive: the revive verb gains --model and --effort, which conflict with --list.
@task_item n=3 part=touches
contract/cli-exit-codes/2: a bad model or effort word, or a pair after the dashes, now lands on usage exit 2. No new exit code.
@task_item n=4 part=touches
contract/cli-stderr/fixed-message-refusal, contract/cli-refusal-line items and contract/cli-streams/stderr: a new fixed stderr line for the pair-after-dashes refusal names the two flags and echoes none of the caller's words.
@task_item n=5 part=touches
decision/hint-never-quotes-text, decision/no-internals-in-errors and decision/start-refusals-show-no-path-or-pid: the new usage refusal quotes no caller text, path or pid.
@task_item n=6 part=touches
decision/clap-without-colour-feature and decision/ascii-only-human-output: the new refusal is a raw clap error and must stay uncoloured and ASCII.
@task_item n=7 part=touches
decision/list-six-fixed-columns, contract/viola-list/columns and contract/viola-list-table/columns: the human board keeps its six columns. The pair is not a column.
@task_item n=8 part=touches
decision/json-one-document-on-stdout and the viola list --json rows: each wrapped item gains model and effort after role, each dropped when it does not re-parse.
@task_item n=9 part=touches
decision/list-rows-escape-all-controls and contract/viola-list-board/row-escaping: model and effort words reach the list output only after the closed-shape re-parse.
@task_item n=10 part=touches
decision/run-prints-nothing-while-wrapped: the new launch record is log-only and writes no terminal byte, so the run stays silent.
@task_item n=11 part=touches
contract/viola-run-output/child-started and contract/viola-revive-output/after-child-start: the child is now started with the pair. The printed output is unchanged.
@task_item n=12 part=touches
contract/viola-revive-output/list and contract/viola-revive-output/list-json: --list now conflicts with --model and --effort.
@task_item n=13 part=touches
contract/viola-revive-refusals/cwd-missing and contract/viola-revive-json-codes/cwd-missing: revive still refuses cwd-missing over every replayed snapshot. This standing behaviour disproved the planned revive-over-corrupt-snapshot check.
@task_item n=14 part=touches
contract/help-output/verb-groups and contract/viola-help/verb-groups: run and revive help gain the two flags. The verb groups are unchanged.
@task_item n=15 part=touches
contract/cli-callers/llm-driver: a launcher that passes the pair after the dashes now gets the usage exit 2 until it moves the pair to the run flags.
@task_item n=16 part=touches
decision/mcp-hidden-verb: the MCP list item gains model and effort, the same as list --json.
@task_fact n=1 slug=surface-cli
fact: viola run and viola revive take --model and --effort. A missing half comes from config.json default_model or default_effort, else from viola's compiled default pair.
@task_fact n=2 slug=surface-cli
fact: A --model or --effort word after the -- of run or revive is refused as usage exit 2, with one fixed stderr line that names the two flags and echoes none of the caller's words.
@task_fact n=3 slug=surface-cli
fact: viola revive hands the child its recorded model and effort unless overridden, and a recorded half that no longer parses takes the default half. The revive flags conflict with --list.
@task_fact n=4 slug=surface-cli
fact: The viola list human board shows no model or effort. The pair appears only as model and effort after role in the list --json and MCP list items.
@task_item n=1 part=untouched
Brand identity, colour palette, typography, spacing, depth, border radius and the web surface: no colour, glyph, weight or layout changed.
@task_item n=2 part=untouched
Send readback, the readback box and the viola send mirror: sending is unchanged.
@task_item n=3 part=untouched
viola wait, viola last, viola verify and the automatic-verify line output: none of them changed.
@task_item n=4 part=untouched
viola link and unlink output, pause and release, answer and plugin install verb output: unchanged.
@task_item n=5 part=untouched
Exit 21 causes, wrapper fault details, peer reply screening, refusal hints and the strict-modes refusals: no cause, hint or code was added.
@task_item n=6 part=untouched
CLI colour tokens, colour decision order and colour depth: the board's colour spans are unchanged.
@task_item n=7 part=untouched
Self-validation protocol and the anti-pattern decisions on colour, spinners and status dots: not reached.
