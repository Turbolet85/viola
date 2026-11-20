@task_prose part=session
Under viola run, a keyless prompt that a claude-scheduled wake fires is now told apart from a typed one, so the driver keeps the wheel, its next send and its dialog (matrix v2-36).
The rule: a human-filed prompt-submitted the in-flight send did not claim is relabelled origin harness with no wheel move when no key was typed since the last human prompt, the last turn end counted at least one scheduled wake, and the CLI is verified; otherwise the human takes the wheel as before.
The wrapper's stdin observer now sets an in-memory typed mark on a key; it clears only when the next human-filed prompt takes it, never at a turn end or a release; the turn-ended scheduled_wakes count is now held and read by that rule.
The capability ledger gains its 26th relied row, stop-session-crons (Stop carries session_crons as an array; is_array only, no element read); the relied rows 25 to 26 and the floor rows 21 to 22; no verify step, IPC method, event kind, env var, CLI flag, config key or log line was added.
Test-only: the fake agent gains argv-only synthetic --stop-crons and --wake-after-ms; the live rig gains the WS, HB and R3W labels and their rehearsals, read live once on claude 2.1.328 under a scratch home (one interval-form wake fired, wake-origin harness, wheel-after-fire kept).
@task_file n=1 path=src/run/wheel.rs
what: the wheel state of viola run - the driver and human wheel holder, the stdin observer's human key move, now the typed mark and the held scheduled-wakes count
@task_file n=2 path=src/run/send/slot.rs
what: the wrapper's hook-event intake that appends events to the event log and decides prompt-submitted origin and the wheel move
@task_file n=3 path=src/run/send/relabel.rs
what: unit tests of the prompt-submitted origin relabel in the send path
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger - its relied rows, row ids and checks over recorded hook payloads, and the stamps verify writes
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: unit tests of the capability ledger rows
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit tests of each capability ledger row's check
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: unit tests of the ledger stamps and the verified or unverified reading of a stamped CLI
@task_file n=8 path=src/cmd/verify.rs
what: viola verify, the only stamps writer, its step lines and summary line; here its tests' row counts
@task_file n=9 path=src/run/version_gate.rs
what: the run start's version gate reading the ledger stamp into cli_verified; here its tests' row counts
@task_file n=10 path=src/bin/viola-fake-agent.rs
what: the fake agent, the test-only stand-in for claude used by CI verify and the integration tests
@task_file n=11 path=crates/viola-e2e/src/harness/run.rs
what: the e2e harness's run helpers, including the expected verify step lines
@task_file n=12 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig against a real claude, its labels, arms, stamp files and summary words
@task_file n=13 path=crates/viola-e2e/src/rig/wake.rs
what: the live rig's scheduled-wake legs and its reading of viola's own wheel records after a fired wake
@task_file n=14 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary, its rehearse words and help text
@task_file n=15 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsals over the fake agent
@task_file n=16 path=tests/cli_wheel.rs
what: integration tests of the wheel through the CLI - pause, release, a fired wake keeping the driver wheel
@task_file n=17 path=tests/tui_wheel.rs
what: e2e tests of the wheel under an outer PTY - a human key, a fired wake, a queued message
@task_file n=18 path=tests/cli_verify.rs
what: integration tests of viola verify's step and summary lines and stamps
@task_file n=19 path=tests/cli_verify/runs.rs
what: integration tests of viola verify runs and their failing-step sets
@task_file n=20 path=tests/cli_verify/stamps.rs
what: integration tests of the stamps viola verify writes and their summaries
@task_file n=21 path=tests/support/verify.rs
what: the test support's synthetic hook spine set fed to verify
@task_file n=22 path=tests/contract_ledger_probes.rs
what: the contract test that stamps every recorded fixture set against the ledger rows
@task_item n=1 part=touches
The wheel product event as the audit record: a keyless prompt told as a scheduled wake makes no wheel move and writes no wheel record; the human-takes-wheel path is unchanged otherwise.
@task_item n=2 part=touches
The wheel path's log events and the run.wheel_transition span: a told wake fires neither, so a wake no longer appears as a pause or a human transition.
@task_item n=3 part=touches
The wrapper send-refused human-typing detail: after a told wake the driver's next send is no longer refused for the human holding the wheel.
@task_item n=4 part=touches
The prompt-submitted origin on the event line: the wrapper now relabels a told wake origin harness; the event line shape and its closed origin words are unchanged.
@task_item n=5 part=touches
Typed keys never logged or in telemetry: the stdin observer now keeps a typed mark, a bool in memory only, never a key, never logged.
@task_item n=6 part=touches
The run log's cli_verified field and an unreadable ledger meaning an unverified CLI: cli_verified is now also a condition of the told-wake rule, an unverified CLI makes a wake take the wheel.
@task_item n=7 part=touches
Capability ledger contents never logged and verify lines carrying no content: the ledger gains the stop-session-crons row, its check reads is_array only and no element.
@task_item n=8 part=touches
viola verify's step line and summary line: the step lines run [NN/26] and a clean summary reads 26 pass 0 fail; any row holding 25 rows or 21 floor rows is now 26 and 22.
@task_item n=9 part=touches
verify the only stamps writer and the run log parse-rejected line for the ledger-stamps parser: the stamp gains the stop-session-crons row key, written by verify only; a missing key reads fail.
@task_item n=10 part=touches
CI verify against the fake agent only and real-CLI verify never in CI: the fake agent gains argv-only synthetic --stop-crons and --wake-after-ms; the real-CLI verify ran only in the live rig under a scratch home, outside CI.
@task_item n=11 part=touches
Recorded fixtures as no telemetry source and claude internals not a contract: Stop's session_crons is now relied on through a ledger row, every recorded set stamped 26 pass with no recording.
@task_item n=12 part=touches
Env vars not a configuration channel: the fake agent's new options are argv-only and read no env var.
@task_item n=13 part=touches
No telemetry from screen content: the live rig's HB arm reads a fresh box's hint cells as attributes and counts only, never text, test-only.
@task_fact n=1 slug=4-span-trace-coverage
fact: A keyless prompt told as a scheduled wake (no key typed since the last human prompt, a scheduled wake counted at the last turn end, a verified CLI) writes no wheel record and no wheel span; its prompt-submitted event carries origin harness.
@task_fact n=2 slug=span-trace-coverage
fact: The stdin typed mark and the held scheduled-wakes count are in-memory wrapper state, uninstrumented, carrying no key and no prompt text.
@task_fact n=3 slug=4-span-trace-coverage
fact: The stop-session-crons ledger row reads only whether Stop's session_crons is an array; no element of it is read, stamped or logged.
@task_item n=1 part=untouched
The MCP server, its tools, stdin frame limit and refusal results: no MCP tool or result moved.
@task_item n=2 part=untouched
Panic capture, the panic line and error-chain handling by role: no panic path or error message changed.
@task_item n=3 part=untouched
CI integration, secret scan, artifact upload and the schema, panic and perf gates: no CI step or gate changed.
@task_item n=4 part=untouched
Dialog hook, dialog log lines and the hook decision lines: the dialog path and hook stdout are unchanged.
@task_item n=5 part=untouched
Log file location, rotation, service identity, config.json keys and the process-log line fields: no field, file or key added.
@task_item n=6 part=untouched
Heartbeat and liveness, revive, link and unlink, and the socket and strict-modes refusals: none reached.
@task_item n=7 part=untouched
The OTel, exporter, error reporter and network egress absences: no telemetry sink or egress added.
