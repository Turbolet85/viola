@task_prose part=session
The session built the fleet console's view: viola console now draws the board's lineage rows, state words, wheel and a seventh LAST cell (age of the newest whole log line) between its title and key line, every cell control-escaped.
The console re-reads the board once a second behind the board scan's strict-modes checks, skips replaying an unchanged log by a stamp that follows no link, and redraws only on a change.
The console now runs the claude agents read on its own thread at most every 5 s, off the key loop; a failed read hands an empty list so STATUS reads unknown.
A home widened mid-run ends the console with exit 1, the home refusal pair and one process-exit strict-modes-failed line; a scan that errs returns internal error after the terminal restore.
The console-frame diagnostics line gains shown and hidden counts and is written only when a field changes; the console newly writes agents-probe, state-recovered and parse-rejected lines to its own log.
No IPC method, endpoint, socket, port, env var, dependency, config key or event was added; list --json and the MCP list item are unchanged; nothing retired.
@task_file n=1 path=crates/viola-core/src/obs.rs
what: edited — the shared observability vocabulary in the core crate (diagnostics line subjects and fields)
@task_file n=2 path=crates/viola-state/src/board.rs
what: edited — the board read model and the board scan over the home's instances (strict-modes scan, snapshot or replay read, per-instance read, skip counts); items gain an in-memory last-activity time never serialised
@task_file n=3 path=schemas/diag-line.v1.json
what: edited — the diagnostics line schema; the console-frame arm gains required shown and hidden counts
@task_file n=4 path=src/cmd/console.rs
what: edited — the viola console verb entry: its terminal and home checks, its sources, the agents read handed to the console's thread, the exit on a mid-run refusal
@task_file n=5 path=src/cmd/list.rs
what: edited — the viola list verb; its agents observations helper is now shared with the console
@task_file n=6 path=src/console/frame.rs
what: edited — the console frame's size floor and the body rows between the title and the key line
@task_file n=7 path=src/console/mod.rs
what: edited — the console run loop: raw-mode terminal, bounded key reader, restore, once-a-second board re-read, redraw on a change, console-frame logging, refused and failed board reads
@task_file n=8 path=src/human.rs
what: edited — the human-mode output module, its row escape and console frame writer
@task_file n=9 path=src/human/board.rs
what: edited — the board's human-form renderer shared with the console
@task_file n=10 path=src/human/board/cells.rs
what: edited — the board's human-form cells (state words, wheel, re-parsed launch pair, lineage)
@task_file n=11 path=src/human/board/rows.rs
what: edited — the board's forest rows, captions, the unwrapped separator and the width-capped name rule
@task_file n=12 path=src/human/board/test_support.rs
what: edited — the board renderer's test fixture item
@task_file n=13 path=src/human/console.rs
what: edited — the console frame writer; it now draws the body lines from row two
@task_file n=14 path=tests/tui_console.rs
what: edited — the console's end-to-end pty tests; every console start now runs with PATH set to an empty test-scoped bin dir and its own test home
@task_file n=15 path=src/console/fleet.rs
what: new — the console's fleet source: the strict-modes board scan re-read, a link-following-free stamp of each instance's snapshot and log, the skip-count line on a change, and the agents thread spawning the claude agents read
@task_file n=16 path=src/console/view.rs
what: new — the console's view: fleet lines cut to the terminal's columns, the more line, shown and hidden counts
@task_file n=17 path=src/human/fleet.rs
what: new — the fleet text: the board's rows plus the LAST age cell, every cell control-escaped, the whole line cut at the columns ending in dots and never inside an escape
@task_item n=1 part=touches
decision/claude-agents-resolved-from-path: viola console is a new caller, resolving claude on its PATH and running the agents read on its own thread every 5 s.
@task_item n=2 part=touches
decision/claude-agents-read-fail-open: the console's failed agents read hands an empty list, STATUS reads unknown and unwrapped rows go; the read stays bounded by MAX_FRAME and the 5 s deadline.
@task_item n=3 part=touches
decision/agents-probe-log-content-free and decision/agents-probe-log-redaction: the console now writes the agents-probe pair into its own log; session ids reach no line.
@task_item n=4 part=touches
decision/child-path-trust and decision/r8-claude-env-strip: the console now spawns a claude child for the agents read, under the R8 strip with HOME inherited.
@task_item n=5 part=touches
decision/console-terminal-and-home-check and contract/strict-modes-refusal/console: the home is now re-checked every second; a home widened mid-run ends with exit 1, the home pair and one process-exit strict-modes-failed line.
@task_item n=6 part=touches
decision/board-home-read-rule, decision/list-home-read-rule, decision/whole-board-refusal and decision/list-scan-checks-before-open: the console reads the board behind the same strict-modes scan each second, and its stamp follows no link.
@task_item n=7 part=touches
decision/human-output-escapes-controls and decision/escape-terminal-controls: every fleet cell is row-escaped, and the line cut never splits an escape.
@task_item n=8 part=touches
decision/bounded-readers-census and decision/bounded-external-readers: new readers — the console's per-second snapshot and log re-read and its agents thread's child read.
@task_item n=9 part=touches
decision/readers-tolerate-torn-lines: last activity is the time of the newest whole log line that parses as RFC 3339; none on a log with no line.
@task_item n=10 part=touches
decision/board-launch-pair-re-parsed, decision/board-lineage-and-links-name-checked and decision/board-window-and-origin-re-parsed: the board's cells are now drawn in the console too, through the same renderer.
@task_item n=11 part=touches
decision/unwrapped-name-never-target and decision/agents-row-name-not-a-target: the console draws unwrapped rows from the agents list; an unwrapped name is not cut by the name width rule, the whole line is cut instead.
@task_item n=12 part=touches
contract/mcp-tools/list and decision/cwd-only-in-snapshot: the board item gains last activity, held in memory only; no list --json item and no MCP list item carries it.
@task_item n=13 part=touches
decision/nothing-written-under-claude-home, decision/no-writes-under-claude-home and decision/viola-writes-nothing-in-claude-dir: whether the console's claude agents child writes under the real Claude home is measured nowhere.
@task_item n=14 part=touches
decision/tests-use-own-home and decision/real-cli-local-only: every console test start now sets PATH to an empty test-scoped bin dir so the agents thread reads no host claude sessions.
@task_item n=15 part=touches
decision/replayed-snapshot-inert and decision/replayed-snapshot-yields-nothing: the console now replays a snapshot from the log per read and logs state-recovered; an unchanged log is not replayed.
@task_item n=16 part=touches
decision/every-error-writer-named and contract/internal-error-output/json: a console board scan that errs returns after the terminal restore and reaches the cli catch site as internal error.
@task_item n=17 part=touches
decision/console-key-reader-bounded: keys still quit while the agents read runs off the loop.
@task_item n=18 part=touches
decision/new-env-read-needs-log-entry: the console's new background child read is logged through the agents-probe pair; no env var or seam added.
@task_item n=19 part=touches
decision/user-content-only-to-diagnostics and decision/never-log-claude-values: instance names and versions are drawn on the screen only; no console line holds a name.
@task_fact n=1 slug=input-validation
fact: viola console re-reads the board once a second through the board scan's strict-modes checks, and decides whether a log changed by a symlink_metadata stamp that follows no link, replaying only a changed log.
@task_fact n=2 slug=error-handling
fact: a console board read whose scan errs (an unlistable instances directory, a state error, not a refusal) restores the terminal and returns to the cli catch site as internal error.
@task_fact n=3 slug=data-protection
fact: the board item's last-activity time is held in memory only and never serialised: no list --json item and no MCP list item carries it.
@task_fact n=4 slug=data-protection
fact: an agents child of viola console may outlive a quit by up to its 5 s bound; accepted as built, no owner.
@task_fact n=5 slug=secret-management
fact: whether the console's claude agents --json child, run with HOME inherited, writes under the real Claude home is unmeasured; console tests run with PATH set to an empty test-scoped bin dir so none reaches a host claude.
@task_fact n=6 slug=security-anti-patterns-never-do-these
fact: the console cuts a too-wide fleet line at the terminal's columns ending in dots and never inside an escape sequence.
@task_item n=1 part=untouched
The local IPC channel, its socket directory, peer euid checks, server verification and channel methods: no method, endpoint, socket or port was added.
@task_item n=2 part=untouched
The MCP server's tools, stdin cap and input schemas: no tool and no item field changed.
@task_item n=3 part=untouched
Dependency security, CI workflows, release, cargo-deny, zizmor and pinned toolchains: no dependency or dev-tool changed.
@task_item n=4 part=untouched
The hook contract, paste and send text validation, answer and dialog rules: not reached.
@task_item n=5 part=untouched
viola verify, its probes, stamps and fixture hygiene; the live rigs: not reached.
@task_item n=6 part=untouched
Pinned binary placement and re-hash, plugin install, revive, run, adopt, switch and the trust record: not reached.
@task_item n=7 part=untouched
Configuration keys and the env-not-config rule: no config key or env var was added.
@task_item n=8 part=untouched
The absences (network, hosting, accounts, secrets, database, webhook): none gained or lost.
