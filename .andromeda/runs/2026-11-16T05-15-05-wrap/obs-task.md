@task_prose part=session
The send's delivery-confirmation claim no longer needs exact equality. It takes the submitted prompt when it equals the typed text, or when it is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed. The tolerance is PROVISIONAL, a boundary widening that is escalated and still waits for the founder's word.
A driver's send whose text ends in whitespace the CLI trims is now relabelled driver and confirmed. It keeps the wheel, a refused character is still refused, and the relabel still writes origin alone and keeps the submitted text.
The readiness-gate turn race is closed by a lock. After the gate returns ready, the human-typing and turn-running reads, the implicit link record, send-issued and the paste all run under one flight lock. A turn whose prompt-submitted reaches the wrapper first refuses turn-running: nothing typed, no link, no send-issued. The confirmation window waits with the lock released.
The hook-event path makes the claim, the session-start claim, the turn start and end and the unsent-human wheel move under that one flight lock. The test-only fake agent gains the argv-only flag --trim-prompt-end.
No event kind, log line, log field, detail word, refusal word, IPC method, env var, config key or crate was added. The send-issued, send-confirmed and send-refused lines are unchanged, and no text is logged.
A residual stays open: a turn the CLI started whose UserPromptSubmit has not reached the wrapper before the paste is beyond reach, because hook latency is unmeasured.
@task_file n=1 path=crates/viola-agent-claude/src/hook.rs
what: the hook component, which parses the hook payload and normalises the prompt text; it now holds the predicate that decides whether a submitted prompt is the typed text (the delivery-confirmation match)
@task_file n=2 path=src/run/send/slot.rs
what: the send slot in viola run, which holds the in-flight send, the prompt claim that confirms a send, the session-start claim and the turn and wheel state moved on hook events
@task_file n=3 path=src/run/send.rs
what: the wrapper side of send in viola run, which runs the readiness gate, the refusal order (human-typing, turn-running), the implicit link record, send-issued, the paste and the confirmation window
@task_file n=4 path=src/run/send/relabel.rs
what: the driver relabel of a hook event, which marks a confirmed send's prompt-submitted origin driver
@task_file n=5 path=src/run/send/test_support.rs
what: test helpers for the send path's unit tests
@task_file n=6 path=src/bin/viola-fake-agent.rs
what: the test-only fake agent that stands in for claude in CI, with its argv-only options and its UserPromptSubmit payload
@task_file n=7 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's argv option parse tests
@task_file n=8 path=tests/cli_send.rs
what: the booted integration tests of viola send against the fake agent
@task_item n=1 part=touches
A row that says the send-confirmed line or a confirmed send follows an exact match of the prompt to the typed text is stale. The match now also accepts the typed text with a trailing ASCII whitespace run removed, PROVISIONAL.
@task_item n=2 part=touches
The unconfirmable send on the send line: a driver send whose text ends in whitespace the CLI trims is now confirmed and logs send-confirmed, where before it went unconfirmed.
@task_item n=3 part=touches
The wrapper send-refused turn-running detail: it now also covers a turn whose prompt-submitted arrives after the readiness gate but before the paste. In that case no send-issued line and no link record are written.
@task_item n=4 part=touches
The send path log events order and the implicit-link-before-send-issued event order: the order is unchanged, but the link record, send-issued and the paste now run under one flight lock held after the gate.
@task_item n=5 part=touches
The confirmation-window close for an ordinary send and the run.confirm_window span: the window now waits with the flight lock released. Its window_ms attribute and close rules are unchanged.
@task_item n=6 part=touches
The readiness-gate span and its attributes (run.readiness_gate): the post-gate reads now happen under the flight lock. The span and its fields are unchanged.
@task_item n=7 part=touches
Send refusal logged by each deciding side: the refusal order and the words are unchanged. A refusal drops the lock first and is recorded as before (no cursor, no link, no send-issued).
@task_item n=8 part=touches
The send-refused human-typing contract and the human-typing wheel log line: the unsent-human wheel move now runs under the flight lock. A human key takes no flight lock and never waits on it.
@task_item n=9 part=touches
The driver-relabel and hook-event rows: the hook-invoked and send-confirmed send log fields are unchanged, and the relabel still sets origin alone.
@task_item n=10 part=touches
Sent text is never logged, and there are no PII or content fields: the new predicate logs nothing. text_bytes on send-issued is kept.
@task_item n=11 part=touches
The hook latency budget and hook correlation: the open residual is a turn whose UserPromptSubmit reaches the wrapper only after the paste. Hook latency is unmeasured, and its owner is the standing live rig with a screen model.
@task_item n=12 part=touches
CI verify against the fake agent only, and the fake agent rows: the fake agent gains the argv-only --trim-prompt-end option. It reads no env, so env vars are still not a configuration channel.
@task_item n=13 part=touches
Recorded fixtures are no telemetry source: no recorded claude fixture changed, and the drift contract stays byte for byte.
@task_fact n=1 slug=snapshot-paste-to-ai-integration
fact: A driver send is confirmed, and logs send-confirmed, when the submitted prompt equals the typed text or is non-empty and equals the typed text with a trailing run of ASCII space, TAB, CR or LF removed. This is a PROVISIONAL tolerance awaiting the founder's word, and either end is held as the candidate.
@task_fact n=2 slug=span-trace-coverage
fact: After the readiness gate, the human-typing and turn-running reads, the implicit link record, send-issued and the paste run under one flight lock. So a turn known to the wrapper before the paste always refuses turn-running with no send-issued line. A turn whose UserPromptSubmit reaches the wrapper only after the paste stays undetected; that residual is unmeasured.
@task_item n=1 part=untouched
Diagnostics line schema, field names, the closed event enum, detail words and the corr-by-event table: no event kind, field or detail word was added or changed.
@task_item n=2 part=untouched
Panic capture, error capture and reporting, panic outcome by role and the panic gates: no panic path or error type changed.
@task_item n=3 part=untouched
Log sinks, file locations, modes, levels, the logging stack, init order and service identity: none changed.
@task_item n=4 part=untouched
Dialog, wait, last, link and unlink methods, MCP tools and frames, verify, revive and run start: none of their paths, lines or exit codes changed.
@task_item n=5 part=untouched
CI integration gates, the secret scan, artifact uploads, perf budgets and SLO thresholds: no gate, threshold, budget or CI step changed.
@task_item n=6 part=untouched
Absences such as the OTel exporter, network egress and the metrics pipeline, and the PII scrubbing regime: no capability was added or removed.
@task_item n=7 part=untouched
Heartbeat, liveness and state recovery rows: no heartbeat, snapshot or replay behaviour changed.
