@task_prose part=chunk
Adds a new flat human CLI verb `viola up` that restores the fleet after a host reboot (capability v2-12).
It reads the board's instances under the home, selects those that were live before the host went down, orders them parents before children by the recorded lineage, and skips each one whose recorded pid and start time verify live.
It re-opens each remaining instance resumed in place through the existing revive start arm, in the terminal window viola open would give it, in its recorded workspace.
It prints one line per instance through the human module naming the instance and a closed outcome word, and under --json one document of codes and closed words only.
It adds tests over the fake agent and the Hyprland fakes under a test-owned home, and re-measures at HEAD the lineage and launch lines, the open window record and revive resuming in place, never copied.
@task_file n=1 path=src/cmd/mod.rs mark=edits
what: the root bin's clap dispatch, the CLI verb table and its help rows — edits: a new up verb entry
@task_file n=2 path=src/cmd/up.rs
what: the new viola up verb: fleet selection, parent-before-child order, live skip, and re-open of each instance through revive in an open-style terminal window
@task_file n=3 path=src/human/verbs.rs mark=edits
what: the human module's verb output lines — edits: the per-instance viola up line with its closed outcome word
@task_file n=4 path=tests/cli_up.rs
what: the new CLI tests proving viola up's selection, order, skip, per-instance lines, --json document and the argv handed to each start, over the fake agent and the Hyprland fakes under a test-owned home
@task_item n=1 part=needs
The new verb's name and place in the flat CLI verb table, the viola --help verb groups and its help row; the human-command name register gains up.
@task_item n=2 part=needs
viola up's human output: one line per instance on the right stream, naming the instance and a closed outcome word (re-opened, skipped as live, refused), ASCII only, two-space columns, control characters escaped, no spinner, no elapsed counter, no step counter beyond the one progress pattern.
@task_item n=3 part=needs
viola up's --json result: one stdout document of codes and closed words only, beside the existing CLI machine contract; no hint line under --json, clean failure documents, the --json catch-site scope and internal-error form.
@task_item n=4 part=needs
viola up's exit codes and exit-cause tables: what 0, 1, 2, 20 and 21 mean for a fleet verb, and the exit-21 causes it can return (strict-modes-failed, socket-dir-refused and the rest).
@task_item n=5 part=needs
Revive's start arm used per instance: its preflight (claude agents --json, session-live), its refusal order, its closed --json refusal codes (already-live, cwd-missing, no-session, session-live, stale-heartbeat, squatted-name, pinned-hash-mismatch, socket-dir-refused, strict-modes-failed, internal-error) and how each folds into the per-instance outcome word.
@task_item n=6 part=needs
Run and revive start refusals keep their fixed message, show no path or pid, and carry no internals; the per-instance refusal line keeps the fixed unable form and its hint only in human mode.
@task_item n=7 part=needs
The terminal window viola open gives each instance: the wrapper's window record (address, workspace, placement, origin) under --origin, the origin word cli, the closed placement words, and the Hyprland seam's closed request set.
@task_item n=8 part=needs
The lineage record (parent, role) as the source of the parent-before-child order, and the launch pair (model, effort) carried at every start the verb makes.
@task_item n=9 part=needs
The board read model and its liveness reading (snapshot pid plus start time, heartbeat) that decides which instance is skipped as still live and which reads as was-live-before-the-host-went-down; stale entries stay on the board, never deleted.
@task_item n=10 part=needs
Strict-modes checks on the home tree before the verb reads or starts anything, and the instance-state and home-state refusal pairs it raises on failure.
@task_item n=11 part=needs
Unwrapped sessions are never a target: a session started by hand with no wrapper record is not revived by up, and the outcome of such an instance is a closed word, not a guess.
@task_item n=12 part=needs
Nothing announces success before confirmation: an instance's re-opened word prints only once revive's start is confirmed, and no document follows once a child held the terminal.
@task_item n=13 part=needs
The obs event catalog gains a per-instance up line under the NEVER-log floor: no address, pid or workspace in a process-log line.
@task_item n=14 part=needs
The verb is CLI-only for a driver session: no MCP tool exposes fleet re-open, the mcp verb stays hidden and unchanged, and both CLI callers (human on a TTY, LLM driver) read the same closed words.
@task_item n=15 part=needs
No interactive prompt or pager and no colour beyond amber and dim; COLUMNS is never read; colour decision order (--json, NO_COLOR, stdout not a terminal, TERM=dumb) governs the up line.
@task_item n=16 part=needs
Every live reading of up (a real host restart, a real resumed session, a real window) is the operator's at the host and named in the plan; the claim is GATED on it where the fakes cannot carry a clause, and the tests prove only over the fake agent and Hyprland fakes.
@task_standing n=1
convention: every new CLI verb's human output is ASCII-only, escapes control characters, prints data on stdout and messages on stderr, and carries no internals, emoji, check glyphs or spinners
home: anti-patterns-never-do-these
@task_standing n=2
convention: every new verb's --json output is one stdout document of codes and closed words, with no hint line and a clean failure document
home: surface
@task_standing n=3
convention: every new verb's outcome and refusal lines use the fixed phraseology vocabulary and confirm nothing before confirmation
home: brand-identity
@task_item n=1 part=not_needed
Web page surfaces, rendered-page tokens, surface scale, radius, depth and spacing tokens: viola renders no page and up draws none.
@task_item n=2 part=not_needed
The flight-progress strip, readback box, send readback and the fleet console's strip-board rows: up is a verb's output, not the console, and sends no prompt.
@task_item n=3 part=not_needed
viola send, answer, wait, last, link, unlink, pause and release output contracts and their refusal hints (not-delivered, human-typing, paste-cut and the like): up delivers no text and transfers no control.
@task_item n=4 part=not_needed
viola verify output, verify refusals, stamps and the run and revive automatic-verify line: up runs no verification of the CLI.
@task_item n=5 part=not_needed
viola plugin install output and its strict-modes refusal: up installs no plugin.
@task_item n=6 part=not_needed
Peer reply screening and wrapper fault --json detail rows of the wrapper socket protocol: up talks to no peer wrapper over the socket.
@task_item n=7 part=not_needed
The viola list table's columns, BAY line, NAME width and cut, WHEEL column width and skip counts as the board's display: up reads the board model, it does not render the list table.
@task_item n=8 part=not_needed
viola focus's exit-1 codes and output: up opens windows through the open path and focuses none.
@task_item n=9 part=not_needed
viola adopt, switching model and effort, and the founder's host launcher scripts: their own entries, outside this chunk.
