@task_prose part=session
The session added a new CLI verb, viola switch <name> [--model M] [--effort E] [--json], which ends a running session's child and resumes the same session id on the new model and effort pair. The pair is recorded so a later revive replays it, and nothing is written to the user's own settings.
It added a new wrapper channel method, switch. Its refusals are human-typing, turn-running, dialog-pending, unverified-cli, no-session, child-not-ended and resume-failed. The served channel methods go from 9 to 10 requests plus the one notification.
The outer-PTY pump can now respawn its child under the same host terminal. It opens a new pseudo-terminal pair sized to the host. The human's keystroke copy persists across the respawn, and viola writes no byte to the terminal.
It added a new compiled time bound, RESPAWN_REAP_BOUND, at 10 s. A child that has not exited inside it is not replaced, and the switch is refused not-delivered child-not-ended.
viola --help's setup: row now reads open, run, revive, adopt, up, switch, verify, plugin install (17 to 18 served verbs). send gains four hint causes, and a send is refused turn-running while a switch stands.
Everything was proven over the fake agent only and no capability is claimed. Forks F1, F3, F5 and F6 stand PROVISIONAL on the delegate overseer's word. The real-CLI readings R6a and R6b are carried, not fired.
@task_file n=1 path=src/cmd/switch.rs
what: new — the viola switch CLI verb: its strict-modes and socket-directory refusals before any frame, its unable/hint refusal pairs, its exit codes and its one --json document
@task_file n=2 path=src/run/switch.rs
what: new — the wrapper's switch channel method: the refusal order (human typing, turn running, dialog pending, unverified CLI, no session), the respawn under the reap time bound, the launch line appended per switch
@task_file n=3 path=tests/cli_switch.rs
what: new — the switch verb's integration tests: literal pins of its refusal layout and its result line, the usage error, the user settings kept byte-equal
@task_file n=4 path=crates/viola-pty/src/pump.rs
what: edited — the outer-PTY boundary pump (the portable-pty driver): human keystrokes to the child, the child's bytes to the terminal, the panic arm; now respawns the child under the same host terminal within RESPAWN_REAP_BOUND
@task_file n=5 path=crates/viola-pty/src/lib.rs
what: edited — the PTY internal crate's public surface: exports the respawn handle, its outcomes and the reap time bound
@task_file n=6 path=crates/viola-channel/src/server.rs
what: edited — the IPC channel server's closed method-label list, 10 to 11 labels (switch added)
@task_file n=7 path=src/cmd/run.rs
what: edited — the viola run wrapper behind the TUI passthrough: routes channel calls to the switch slot and threads the respawn handle through pump_child
@task_file n=8 path=src/cmd/verify/typed/run.rs
what: edited — viola verify's run step that spawns the child through the pump (passes a respawn handle no one uses)
@task_file n=9 path=src/cmd/mod.rs
what: edited — the CLI verb dispatch and the viola --help literal, its setup: group row gaining switch after up
@task_file n=10 path=src/run/mod.rs
what: edited — the wrapper's module list (switch module added)
@task_file n=11 path=src/run/send.rs
what: edited — the confirmed send: refused turn-running while a switch stands
@task_file n=12 path=src/run/send/slot.rs
what: edited — the send slot: the switch mark that holds sends off, and the one-shot screen of the old child's late session-end after a respawn
@task_file n=13 path=src/human.rs
what: edited — the human output module's surface (exports the switched result line)
@task_file n=14 path=src/human/verbs.rs
what: edited — the per-verb human result lines: the new one-line switched line on stdout
@task_file n=15 path=src/human/hints.rs
what: edited — the hint table, one hint per refusal code and cause: four new not-delivered causes
@task_file n=16 path=tests/cli_output_plain.rs
what: edited — the plain-output discipline check (no SGR, plain ASCII) and the help's served-verbs list
@task_file n=17 path=tests/contract_error_bodies.rs
what: edited — the error-body census, each body with its writer and its witness: three new CLI documents for switch, and the channel method list 10 to 11
@task_file n=18 path=tests/cross_exit_causes.rs
what: edited — the exit-cause table of refusals: 1 to 5 rows (dialog-pending, no-session, child-not-ended, resume-failed, each exit 13)
@task_file n=19 path=tests/support/home.rs
what: edited — the test support's stamped home and wrapper helpers: adopt as a given user home
@task_item n=1 part=touches
viola --help: the setup: group row now lists switch after up, and the served verbs go from 17 to 18. Check whether a cli-help row (served verbs only, group labels, usage line) holds the setup row's verbs or their count. A row that only says help lists served verbs stays true.
@task_item n=2 part=touches
Time bounds: a new compiled bound, RESPAWN_REAP_BOUND at 10 s (PROVISIONAL, fork F6), falls on the wrapper and never on the human. Rows that count the time bounds (the Eight time bounds list rows, time bounds fall on the driver only, time bounds never fall on the human) hold a count that is now one more. The report says the wrap directive leaves those list rows unamended and wants a row of its own for the bound.
@task_item n=3 part=touches
Send refused while the human types: the convention now also covers switch, which is refused human-typing with exit 10 while the human holds the wheel. The human key takes the wheel, and the wheel rules are read by one more verb.
@task_item n=4 part=touches
CLI exit codes: viola switch exits 0, 10, 12, 13, 14, 20 or 21. Neither --model nor --effort, a bad effort or a bad model is a usage error with exit 2. This touches the exit-code rows (2, 10, 13, 14, 20, 21) and the usage-error exit 2 row wherever they list the verbs they cover.
@task_item n=5 part=touches
Error bodies have a writer and a witness: three new CLI error bodies for switch (human-typing 10, unverified-cli 12, not-delivered 13) are added to the census. The census's channel method list goes from 10 to 11.
@task_item n=6 part=touches
One hint per code, no hint names release, unlisted code words stay visible, reason codes beside unable, every refusal ends with a hint: the hint table gains four not-delivered causes (dialog-pending, no-session, child-not-ended, resume-failed), each with its hint and none naming release.
@task_item n=7 part=touches
CLI refusal output (unable line, hint line, stderr layout, --json document, exit code): switch's refusals follow the unable <name> <code> <detail> pair, for example not-delivered turn-running. Under --json they print one document with no hint.
@task_item n=8 part=touches
Fixed CLI strings pinned by literal and refusal layout held by literal pins: the new switched line and switch's refusal pairs are pinned by literals in the switch integration tests and the help literal.
@task_item n=9 part=touches
TUI passthrough (viola terminal bytes, own bytes, zero viola bytes on the terminal, human keystrokes, entry): the pump now replaces the child in place under the same host terminal. It opens a new PTY pair at the host's current size, keeps the host-input copy and the write queue, restarts the output copy, and writes no viola byte.
@task_item n=10 part=touches
portable-pty as the outer-PTY boundary driver, and the TUI terminal belongs to the child: the boundary can now end its own child and hand the terminal to a new one. It kills only the child its own PTY handle holds (PROVISIONAL, fork F1).
@task_item n=11 part=touches
TUI passthrough pump_child error and vt100 feed panic rows: the pump's panic arm now kills the current child, which after a respawn is the new one. A respawn whose spawn fails ends the pump with the old exit.
@task_item n=12 part=touches
Plugin and IPC channel not assertable, and internal crates not assertable: the IPC channel gains a served method, switch (9 to 10 requests plus one notification). The PTY internal crate gains a respawn seam. The report marks both a11y n/a.
@task_item n=13 part=touches
Strict-modes refusal before any write, socket-directory refusal before any frame, the frame verb refusals and the instance-unreachable details (instance-dead, server-verify-failed, socket-dir-refused, strict-modes-failed): switch is a new frame verb that runs the untrusted-home refusal, then the live endpoint, then the connect check, and refuses exit 21 for a gone instance.
@task_item n=14 part=touches
viola send refusal rows: a send made while a switch stands is refused not-delivered turn-running until the new child's session-start. This is a new cause for an existing word.
@task_item n=15 part=touches
Log identity tags (process): the switch verb logs as process cli into cli-<name>.ndjson under a cli.switch dispatch span.
@task_item n=16 part=touches
Printed names are the parsed arguments: the switched line prints the parsed name, followed by the model and effort the wrapper confirmed.
@task_item n=17 part=touches
Live readings never gate: the real-CLI readings R6a (idle switch of a real session, SessionEnd on SIGHUP) and R6b (a real switch refused mid-turn) are carried to the founder's host, not fired, and gate nothing.
@task_fact n=1 slug=cognitive-accessibility
fact: RESPAWN_REAP_BOUND is 10 s. A switch waits at most that long for the old child to exit after its hang-up, and a child that has not exited is not replaced: the switch is refused not-delivered child-not-ended with exit 13. The bound falls on the wrapper, never on the human (PROVISIONAL, fork F6).
@task_fact n=2 slug=cognitive-accessibility
fact: viola switch refuses with human-typing (exit 10), unverified-cli (exit 12) or not-delivered (exit 13) with detail turn-running, dialog-pending, no-session, child-not-ended or resume-failed. The first rung in that order wins, each cause carries its own hint, and no hint names release (PROVISIONAL, fork F3).
@task_fact n=3 slug=1-a11y-scope-summary
fact: viola switch prints one line, <name>  switched  <model> <effort>, on stdout in plain ASCII with no SGR. Under --json it prints one document, {"ok":{"model","effort"}}.
@task_fact n=4 slug=1-a11y-scope-summary
fact: At a switch the wrapper replaces its child under the same host terminal, in a new PTY pair at the host's current size. viola writes no byte to the terminal, and the human's keystrokes keep flowing across the respawn (PROVISIONAL, fork F1).
@task_fact n=5 slug=4-aria-patterns-roles
fact: A switch is refused while the human types, mid-turn and with a dialog pending. While a switch stands, a send is refused not-delivered turn-running until the new child's session-start (PROVISIONAL, fork F3).
@task_fact n=6 slug=a11y-scope-summary
fact: viola switch needs at least one of --model and --effort. Neither one, a bad effort or a bad model is a usage error with exit 2.
@task_item n=1 part=untouched
The viola list board and its caption, order, empty board, colour, stale-row dim and unwrapped separator rows: no board output changed.
@task_item n=2 part=untouched
Colour and emphasis rows for a colour-capable terminal (amber DIALOG word, bold NAME, NAME truncation, failure words never coloured): the session adds no coloured output.
@task_item n=3 part=untouched
Screen-reader targets and the manual Orca and keyboard passes: their setups and their supplemental status are unchanged.
@task_item n=4 part=untouched
CI integration, the SLO and gate decisions (fail-fast gate, zero flakiness, no skips, coverage and doctest required, mutation audit): no gate, workflow or threshold moved.
@task_item n=5 part=untouched
viola plugin install, viola link and unlink, viola wait, viola verify interrupted, viola up and the revive --list rows: none of these outputs changed.
@task_item n=6 part=untouched
viola revive --json refusal rows and the start-against-a-holder refusals: revive replays the switched pair, but its output and its refusal words are unchanged. A shared no-session token is no touch.
@task_item n=7 part=untouched
MCP server frame tools and the MCP wait tool: switch is CLI-only and gains no MCP tool.
@task_item n=8 part=untouched
Hook event and dialog calls, the dialog hold bound and config.json dialog_hold_s: unchanged. The switch only reads the pending dialog from the wrapper's snapshot.
@task_item n=9 part=untouched
The absences (internationalization, touch surface, web or graphical frontend, rendered page, a time limit on the user): all still stand. The new bound falls on the wrapper, not on the user.
@task_item n=10 part=untouched
The N/A areas (keyboard navigation for a web page, contrast, focus management, WCAG criteria mapping) and the structured violation JSON rows: nothing there moved.
