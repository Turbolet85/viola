@task_prose part=chunk
Adds a viola verb that switches a running viola-wrapped session, a named instance, to a given model and effort.
It records the new pair so that a later resume (revive, up) replays the new pair, not the one named at launch.
The verb has no side effect on the user's own settings: it writes nothing to the user's claude home or a project's settings, and does not use the CLI's own model command as its channel.
It adds the verb's argument surface, closed words, exit codes, hints, output lines, --json document, the events-log record of the switch and its process-log lines, each beside the existing tables.
Nothing live: no real session is read, switched, ended or resumed, and no real settings file is read in this phase.
@task_file n=1 path=src/cmd/revive.rs
what: the viola revive verb, the restart start path whose --model and --effort override the recorded pair on the resumed child's argv; a restart-based switch may ride it
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: the claude agent's closed effort set (is_effort_level: low, medium, high, xhigh, max) and the compiled default model and effort pair
@task_item n=1 part=needs
A new short-lived CLI verb, the switch, addressed to a named instance: its arguments, its closed model and effort words, its exit codes and the closed detail code each exit cause carries, beside the existing CLI verb and exit-cause tables.
@task_item n=2 part=needs
The switch verb's human output: lines on stdout for success, and on a refusal the fixed unable and hint pair on stderr, interpolating no path, pid or content.
@task_item n=3 part=needs
The switch verb's --json document, for success and for each refusal, including an internal-error document and an instance-unreachable document with its one cause word when the named instance is dead, lost or not wrapped.
@task_item n=4 part=needs
The switch verb's CLI process-log file and lines: process-start and process-exit in the cli role file, written only under a parsed instance name, with a distinct closed detail code per exit cause; the instance is taken from the name, never the working directory.
@task_item n=5 part=needs
A new events-log record of the switch carrying the new model and effort pair, in the event line shape, emitted deterministically; the event payload stays out of the process logs.
@task_item n=6 part=needs
The recorded pair a resume replays: the launch line and the snapshot's launch field, read by product readers through replay, so that revive and up start the resumed child on the last recorded pair.
@task_item n=7 part=needs
Where the switch restarts the session through the revive start path with a new pair, that path's rules bind it: revive start logs as run, the revive preflight order and its cwd-missing, no-session and session-live details, the run start process log and spans, and the claude-child process-start line.
@task_item n=8 part=needs
Model and effort words are closed sets validated before any action: a word outside them is refused with a closed detail code, never echoed back in a log line or error message.
@task_item n=9 part=needs
Any new log field naming the model or effort is an additive, allow-listed, closed-enum field in the diagnostics line schema; the field allow-list is default-deny and existing field names stay frozen.
@task_item n=10 part=needs
No side effect on settings: the verb writes no user-scope or project settings file; the child settings argument and the stripped CLAUDE environment values passed on a restart are never logged.
@task_item n=11 part=needs
The home strict-modes and home-tree checks run before the verb's log opens; a refusal before the log opens writes no line, and socket-directory and server-verification refusals keep their fixed codes if the verb reaches the instance's endpoint.
@task_item n=12 part=needs
If the switch reaches the live wrapper over the channel, its channel-request and channel-response lines carry no params or result bodies, correlate by corr and conn, and the wrapper's refusal reasons stay closed detail words.
@task_item n=13 part=needs
The verb's dispatch gets one manual span with a static name, skipping all arguments, with an explicit field allow-list and no endpoint, path or env values; spans are not serialized.
@task_item n=14 part=needs
Errors in the verb are typed with fixed Display messages, anyhow only at the dispatch edge, the full error chain only in the detail file; serde errors become the fixed internal-error message; a cli panic follows the cli panic outcome.
@task_item n=15 part=needs
Every line the verb writes goes through the obs_event macro with a closed event value; no print macros and no raw tracing macros in its code paths; log level never from the environment.
@task_item n=16 part=needs
The verb's tests run against the fake agent only, never the real CLI and never the operator's real home or board; integration-test homes stay under the scanned test root and their emitted lines pass the schema conformance and zero-panics gates.
@task_item n=17 part=needs
Session ids the switch reads or passes to resume are typed fields, never logged in content or interpolated into messages.
@task_standing n=1
convention: each new log line or event is emitted through the obs_event macro with a closed event value, corr where correlated, and carries no content, path or secret
home: 11-obs-anti-patterns-never-do-these
@task_standing n=2
convention: each new log field is additive to the diagnostics line schema, named under the additive naming rule and admitted to the default-deny allow-list
home: 6-log-coverage · log-coverage
@task_standing n=3
convention: each new span declares a static name, skips all arguments and lists its allowed fields explicitly
home: 4-span-trace-coverage · span-trace-coverage
@task_standing n=4
convention: each new integration or end-to-end test keeps its home under the scanned test root until the gates run, and its emitted lines are schema-checked and counted by the panic gate
home: 9-ci-integration · ci-integration
@task_standing n=5
convention: each new fixture or test input is synthetic, never recorded from a real session or the operator's real home
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_item n=1 part=not_needed
The hook role's own rows: hook stdout bodies, hook latency and perf budgets, hook fail-open and hidden capture arms; the switch adds no hook and changes no hook behaviour.
@task_item n=2 part=not_needed
The list verb's liveness output and heartbeat staleness, and the agent-run status and logs wrapper rows; the switch neither lists nor reports fleet status.
@task_item n=3 part=not_needed
The verify verb's own rows: its step and summary stdout, its child spawn subjects, its stamp writing and its exit codes; the switch does not run or change verify.
@task_item n=4 part=not_needed
Link and unlink, lineage and implicit-link rows; the switch records no link between instances.
@task_item n=5 part=not_needed
The wait and last verbs' spans, log fields and connect refusals; the switch is not a wait or a readback.
@task_item n=6 part=not_needed
Plugin install rows and the future opt-in error reporter rows (transport, egress entry, off by default); the switch installs nothing and reports nowhere.
@task_item n=7 part=not_needed
OTel, Sentry, OTLP, exporter, network egress, database, gRPC, deployment and on-call absences; the switch adds none of these and they need no new row.
