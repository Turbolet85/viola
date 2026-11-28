@task_prose part=chunk
viola console gains actions over the fleet view: a selection of one board row and the verbs reached from it by keyboard alone.
Each action acts through the CLI verb it names (open, focus, send, answer, last, pause, release, revive, and the switch of model and effort), with that verb's own outcome and refusal and no second code path.
A send shows its readback as the frame verb's own reply; a refused action shows the verb's unable line on the console's own surface; every mode the actions add is left by keyboard alone.
The scaffold's one-class-per-read key decoder is replaced, where the actions need it, by a decoder of key sequences; the view's content changes only as far as a selection and an action's result need.
No live reading is made: a capability clause that needs one is claimed gated, and the gated fleet-console-view chunk stays gated.
@task_item n=1 part=needs
The console key reader becomes a decoder of key sequences: it must stay bounded and closed, every byte sequence mapping to a closed key class, with interrupt and quit still reached and anything unknown read as unbound, never acted on.
@task_item n=2 part=needs
No key trap: every mode the actions add (a selection, a text entry, any confirm) is left by keyboard alone, and the console's terminal is still restored to cooked mode after the quit, proved on the console's own terminal.
@task_item n=3 part=needs
An action reaches its frame verb through the same client seam the CLI verb uses, so the client's check order binds it: the instance and home strict-modes check, the socket-directory check, the client's own refusals, and the server verification with the peer euid check before any frame leaves.
@task_item n=4 part=needs
The selection names its target by the board's item, never the drawn row text; the target crosses to the verb only through the validated viola name type, and an unwrapped agent name or a claude agents row name is never a target.
@task_item n=5 part=needs
The send's text entry hands its text to the send verb under the paste text rules: the paste character set, control characters refused whole and never stripped, empty text refused, line endings normalised, the text typed as one bracketed paste, with the wrapper re-checking it authoritatively.
@task_item n=6 part=needs
The send readback is the send frame verb's reply (ok or a refusal code word), never the view's STATUS cell; the client reads the server's answer as closed values and refusal code words only.
@task_item n=7 part=needs
A refusal shown on the console is the verb's unable line: fixed message words that interpolate nothing and hold no paths, pids or ids, no identity values, and no upstream text; any text drawn on the console surface is escaped of C0/C1 controls.
@task_item n=8 part=needs
Text an action brings back to the console (a last reply, an answer's question, a readback) is upstream content, never a command; the console acts on none of it.
@task_item n=9 part=needs
The answer action keeps the closed answer behaviour, the dialog free-text character set, and the rule that a dialog answer needs a verify stamp.
@task_item n=10 part=needs
The release and pause actions take their from as the CLI verbs do (from VIOLA_NAME); a driver-originated release stays refused, and the console as the human's own surface reaches release only as the human verb.
@task_item n=11 part=needs
The switch action (v2-31's console leg) reaches the switch only through the CLI verb: the switch refusal order, the switch method's place behind the IPC controls, and the switch ending only the wrapper's own child all bind it.
@task_item n=12 part=needs
The open and focus actions keep their CLI rules: open's closed argument allowlist and direct spawn of its terminal, focus raising only a window of the session's ancestry with its ancestor walk held in memory.
@task_item n=13 part=needs
The revive action keeps revive's rules: strict-modes check before use, the recorded cwd used only to spawn, trailing words as argv, and the replayed snapshot inert.
@task_item n=14 part=needs
If the console runs a verb as a child process rather than in-process, the child is spawned directly with no shell, its program resolved as the CLI resolves it, and its output read bounded and tolerant, failing to unknown.
@task_item n=15 part=needs
A security refusal of an action never blocks the human: the console stays usable after any refusal, and no config, env var or flag the console reads switches a control off.
@task_item n=16 part=needs
The console's start keeps its terminal and home check first and its strict-modes refusal line; the actions add no read of trusted state before that check.
@task_item n=17 part=needs
The view moving to a selected row past the body (the carried scroll gap) changes the view's drawing only; the board's reading, its home read rule and the agents thread's STATUS (at most every 5 s, unknown after a failed read) are untouched, and the console's agents child may still outlive a quit.
@task_item n=18 part=needs
Every error writer the actions add is named, and their full error detail goes only to diagnostics, honouring the never-log floor on stderr.
@task_item n=19 part=needs
No live start, no real claude, no window and no read of the real Claude home or viola home in this chunk's tests or first pass; the real claude CLI stays local-only and nothing is written under the Claude home.
@task_standing n=1
convention: every new console test runs under its own test home and the test-scoped PATH, never reading host sessions
home: secret-management
@task_standing n=2
convention: every test seam a new test needs sits under the fake-agent feature only and is compiled out of release
home: security-anti-patterns
@task_standing n=3
convention: a new seam or environment read added for the tests or the console needs its log entry
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: test-side removal deletes only the directories the test itself created
home: security-anti-patterns
@task_standing n=5
convention: CI runs the new tests with no secrets and no real claude
home: threat-model-summary
@task_item n=1 part=not_needed
The MCP server's tools, tool inputs, stdin line cap and isError results: the console acts through the CLI verbs, not the MCP surface.
@task_item n=2 part=not_needed
The hook contract (hook stdin, hook-decision details, hook capture and hook frames): no action runs or changes viola hook.
@task_item n=3 part=not_needed
Verify's probes, stamps writing, fixture hygiene and refusal codes, and the live rig arms: the chunk makes no live reading and runs no verify.
@task_item n=4 part=not_needed
The wrapper's server side (bind, socket chmod after bind, exclusive endpoint, server-side peer euid drop, snapshot writing): the console is a client and the wrapper is unchanged.
@task_item n=5 part=not_needed
Pinned binary placement and re-hash, plugin install and the marketplace tree: no action places, installs or rewrites them.
@task_item n=6 part=not_needed
Event log rotation, link lines and the link and unlink methods: link and unlink are not among the console's verbs.
@task_item n=7 part=not_needed
Release, signing and deploy decisions and the release workflow's attestations and cache rules: the chunk ships no release.
@task_item n=8 part=not_needed
Fuzz scope, the fuzz workspace and mutation testing at the epoch audit: these run at the audit, not in this chunk.
