@task_prose part=session
The session built the fleet console's view: viola console now draws the board's lineage forest, state words, wheel and a seventh LAST cell between its title and key line, every cell control-escaped, wide lines cut ending in dots, a tall fleet ending in a more line.
The console re-reads the board from disk once a second, replays no unchanged log, and redraws and logs its frame line only when the view changes.
The console runs the claude agents read on its own thread at most every 5 s, off the key loop; a failed read hands an empty list so STATUS reads unknown.
The board read model gains a last-activity time per instance, the newest log line time, never serialised into list --json or the MCP list item; the board read is rebuilt on per-instance read and relive steps with its order and callers unchanged.
A home widened mid-run ends the console with exit 1, the home pair and one process-exit strict-modes-failed line; the console-frame diagnostics line gains shown and hidden counts.
No IPC method, endpoint, socket, port, env var, dependency, crate, config key or event kind was added; the live half of the console's acceptance stays gated on the operator's reading.
@task_file n=1 path=crates/viola-core/src/obs.rs
what: the shared observability event fields in viola-core, here the console-frame diagnostics line's fields
@task_file n=2 path=crates/viola-state/src/board.rs
what: the board read model in viola-state, the sync Claude-free read of every instance's snapshot and log that the CLI list and MCP list share, its board item and its skip counts
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostics line schema, every process-log line's shape per event
@task_file n=4 path=src/cmd/console.rs
what: the console subcommand's entry in the root bin, wiring the console's board source and its agents read
@task_file n=5 path=src/cmd/list.rs
what: the list verb, the caller-side board read joined with the claude agents observations, behind both the CLI list and the MCP list tool
@task_file n=6 path=src/console/frame.rs
what: the console's frame geometry, the rows its body may use
@task_file n=7 path=src/console/mod.rs
what: the terminal fleet console's loop, its key handling, terminal restore, board re-read interval and redraw on a change
@task_file n=8 path=src/human.rs
what: the root bin's one human writer module, the human text output of every verb
@task_file n=9 path=src/human/board.rs
what: the human board rendering of list, its captions and rows
@task_file n=10 path=src/human/board/cells.rs
what: the human board's cells, each upstream value escaped into a row cell
@task_file n=11 path=src/human/board/rows.rs
what: the human board's rows, the lineage forest order and indent and the six fixed captions
@task_file n=12 path=src/human/board/test_support.rs
what: the human board's test fixtures, a board item literal
@task_file n=13 path=src/human/console.rs
what: the console's human frame writer, the title, body and key line drawn on the terminal
@task_file n=14 path=tests/tui_console.rs
what: the console's end-to-end cases under their own pty and test home, now with a test-scoped PATH on every start
@task_file n=15 path=src/console/fleet.rs
what: new; the console's board source, a strict-modes scan of the home re-read without replaying an unchanged log, and its agents-read thread
@task_file n=16 path=src/console/view.rs
what: new; the console's view, the drawn lines cut to the terminal with shown and hidden counts, compared to decide a redraw
@task_file n=17 path=src/human/fleet.rs
what: new; the fleet text of the console, the board's rows with a seventh LAST age cell and the line cut that never splits an escape
@task_item n=1 part=touches
decision/terminal-fleet-console and the Project Intent overview, which says the fleet console is planned and not built: the console now draws the fleet view, its live acceptance still gated.
@task_item n=2 part=touches
decision/board-read-model-shape: the board item gains a serde-skipped last-activity time; the model stays sync and Claude-free and carries no endpoint, pinned bin, cwd, pid or started_at.
@task_item n=3 part=touches
decision/agents-join-read-ungated and decision/mcp-list-reads-agents: the console is a new caller of the claude agents read, on its own thread every 5 s rather than per call.
@task_item n=4 part=touches
contract/board/agents-read-failure, contract/board/status and decision/agents-listing-never-errors: in the console a failed agents read hands an empty list, so STATUS reads unknown and the unwrapped rows go.
@task_item n=5 part=touches
decision/list-reads-state-directly, decision/list-is-caller-side-read and decision/list-needs-no-wrapper: the board read they rule is rebuilt on per-instance read and relive steps and now also feeds the console; its order and callers are unchanged.
@task_item n=6 part=touches
contract/list-payload/items and the contract/list-item rows: no list --json item and no MCP list item carries the new last-activity time.
@task_item n=7 part=touches
contract/list-payload/skipped, contract/list-json/skipped and decision/reader-counts-skipped-lines: the scan's skip counts are now read through a shared step, and the console logs its skip line only when the count changes while list still logs it per scan.
@task_item n=8 part=touches
decision/board-refused-whole, decision/list-home-read-rule and decision/strict-read-rule: the console's board source reads behind the same strict-modes scan, and a home widened mid-run refuses the whole board with exit 1.
@task_item n=9 part=touches
contract/cli-exit-codes/1 and contract/process-exit-detail rows: the console gains an exit 1 with one process-exit strict-modes-failed line on a mid-run refusal.
@task_item n=10 part=touches
decision/silent-roles-at-internal-error-catch and contract/cli-internal-error/output: a console board read whose scan errs returns after the terminal restore and reaches the cli catch site as internal error.
@task_item n=11 part=touches
contract/diag-line/state-recovered and contract/diag-line/format: the console now writes state-recovered per replay that runs, parse-rejected for the state-entry parser on a count change, and the agents-probe pair.
@task_item n=12 part=touches
contract/diagnostics-role-files rows: the console's own role file now holds board and agents lines, and no row lists that role file.
@task_item n=13 part=touches
decision/control-characters-refused, decision/untrusted-upstream-text, decision/upstream-text-is-content and decision/option-text-never-in-human-line: every console cell is row-escaped and the line cut never splits an escape.
@task_item n=14 part=touches
decision/single-human-writer: the console's fleet text and frame body are new human output, written through the root bin's human module.
@task_item n=15 part=touches
decision/channel-blocks-never-polls, decision/wait-blocks-nothing-polls and absence/polling-endpoints: the console re-reads the disk once a second; it polls no channel and no endpoint.
@task_item n=16 part=touches
decision/bounded-readers and decision/process-liveness-signals: the console's agents read resolves claude on PATH and is bounded by MAX_FRAME and the 5 s deadline.
@task_item n=17 part=touches
decision/codes-only-process-logs and decision/process-logs-hold-codes-only: the console's new lines hold counts and codes, no instance name or session id.
@task_item n=18 part=touches
decision/test-isolated-viola-home, decision/no-live-claude-in-ci and decision/real-claude-local-only: every console test start now sets PATH to an empty test-scoped bin dir so no test reads the host's real claude sessions.
@task_item n=19 part=touches
name/command/console: the console verb's name now carries the fleet view, no new name taken.
@task_fact n=1 slug=standard-contracts
fact: The fleet console re-reads the board from disk once a second, replays no log whose file stamp is unchanged, and redraws and logs its console-frame line only when its view or size changes.
@task_fact n=2 slug=standard-contracts
fact: The fleet console reads claude agents on its own thread at most every 5 s, off the key loop; a failed read hands an empty list, so STATUS reads unknown and unwrapped rows are not shown, never a stale busy.
@task_fact n=3 slug=project-directory-structure
fact: The board item carries a last-activity time, the newest whole log line time that parses as RFC 3339, none for an unwrapped row or an empty log; it is never serialised, so no list --json item and no MCP list item carries it.
@task_fact n=4 slug=standard-contracts
fact: A home widened while the console runs ends it with exit 1, the home pair and one process-exit strict-modes-failed line, after the terminal is restored; a scan that errs ends it as internal error.
@task_fact n=5 slug=standard-contracts
fact: The fleet console's view is the board's rows in its forest order with the six captions then a seventh LAST cell in one unit, an age or unknown or n/a, a wide line cut ending in dots and a tall fleet ending in a more line.
@task_item n=1 part=untouched
The wrapper channel, its methods, frames, fault codes and protocol version: no IPC method, endpoint or socket was added.
@task_item n=2 part=untouched
Hooks, the hook-to-kind map, dialogs, the wheel, sends and their refusals and confirmations.
@task_item n=3 part=untouched
The stack and dependencies, cargo-deny policy, toolchain and MSRV, the build system and CI: no dependency, crate or workflow changed.
@task_item n=4 part=untouched
The instance snapshot and snapshot envelope contracts, the events log and normalised event kinds: no event kind and no snapshot field was added.
@task_item n=5 part=untouched
config.json keys, the environment variable namespace and the harness variables: no config key or env var was added.
@task_item n=6 part=untouched
Verify, the capability ledger and its stamps, pinned copies, the plugin install and the Hyprland window seam.
@task_item n=7 part=untouched
run, revive, up, adopt, switch, open and focus verbs and their exit codes and refusal documents.
