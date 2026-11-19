@task_prose part=session
The session changed test-only code alone: the live rig and the fake agent; no product crate, no viola verb, no CLI output, exit code, hint, refusal or styling of viola changed.
It added the rig's SF arm (live labels SF and SF-2): one contained live StopFailure start on claude 2.1.328, inside a network-closed bwrap sandbox, refused with the word refused: sandbox before its ledger line when the sandbox is absent, and refused no-runtime-dir unless XDG_RUNTIME_DIR is absolute and outside the temp root.
It measured one live StopFailure turn end: turn-end stop-failure, the CLI's error word server_error read as cause server-error, no unknown-value line; one start, of the connection-lost class, on one version; the other twelve error words were not seen live.
It added the rig's StopFailure capture (key names, the error word of the CLI's closed 13 else other, two byte lengths, never a text) and a read of the Claude input box row's cell attributes (dim, italic, bold, inverse, fg colour words, plain), never its text, after the turn end.
It settled the StopFailure exception (registered with no capability-ledger row and no verify probe) as outcome B, kept on the founder's word on the green live reading (2.1.328, 2026-11-19, start SF, word server_error), no longer provisional; outcome A is not opened.
It keyed the rig's kept record dir by CLI version first, and gave the fake agent two argv options, --turn-fail and --settings-hooks; nothing retired.
@task_file n=1 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's isolation of its child: the scratch home, the kept record per CLI version, the network-closed sandbox argv and the runtime-dir precondition (test-only)
@task_file n=2 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig's arms, plans and summary words: the new SF arm, the sandbox probe and its refusals, the turn-end and cause reading (test-only)
@task_file n=3 path=crates/viola-e2e/src/rig/capture.rs
what: the live rig's closed capture of hook payloads: now a StopFailure line with key names, the closed error word and two lengths, never a text (test-only)
@task_file n=4 path=crates/viola-e2e/src/rig/screen.rs
what: the live rig's screen model of the Claude terminal: now cell attribute words of the input box row, never a cell's text (test-only)
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig's own command line: rehearse sf with --contain, live SF and SF-2 (test-only, not a viola verb)
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig rehearsals over the fake agent: four sf rehearsals added (test-only)
@task_file n=7 path=src/bin/viola-fake-agent.rs
what: the fake Claude agent the tests run in place of claude: argv options --turn-fail and --settings-hooks added (test-only)
@task_item n=1 part=touches
Any row whose cell names the StopFailure exception (StopFailure registered with no capability-ledger row and no verify probe, a synthetic StopFailure payload, or the exception owned by a live StopFailure payload still to come) is stale: the exception is now settled as outcome B on the founder's word, citing the live reading (claude 2.1.328, 2026-11-19, start SF, word server_error), no longer provisional.
@task_item n=2 part=touches
Indirect, through the rig's turn-end reading: rows on how a turn end and its cause reach the user (viola wait turn-ended output, turn-ended not a send outcome, the cause words) now stand beside one live StopFailure reading where the CLI word server_error mapped to cause server-error; no output changed, only a picker reading a row that claims StopFailure is unseen live should flag it.
@task_item n=3 part=touches
Indirect, through the rig's runtime-dir precondition: under an unset XDG_RUNTIME_DIR viola put its socket directory under the temp root, and viola send from outside the sandbox read exit 21 instance-unreachable with detail instance-dead; rows on the socket directory's location or the instance-dead detail hold true as worded, a row that names where the socket directory falls back may want the reading beside it.
@task_item n=4 part=touches
Indirect, through the rig's screen model: the rig now reads the Claude input box row's cell attributes (dim, italic, bold, inverse, fg colour) never text, an exception to not parsing screen content; a row of this master that forbids reading terminal screen content or ties attribute words to meaning should be checked against it.
@task_item n=5 part=touches
Indirect, through the rig's capture: the rig keeps the CLI's error word only from its closed list of 13 (else other) and text lengths, never text; a row that says no CLI error text is ever kept or shown should hold true as worded, flag only if it names the rig.
@task_item n=6 part=touches
Indirect, through the kept record dir keyed by CLI version: a row of viola verify --record or the recorded fixture sets that names how recorded sets per CLI version are kept side by side may want the rig's per-version kept dir beside it; viola verify itself did not change.
@task_item n=1 part=untouched
The viola CLI verbs and their output contracts (list, run, revive, send, wait, last, answer, link, unlink, pause, release, plugin install, help, verify): no verb, line form, document, column or word changed.
@task_item n=2 part=untouched
The CLI exit codes, the refusal line form, the shared refusal word column and the hints (one per cause, last on stderr, naming the next verb, never quoting or locating): the rig's own words refused: sandbox and refused: usage are a test tool's, not viola's.
@task_item n=3 part=untouched
Token rendering, SGR styling, colour depth, TTY detection and the no-spinner, no-banner, no-footer, no-pager rules over viola's own output: the attribute words the rig reads are Claude's terminal cells, not viola's styling.
@task_item n=4 part=untouched
The board, the BAY line, the readback mirror and the send lines: untouched.
@task_item n=5 part=untouched
The MCP server, its tools and tool results, and the hidden verbs: untouched.
@task_item n=6 part=untouched
The web surface absence and the deep-link absence: untouched.
@task_item n=7 part=untouched
CLI parsing (clap derive), the global flags, ViolaName targets and the prompt text from stdin or file: the fake agent's argv options are a test binary's, not viola's.
