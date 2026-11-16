@task_prose part=chunk
A driver's send of a local slash command that the CLI runs reads as delivered (ok), confirmed by an echo of that command the product can observe, never presumed; a local command the CLI did not run is never confirmed by it.
Today an unlisted local command waits for a prompt-submitted that never fires and ends not-delivered / no-prompt-submitted; a listed command with no measured post-condition reads ok with confirmed false, detail unconfirmable.
What the echo is (a hook event, a screen line, another signal) and which commands it covers is re-measured from the code and the recorded fixtures; an echo relied on becomes a capability-ledger row with a viola verify probe.
The three readings of a local-command send today (listed post-conditions, unconfirmable, no-prompt-submitted) are re-read against the send path, its classifier, the CLI send human mirror and the MCP send result.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path that confirms a send and gives its outcome (ok, not-delivered) — edits: the local-command confirmation
@task_file n=2 path=src/run/send/classify.rs mark=edits
what: the send classifier that decides what confirms a send (prompt-submitted, a local command's post-condition) — edits: the local-command classification
@task_file n=3 path=src/run/send/slot.rs mark=edits
what: the send slot that waits for a send's confirming fact within its window
@task_file n=4 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the capability ledger of Claude CLI behaviours and the compiled local-command list with each command's post-condition — edits: the local-command list, a new ledger row for the echo
@task_file n=5 path=src/cmd/verify/ mark=edits
what: viola verify, the probes that measure and stamp each capability-ledger row — edits: the echo's probe
@task_file n=6 path=src/run/version_gate.rs mark=edits
what: the CLI version gate that decides whether a measured capability holds on the running claude version (verified or unverified)
@task_file n=7 path=src/human/mirror.rs mark=edits
what: the CLI send human readback mirror, its issue and outcome lines and the fixed note for unconfirmable
@task_file n=8 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent standing in for claude in tests, its local-command mode
@task_file n=9 path=fixtures/fake-scripts/ mark=edits
what: the fake agent's scripts that drive test scenarios
@task_item n=1 part=needs
The viola send outcome for a local slash command: ok when its own echo confirms it, with what that reading shows in the human readback mirror (issue and outcome lines, read back, word column).
@task_item n=2 part=needs
The unconfirmable reading of a send (ok, confirmed false, its fixed note) and when a local-command send still reads so: a listed command with no measured echo or post-condition, or an unverified CLI.
@task_item n=3 part=needs
The not-delivered reading of a send and its detail (no-prompt-submitted) which an unlisted local command that ran must no longer reach, while a local command the CLI did not run is never confirmed.
@task_item n=4 part=needs
viola send --json: its typed document's outcome, confirmed flag and detail for a local-command send.
@task_item n=5 part=needs
The MCP send tool's result for a local-command send, read only as it already carries the wrapper's reply; the MCP tool surface's shape stays as it is.
@task_item n=6 part=needs
The read back for /clear and /compact keeps its measured post-conditions; a new echo confirmation sits beside them, never replaces them.
@task_item n=7 part=needs
viola verify: a new probe for the echo's capability-ledger row, its step line, failing row and summary as verify already prints them.
@task_item n=8 part=needs
Indirect: what counts as a local command's echo is read in the Claude-specific agent crate only; the send path consumes a normalised fact, so no Claude shape reaches CLI or MCP output.
@task_item n=9 part=needs
Indirect: upstream text (a local command's output) is content, never a viola command or config, and crosses no log, event, error, refusal line or hint.
@task_item n=10 part=needs
Indirect: send lines are appended, never redrawn, with no progress glyph or spinner; a new confirmation reading adds no new progress pattern.
@task_item n=11 part=needs
Indirect: human output is a contract and words carry every state; any new or changed send state word or note is a fixed caption from the readback vocabulary, the same on CLI and in --json.
@task_item n=12 part=needs
Indirect: turn-ended is not a send outcome; a local command's confirmation is the send's own, never a wait or turn reading.
@task_item n=13 part=needs
New tests drive the fake agent's local-command mode and the recorded payloads, never a live claude start; each test waiting on a channel or thread takes a bound, test sockets go under the short /tmp root, a stamped home comes from the shared stamp's copy.
@task_standing n=1
convention: each new or changed human send reading is pinned as a fixed line of the readback mirror and its word column, with a matching --json document form
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page; nothing here reaches it.
@task_item n=2 part=not_needed
The viola list board and its BAY line, columns, colours, truncation and stale rows: a send's confirmation shows on no board row.
@task_item n=3 part=not_needed
viola run and viola revive starts, their refusals, --list and --json error codes: the chunk starts nothing.
@task_item n=4 part=not_needed
viola wait output (waiting line, dialog, timed out, turn-ended) and wait's wake rule: left untouched.
@task_item n=5 part=not_needed
viola answer output and the dialog paths: left untouched.
@task_item n=6 part=not_needed
Wheel verb output (pause, release) and the wheel's rules, and viola link / unlink output: not reached.
@task_item n=7 part=not_needed
viola plugin install, viola last output and the CLI token rendering rows (colours, radius, spacing, motion): not reached.
@task_item n=8 part=not_needed
viola --help and per-verb usage, global flags and the frame verbs' exit-21 hints and details: no verb, flag or connection path changes.
@task_item n=9 part=not_needed
The send text refusals before any request (empty-text, control-character) and the prompt-submitted match for ordinary sends: left untouched.
