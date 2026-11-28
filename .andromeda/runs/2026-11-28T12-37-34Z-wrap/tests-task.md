@task_prose part=session
The session built the fleet console's view: viola console now draws the board's lineage rows, state words, wheel and a seventh LAST cell (last activity age), every cell control-escaped, wide lines cut ending in dots, a tall fleet ending in a more line.
The console re-reads the board once a second (an unchanged log not replayed), reads the claude agents list on its own thread at most every 5 s, and redraws and logs console-frame only when the view or its fields change.
The board read gains a last-activity time per item that is never serialised: viola list, list --json and the MCP list item are unchanged; board::read is rebuilt on a per-instance read and relive.
console-frame's fields grow from 3 to 5 (shown, hidden); the console newly writes agents-probe pairs, state-recovered, parse-rejected on a count change, and one process-exit strict-modes-failed with exit 1 on a home widened mid-run.
Every console end-to-end start now runs with PATH set to an empty test-scoped bin dir so the new agents thread never reads the host's claude sessions; three e2e cases and many unit cases added; three controls fleet-cut, fleet-escape, view-redraw hold.
The live half of matrix v2-14 (the operator's tiled quarter-screen reading on his real home) was not fired and stays GATED; CI green.
@task_file n=1 path=crates/viola-core/src/obs.rs
what: the shared observability vocabulary, the log event and field names the process log lines carry
@task_file n=2 path=crates/viola-state/src/board.rs
what: the board read model, the read of every instance through read-or-replay into board items with liveness and skipped counts, now with a per-instance read, a relive that opens no log, and a last-activity time kept off serialisation
@task_file n=3 path=schemas/diag-line.v1.json
what: the diagnostics line schema the schema conformance check validates every log line against, its console-frame arm now requiring shown and hidden
@task_file n=4 path=src/cmd/console.rs
what: the viola console verb, its start, strict-modes refusal and exit, now handing the board source and the agents read to the console loop
@task_file n=5 path=src/cmd/list.rs
what: the viola list verb, its board read and the observations joined from the agents list, shared with the MCP list tool
@task_file n=6 path=src/console/frame.rs
what: the console frame geometry, the rows a frame holds and the fit floor
@task_file n=7 path=src/console/mod.rs
what: the console loop, its raw-mode enter and restore, key handling, resize, redraw and console-frame logging, now with the board re-read interval and a refused or failed board read ending the run
@task_file n=8 path=src/human.rs
what: the human-mode output module, the console frame writer and the row escaping of human output
@task_file n=9 path=src/human/board.rs
what: the human board, the viola list human board with its columns and unwrapped separator
@task_file n=10 path=src/human/board/cells.rs
what: the human board's cells, the state words, wheel and version cells and their control escaping
@task_file n=11 path=src/human/board/rows.rs
what: the human board's rows, the lineage forest order and indent and the six captions
@task_file n=12 path=src/human/board/test_support.rs
what: the human board's test fixture board items
@task_file n=13 path=src/human/console.rs
what: the console's human frame, the title line, body lines and key line drawn on the terminal
@task_file n=14 path=tests/tui_console.rs
what: the viola console end-to-end binary through an outer PTY, every start now with a test-scoped PATH, asserting on logged console lines
@task_file n=15 path=src/console/fleet.rs
what: the console's fleet source, the board re-read under the strict-modes scan with an unchanged log skipped, and the agents list read on its own thread every 5 s
@task_file n=16 path=src/console/view.rs
what: the console's view, the drawn fleet lines cut to the columns and rows with the more line and its shown and hidden counts
@task_file n=17 path=src/human/fleet.rs
what: the console's fleet text, the board rows with a seventh LAST cell, the age form in one unit with unknown and n/a, and the line cut ending in dots never inside an escape
@task_item n=1 part=touches
decision/tests-never-read-host-sessions: the console gained a background agents read; every viola console e2e start, old and new, now sets PATH to an empty test-scoped bin dir so no test reads the host's claude sessions.
@task_item n=2 part=touches
decision/fake-agent-on-test-path, decision/agents-stub-from-files-no-env, decision/test-started-viola-drops-env, decision/test-viola-drops-session-vars: the console tests' child environment now carries a test-scoped PATH on every start.
@task_item n=3 part=touches
decision/no-real-cli-in-ci, decision/real-claude-local-only, decision/ci-never-real-claude: viola console now runs claude agents --json through PATH every 5 s; tests and CI reach no real claude.
@task_item n=4 part=touches
decision/agents-json-fallback-unknown, decision/agents-parser-fails-to-unknown: in the console a failed agents read hands an empty list, so STATUS reads unknown and unwrapped rows drop, never a stale busy.
@task_item n=5 part=touches
decision/human-output-escapes-control, decision/human-output-control-escaping, decision/list-row-control-escaping, decision/list-rows-escape-control-chars, decision/list-names-escaped: every console fleet cell is row-escaped (control fleet-escape), and the cut never splits an escape.
@task_item n=6 part=touches
contract/list-human/columns, contract/list-human/unwrapped-separator: the human board rows are rebuilt on a shared forest-row builder, their lines unchanged; the console draws the six captions then a seventh LAST.
@task_item n=7 part=touches
contract/viola-list-json/items, contract/viola-list-json/live-instance, contract/list-json/name, contract/list-json/skipped-counts, decision/cli-mcp-identical-payloads: the board item gains a last-activity time that no list --json item and no MCP list item carries.
@task_item n=8 part=touches
decision/list-reads-live-through-replay, decision/snapshot-read-through-replay, decision/board-on-the-read-rule, decision/list-board-refusal-opens-no-log, decision/replay-writes-no-file: the board read is rebuilt on a per-instance read, a relive that opens no log and an assemble, keeping its line order.
@task_item n=9 part=touches
contract/strict-modes-refusal/console, decision/widened-home-tree-refused-at-start, decision/home-read-rule: the console now also refuses a home widened mid-run with exit 1, the home pair and one process-exit strict-modes-failed; an unlistable board directory ends with internal error.
@task_item n=10 part=touches
decision/tailing-ignores-symlinks, contract/event-tailing/ignored-entries, decision/unknown-events-counted-skipped: the console writes the skip line only on a change of the scan's counts while viola list still writes it per scan; its change stamp follows no link.
@task_item n=11 part=touches
decision/diagnostics-schema-conformance, decision/diag-line-schema-requires-corr, decision/jsonschema-validator: the diagnostics schema's console-frame arm gains required shown and hidden; every console line passes the schema.
@task_item n=12 part=touches
contract/log-event-corr/state-recovered, contract/log-event-corr/parse-rejected, contract/log-event-corr/process-exit: the console process newly writes state-recovered per replay, parse-rejected for state-entry on a count change, and process-exit strict-modes-failed.
@task_item n=13 part=touches
decision/waits-key-on-events, decision/no-sleep-synchronization, decision/no-screen-scraping-in-e2e, decision/tui-verdicts-from-records: the new console e2e cases wait on logged console-frame shown counts and the agents-probe exit, not on screen text.
@task_item n=14 part=touches
decision/injected-clock, decision/test-clocks-behind-clock-seam, decision/time-as-argument, decision/utc-now-read-directly: the console's board re-read interval and clock are injected sources so unit cases run at zero interval; the age cell takes its now.
@task_item n=15 part=touches
decision/human-layouts-pinned-by-literal, decision/human-output-pinned-by-literals, decision/human-output-as-literals: the console body rows and the board's rows stay pinned by literal cases; a scaffold literal moved to the body-row form.
@task_item n=16 part=touches
decision/console-restore-read-on-own-terminal, decision/key-after-resize-waits-for-size: the console loop now polls the board between keys and ends on a refused or failed read after restoring the terminal.
@task_item n=17 part=touches
decision/one-negative-per-security-control, decision/controls-not-disableable, decision/bounded-remove-the-guard-witness: three new controls fleet-cut, fleet-escape and view-redraw were fired and hold.
@task_item n=18 part=touches
decision/bounded-readers, decision/bounded-readers-keep-the-cap: the console's agents read is bounded by the frame cap and the 5 s deadline of the existing agents read.
@task_item n=19 part=touches
decision/live-failing-reading-not-verification, decision/no-human-in-loop-verification, decision/window-placement-proven-live, decision/fake-agent-not-proof-of-real-cli: the console view is proven over fakes only; its live half (matrix v2-14) is not fired and stays GATED.
@task_item n=20 part=touches
decision/no-mocks-in-e2e, decision/external-processes-real-stubs: the console's agents read in e2e meets an empty PATH, so the read fails and the fleet shows unknown, no mock in between.
@task_fact n=1 slug=test-strategy
fact: A verb that gains a background read of the claude agents list must set a test-scoped PATH on every existing test start of that verb, not only its new cases, since the inherited PATH would reach the host's claude.
@task_fact n=2 slug=e2e-test-strategy
fact: The fleet console's view is proven only over fakes; a tiled window usable at a quarter screen, the fleet kept current on the real home, the cost of the real agents read every 5 s and real log sizes against the one-second re-read are proven only by the gated live reading v2-14.
@task_fact n=3 slug=unit-test-strategy
fact: The console's re-read interval is injected through its sources so unit cases run it at zero, and a board read once relived matches the full board read.
@task_fact n=4 slug=e2e-test-strategy
fact: The board item's last-activity time is never serialised, held by a unit case and by the list and cross-list e2e entries reading no such key.
@task_item n=1 part=untouched
The harness contract, its status document shape, session record, boot readiness, cleanup and exit codes: unchanged.
@task_item n=2 part=untouched
CI workflows, supply-chain gates, cargo-deny bans, MSRV, toolchain pins and artifact retention: no workflow or dependency changed.
@task_item n=3 part=untouched
Coverage floors, mutation gates, perf budgets and hook deadlines: no threshold, floor or budget moved.
@task_item n=4 part=untouched
The hook verbs, dialog decision bodies, send, answer, wait, release, last and revive contracts: not reached.
@task_item n=5 part=untouched
The fake agent command line, receipts, recorded fixtures and fixture hygiene: no fake-agent mode or fixture changed.
@task_item n=6 part=untouched
The wrapper channel, JSON-RPC errors, MCP tool set and MCP invalid-params fields: no IPC method, endpoint, socket or env var added.
@task_item n=7 part=untouched
The event line format, event data keys and the closed log event enums: no new event; only console-frame's fields grew.
