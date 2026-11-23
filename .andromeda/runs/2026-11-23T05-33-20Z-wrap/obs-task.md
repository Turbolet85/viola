@task_prose part=session
The session added a lineage registry: viola run takes --parent NAME and --role WORD, else reads the inherited VIOLA_NAME as the parent only, and a parent equal to the instance's own name is a usage error, exit 2, before anything is read.
Every run and revive start now writes three records before the hook's session-start instead of two: the wheel start line, then one lineage line (source wrapper, data parent and role), then session-start; the snapshot gains lineage and the replay recovers it as a fourth field.
The lineage kind is the sixteenth product event kind (15 to 16), log-only, waking no wait; it writes no process-log line, no span and no terminal byte.
viola list draws wrapped rows as a tree under their parents (NAME indented two spaces a depth, six captions unchanged) and a separate -- LINKS -- block of driver -> driven lines; list --json and MCP list items gain parent and role, and ok gains links, each name re-parsed and dropped when not a name.
revive keeps the recorded parent and role; the test harness supervisor now drops the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN when it spawns a wrapper, so a harness session started inside a wrapped session records no parent.
Nothing was retired; no config key, env var, closed word, dependency or process-log event was added.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: edited; the closed product event-kind list of events.ndjson (each kind's data keys, the wait wake kinds) in viola-core, which emits no telemetry
@task_file n=2 path=crates/viola-state/src/snapshot.rs
what: edited; the instance snapshot.json shape in the state crate
@task_file n=3 path=crates/viola-state/src/replay.rs
what: edited; the snapshot replay from the events log, the fields a replayed snapshot recovers
@task_file n=4 path=crates/viola-state/src/board.rs
what: edited; the board read model behind viola list, list --json and the MCP list tool (items, liveness, links)
@task_file n=5 path=crates/viola-state/src/liveness.rs
what: edited; the instance liveness and heartbeat staleness reading (a snapshot literal gained lineage)
@task_file n=6 path=crates/viola-state/src/strict.rs
what: edited; the home strict-modes check (a snapshot literal gained lineage)
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: edited; the state crate's events-log tests (the per-kind list)
@task_file n=8 path=crates/viola-state/tests/state_replay.rs
what: edited; the state crate's replay tests (a snapshot cut short replayed with the logged lineage)
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs
what: edited; the viola mcp stdio server tool handler tests
@task_file n=10 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the observability harness boot readiness, which reads the start records before session-start
@task_file n=11 path=crates/viola-e2e/src/harness/supervise.rs
what: edited; the observability harness supervisor's wrapper spawn and the environment it hands on
@task_file n=12 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited; the harness session boot, status, logs and teardown test
@task_file n=13 path=src/cmd/run.rs
what: edited; the viola run start (arguments, start order of snapshot and events lines, the instance source of run)
@task_file n=14 path=src/cmd/revive.rs
what: edited; the viola revive start, which starts as run does
@task_file n=15 path=src/cmd/list.rs
what: edited; the viola list command and its --json document
@task_file n=16 path=src/cmd/hook.rs
what: edited; the viola hook role (snapshot literals in its tests gained lineage)
@task_file n=17 path=src/human/board/rows.rs
what: edited; the terminal rendering of the viola list board rows, the unwrapped rack and the links block
@task_file n=18 path=src/human/board/cells.rs
what: edited; the terminal rendering of one viola list board row's cells
@task_file n=19 path=src/human/board/test_support.rs
what: edited; test support for the viola list board rendering
@task_file n=20 path=src/run/mod.rs
what: edited; the viola run session core (a snapshot literal gained lineage)
@task_file n=21 path=src/run/wheel.rs
what: edited; the wheel, human-typing pause and release in viola run (a test literal gained lineage)
@task_file n=22 path=src/run/dialog.rs
what: edited; the dialog path in viola run (a test literal gained lineage)
@task_file n=23 path=src/run/send/slot.rs
what: edited; the send slot of the confirmed send path in viola run (a test literal gained lineage)
@task_file n=24 path=tests/cli_list.rs
what: edited; the viola list integration tests (tree, json parent and role, links block)
@task_file n=25 path=tests/cli_revive.rs
what: edited; the viola revive integration tests (revive keeps parent and role)
@task_file n=26 path=tests/cli_links.rs
what: edited; the link and unlink integration tests (lineage changes no wheel, link or send)
@task_file n=27 path=tests/cli_instance_state.rs
what: edited; the instance state tests, the start's events records before the spawn
@task_file n=28 path=tests/chaos_snapshot_replay.rs
what: edited; the snapshot replay chaos tests
@task_file n=29 path=tests/hook_events.rs
what: edited; the hook events tests counting the start's records in events.ndjson
@task_file n=30 path=tests/path1_run_start.rs
what: edited; the run start path tests, session-start's position among the start records
@task_file n=31 path=tests/hook_fail_open.rs
what: edited; the hook fail-open tests
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: edited; the server identity security negative tests
@task_file n=33 path=tests/support/home.rs
what: edited; the integration-test stamped home and wrapper boot support
@task_file n=34 path=tests/contract_lineage_label.rs
what: added; the census check that lineage is read by no wheel, link, send or answer path
@task_item n=1 part=touches
The run start events order in events.ndjson moves from wheel then session-start to wheel, lineage, session-start; the row naming the order is stale.
@task_item n=2 part=touches
The start product order read from the events log now holds three records before the hook's session-start, on every run and revive start.
@task_item n=3 part=touches
The instance source fixed per role: run now reads the inherited VIOLA_NAME a second time, as the self-reported default parent only, never as its instance, dropped when equal to its own name.
@task_item n=4 part=touches
The rule that environment variables are not a configuration channel gains the inherited VIOLA_NAME read as a parent default; no env var or config key was added.
@task_item n=5 part=touches
Self-reported from is not an identity: the inherited or flagged parent is equally self-reported and authenticates nothing.
@task_item n=6 part=touches
Link pair names never in a process log, and link names stay out of process logs: the link pairs now reach list --json, MCP list and the terminal -- LINKS -- block, and still no process-log line.
@task_item n=7 part=touches
The MCP list tool's payload gains item parent and role and ok links, additive, with no v move.
@task_item n=8 part=touches
Event payloads kept out of process logs: the lineage line's parent and role are written to events.ndjson and the snapshot only.
@task_item n=9 part=touches
The event stream line shape gains a sixteenth kind, lineage, source wrapper, data parent and role each a string or null.
@task_item n=10 part=touches
A snapshot rebuilt by replay and the state-recovered replay now recover lineage as a fourth field beside links, agent session id and wheel.
@task_item n=11 part=touches
viola revive start logs as run: revive shares the run start and writes the same lineage line, keeping the recorded parent and role.
@task_item n=12 part=touches
viola run silent on the terminal, and the run start spans: the lineage line adds no terminal byte, no span and no span attribute to the start chain or the snapshot write.
@task_item n=13 part=touches
Components left uninstrumented gain the lineage write, the run --parent and --role parsing, the board tree and the links block, each with no line or span by rule.
@task_item n=14 part=touches
Control characters escaped in terminal rendering: the tree indent and the -- LINKS -- lines pass through the same row escaping and the existing tokens only.
@task_item n=15 part=touches
Run terminal output start refusal and exit codes: a --parent equal to the name is now a clap usage error, exit 2, before anything is read, a refusal before any log opens.
@task_item n=16 part=touches
Hook silent without an instance and the hook decision rows read a start whose session-start is now the third record; the hook's own behaviour is unchanged.
@task_item n=17 part=touches
The observability harness boot readiness now reads three start records; the harness supervisor drops the inherited VIOLA_* names on its wrapper spawn, read against rows on test-started viola and harness logs.
@task_item n=18 part=touches
The list --json liveness contract stands, while a list item's keys grow to up to ten with parent and role when present.
@task_item n=19 part=touches
Liveness transitions not logged and the heartbeat staleness rows: the liveness reading gained only the lineage field in its literal, no new line.
@task_fact n=1 slug=span-trace-coverage
fact: The lineage events line written at every run and revive start is log-only product audit data: it wakes no wait and writes no process-log line, span or terminal byte.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: Parent, role and link pair names reach only events.ndjson, the snapshot, the viola list board, list --json and the MCP list result, each name re-parsed as a viola name and each role as a code word, dropped otherwise; never a process-log or detail line.
@task_fact n=3 slug=otel-sdk-init
fact: viola run takes the inherited VIOLA_NAME as its default parent, a self-reported claim that is no identity, dropped when it equals the instance's own name, and a test-started wrapper from the harness carries no inherited VIOLA_* names.
@task_item n=1 part=untouched
Panic capture, panic lines and error reporting: no catch site, panic hook or reporter changed.
@task_item n=2 part=untouched
CI gates, the secret scan, artifact uploads and pinned actions: no gate or job changed.
@task_item n=3 part=untouched
Performance budgets, SLO invariants and hyperfine perf gates: no budget or threshold moved.
@task_item n=4 part=untouched
Metric coverage and metrics derived by jq: no counter, gauge or grouping changed.
@task_item n=5 part=untouched
Log levels, log file locations, log rotation, config.json keys and the diagnostics line schema and obs_event enum: none changed.
@task_item n=6 part=untouched
OTel, exporters, network egress and the other absences: none added or removed.
@task_item n=7 part=untouched
Channel JSON-RPC methods, corr and conn propagation, and the channel lines: no method or field changed.
@task_item n=8 part=untouched
viola verify, plugin install, the capability ledger and the agents probe: untouched.
