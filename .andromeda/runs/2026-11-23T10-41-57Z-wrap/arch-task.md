@task_prose part=session
Every start now names a model and an effort for the claude child: the run flag given, else the config default, else the compiled default pair, and the pair is handed on the child's argv after the plugin flag.
The pair is recorded per instance: a new wrapper-appended, log-only event kind launch written after the lineage line at every start, and an optional launch field on the instance snapshot that the replay recovers from the last launch line.
revive gains its own model and effort overrides; with none it hands the recorded pair while each half still parses, else the default half, and the revived start writes its own launch line.
config.json gains two known keys, default_model and default_effort; a bad value falls back to that half of the compiled pair with one parse-rejected line per key.
The list document and the MCP list item gain optional model and effort after role, re-parsed at the board; a pair written in the caller's words after the dashes is refused as usage exit 2 with one fixed line.
Measured: the planned acceptance clause that a revive over a corrupt snapshot still hands the pair cannot be met, since revive refuses every replayed snapshot as cwd-missing; event kinds 16 to 17, start records 3 to 4, config known keys 5 to 7.
@task_file n=1 path=src/run/launch.rs
what: added — the start's launch-pair module in the root bin: the compiled default pair, the config default reader, the resolve of flag over config over compiled, the caller-words pair check and its fixed usage line
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited — the Claude agent crate holding the claude flag literals and child argv builders: now the --model and --effort literals, the five effort words, the compiled default pair and the launch argv
@task_file n=3 path=crates/viola-core/src/lib.rs
what: edited — viola-core, the agent-neutral core holding the normalised event kinds and closed word shapes: the launch kind added last, the model word shape added
@task_file n=4 path=crates/viola-state/src/snapshot.rs
what: edited — the instance snapshot record: gains the optional launch field
@task_file n=5 path=crates/viola-state/src/replay.rs
what: edited — the snapshot replay from the events log: recovers the launch pair by the last launch line
@task_file n=6 path=crates/viola-state/src/board.rs
what: edited — the board read model behind the CLI list and MCP list: items gain model and effort, re-parsed
@task_file n=7 path=crates/viola-state/tests/state_replay.rs
what: edited — the state crate's replay integration tests: a cut-short snapshot replayed with the logged launch pair
@task_file n=8 path=crates/viola-state/tests/state_events.rs
what: edited — the state crate's event-kind tests: every kind, now seventeen
@task_file n=9 path=src/run/mod.rs
what: edited — the wrapper's run path that builds the child's argv: the pair handed after the plugin flag, before the caller's arguments
@task_file n=10 path=src/cmd/run.rs
what: edited — the viola run command: its arguments gain --model and --effort, the start-state writes the launch line after lineage, the caller-words pair refusal
@task_file n=11 path=src/cmd/revive.rs
what: edited — the viola revive command: its arguments gain --model and --effort, the recorded pair handed on resume, the caller-words pair refusal
@task_file n=12 path=src/cmd/list.rs
what: edited — the viola list command and its JSON document: carries the launch pair
@task_file n=13 path=src/obs.rs
what: edited — the root bin's observability module holding the known config.json keys list, now seven
@task_file n=14 path=schemas/config.v1.json
what: edited — the config.json schema: gains default_model and default_effort
@task_file n=15 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent test binary: takes --model and --effort on argv, its start receipt gains model and effort
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited — the fake agent's option-parsing tests
@task_file n=17 path=crates/viola-e2e/src/harness/boot.rs
what: edited — the end-to-end harness boot readiness check: reads four start records, its status list items carry model and effort
@task_file n=18 path=tests/cli_instance_state.rs
what: edited — integration tests of the instance state a start writes: snapshot and events log
@task_file n=19 path=tests/hook_events.rs
what: edited — integration tests of hook events arriving normalised, counting the start records ahead of them
@task_file n=20 path=tests/path1_run_start.rs
what: edited — integration tests of the run start path: the default pair and a given pair handed and recorded
@task_file n=21 path=tests/cli_revive.rs
what: edited — integration tests of viola revive: the recorded pair, an override, a recorded word that no longer parses
@task_file n=22 path=tests/chaos_snapshot_replay.rs
what: edited — chaos tests of the snapshot replay after a killed wrapper
@task_file n=23 path=tests/cli_list.rs
what: edited — integration tests of viola list and its JSON items
@task_file n=24 path=tests/run_cli.rs
what: edited — CLI tests of viola run usage refusals
@task_file n=25 path=tests/cli_controls_not_disableable.rs
what: edited — the check that no setting disables a human control: its settings list gains the two default-pair keys
@task_file n=26 path=tests/contract_bounded_readers.rs
what: edited — the bounded-readers guard and its boundaries list: gains the config.json default-pair reader
@task_file n=27 path=crates/viola-state/src/liveness.rs
what: edited — session liveness in the state crate: a snapshot literal gains an empty launch
@task_file n=28 path=crates/viola-state/src/strict.rs
what: edited — the strict-modes read rule in the state crate: a snapshot literal gains an empty launch
@task_file n=29 path=crates/viola-mcp/tests/handlers.rs
what: edited — the MCP tool handlers' tests: a snapshot literal gains an empty launch
@task_file n=30 path=src/cmd/hook.rs
what: edited — the viola hook command: test snapshot literals gain an empty launch
@task_file n=31 path=src/run/dialog.rs
what: edited — the wrapper's dialog hold: a test snapshot literal gains an empty launch
@task_file n=32 path=src/run/wheel.rs
what: edited — the wrapper's wheel: a test snapshot literal gains an empty launch
@task_file n=33 path=src/run/send/slot.rs
what: edited — the wrapper's send slot: a test snapshot literal gains an empty launch
@task_file n=34 path=src/human/board/test_support.rs
what: edited — the human board's test support: the board item literal gains model and effort
@task_file n=35 path=tests/hook_fail_open.rs
what: edited — integration tests that hooks fail open: a snapshot literal gains an empty launch
@task_file n=36 path=tests/security_negatives_server_identity.rs
what: edited — security negative tests of the server identity check: a snapshot literal gains an empty launch
@task_file n=37 path=tests/cli_fake_agent.rs
what: edited — CLI tests of the fake agent: its start receipt key pin gains model and effort
@task_item n=1 part=touches
decision/wrapper-events-log-only and the normalised event kinds: a new wrapper-appended kind launch, log-only, not a wait wake kind; the event-data rows and the wrapper event appends gain a launch sibling.
@task_item n=2 part=touches
decision/event-kinds-agent-neutral: the launch kind is added in viola-core, appended last.
@task_item n=3 part=touches
The instance snapshot contract rows (data, spawn order, writer): an optional launch field with model and effort, omitted when absent, no v move; the first snapshot carries it.
@task_item n=4 part=touches
decision/snapshot-replay-log-fields-only and contract/snapshot-envelope/unreadable-snapshot: the replay now also recovers the launch pair from the last launch line, both members strings or none.
@task_item n=5 part=touches
decision/revive-resumes-in-place: its claim that revive replays no launch spec is stale; revive now hands the recorded model and effort (or an override) onto the resumed child's argv.
@task_item n=6 part=touches
decision/revive-in-place and contract/viola-cli-arguments/revive: revive gains --model and --effort, conflicting with --list; the revived start writes its own launch line.
@task_item n=7 part=touches
decision/revive-refused-without-cwd: the standing refusal of revive over a replayed snapshot is the basis of the disproved acceptance clause; the row stands unmoved, but a criterion row claiming a revive over a corrupt snapshot hands the pair must be amended to what was built (the replay recovers the pair, read by list).
@task_item n=8 part=touches
The child argv contracts (child-argv caller-arguments, plugin-dir, resume; claude-child-argv plugin-dir, resume): the argv is now the plugin flag, then --model M --effort E, then the caller's words; the resume argv itself is unchanged.
@task_item n=9 part=touches
decision/claude-flag-literals-in-agent-crate: the agent crate now also holds the --model and --effort literals, the five effort words and the compiled default pair; decision/agent-claude-pure and decision/claude-shapes-isolated hold over them.
@task_item n=10 part=touches
contract/child-argv/caller-arguments and decision/fake-agent-after-double-dash: the caller's words after the dashes may no longer carry --model or --effort (or the equals form); run and revive refuse them as usage exit 2 with one fixed line naming the two flags, echoing none of the caller's words.
@task_item n=11 part=touches
The viola CLI arguments contracts for run: run gains --model and --effort beside --parent and --role, each a value parser over a closed word shape; decision/cli-flag-and-subcommand-style holds over them.
@task_item n=12 part=touches
decision/config-keys-match-schema and contract/config-json/unknown-keys: config.json known keys 5 to 7 (default_model, default_effort), schema and known-keys list moved together.
@task_item n=13 part=touches
decision/config-bad-value-falls-back and decision/config-falls-back-to-defaults: a bad default_model or default_effort takes that half of the compiled pair with one parse-rejected config-json malformed process-log line per key.
@task_item n=14 part=touches
decision/config-precedence: the launch pair resolves flag over config.json over the compiled default, each half apart; decision/no-environment-configuration holds, no environment variable feeds the pair.
@task_item n=15 part=touches
The list payload and list item contracts (list-payload/items, list-item parent and role siblings) and contract/mcp-tools/list: wrapped items gain optional model and effort after role, kept only when each re-parses; the six-column human board is unchanged.
@task_item n=16 part=touches
decision/board-read-model-shape: the board read model re-parses the launch words with core shapes, staying sync and Claude-free.
@task_item n=17 part=touches
decision/run-start-order: the start now writes wheel, lineage, launch, then the hook's session-start; the child is still spawned last.
@task_item n=18 part=touches
decision/bounded-readers: the config.json default-pair reader is a new bounded reader, capped at MAX_FRAME, and the guard's boundaries list gains its row.
@task_item n=19 part=touches
decision/unparseable-external-values-unknown and decision/missing-equals-null-on-read: a recorded model or effort that no longer parses reads as missing, the default half on revive and dropped on the board.
@task_item n=20 part=touches
decision/versioned-records-no-migrations and decision/mixed-version-tolerance: older snapshots without launch stay readable; the new field and the new kind need no version move.
@task_item n=21 part=touches
decision/codes-only-process-logs: the launch line opens no span and writes no process-log line; only the config rejection writes a codes-only parse-rejected line.
@task_item n=22 part=touches
contract/code-word/format and decision/control-characters-refused: the effort word is re-read as a code word at the board; the model word gets its own closed shape, 1 to 64 bytes, a lowercase letter first, then lowercase letters, digits, dots and dashes.
@task_item n=23 part=touches
decision/fake-agent-test-seams: the fake agent takes the pair on argv only, no new seam variable; its start receipt gains model and effort.
@task_fact n=1 slug=established-decisions
fact: Every start, run and revive, hands the claude child a model and effort pair; the pair is handed on the argv, never read back or verified as the one the session runs on.
@task_fact n=2 slug=occupied-resources
fact: The child argv carries --model M --effort E after --plugin-dir and before the caller's words, at every start and every resume.
@task_fact n=3 slug=conventions
fact: The normalised event kind launch is wrapper-appended and log-only, written after lineage at every start with source wrapper, and is no wait wake kind.
@task_fact n=4 slug=standard-contracts
fact: The launch event data holds model and effort, both strings; the replay takes the last launch line, a line with a non-string member reading none.
@task_fact n=5 slug=standard-contracts
fact: The instance snapshot carries an optional launch of model and effort, omitted when absent, with no snapshot version move.
@task_fact n=6 slug=occupied-resources
fact: config.json default_model is a string of the model word shape and default_effort one of the five effort words; a bad value takes that half of DEFAULT_PAIR with one parse-rejected line per key, and neither key disables a control.
@task_fact n=7 slug=occupied-resources
fact: viola run takes --model and --effort, each refused as usage exit 2 when it fails its closed shape; a missing half takes the config default, else the compiled default.
@task_fact n=8 slug=occupied-resources
fact: viola revive takes --model and --effort, conflicting with --list; each half is the override, else the recorded half while it re-parses, else the start's default.
@task_fact n=9 slug=conventions
fact: A model or effort flag in the caller's words after the dashes, plain or equals form, is refused on run and revive as usage exit 2 with one fixed line naming the two flags and echoing none of the caller's words.
@task_fact n=10 slug=standard-contracts
fact: A wrapped list item, CLI and MCP, carries optional model and effort after role, each kept only when it re-parses at the board.
@task_fact n=11 slug=occupied-resources
fact: The compiled default pair is one named constant in the agent crate, DEFAULT_PAIR, and the effort words are the five of claude --help; no environment variable sets the pair.
@task_item n=1 part=untouched
The Hyprland window features and the compositor seam: nothing of the session reaches them.
@task_item n=2 part=untouched
viola verify, the capability ledger and its stamps, recorded fixtures and probes: no capability row is claimed or added and the relied rows are unchanged.
@task_item n=3 part=untouched
The wrapper channel protocol: no channel method, frame shape, fault code or protocol version moves.
@task_item n=4 part=untouched
Sockets, endpoint names, listening ports and the VIOLA_ environment namespace: none added or changed.
@task_item n=5 part=untouched
The exit code tables: no exit code is added or retired; the new refusal is the standing usage exit 2.
@task_item n=6 part=untouched
The dependency policy, toolchain pin, CI workflows and build-system rules: no dependency or tool moved.
@task_item n=7 part=untouched
The pinned binary copy and the plugin compiled into the binary: unchanged.
@task_item n=8 part=untouched
Link recording and the link set: unchanged; the launch line is no link event.
@task_item n=9 part=untouched
absence/model-api-calls and decision/no-model-api-calls: still true; the pair is two words handed to the unmodified claude CLI, no model API is called.
