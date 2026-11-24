@task_prose part=chunk
Adds a new flat viola CLI verb that switches a running viola-wrapped session (a named instance) to a given model and effort.
It records the new model and effort pair so that a later resume through viola revive or viola up replays the new pair, not the one named at launch.
The verb has no side effect on the user's own settings and does not use the CLI's own model command as its channel.
It adds the verb's argument surface, closed words, exit codes, hints, output lines, --json document, events-log record and process-log lines beside the existing CLI verb, exit-cause, closed-detail and event-kind tables.
@task_item n=1 part=needs
The register of CLI command names gains one new verb name, a flat top-level verb beside run, revive, up, open, send and the rest, never a nested subcommand group.
@task_item n=2 part=needs
The new verb's human output: a result line on stderr confirming the switch with its context (the instance name and the new model and effort), in the fixed phraseology vocabulary, ASCII-only, with no check-mark or emoji prefix.
@task_item n=3 part=needs
Nothing announces success before confirmation: the verb reports the switch only once it is confirmed to have taken effect for the instance, never optimistically.
@task_item n=4 part=needs
The new verb's --json result is one document on stdout, success or refusal, with clean failure documents and no hint line under --json.
@task_item n=5 part=needs
The new verb's exit codes join the CLI exit-code table: 0 on a confirmed switch, 1 for its own refusals with closed refusal codes, 2 for a usage error such as an unknown model word or effort level, and 21 with its existing causes where the instance cannot be reached.
@task_item n=6 part=needs
Instance-unreachable refusals of the new verb use the existing exit-21 cause catalogue (unwrapped-name, instance-dead, endpoint-lost, socket-dir-refused, strict-modes-failed, server-verify-failed) and their one hint per cause, in human and --json form.
@task_item n=7 part=needs
Unwrapped sessions are never a target: a name that is no viola-wrapped instance is refused as unwrapped-name, never switched.
@task_item n=8 part=needs
Refusal lines of the new verb follow the CLI refusal-line form (an unable line and at most one hint line on stderr), hints in human mode only, no hint for faults or unknown, no hint naming viola release, hints never quoting text, and no internals, paths or pids in errors or hints.
@task_item n=9 part=needs
Closed argument words: the model word and effort level the verb accepts are closed sets (efforts low, medium, high, xhigh, max), named in its argument surface and refused with a usage error outside them.
@task_item n=10 part=needs
The verb's strict-modes checks over the instance state follow the existing strict-modes refusal pair and its --json document, as the frame verbs do.
@task_item n=11 part=needs
viola revive and viola up after a switch replay the recorded new pair on resume; their outputs and refusals otherwise stay as they are, and a revive given --model or --effort still overrides the recorded pair.
@task_item n=12 part=needs
Output streams: data on stdout, messages on stderr; the colour decision order (--json, NO_COLOR, stdout not a terminal, TERM=dumb) governs any styling of the new verb's lines, with colour only as a second cue.
@task_item n=13 part=needs
Control characters in any echoed instance name or value are escaped in the verb's human output and in --json message mode.
@task_item n=14 part=needs
No interactive prompt or pager, no spinner or elapsed-time counter and no stack trace in the new verb; any progress uses the one step-counter pattern.
@task_item n=15 part=needs
viola --help lists the new verb in its verb groups.
@task_item n=16 part=needs
The switch is CLI-only in this chunk's conservative arm: the verb writes nothing to the user's settings files and reaches no CLI model command; the console's keyboard reach of the switch is not built here.
@task_standing n=1
convention: every new verb's output lines, result line and --json document are stated as a CLI verb-output contract beside the existing verbs' rows
home: surface-cli · surface
@task_standing n=2
convention: every new verb's exit codes and closed --json refusal codes join the CLI exit-code table and refusal-code lists, each with its fixed hint
home: surface-cli · surface
@task_standing n=3
convention: every new human output line obeys the anti-patterns: colour carries meaning only as a second cue, confirmations carry context, no internals in errors, controls escaped, plain output modes
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
The web-page surface and rendered-page absences, and the self-validation token tests: viola renders no page and this chunk adds none.
@task_item n=2 part=not_needed
viola verify output and refusals, stamps and trust records: the chunk adds no verification run.
@task_item n=3 part=not_needed
viola link and unlink output and their --json ok documents: the chunk links no pair.
@task_item n=4 part=not_needed
viola focus exit-1 codes, placement words and window refusals: the chunk opens, places or focuses no window.
@task_item n=5 part=not_needed
viola wait output forms and viola last output: the chunk waits on no event and reads no last message.
@task_item n=6 part=not_needed
Pause and release as control transfer, the handoff transfer marker and handoff blue: the switch transfers no control between human and driver.
@task_item n=7 part=not_needed
viola plugin install output and its strict-modes refusal: the chunk installs no plugin.
