@task_prose part=session
Every wrapped start now names a model and an effort for the child: viola run takes --model and --effort, else the default pair read from config.json default_model and default_effort, else the compiled default pair; the pair is handed on the child's argv after the plugin flag.
The pair is recorded per instance: a new log-only launch event kind (event kinds 16 to 17) written after the lineage line (start records 3 to 4), and an optional launch field in the instance snapshot that the replay recovers.
viola revive takes --model and --effort (conflicting with --list); without them it hands the recorded pair, a recorded half that no longer parses taking the default half.
A pair written after the -- among the caller's program words is refused as a usage error, clap's exit 2, with one fixed stderr line that names the two flags and echoes none of the caller's words.
list --json and the MCP list item gain optional model and effort after role, re-parsed at the board; the human six-column board is unchanged.
The plan's clause that a revive over a corrupt snapshot still hands the pair was disproved (revive refuses cwd-missing over every replayed snapshot, unmoved); no capability claimed, no new exit code, closed word, hint or time bound.
@task_file n=1 path=src/run/launch.rs
what: new module of the root bin: the start's launch pair, the default read from config.json, the resolve of each missing half, the check that finds a pair in the caller's words, and the fixed usage line naming the two flags
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent crate holding the Claude flag literals and the child's argv builders; gains the --model and --effort flags, the five effort words and the compiled default pair
@task_file n=3 path=crates/viola-core/src/lib.rs
what: the core crate's closed shapes and the normalised event kinds; gains the model-word shape and the launch kind
@task_file n=4 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot document; gains the optional launch field
@task_file n=5 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the events log; recovers the launch pair, the last launch line winning
@task_file n=6 path=crates/viola-state/src/board.rs
what: the board read behind viola list and list --json; the list item gains model and effort, each kept only when it re-parses
@task_file n=7 path=crates/viola-state/tests/state_replay.rs
what: the state crate's replay tests; a cut-short snapshot replayed with the logged launch pair
@task_file n=8 path=crates/viola-state/tests/state_events.rs
what: the state crate's event-kind tests; every kind now seventeen
@task_file n=9 path=src/run/mod.rs
what: viola run's wrapper start and the child launch; the child argv builder now hands the pair after the plugin flag
@task_file n=10 path=src/cmd/run.rs
what: the viola run command's arguments and start; the --model and --effort flags, their value parsers and the usage refusal of a pair after the dashes
@task_file n=11 path=src/cmd/revive.rs
what: the viola revive command; the --model and --effort override, the recorded pair handed, the same usage refusal after the dashes
@task_file n=12 path=src/cmd/list.rs
what: the viola list command's document for --json; carries the launch pair
@task_file n=13 path=src/obs.rs
what: the observability module's list of config.json known keys, five to seven
@task_file n=14 path=schemas/config.v1.json
what: the config.json schema; gains default_model and default_effort
@task_file n=15 path=src/bin/viola-fake-agent.rs
what: the fake agent test double; takes --model and --effort and reports them in its start receipt
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option-parsing tests
@task_file n=17 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot readiness check; now reads four start records
@task_file n=18 path=tests/cli_instance_state.rs
what: the instance state integration tests; the snapshot and start records with the launch line
@task_file n=19 path=tests/hook_events.rs
what: the hook event integration tests; the start-record count moved by one
@task_file n=20 path=tests/path1_run_start.rs
what: the run start integration tests; the default pair and a given pair handed and recorded
@task_file n=21 path=tests/cli_revive.rs
what: the viola revive integration tests; the recorded pair, the override and a recorded word that no longer parses
@task_file n=22 path=tests/chaos_snapshot_replay.rs
what: the chaos snapshot-replay tests; the replay reads the launch pair after a killed wrapper
@task_file n=23 path=tests/cli_list.rs
what: the viola list integration tests; list --json carries model and effort, the human board literal rows unchanged
@task_file n=24 path=tests/run_cli.rs
what: the viola run usage tests; a bad effort and a pair after the dashes each exit 2
@task_file n=25 path=tests/cli_controls_not_disableable.rs
what: the check that no setting disables a control; its settings list gains the two default-pair keys
@task_file n=26 path=tests/contract_bounded_readers.rs
what: the bounded-readers guard's boundary list; gains the config.json default pair
@task_file n=27 path=crates/viola-state/src/liveness.rs
what: the instance liveness reader; a snapshot literal gains an empty launch
@task_file n=28 path=crates/viola-state/src/strict.rs
what: the strict-modes checks; a snapshot literal gains an empty launch
@task_file n=29 path=crates/viola-mcp/tests/handlers.rs
what: the MCP server handler tests; a snapshot literal gains an empty launch
@task_file n=30 path=src/cmd/hook.rs
what: the viola hook event and dialog call; snapshot literals gain an empty launch
@task_file n=31 path=src/run/dialog.rs
what: the wrapper's dialog handling; a snapshot literal gains an empty launch
@task_file n=32 path=src/run/wheel.rs
what: the wrapper's wheel between human and driver; a snapshot literal gains an empty launch
@task_file n=33 path=src/run/send/slot.rs
what: the wrapper's send slot; a snapshot literal gains an empty launch
@task_file n=34 path=src/human/board/test_support.rs
what: the human board's test support; the list item literal gains model and effort
@task_file n=35 path=tests/hook_fail_open.rs
what: the hook fail-open tests; a snapshot literal gains an empty launch
@task_file n=36 path=tests/security_negatives_server_identity.rs
what: the server-identity security negative tests; a snapshot literal gains an empty launch
@task_file n=37 path=tests/cli_fake_agent.rs
what: the fake agent integration tests; the start receipt's key pin gains model and effort
@task_item n=1 part=touches
The CLI usage-error exit 2 rows: a new usage refusal, a launch pair after the -- in viola run or viola revive, exits 2 through clap with one fixed stderr line; a bad --model or --effort value is a usage error too.
@task_item n=2 part=touches
Fixed CLI strings pinned by literal: the new usage line naming --model <MODEL> and --effort <EFFORT> is one fixed string pinned by a literal test.
@task_item n=3 part=touches
Printed names are the parsed arguments and refusal layout rows: the new usage line echoes none of the caller's words.
@task_item n=4 part=touches
viola --help rows (usage line, served verbs only): viola run and viola revive each gain two flags in their usage; no verb added.
@task_item n=5 part=touches
viola revive --list rows: --model and --effort conflict with --list, a clap usage refusal.
@task_item n=6 part=touches
The config.json contract rows beside dialog_hold_s: two new keys default_model and default_effort; a bad value is no refusal, the start takes that half of the default with one parse-rejected diagnostic line per key.
@task_item n=7 part=touches
Run diagnostics never reach the terminal and zero viola bytes on the terminal: the new launch event line and the config parse-rejected line are written off the terminal; the launch line writes no terminal byte.
@task_item n=8 part=touches
TUI passthrough entry and the child's terminal: the child is now started with the pair on its argv after the plugin flag; the human's terminal still belongs to the child.
@task_item n=9 part=touches
viola list board rows (structure, caption order, colour): the list item gains model and effort under --json and MCP list only; the six-column human board, its captions and colour stay unchanged.
@task_item n=10 part=touches
Revive refusal rows, cwd-missing: revive still refuses cwd-missing over a replayed snapshot; the disproved plan clause leaves that refusal standing.
@task_item n=11 part=touches
Boundary-test rows sequenced on records: the start now writes four records (wheel, lineage, launch, session-start), so tests sequenced on the start records read four lines.
@task_item n=12 part=touches
Paths not carried: the launch pair is two closed words and carries no path into any output.
@task_fact n=1 slug=a11y-scope-summary
fact: A launch pair written among the caller's words after -- in viola run or viola revive is a usage error, exit 2, with one fixed stderr line naming --model <MODEL> and --effort <EFFORT> and echoing none of the caller's words.
@task_fact n=2 slug=cognitive-accessibility
fact: config.json default_model and default_effort never refuse a start: a missing or malformed value takes that half of the compiled default pair, and no value of either key disables a control.
@task_fact n=3 slug=a11y-scope-summary
fact: The launch pair shows in list --json and the MCP list item as optional model and effort after role, each dropped when it no longer parses; the human six-column board shows no pair.
@task_item n=1 part=untouched
Colour and SGR rules (amber on DIALOG, bold on NAME, dim on stale rows, no SGR off a terminal): no colour or word on the human board changed.
@task_item n=2 part=untouched
Keyboard and wheel rows: human keystrokes, the non-editing stdin list, send refused while the human types, focus, mouse and resize sequences; nothing in the wheel's behaviour moved.
@task_item n=3 part=untouched
Screen-reader targets and the manual Orca and keyboard passes: no setup or target changed.
@task_item n=4 part=untouched
CI integration, SLO and gate rows (no skips, zero flakiness, coverage and doctest, fail-fast): no gate, threshold or driver changed.
@task_item n=5 part=untouched
Absences (web or graphical frontend, touch surface, internationalization, rendered page, a time limit on the user): none changed; no time bound was added.
@task_item n=6 part=untouched
Hint and fault rows (one hint per code, faults carry no hint, unknown refusal, instance-unreachable details, exit codes 10 to 21): no new exit code, code word, hint or fault body.
@task_item n=7 part=untouched
Link, unlink, plugin install, send, wait and verify output rows: none of those verbs changed.
