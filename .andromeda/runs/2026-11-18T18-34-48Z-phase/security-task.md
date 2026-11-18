@task_prose part=chunk
Lands a new recorded fixture set for claude 2.1.325, recorded whole by a clean live viola verify --record under the live rig's private config, through the recorder's scrub, no file hand-edited.
Re-records the MCP tool list fixture live through the test-only MCP tap on 2.1.325 and places it in the new set.
Re-reads the ledger probes' STAMPED reading and the drift suites against the new set, which the fake agent replays in CI.
Measures live: a StopFailure hook payload and its error word against the cause map, the /context post-condition, whether /mcp disable writes the user's real Claude config, the input box's placeholder hint, and whether verify's Run E starts.
Re-fires the remove-the-guard control for the browser-tools row on a filtered run; the pinned product binary is not rebuilt or re-pinned.
@task_file n=1 path=fixtures/claude/2.1.325
what: the new recorded fixture set for claude 2.1.325 — every file the verify recorder writes, scrubbed, refused whole on an unclean payload, replayed by the fake agent in CI
@task_file n=2 path=fixtures/claude/2.1.325/Mcp.tools-list-1.json
what: the MCP tool list fixture recorded through the test-only MCP tap, holding the list tool's description
@task_item n=1 part=needs
Recorded fixture hygiene over every recorded file: no host path, user name, email, token or status-line text; a payload holding /home/ is refused whole with its unclean-fixture refusal code, never scrubbed or edited by hand.
@task_item n=2 part=needs
The home a live viola verify --record takes comes from a record-home helper and holds no path component named home, so the recorder's home-path refusal does not fire.
@task_item n=3 part=needs
MCP fixtures are recorded only through the test-only tap; the list tool's description is read from the live re-record and the MCP tool list stays an affordance, never enforcement.
@task_item n=4 part=needs
The live rig's child runs isolated from the user's Claude config under a private config: its credentials copy transient and unrecorded, its config seeded from bundle logic only, the extension cache read by key name only; viola writes nothing under the Claude home, and whether /mcp disable writes the user's real config is measured.
@task_item n=5 part=needs
Verify never answers a dialog and never types into one; viola never answers or seeds the CLI's browser-tools prompt; a real-CLI verify needs a prepared pre-trusted folder.
@task_item n=6 part=needs
Verify reads the screen only by signatures against compiled literals; the box-only screen and the input box readiness (the placeholder hint against typed text) are read that way, and the stamp's first screen is kept in closed words.
@task_item n=7 part=needs
A live StopFailure hook payload is read inside a started session's turn: its error word mapped to the nine-word API-error cause map kept inside the Claude adapter, the turn end carrying no error text, no parse-rejected unknown-value line for a word of the CLI's thirteen; hook payload parsing stays tolerant and the payload is content.
@task_item n=8 part=needs
The live re-stamp: only viola verify writes the stamps file, read through the strict-modes check; every stamp written before the 24 relied rows reads unverified until verify runs again, and the unverified-CLI gate holds the version until then.
@task_item n=9 part=needs
The pinned product binary that drives the session is neither rebuilt nor re-pinned nor replaced; it is re-hashed before reuse and its pinned folders kept.
@task_item n=10 part=needs
Verify's Run E resume target: the clean record shows whether Run E starts, read from a SessionStart capture the resume target accepts.
@task_item n=11 part=needs
The real claude CLI runs only locally; CI holds no secrets and no real claude, and replays the new set through the fake agent; drift reports go only to diagnostics.
@task_item n=12 part=needs
Every live start names the 2.1.325 install by its absolute path rather than the bare claude a PATH lookup would resolve to; verify's children are spawned directly.
@task_item n=13 part=needs
Live session screens are kept local and never committed; verify probe captures are transient, probe effects stay in their own directory and probe directories are removed; probe prompts are synthetic.
@task_item n=14 part=needs
The live-round rig keys only into its own pty and its prompt capture keeps closed words only.
@task_item n=15 part=needs
The remove-the-guard control for the browser-tools row is fired on a filtered run so the named case is read red without the guard; any test seam it uses stays under the fake-agent feature and compiled out of release.
@task_item n=16 part=needs
Parent identity and inherited CLAUDE credential variables are stripped from the live child by name, never serialised or recorded into a fixture.
@task_standing n=1
convention: each recorded fixture file keeps only signature rows, passes the hygiene walk and is reviewed before commit
home: data-protection
@task_standing n=2
convention: each new test or live run uses its own home, kept out of git, and removes only the directories it created
home: secret-management
@task_standing n=3
convention: no secret, credential literal or local home lands in git with a new file
home: security-anti-patterns-never-do-these · security-anti-patterns
@task_item n=1 part=not_needed
The channel's peer verification, socket directory and socket file modes, and their refusal contracts per verb: the chunk adds no listener, client seam or channel method.
@task_item n=2 part=not_needed
The typed-text and dialog free-text character sets and the send and answer refusal words: no typed text path changes.
@task_item n=3 part=not_needed
revive, its arguments and its --json codes, and the recorded cwd of a snapshot: the chunk touches no revive path.
@task_item n=4 part=not_needed
link and unlink, their replies and the event log's link lines: untouched.
@task_item n=5 part=not_needed
pause and release, the driver-originated release guard and its contracts: untouched.
@task_item n=6 part=not_needed
The Hyprland recorder, its reply-text drop and the hyprctl deadline: no window-manager reading is recorded here.
@task_item n=7 part=not_needed
config.json keys and their bad-value handling: no configuration key is added or read anew.
@task_item n=8 part=not_needed
Release, signing and deploy rows, release-check output and attested release builds: the chunk ships no release.
@task_item n=9 part=not_needed
The board scan and viola list's home read rule and colour facts: untouched.
@task_item n=10 part=not_needed
The CLI exit-code table, the --json error documents and the instance-unreachable detail causes: no CLI verb's output changes.
