@task_prose part=chunk
Builds one standing, committed live rig that later live rounds reuse: it starts the real claude CLI (or viola wrapping it) in a pty it owns, keeps a terminal screen model of what the CLI draws, keeps the screens it reads as evidence, and reads the trust dialog and modal dialogs by their drawn literals and selector rows.
The rig owns its isolation (scratch root outside every repository, private config dir, guarded credentials copy removed on every exit path, a start refused under 20 minutes of token left) and its process hygiene (it ends every process it starts on every exit path, proven by a census at exit).
It carries live readings: the whitespace the CLI trims at submit, the hook-latency window before a send's paste, a fired scheduled wake's prompt-submit source field and a real wake count of 1, and the live record home for viola verify --record.
It re-measures at P3 what each earlier evidence script built and got wrong; the rig's home (a test-only binary of the e2e crate, a script, or both) and its screen model are P3's to settle; the product binary gains no verb and no surface.
At the end of P3 the phase stops with one sizing block: the rig's steps and files, whether rig and readings fit, a split if not, and each live start by shape, count and typed keys.
@task_item n=1 part=needs
The outer-PTY boundary driver: the rig starts the CLI or the wrapped TUI in a pty it owns, so the pinned portable-pty boundary driver and the TUI boundary driver choice bind how it drives the terminal.
@task_item n=2 part=needs
The terminal screen model: the rig keeps a vt100-style model of what the CLI draws (the vt100 crate the product pins for readiness is a candidate), so the rule on a contained vt100 feed panic and what the passthrough does after one bind it when it feeds bytes.
@task_item n=3 part=needs
Screen reading by signature only: the rig reads the trust dialog and modal dialogs by drawn literals and selector rows; the ban on screen-content parsing beyond signatures and the rule that a boundary check never inspects claude content bound what it may read off the screen.
@task_item n=4 part=needs
Live readings never gate: the rig's live starts and carried readings are evidence, not a CI gate; a reading only informs the founder's placement.
@task_item n=5 part=needs
The TUI passthrough boundary as wrapped by viola: when the rig drives viola wrapping claude, zero viola bytes reach the terminal, run diagnostics stay off the terminal, and the terminal belongs to the child; the rig's screens must read that boundary, not alter it.
@task_item n=6 part=needs
Human keystrokes and the wheel: the rig types keys into the pty as the human would (DEL after a resize, submit), so the human-key-takes-the-wheel rule, the closed non-editing stdin list and focus/mouse/resize sequences that never move the wheel are what its keyed starts exercise.
@task_item n=7 part=needs
Send behaviour under the rig: the delivery confirmation's trailing-only whitespace tolerance (ASCII space, TAB, CR, LF) stays as is until the trim reading lands; send refused while the human types and the input-not-ready and no-prompt-submitted refusals are what the hook-latency reading probes around.
@task_item n=8 part=needs
Paths not carried: no host path, nothing of the founder's status line or config reaches a record the rig keeps, and the live record home holds no path component named home, taken from a helper like the tests' record home.
@task_item n=9 part=needs
Time bounds fall on the driver only: the rig's own kill deadline, the token-window refusal and every test waiting on a channel or thread take a bound that falls on the driver, never on the human; the dialog hold bounds the driver when the rig reads a pending dialog.
@task_item n=10 part=needs
Boundary tests sequenced on records: isolation and process-hygiene properties are each proven once by a named check on the fake agent with no live start, sequenced on records rather than on sleeps; the fake agent's DEL fidelity and its trust-dialog literal and selector-row fidelity ride the rig.
@task_item n=11 part=needs
Test environment rules for the rig's offline checks: a test socket under the short /tmp root, a stamped home from the shared stamp's copy, no mutation run in this chunk.
@task_item n=12 part=needs
No product behaviour change: no new verb, no CLI output, refusal, help or board surface; any change a carried reading forces waits on the founder's placement, and the fired-wake classification stays its own later entry.
@task_standing n=1
convention: each new automated check the rig adds runs inside the existing test suites and fails as a nextest failure, with no separate a11y driver or suite
home: bootstrap-phases-derive-for-route-setup-project
@task_standing n=2
convention: each new test is deterministic, with no retries and no flakiness tolerated
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: each new test runs in CI with no skips allowed
home: 1-a11y-scope-summary
@task_item n=1 part=not_needed
CLI refusal layout, fault, exit-code and --json failure document contracts (unable/hint pairs, hint wording, exit 2/10/11/13/14/20/21, instance-unreachable details, start-holder and revive refusals): the chunk adds no verb and changes no refusal text or document.
@task_item n=2 part=not_needed
Colour and SGR decisions and the viola list board (bold NAME, amber DIALOG, dim stale rows, truncation, captions, separator): the chunk prints nothing on the product's board or colour paths.
@task_item n=3 part=not_needed
viola --help contracts (usage line, group labels, handoff row, served verbs only): the product binary gains no verb and no help row.
@task_item n=4 part=not_needed
viola revive, revive --list, link/unlink and plugin install output contracts: none of these commands' output changes.
@task_item n=5 part=not_needed
Manual screen-reader setups and Orca targets for the cli and tui: the rig is an automated driver, not a manual screen-reader pass.
@task_item n=6 part=not_needed
Absences of a web, graphical or touch frontend, i18n and a rendered page, and the N/A contrast, focus-management and WCAG mapping harness areas: nothing in the chunk touches them.
@task_item n=7 part=not_needed
Log identity tags (process, service_name) and the MCP server frame tool and MCP wait tool contracts: the chunk adds no log line identity and no MCP tool.
@task_item n=8 part=not_needed
Cognitive accessibility tier and reading-level decisions (no AAA escalation, no reading-level gate, fixed copy from design): the chunk writes no user-facing copy.
