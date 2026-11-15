@task_prose part=chunk
A turn the claude CLI ends on an API error is logged as a turn end, turn-ended, carrying a cause code from a closed viola set; the error text never reaches a log line, an event or a snapshot.
After that turn end the instance reads idle on the board, the wheel's turn-running state is cleared exactly as a Stop clears it, a wait on the turn end returns, and the driver's next send is accepted with no pause and release.
It registers a hook for the CLI signal that ends such a turn, reads its payload in the Claude agent crate, maps the CLI's error string into the neutral cause set, and lets the fake agent emit the end.
A test drives the API-error end through the fake agent and recorded or synthetic payloads and fails on the tree before the fix; P3 re-reads the measured defect (board idle while send is refused turn-running) from the code.
@task_file n=1 path=plugin/hooks/hooks.json mark=edits
what: the plugin's hook registration list, the boundary where the claude CLI's turn signals reach viola's hook verb — edits: the registration for the signal that ends a turn on an API error
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the run pump's send slot, the wheel state that refuses a send while a turn is running — edits: the turn-end path that clears turn-running
@task_file n=3 path=src/run/wait.rs mark=edits
what: the run pump's wait, which wakes a waiter on a turn-ended event — edits: the turn-end path
@task_item n=1 part=needs
The turn-ended event written to the instance's event log gains a cause code: a fixed code word from a closed set, every member named, with a fixed fallback member for any CLI value outside it; the CLI's own error string is never carried.
@task_item n=2 part=needs
The error text on this path (the last assistant message and the error details) reaches no event line, no snapshot and no log line; a log line or event datum names the cause by its code word alone.
@task_item n=3 part=needs
The viola list board STATUS reads idle after an API-error turn end, and the WHEEL column, shown as words, no longer reads a running turn: the board and the wheel agree.
@task_item n=4 part=needs
viola send after an API-error turn end is accepted and read back as any prompt send; the not-delivered turn-running refusal (exit 13) and its hint stay only for a turn that is really running.
@task_item n=5 part=needs
viola wait on the turn end returns: its event form, and its --json result as one stdout document, carry the turn-ended event like a Stop-ended one; the wait never times out on a turn that has ended.
@task_item n=6 part=needs
A turn-ended event never fills the readback box; the API-error end, its cause code included, fills no readback either.
@task_item n=7 part=needs
Recovery needs no human pause and release: the control-transfer verbs are not the way back to idle after an API-error end.
@task_item n=8 part=needs
The new hook registration is served by the hidden hook verb: it keeps that verb's exit-code and stderr contract, printing nothing a human reads and never failing the CLI's turn.
@task_item n=9 part=needs
No error, hint or output line shows internals or quotes the CLI's error text: a cause is a fixed code word, never free text from the agent.
@task_item n=10 part=needs
The cause code's name is neutral, with no Claude-specific word; the mapping from the CLI's error values lives only with the Claude agent.
@task_item n=11 part=needs
The turn-ended data keys grow by a cause key; readers skip and count unknown keys, so an older reader still reads the event.
@task_item n=12 part=needs
If P3 or P4 makes the CLI's API-error hook a capability-ledger row, its viola verify probe reads as a step line and the summary like any other probe, and an unmeasured row leaves the CLI verification state as it is.
@task_item n=13 part=needs
The new test reaches the API-error end through the fake agent and recorded or synthetic payloads, never a live claude start, and fails on the tree before the fix.
@task_standing n=1
convention: each new log line or event datum names a cause or detail by a fixed code word from a closed list, never by free text read from a peer or the agent
home: surface
@task_item n=1 part=not_needed
Colour palette, typography, spacing, border radius and iconography rows: the chunk adds no colour, glyph, column or styling; the board's columns are only read as they stand.
@task_item n=2 part=not_needed
The web page surface and the rendered page absence: viola renders no page.
@task_item n=3 part=not_needed
viola revive output, its --json exit-1 codes and its refusals, and viola run start refusals: the chunk changes no start or revive path.
@task_item n=4 part=not_needed
viola link and unlink output and their --json documents: pairing is not touched.
@task_item n=5 part=not_needed
Exit 21 causes, their --json causes, and the strict-modes and socket-directory refusals: the chunk adds no instance-unreachable cause and no home-tree check.
@task_item n=6 part=not_needed
Wrapper fault --json detail and peer reply screening: the chunk adds no wrapper fault code and no peer reply.
@task_item n=7 part=not_needed
The CLI colour decision order, colour depth and colour tokens: no new styled output.
@task_item n=8 part=not_needed
viola --help verb groups and the command name register: the chunk adds no verb.
@task_item n=9 part=not_needed
The mcp verb's stdout and the self-validation tests (squint, swap, readback box signature): the chunk shows no new design element.
@task_item n=10 part=not_needed
viola last output: the chunk changes no last-message reading.
