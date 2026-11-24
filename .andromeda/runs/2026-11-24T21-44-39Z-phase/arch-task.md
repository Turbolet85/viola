@task_prose part=chunk
Changes the folder `viola adopt` spawns its resumed claude child in, today the caller's current directory, so the CLI finds the adopted session where it was made.
Adds what `viola adopt` says and records when the resumed child ends at once after the start: a closed word, an exit, a hint and an output line beside the existing refusal tables, an event and a log line, and whether the instance directory and the name are released after it.
Changes the R5 live-reading rig script with the fix if the fix moves where the command must be typed, its rehearse form reaching the changed path with no claude.
Re-measures at P3, with no live start: what the wrapper recorded of the two red children, where revive and up take their resume folder (the snapshot's recorded cwd) and that adopt has no such record, and what the capability ledger rows hold about a resume from another folder.
@task_file n=1 path=src/cmd/adopt.rs mark=edits
what: the `viola adopt` verb's start path in the root bin — the resumed claude child's spawn and its refusals — edits: the spawn folder of the adopt start, the handling of a child that exits at once after the start
@task_file n=2 path=scripts/r5-adopt.sh mark=edits
what: the R5 live-reading rig script that types `viola adopt` in its own scratch folder, with a live arm and a `--rehearse` arm that runs no claude — edits: the folder the command is typed in, the rehearse arm
@task_item n=1 part=needs
The adopt child's working folder is set explicitly by viola at spawn, never left to the inheriting caller's current directory; where that folder comes from is the chunk's own design choice.
@task_item n=2 part=needs
The resume folder that revive and up hand their child comes from the instance snapshot's recorded cwd; adopt has no snapshot record of the adopted session's folder, so any folder adopt uses must be sourced and recorded by the adopt start itself.
@task_item n=3 part=needs
A resume folder that is missing or unusable at start: revive refuses it with its cwd-missing refusal; adopt's analogue, if any, is a refusal before the child spawns, following the start order with the child spawned last.
@task_item n=4 part=needs
The resumed child's command line: the logged session id passed through the resume flag and the recorded launch pair, unchanged by the folder fix.
@task_item n=5 part=needs
Detecting a child that ends at once: the child's exit is read on the process handle and recorded in the child exit record with its exit source; the early exit is told apart by time and exit right after the start.
@task_item n=6 part=needs
The early-exit closed word joins the adopt --json refusal codes and the closed set of refusal details, beside the existing adopt refusal table.
@task_item n=7 part=needs
The early-exit exit status is taken from the CLI exit code table without reusing a retired code; once the child has held the terminal no JSON document is printed, and the start-time --json document shape applies only before that.
@task_item n=8 part=needs
The early-exit human hint and output line follow the human stderr refusal line and code-word format; upstream text and error chains stay out of the human line and reach only the detail file.
@task_item n=9 part=needs
What is recorded of the early exit: an event line appended to the instance's events log under the event-line shape and append discipline, and a codes-only line in the adopt role's process log.
@task_item n=10 part=needs
After an early exit, whether the instance directory and the name are released: a later start must not be refused by a held name or a stale instance left behind, and the snapshot writes stay atomic.
@task_item n=11 part=needs
Adopt keeps refusing a session a live claude holds, and keeps its silent role at the internal-error catch site, with the new early-exit path added after those checks.
@task_item n=12 part=needs
No live claude in CI and real claude probes only locally: the early exit is proved against the fake agent through its test seams, and the rig's rehearse arm runs no claude.
@task_item n=13 part=needs
The capability ledger is the gate for CLI behaviour: a resume from another folder has no measured row unless one exists, read as UNMEASURED, and nothing in the design may rest on the unmeasured hypothesis that the CLI resumes only from the folder a session was made in.
@task_item n=14 part=needs
The child's environment at adopt spawn: inherited CLAUDE names stripped and the VIOLA_ names set as for any wrapped child, unchanged by a new spawn folder.
@task_item n=15 part=needs
Session-home resolution and the strict-modes read rule bound what adopt may read about the adopted session; nothing of the operator's real Claude config is read by the chunk or its tests.
@task_standing n=1
convention: each new test runs against the fake agent in its own isolated viola home, never a real claude
home: established-decisions
@task_standing n=2
convention: each new refusal word or event value is kebab-case on the wire, from a closed set, with only the first refusal returned
home: conventions
@task_standing n=3
convention: each new event kind is added only in viola-core and stays agent-neutral
home: conventions
@task_standing n=4
convention: each new log line is codes-only in its role's process log, content-bearing detail kept to the instance detail file
home: occupied-resources
@task_standing n=5
convention: each new test seam is a fake-agent seam variable, never a configuration channel, and never read by the product as configuration
home: occupied-resources
@task_standing n=6
convention: end-to-end runs of new tests leave zero panic lines
home: project-directory-structure
@task_item n=1 part=not_needed
The wheel, send, paste and confirmation machinery: the chunk changes no send, no typed text and no wheel claim or release.
@task_item n=2 part=not_needed
Dialog holds, hook tiers and the hook-to-kind map: no hook output or dialog answer changes.
@task_item n=3 part=not_needed
The MCP server, its tools and stdin bounds: adopt has no MCP tool and the chunk adds none.
@task_item n=4 part=not_needed
Links, link recording and the link and unlink verbs: no link is made or changed.
@task_item n=5 part=not_needed
Switch, focus, open and the Hyprland window seam: model, effort and windows are untouched.
@task_item n=6 part=not_needed
Verify's probe runs, its step and summary lines, its refusal codes and the stamps writer: the chunk writes no stamp and runs no verify.
@task_item n=7 part=not_needed
The wrapper channel frames, JSON-RPC fault codes and frame-client server checks: no channel method changes.
@task_item n=8 part=not_needed
Plugin install, the marketplace tree and the plugin's MCP config: unchanged.
@task_item n=9 part=not_needed
Fuzz workspace, release tooling, dist and mutation testing schedules: outside this chunk's change.
@task_item n=10 part=not_needed
The usage-budget absence, graphical frontends, mobile views and other platform absences: nothing here touches them.
