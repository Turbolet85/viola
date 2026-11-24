@task_prose part=chunk
Builds a new `viola focus <name>` CLI verb in the root bin: resolve the home, validate the name as a ViolaName, read the instance's state under the strict-modes checks.
It finds the session's window by recorded identity (the window record a session started with --origin keeps) or by the wrapper's process ancestry matched against the compositor's client list.
It shows the window's workspace and brings the window forward on Hyprland through the seam's closed request set only.
With no window found it refuses with a named cause; outside Hyprland it refuses as unsupported.
It adds tests on the fake compositor and fake agent, and a row in the help, exit-cause and verb tables.
@task_file n=1 path=src/cmd mark=edits
what: the root bin's CLI verb commands — edits: a new viola focus verb, its entry in the help, exit-cause and verb tables
@task_item n=1 part=needs
A new flat verb plus a target: viola focus takes one target, and that target is always a ViolaName, validated and never case-transformed.
@task_item n=2 part=needs
The CLI verbs table and the viola --help output gain viola focus: its usage line, its verb group row and its per-verb usage under viola focus --help.
@task_item n=3 part=needs
The --home global flag resolves the home the verb reads.
@task_item n=4 part=needs
The instance state is read under the strict-modes checks; a strict-modes failure refuses as the other state-reading verbs do, in human form and in the --json detail.
@task_item n=5 part=needs
A name with no instance directory, a dead instance or an unreachable wrapper refuses with the shared instance-unreachable refusal, its exit code, its --json detail and its hint, where the verb reaches them.
@task_item n=6 part=needs
Every no-window outcome is a named refusal, and outside Hyprland the verb refuses as unsupported; the refusal words, exit codes and hints are new entries beside the existing tables.
@task_item n=7 part=needs
Each refusal line follows the CLI refusal line form and the shared refusal word column; every refusal cause has a hint, the hint is the last stderr line, names the next verb, and never quotes text or names a path.
@task_item n=8 part=needs
The CLI exit codes and their --json form bind the new refusals; --json mode, where the verb takes it, prints one document.
@task_item n=9 part=needs
Results go on stdout and everything else on stderr; human output is a contract; no prompt, pager, spinner, banner, progress glyph or terminator word; plain output modes and TTY detection through std IsTerminal hold.
@task_item n=10 part=needs
An internal error in the verb surfaces through the CLI catch site as the CLI internal error, in human and --json output.
@task_item n=11 part=needs
Hyprland dispatches go only through viola-hyprland's closed request set: a workspace-focus dispatch and a focus-window dispatch, recorded for Hyprland 0.56.2 and replayed by the fake compositor.
@task_item n=12 part=needs
Window identity arm one: the window record the wrapper keeps in the snapshot for a session started with --origin, its address re-parsed through the seam's Address validator.
@task_item n=13 part=needs
Window identity arm two: for a hand-started viola run, which records no window, the wrapper's process ancestry (at most 8 parent pids, walked in memory) matched against the compositor's client list.
@task_item n=14 part=needs
The CLI holds every action: focus is CLI-only, no MCP focus tool is added (the conservative arm, provisional on the operator's word), and viola mcp exposes nothing new.
@task_item n=15 part=needs
No new time bound is grown for the verb while the time-bound list question stands open.
@task_item n=16 part=needs
Tests run on the fake compositor and the fake agent only; a clause the fakes cannot carry (a live focus reading) is planned gated on the operator's grant, with no live start, window, dispatch or key in this phase.
@task_standing n=1
convention: every new refusal cause a verb adds carries its own hint, exit code and --json detail code, each pinned by a test
home: surface-cli
@task_standing n=2
convention: every new verb's human output lines are a contract, each pinned by a test of its exact text
home: surface-cli
@task_item n=1 part=not_needed
The web surface absence: viola renders no page, and this verb adds none.
@task_item n=2 part=not_needed
The viola list board and BAY line rows (cell order, column widths, tree, stale rows, links block, name truncation, colour depth, WHEEL and DIALOG columns): focus renders no board.
@task_item n=3 part=not_needed
The viola send rows, the readback mirror and the readback vocabulary: focus sends no prompt and reads nothing back.
@task_item n=4 part=not_needed
The viola wait, viola last and viola answer output rows: focus waits on no turn, reads no message and answers no dialog.
@task_item n=5 part=not_needed
The viola run and viola revive start, usage, refusal and --json rows: focus starts and revives nothing.
@task_item n=6 part=not_needed
The viola verify, viola plugin install, viola link and unlink, and wheel verb (pause, release) output rows: none of those verbs changes.
@task_item n=7 part=not_needed
The viola open rows, its --parent default among them: the gated real window of viola open is not this chunk's.
@task_item n=8 part=not_needed
The existing MCP tool rows and MCP tool results (answer, last, list, send, wait): no tool is added or changed.
@task_item n=9 part=not_needed
The deep links and URL coupling absence: focus carries no link.
