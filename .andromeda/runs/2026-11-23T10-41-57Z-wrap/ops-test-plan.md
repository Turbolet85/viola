@row_set n=1 row=contract/run-start-events/order cell=promise seen=3 detector=D-tests-count-literals
value: The events log reads wheel with cause start, then lineage with source wrapper, then launch with source wrapper, then the agent's session-start with source hook.
basis: fct-w2kbdeoj
because: The start records per start moved 3 to 4, a launch line written by the wrapper after the lineage line, so the order the row states is now short of one record.
@row_set n=2 row=contract/boot-readiness/events cell=promise seen=3 detector=D-tests-count-literals
value: Event lines 1, 2, 3 and 4 are a wheel line with cause start, a lineage line with source wrapper, a launch line with source wrapper, then session-start with source hook; missing reads events.
basis: fct-tsianv6s
because: The harness readiness probe now reads four complete start lines with launch third, so the three lines the row names no longer match the probe.
@row_set n=3 row=contract/viola-list-json/items cell=promise seen=3 detector=D-tests-count-literals
value: In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?}.
basis: fct-xb6c4igy
because: The list --json item gains model? and effort? after role, so the item's key list the row promises is short of two keys.
@row_set n=4 row=decision/snapshot-replay-fields cell=rule seen=3 detector=none
value: A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id, wheel, lineage and launch with dialog_pending false, the last line of each winning, so a later lineage line with no parent clears it; the replay writes no file and gives no cwd.
basis: fct-ydz7awsz
because: The replay's recovered field set gains the launch pair with the last launch line winning, so the fixed field set the row names is no longer the whole set.
@row_set n=5 row=decision/bounded-readers-keep-the-cap cell=rule seen=4 detector=none
value: The version gate drain, the hyprctl reader, the dialog hold and env-keep readers and the config.json default-pair reader keep no more than the frame cap of what they read; a command the cut ends is a failed command.
basis: fct-objkygma
because: The config.json default-pair reader is a new bounded reader held to the frame cap and added as a row of the BOUNDARIES list, so the readers the rule names are no longer all of them.
@row_set n=6 row=contract/fake-agent-receipt/start cell=promise seen=2 fact=F1 detector=none
value: Carries cli_version, pid, the model and effort it was handed on argv, and the print-mode prompt; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
basis: fct-ydz7awsz
because: The fake agent takes --model and --effort on argv only and its start receipt now carries them, the proof that the pair was handed to the child.
@row_set n=7 row=decision/snapshot-replay-from-log cell=rule seen=4 fact=F7 detector=none
value: Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the wheel, the agent session id and the launch pair included, with no dialog pending, and write no file.
basis: fct-nz2usoww
because: The replay now recovers the launch pair, proved through list --json after a killed wrapper's cut snapshot since a revive over a replayed snapshot is refused cwd-missing, so the recovered state the row names is short of the pair.
@row_set n=8 row=decision/rstest-case-tables cell=except seen=2 fact=F9 detector=D-tests-framework
value: A crate with no rstest dev-dependency (viola-e2e, viola-core) writes its table-driven cases, and a shared helper's own cases taken by path, which run in every taker, as labelled tables in one test.
basis: fct-mdae3bw6
because: A table case was written as one literal table in a single test because its crate carries no rstest dev-dependency, which makes the exception a rule for any such crate rather than for two named ones.
@row_add n=1 row=contract/event-data/launch area=integration-test-strategy fact=F2 detector=none
boundary: Event data keys
item: launch
promise: data holds the keys model and effort, both closed words; the wrapper writes the line with source wrapper, and the kind is log-only, never a wake kind.
basis: fct-ydz7awsz
because: The chunk adds the 17th event kind launch with data keys model and effort, kept out of the wake kinds, and the event-data contract has no item for it.
@row_add n=2 row=contract/config-json/default-model area=integration-test-strategy fact=F3 detector=none
boundary: config.json
item: default_model
promise: A string of the closed model shape, a lowercase letter, then lowercase letters, digits, dots or dashes, 1 to 64 bytes; a missing or bad value takes the model half of the compiled DEFAULT_PAIR.
basis: fct-xb6c4igy
because: config.json gains the default_model key with a closed shape and a fallback to the compiled pair, a contract the test-plan states nowhere.
@row_add n=3 row=contract/config-json/default-effort area=integration-test-strategy fact=F3 detector=none
boundary: config.json
item: default_effort
promise: One of the five effort levels of EFFORT_LEVELS; a missing or bad value takes the effort half of the compiled DEFAULT_PAIR.
basis: fct-xb6c4igy
because: config.json gains the default_effort key as an enum of the effort levels with a fallback to the compiled pair, a contract the test-plan states nowhere.
@row_add n=4 row=contract/viola-run-start/launch-pair area=6-e2e-test-strategy fact=F4 detector=none
boundary: viola run start
item: launch pair
promise: viola run --model and --effort set the start's pair handed to the child; each missing half comes from the config.json default, else the compiled DEFAULT_PAIR.
basis: fct-ere5pjev
because: viola run gains --model and --effort, the default pair and the given pair both handed and recorded at the start, and no row of the start contract says where the pair comes from.
@row_add n=5 row=contract/run-revive-usage/pair-after-dashes area=6-e2e-test-strategy fact=F5 detector=none
boundary: run and revive usage
item: pair after the dashes
promise: A --model or --effort, or its equals form, in the caller's words after the dashes is usage exit 2 for run and for revive, with one fixed line naming both flags and echoing none of the caller's words.
basis: fct-dgfm4vwr
because: run and revive now refuse a pair in the caller's words with a fixed usage line, a promise at the CLI boundary that no row holds.
@row_add n=6 row=decision/revive-launch-pair area=test-scope-summary fact=F6 detector=none
name: Revive hands an override, recorded or default pair
rule: viola revive hands the revived child each half of the pair from its --model or --effort override, else from the recorded half while it still parses, else from the start's default half, and the new start records its own launch line.
guard: revive_given_pair_prefers_the_override_then_the_record
state: provisional
basis: fct-ckuwa7cb
because: viola revive now hands the revived child a launch pair chosen half by half, a decision the test-plan's revive rows do not state.
@row_add n=7 row=decision/list-launch-pair-re-parsed area=6-e2e-test-strategy fact=F8 detector=none
name: List shows the launch pair only re-parsed
rule: A list item keeps its model only when it re-parses as a model word and its effort only when it re-parses as a code word, otherwise the key is absent, and the human board's six columns do not show the pair.
guard: board_read_drops_a_launch_word_that_does_not_parse
state: provisional
basis: fct-ydz7awsz
because: The board now carries the launch pair on the list item under a re-parse check, a rule the list rows do not state.
