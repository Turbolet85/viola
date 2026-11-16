@task_prose part=chunk
Makes the driver's send match the submitted prompt when the claude CLI has trimmed whitespace from it: the prompt-submitted line is relabelled driver, the send is confirmed ok, and the wheel stays with the driver. Only the whitespace the CLI trims is tolerated; a prompt differing in any other byte still claims nothing.
The driver's text is still validated as received. A refused control character stays refused, never stripped, and viola types the text without removing the whitespace the CLI will trim.
Closes the race at the pre-send readiness gate: a turn that starts while a send waits on the settled-input-box predicate never receives the text. The send is refused turn-running, or typed only once that turn ends, and a test reproduces the race.
Re-measures from the code and the recorded claude fixtures what the product compares and what the CLI's submitted prompt shows of the trim. No live claude start is planned.
@task_file n=1 path=src/run/send.rs mark=edits
what: the wrapper's send path: the pre-send readiness gate (the settled-input-box predicate), the turn rung check and the paste that types the driver's text — edits: the window between the turn rung check and the paste, the validation of the text as received
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the in-flight send slot, which matches a submitted prompt to the sent text and confirms the send, fed by the hook-event append — edits: the slot's claim comparison, the hook-event append that feeds it
@task_file n=3 path=src/run/send/relabel.rs mark=edits
what: the relabel of a prompt-submitted line to driver origin and the human wheel move on an unmatched prompt — edits: the relabel on a trim-tolerant match
@task_file n=4 path=src/run/send/classify.rs mark=edits
what: the local-command list, which reads the exact typed text to classify a send as a prompt send or a local command
@task_file n=5 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the claude-specific hook shapes, including the typed-text rule (CR LF and CR as one LF, trailing CR and LF not typed) — edits: a rule naming the whitespace the CLI trims, offered to the send path as a normalised predicate
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent the tests drive in place of claude — edits: submitting a trimmed prompt, and starting a turn inside the readiness gate's window
@task_item n=1 part=needs
A prompt send whose text the CLI submits with whitespace trimmed ends as a confirmed prompt send. The send readback and the viola send readback mirror read back the driver's text, never unconfirmable or unable, and the send exits 0 instead of 13 not-delivered.
@task_item n=2 part=needs
Confirmation still comes only from the matched prompt-submitted line, never before it. The relaxed match must not let a send announce success optimistically or claim a prompt that differs in any byte other than the trimmed whitespace.
@task_item n=3 part=needs
On a trim-tolerant match the wheel stays with the driver. The flight-progress strip's WHEEL field and any wheel shown as words read the driver, not the human or human-input, and a later driver send is not refused for the wheel. A human's own unmatched prompt still takes the wheel.
@task_item n=4 part=needs
The refusal not-delivered with control-character still applies to a text holding a C0 other than LF, CR or TAB, a DEL or a C1. Its fixed refusal line and its hint are unchanged.
@task_item n=5 part=needs
A text that is empty after the typed-text rule is still refused not-delivered with empty-text, with that refusal's hint. Whether a text made only of trimmable whitespace needs its own outcome is left to P3 and P4.
@task_item n=6 part=needs
A turn starting while a send waits on the readiness gate ends that send as not-delivered with turn-running, carrying the turn-running refusal hint, or the text is typed only after that turn ends. The input-not-ready and no-prompt-submitted outcomes keep their meanings.
@task_item n=7 part=needs
CLI exit codes for send: exit 0 on a confirmed send, exit 13 not-delivered only where the text was truly not delivered, with stdout carrying data and stderr carrying the refusal line and its hint.
@task_item n=8 part=needs
The CLI send and MCP send outcomes change only through the wrapper's reply they already read. Peer reply screening of refusal code words and the --json result document keep their shape, with no new code word unless P4 decides one.
@task_item n=9 part=needs
Claude-specific knowledge of what the CLI trims lives only in the claude agent crate and reaches the send path as a normalised predicate. The wrapper's send path names no claude shape of its own.
@task_item n=10 part=needs
The trim is undocumented claude CLI behaviour. Whether the tolerance stands on a capability-ledger row with a viola verify probe, on the recorded fixtures or on a closed rule is decided at P4, and the CLI verification state of verified or unverified-cli applies as it already does to sends.
@task_item n=11 part=needs
Hints and errors from the send path quote no text and name no internals. No hint names viola release, even though the old mismatch could only be cleared by a release.
@task_item n=12 part=needs
Every test that waits on a channel or a thread is bounded and cannot hang past a failure. The race test drives the fake agent to start a turn inside the gate's window.
@task_standing n=1
convention: every new or changed human-facing line on the send path is ASCII only, escapes control characters, is never wrapped, and uses the fixed phraseology vocabulary
home: surface-cli · surface
@task_standing n=2
convention: no human or --json output of a send confirms before the matching prompt-submitted line is read
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
Colour palette, typography, spacing, border radius and depth: the chunk adds no colour, weight, column or spacing to any output.
@task_item n=2 part=not_needed
The viola list board and table, its columns, NAME cut, skip counts and strict-modes scan: the chunk changes no board rendering.
@task_item n=3 part=not_needed
viola run and revive start refusals, revive --list output and revive --json refusal codes: start and revive paths are untouched.
@task_item n=4 part=not_needed
viola verify output and its refusals: the chunk plans no verify run, and the ledger question is decided at P4.
@task_item n=5 part=not_needed
viola link and unlink output and their --json results: linking is untouched.
@task_item n=6 part=not_needed
viola wait output forms and the wait wake rule: the scope leaves wait's wake rule untouched.
@task_item n=7 part=not_needed
Dialog-pending display, the cocked strip and arrival amber: the dialog paths are untouched.
@task_item n=8 part=not_needed
Exit 21 causes, the strict-modes refusals and the socket-directory refusals: instance reachability is untouched.
@task_item n=9 part=not_needed
viola --help verb groups, the flat verb list and the hidden mcp verb: the MCP tool surface's shape and the verbs stay as they are.
@task_item n=10 part=not_needed
The web page surface and the self-validation squint, swap and token tests: viola renders no page and the chunk adds no visual element.
