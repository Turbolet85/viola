@task_prose part=chunk
Adds a new human CLI verb `viola up` that re-opens the fleet after a host reboot (capability v2-12).
It reads the board's instances under the home, selects those recorded as live before the host went down, and orders them parents before children by the recorded lineage.
It skips every instance whose recorded pid and start time verify live, and re-opens each remaining one resumed in place through the existing revive start path, in the terminal window viola open would give it, in its recorded workspace.
It prints one line per instance with a closed outcome word, or under --json a document of codes and closed words only, and adds tests over the fake agent and the Hyprland fakes under a test-owned home.
It re-measures at HEAD the lineage and launch lines at every start, viola open's window record under --origin, and revive resuming in place, without copying earlier readings.
@task_file n=1 path=src/cmd/ mark=edits
what: the root binary's CLI verb dispatch, verb table, help rows and exit-cause tables — edits: a new `up` verb in the clap dispatch
@task_file n=2 path=src/human/ mark=edits
what: the human-mode output module that renders CLI lines for the operator — edits: the per-instance line of `viola up`
@task_item n=1 part=needs
A new CLI verb in the verb table, help rows, exit-cause tables and exit codes, and its --json machine contract: an error document and output of codes and closed words only, beside the existing CLI machine contract.
@task_item n=2 part=needs
The verb is CLI-only: a driver session re-opening the fleet is a boundary widening, held to its conservative arm, so no agent-facing tool is added for it.
@task_item n=3 part=needs
CLI input handling for the new verb: its flags and home selection (--home, VIOLA_DIR, the CLI home resolution order) checked before anything is read, and any instance name passed through the validated name type.
@task_item n=4 part=needs
The board read model: the home and state tree pass the strict-modes check before any read, the board scan refuses a readable home and refuses the whole board, readers are bounded, tolerate torn lines and never write.
@task_item n=5 part=needs
Snapshot values (recorded cwd, workspace, placement, lineage, launch pair, window and origin) are used only after the strict-modes check, only when they re-parse, and a replayed snapshot stays inert; the recorded cwd is used only to spawn.
@task_item n=6 part=needs
The parent-before-child order comes from the self-reported lineage record, which authenticates nothing; a missing, never-live or cyclic parent must not let a recorded value become a command or a target.
@task_item n=7 part=needs
Liveness skip: an instance is skipped only when its recorded pid and process start time verify live through the process-identity reading, with the heartbeat as the board's other liveness signal.
@task_item n=8 part=needs
Each re-open goes through revive's existing start arm with its preflight (the claude agents read, session-live), its strict-modes checks, socket-directory check before bind, start collision check (already-live, stale-heartbeat) and closed revive refusal codes, which up must carry as closed words, never as upstream text.
@task_item n=9 part=needs
The argv handed to each start: session id as one argv element of a closed shape, the launch pair on the claude child's argv, no shell, the claude program resolved from PATH, the pinned binary re-hashed before reuse.
@task_item n=10 part=needs
The child environment at each start: parent identity variables (no inherited VIOLA_* name) and inherited CLAUDE credentials stripped from the child.
@task_item n=11 part=needs
The terminal window per instance through viola open's path: the terminal spawned directly with its closed argument allowlist, the wrapper's window record under --origin, the recorded window origin authenticating nothing, and the wrapper's window step moving only its own terminal's window.
@task_item n=12 part=needs
The Hyprland seam: only its closed request set, hyprctl children bounded by a deadline, Hyprland reply text dropped at the parse; a recorded window address dies with the compositor and is never trusted after a reboot.
@task_item n=13 part=needs
Lineage, launch pair and window records stay kept to the snapshot and the event log at every start.
@task_item n=14 part=needs
The per-instance outcome line in human mode: C0/C1 controls escaped, no paths, pids, ids, addresses or workspaces in a refusal or error line, error messages that interpolate nothing and carry no upstream text.
@task_item n=15 part=needs
The obs event catalog gains a per-instance line for up, under the never-log floor: no address, pid or workspace in a process-log line, full error detail only in diagnostics.
@task_item n=16 part=needs
Nothing written or read under the operator's Claude home: viola writes nothing under the Claude home, and this phase reads nothing of the real ~/.claude* trees.
@task_item n=17 part=needs
No live act in this phase: no live start, no window opened, moved, focused or closed, no Hyprland dispatch, no key sent, no real session revived, no list against the real claude on PATH; every live reading of up is gated on the operator at the host.
@task_item n=18 part=needs
The new tests prove selection, order, skip, lines and the argv of each start over the fake agent and the Hyprland fakes, with the real claude CLI only local and never in CI.
@task_standing n=1
convention: every new test runs under its own test-owned home, kept out of git, and removes only the directories it created
home: secret-management · security-anti-patterns
@task_standing n=2
convention: every test seam a new test needs lives only under the fake-agent feature and is compiled out of release
home: security-anti-patterns · input-validation
@task_standing n=3
convention: every new log line, seam or environment read is named in the log catalog entry and holds to the never-log floor
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: every new error writer is named and uses fixed display messages that interpolate nothing
home: error-handling
@task_standing n=5
convention: CI holds no secrets and runs no real claude CLI; new CI tests run over the fakes only
home: threat-model-summary
@task_item n=1 part=not_needed
The MCP server's tools, tool inputs, stdin line cap and isError results: up adds no MCP tool and reaches no MCP surface.
@task_item n=2 part=not_needed
The wrapper channel's frame methods and their params (send, answer, wait, last, pause, release, link, unlink) and the paste and dialog text character sets: up sends no frame and types no text.
@task_item n=3 part=not_needed
The Claude Code hook contract: hook stdin, hook-decision details, hook capture arm and hook silent exit.
@task_item n=4 part=not_needed
viola verify, its stamps, probes, recorded fixtures and the fixture refusal codes, and the live rigs' arms: up neither verifies nor records fixtures.
@task_item n=5 part=not_needed
viola plugin install, the marketplace tree and plugin files rewritten at start.
@task_item n=6 part=not_needed
viola focus's window raising and its ancestor walk: up opens windows for re-opened sessions and focuses none.
@task_item n=7 part=not_needed
Link and unlink names, link event lines and the link verbs' exit codes.
@task_item n=8 part=not_needed
config.json keys (claude_env_keep, diagnostics_level, dialog_hold_s) and the stdin classifier's non-editing list.
@task_item n=9 part=not_needed
Transcript reads, stop-hook crons and background-task counts, and API-error classes of the Claude adapter.
