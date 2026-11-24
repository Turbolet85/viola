@task_prose part=chunk
Adds a CLI verb `viola adopt <session-id> --name <n> [--parent <p>]`: it starts a viola wrapper whose claude child resumes a plain claude session, so it stands on the board as a named instance with its lineage.
It is built on the revive start path: the resume argv, the `claude agents --json` live-elsewhere preflight with its closed session-live refusal, and the start shared with run.
A malformed session id, a session live elsewhere, a second adopt of the same id and an adopt onto a held name each end in a defined closed refusal; viola never ends or signals a process it did not start.
Folded CI item: it re-measures two env-capture security-negative cases against the per-test kill line, locally and under coverage, and gives them a margin by their test class or by less work, never by raising the kill line for every test.
@task_file n=1 path=src/cmd/revive.rs
what: the viola revive start path: the resume argv, the claude agents preflight with its session-live refusal, and the start shared with run, which adopt is built on
@task_file n=2 path=.config/nextest.toml mark=edits
what: the test runner configuration holding the per-test slow and kill timeouts and the test classes — edits: the class or override for the two env-capture security-negative cases
@task_item n=1 part=needs
The new adopt verb's own process logging as a short-lived cli verb: a CLI process-log file entry for adopt, written only under a parsed name, with its process-start and process-exit lines and the fixed stderr refusal and fault lines.
@task_item n=2 part=needs
The adopt start, once its preflight passes, logs as a run start, as the revive start does: the run process log lines for self and the claude child, the run start spans and the run start events in events.ndjson in their order.
@task_item n=3 part=needs
The preflight order the adopt start keeps, as revive's does: the closed session-id shape check before any home read beyond the strict ones, the home-tree and strict-modes checks before the start's log opens, then the live-elsewhere read, all before any child is spawned; a refused check before the log opens writes no line.
@task_item n=4 part=needs
The live-elsewhere read through `claude agents --json`: its agents-probe spawn subject and process log lines, its output untrusted, a parse failure read as unknown with a parse-rejected count, its rows never logged verbatim, and the session-live refusal with its exit-1 detail code.
@task_item n=5 part=needs
New closed refusal words adopt may add: a malformed session id, a second adopt of the same id, an adopt onto a name already held; each a distinct exit-1 detail code carrying no path or pid, added to the process-exit detail words and the diagnostic schema additively, with its unable and hint stderr pair and its --json error document.
@task_item n=6 part=needs
The resume argv crossing into the claude child: the process-start line of the claude child with its env fields as names and counts only, the stripped CLAUDE values never logged, the child settings argument never logged, and the session id carried only as a typed field, never as content.
@task_item n=7 part=needs
The name and parent landing as the instance's first snapshot and its lineage record: the lineage line has no trace of its own, and parent and role stay confined to product surfaces, out of process logs.
@task_item n=8 part=needs
The instance comes from the operator-given name, never from the working directory; where cwd, model and effort have no recorded source, no telemetry is taken from screen content or from wrapped claude internals.
@task_item n=9 part=needs
A refusal of a live session spawns no child and sends no signal; any terminal or window line adopt's start writes follows the existing start window line, which has no trace of its own.
@task_item n=10 part=needs
Adopt's tests drive a fake agent only: the real claude CLI never runs in CI, and no real session, operator home or real claude data is read; any live reading is gated on the operator.
@task_item n=11 part=needs
The folded CI item: a measured time per case for the two env-capture security-negative cases against their kill line, locally and under coverage, with what each waits on said from the code; the margin comes from the case's class or reduced work, no retry and no re-run substitutes for it, and no sleep-based log wait is added.
@task_item n=12 part=needs
The env-capture cases' own promise stays: an env name with the user word fails its read and records nothing, and a kept name reaches no stamp or record; the R8 strip is logged as names only.
@task_item n=13 part=needs
Adopt's panics follow the panic outcome by role for cli, cli --json and run: one line, the exit decided at the catch site.
@task_standing n=1
convention: every new log line goes through the single obs_event macro with a closed event value, corr set explicitly, fields from the default-deny allow-list, and no paths, content or secrets
home: 11-obs-anti-patterns-never-do-these
@task_standing n=2
convention: every new span is manual, declared with skip_all and an explicit field allow-list, with a static low-cardinality name
home: 4-span-trace-coverage · span-trace-coverage
@task_standing n=3
convention: every new integration or E2E test keeps its home under the scanned test root, has its emitted lines schema-checked, and lets the panic and schema gates run after it
home: 9-ci-integration · ci-integration
@task_standing n=4
convention: every new fixture of claude output is synthetic and scrub-required and is no telemetry source
home: 1-obs-scope-summary
@task_standing n=5
convention: every new test that waits on a log line or a time does so without sleep-based waits and asserts its budget by exit code
home: 10-slo-invariants-telemetry-budgets · slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The send path: readiness gate, confirm window, paste cut, send log fields and send refusals; adopt sends no text.
@task_item n=2 part=not_needed
The dialog and hook paths: dialog-raised and answered lines, hook stdout, hook decisions, hook latency and perf budgets.
@task_item n=3 part=not_needed
The wheel, pause and release paths and the wait and last paths with their spans and log fields.
@task_item n=4 part=not_needed
Channel JSON-RPC lines, conn and srv_conn joins, and the link and unlink methods and their events.
@task_item n=5 part=not_needed
Future error reporters, OTel exporters and SDKs, network egress and the hosted-deployment absences; adopt adds no exporter.
@task_item n=6 part=not_needed
The harness logs and agent-run logs and status output, and the snapshot-paste integration.
@task_item n=7 part=not_needed
The secret-scan hit report and the fuzz and mutation upload rules; this chunk adds no upload.
@task_item n=8 part=not_needed
viola up's reboot reading with its boot-time-unread detail, the fleet console, and model and effort switching: out of scope.
@task_item n=9 part=not_needed
PTY paste and kill-fallback lines and vt100 feed containment beyond the existing start.
