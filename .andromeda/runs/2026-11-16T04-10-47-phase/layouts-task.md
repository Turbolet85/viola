@task_prose part=chunk
Makes a driver's send whose text the CLI submits with whitespace trimmed match the in-flight send: the submitted prompt is relabelled the driver's, the send is confirmed ok, the wheel stays with the driver, and later driver sends are not refused for the wheel.
The match tolerates exactly the whitespace the CLI trims and no other byte; the driver's text is validated as received (a refused control character still refused, never stripped) and typed without removing the trimmed whitespace.
Closes the readiness-gate turn race: a turn starting while a send waits on the settled-input-box gate never receives the text; the send is refused turn-running or typed only after that turn ends, and a test reproduces the race.
Re-measures from the code and the recorded fixtures what the product compares and what the CLI's submitted prompt shows, before the trim's shape is stated; no live claude start without asking first.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path: the pre-send readiness gate (the settled-input-box predicate), the turn rung check and the paste of the typed text — edits: the readiness gate, the window between the rung check and the paste
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the in-flight send slot that claims a submitted prompt for the driver's send and moves the wheel to the human when unclaimed — edits: SendSlot::claim, the text comparison fed by append_hook_event
@task_file n=3 path=src/run/send/relabel.rs mark=edits
what: the relabel of a prompt-submitted line's origin to driver and the human wheel move on an unmatched prompt
@task_file n=4 path=src/run/send/classify.rs mark=edits
what: the local-command list classification of a send, read from the exact typed text
@task_file n=5 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the Claude agent's hook shapes: the typed-text rule and the rule naming the whitespace the CLI trims, consumed by the send path as a normalised predicate — edits: typed_text, the trim predicate
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent that stands in for the claude CLI in tests — submitting a trimmed prompt or starting a turn inside the readiness gate's window
@task_file n=7 path=fixtures/fake-scripts mark=edits
what: the fake agent's scripts driving the trimmed-prompt and the gate-window turn cases
@task_item n=1 part=needs
A driver send whose text the CLI submits with whitespace trimmed is now confirmed: the send outcome is ok (the read-back of a delivered send), no longer an exit 13 not-delivered, on viola send and on the MCP send tool alike.
@task_item n=2 part=needs
The match tolerates exactly the whitespace the CLI trims: a submitted prompt differing in any other byte still claims nothing and is read as the human's, as today.
@task_item n=3 part=needs
The wheel after a trimmed-but-matched send stays with the driver, and a later driver send is not refused for the wheel; a human's own prompt still takes the wheel (no release needed for the matched case).
@task_item n=4 part=needs
The driver's text is validated as received: a C0 other than LF/CR/TAB, DEL or C1 is refused not-delivered with cause control-character, never removed; the CLI and the MCP wrapper keep repeating that refusal as today.
@task_item n=5 part=needs
The typed text keeps today's rules (CR LF and CR as one LF, trailing CR and LF not typed) and nothing else is taken off; an empty typed text stays the empty-text refusal.
@task_item n=6 part=needs
A send waiting on the readiness gate when a turn starts is refused with cause turn-running or typed only once that turn ends; the turn-running refusal reaches viola send and MCP send through their existing refusal line, exit code, hint and --json forms.
@task_item n=7 part=needs
The readiness gate's own outcome (the input-not-ready cause and its hint) keeps its existing form; only the race window between the turn rung check and the paste is closed.
@task_item n=8 part=needs
The CLI send and MCP send outcomes change only as they already read the wrapper's reply: the send --json typed document, the readback mirror line, the word column and the exit codes keep their shapes; no new outcome word or cause is added.
@task_item n=9 part=needs
turn-ended stays not a send outcome: a send typed after the racing turn ends reports its own outcome, not the turn's end.
@task_item n=10 part=needs
Claude shapes live only in the Claude agent crate: the rule naming what the CLI trims is stated there and the send path consumes it as a normalised predicate, naming no CLI byte itself.
@task_item n=11 part=needs
The local-command list still reads the exact typed text: the read-back for /clear and /compact and their post-conditions are unchanged by the trim tolerance.
@task_item n=12 part=needs
Tests stand on the fake agent and the recorded payloads: the fake agent submits a trimmed prompt or starts a turn inside the gate's window; any test waiting on a channel or thread takes a bound and cannot hang past a failure.
@task_item n=13 part=needs
No live claude start is planned without asking first; no mutation run; every control fires with a timeout of 120 s or less on an entry filtered to the guard's own cases.
@task_item n=14 part=needs
Whether the trim tolerance stands on a capability-ledger row with a viola verify probe, on the recorded fixtures, or on a closed rule needing no row is left to P3 and P4; the verify surface may be reached if a probe is chosen.
@task_standing n=1
convention: each new test of a send outcome asserts the human and --json output as a contract: the refusal line form, the cause word, its hint as the last stderr line, and the exit code
home: surface-cli
@task_item n=1 part=not_needed
The web surface: viola renders no page, and this chunk adds none.
@task_item n=2 part=not_needed
The viola list board and its BAY line, columns, colour depth, truncation and --json document: the chunk changes no board or list output.
@task_item n=3 part=not_needed
viola revive and viola run start outputs, refusals and --json error codes: the chunk touches no start path.
@task_item n=4 part=not_needed
viola link / unlink output and viola plugin install output: untouched verbs.
@task_item n=5 part=not_needed
viola wait output and its wake rule: wait's wake rule is untouched by this chunk.
@task_item n=6 part=not_needed
Dialog paths: viola answer output, the DIALOG attention word and the dialog wait outputs are untouched.
@task_item n=7 part=not_needed
CLI token rendering, colour roles, spacing and motion tokens: the chunk adds no styled output.
@task_item n=8 part=not_needed
viola --help and per-verb usage output, the flat verb set and argument order: no verb or flag is added or changed.
@task_item n=9 part=not_needed
The MCP tool surface's shape (the tool list, which verbs MCP hides, invalid-params results): untouched; only the send outcome as read from the wrapper's reply.
@task_item n=10 part=not_needed
Wheel verb output for viola pause and viola release: the wheel's rules other than the match are untouched.
