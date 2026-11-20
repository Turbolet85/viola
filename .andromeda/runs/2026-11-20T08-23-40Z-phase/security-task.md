@task_prose part=chunk
Changes the pre-send gate of the wrapper's send (the input-box readiness predicate viola run reads before it types a driver's text) so a compiled modal literal quoted inside the session's own transcript is no longer read as a modal standing.
A driver's send into an idle session whose screen holds such a quoting row is delivered, not refused not-delivered input-not-ready.
A real modal standing, for every compiled modal literal on its recorded screens, still refuses the send exactly as today; tests carry both directions, with the remove-the-guard control run on the refusal side too.
The discriminator is chosen from the screen model and the recorded screens with no live start, its conservative side stated; whether status, list and the dialog relay read the same predicate is settled at P3.
@task_item n=1 part=needs
The Claude adapter's input-box readiness predicate, read from compiled literals only, and its pre-send gate in the wrapper's send: the rule that it classifies the screen against compiled classification sources only must hold for the new discriminator too.
@task_item n=2 part=needs
The refusal side stays the safe side: the driver must never type into a standing dialog, so when the discriminator cannot tell a quoted literal from a standing modal it refuses (the never-type-into-a-dialog rule and dialog answers that need a verify stamp).
@task_item n=3 part=needs
Transcript and chat text in the driven session is upstream content, never a command or a state signal: a quoted literal a message carries must not be able to move the gate either way, and what a message can still do to defeat the discriminator is stated (driver prompt injection accepted, upstream text never a command).
@task_item n=4 part=needs
The send refusal surface for input-not-ready: the channel result.refusal not-delivered word, the CLI exit code and the MCP send result keep their closed words; the chunk changes when the refusal fires, not its shape.
@task_item n=5 part=needs
The same predicate decides verify's Run B framing pastes: verify reads the screen only against compiled literals, by signatures only, and never writes a byte into a dialog; the change must keep those reads within that rule.
@task_item n=6 part=needs
The screen model the discriminator reads: the vt100 feed stays inside its panic catch, isolated from passthrough and on its bounded queue; any new reading of rows or cells works on the parsed screen, not raw output.
@task_item n=7 part=needs
Whether status, list and the dialog relay read the same predicate: moved together or kept deliberately apart, decided at P3; either way the readers stay read-only over the screen model.
@task_item n=8 part=needs
Test inputs are the fake agent's screens and the recorded fixture sets; a screen no set holds is added as a synthetic screen beside them, named synthetic, got without a live start, and owes fixture hygiene: scrubbed or refused whole, signature rows only, reviewed before commit, under the hygiene walk.
@task_item n=9 part=needs
No live start, no read of the operator's real Claude config and nothing written under the Claude home; the real claude CLI is not run and CI holds no real CLI.
@task_item n=10 part=needs
No relied ledger row or its verify probe changes: verify's stamps, probes and their signatures stay as they are (closed at P3 against the ledger code).
@task_item n=11 part=needs
The new gate code adds no panic path past the zero-panics gate and no new environment read, config key or flag that could switch the gate off (controls not switchable).
@task_item n=12 part=needs
Carry: the intermittent rig rehearsal red reading the box empty after x and del stays open; at its next red the printed rig summary and the kept after-del frame are read and the cause named before any mend, with no retry rule and no loosened entry.
@task_item n=13 part=needs
Watch: the supervise test where stop presses ctrl-c again when the first is lost is read from its log (the fake agent's exit code and what the supervisor sent it) before any mend; no bound or assertion moves to get past it.
@task_standing n=1
convention: every new test runs in its own viola home, never the user's, and test homes stay out of git
home: secret-management
@task_standing n=2
convention: any new test seam or fake-agent screen hook is compiled only under the fake-agent test feature and out of release
home: security-anti-patterns
@task_standing n=3
convention: every new recorded or synthetic screen fixture is scrubbed, keeps signature rows only and is reviewed before commit
home: data-protection
@task_item n=1 part=not_needed
Channel peer and socket controls: peer euid checks, socket directory and file modes, server verification before a frame; the chunk changes no connection or bind.
@task_item n=2 part=not_needed
Home and state-tree strict-modes checks and their refusal contracts; the chunk reads no new state file.
@task_item n=3 part=not_needed
Revive, the instance snapshot, recorded cwd and session id argv rules; nothing of revive is touched.
@task_item n=4 part=not_needed
Plugin install, the marketplace tree, the pinned binary and its re-hash; no install or placement changes.
@task_item n=5 part=not_needed
MCP server transport, its tool list as affordance and the rmcp feature set; no MCP tool or transport changes.
@task_item n=6 part=not_needed
Link and unlink methods and their event-log lines, and event-log rotation; the chunk touches neither.
@task_item n=7 part=not_needed
Hook stdin parsing, hook payload caps and the hook's silent exit; the hook path is not part of the pre-send gate.
@task_item n=8 part=not_needed
Live rig arms (context, MCP, wake, SF, config-top) and live plugin-install rounds, and the live rig's credentials copy; no live round runs in this chunk.
@task_item n=9 part=not_needed
Release, signing and deploy decisions, workflow action pinning, cache and advisory schedules; no workflow or release change.
@task_item n=10 part=not_needed
Network, hosting, accounts, database, webhook and secret-manager absences; the chunk adds none of these.
