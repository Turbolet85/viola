@task_prose part=chunk
`viola console`, the terminal fleet console, gains actions over the board view the previous chunk built: a selection of one board row and verbs reached from it by keyboard alone.
Each action runs through the CLI verb it names (open a child, focus, send, answer, last, pause, release, revive, and the switch's console leg), with the same outcome and refusal as the verb at a shell and no second code path.
A send shows the frame verb's own readback, never the view's STATUS cell; a refused action shows the verb's `unable` line on the console's own surface.
Every mode the actions add is left by keyboard alone (no key trap), and a key-sequence decoder replaces the scaffold's one-class-per-read key decoder where the actions need it.
Its first pass makes no live reading: a capability that needs one is claimed gated, the reading the operator's to grant; the view's content changes only as far as a selection and an action's result need.
@task_item n=1 part=needs
Keyboard-only reach of every console verb: each action and each mode it opens (selection, the send's text entry, any confirm) is entered and left by keyboard alone, with no key trap and no mouse or pointer path required.
@task_item n=2 part=needs
The console is keyboard-interactive while the CLI verbs it drives stay non-interactive: the console reads keys from its own terminal, and each verb it reaches still takes no interactive input of its own.
@task_item n=3 part=needs
A refused action prints the CLI refusal's unable line on the console surface, with its kebab-case reason code beside the plain word, followed by its hint line, exactly as the verb prints it at a shell; the `unknown` refusal and faults carry no hint.
@task_item n=4 part=needs
The send path's readback: the confirmed-send critical path, the send frame verb's reply (`ok` or a refusal code) shown as the result, and the send refusals input-not-ready and no-prompt-submitted, including send refused while the human types.
@task_item n=5 part=needs
Answering a pending dialog from the console follows the dialog-pending-then-answer critical path, and the human-takes-the-wheel path stays the human's: a human key takes the wheel and human keystrokes are never blocked.
@task_item n=6 part=needs
The revive action's human refusals show as the revive unable/hint pair, and the switch action's result shows as the viola switch result line, both as the CLI verb prints them.
@task_item n=7 part=needs
No hint the console prints names release, even though the console, driven by the human, may reach the release verb.
@task_item n=8 part=needs
Time bounds of an action (a send's reply slower than the one-second re-read, the agents-thread STATUS read at most every 5 s, any wait or reap bound of a verb it calls) fall on the driver, never on the human; no time limit on the user and no harsh timeout without warning.
@task_item n=9 part=needs
No spinner or animated progress while an action waits for its verb's reply; the result appears as a static line.
@task_item n=10 part=needs
Colour stays a second cue on the console surface: no information by colour alone, a word beside every SGR span, refusal and failure words never coloured, no SGR without a colour-capable terminal.
@task_item n=11 part=needs
C0/C1 control characters in any verb output or peer reply the console shows (a readback, a refusal detail, a row's name) are escaped before printing; peer replies are screened to code words and unlisted code words stay visible.
@task_item n=12 part=needs
A selection names its row by the board's item, never by the drawn text (a cut line ends `...`, an unwrapped NAME is not cut separately); NAME truncation and the unwrapped separator keep their board rules when a selected row is shown.
@task_item n=13 part=needs
The key-sequence decoder treats terminal sequences (focus, mouse, resize, other escape sequences) as non-editing input: they never move the wheel and never fire a verb by themselves.
@task_item n=14 part=needs
Fixed console strings an action adds (a selection marker, a prompt for the send's text, a confirm line, a mode hint) come from design copy and are pinned by literal in tests.
@task_item n=15 part=needs
The console's tests drive it with the keystroke driver over its own pseudo-terminal (the TUI boundary driver), read state rather than parse screen content beyond signatures, and stay at TUI boundary assertions.
@task_item n=16 part=needs
Live readings never gate: the gated capability clauses of this chunk are claimed gated, the manual keyboard and screen-reader passes are supplements that gate nothing.
@task_standing n=1
convention: a11y assertions for the console's actions live inside the existing test suites, not a separate a11y driver or suite
home: 1-a11y-scope-summary · 9-ci-integration
@task_standing n=2
convention: every new test is deterministic, with zero flakiness and no retries, and the CI test gate allows no skips
home: 10-slo-invariants-a11y-budgets · 1-a11y-scope-summary
@task_standing n=3
convention: every fixed CLI or console string a new test checks is pinned by its literal
home: 8-cognitive-accessibility
@task_standing n=4
convention: a11y failures are ordinary nextest failures in the GitHub Actions push/PR workflow and a breach fails the build
home: bootstrap-phases-derive-for-route-setup-project · 10-slo-invariants-a11y-budgets
@task_item n=1 part=not_needed
The web, graphical, touch and internationalisation absences and their N/A harness areas (contrast verification, focus management harness, WCAG criteria mapping): the console is a terminal surface with English copy only.
@task_item n=2 part=not_needed
Hook event and dialog call refusals and the MCP server frame tools: the console acts through the CLI verbs, not through the hook calls or the MCP tools.
@task_item n=3 part=not_needed
The viola link and unlink, viola plugin install, viola up and viola verify output contracts and the interrupted-verify rows: none of these verbs is among the actions the console reaches.
@task_item n=4 part=not_needed
The manual screen-reader setup rows (Orca targets for the cli and the tui) as build work: this chunk adds no screen-reader pass, and its live readings are the operator's.
