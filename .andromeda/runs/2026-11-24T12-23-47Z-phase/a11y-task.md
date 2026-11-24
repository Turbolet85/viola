@task_prose part=chunk
Builds a new viola CLI verb that switches a running viola-wrapped session (a named instance) to a given model and effort.
It records the new pair so a later resume (viola revive, viola up) replays the new pair, not the one named at launch.
It has no side effect on the user's own settings: it writes nothing to the user's or a project's claude settings and does not use the CLI's own model command as its channel.
It adds the verb's argument surface, closed words, exit codes, hints, output lines, --json document, an events-log record of the switch and its process-log lines, each beside the existing tables.
Nothing live: its proof is by tests only, with no real session read, switched or resumed and no read of the operator's real home.
@task_item n=1 part=needs
The new verb on the viola --help surface: its usage line and its row among the served verbs, with its argument surface (instance name, model, effort).
@task_item n=2 part=needs
The closed sets the verb's arguments are checked against: model words and effort levels (low, medium, high, xhigh, max); a word outside them is a usage error with CLI exit code 2 and a refusal on stderr.
@task_item n=3 part=needs
The verb's refusals on stderr: the unable line with its kebab-case reason code beside it, the detail, and the hint line ending every refusal, one hint per code, faults carrying no hint, no stack trace or bare status as error text.
@task_item n=4 part=needs
The verb's --json output: one document, success or failure, with no hint and clean failure documents, beside the existing --json refusal and fault documents.
@task_item n=5 part=needs
The verb's exit codes and exit causes beside the CLI exit code catalogue (2, 10, 11, 13, 14, 20, 21 as they apply), including exit 21 when the named instance is unreachable and its instance-unreachable detail (instance-dead, endpoint-lost, call-deadline, unwrapped-name, socket-dir-refused, strict-modes-failed, server-verify-failed).
@task_item n=6 part=needs
Reaching a named instance from a CLI verb: socket-directory and strict-modes refusals before any frame or write, server-verify-failed when the peer is not the recorded wrapper, peer replies screened to code words, unlisted code words left visible.
@task_item n=7 part=needs
The verb's human result line: static, linear, written in one write, no spinner, no SGR off a colour-capable terminal and never on a failure word, C0/C1 control characters escaped, the printed instance name being the parsed argument.
@task_item n=8 part=needs
The recorded launch pair a resume replays: after a switch, viola revive and viola up start the session on the new model and effort, not the launch pair, including the revive --model / --effort override path and the start refusals of a resumed start (already-live, stale-heartbeat, session-live and the other revive refusals).
@task_item n=9 part=needs
If the switch rides a restart of the session (end and resume), the TUI passthrough of the resumed child holds: zero viola bytes on the human's terminal, run diagnostics never on the terminal, the terminal belonging to the child.
@task_item n=10 part=needs
A switch against a session mid-turn, holding a dialog, or while the human types: human keystrokes are never blocked, a human key takes the wheel, a driver write is refused while the human types, and the dialog hold bounds the driver, not the human.
@task_item n=11 part=needs
Time bounds the switch may wait on (a reply deadline, a resume's live wait, a stop grace) fall on the driver only, never on the human, and none is a harsh timeout without warning.
@task_item n=12 part=needs
The events-log record of the switch: a new event line kind with its fields, beside the existing event line kinds.
@task_item n=13 part=needs
The verb's process-log lines: log identity tags (process, service_name), and the lines kept off the terminal.
@task_item n=14 part=needs
The settings boundary as a check: the switch writes nothing to the user's or project's claude settings, proved by tests against a fixture home, never the operator's real one, with no live reading gating the proof.
@task_item n=15 part=needs
The verb's new fixed CLI strings (result line, refusal copy, hints) come from design copy, are pinned by literal in tests, and name no release.
@task_standing n=1
convention: each new a11y assertion runs inside the existing test suites under nextest in the GitHub Actions push/PR workflow, as a nextest failure, with no skips, no retries and no a11y artifact of its own
home: 9-ci-integration
@task_standing n=2
convention: each new fixed CLI string, caption, refusal and hint is pinned by its literal in a test, and refusal layout is held by literal pins
home: 8-cognitive-accessibility · cognitive-accessibility
@task_standing n=3
convention: each new refusal is a word, a detail and a hint, its reason code beside the unable word, one hint per code, faults with no hint
home: cognitive-accessibility
@task_standing n=4
convention: each new CLI output line is linear and static, escapes control bytes, carries no SGR off a terminal and none beside a failure word
home: aria-patterns-roles · 4-aria-patterns-roles
@task_standing n=5
convention: each new log line carries the log identity tags and never reaches the human's terminal
home: a11y-scope-summary
@task_standing n=6
convention: each new boundary test is sequenced on records, never inspects claude content, and no live reading gates it
home: keyboard-test-harness
@task_item n=1 part=not_needed
The viola list board's rows, captions, colour, stale-row dim, amber DIALOG word and NAME truncation: the chunk prints no board.
@task_item n=2 part=not_needed
The console's keyboard reach of the switch and keyboard navigation of a fleet console: owned by another route entry, not this chunk.
@task_item n=3 part=not_needed
Manual Orca and keyboard passes and the screen-reader targets and setups for the cli and tui: supplemental, gating nothing this chunk builds.
@task_item n=4 part=not_needed
Other verbs' own outputs: viola link and unlink, viola plugin install, viola verify and its interrupted pair, viola wait, viola open's live wait and the compositor and window wait bounds.
@task_item n=5 part=not_needed
Web, graphical, touch, rendered-page and internationalization absences, and the N/A structured-violation, WCAG mapping, contrast and focus-management harness areas.
@task_item n=6 part=not_needed
The claude agents read and the client euid refusal test strategy: the switch neither lists agents nor serves a hook client.
