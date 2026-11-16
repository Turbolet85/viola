@task_prose part=chunk
Builds a trim-tolerant send match: a driver's send whose prompt the claude CLI submits with whitespace trimmed is claimed by the in-flight send, the prompt-submitted line relabelled driver, the send confirmed ok and the wheel kept with the driver.
The match tolerates exactly the whitespace the CLI trims and no other byte; a rule naming that trim lives in the claude agent crate and the send path consumes it as a normalised predicate.
The driver's text is validated as received and typed without removing that whitespace: a control character is still refused not-delivered / control-character, never stripped, and an empty typed text stays empty-text.
Closes the readiness-gate turn race: a turn starting while a send waits on the pre-send readiness gate never receives the text; the send is refused turn-running or typed only after that turn ends, and a test reproduces the race.
Re-measures from the code and the recorded claude fixtures what the product compares and what the CLI's submitted prompt shows of the trim; no live claude start is planned.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path — the pre-send readiness gate (settled-input-box predicate), the turn rung check, the paste of the typed text and the send outcome reply — edits: the readiness gate, the turn rung check before the paste
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the in-flight send slot that claims a prompt-submitted hook event for the pending driver send, fed by the hook event append, and the human wheel move — edits: the slot claim comparison, the hook event append
@task_file n=3 path=src/run/send/relabel.rs mark=edits
what: the relabel of a claimed prompt-submitted line from human to driver origin and the wheel's human move
@task_file n=4 path=src/run/send/classify.rs
what: the local-command list (/clear, /compact) classification of a send, read from the exact typed text, and its post-conditions
@task_file n=5 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the claude-shaped hook payload rules in the claude agent crate, including the typed-text rule (CR LF and CR as one LF, trailing CR and LF not typed) — edits: typed_text, a new predicate naming the whitespace the CLI trims
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent that stands in for the claude CLI in tests and CI, driven by fake scripts, emitting hook events and turns
@task_item n=1 part=needs
The send-confirmed event and the confirmed send log fields: a trimmed driver prompt now ends in send-confirmed with outcome ok instead of a not-delivered exit, and the send path log events keep their order (send-issued, channel lines, hook-invoked, send-confirmed).
@task_item n=2 part=needs
The wheel path: a matched trimmed prompt makes no wheel transition to the human, so no human-typing wheel line or wheel product event is written for it and no later driver send is refused human-typing; a human's own prompt still moves the wheel as before.
@task_item n=3 part=needs
The unconfirmable send and the no-prompt-submitted refusal: a prompt differing in any byte other than the trimmed whitespace still claims nothing and the send line reads as it does today.
@task_item n=4 part=needs
Client-side send validation refusals control-character and empty-text: the text is validated as received, a refused character is refused not-delivered / control-character and never removed, an empty typed text stays empty-text.
@task_item n=5 part=needs
The wrapper send-refused detail turn-running and the readiness gate: a turn starting inside the gate's window refuses the send turn-running (or the text is typed only after the turn ends); input-not-ready keeps its meaning; the run.readiness_gate span covers the closed window.
@task_item n=6 part=needs
The send-refused detail words stay a closed RefusalReason set: the race closure uses an existing code, and a send refusal is logged by the side that decides it, each with the send's corr.
@task_item n=7 part=needs
Correlation on send lines: the relabelled prompt-submitted hook line and the send-confirmed line carry the corr of the in-flight send it matched (corr by event for send and hook lines, conn and rpc_id on the send line).
@task_item n=8 part=needs
Indirect, through the trim comparison: sent text and the CLI's submitted prompt are never logged nor put in a span field or detail code; the match records only its outcome, and text_bytes stays the byte count of the text as typed.
@task_item n=9 part=needs
Indirect, through the claude boundary: the trim is an undocumented behaviour of the wrapped claude CLI, its internals are not a contract; the claude-shaped rule lives only in the claude agent crate and the send path calls it as a normalised predicate.
@task_item n=10 part=needs
Indirect, through the hook payload: the prompt field of a UserPromptSubmit payload is parsed as received; a payload that fails to parse surfaces as unknown with a parse-rejected count, never as a match.
@task_item n=11 part=needs
The confirm window: run.confirm_window closes on an ordinary send matched by the trim-tolerant claim as on an exact match; the /clear and /compact closes still read the exact typed text.
@task_item n=12 part=needs
The CLI send and MCP send tool outcomes read the wrapper's reply unchanged: ok for a confirmed send, the refusal codes as isError refusal, exit 13 not-delivered only where the wrapper refused.
@task_item n=13 part=needs
The recorded claude fixtures (fixtures/claude/2.1.325 and older) are re-read for the submitted prompt's shape; they are scrub-required and never a telemetry source.
@task_item n=14 part=needs
The fake agent submits a trimmed prompt or starts a turn inside the readiness gate's window so CI proves both behaviours against the fake agent only; real-CLI verify never runs in CI.
@task_item n=15 part=needs
New tests that wait on a channel, a thread or a log line take a bound and never hang past a failure; no sleep-based log waits.
@task_item n=16 part=needs
If a live claude start were ever needed to read the trim's shape, it runs under a private CLAUDE_CONFIG_DIR and only after asking; stripped CLAUDE values are never logged.
@task_standing n=1
convention: every diagnostics line an integration or E2E test makes a process emit is schema-checked against the diag-line schema
home: ci-integration
@task_standing n=2
convention: integration-test homes live under the one scanned e2e root and are kept until the panic, schema and secret-scan gates run
home: 9-ci-integration
@task_standing n=3
convention: a test that waits for a log line or an event waits on a bounded condition, never a sleep, and asserts budgets by exit code
home: 10-slo-invariants-telemetry-budgets
@task_standing n=4
convention: any new log line goes through the obs_event macro with a closed event value, explicit corr and instance, and only allow-listed typed fields
home: 11-obs-anti-patterns-never-do-these
@task_standing n=5
convention: new fixtures or fake scripts carry synthetic inputs only and are no telemetry source
home: pii-scrubbing-compliance
@task_item n=1 part=not_needed
Dialog paths: dialog-raised and dialog-answered lines, dialog spans, the dialog hook's fail-open and deadline rules; the chunk leaves the dialog paths untouched.
@task_item n=2 part=not_needed
The wait and last paths: wait log lines, wait spans and the wait wake rule; untouched by this chunk.
@task_item n=3 part=not_needed
The run start sequence: start spans and attributes, exit-1 start detail codes, pinned copy, version gate, collision checks and the run start events order.
@task_item n=4 part=not_needed
Revive, revive --list and the session-live preflight, and the claude agents probe and its parse rules.
@task_item n=5 part=not_needed
Link and unlink methods, their events and refusals, and the release-from-driver refusal.
@task_item n=6 part=not_needed
Heartbeat file, liveness and staleness thresholds, list --json liveness.
@task_item n=7 part=not_needed
Home-tree and socket-directory strict-modes refusals, server-verify refusals and the exit 21 unreachable cause words.
@task_item n=8 part=not_needed
State recovery: snapshot replay, torn-line healing, state-recovered lines and the harness logs command's torn-line handling.
@task_item n=9 part=not_needed
Panic capture outcomes by role, the panic line and backtrace capture; the chunk adds no catch site.
@task_item n=10 part=not_needed
Future error reporter, OTel exporters, network egress, metrics pipelines and deployment absences.
@task_item n=11 part=not_needed
Artifact upload, secret-scan hit reports, mutation outputs upload and artifact retention in CI.
