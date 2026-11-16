@task_prose part=chunk
Makes the driver's send match a prompt the claude CLI submits with whitespace trimmed: the prompt-submitted line is relabelled driver, the send is confirmed ok and the wheel stays with the driver; exactly the trimmed whitespace is tolerated, any other differing byte claims nothing.
Keeps the driver's text validated as received: a refused control character is still refused not-delivered / control-character and never removed, the typed text keeps today's rules, and empty typed text stays empty-text.
Closes the readiness-gate turn race: a turn that starts while a send waits on the settled-input-box gate never receives the text; the send is refused turn-running or typed only after that turn ends, and a test reproduces the race.
P3 re-measures from the code and the recorded fixtures what the product compares and what the CLI's submitted prompt shows of the trim, before the trim's shape is stated; no live claude start is planned.
@task_file n=1 path=src/run/send.rs mark=edits
what: the send path of the wrapper: the pre-send readiness gate, the turn rung check and the paste of the driver's text — edits: the readiness gate and the window between the rung check and the paste
@task_file n=2 path=src/run/send/slot.rs mark=edits
what: the in-flight send slot that matches a submitted prompt to the driver's send and moves the wheel — edits: SendSlot::claim, the human wheel move
@task_file n=3 path=src/run/send/relabel.rs mark=edits
what: the relabel of a prompt-submitted line to origin driver for a confirmed send
@task_file n=4 path=src/run/send/classify.rs
what: the local-command list of the send path, which reads the exact typed text
@task_file n=5 path=crates/viola-agent-claude/src/hook.rs mark=edits
what: the single claude vendor boundary for hook payloads and the typed text rule — edits: typed_text, a new normalised predicate naming what the CLI trims
@task_item n=1 part=needs
The confirmed send and readback critical path: a driver send whose prompt the CLI trims ends ok, its readback reads origin driver, and a later driver send is not refused for the wheel.
@task_item n=2 part=needs
The human takes the wheel critical path: a human's own prompt, or a prompt differing from the send in any byte other than the trimmed whitespace, still takes the wheel as today.
@task_item n=3 part=needs
C0/C1 control characters in the driver's text: a C0 other than LF, CR or TAB, DEL or C1 is refused with the code word control-character, never stripped or escaped away; the CR LF and trailing CR/LF rules of the typed text stay as they are.
@task_item n=4 part=needs
The viola send refusals and outcomes the chunk reaches: not-delivered with control-character, empty-text, turn-running for the readiness-gate race, input-not-ready and no-prompt-submitted — each as code word beside unable, with its exit code (13 for not-delivered) and its hint, on the human output and under --json.
@task_item n=5 part=needs
Send refused while the human types and the readiness gate: the settled-input-box predicate still guards the paste, and a turn starting inside the gate's window refuses or defers the send rather than typing into a running turn.
@task_item n=6 part=needs
The single vendor-content boundary: the rule naming what the claude CLI trims lives in the claude agent crate and is consumed by the send path as a normalised predicate; no claude shape in the wrapper's send code.
@task_item n=7 part=needs
Live readings never gate: the trim's tolerance stands on the recorded fixtures and the fake agent, not on a live claude start; a live reading, if one is ever asked for, gates nothing.
@task_item n=8 part=needs
The fake agent and its scripts: it must submit a trimmed prompt or start a turn inside the readiness gate's window, so the tests reproduce the CLI's behaviour without inspecting claude content.
@task_item n=9 part=needs
Boundary tests sequenced on records: the race test orders the turn start and the send on recorded events, not on sleeps; it is deterministic, takes a bound on every channel or thread wait and cannot hang past a failure.
@task_item n=10 part=needs
Time bounds fall on the driver only: the readiness gate's wait and any deferral until the turn ends bound the driver's send, never the human's keystrokes.
@task_item n=11 part=needs
Human keystrokes never blocked: closing the race moves nothing onto the human's input; the human's typing still reaches the child while a send waits.
@task_item n=12 part=needs
Fixed CLI strings pinned by literal: the refusal code words and the CLI send / MCP send outcome lines the chunk's tests read are pinned by their literal, as they already read the wrapper's reply.
@task_item n=13 part=needs
Zero viola bytes on the terminal: the send's text is typed into the child as received, and the chunk adds no viola byte to the human's terminal.
@task_standing n=1
convention: each new test is an assertion inside the existing test suites, run by nextest, with no a11y suite or driver of its own
home: 1-a11y-scope-summary · bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new test passes in CI with no skip and no retry; a failure fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: each new test case runs inside the coverage suite in the GitHub Actions push/PR workflow
home: 9-ci-integration
@task_item n=1 part=not_needed
The viola list board and its captions, colour, SGR, dim stale rows, bold NAME, amber DIALOG and NAME truncation: the chunk prints no board line.
@task_item n=2 part=not_needed
viola --help, its usage line, group labels and handoff row: no verb or flag is added.
@task_item n=3 part=not_needed
viola revive refusals, --list and --json documents, and start-against-a-holder refusals: the chunk does not touch starting or reviving a session.
@task_item n=4 part=not_needed
viola link and unlink, plugin install and their exit codes 20 and 21: untouched.
@task_item n=5 part=not_needed
viola verify output and the interrupted-verify pair: no verify step or probe is planned by this chunk at P1.
@task_item n=6 part=not_needed
viola wait output, its waiting line, timed_out and the MCP wait tool's timeout_ms: wait's wake rule is untouched.
@task_item n=7 part=not_needed
The dialog paths: dialog pending and answer, dialog_hold_s and the dialog-expired event line are untouched.
@task_item n=8 part=not_needed
Socket-directory, strict-modes and server-verify refusals on frame verbs, MCP frame tools and instance-unreachable details: the chunk changes no socket or peer check.
@task_item n=9 part=not_needed
Screen-reader targets and setups, the manual keyboard and Orca passes: supplemental, gate nothing, and nothing in the chunk changes what is read aloud.
@task_item n=10 part=not_needed
The absences (web or graphical frontend, touch, internationalization, rendered page) and the structured violation JSON schema: no frontend or report shape is touched.
@task_item n=11 part=not_needed
Log identity tags and run diagnostics off the terminal: the chunk adds no log identity or diagnostic channel.
