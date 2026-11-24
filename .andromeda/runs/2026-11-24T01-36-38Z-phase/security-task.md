@task_prose part=chunk
Builds a new `viola focus <name>` CLI verb in the root binary: resolve the home, validate the name, read the instance's state under the strict-modes checks, find the session's window and bring it forward on its workspace on Hyprland.
Window identity comes from two sources: the window record the wrapper keeps in the snapshot for a session started with an origin (its address re-parsed through the Hyprland seam's address validator), or, for a hand-started session, the wrapper's process ancestry matched against the compositor's client list.
Every no-window outcome is a named refusal, and outside Hyprland the verb refuses as unsupported; the dispatches go only through the Hyprland seam's closed request set.
Re-measures at HEAD that the seam holds a focus-window and a workspace-focus dispatch recorded for Hyprland 0.56.2 and that the fake compositor replays them; adds tests on the fake compositor and the fake agent, and the verb's entries in the help, exit-cause and verb tables.
CLI only; no live start, no window, no dispatch and no key against the real desktop in this phase; a live focus reading is planned gated on the operator.
@task_file n=1 path=src/cmd/focus.rs
what: the new viola focus CLI verb — its name check, home resolution, strict-modes reads of the instance state, window identity lookup, Hyprland dispatch and named refusals
@task_file n=2 path=src/cmd/mod.rs mark=edits
what: the CLI verb set and its dispatch in the root binary — edits: the command list's focus entry and its dispatch arm
@task_item n=1 part=needs
The CLI input trust boundary for a new verb: the session name is checked through the validated viola name type before anything is read, and any instance path is built only from that validated name; unwrapped or agent-list names are never a target.
@task_item n=2 part=needs
Home resolution for a new verb follows the CLI state-root selection order (the --home flag, then VIOLA_DIR), and the home tree and the instance's snapshot pass the strict-modes check before any read or narrowing; a strict-modes refusal for the new verb leaves no trace and follows the strict-modes refusal message forms.
@task_item n=3 part=needs
The snapshot's window record (address, workspace, placement, origin) is a reference value read only after the strict-modes check and re-parsed before use; a recorded window or its origin authenticates nothing, and the record stays kept to the snapshot and the event log — focus copies it nowhere else.
@task_item n=4 part=needs
State readers never write: focus reads the snapshot written only by the instance's wrapper and writes no state file, stamp, trust record or log line under the home of its own beyond what the logging rules admit.
@task_item n=5 part=needs
The compositor is reached only through the Hyprland seam's closed request set: the focus-window and workspace-focus dispatches recorded for Hyprland 0.56.2, the hyprctl child spawned directly and bounded by a deadline, its reply text dropped at the parse and only closed values kept.
@task_item n=6 part=needs
Which windows viola may move: today the wrapper's window step and the Hyprland recorder move only their own terminal's window; focus raises and shows the workspace of the one window it resolves for the named session and never moves, resizes, closes or types into a window.
@task_item n=7 part=needs
The ancestry arm: the wrapper's process ancestry walked through the existing in-memory walk (at most 8 parent pids, process identity through sysinfo) and matched against the compositor's client list; a self-reported parent, lineage or launch pair authenticates nothing, and no pid leaves memory.
@task_item n=8 part=needs
Compositor client-list text (titles, classes, workspace names) is upstream content, never a command, read through a bounded reader and screened to closed values; a new external reader is bounded or named in the bounded-readers census and capped at the shared frame limit.
@task_item n=9 part=needs
Detecting Hyprland (the unsupported arm) reads the compositor's environment; a new environment read or seam needs its log entry, environment variables are only compared and are never configuration, and no flag, env var or config switches the checks off.
@task_item n=10 part=needs
The new named refusals (no window found, a recorded window gone, unsupported outside Hyprland) and their exits join the CLI exit-code and exit-cause tables; refusal and error text holds no paths, pids, ids or window addresses, interpolates nothing, carries no upstream text and maps parser errors to fixed messages.
@task_item n=11 part=needs
Output of the new verb: human-mode output escapes C0/C1 controls, a --json error document uses the closed CLI error codes, full error detail goes only to diagnostics, and stderr honours the never-log floor.
@task_item n=12 part=needs
Every new product error writer of the verb is named, under the error-enum convention and its named exceptions; an internal error is held at the unit layer and the verb's catch-site internal error output follows the --json form.
@task_item n=13 part=needs
The verb sends no frame to the instance and connects to no channel; if a later plan has it reach the wrapper, it falls under the client's check order (strict modes, socket directory, server verification before any frame).
@task_item n=14 part=needs
Tests run on the fake compositor's recorded replies for 0.56.2 and the fake agent: recorded Hyprland fixtures keep strings by the recorder's closed allowlist, are scrubbed or refused whole and reviewed before commit, and no test touches the real desktop, the operator's real Claude home or PATH's claude.
@task_item n=15 part=needs
The phase boundary: no live start, no window opened, moved, focused or closed, no dispatch and no key against the operator's session; a clause the fakes cannot carry is planned gated on the operator's live reading at the host.
@task_item n=16 part=needs
CLI only: the MCP tool list is not grown with a focus tool; a driver session moving the operator's focus is a boundary widening left provisional on the operator's word.
@task_standing n=1
convention: each new test uses its own home, never the operator's, and removes only the directories it created
home: secret-management · security-anti-patterns
@task_standing n=2
convention: each new recorded fixture is scrubbed or refused whole, reviewed before commit and kept under the fixture hygiene walk
home: data-protection
@task_standing n=3
convention: each new test seam lives only under the fake-agent test feature and is compiled out of the release build
home: security-anti-patterns
@task_standing n=4
convention: each new seam or environment read gets its log entry
home: security-anti-patterns-never-do-these
@task_standing n=5
convention: each new error writer is named in the error writer census and its display message is fixed, interpolating nothing
home: error-handling
@task_standing n=6
convention: each new reader of external bytes is bounded or named in the bounded-readers census
home: input-validation
@task_item n=1 part=not_needed
The wrapper's channel methods, channel frames and their params, and the paste, send and answer text character sets: focus types no text and sends no method to the wrapper.
@task_item n=2 part=not_needed
Hook contract rows (hook stdin, hook decision detail, hook fail-open and silent exit): the verb is no hook.
@task_item n=3 part=not_needed
MCP server rows (stdin line cap, tool inputs, isError structured content, frame tool results): the chunk adds no MCP tool.
@task_item n=4 part=not_needed
viola open's closed argument allowlist, its direct terminal spawn and the parent-identity strip for the claude child: focus spawns no terminal and no claude child.
@task_item n=5 part=not_needed
revive, verify and plugin install rows (revive --json codes, verify refusal codes, probe residuals, stamps, trust record, marketplace): none of those verbs changes.
@task_item n=6 part=not_needed
Pinned binary and plugin folder rows (placement, re-hash, no-clobber, code-bearing files rewritten): focus places and runs no pinned copy.
@task_item n=7 part=not_needed
Live rig arms and real-CLI verify probe rows: the chunk adds no live rig arm and no real claude run.
@task_item n=8 part=not_needed
Child credential and environment strip rows (CLAUDE credentials stripped, env kept and stripped lines, pre-push empty-environment launch): no claude child is launched.
@task_item n=9 part=not_needed
Event log rotation, link lines and link restore rows: focus writes no event line and changes no link.
@task_item n=10 part=not_needed
Dialog, stdin classifier and human-typing wheel rows: focus answers no dialog and keys nothing into a pty.
@task_item n=11 part=not_needed
Release, signing, attestation and self-update rows: the chunk adds no release stage.
