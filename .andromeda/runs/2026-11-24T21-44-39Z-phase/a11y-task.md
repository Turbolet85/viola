@task_prose part=chunk
Changes where `viola adopt` starts its resumed claude child, so the CLI finds the adopted session; today it spawns in the caller's current folder.
Adds what `viola adopt` says and records when the resumed child ends at once: a closed code word, an exit, a hint and an output line beside the existing refusal tables, an event and a log line, and whether the instance directory and name are released.
If the fix moves where the R5 live-reading script types `viola adopt`, that script changes with it, and its rehearse form reaches the changed path with no claude.
No live start, no real claude, no read of the operator's real Claude config; the folder hypotheses stay unmeasured unless the operator buys a live measurement through the overseer.
@task_file n=1 path=src/cmd/adopt.rs mark=edits
what: the `viola adopt` command: the start of its resumed claude child and its refusal output — edits: the spawn folder of the adopt start, the early-exit handling of the resumed child (its code word, exit, hint, output line, recorded event and log line, release of instance directory and name)
@task_file n=2 path=scripts/r5-adopt.sh mark=edits
what: the R5 live-reading rig script for `viola adopt`, with its live arm and its rehearse form — edits: the folder the live arm types the command in, the rehearse form reaching the changed path
@task_item n=1 part=needs
The spawn folder of the adopt start: the folder the resumed claude child is started in, chosen so the CLI finds the adopted session.
@task_item n=2 part=needs
A new early-exit outcome of `viola adopt`: when the resumed child ends right after the start, viola prints a refusal of the CLI refusal format, an unable line with a plain word and its kebab-case reason code beside it, then a hint line, on stderr in the refusal stderr layout.
@task_item n=3 part=needs
The exit code that early exit returns, taken from the CLI exit-code table, never the child's bare exit status passed through as error text.
@task_item n=4 part=needs
The new code word's hint: exactly one hint per code, the refusal ends with it, no hint names release; if the outcome is a fault instead, it carries no hint.
@task_item n=5 part=needs
Under --json the early exit is one clean failure document with no hint and no stray bytes, of the same shape as the other start refusal documents.
@task_item n=6 part=needs
The new fixed strings (unable word, code, hint, output line) come from the design copy and are pinned by literal in tests; the error message interpolates nothing.
@task_item n=7 part=needs
Refusal and failure words of the new outcome carry no colour and no SGR; no SGR off a terminal or in plain modes.
@task_item n=8 part=needs
Any value the new output line or hint could carry (a folder, a session id) is not printed as a path, and any control byte reaching CLI output is printed escaped.
@task_item n=9 part=needs
The recorded event and log line for the early exit: the event line form and the log identity tags (process, service name); run diagnostics go to the log, never to the terminal.
@task_item n=10 part=needs
The terminal while the resumed child runs and after it ends belongs to the child: viola writes no bytes of its own onto it during the passthrough; its refusal is printed only after the child is gone.
@task_item n=11 part=needs
Telling an early exit from a normal one is a time bound that falls on the driver only, never on the human; no spinner or unwarned timeout.
@task_item n=12 part=needs
The state after an early exit: whether the instance directory and the adopted name are released, said by the outcome and kept consistent with what the board and revive later read.
@task_item n=13 part=needs
The R5 rig's live readings never gate CI; its rehearse form runs with no claude and is the only form a test or CI may fire.
@task_standing n=1
convention: each new a11y assertion runs inside the existing test suites as a nextest failure, never in a separate a11y driver, suite or artifact
home: bootstrap-phases-derive-for-route-setup-project · 9-ci-integration
@task_standing n=2
convention: each new test passes with no skips, no retries and zero flakiness; a gate breach fails the build
home: 10-slo-invariants-a11y-budgets
@task_standing n=3
convention: each new boundary test is sequenced on records, never on timing, and never inspects claude content
home: keyboard-test-harness
@task_item n=1 part=not_needed
The `viola list` board's caption, order, colour, dim stale rows and NAME truncation: the chunk changes no board output.
@task_item n=2 part=not_needed
`viola --help` rows (usage line, group labels, handoff row, served verbs): no verb is added or regrouped.
@task_item n=3 part=not_needed
`viola link`/`unlink`, `viola plugin install`, `viola revive --list`, `viola switch`, `viola up`, `viola verify` and `viola wait` output contracts: other commands' lines, untouched.
@task_item n=4 part=not_needed
MCP server frame tools, hook event and dialog calls, and frame verb refusals: the adopt start sends no frame over those channels anew.
@task_item n=5 part=not_needed
The send and answer refusals and the dialog hold: the chunk sends no prompt and answers no dialog.
@task_item n=6 part=not_needed
Screen-reader targets and the manual Orca and keyboard passes: supplemental, gating nothing, not owed by this fix.
@task_item n=7 part=not_needed
Absences of a web or graphical frontend, touch surface, rendered page, i18n, and the structured-violation JSON schema: nothing of these is reached.
