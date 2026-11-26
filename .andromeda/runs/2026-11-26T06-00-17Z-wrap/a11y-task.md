@task_prose part=session
This session is the gated run of the chunk placing without moving the view. It changes no code, test, script or master and edits no file of the tree's source.
It measured the gated live check R7 once at the operator's host on the real Hyprland compositor: after a placing viola open, the window step had restored the active workspace its move took, and R7 read placed with the active workspace unchanged.
That reading cleared the chunk's gated entry, and the window-placing capability now reads implemented with its clearance. CI on the pushed pre-CI commit read green 7/7.
Still unproven, and not an acceptance of the capability: where keyboard focus lands after the restore, and what a failed restore looks like live.
No count, threshold, bound or qualifier moved; nothing was retired and no new surface was added.
@task_item n=1 part=touches
The window step's wait bound of 5 s (decision window-wait-bound): the step now also restores the active workspace its move took; R7 confirmed it on the live compositor with no change to the bound.
@task_item n=2 part=touches
Each compositor read bounded at 5 s (decision hyprctl-read-deadline): the restore of the active workspace goes through compositor reads; the bound is unchanged.
@task_item n=3 part=touches
Live readings never gate (decision live-readings-never-gate): R7 is a live reading fired once at the operator's host whose green result cleared a gated entry and let the capability read implemented; a picker should check whether the row still holds as written.
@task_item n=4 part=touches
Manual checks are supplemental only, and the manual screen-reader pass gates nothing (decisions manual-checks-supplemental-only, manual-a11y-pass-gates-nothing): R7 is an operator-fired check on the operator's desktop, not a screen-reader or keyboard pass; check that no row is contradicted.
@task_item n=5 part=touches
Orca with foot on Hyprland for the tui (decision screen-reader-target-tui): the measured environment was foot on Hyprland; where keyboard focus lands after the workspace restore is still unproven, which bears on a screen-reader user of the wrapped tui.
@task_item n=6 part=touches
Tui terminal belongs to the child, human keystrokes never blocked, and focus, mouse and resize never move the wheel (decisions tui-terminal-belongs-to-child, human-keystrokes-never-blocked, wheel-ignores-terminal-events): the workspace restore can move compositor focus; focus landing after it remains unproven.
@task_item n=7 part=touches
Time bounds fall on the driver only, never on the human (decisions time-bounds-fall-on-the-driver, time-bounds-never-on-human): the window step's restore is a driver-side step under its existing bound; nothing new waits on the human.
@task_item n=8 part=touches
Gate breach fails the build, zero flakiness, and a11y checks in the GitHub Actions push workflow (decisions a11y-gate-breach-fails-build, zero-flakiness, a11y-ci-github-actions): CI on the pushed commit read green 7/7 with no retries; no threshold moved.
@task_item n=9 part=touches
Zero viola bytes on the terminal (decision zero-viola-bytes-on-the-terminal): the window step acts on the compositor, not the terminal; R7 writes no viola byte to the child's terminal.
@task_item n=10 part=touches
Linux only (decision linux-only): the restore and its live check are on Hyprland under Linux; no other platform is reached.
@task_item n=1 part=untouched
CLI output discipline rows: control-character escaping, SGR and colour rules, word beside every SGR span, dim stale rows, amber DIALOG and bold NAME: no printed output changed.
@task_item n=2 part=untouched
CLI refusal, fault and --json failure contracts, exit codes, hint and unable lines, reason codes and code words: no refusal, code or exit changed.
@task_item n=3 part=untouched
viola list board, caption, unwrapped separator and NAME truncation contracts: the board is not reached.
@task_item n=4 part=untouched
viola revive, link and unlink, switch, up, wait, send and plugin install contracts, and the frame verb, hook call and MCP tool refusals: none changed.
@task_item n=5 part=untouched
viola --help rows and fixed CLI copy pinned by literal: no string changed.
@task_item n=6 part=untouched
The absences (internationalization, touch surface, web or graphical frontend, rendered page, user time limit) and the N/A areas for browser a11y tooling: still absent.
@task_item n=7 part=untouched
Structured violation JSON and the harness rows on nextest, coverage and doctest: no test, suite or harness changed.
