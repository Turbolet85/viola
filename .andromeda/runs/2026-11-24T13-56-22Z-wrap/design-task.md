@task_prose part=session
Added a new CLI verb, viola switch <name> with --model and/or --effort and --json, that moves a running wrapped session to a new model and effort pair in place: the wrapper ends its own child and resumes the same session id on the new pair, its name, slot and wrapper pid unchanged; nothing is written to the user's own settings.
Added a new wrapper channel method switch, refused in order: human-typing (exit 10), not-delivered turn-running, not-delivered dialog-pending, unverified-cli (exit 12), not-delivered no-session, then at the respawn not-delivered child-not-ended or not-delivered resume-failed (each exit 13); a call with neither half or a bad half is invalid params.
On success the verb prints one ASCII line, name then switched then the model and the effort, on stdout; --json prints the pair document; neither flag or a bad value is a usage error, exit 2.
A send is refused not-delivered turn-running while a switch stands; viola --help lists switch in the setup group after up; a revive after a switch replays the switched pair.
Every reading is over the fake agent alone; no capability is claimed; the forks F1 (in-place restart by a channel method), F3 (the refusal rungs), F5 (CLI-only, no MCP tool) and F6 (a 10 s reap bound) stand PROVISIONAL on the delegate overseer's word.
@task_file n=1 path=src/cmd/switch.rs
what: new file: the viola switch CLI verb, its arguments, its frame-verb checks first, its call to the wrapper and its refusal lines and exit codes
@task_file n=2 path=src/run/switch.rs
what: new file: the wrapper's switch channel method, its parameter checks, its refusal order and its respawn of the child on the new pair
@task_file n=3 path=tests/cli_switch.rs
what: new file: the integration checks of viola switch, its output line, its refusals, its usage errors and the revive after a switch
@task_file n=4 path=crates/viola-pty/src/pump.rs
what: edited: the PTY pump between the host terminal and the child, now able to end its child and swap in a respawned one under the same host terminal, with a reap time bound
@task_file n=5 path=crates/viola-pty/src/lib.rs
what: edited: the PTY crate's public surface, exporting the respawn handle and its outcome
@task_file n=6 path=crates/viola-channel/src/server.rs
what: edited: the wrapper channel server and its closed list of method labels, gaining switch
@task_file n=7 path=src/cmd/run.rs
what: edited: viola run, the wrapper itself, its channel dispatch now routing switch and its pump given the respawn handle
@task_file n=8 path=src/cmd/verify/typed/run.rs
what: edited: viola verify's typed run spawner, passing the pump a respawn handle no one requests on
@task_file n=9 path=src/cmd/mod.rs
what: edited: the CLI verb table and dispatch, gaining the switch verb and its help row
@task_file n=10 path=src/run/mod.rs
what: edited: the wrapper's run module list, gaining the switch module
@task_file n=11 path=src/run/send.rs
what: edited: the wrapper's send method, refused turn-running while a switch stands
@task_file n=12 path=src/run/send/slot.rs
what: edited: the send slot and flight, holding the switch mark and screening the old child's late session-end after a switch
@task_file n=13 path=src/human.rs
what: edited: the human-output module, exposing the switched line
@task_file n=14 path=src/human/verbs.rs
what: edited: the per-verb human result lines, gaining the switched line
@task_file n=15 path=src/human/hints.rs
what: edited: the refusal hint table, gaining hints for the not-delivered causes dialog-pending, no-session, child-not-ended and resume-failed
@task_file n=16 path=tests/cli_output_plain.rs
what: edited: the plain-output checks of the help text and its served verbs, gaining switch
@task_file n=17 path=tests/contract_error_bodies.rs
what: edited: the census of error bodies and of the channel's served methods, gaining switch's refusals and the switch method
@task_file n=18 path=tests/cross_exit_causes.rs
what: edited: the table of refusal causes and their exit codes, gaining the four new not-delivered causes at exit 13
@task_file n=19 path=tests/support/home.rs
what: edited: the test support for a stamped home and adopted wrappers, gaining an adopt with a given user home
@task_item n=1 part=touches
The viola --help verb groups (contract/help-output/verb-groups and contract/viola-help/verb-groups): the setup row now reads open, run, revive, adopt, up, switch, verify, plugin install; both rows go stale.
@task_item n=2 part=touches
The command names (name/command/*): the name switch is now taken by a served verb, with no row of its own yet.
@task_item n=3 part=touches
Flat CLI verbs (decision/flat-cli-verbs): a new flat verb, switch, joins them.
@task_item n=4 part=touches
The CLI exit codes (contract/cli-exit-codes/10-14, /10, /12, /13, /2, /20, /21): switch exits 0, 10, 12, 13, 14, 20 and 21, and 2 for neither flag or a bad value; exit 13 gains the not-delivered causes dialog-pending, no-session, child-not-ended and resume-failed.
@task_item n=5 part=touches
The refusal hints (contract/refusal-hints/*): four new not-delivered causes get hints (dialog-pending, no-session, child-not-ended, resume-failed); human-typing, unverified-cli and not-delivered turn-running now also answer switch.
@task_item n=6 part=touches
The send turn-running refusal (contract/refusal-hints/not-delivered-turn-running and the send readback rows): a send is now also refused turn-running from a switch's request until the new child's session start.
@task_item n=7 part=touches
No hint names viola release (decision/no-hint-names-release and decision/no-hint-names-viola-release): the new hints keep it; a rule over the hint table this session grew.
@task_item n=8 part=touches
The CLI verb output rows (contract/cli-verb-output/*): a new verb output, name then switched then model and effort, two-space separated, one ASCII line, no SGR, on stdout; no row for it yet.
@task_item n=9 part=touches
Confirmation and success rules (decision/no-success-before-confirmation, decision/confirmations-carry-context, decision/no-optimistic-send-confirmation): the switched line names the new pair and is printed only on the wrapper's ok after the respawn.
@task_item n=10 part=touches
The --json document rules (decision/json-one-document-on-stdout, decision/json-failure-documents-clean, contract/cli-catch-site/json-flag-scope): switch --json prints the pair document or a refusal document; its only exit 1 is the catch site's internal-error (contract/cli-catch-site/json-internal-error).
@task_item n=11 part=touches
The wrapper fault detail (contract/wrapper-fault-detail/other-invalid-params): a switch call with neither half, a half of another type or one failing its check is -32602 with no data.
@task_item n=12 part=touches
The exit 21 causes (contract/exit-21-cause/* and contract/exit-21-json-cause/*): switch reaches them, a gone instance refused instance-dead.
@task_item n=13 part=touches
The frame-verb and strict-modes refusals (contract/frame-verb-refusals/strict-modes, contract/frame-verb-refusals/socket-dir, contract/strict-modes-refusal/*): switch runs the untrusted-home refusal first, then the live endpoint, then the connect checks.
@task_item n=14 part=touches
CLI verification (contract/cli-verification/unverified-cli and contract/refusal-hints/unverified-cli): a switch on an unverified CLI is refused unverified-cli, exit 12.
@task_item n=15 part=touches
Dialog-pending (decision/dialog-pending-cocked-strip, contract/flight-progress-strip/dialog): a switch with a dialog pending is refused not-delivered dialog-pending and respawns nothing.
@task_item n=16 part=touches
The human-typing wheel (contract/flight-progress-strip/wheel, decision/wheel-as-words, contract/refusal-hints/human-typing): a switch while the human types is refused human-typing, exit 10.
@task_item n=17 part=touches
Stable session slots and one strip per instance (decision/stable-session-slots, decision/one-strip-per-run-instance): a switch keeps the name, the wrapper and its slot; only the child is replaced.
@task_item n=18 part=touches
viola run silent while wrapped (decision/run-prints-nothing-while-wrapped, contract/viola-run-output/child-started): viola writes no byte to the terminal at a respawn; the child is respawned at the host's current size under the same host terminal.
@task_item n=19 part=touches
viola revive (contract/viola-revive-refusals/no-session, contract/viola-revive-output/after-child-start): a revive with no pair after a switch resumes on the switched pair; switch's no-session is its own refusal beside revive's.
@task_item n=20 part=touches
Peer reply screening (contract/peer-reply-screening/*): the CLI now reads the wrapper's new refusal code words and details for switch.
@task_item n=21 part=touches
ASCII and plain output (decision/cli-ascii-only, decision/ascii-only-human-output, decision/plain-output-modes, decision/stdout-data-stderr-messages): the switched line and the switch refusal lines follow them.
@task_fact n=1 slug=surface-cli
fact: The command name switch is occupied: viola switch <name> [--model MODEL] [--effort EFFORT] [--json], a setup verb listed after up.
@task_fact n=2 slug=surface-cli
fact: viola switch prints on success one ASCII line on stdout, the name, the word switched, then the model and the effort, two spaces between; --json prints the pair document {model, effort}.
@task_fact n=3 slug=surface-cli
fact: viola switch needs at least one of --model and --effort; neither, or a value failing its check, is a usage error, exit 2.
@task_fact n=4 slug=surface
fact: viola switch refusals, the first rung winning, none ending the child: human-typing (exit 10), not-delivered turn-running, not-delivered dialog-pending, unverified-cli (exit 12), not-delivered no-session, then at the respawn not-delivered child-not-ended and not-delivered resume-failed (each exit 13); PROVISIONAL on fork F3.
@task_fact n=5 slug=surface
fact: The refusal hint table holds hints for the not-delivered causes dialog-pending, no-session, child-not-ended and resume-failed, none naming viola release.
@task_fact n=6 slug=surface-cli
fact: A send is refused not-delivered turn-running from a switch's request until the new child's session start.
@task_fact n=7 slug=surface-cli
fact: A switch is CLI-only: no MCP tool switches a session (PROVISIONAL on fork F5).
@task_fact n=8 slug=brand-identity
fact: A switch keeps the session's slot: the name and the wrapper stay, only the child is replaced, viola writes nothing to the terminal at the respawn and nothing to the user's own settings; a revive afterwards replays the switched pair (PROVISIONAL on fork F1).
@task_item n=1 part=untouched
The colour palette and its accent, the amber and dim tokens and the colour decision order: the switched line and the switch refusals carry no colour and no new colour token.
@task_item n=2 part=untouched
Typography, spacing, depth, border radius and the rendered-page and web-surface absences: no page and no web surface is added.
@task_item n=3 part=untouched
The viola list board and table, its six columns, row order, NAME cut and skip counts: no column or status word is added.
@task_item n=4 part=untouched
viola link and unlink output, viola wait output, viola last output and viola focus with its placement words and exit-1 codes: none of those verbs changed.
@task_item n=5 part=untouched
viola up outcome words and output and viola adopt refusals: their words and lines stand as they were.
@task_item n=6 part=untouched
The automatic-verify line and the readback box signature, the send readback slash-command rows and the self-validation tests: no readback form changed.
@task_item n=7 part=untouched
Iconography, glyphs and separators: no glyph or separator added.
