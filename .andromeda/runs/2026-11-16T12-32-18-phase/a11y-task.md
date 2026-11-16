@task_prose part=chunk
Makes a key the human presses to answer or dismiss the claude CLI's session-feedback survey edit no prompt, so it leaves the wheel with its holder; every other key keeps taking the wheel.
The survey's grammar (which keys answer or dismiss it, and the screen state in which they do) is measured live on the installed CLI before it joins the closed non-editing list; an unmeasured prompt shape still takes the wheel.
The measured grammar becomes a capability-ledger row in the claude agent crate, with a compiled screen signature and a viola verify probe.
The fake agent learns to draw a survey screen for CI, and any recorded survey screen lands in the recorded claude fixtures.
At P3 it re-measures the closed list, the wheel classification and what is known of the survey; live claude starts wait on the founder's word.
@task_file n=1 path=src/run/wheel.rs mark=edits
what: the wheel's stdin classifier, which decides whether a human stdin read holds an editing key against the closed non-editing list — edits: the Classifier / Observed path
@task_file n=2 path=crates/viola-agent-claude mark=edits
what: the claude capability ledger, its compiled screen signatures and the viola verify probes — edits: a new ledger row for the survey grammar, its screen signature, its verify probe
@task_file n=3 path=fixtures/claude
what: the recorded claude screen payloads per CLI version — a recorded survey screen
@task_item n=1 part=needs
The closed non-editing stdin list: a survey answer or dismiss key joins it only as measured, and only while the survey's screen state is shown; the list stays closed and nothing widens it by env var, flag or config key.
@task_item n=2 part=needs
Human key takes the wheel: every key outside the measured survey grammar, including the same key with no survey on screen, still takes the wheel.
@task_item n=3 part=needs
Terminal sequences are not human editing: focus reports, mouse reports, terminal replies and resizes keep their place on the list unchanged.
@task_item n=4 part=needs
Human keystrokes never blocked and the terminal belongs to the child: the survey key still reaches the child unchanged; the wheel classifier only observes and viola never answers the survey nor types a survey key itself.
@task_item n=5 part=needs
Screen-content parsing goes no further than compiled signatures: the survey's screen state reaches the wheel decision only through a ledger-compiled literal read from the pump's vt100 model, never a free parse of claude content.
@task_item n=6 part=needs
The TUI boundary checks: the human-keystrokes and focus, mouse and resize sequences contracts of the viola run passthrough, and zero viola bytes on the terminal while the survey is up.
@task_item n=7 part=needs
Boundary tests sequenced on records: the CI proof runs on the fake agent's survey screen and the recorded payloads, never on a live claude.
@task_item n=8 part=needs
Live readings never gate: the live measurement of the survey grammar is a reading that informs the ledger row, while CI's pass rests on recorded and fake-agent cases only.
@task_item n=9 part=needs
TUI boundary driver: the outer-PTY driver that pushes the survey key and reads the wheel holder in the boundary tests.
@task_item n=10 part=needs
Send refused while the human types: a survey key that keeps the wheel must not count as human typing for a driver's send.
@task_item n=11 part=needs
Wrapped TUI reads as unwrapped: the survey screen under viola reads to a screen reader exactly as it does without viola.
@task_item n=12 part=needs
Time bounds never fall on the human: waits in the rig and tests are bounded and fall on the driver, never on the human answering the survey.
@task_item n=13 part=needs
Run diagnostics never reach the terminal: any log line about a survey key classification goes off the terminal.
@task_item n=14 part=needs
The viola verify step line: the new survey probe reports through the existing step-line format.
@task_standing n=1
convention: each new a11y assertion runs inside the existing test suites as a nextest failure, no separate a11y driver, suite or artifact
home: bootstrap-phases-derive-for-route-setup-project · 9-ci-integration
@task_standing n=2
convention: each new test is deterministic with zero flakiness and no retries, and no skips in the CI test gate
home: 10-slo-invariants-a11y-budgets · 1-a11y-scope-summary
@task_standing n=3
convention: each new case passes inside the coverage suite and the gate that requires coverage and doctest, and the mutation audit covers invariants held in crate tests
home: ci-integration · 10-slo-invariants-a11y-budgets
@task_item n=1 part=not_needed
The viola list board: its colour, caption, order, stale-row and unwrapped-separator rows; the chunk prints no board output.
@task_item n=2 part=not_needed
CLI refusal, hint, exit-code and --json failure document rows, including revive --json refusals; the chunk adds no refusal and no exit code.
@task_item n=3 part=not_needed
Colour and SGR rows (bold NAME, amber DIALOG, dim stale rows, no SGR off a terminal); the chunk prints no coloured output.
@task_item n=4 part=not_needed
viola --help rows (group labels, usage line, served verbs, handoff row); no verb or help text changes.
@task_item n=5 part=not_needed
Link, unlink and plugin install output rows; those verbs are untouched.
@task_item n=6 part=not_needed
Frame verb, hook call and MCP frame tool refusals over socket directories, strict modes and server verification; no IPC path changes.
@task_item n=7 part=not_needed
Manual Orca and keyboard passes and the screen-reader target setups; they are supplemental and gate nothing in this chunk.
@task_item n=8 part=not_needed
Absences of a web or graphical frontend, touch surface, rendered page and internationalization; nothing here reaches them.
@task_item n=9 part=not_needed
Dialog hold and dialog-expired rows, config dialog_hold_s and the MCP wait tool timeout; the survey is not a permission dialog and no wait bound changes.
@task_item n=10 part=not_needed
Error body, fault body, thiserror message and code-word format rows; the chunk adds no error type.
