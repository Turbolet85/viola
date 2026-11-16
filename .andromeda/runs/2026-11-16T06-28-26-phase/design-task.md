@task_prose part=chunk
A driver's send of a local slash command the CLI runs reads as delivered (ok), confirmed by an echo of that command the product can observe, never presumed; a local command the CLI did not run is never confirmed by it.
Today an unlisted local command waits for a prompt-submitted that never fires and ends not-delivered / no-prompt-submitted although it ran; a listed command with no measured post-condition reads ok, unconfirmed, detail unconfirmable.
The echo is measured, not assumed: what the CLI gives as a local command's echo is read in the Claude adapter and handed to the send path as a normalised fact; an echo the product relies on becomes a capability-ledger row with a viola verify probe, or the send stays honest about what it cannot confirm.
It re-measures what a local-command send reads today (the listed post-conditions, unconfirmable, no-prompt-submitted) in the send path, the CLI send mirror and the MCP send result.
@task_file n=1 path=src/run/send.rs mark=edits
what: the send path of the wrapper: delivers a driver's text and decides its readback (delivered, not-delivered, unconfirmable) from the confirmation it waits for
@task_file n=2 path=src/run/send/classify.rs mark=edits
what: the send classification: which confirmation a send waits for, a prompt or a local command's post-condition — edits: the local-command classification (Confirms::Prompt for an unlisted local command)
@task_file n=3 path=src/run/send/slot.rs mark=edits
what: the send slot: the pending send waiting for its confirming event and its window
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the Claude adapter's capability ledger: the local-command list and each command's measured post-condition — edits: the local-command list, a new ledger row for the echo if it is relied on
@task_file n=5 path=src/cmd/verify/typed.rs mark=edits
what: viola verify's probes, each a step that measures one ledger row against the installed CLI and stamps it — edits: a probe for the local-command echo
@task_file n=6 path=src/run/version_gate.rs mark=edits
what: the version gate: whether the installed CLI is verified, deciding confirmable versus unconfirmable readbacks
@task_file n=7 path=src/human/mirror.rs mark=edits
what: the human CLI mirror of viola send's readback: read back, unable, unconfirmable and its fixed note
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent the tests drive in place of claude — edits: its local-command mode, emitting the measured echo
@task_item n=1 part=needs
The send readback of a local slash command: a run command reads delivered (ok), confirmed by its own echo; a command not run is never confirmed; no readback announces success before that confirmation.
@task_item n=2 part=needs
The unconfirmable readback: a local command with no measured echo or post-condition, or on an unverified CLI, still reads ok with confirmed false and the unconfirmable detail, never a presumed confirmation.
@task_item n=3 part=needs
The not-delivered readback with detail no-prompt-submitted and its refusal hint: an unlisted local command that ran must no longer end there; an ordinary prompt send that got no prompt-submitted still does.
@task_item n=4 part=needs
The /clear and /compact send readbacks: their measured post-conditions (new session, compacted session) stay as they are; the echo must not displace them.
@task_item n=5 part=needs
The viola send human mirror (read back, unable, unconfirmable with its fixed note) and the readback box it renders: what a local-command send prints once confirmed by its echo.
@task_item n=6 part=needs
The send --json result and the MCP send result: they carry the wrapper's reply as it is (ok, confirmed, detail); the MCP tool surface's shape does not change.
@task_item n=7 part=needs
CLI verification state (verified, unverified-cli): an echo relied on is a ledger row measured and stamped for the installed version; on an unverified CLI the local-command send falls back to unconfirmable.
@task_item n=8 part=needs
viola verify gains a probe for the local-command echo: its step line, fail word and summary line follow the existing verify output forms.
@task_item n=9 part=needs
Claude shapes only in the Claude adapter: what counts as a local command's echo is read there and the send path consumes a normalised fact.
@task_item n=10 part=needs
Upstream text (a command's output) is content, never a viola command or config; it crosses no log line, event, error, hint or readback detail.
@task_item n=11 part=needs
The readback vocabulary is the fixed phraseology: any new detail word for an echo-confirmed send is a fixed code word, not free text, and names no internals.
@task_item n=12 part=needs
The prompt-submitted match for ordinary sends (its trailing-whitespace tolerance) stays untouched: the echo path applies to local commands only.
@task_item n=13 part=needs
The send window: a local command waiting on its echo is bounded by the same window and never hangs; its outcome after the window is honest about what it saw.
@task_standing n=1
convention: every new human output line is ASCII only, its marks words not glyphs, with no spinner, colour beyond the allowed tokens, or check-mark prefix
home: surface-cli
@task_standing n=2
convention: every new message goes to stderr and data to stdout; under --json the result stays one stdout document with no hint line
home: surface-cli
@task_standing n=3
convention: every new error, hint or log line escapes control characters and names no internals or quoted upstream text
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
viola list board, its columns, STATUS, WHEEL, NAME widths and skip counts: the chunk does not change what the board shows.
@task_item n=2 part=not_needed
viola revive, run start refusals and their --json codes: no start or revive path changes.
@task_item n=3 part=not_needed
viola link, unlink, pause, release and the handoff display: control transfer is untouched.
@task_item n=4 part=not_needed
viola wait output and its wake rule: the chunk leaves wait as it is.
@task_item n=5 part=not_needed
Dialog-pending display and viola answer: the dialog paths are untouched.
@task_item n=6 part=not_needed
exit 21 causes, strict-modes refusals, socket-directory refusals and wrapper fault details: no instance-reachability or home-state path changes.
@task_item n=7 part=not_needed
viola --help verb groups and the command names register: no verb is added or renamed.
@task_item n=8 part=not_needed
Typography, spacing, colour palette and depth rows: no new styled element is drawn.
@task_item n=9 part=not_needed
A web page or rendered page surface: viola has none.
