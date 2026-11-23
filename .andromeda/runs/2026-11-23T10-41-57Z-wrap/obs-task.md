@task_prose part=session
Every wrapped start now names a model and an effort: viola run takes --model and --effort, else config.json default_model and default_effort, else the compiled default pair DEFAULT_PAIR; the pair is handed to the claude child after the plugin flag.
Each start writes a fourth product record, a launch line (model, effort, source wrapper) after the lineage line and before the hook's session-start; the snapshot gains an optional launch field and the replay recovers the pair from the last launch line.
viola revive takes --model and --effort too and hands the override half, else the recorded half while it re-parses, else the default half; it writes its own launch line.
A pair among the words after -- is refused by run and revive as a usage error (clap exit 2) with one fixed stderr line naming the two flags and echoing none of the caller's words.
list --json and the MCP list item gain model and effort after role, each re-parsed at the board; the human board's six columns are unchanged.
The launch line opens no span and writes no process-log line; no exit code, detail word, env var, socket or capability was added, and the plan's revive-over-a-corrupt-snapshot clause was disproved by the standing cwd-missing refusal.
@task_file n=1 path=src/run/launch.rs
what: new; the start's launch-pair module: the compiled default pair, the config.json default read, the resolve of each missing half, the caller-words pair check and its fixed usage line
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited; the claude agent crate's flag literals and child argv builders, now with the model and effort flags, the five effort words and the default pair
@task_file n=3 path=crates/viola-core/src/lib.rs
what: edited; the core types: the closed model-word shape and the normalised event kinds, the launch kind appended (16 to 17), not a wait wake
@task_file n=4 path=crates/viola-state/src/snapshot.rs
what: edited; the instance snapshot record, gaining the optional launch field
@task_file n=5 path=crates/viola-state/src/replay.rs
what: edited; the snapshot replay over the events log, now recovering the launch pair from the last launch line
@task_file n=6 path=crates/viola-state/src/board.rs
what: edited; the board read behind list --json and the MCP list item, now carrying model and effort after role, each re-parsed
@task_file n=7 path=crates/viola-state/tests/state_replay.rs
what: edited; the state replay tests, a cut snapshot replayed with the logged launch pair
@task_file n=8 path=crates/viola-state/tests/state_events.rs
what: edited; the event-kind census test, now seventeen kinds
@task_file n=9 path=src/run/mod.rs
what: edited; the wrapper's claude child launch, building the plugin flag, then the launch pair, then the caller's words
@task_file n=10 path=src/cmd/run.rs
what: edited; viola run's arguments and start: the model and effort flags, the default read beside the dialog hold, the launch line after the lineage line, the pair-after-dashes refusal
@task_file n=11 path=src/cmd/revive.rs
what: edited; viola revive: the model and effort override flags, the recorded pair handed to the resumed child, the pair-after-dashes refusal
@task_file n=12 path=src/cmd/list.rs
what: edited; viola list and its --json document, the item carrying the launch pair
@task_file n=13 path=src/obs.rs
what: edited; the obs module's known config.json keys list, five to seven
@task_file n=14 path=schemas/config.v1.json
what: edited; the config.json schema, gaining default_model and default_effort
@task_file n=15 path=src/bin/viola-fake-agent.rs
what: edited; the fake agent, taking the model and effort flags and adding them to its start receipt
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited; the fake agent's option tests, the pair taken from argv
@task_file n=17 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the harness boot readiness, reading four start records, and the harness status list items carrying model and effort
@task_file n=18 path=tests/cli_instance_state.rs
what: edited; the instance state integration tests, the start records and snapshot with the launch pair
@task_file n=19 path=tests/hook_events.rs
what: edited; the hook events integration tests, the start-record counter moved from three to four
@task_file n=20 path=tests/path1_run_start.rs
what: edited; the run start path tests, the default and the given pair handed and recorded, the launch line third
@task_file n=21 path=tests/cli_revive.rs
what: edited; the revive integration tests, the recorded pair, the override and a recorded word that no longer parses
@task_file n=22 path=tests/chaos_snapshot_replay.rs
what: edited; the snapshot replay chaos tests, the replay reading the launch pair a killed wrapper wrote
@task_file n=23 path=tests/cli_list.rs
what: edited; the list integration tests, list --json carrying model and effort and the moved key lists
@task_file n=24 path=tests/run_cli.rs
what: edited; the run usage tests, a bad effort and a pair after the dashes each a usage error
@task_file n=25 path=tests/cli_controls_not_disableable.rs
what: edited; the check that no config.json setting disables a control, its settings 9 to 11
@task_file n=26 path=tests/contract_bounded_readers.rs
what: edited; the bounded-readers census, one boundary added for the default pair keys of config.json
@task_file n=27 path=crates/viola-state/src/liveness.rs
what: edited; the liveness reader, a test snapshot literal gaining an empty launch
@task_file n=28 path=crates/viola-state/src/strict.rs
what: edited; the strict-modes check, a test snapshot literal gaining an empty launch
@task_file n=29 path=crates/viola-mcp/tests/handlers.rs
what: edited; the MCP handler tests, a snapshot literal gaining an empty launch
@task_file n=30 path=src/cmd/hook.rs
what: edited; the viola hook role, its test snapshot literals gaining an empty launch
@task_file n=31 path=src/run/dialog.rs
what: edited; the wrapper's dialog handling, a test snapshot literal gaining an empty launch
@task_file n=32 path=src/run/wheel.rs
what: edited; the wrapper's wheel, a test snapshot literal gaining an empty launch
@task_file n=33 path=src/run/send/slot.rs
what: edited; the wrapper's send slot, a test snapshot literal gaining an empty launch
@task_file n=34 path=src/human/board/test_support.rs
what: edited; the human board's test support, the list item literal gaining model and effort
@task_file n=35 path=tests/hook_fail_open.rs
what: edited; the hook fail-open tests, a snapshot literal gaining an empty launch
@task_file n=36 path=tests/security_negatives_server_identity.rs
what: edited; the server-identity security negatives, a snapshot literal gaining an empty launch
@task_file n=37 path=tests/cli_fake_agent.rs
what: edited; the fake agent's receipt key pin, gaining model and effort
@task_item n=1 part=touches
contract/run-start-events/order: the start now writes four records, wheel, lineage, launch, then the hook's session-start; three is stale.
@task_item n=2 part=touches
decision/product-order-from-events-log: the start product order read from the events log now holds the launch line after the lineage line.
@task_item n=3 part=touches
contract/config-json/known-keys: the known keys move from five to seven, adding default_model and default_effort.
@task_item n=4 part=touches
contract/diagnostics-events/config-parse-rejected and contract/parse-rejected-detail/malformed: a bad default_model or default_effort value writes one parse-rejected line, parser config-json, detail malformed, per key, and that half takes the compiled default.
@task_item n=5 part=touches
contract/config-json/v and decision/config-observed-at-load: the default pair is read at start beside dialog_hold_s, only under v 1, capped at the frame size.
@task_item n=6 part=touches
decision/lineage-line-untraced: a sibling is owed, the launch line has no trace of its own, no span and no process-log line.
@task_item n=7 part=touches
decision/lineage-names-confined: a sibling is owed, model and effort are confined to the snapshot, the events log and the list item, never a process log.
@task_item n=8 part=touches
decision/event-payload-not-in-process-logs and decision/no-event-payloads-in-process-logs: the new launch event's data stays in the events log, never copied into a process log.
@task_item n=9 part=touches
contract/event-line/shape and contract/event-stream/line: a new event kind launch (data model and effort, source wrapper) is written to the events log; event kinds 16 to 17.
@task_item n=10 part=touches
contract/replayed-snapshot/live-fields and decision/no-raw-snapshot-reads: a snapshot rebuilt by replay now carries the launch pair from the last launch line; product readers read it through replay.
@task_item n=11 part=touches
contract/viola-mcp-tools/list and contract/list-json/liveness: the wrapped list item gains model and effort after role, each re-parsed at the board, an unparsable word dropped.
@task_item n=12 part=touches
contract/run-terminal-output/start-refusal, decision/fixed-stderr-messages and decision/error-messages-interpolate-nothing: run and revive refuse a pair among the words after -- with clap's usage exit 2 and one fixed stderr line naming the two flags, echoing none of the caller's words.
@task_item n=13 part=touches
decision/child-settings-never-logged and contract/run-process-log/process-start-claude-child: the claude child's argv now carries --model and --effort after the plugin flag; the process-start claude-child line gains no field.
@task_item n=14 part=touches
decision/revive-start-logs-as-run, decision/revive-preflight-order and contract/cli-log-file/revive-start: viola revive takes --model and --effort (conflicting with --list) and its start writes its own launch line; the preflight order and its codes are unmoved.
@task_item n=15 part=touches
contract/revive-preflight-detail/cwd-missing and decision/replayed-snapshot-unreachable: the standing cwd-missing refusal over every replayed snapshot is the basis that disproved the plan's revive-over-a-corrupt-snapshot clause; the rows stand unmoved.
@task_item n=16 part=touches
decision/redaction-cannot-be-disabled and decision/allow-list-not-widenable: setting the default pair disables no control; the settings check grows from nine to eleven.
@task_item n=17 part=touches
decision/env-vars-not-config-channel and decision/env-vars-not-configuration: the pair comes from flags, config.json or the compiled constant, never from the environment.
@task_item n=18 part=touches
contract/agent-run-status/state and contract/agent-run-logs/events-record: the harness readiness reads four start records and the status document's list items carry model and effort.
@task_item n=19 part=touches
decision/claude-internals-not-a-contract and decision/claude-boundary-only-instrumentation: the pair is handed to claude; whether a real session runs on it, at start or on resume, is unmeasured, so rows say handed, never runs on.
@task_fact n=1 slug=span-trace-coverage
fact: The launch line every start writes after the lineage line has no trace of its own: it opens no span, writes no process-log line and no terminal byte.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: Model and effort are two closed words confined to product surfaces, the snapshot, the events log launch line and the list item, re-parsed at the board, never written to a process log.
@task_fact n=3 slug=otel-sdk-init
fact: config.json default_model takes the closed model-word shape and default_effort one of the five effort words; a bad value takes that half of DEFAULT_PAIR with one parse-rejected config-json malformed line per key, and neither key disables a control.
@task_fact n=4 slug=log-format-json-schema
fact: The events log carries a launch kind, data model and effort, source wrapper, log-only and no wait wake; the replay takes the last launch line whole, a line with a non-string member reading none.
@task_fact n=5 slug=3-observability-harness-contract
fact: viola run and viola revive refuse --model or --effort among the words after -- as a usage error, exit 2, with one fixed stderr line naming the two flags, echoing none of the caller's words and writing no diagnostics line.
@task_fact n=6 slug=log-coverage
fact: viola revive hands the resumed child the override half, else the recorded half while it re-parses, else the start's default half, and records the pair in its own launch line.
@task_item n=1 part=untouched
Panic capture, the panic line and its outcome by role, and the error-reporter absences: no panic path or reporter moved.
@task_item n=2 part=untouched
Span naming, span kinds, span attribute lists and the OpenTelemetry absences: the session declares no span.
@task_item n=3 part=untouched
Metric coverage, perf budgets and the SLO gates: no counter, budget or threshold moved.
@task_item n=4 part=untouched
CI integration: the secret scan, artifact uploads and retention, pinned actions and the fail-closed gates are unchanged.
@task_item n=5 part=untouched
Log levels, log rotation, diagnostics file paths and owner-only modes: no sink, level or file moved.
@task_item n=6 part=untouched
Correlation through corr and conn and the channel join keys: no correlated line added.
@task_item n=7 part=untouched
The exit 21 cause words and unreachable-instance documents: no exit code or detail word added.
@task_item n=8 part=untouched
The detail-file line schema, veil redaction and the secret-scan hit report: no content-bearing record added.
