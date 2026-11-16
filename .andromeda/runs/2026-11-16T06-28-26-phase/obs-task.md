@task_prose part=chunk
A driver's send of a local slash command that the claude CLI runs now reads as delivered (ok), confirmed by an echo of that command the product can observe, never presumed; a local command the CLI did not run is never confirmed by it.
Today an unlisted local command waits for a prompt-submitted that never fires and ends not-delivered with detail no-prompt-submitted, and a listed command with no measured post-condition reads ok with confirmed false and detail unconfirmable.
The echo's shape (a hook event, a screen line the vt100 readiness model reads, or another signal) and which commands it covers are re-measured from the code and the recorded claude fixtures before the plan states it; an echo relied on becomes a capability-ledger row with a viola verify probe.
The three readings of a local-command send today (listed post-conditions, unconfirmable, no-prompt-submitted) are re-read from the send path, the CLI send human mirror and the MCP send result.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path — readiness gate, confirmation window and the send-issued / send-confirmed / send-refused outcome a driver reads — edits: how a local command's send is confirmed by its echo
@task_file n=2 path=src/run/send/classify.rs mark=edits
what: the send classification that decides what confirms a send (prompt-submitted, a listed local command's post-condition, unconfirmable) — edits: the classification of a local slash command outside the compiled list
@task_file n=3 path=src/run/send/slot.rs mark=edits
what: the send's confirmation slot, the pending send awaiting its confirming fact
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of claude behaviours and the compiled local-command list with each command's post-condition — edits: the local-command list, a new ledger row for the echo if it is relied on
@task_file n=5 path=src/cmd/verify/ mark=edits
what: the viola verify command and its capability probes that stamp ledger rows — edits: the probe for the local-command echo row
@task_file n=6 path=src/run/version_gate.rs mark=edits
what: the version gate that reads the verified CLI and the ledger stamps at run start, deciding verified or unverified CLI
@task_file n=7 path=src/human/mirror.rs mark=edits
what: the CLI send human mirror, rendering a send's outcome and the fixed note for an unconfirmable send
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent that stands in for claude in tests, with its local-command mode — edits: its local-command mode's echo
@task_file n=9 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's scripted scenarios driving tests of the send path
@task_item n=1 part=needs
The send-confirmed outcome for a local slash command: delivered (ok) only on an observed echo of that command, never presumed; a local command the CLI did not run is never confirmed.
@task_item n=2 part=needs
The send-refused detail no-prompt-submitted no longer ends a send of a local command the CLI ran; the not-delivered refusal stays for a command not run.
@task_item n=3 part=needs
The unconfirmable send reading (ok, confirmed false, detail unconfirmable) and its fixed note in the human mirror: which local commands still read it once an echo is measured, and on an unverified CLI.
@task_item n=4 part=needs
The send log lines (send-issued, send-confirmed, send-refused) on the local-command path: their required fields, corr, detail as a closed code, and correlation with the channel lines.
@task_item n=5 part=needs
The confirmation window on a local-command send: its close and window_ms, beside the existing /clear and /compact closes and the ordinary-send close.
@task_item n=6 part=needs
The confirmed-send span chain and attributes (readiness gate, confirm window) as a local-command send passes through them.
@task_item n=7 part=needs
The capability ledger: an echo the product relies on is a ledger row with a viola verify probe, measured and stamped; an unreadable ledger or unstamped row means an unverified CLI and the send stays honest about what it cannot confirm.
@task_item n=8 part=needs
viola verify's probe for the echo row: its child spawn subject, its step line on stdout, its log lines carrying no content, and CI verify running against the fake agent only, never the real CLI.
@task_item n=9 part=needs
Claude shapes read only in viola-agent-claude: what counts as a local command's echo is parsed there and the send path consumes a normalised fact; wrapped claude internals and transcripts are not a contract.
@task_item n=10 part=needs
Upstream text — a command's output, a screen line read for the echo — is content: it never crosses a log line, an event or an error, and no telemetry is taken from screen content.
@task_item n=11 part=needs
Sent text and typed keys are never logged on the local-command path; the PTY byte stream is never logged.
@task_item n=12 part=needs
The MCP send result and the CLI send outcome read the wrapper's new reply for a local command without a change to the MCP tool surface's shape.
@task_item n=13 part=needs
The fake agent's local-command mode emits the measured echo so tests stand on it and the recorded fixtures; no live claude start is planned without asking first, and any live reading runs under a private CLAUDE_CONFIG_DIR inside a started session's turn.
@task_item n=14 part=needs
Parse failure of the echo or the ledger surfaces as unknown with a parse-rejected count, never as a guessed confirmation.
@task_item n=15 part=needs
The new tests' bounds: a test waiting on a channel or thread takes a bound and cannot hang past a failure; no sleep-based log waits; a test socket under the short /tmp root; a stamped home copied from the shared stamp.
@task_item n=16 part=needs
No mutation run in this chunk: each control's --timeout is named in the plan on an entry filtered to the guard's own cases, a control id within 16 characters; acceptance thresholds are checked at P4 against the class's ci and mutants kills.
@task_standing n=1
convention: every new log line goes through the single obs_event macro with a closed event value, the required fields and corr where correlated; no raw tracing or print macros
home: 11-obs-anti-patterns-never-do-these
@task_standing n=2
convention: every new diagnostics line conforms to the diagnostic line schema, additive only, checked by the schema conformance gate on real process output in integration and E2E tests
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: every new integration test home lives under the scanned test-home root and is kept until the gates run
home: ci-integration
@task_standing n=4
convention: every new detail code is a closed code carrying no path, pid or content
home: 4-span-trace-coverage
@task_standing n=5
convention: every new span is declared with skip_all and an explicit field allow-list, a static name, and no err or ret option
home: span-trace-coverage
@task_standing n=6
convention: every new test leaves zero panic lines and does not wait by sleeping on a log
home: 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
The dialog paths: dialog-raised, dialog-answered, hook decisions on a dialog, dialog hold and expiry; the chunk leaves them untouched.
@task_item n=2 part=not_needed
The wheel: human-typing pauses, pause and release, release-from-driver; the wheel's rules are untouched.
@task_item n=3 part=not_needed
wait and last: their wake rule, outcomes, connect refusals and span chain.
@task_item n=4 part=not_needed
Run start: collision check, pinned copy, pty spawn, start failure details (already-live, squatted-name, stale-heartbeat, pinned-hash-mismatch), the run start span chain and start events order.
@task_item n=5 part=not_needed
Revive and its preflight, the claude agents probe and its parse, link and unlink records.
@task_item n=6 part=not_needed
Heartbeat ticks, liveness, list status and staleness thresholds.
@task_item n=7 part=not_needed
State recovery: snapshot replay, torn-line healing, the state-recovered line.
@task_item n=8 part=not_needed
Socket-directory, strict-modes and server-verify refusals and the exit-21 cause words.
@task_item n=9 part=not_needed
Hook performance budgets and the spine and session-end hook perf gates.
@task_item n=10 part=not_needed
Panic outcome by role, panic capture and backtrace placement.
@task_item n=11 part=not_needed
The MCP stdin frame limit and oversize-line handling, and MCP refusals for unreachable instances.
@task_item n=12 part=not_needed
Future error reporters, OTel exporters, network egress and hosted deployment.
