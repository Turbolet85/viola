@task_prose part=chunk
Builds a new flat CLI verb, viola focus <name>, in the root bin: resolve the home, validate the name, read the instance's state under the strict-modes checks, find the session's window, and bring it forward on its workspace through Hyprland.
The window comes from one of two identity sources: the window record the wrapper keeps for a session viola open started with --origin, or, for a hand-started viola run that records no window, the wrapper's process ancestry matched against the compositor's client list.
Every no-window outcome is a named refusal, and outside Hyprland the verb refuses as unsupported.
Proved by tests on the fake compositor's recorded Hyprland replies and the fake agent; the help, exit-cause and verb tables gain the new verb; CLI-only, with no MCP focus tool and no new time bound.
@task_file n=1 path=src/cmd/focus.rs
what: the new viola focus CLI verb of the root bin — home resolution, name validation, strict-modes state read, window identity by record or ancestry, the Hyprland focus and workspace dispatches, and its named refusals
@task_file n=2 path=src/cmd/mod.rs mark=edits
what: the root bin's CLI verb set and its dispatch — edits: the verb table entry for focus
@task_item n=1 part=needs
The new command name focus joins the register of CLI verb names as a flat verb beside open, list, send and the rest, and takes its place in the viola --help verb groups.
@task_item n=2 part=needs
The session name argument is validated before any read, and a name that is no wrapped session is refused the way the other instance verbs refuse an unwrapped name: unwrapped sessions are never a target.
@task_item n=3 part=needs
The instance's state is read under the strict-modes checks; a strict-modes failure refuses with the fixed instance-state unable/hint pair and its exit 21 cause, as the other frame verbs do.
@task_item n=4 part=needs
Window identity, recorded arm: the window record a session started with --origin carries (address, workspace, placement, origin) is read back, and its address is re-parsed before use rather than trusted; the origin and placement words it carries are the existing fixed vocabularies.
@task_item n=5 part=needs
Window identity, ancestry arm: for a hand-started viola run the wrapper's process ancestry is matched against the compositor's client list, bounded by the existing in-memory ancestry walk's limit of parent pids.
@task_item n=6 part=needs
The focus and workspace moves go only through the Hyprland seam's closed request set: no other call to the compositor, no dispatch outside that set, and no key sent.
@task_item n=7 part=needs
A named refusal for every no-window outcome, and an unsupported refusal outside Hyprland: each on the fixed refusal line form on stderr with its exit code from the CLI exit-code table and its cause word, the words and exits fitted beside the existing tables.
@task_item n=8 part=needs
Each refusal cause carries at most one hint, in human mode only, that never quotes text, never names viola release, and shows no path, pid, window address or other internal; faults and unknown causes carry no hint.
@task_item n=9 part=needs
Success is announced only after the compositor confirms the window was brought forward, and the confirmation line carries the session it acted on; nothing optimistic before the reply.
@task_item n=10 part=needs
Output discipline: data on stdout, messages on stderr; ASCII-only words, no glyph or emoji prefixes, control characters in any echoed session name escaped, no spinner, no interactive prompt or pager, no stack traces.
@task_item n=11 part=needs
Colour follows the CLI colour decision order and is limited to amber and dim, decided by std IsTerminal TTY detection, NO_COLOR and TERM=dumb, with no background colour set.
@task_item n=12 part=needs
If the verb takes --json, its result is one stdout document, its failure documents clean with the cause code and no hint line, and the catch site's --json scope and internal-error document apply to it.
@task_item n=13 part=needs
CLI-only boundary: the verb is for the human and the LLM driver as CLI callers; no MCP focus tool is added and the hidden mcp verb gains nothing.
@task_item n=14 part=needs
Tests reach the verb through the fake compositor's recorded Hyprland 0.56.2 replies and the fake agent, covering a viola-opened session, a hand-started session, no window found and a non-Hyprland host; a clause the fakes cannot carry is planned gated on the operator's live reading.
@task_standing n=1
convention: each new CLI verb is a flat verb named in the command register and listed in the viola --help verb groups
home: surface-cli · surface
@task_standing n=2
convention: each new refusal cause gets its fixed unable line, its exit code and --json cause word, and at most one human-mode hint per cause that shows no internals
home: surface-cli · surface
@task_standing n=3
convention: each new line of human output is ASCII-only, escapes control characters, puts data on stdout and messages on stderr, and carries colour only as a second cue
home: anti-patterns-never-do-these
@task_item n=1 part=not_needed
The strip-board look of the brand identity — flight-progress strip fields, the readback box, send readback rows, the handoff transfer marker and the dialog-pending cocked strip: focus draws no board and sends no prompt.
@task_item n=2 part=not_needed
The viola list board and table rows — columns, NAME cut and width, WHEEL width, row order, skip counts, empty board, colour spans: focus prints no table.
@task_item n=3 part=not_needed
viola verify output and refusals, the automatic-verify line and the CLI verification state: focus runs no verification.
@task_item n=4 part=not_needed
viola run and revive start refusals and revive --json refusal codes and --list output: focus starts and revives nothing.
@task_item n=5 part=not_needed
viola send, wait, last, answer, link, unlink, pause and release output rows, and the refusal hints for not-delivered prompts: focus sends no text and transfers no control.
@task_item n=6 part=not_needed
Wrapper fault --json detail codes and peer reply screening of wrapper replies: focus reads the recorded state and the compositor, not a wrapper request reply.
@task_item n=7 part=not_needed
The web-spa surface, the rendered-page absence and the self-validation token tests: viola renders no page.
