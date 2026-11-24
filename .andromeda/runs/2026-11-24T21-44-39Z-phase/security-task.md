@task_prose part=chunk
Changes where `viola adopt` spawns its resumed claude child, so that the claude CLI finds the adopted session; today the child starts in the caller's current directory.
Adds what `viola adopt` says and records when the resumed child ends at once after the start: a closed word, an exit code, a hint and an output line beside the existing refusal tables, an event and a log line, and whether the instance directory and the name are released after it.
If the fix moves where the R5 live-reading script must type `viola adopt`, that script changes with it, and its rehearse form reaches the changed path with no real claude.
Makes no live start and runs no real claude; P3 measures from records only what the wrapper kept of the two red children, where revive and up get their resume folder, and what the capability ledger holds about resuming from another folder.
@task_file n=1 path=src/cmd/adopt.rs mark=edits
what: the viola adopt command, which starts a wrapped, resumed claude child for an adopted session — edits: the spawn folder of the adopt start, and its handling of a child that ends at once (closed word, exit, hint, output line, event, log line, release of the instance directory and name)
@task_file n=2 path=scripts/r5-adopt.sh mark=edits
what: the R5 adopt live-reading rig script, which types `viola adopt` in a folder of its own and ends only its own processes — edits: the folder its live arm types the command in, and its rehearse form
@task_item n=1 part=needs
The spawn directory of adopt's resumed claude child: where it comes from, and the rules that a recorded cwd is kept to the snapshot only and is used only to spawn, as revive does with its recorded cwd.
@task_item n=2 part=needs
A resume folder that is missing or unusable at adopt's start: what adopt refuses with, by analogy with revive's cwd-missing code, and that the refusal names no path.
@task_item n=3 part=needs
The new early-exit outcome of viola adopt: a closed code word, a CLI exit code, a stderr hint and a human line and a --json error document, each fixed text with no path, pid, session id or upstream text interpolated, and control characters escaped in human output.
@task_item n=4 part=needs
The event and the log line adopt records for a child that ends at once: what they may hold (closed words, exit status, time to exit), and that no claude output, CLAUDE value, session content or path enters them; full detail only in diagnostics.
@task_item n=5 part=needs
Telling a child that ended at once from other outcomes by the exit status and timing the wrapper already holds, reading the child's output only tolerantly and as content, never as a command, never by matching upstream error text.
@task_item n=6 part=needs
Release of the instance directory and the name after an early exit: only the instance's own paths, built from the validated name, are removed or freed; the start collision check for a later start still holds.
@task_item n=7 part=needs
Adopt acts only on the child it started: no signal, end or resume of any process or session it did not start, and the early-exit path ends nothing foreign.
@task_item n=8 part=needs
The order of adopt's start checks stays: strict-modes check of the home tree before any log opens, socket-directory check before any bind, snapshot or child, and the adopt strict-modes refusal unchanged.
@task_item n=9 part=needs
The resumed child's argv: the session id passed as one argv element of a closed shape, no shell between viola and the child, claude resolved from PATH with the pinned folder first.
@task_item n=10 part=needs
The resumed child's environment in a new folder: parent identity variables and inherited CLAUDE credentials stripped by name, the strip recorded by names only, as for every claude child.
@task_item n=11 part=needs
viola writes nothing under the Claude home and in Claude's config: finding the folder a session was made in must not write there, and any read of the Claude projects store is bounded, read-only and keeps no transcript byte or path outward.
@task_item n=12 part=needs
The start trust step when the child is spawned in a folder other than the caller's: start skips the home folder and already trusted folders, and writes the trust record only by its guarded writer, flag only.
@task_item n=13 part=needs
The R5 adopt rig: it ends only its own processes, keeps only closed words of what it reads, its child isolated from the user's config with any credentials copy transient and unrecorded, and its rehearse form runs no real claude.
@task_item n=14 part=needs
The real claude CLI runs only locally: no CI job and no test of this chunk starts a real claude; the new behaviour is proved with the fake agent.
@task_item n=15 part=needs
New tests of the early-exit path and the spawn folder: a test seam that makes the fake child end at once lives only under the fake-agent feature and is compiled out of release; tests use their own home and remove only their own dirs.
@task_item n=16 part=needs
The new error variant for an early exit follows the error-enum convention and fixed display messages, and its writer is a named product error writer.
@task_standing n=1
convention: every new test uses its own viola home and never the user's real home or Claude home
home: secret-management
@task_standing n=2
convention: every new test seam lives only under the fake-agent feature and is compiled out of the release build
home: security-anti-patterns
@task_standing n=3
convention: every new environment read or seam is entered in the log, and test-side removal touches only dirs the test made
home: security-anti-patterns-never-do-these
@task_standing n=4
convention: every new recorded fixture is scrubbed of home paths, usernames and absolute paths, reviewed before commit, and kept under the hygiene walk
home: data-protection
@task_standing n=5
convention: every new CI job or test runs no real claude CLI and holds no secrets
home: threat-model-summary
@task_standing n=6
convention: every new error writer is named and every new error message interpolates nothing
home: error-handling
@task_item n=1 part=not_needed
The MCP server surface: its tools, tool inputs, stdin line cap and isError results; adopt's early exit is not an MCP tool outcome.
@task_item n=2 part=not_needed
Typed and pasted text rules: the paste character set, control-character refusal of send text, bracketed paste and line-ending normalisation; the chunk types no text into a session.
@task_item n=3 part=not_needed
Dialog answering and hook payload handling: answer behaviour, hook stdin caps, hook decision details and the hook capture arm; nothing of the hook path changes.
@task_item n=4 part=not_needed
Window and focus rows: the Hyprland recorder, window origin, focus and open ancestor walks; adopt's resume folder is no window concern.
@task_item n=5 part=not_needed
Link and unlink, lineage and launch pair rows: their channel methods, frames and event lines; the chunk adds no link, parent or model and effort flag.
@task_item n=6 part=not_needed
The switch and pause and release channel methods, and the driver-originated release guard; the chunk changes no channel method.
@task_item n=7 part=not_needed
Board rendering of viola list: terminal colour facts, the whole-board refusal and the re-parse of shown values; the board is not changed.
@task_item n=8 part=not_needed
Plugin install and marketplace rows, and the pinned plugin files rewritten at start; the chunk installs nothing.
@task_item n=9 part=not_needed
Release, publish, signing and self-update rows, and the fuzz workspace with its nightly pin and lockfile audit; the chunk ships no release and adds no parser to fuzz.
@task_item n=10 part=not_needed
Verify's probe rows: its probe directories, echo, context and cross-session probe residuals and its unclean-fixture refusal codes; the chunk adds no verify probe.
@task_item n=11 part=not_needed
Messaging credentials and absences of a network surface, accounts, databases, webhooks and uploads; the chunk adds no network or account surface.
