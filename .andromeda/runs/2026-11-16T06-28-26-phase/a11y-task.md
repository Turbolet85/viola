@task_prose part=chunk
Makes a driver's send of a local slash command that the CLI runs read as delivered (ok), confirmed by an echo of that command the product can observe, never presumed; a local command the CLI did not run is never confirmed by it.
Today an unlisted local command waits for a prompt-submitted the CLI never fires and ends not-delivered / no-prompt-submitted although it ran; a listed command with no measured post-condition, or on an unverified CLI, reads ok with confirmed false, detail unconfirmable.
Re-measures from the code and the recorded payloads what echo the CLI gives for a local command and which commands it covers, before the plan states it; an echo relied on becomes a capability-ledger row with a viola verify probe.
Re-reads what a send of a local command reads today on the send path, its classifier, the CLI send human mirror and the MCP send result.
Stands on the fake agent's local-command mode and recorded payloads; no live claude start is planned.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path, which decides a send's outcome (ok, not-delivered) and its confirmation — edits: the local-command send outcome, consuming a normalised echo fact
@task_file n=2 path=src/run/send/classify.rs mark=edits
what: the send classifier that decides what confirms a send (a prompt-submitted or a local command's post-condition) — edits: the local-command classification
@task_file n=3 path=src/run/send/slot.rs mark=edits
what: the send slot that waits within the send window for the confirming signal
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the claude capability ledger, the only home of claude shapes — edits: the local-command list, a new ledger row if the echo is relied on
@task_file n=5 path=src/cmd/verify/ mark=edits
what: viola verify, the probes that measure and stamp the capability-ledger rows — edits: the probe of the local-command echo
@task_file n=6 path=src/run/version_gate.rs mark=edits
what: the version gate deciding whether the running CLI is verified, so whether a ledger fact may confirm a send
@task_file n=7 path=src/human/mirror.rs mark=edits
what: the CLI send human mirror, the human-readable send outcome lines — edits: unconfirmable's fixed note
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent standing in for claude in tests — edits: its --local-command-mode
@task_file n=9 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's scripts that drive the send tests
@task_file n=10 path=fixtures/claude/ mark=re-reads
what: the recorded claude payloads per CLI version, re-measured for the echo a local command gives
@task_item n=1 part=needs
The send outcome of a local slash command: a local command the CLI runs reads ok, confirmed by its own observable echo; one the CLI did not run is never confirmed by it.
@task_item n=2 part=needs
The send refusal no-prompt-submitted: an unlisted local command that ran no longer ends with it; it still holds for an ordinary send whose prompt-submitted never comes.
@task_item n=3 part=needs
The unconfirmable reading (ok with confirmed false) and its fixed note on the CLI send human mirror: a send stays honest about what it cannot confirm where no echo is measured or the CLI is unverified.
@task_item n=4 part=needs
The critical path of a confirmed send and its readback: the driver reads the send's confirmation from the wrapper's reply.
@task_item n=5 part=needs
Every dependence on an undocumented claude behaviour is a capability-ledger row with a viola verify probe, measured and stamped; the probe's result prints as a viola verify step line.
@task_item n=6 part=needs
Claude shapes only in the claude agent crate: what counts as a local command's echo is read there, and the send path consumes only a normalised fact.
@task_item n=7 part=needs
Upstream text (a local command's output) is vendor content, never a viola command or config, and crosses no log, event or error line; the single vendor-content boundary holds.
@task_item n=8 part=needs
If the echo is read from the screen, no screen-content parsing beyond signatures: the echo is matched as a signature the vt100 readiness model already reads, never by interpreting claude's content.
@task_item n=9 part=needs
The CLI send under --json and the MCP send result read the wrapper's reply as they already do; the MCP tool surface's shape is unchanged, and a --json send outcome stays one clean document.
@task_item n=10 part=needs
Any new detail or code word on a send outcome is a kebab-case code beside a plain word, its fixed string pinned by literal, with one hint per code where it is a refusal.
@task_item n=11 part=needs
Time bounds fall on the driver only: the send window that waits for an echo bounds the driver's call, never the human.
@task_item n=12 part=needs
Tests stand on the fake agent and recorded payloads; a live reading never gates; no live claude start without asking first, and any live start runs under a private config dir inside a started session's turn.
@task_item n=13 part=needs
A test that waits on a channel or thread takes a bound and cannot hang past a failure; a test socket goes under the short /tmp root; a stamped home comes from the shared stamp's copy.
@task_item n=14 part=needs
No mutation run this phase: each control's --timeout is named in the plan, filtered to the guard's own cases, its id within 16 characters; acceptance thresholds are checked at P4 against the class's ci and mutants kills.
@task_standing n=1
convention: each new test runs inside the existing nextest suites as an ordinary test; its failure is a nextest failure, with no separate a11y suite and no skip
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new test is deterministic, zero flakiness and no retries; a breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: each new boundary test is sequenced on records, never on timing or claude content
home: keyboard-test-harness
@task_standing n=4
convention: each fixed CLI string a new test checks is pinned by its literal
home: 8-cognitive-accessibility · cognitive-accessibility
@task_item n=1 part=not_needed
The viola list board and its colour, caption, stale-row dimming and NAME truncation rules: the chunk prints no board.
@task_item n=2 part=not_needed
viola revive refusals, revive --json documents and revive --list rows: no revive path changes.
@task_item n=3 part=not_needed
viola link, unlink and plugin install contracts: untouched by a send's confirmation.
@task_item n=4 part=not_needed
Dialog paths: the dialog hold, dialog-expired event, the DIALOG word's amber and answering a pending dialog are untouched.
@task_item n=5 part=not_needed
wait's wake rule and output: the waiting line, timed_out and the MCP wait tool's timeout are untouched.
@task_item n=6 part=not_needed
The wheel's rules: a human key taking the wheel, the closed non-editing stdin list, and focus, mouse and resize sequences are untouched.
@task_item n=7 part=not_needed
Manual Orca and keyboard passes and the screen-reader target setups: supplemental, gating nothing for this chunk.
@task_item n=8 part=not_needed
Absences of a web or graphical frontend, a touch surface, a rendered page and internationalization: nothing here adds one.
@task_item n=9 part=not_needed
viola --help group labels, usage line and served verbs: no verb is added or renamed.
@task_item n=10 part=not_needed
Start refusals (home strict-modes, socket directory, already-live, stale-heartbeat holders): no start path changes.
