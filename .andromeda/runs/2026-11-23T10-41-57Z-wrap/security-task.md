@task_prose part=session
Every viola run start now names a model and an effort: viola run --model and --effort, else the config.json default_model and default_effort, else the compiled DEFAULT_PAIR; the pair is handed to the claude child's argv as --model M --effort E after the plugin-dir flag.
The pair is recorded per instance as a new log-only launch event line (the 17th event kind, after lineage) and an optional launch field of the instance snapshot; the replay recovers it, last launch line winning.
viola revive takes --model and --effort overrides, else hands the recorded pair while each half re-parses, else the default half; a pair written in the program words after the dashes is refused on run and on revive with one fixed usage line, clap's exit 2.
list --json and the MCP list item carry model and effort, each shown only when it re-parses; config.json known keys move 5 to 7; no IPC method, socket, port, env var, exit code or dependency added; nothing retired.
Whose word: the two flags, the two config keys and the compiled default, and the recorded pair crossing onto the revived child's argv, are the operator's own; every other answer (a recorded half that no longer parses, the revive override) is a delegate's word, provisional; no real claude session was measured running on the pair, rows say handed.
Measured: the plan's clause that a revive over a corrupt snapshot still hands the pair is disproved, revive refuses cwd-missing over every replayed snapshot, a standing behaviour unmoved.
@task_file n=1 path=src/run/launch.rs
what: new; the launch pair module of the run start: the compiled default pair, the bounded config.json default read, the per-half resolve, and the check that finds a model or effort flag in the caller's program words
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited; the Claude adapter crate holding the claude flag literals, now with the model and effort flags, the five effort words, the effort-word check, the compiled default pair and the launch argv builder
@task_file n=3 path=crates/viola-core/src/lib.rs
what: edited; the core crate of shared validators and the normalised event kinds, gaining the model-word closed-shape check and the log-only launch event kind
@task_file n=4 path=crates/viola-state/src/snapshot.rs
what: edited; the instance snapshot written only by the wrapper, gaining the optional launch field of model and effort
@task_file n=5 path=crates/viola-state/src/replay.rs
what: edited; the snapshot replay from the events log, gaining the launch-line rule (last line wins, a non-string member reads none)
@task_file n=6 path=crates/viola-state/src/board.rs
what: edited; the board read behind viola list and the MCP list tool, gaining model and effort shown only when each re-parses
@task_file n=7 path=crates/viola-state/tests/state_replay.rs
what: edited; the state replay tests, a cut-short snapshot replayed with the logged launch pair
@task_file n=8 path=crates/viola-state/tests/state_events.rs
what: edited; the events log kind tests, every kind now seventeen
@task_file n=9 path=src/run/mod.rs
what: edited; the wrapper's child launch building the claude child's argv: plugin-dir flag, then the launch pair, then the caller's words
@task_file n=10 path=src/cmd/run.rs
what: edited; the viola run command: its CLI arguments and their checks, the version-gate and config reads at start, and the start state records written before the spawn
@task_file n=11 path=src/cmd/revive.rs
what: edited; the viola revive command: its arguments, its trailing words passed as argv, and the pair it hands the resumed child
@task_file n=12 path=src/cmd/list.rs
what: edited; the viola list command and its --json document
@task_file n=13 path=src/obs.rs
what: edited; the observability module holding the closed list of known config.json keys
@task_file n=14 path=schemas/config.v1.json
what: edited; the config.json schema, gaining the default_model and default_effort keys
@task_file n=15 path=src/bin/viola-fake-agent.rs
what: edited; the test-only fake agent under the fake-agent feature, its argv options and its start receipt
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited; the fake agent's option tests
@task_file n=17 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the e2e harness boot readiness reading the start records of a wrapped session
@task_file n=18 path=tests/cli_instance_state.rs
what: edited; the instance state integration tests over the snapshot and the events log
@task_file n=19 path=tests/hook_events.rs
what: edited; the hook event integration tests, their count of start records
@task_file n=20 path=tests/path1_run_start.rs
what: edited; the run start integration tests, the default and given pair handed to the child and recorded
@task_file n=21 path=tests/cli_revive.rs
what: edited; the revive integration tests, the recorded pair, the override and a recorded word that no longer parses
@task_file n=22 path=tests/chaos_snapshot_replay.rs
what: edited; the killed-wrapper snapshot replay chaos tests
@task_file n=23 path=tests/cli_list.rs
what: edited; the viola list integration tests, the --json item keys
@task_file n=24 path=tests/run_cli.rs
what: edited; the viola run usage tests, a bad effort and a pair after the dashes refused
@task_file n=25 path=tests/cli_controls_not_disableable.rs
what: edited; the guard that no config.json setting disables a control, its settings list
@task_file n=26 path=tests/contract_bounded_readers.rs
what: edited; the bounded readers census guard, its list of reader boundaries
@task_file n=27 path=crates/viola-state/src/liveness.rs
what: edited; the instance liveness read, an instance snapshot literal gaining the empty launch field
@task_file n=28 path=crates/viola-state/src/strict.rs
what: edited; the strict-modes check, a test's instance snapshot literal gaining the empty launch field
@task_file n=29 path=crates/viola-mcp/tests/handlers.rs
what: edited; the MCP handler tests, an instance snapshot literal gaining the empty launch field
@task_file n=30 path=src/cmd/hook.rs
what: edited; the viola hook command, test instance snapshot literals gaining the empty launch field
@task_file n=31 path=src/run/dialog.rs
what: edited; the wrapper's dialog handling, an instance snapshot literal gaining the empty launch field
@task_file n=32 path=src/run/wheel.rs
what: edited; the wrapper's wheel, an instance snapshot literal gaining the empty launch field
@task_file n=33 path=src/run/send/slot.rs
what: edited; the wrapper's send slot, an instance snapshot literal gaining the empty launch field
@task_file n=34 path=src/human/board/test_support.rs
what: edited; the human board's test support, its board item literal gaining model and effort
@task_file n=35 path=tests/hook_fail_open.rs
what: edited; the hook fail-open tests, an instance snapshot literal gaining the empty launch field
@task_file n=36 path=tests/security_negatives_server_identity.rs
what: edited; the server-identity security negative tests, an instance snapshot literal gaining the empty launch field
@task_file n=37 path=tests/cli_fake_agent.rs
what: edited; the fake agent's CLI tests, its start receipt key pin gaining model and effort
@task_item n=1 part=touches
decision/session-id-argv-closed-shape and decision/session-id-argv-no-shell: a second crossing onto the claude child's argv now sits beside the session id, the launch pair as four separate argv elements of closed shape after the plugin-dir flag, never through a shell; the recorded pair also crosses onto the revived child's argv
@task_item n=2 part=touches
decision/run-lineage-flags-checked: viola run gains two more flags, --model and --effort, checked by closed-shape value parsers before anything is read; a sibling row is owed
@task_item n=3 part=touches
decision/lineage-kept-to-snapshot-and-event-log: the launch pair is kept the same way, to the snapshot's launch field and the events log's launch line, and crosses out through list --json and the MCP list item
@task_item n=4 part=touches
decision/lineage-parent-authenticates-nothing: the recorded model and effort are self-reported words beside the lineage and grant nothing; a sibling row is owed
@task_item n=5 part=touches
decision/board-lineage-and-links-name-checked: the board now also shows model and effort only when each re-parses (the model by the model-word check, the effort by the code-word check)
@task_item n=6 part=touches
contract/config-json/claude-env-keep, contract/config-json/diagnostics-level, contract/config-json/dialog-hold-s: two sibling keys, default_model and default_effort, join config.json; known keys 5 to 7
@task_item n=7 part=touches
decision/config-bad-value-handling and decision/config-tolerant-parse: a bad default_model or default_effort takes that half of the compiled default pair with one parse-rejected config-json malformed line per key; the read keeps no key past the frame cap and needs v 1
@task_item n=8 part=touches
decision/config-cannot-disable-controls, decision/controls-not-switchable and decision/no-control-switch-off: the guard's settings list grows by the two pair keys (9 to 11) and setting the pair disables no control
@task_item n=9 part=touches
decision/bounded-readers-census and decision/bounded-external-readers: one new bounded reader, the config.json default pair read, added to the census's boundaries
@task_item n=10 part=touches
decision/max-frame-shared-cap, decision/reads-capped-at-max-frame and decision/max-frame-cap: the new config default read is capped at MAX_FRAME
@task_item n=11 part=touches
decision/revive-trailing-words-as-argv: revive's trailing words now refuse a model or effort flag (or its equals form) with a fixed usage line; run's program words likewise
@task_item n=12 part=touches
decision/refusals-hold-no-paths-pids-ids, decision/error-messages-interpolate-nothing and decision/fixed-error-display-messages: the new pair-after-dashes refusal is one fixed line naming the two flags and echoing none of the caller's words
@task_item n=13 part=touches
decision/cli-input-trust-boundary and decision/cli-input-checked-before-frame: two new CLI inputs on run and on revive enter at the CLI boundary, checked by value parsers; a bad value is clap's usage exit 2
@task_item n=14 part=touches
decision/shared-validators-in-core: the model-word check lands in the core crate, but the effort-word check lives in the Claude adapter crate beside the flag literals, an exception to the convention
@task_item n=15 part=touches
decision/replayed-snapshot-inert, decision/replayed-snapshot-fallbacks and decision/replayed-snapshot-yields-nothing: the replay now recovers the launch pair from the last launch line, and revive's pair reading reads a replayed state's pair; revive over a replayed snapshot still refuses cwd-missing
@task_item n=16 part=touches
contract/revive-json/cwd-missing: measured as the reason the plan's revive over a corrupt snapshot cannot hand the pair; the row stands true and unmoved
@task_item n=17 part=touches
decision/readers-tolerate-torn-lines and decision/events-reader-bounded-lines: a launch line whose model or effort is not a string reads none; another kind changes nothing
@task_item n=18 part=touches
decision/snapshot-written-only-by-wrapper and decision/state-readers-never-write: the snapshot gains the optional launch field, written by the wrapper at start; no v move
@task_item n=19 part=touches
decision/revive-cwd-spawn-only and decision/revive-recorded-cwd-spawn-dir: a recorded value other than the cwd now reaches the revived child, the recorded model and effort onto its argv
@task_item n=20 part=touches
contract/revive-arguments/id: sibling revive arguments --model and --effort are added, conflicting with --list
@task_item n=21 part=touches
contract/mcp-tools/list: the MCP list item gains optional model and effort after role
@task_item n=22 part=touches
decision/data-classes and decision/user-content-only-to-diagnostics: the pair is a new recorded value, two closed words, no personal data; the launch line opens no span and writes no process-log line
@task_item n=23 part=touches
decision/env-not-config-channel: the default pair is read from config.json or the compiled constant, never from an environment variable
@task_item n=24 part=touches
decision/test-seams-fake-agent-only and decision/test-seams-compiled-out: the fake agent takes --model and --effort on argv only and its start receipt gains model and effort
@task_fact n=1 slug=input-validation
fact: viola run --model and --effort are checked before anything is read: a model word is 1 to 64 bytes, a lowercase letter first then lowercase letters, digits, dots and dashes; an effort is one of the five effort words of claude --help; a bad value is clap's usage exit 2
@task_fact n=2 slug=input-validation
fact: the launch pair is handed to the claude child as four separate argv elements, --model M --effort E, after the plugin-dir flag and before the caller's words, never through a shell; revive hands the recorded pair onto the resumed child's argv as a second ratified crossing
@task_fact n=3 slug=input-validation
fact: run and revive refuse a --model or --effort flag, or its equals form, in the program words after the dashes with one fixed usage line naming the two flags and echoing none of the caller's words
@task_fact n=4 slug=input-validation
fact: config.json default_model and default_effort hold the same closed shapes; a bad value takes that half of the compiled default pair, named once by the constant DEFAULT_PAIR, with one parse-rejected line per key, and neither key disables a control
@task_fact n=5 slug=input-validation
fact: revive takes each half from its override, else the recorded half while it still re-parses, else the default half; the replay takes the last launch line whole, a non-string member reading none
@task_fact n=6 slug=input-validation
fact: the board shows model and effort only when each re-parses, the model by the model-word check, the effort by the code-word check
@task_fact n=7 slug=data-protection
fact: the launch pair is kept to the instance snapshot's launch field and the events log's launch line, log-only with no span, process-log line or terminal byte, and leaves only through list --json and the MCP list item
@task_fact n=8 slug=authentication-authorization
fact: the recorded model and effort are self-reported words and authenticate nothing
@task_fact n=9 slug=input-validation
fact: the effort-word check lives in the Claude adapter crate beside the claude flag literals, an exception to shared validators in the core crate
@task_item n=1 part=untouched
dependency-security: no dependency, toolchain, version floor, action pin, audit tool or workflow moved
@task_item n=2 part=untouched
secret-management: the credential strip, the child environment and its kept names are unchanged; the pair reads no environment variable
@task_item n=3 part=untouched
api-security: the rmcp feature set and its bans are unchanged
@task_item n=4 part=untouched
the absences: no network listener, outbound call, model API call of viola's own, hosting, release, secret or account added; the pair is only words handed to the unmodified claude CLI
@task_item n=5 part=untouched
the paste and answer text character sets and the bracketed-paste rows: no typed text path moved
@task_item n=6 part=untouched
the verify, fixture, probe and live-rig rows of data-protection and secret-management: no verify run, fixture or rig moved
@task_item n=7 part=untouched
the socket directory rows and the wrapper channel method and frame rows: no channel method, frame param, socket or port added
@task_item n=8 part=untouched
the pinned binary, plugin rewrite and marketplace rows: no pinned artefact or plugin file changed
