@task_prose part=session
Every viola start now names a model and an effort: viola run takes --model and --effort, and where either is missing it takes the default pair from config.json default_model and default_effort, else the compiled DEFAULT_PAIR. The pair is handed to the claude child's argv after the plugin flag, and words before the caller's own.
The pair is recorded per instance. A new log-only event kind, launch {model, effort} with source wrapper, is written after the lineage line, and the first snapshot carries an optional launch field. Replay recovers the pair from the last launch line, and the list --json and MCP list items gain model and effort after role, each re-parsed at the board.
viola revive takes --model and --effort too. It hands the override half, else the recorded half while it still parses, else the start's default half, and the new start writes its own launch line. A pair in the caller's words after the dashes is refused as usage exit 2, with one fixed line that names the two flags and echoes none of the caller's words.
Counts moved: event kinds 16 to 17; start records per start 3 to 4 (wheel, lineage, launch, then session-start); config.json known keys 5 to 7; the controls-not-disableable settings list 9 to 11; the bounded-readers boundary list gains one row. The fake agent takes --model and --effort on argv only, and its start receipt gains model and effort.
The planned chaos case, where a revive over a corrupt snapshot still hands the pair, was ruled a deviation. A replayed snapshot carries no cwd, so revive refuses it as cwd-missing. What is proved instead is that the replay recovers the pair, read by list --json after a killed wrapper's cut snapshot. No capability is claimed, and no real claude session was measured running on the pair: tests show it handed, never that the session runs on it.
@task_file n=1 path=src/run/launch.rs
what: added — the start's launch pair: the compiled default pair, the bounded config.json default-pair reader, the resolve of each missing half, and the caller-words pair check with its fixed refusal line
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited — the agent crate's claude flag literals and argv builders: --model and --effort, the five effort levels, the default pair and the launch argv
@task_file n=3 path=crates/viola-core/src/lib.rs
what: edited — the core closed shapes and the normalised event kinds: the model-word shape and the 17th kind launch, not a wake kind
@task_file n=4 path=crates/viola-state/src/snapshot.rs
what: edited — the instance snapshot: an optional launch field {model, effort}
@task_file n=5 path=crates/viola-state/src/replay.rs
what: edited — snapshot replay from the events log: the recovered field set gains the launch pair, the last launch line wins
@task_file n=6 path=crates/viola-state/src/board.rs
what: edited — the list board's item: model and effort after role, each kept only when it re-parses
@task_file n=7 path=crates/viola-state/tests/state_replay.rs
what: edited — the state replay integration tests: a snapshot cut short is replayed with the logged launch pair
@task_file n=8 path=crates/viola-state/tests/state_events.rs
what: edited — the event-kind table tests: every kind, now 17 with launch
@task_file n=9 path=src/run/mod.rs
what: edited — the wrapper's child spawn: the claude child argv takes the plugin flag, then the launch pair, then the caller's words
@task_file n=10 path=src/cmd/run.rs
what: edited — the viola run verb: --model and --effort parsing, the pair-after-dashes usage refusal, and the start state writing the launch line after the lineage line
@task_file n=11 path=src/cmd/revive.rs
what: edited — the viola revive verb: --model and --effort overrides, the pair-after-dashes usage refusal, and the pair given to the revived child from override, record or default
@task_file n=12 path=src/cmd/list.rs
what: edited — the viola list verb's JSON document: the item carries the launch pair
@task_file n=13 path=src/obs.rs
what: edited — the config.json known keys list: 5 to 7 with default_model and default_effort
@task_file n=14 path=schemas/config.v1.json
what: edited — the config.json schema: default_model (closed model shape) and default_effort (enum of five)
@task_file n=15 path=src/bin/viola-fake-agent.rs
what: edited — the fake agent: argv-only --model and --effort options, and the start receipt carrying model and effort
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited — the fake agent's option-parsing unit cases: model and effort taken
@task_file n=17 path=crates/viola-e2e/src/harness/boot.rs
what: edited — the harness boot readiness probe: the start records read as four complete lines, wheel, lineage, launch, session-start
@task_file n=18 path=tests/cli_instance_state.rs
what: edited — the instance state integration tests: the start's snapshot and events-log records, now with launch
@task_file n=19 path=tests/hook_events.rs
what: edited — the hook events integration tests: the start-record count read as four
@task_file n=20 path=tests/path1_run_start.rs
what: edited — the run-start critical path E2E: the default pair and the given pair handed and recorded
@task_file n=21 path=tests/cli_revive.rs
what: edited — the viola revive integration tests: the recorded pair handed, the override handed and recorded, an unparsable recorded word taking the default half
@task_file n=22 path=tests/chaos_snapshot_replay.rs
what: edited — the snapshot replay chaos tests: the replay reads the launch pair the wrapper wrote after a killed wrapper
@task_file n=23 path=tests/cli_list.rs
what: edited — the viola list integration tests: list --json carries model and effort, the item key lists moved
@task_file n=24 path=tests/run_cli.rs
what: edited — the viola run usage tests: a bad effort and a pair after the dashes are usage exit 2
@task_file n=25 path=tests/cli_controls_not_disableable.rs
what: edited — the controls-not-disableable check: its settings list 9 to 11, setting the default pair disables no control
@task_file n=26 path=tests/contract_bounded_readers.rs
what: edited — the bounded-readers boundary list: one row added for the config.json default pair
@task_file n=27 path=crates/viola-state/src/liveness.rs
what: edited — the liveness classifier's unit tests: an instance snapshot literal gains an empty launch
@task_file n=28 path=crates/viola-state/src/strict.rs
what: edited — the strict-modes unit tests: an instance snapshot literal gains an empty launch
@task_file n=29 path=crates/viola-mcp/tests/handlers.rs
what: edited — the MCP handler tests: an instance snapshot literal gains an empty launch
@task_file n=30 path=src/cmd/hook.rs
what: edited — the viola hook verb's unit tests: instance snapshot literals gain an empty launch
@task_file n=31 path=src/run/dialog.rs
what: edited — the wrapper's dialog unit tests: an instance snapshot literal gains an empty launch
@task_file n=32 path=src/run/wheel.rs
what: edited — the wrapper's wheel unit tests: an instance snapshot literal gains an empty launch
@task_file n=33 path=src/run/send/slot.rs
what: edited — the wrapper's send slot unit tests: an instance snapshot literal gains an empty launch
@task_file n=34 path=src/human/board/test_support.rs
what: edited — the human board's test support: the list item literal gains model and effort
@task_file n=35 path=tests/hook_fail_open.rs
what: edited — the hook fail-open integration tests: an instance snapshot literal gains an empty launch
@task_file n=36 path=tests/security_negatives_server_identity.rs
what: edited — the server-identity security negatives: an instance snapshot literal gains an empty launch
@task_file n=37 path=tests/cli_fake_agent.rs
what: edited — the fake agent's CLI tests: the start receipt key pin gains model and effort
@task_item n=1 part=touches
The run start events order goes from three records to four: wheel, lineage, launch with source wrapper, then the hook's session-start. The first record of a start is still wheel.
@task_item n=2 part=touches
Boot readiness on events now reads four complete start lines, where it read lines 1 to 3. The missing code name:events is unchanged.
@task_item n=3 part=touches
The fixed field set that snapshot replay recovers gains the launch pair. The last launch line whose two members are both strings wins, and a line with a non-string member reads none.
@task_item n=4 part=touches
The viola list --json ok.items shape gains optional model and effort after role. A word that no longer parses is dropped at the board. The MCP list mirror carries the same, so the rows on CLI and MCP returning identical payloads cover the new keys.
@task_item n=5 part=touches
The fake-agent receipt's start record gains the model and effort keys.
@task_item n=6 part=touches
The fake-agent command line's argv-only options gain --model and --effort. The fake agent configured by argv alone holds for them.
@task_item n=7 part=touches
In the agent-run status document, the list items now carry model and effort, read by the boot smoke.
@task_item n=8 part=touches
Under controls not disableable and no switch disables a security control, the settings list goes from 9 to 11. Setting the default pair disables no control.
@task_item n=9 part=touches
Bad config value falls back to its default: a bad default_model or default_effort takes that half of the compiled default pair. It writes one parse-rejected line with parser config-json and detail malformed per key.
@task_item n=10 part=touches
Every external reader is bounded, and bounded readers keep the frame cap. The boundary list gains a config.json default_model and default_effort row, read under MAX_FRAME.
@task_item n=11 part=touches
On the event line and its event kinds and data keys: a new 17th kind, launch, with data keys model and effort and source wrapper. It is log-only.
@task_item n=12 part=touches
The wait wake kinds, the channel wait wake kinds and the non-waking records of viola wait: launch is not a wake kind.
@task_item n=13 part=touches
On viola run start and the snapshot's run fields: the launch line and the first snapshot's launch field are written before the child spawn.
@task_item n=14 part=touches
viola run checks before the child: a pair in the program words after the dashes is refused as usage exit 2 before any spawn. The refusal is one fixed line naming --model and --effort and echoing none of the caller's words.
@task_item n=15 part=touches
The revive exit code 2 and the revive preflight now include the same pair-after-dashes refusal. --model and --effort conflict with --list.
@task_item n=16 part=touches
The revive cwd-missing refusal and the rule that a replayed snapshot has no endpoint or cwd stand unmoved. They are why a revive over a replayed snapshot cannot hand the recorded pair. The plan's chaos criterion is amended to say the replay recovers the pair instead.
@task_item n=17 part=touches
Broken or unreadable snapshot replayed from the log, and list reads a live wrapper through the replay: a replayed row now carries the logged launch pair.
@task_item n=18 part=touches
A fake-agent run is not proof of real-CLI behaviour. The pair is proved handed on the child's argv, never that a real claude session runs on it, and the real-CLI reading is carried.
@task_item n=19 part=touches
Table-driven unit cases in rstest gain an exception: the model-word shape cases are one literal table in a single test, because viola-core carries no rstest dev-dependency.
@task_item n=20 part=touches
One negative test per security control and the bounded remove-the-guard witness: three controls are added and each holds. They are launch-last-wins, caller-words and revive-pair.
@task_item n=21 part=touches
On the claude child holding nothing of viola's and the child env strip: the child's argv now carries viola-chosen --model and --effort words. Nothing is read from the environment and the env strip is unchanged.
@task_item n=22 part=touches
Every CLI-version fixture set replayed, and the fake agent held to recorded fixtures. The five effort levels are fixed from claude --help at 2.1.330, with no fixture recorded for them.
@task_fact n=1 slug=test-data-fixtures
fact: The fake agent takes --model and --effort on argv only, and its start receipt carries them as model and effort. That is the proof that the pair was handed to the child.
@task_fact n=2 slug=integration-test-strategy
fact: The launch event's data keys are model and effort, both closed words. It is written by the wrapper with source wrapper, is log-only and is never a wake kind.
@task_fact n=3 slug=integration-test-strategy
fact: config.json default_model takes the closed model shape: a lowercase letter, then lowercase letters, digits, dots or dashes, 1 to 64 bytes. default_effort is one of the five effort levels. A missing or bad key takes that half of the compiled DEFAULT_PAIR.
@task_fact n=4 slug=6-e2e-test-strategy
fact: viola run --model and --effort set the start's pair. Each missing half comes from the config default, else the compiled DEFAULT_PAIR.
@task_fact n=5 slug=6-e2e-test-strategy
fact: A --model or --effort, or its equals form, in the caller's words after the dashes is usage exit 2 for run and for revive. The refusal is one fixed line naming both flags and echoing none of the caller's words.
@task_fact n=6 slug=test-scope-summary
fact: viola revive hands the revived child a pair taken from the override half, else the recorded half while it still parses, else the start's default half. The new start records its own launch line.
@task_fact n=7 slug=e2e-test-strategy
fact: A revive over a replayed snapshot cannot hand the recorded pair because it is refused cwd-missing. The replay's recovery of the pair is proved through list --json after a killed wrapper's cut snapshot.
@task_fact n=8 slug=6-e2e-test-strategy
fact: The board keeps a list item's model only when it re-parses as a model word, and its effort only when it re-parses as a code word. Otherwise the key is absent, and the human board's six columns do not show the pair.
@task_fact n=9 slug=2-test-strategy
fact: A crate without an rstest dev-dependency writes its table cases as one literal table in a single test rather than adding the dependency.
@task_item n=1 part=untouched
CI integration, meaning workflows, SHA-pinned actions, cargo-deny bans, zizmor, the cache and the toolchain pins: no dependency, tool or workflow moved.
@task_item n=2 part=untouched
Mutation testing gates, verdicts, thresholds and kill bounds: no mutation run was made and no threshold moved.
@task_item n=3 part=untouched
Performance budgets, hyperfine invocation and the hook spine deadlines: no time bound moved.
@task_item n=4 part=untouched
Fuzzing targets, corpora and replay gates: no parser fuzz target was added.
@task_item n=5 part=untouched
Coverage floors, exclusions and the coverage threshold: none moved.
@task_item n=6 part=untouched
Secret scan classes, hits and artifacts: unchanged.
@task_item n=7 part=untouched
The process log line format, its corr fields and its log events: the launch line opens no span and writes no process-log line.
@task_item n=8 part=untouched
The pre-push stages, details and reasons, the gate breaches and the gate exit codes: unchanged.
@task_item n=9 part=untouched
Fixture recording, scrub, hygiene walk and verify stamping: no fixture was recorded and no stamp rule moved.
@task_item n=10 part=untouched
The CLI exit-code table beyond usage exit 2: no new exit code, refusal word, hint or closed word.
@task_item n=11 part=untouched
Harness session record, cleanup document, supervisor and the harness exit codes: unchanged.
