@task_prose part=session
Added a new CLI verb, viola switch <name> with --model and/or --effort and --json, which moves a running wrapped session to a given model and effort. The user's own settings are not written.
Added a new wrapper channel method, switch, taking sender, model and effort. It reads no from. It answers ok with the pair, or refuses with human-typing, turn-running, dialog-pending, unverified-cli, no-session, child-not-ended or resume-failed. A missing, wrongly typed or failing half is -32602 with data null.
The PTY pump can now respawn its child in place: it reaps the old child within a new 10 s RESPAWN_REAP_BOUND (PROVISIONAL, F6), then spawns claude --resume <id> on the new pair in a new pseudo-terminal under the same host terminal. A switch appends one launch line, plus the window line again where the snapshot holds one, and rewrites the snapshot's launch and child_pid.
The channel's closed method-label list grows from 10 to 11 with switch. While a switch stands, sends are refused turn-running. After the respawn, the old child's late session-end of the switched id is screened once.
Everything is proven over the fake agent alone and no capability is claimed. F1, F3, F5 and F6 stand PROVISIONAL on the delegate overseer's word. The real-CLI readings R6a and R6b are carried to the fleet on the founder's host.
@task_file n=1 path=src/cmd/switch.rs
what: new; the viola switch CLI verb, a short-lived cli role with its manual dispatch span cli.switch, its strict-modes and server checks before the frame, and its exit codes and refusal stderr pair
@task_file n=2 path=src/run/switch.rs
what: new; the wrapper's switch channel method: its param checks (-32602), its refusal order and refusal reasons, the launch append and the child respawn
@task_file n=3 path=tests/cli_switch.rs
what: new; integration tests of the viola switch verb over the fake agent
@task_file n=4 path=crates/viola-pty/src/pump.rs
what: edited; the PTY seam's pump, which now owns a respawned claude child: reap bound, kill of only its own child, the panic arm killing the current child
@task_file n=5 path=crates/viola-pty/src/lib.rs
what: edited; the PTY crate's public surface, now exporting the respawn handle and the reap bound
@task_file n=6 path=crates/viola-channel/src/server.rs
what: edited; the viola-channel server and its closed method-label list for channel-request and channel-response lines
@task_file n=7 path=src/cmd/run.rs
what: edited; viola run, the wrapper role: channel method dispatch and the claude child's pump
@task_file n=8 path=src/cmd/verify/typed/run.rs
what: edited; viola verify's spawn of the child under the PTY seam
@task_file n=9 path=src/cmd/mod.rs
what: edited; the CLI verb table and dispatch, where the role and the verb's span are chosen
@task_file n=10 path=src/run/mod.rs
what: edited; the wrapper's run module list
@task_file n=11 path=src/run/send.rs
what: edited; the wrapper's send handling and its send-refused turn-running decision
@task_file n=12 path=src/run/send/slot.rs
what: edited; the wrapper's send flight slot: the switch mark and the screen of a late session-end hook line after a respawn
@task_file n=13 path=src/human.rs
what: edited; the human-facing stdout and stderr renderer of the CLI
@task_file n=14 path=src/human/verbs.rs
what: edited; the human stdout lines of each verb, now with the switched line
@task_file n=15 path=src/human/hints.rs
what: edited; the stderr unable / hint pair table, now with four more refusal causes
@task_file n=16 path=tests/cli_output_plain.rs
what: edited; the test of the plain CLI output and of viola --help's served-verb list
@task_file n=17 path=tests/contract_error_bodies.rs
what: edited; the error-body contract census: channel methods and the CLI --json refusal documents
@task_file n=18 path=tests/cross_exit_causes.rs
what: edited; the cross-check of exit codes by refusal cause
@task_file n=19 path=tests/support/home.rs
what: edited; test support for the test-owned homes and the adopted wrapper
@task_item n=1 part=touches
The CLI process-log file rows, one per verb: viola switch is a new verb that logs as cli into cli-<name>.ndjson. No row holds it yet.
@task_item n=2 part=touches
The manual dispatch span per cli verb: cli.switch is a new span name.
@task_item n=3 part=touches
The process-exit detail rows for exit 1 by verb: switch adds no exit-1 detail of its own. Its exit 1 is the cli catch site's internal-error alone. A row expecting an exit-1-switch row is not carried.
@task_item n=4 part=touches
The rows on channel-request and channel-response lines and their closed method label: the label list grows from 10 to 11 (switch), so the switch pair names its method.
@task_item n=5 part=touches
The rows on channel invalid-params causes (-32602): the switch method answers -32602 with data null on neither half, a half of another type, or a half failing its check.
@task_item n=6 part=touches
The rows on channel params sender and the unauthenticated from: the switch method takes sender and reads no from.
@task_item n=7 part=touches
The rows on RefusalReason detail words and driver-side refusals ending a channel-response with result_class refusal and no word: switch keeps that shape. The four new words dialog-pending, no-session, child-not-ended and resume-failed appear in no diag line.
@task_item n=8 part=touches
The diag-line schema and the rule that removes schema words no build can emit: the schema is unchanged and none of the four new words was entered, because no line emits one.
@task_item n=9 part=touches
The wrapper send-refused turn-running detail: a send is now also refused turn-running from a switch's request until the new child's session-start.
@task_item n=10 part=touches
The run process log's claude-child process-start and process-exit lines: a switch ends the old child (exit through kill-fallback on the respawn's SIGHUP) and starts a new one, adding a claude-child pair per switch.
@task_item n=11 part=touches
The PTY seam log events (spawn, kill-fallback) and the pty.spawn span attribute row: the respawn runs under the existing pty.spawn span and reaps the old child through the kill fallback.
@task_item n=12 part=touches
The decision that the panic arm kills the child: the pump's panic arm now kills the current, possibly respawned, child.
@task_item n=13 part=touches
The launch-line-untraced and window-line-untraced decisions and the run start events order: a switch appends one launch line after the reap and before the respawn, then the recorded window line again. These are events-log appends outside a start.
@task_item n=14 part=touches
The rows on product readers reading the snapshot through replay: a switch rewrites the snapshot's launch and child_pid, and the switch reads pending_dialog, agent_session_id, launch and window through replay.
@task_item n=15 part=touches
The hook-invoked and hook-decision rows for session-end: after a respawn, the old child's late session-end of the switched id is screened once. No row says what a hook line after a respawn means.
@task_item n=16 part=touches
The NEVER-log floor rows (no params or results in channel lines, session id never logged, child settings argument never logged): the switch logs no pair, model or session id, and passes no --settings.
@task_item n=17 part=touches
The unreadable or unverified ledger rows and the unverified-cli refusal: the switch refuses unverified-cli from the start's cli_verified.
@task_item n=18 part=touches
The CLI stderr unable / hint pair rows: four new hint causes, with no hint naming viola release.
@task_item n=19 part=touches
The child exit from the process handle decisions: the pump now waits on and reaps a child it replaces, within a 10 s bound, and refuses child-not-ended past it.
@task_fact n=1 slug=span-trace-coverage
fact: viola switch logs as cli into cli-<name>.ndjson under the manual dispatch span cli.switch, with process-start and process-exit. A refusal ends process-exit with no detail, as send's does.
@task_fact n=2 slug=span-trace-coverage
fact: The wrapper's switch channel method (PROVISIONAL, F1) writes the channel-request and channel-response pair labelled switch, carrying no model, effort or session id.
@task_fact n=3 slug=span-trace-coverage
fact: A switch writes a claude-child process-exit for the old child and a claude-child process-start for the respawned one, under the existing pty.spawn span. viola writes no terminal byte at a respawn.
@task_fact n=4 slug=log-coverage
fact: The switch refusal words dialog-pending, no-session, child-not-ended and resume-failed are channel refusal details only. No diag line emits them, so the diag-line schema does not carry them.
@task_fact n=5 slug=span-trace-coverage
fact: A switch appends one launch line to the events log after the old child is reaped and before the respawn, plus the window line again where the snapshot holds one. The append has no trace of its own (PROVISIONAL).
@task_fact n=6 slug=log-coverage
fact: viola switch has no exit-1 refusal detail of its own. Its exit 1 is the cli catch site's internal-error.
@task_fact n=7 slug=span-trace-coverage
fact: After a respawn the wrapper screens one late session-end of the switched id while the new child lives. Whether a real CLI fires SessionEnd on SIGHUP is unproven (R6a, carried to the fleet on the founder's host).
@task_fact n=8 slug=span-trace-coverage
fact: A send is refused turn-running while a switch stands, from the switch's request until the new child's session-start (PROVISIONAL, F3).
@task_item n=1 part=untouched
The MCP stdio server, its tools and its frame-tool refusals: the switch is CLI-only (F5) and adds no MCP tool.
@task_item n=2 part=untouched
The hook role's fail-open, stdout and latency budget rows, and the hook perf gates.
@task_item n=3 part=untouched
viola verify's refusals, stdout step lines and spawn subjects: verify only passes a handle no one requests a respawn on.
@task_item n=4 part=untouched
The CI integration gates, the secret scan, artifact upload and retention, and the panic and schema-conformance gates.
@task_item n=5 part=untouched
The heartbeat, liveness and staleness rows, and config.json keys: no config key was added.
@task_item n=6 part=untouched
Link and unlink, the dialog spans and dialog log fields, wheel pause and release, wait and last.
@task_item n=7 part=untouched
The error-reporter, OTel, exporter and network-egress decisions and absences, and the metric coverage rows.
@task_item n=8 part=untouched
The service identity, process-start os and pid fields, and the log line field set: no field was added.
