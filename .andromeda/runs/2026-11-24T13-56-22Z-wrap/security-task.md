@task_prose part=session
The session added a new CLI verb, viola switch, and a new wrapper channel method, switch, that move a running session to a given model and effort.
On a switch the wrapper ends its own claude child (only the child its own PTY handle holds, no pid lookup), waits up to a new 10 s reap bound, then respawns claude --resume with the recorded session id and the new pair under a new pseudo-terminal behind the same host terminal.
The new pair is recorded in the wrapper's snapshot and as one launch line per switch in the event log so a revive replays it; nothing is written to the user's own Claude settings and no --settings flag is passed.
A switch is refused while the human types, mid-turn, with a dialog pending, on an unverified CLI, with no session or a session id of another shape, when the child does not end within the bound, or when the resume fails; a send is refused turn-running while a switch stands.
The switch is CLI-only (no MCP tool); forks F1, F3, F5 and F6 stand PROVISIONAL on the delegate overseer's word, and the behaviour is proven over the fake agent alone, no capability claimed.
@task_file n=1 path=src/cmd/switch.rs
what: new; the CLI frame verb viola switch: the client seam checks (strict-modes refusal, live endpoint, server verification before any frame), its --model and --effort launch pair flags, its refusals and exit codes
@task_file n=2 path=src/run/switch.rs
what: new; the wrapper's handler of the switch channel method: re-checks the model and effort halves, the refusal order, reads the wrapper's own snapshot, appends the launch line, ends its own child and resumes the session on the new pair
@task_file n=3 path=tests/cli_switch.rs
what: new; integration tests of viola switch, including that the user's settings stay byte- and mtime-equal
@task_file n=4 path=crates/viola-pty/src/pump.rs
what: edited; the PTY pump, the passthrough between the host terminal and the claude child's PTY: now respawns its own child within a reap bound, and its panic arm kills the current child
@task_file n=5 path=crates/viola-pty/src/lib.rs
what: edited; the PTY crate's public exports (the respawn handle and the reap bound)
@task_file n=6 path=crates/viola-channel/src/server.rs
what: edited; the channel server's closed list of method labels used for logging
@task_file n=7 path=src/cmd/run.rs
what: edited; the wrapper start (viola run): its channel method dispatch and the pump of its claude child
@task_file n=8 path=src/cmd/verify/typed/run.rs
what: edited; viola verify's typed run that spawns a child under the PTY pump (passes a respawn handle no one uses)
@task_file n=9 path=src/cmd/mod.rs
what: edited; the CLI verb table and its dispatch
@task_file n=10 path=src/run/mod.rs
what: edited; the wrapper's module list
@task_file n=11 path=src/run/send.rs
what: edited; the wrapper's send method, now refused turn-running while a switch stands
@task_file n=12 path=src/run/send/slot.rs
what: edited; the wrapper's send slot and in-flight state: the switch mark and a one-shot screen of the old child's late session-end
@task_file n=13 path=src/human.rs
what: edited; human-mode output module
@task_file n=14 path=src/human/verbs.rs
what: edited; human-mode verb output writers (the switched line: name, model, effort)
@task_file n=15 path=src/human/hints.rs
what: edited; CLI stderr hints per refusal cause (four new causes)
@task_file n=16 path=tests/cli_output_plain.rs
what: edited; tests of plain human output and the help verb list
@task_file n=17 path=tests/contract_error_bodies.rs
what: edited; the census of channel methods and CLI error documents
@task_file n=18 path=tests/cross_exit_causes.rs
what: edited; the cross-check table of refusal words and their exit codes
@task_file n=19 path=tests/support/home.rs
what: edited; test support for wrapper homes (adopt as a given user home)
@task_item n=1 part=touches
A new wrapper channel method, switch, sits beside the contract rows of wrapper channel methods (answer, last, link, pause, release, send, unlink, wait, hook.dialog, hook.event): params v, sender, model?, effort?; reply ok with model and effort, or a refusal; the served requests go from 9 to 10, hook.event still the one notification.
@task_item n=2 part=touches
The rule that a new listener or channel method needs a Security Decisions Log entry: switch is a new channel method, served only after the existing peer euid and server checks of the client connect, and is owed its entry (PROVISIONAL, F1).
@task_item n=3 part=touches
The decision that CLI frame verbs check the instance first (strict modes first): it does not name switch, which now runs the untrusted-home refusal, then the live endpoint, then the verified connect.
@task_item n=4 part=touches
The decision that launch pair flags are checked before anything is read: switch's --model and --effort now go through the same model and effort parsers, and the wrapper re-checks each half.
@task_item n=5 part=touches
Wrapper re-validates every input, closed channel types and the -32602 channel error rows: a switch with neither half, a half of another type, or a half failing its check is -32602 with data null.
@task_item n=6 part=touches
Self-reported from authenticates nothing and channel from is not an identity: the switch method reads no from at all.
@task_item n=7 part=touches
Capability-based all-or-nothing channel access and the same-user trust boundary: any same-user peer that passes the channel checks can now end and restart the session's claude child on another model and effort.
@task_item n=8 part=touches
Adopt acts on no process it did not start and the R5 adopt script ends only its own processes: a wrapper now ends its own live child on a switch, killing only the child its own PTY handle holds, with no pid lookup.
@task_item n=9 part=touches
Human typing takes the wheel, security refusals never block the human and the human-typing send refusal: switch is refused human-typing while the human types, and refused mid-turn and with a dialog pending (F3, PROVISIONAL).
@task_item n=10 part=touches
The unverified CLI gate: switch is refused unverified-cli, read from the start's recorded verification.
@task_item n=11 part=touches
viola writes nothing under the Claude home or in Claude's config: the switch keeps the user's settings byte- and mtime-equal and passes no --settings, measured over the fake agent only.
@task_item n=12 part=touches
Launch pair kept to the snapshot and the event log, launch pair handed on the child's argv, and the recorded launch pair authenticates nothing: a switch rewrites the snapshot's launch and child_pid, appends one launch line per switch (and the recorded window line again after it), and puts the new pair on the respawned child's argv.
@task_item n=13 part=touches
Session id on child argv in a closed shape and as one argv element: the respawn passes the recorded session id to claude --resume; an id of another shape is refused no-session.
@task_item n=14 part=touches
Snapshot reference values only after the strict-modes check, replayed snapshot inert, and only the instance's wrapper writes its snapshot: the switch's refusal rungs read the wrapper's own disk snapshot (pending dialog, session id, launch, window), and the wrapper itself rewrites it.
@task_item n=15 part=touches
Window record kept to the snapshot and the event log: a switch re-appends the recorded window line after the launch line.
@task_item n=16 part=touches
Rules over the claude child's spawn (parent identity variables and inherited CLAUDE credentials stripped, pinned folder first on PATH, run resolves its child program itself, channel handles not inheritable): the respawned child is a new spawn from the wrapper's plan, so each rule now also reaches a respawn.
@task_item n=17 part=touches
Rules over the PTY pump and the screen feed (screen feed isolated from passthrough, bounded queue, under a panic catch): the pump now swaps a new pseudo-terminal under the same host stdin and stdout copies, restarts its writer and output threads on the new child, and its panic arm kills the current child.
@task_item n=18 part=touches
Bounded time rows: a new compiled time bound, RESPAWN_REAP_BOUND at 10 s (F6, PROVISIONAL); a child not ended within it is not replaced and the switch is refused child-not-ended.
@task_item n=19 part=touches
CLI exit-code and instance-unreachable rows (exits 12, 21 and the instance-unreachable causes): viola switch exits 0, 10, 12, 13, 14, 20, 21, with the new refusal words dialog-pending, no-session, child-not-ended and resume-failed at exit 13.
@task_item n=20 part=touches
Refusals hold no paths, pids or ids, error messages interpolate nothing and every product error writer named: switch adds a new verb's refusal writer and hints for four new causes; no pair, model or session id appears in any log line.
@task_item n=21 part=touches
Human output escapes control characters: a new human line, the name then switched then the model and effort, is printed on stdout.
@task_item n=22 part=touches
Send rows (send text is one bracketed paste, turn-running refusal): a send is now refused turn-running from a switch's request until the new child's session-start.
@task_item n=23 part=touches
Test seams only under the fake-agent feature and compiled out of release: the pump gains a spawner seam for its respawn tests, and test support gains adopt as a given user home.
@task_item n=24 part=touches
The hook event contract (hook.event, session-end): after a respawn the old child's late session-end of the switched id is screened once while the new child is alive; it cannot tell the old child's late session-end from the new child's own (an insufficient fix, owner the real-CLI reading R6a).
@task_fact n=1 slug=authentication-authorization
fact: A switch ends only the wrapper's own claude child, the one its own PTY handle holds, with no pid lookup, then resumes the session in place (PROVISIONAL, fork F1, on the delegate overseer's word).
@task_fact n=2 slug=threat-model-summary
fact: The wrapper channel serves a switch method, params v, sender, model? and effort?, reply ok with model and effort or a refusal (human-typing, not-delivered with turn-running, dialog-pending, no-session, child-not-ended or resume-failed, unverified-cli); it reads no from (PROVISIONAL, F1).
@task_fact n=3 slug=input-validation
fact: The wrapper re-checks the switch method's model and effort halves; neither half, a half of another type or one failing its check is -32602 with data null.
@task_fact n=4 slug=authentication-authorization
fact: A switch is refused in a fixed order, the first rung winning and none appending a line or ending the child: human typing, a running turn, a pending dialog, an unverified CLI, then no session (PROVISIONAL, F3).
@task_fact n=5 slug=data-protection
fact: A switch writes nothing to the user's own Claude settings and passes no --settings; the new pair is kept to the wrapper's snapshot and one launch line per switch in the event log, which a revive replays.
@task_fact n=6 slug=threat-model-summary
fact: Switching a session's model and effort is CLI-only; no MCP tool switches a session (PROVISIONAL, F5).
@task_fact n=7 slug=new
fact: RESPAWN_REAP_BOUND is a compiled 10 s bound on the old child's exit at a switch; a child not ended within it is not replaced and the switch is refused child-not-ended (PROVISIONAL, F6).
@task_fact n=8 slug=new
fact: A send is refused turn-running while a switch stands, from the switch's request until the new child's session-start.
@task_item n=1 part=untouched
Dependency security: no dependency, crate, toolchain, pin, licence, advisory or CI workflow changed.
@task_item n=2 part=untouched
API security and the MCP server: no rmcp feature changed and no MCP tool was added; the MCP tool contracts and MCP stdin rows stand.
@task_item n=3 part=untouched
The viola hook binary's rows (strict modes first, fail open, silent exit zero, stdin caps): the hook itself was not changed.
@task_item n=4 part=untouched
Plugin install, the pinned binary and its re-hash, and the marketplace tree: none was touched.
@task_item n=5 part=untouched
Recorded fixtures, live rigs, probe residuals and fixture refusal codes: no fixture or rig changed.
@task_item n=6 part=untouched
viola list and the board, viola open, focus and the Hyprland recorder rows: none was changed.
@task_item n=7 part=untouched
config.json keys and the diagnostics line schema: no config key added and no new detail word emitted in any log line.
@task_item n=8 part=untouched
CI secret handling, the artifact canary scan and the pre-push hermetic gate: unchanged.
@task_item n=9 part=untouched
Link, unlink, pause, release, answer and dialog channel method rows: their params, replies and refusals did not change.
@task_item n=10 part=untouched
The absences (network, hosting, release stage, secrets, databases, accounts): the session adds no such surface.
