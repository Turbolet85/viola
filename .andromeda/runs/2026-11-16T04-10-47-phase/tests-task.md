@task_prose part=chunk
Makes the driver-send confirmation tolerate exactly the whitespace the claude CLI trims from a submitted prompt: the prompt-submitted line is relabelled driver, the send confirmed ok, the wheel kept by the driver.
The driver's text is still validated as received and typed without stripping that whitespace; a refused control character stays refused, an empty typed text stays empty-text.
Closes the readiness-gate turn race: a turn starting while a send waits on the gate never receives the text — the send is refused turn-running or typed only after that turn ends — and a test reproduces the race.
Re-measures from the code and the recorded claude fixtures what the product compares and what the CLI's submitted prompt shows of the trim, with no live claude start.
@task_file n=1 path=src/run/send/slot.rs mark=edits
what: the in-flight send slot of the wrapper's driver send: the send confirmation match of a prompt-submitted line against the sent text, the confirmation window and the wheel move to the human on an unmatched prompt — edits: the slot's claim comparison, the hook-event append that feeds it
@task_file n=2 path=src/run/send/relabel.rs mark=edits
what: the relabel of a matched prompt-submitted line to origin driver and the human wheel move on an unmatched one — edits: the relabel on a trim-tolerant match
@task_file n=3 path=src/run/send.rs mark=edits
what: the wrapper's driver send path: the send refusal order, the pre-send readiness gate (settled input box, its maximum wait), the turn-running refusal, the paste of the typed text — edits: the window between the turn check and the paste
@task_file n=4 path=src/run/send/classify.rs
what: the ledger-listed local-command list matched by exact equality on the typed text, and its post-conditions
@task_file n=5 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the claude-specific hook shapes, the typed-text normalisation of sent text (CR LF and CR as one LF, trailing CR and LF not typed) — edits: a new rule naming the whitespace the CLI trims, exposed as a normalised predicate
@task_file n=6 path=src/bin/viola-fake-agent.rs mark=edits
what: the fake agent serving the claude boundary: its input handling (bracketed paste, keystroke), its fired hook events and payloads, its turn scripts and gated steps — edits: submitting a trimmed prompt, starting a turn inside the gate's window
@task_item n=1 part=needs
The driver send confirmation: a prompt-submitted payload whose prompt equals the typed text up to the whitespace the CLI trims is confirmed as the driver's own — origin driver, send ok, exit 0, the send-confirmed log line — and any other byte difference still claims nothing.
@task_item n=2 part=needs
The send confirmation rule changes from an exact normalised match to a match tolerant of exactly the CLI's trim; the rule's own tests must show both the tolerated trim and a one-byte non-whitespace difference left unmatched.
@task_item n=3 part=needs
The wheel after a trimmed driver send stays with the driver: no wheel human-input record, no human-typing refusal of a later driver send, no release needed; a genuinely human prompt still takes the wheel.
@task_item n=4 part=needs
The send result outcomes: a trimmed send no longer exits 13 not-delivered / no-prompt-submitted; the CLI send --json document, the human-mode confirmed line and the MCP send ok result read the wrapper's reply as they already do.
@task_item n=5 part=needs
Sent text validated as received at client and wrapper: a C0 other than LF, CR and TAB, DEL or C1 is refused not-delivered / control-character, never removed; the trimmable whitespace is not stripped before validation or typing.
@task_item n=6 part=needs
The typed text keeps its rules (CR LF and CR as one LF, trailing CR and LF not typed, multi-line text one prompt, the typed framing); nothing else is taken off, and an empty typed text is refused empty-text before the wheel is read.
@task_item n=7 part=needs
The readiness gate (settled input box, GATE_MAX_WAIT) and the send refusal order: a turn starting during the gate wait yields a turn-running refusal or a paste only after turn-ended — never text typed into the running turn.
@task_item n=8 part=needs
The send event records: send-issued, send-confirmed and send-refused lines and the prompt-submitted event of the event log, with their corr fields, for a trimmed send and for the race.
@task_item n=9 part=needs
Claude shapes only in the claude agent crate: the rule naming what the CLI trims lives there and the send path consumes it as a normalised predicate, never a claude literal in the send path.
@task_item n=10 part=needs
Undocumented CLI behaviour: whether the trim tolerance stands on a capability-ledger row with a viola verify probe, on the recorded fixtures, or on a closed rule is the plan's to read; the recorded claude fixtures per CLI version and their replay are the evidence read.
@task_item n=11 part=needs
The fake agent held to the recorded fixtures: a trimmed-prompt submission and a turn started inside the gate's window are added through its argv-only options, turn scripts, control-file gated steps and receipts, matching what the recordings show.
@task_item n=12 part=needs
Race and match tests are E2E over the fake agent with real PTY and real filesystem, waits keyed on logged events, no sleeps or retries, each wait on a channel or thread bounded so none hangs past a failure, and the stamped-home kill lines respected.
@task_item n=13 part=needs
Unit cases for the match and the trim predicate are table-driven inline unit tests with literal oracles, with property tests over the parser-like predicate where the plan asks it.
@task_item n=14 part=needs
No live claude start and no mutation run in this chunk: real claude stays local-only and outside CI, and any guard control fires with a bounded timeout on the guard's own filtered cases.
@task_item n=15 part=needs
Send confirmation window expiry and the unconfirmable local-command outcome stay intact for a send that is not matched; the local-command list still matches the exact typed text.
@task_standing n=1
convention: every new test owns its own fresh private home, seeds its own data, and runs in parallel without shared mutable state
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=2
convention: every new test waits on logged events under a bound, never on sleep, retries or the child's rendered screen
home: 11-test-anti-patterns-never-do-these
@task_standing n=3
convention: every new test is named by the test-function naming rule and sits in the layer and binary prefix of its surface
home: 2-test-strategy · test-strategy
@task_standing n=4
convention: every new fixture or fake-agent turn script carries synthetic, scrubbed text with a canary and no real prompt
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=5
convention: every new or changed log line is a structured JSON line with its corr and carries no secrets or session text
home: log-format
@task_item n=1 part=not_needed
Dialog hook paths: decision bodies, dialog hold expiry, answer mapping and the answer verb are untouched.
@task_item n=2 part=not_needed
The wait verb's wake rule and the channel wait method's wake kinds are untouched.
@task_item n=3 part=not_needed
The MCP tool surface's shape: tools/list, tool schemas and invalid-params fields are not changed.
@task_item n=4 part=not_needed
Revive, run start refusals, strict-modes refusals, heartbeat staleness and instance liveness are not reached.
@task_item n=5 part=not_needed
Link and unlink methods, the list board and list --json, release and last are not changed.
@task_item n=6 part=not_needed
The harness commands' own contracts — boot, status, cleanup and session records — are not changed by this chunk.
@task_item n=7 part=not_needed
Hook performance budgets and hyperfine timing: send confirmation latency is not timed.
@task_item n=8 part=not_needed
Hyprland, the vt100 screen feed and passthrough rendering are not reached.
