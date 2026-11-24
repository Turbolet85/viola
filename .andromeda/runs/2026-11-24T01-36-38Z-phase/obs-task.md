@task_prose part=chunk
Builds a new `viola focus <name>` CLI verb in the root binary: resolve the home, validate the name, read the instance's state under the strict-modes checks.
It finds the session's window by one of two sources: the window record the wrapper keeps in the snapshot for a session `viola open` started, with its address re-parsed through the Hyprland seam's validator, or, for a hand-started `viola run`, the wrapper's process ancestry matched against the compositor's client list.
It shows the window's workspace and brings the window forward, using only the Hyprland seam's closed request set (a workspace-focus dispatch and a focus-window dispatch).
Every no-window outcome ends in a named refusal, and outside Hyprland the verb refuses as unsupported.
Tests run on the fake compositor (recorded Hyprland 0.56.2 replies) and the fake agent; the help, exit-cause and verb tables gain the new verb. Nothing live runs in this phase.
@task_file n=1 path=src/cmd/ mark=edits
what: the root binary's CLI verbs, the cli role's dispatch edge — edits: a new focus verb beside the existing verbs, and the verb, help and exit-cause tables
@task_item n=1 part=needs
A new short-lived cli verb, `viola focus`. It is a cli role process that writes its own process log, and only under a name parsed as a ViolaName. It gets one manual dispatch span, a process-start line and a process-exit line carrying an exit code and a detail code.
@task_item n=2 part=needs
The instance comes from the name the verb is given, never from the working directory. The home is resolved in the fixed order, and the home-tree and strict-modes checks run before the cli log opens. A strict-modes refusal that comes before the log opens writes no line and leaves with the existing strict-modes-failed exit codes and stderr pair.
@task_item n=3 part=needs
Reading the instance's state: the verb reads the snapshot's window record (address, workspace, placement, origin) through replay, never raw. A snapshot rebuilt by replay carries no live endpoint. Entries the reader rejects or skips (symlinks, unknown keys, torn lines) surface as parse-rejected counts, never as content.
@task_item n=4 part=needs
The Hyprland seam as a child process: the compositor probe is the hyprctl-probe spawn subject. Its JSON replies (the client list, dispatch replies) are untrusted. A reply or a recorded window address that fails to parse surfaces as unknown, with a parse-rejected line for the hyprctl-json parser and a closed detail word.
@task_item n=5 part=needs
New refusal causes: no window found, a recorded window that is gone, and unsupported outside Hyprland. Each needs its own closed exit detail code, distinct per cause, with no path, pid, window address or workspace in it. Each reaches stderr as a fixed unable/hint pair, and reaches the --json error document as a closed code.
@task_item n=6 part=needs
The ancestry arm reuses the state crate's in-memory walk of at most 8 parent pids. That walk is instrumented synchronously without Tokio. The pids it walks and the window addresses it matches are never logged as values in detail codes or in home-level lines.
@task_item n=7 part=needs
Only typed low-class fields are logged on the focus path: the window address, workspace, client list rows and process ancestry stay out of home-level lines. Anything content-bearing goes only to the instance detail file. Error chains reach stderr as fixed messages, with no serde source in them.
@task_item n=8 part=needs
A panic in the cli role while focusing: the panic hook is installed first, the panic is recorded as one line, and the exit is decided at the catch site. The panic outcome for the cli role (human and --json) applies to the new verb.
@task_item n=9 part=needs
The cli role's stdout carries only the verb's result, with no telemetry on it. Terminal rendering of any compositor-supplied text escapes control characters.
@task_item n=10 part=needs
Each Hyprland dispatch is issued once and never retried: retry policies and retry-once are forbidden. No new time bound is added while the operator's fork on time bounds stands.
@task_item n=11 part=needs
The tests read the recorded Hyprland replies through the fake compositor. Those fixtures are not a telemetry source. Every line the new tests make the binary emit is checked against the diagnostic schema, and the zero-panics gate reads those lines. The tests wait on logs by condition, never by sleep.
@task_item n=12 part=needs
Linux is the only OS target. On a host that is not running Hyprland, focus must refuse as unsupported and must not start the compositor probe.
@task_standing n=1
convention: every new log line goes through the single obs_event macro, with a closed event value and the required fields, corr passed explicitly, and no raw tracing or print macros
home: logging-stack
@task_standing n=2
convention: every new span is a manual span with a static area.operation name, skips all of its arguments and declares an explicit field allow-list, with no err or ret options
home: 4-span-trace-coverage · span-trace-coverage
@task_standing n=3
convention: every new process-exit or refusal detail is a closed code word, distinct per cause, with no path or pid in it
home: log-coverage
@task_standing n=4
convention: every new integration or E2E test keeps its home under the scanned test-home root until the gates run, and its emitted lines are schema-checked
home: ci-integration
@task_standing n=5
convention: every new fixture holds synthetic, scrubbed input only and is never read as telemetry
home: pii-scrubbing-compliance
@task_standing n=6
convention: every new logged field is added under the default-deny allow-list, additive only, with no paths, content or secrets
home: 8-pii-scrubbing-compliance
@task_item n=1 part=not_needed
Hook role rows (hook stdin and stdout, fail-open, hook decision lines, hook latency and perf budgets). Focus raises no dialog and runs no hook.
@task_item n=2 part=not_needed
Channel, send, wait, last, dialog and wheel path rows (JSON-RPC frames, send confirmation, wait spans, pause and release). Focus sends no frame to the wrapper.
@task_item n=3 part=not_needed
viola-mcp stdio server tool rows. Focus is a CLI verb only, and adding an MCP focus tool is a boundary widening this chunk does not make.
@task_item n=4 part=not_needed
viola run start rows (PTY spawn, claude child env strip, pinned copy, version gate, run start spans, the confirm window). Focus starts no session and spawns no claude child.
@task_item n=5 part=not_needed
verify, agents-probe and revive rows (verify spawn subjects, verify stdout, agents parse failures, revive preflight). Focus probes no agent and revives nothing.
@task_item n=6 part=not_needed
Link, unlink and lineage rows. Focus records no link and writes no lineage.
@task_item n=7 part=not_needed
Future error reporter rows (opt-in, ureq and rustls, egress entry). There is no reporter in v1 and this chunk adds none.
