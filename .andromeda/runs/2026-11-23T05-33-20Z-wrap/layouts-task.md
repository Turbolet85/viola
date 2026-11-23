@task_prose part=session
The session added a lineage registry: each wrapped instance may declare a parent and a role at launch, kept in its state and drawn on the board.
viola run gained the --parent and --role flags; without --parent the inherited own-session name stands as the parent; a parent equal to the instance's own name is a usage error, exit 2, before anything is read.
Every run and revive start now writes a lineage event after the wheel start line, so a start writes three records before the hook's session-start (was two); the event kinds went from 15 to 16 and the replay recovers a fourth field, the lineage.
viola list now draws wrapped rows as a tree, each child under its parent indented two spaces per depth in the NAME cell, and prints a separate -- LINKS -- block of driver -> driven lines after the board and the unwrapped rack.
list --json and the MCP list tool carry optional parent and role per item and an optional links list in ok, additively with no version move; revive keeps the recorded parent and role.
The test harness now drops the inherited viola session names when it spawns a wrapper, so harness sessions record no parent.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the shared core types: the normalised event kinds and their data keys, the wake kinds, the code-word check
@task_file n=2 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot, the per-instance state record, which gains the optional lineage (parent, role)
@task_file n=3 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the events log, which now recovers the lineage as its fourth field
@task_file n=4 path=crates/viola-state/src/board.rs
what: the board read model behind viola list and the MCP list tool: its items gain parent and role and the board gains the name-checked link list
@task_file n=5 path=crates/viola-state/src/liveness.rs
what: the instance liveness check; a test snapshot literal gains the lineage field
@task_file n=6 path=crates/viola-state/src/strict.rs
what: the strict-modes check on the state files; a test snapshot literal gains the lineage field
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: the events-log tests over the event kinds, now sixteen
@task_file n=8 path=crates/viola-state/tests/state_replay.rs
what: the snapshot replay tests, including a cut-short snapshot replayed with the logged lineage
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs
what: the MCP tool handler tests; a snapshot literal gains the lineage field
@task_file n=10 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e harness boot readiness, which reads the three start records
@task_file n=11 path=crates/viola-e2e/src/harness/supervise.rs
what: the e2e harness supervisor's wrapper spawn, which now drops the inherited viola session names
@task_file n=12 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the harness lifecycle test, asserting no parent on a harness session's items
@task_file n=13 path=src/cmd/run.rs
what: the viola run verb: its flags, the start order of state, wheel and lineage records before the spawn
@task_file n=14 path=src/cmd/revive.rs
what: the viola revive verb, which hands the recorded lineage into its start
@task_file n=15 path=src/cmd/list.rs
what: the viola list verb and its --json document, shared with the MCP list tool
@task_file n=16 path=src/cmd/hook.rs
what: the hook entry that records hook events; its tests count the start records
@task_file n=17 path=src/human/board/rows.rs
what: the viola list board's row drawing: the tree order, the NAME indent, the rack and the new LINKS block in one write
@task_file n=18 path=src/human/board/cells.rs
what: the viola list board's cell layout of a row line, which gains the NAME indent
@task_file n=19 path=src/human/board/test_support.rs
what: test helpers for the viola list board
@task_file n=20 path=src/run/mod.rs
what: the wrapper's run loop; a test snapshot literal gains the lineage field
@task_file n=21 path=src/run/wheel.rs
what: the wrapper's wheel state; a test snapshot literal gains the lineage field
@task_file n=22 path=src/run/dialog.rs
what: the wrapper's dialog handling; a test snapshot literal gains the lineage field
@task_file n=23 path=src/run/send/slot.rs
what: the wrapper's PTY send slot; a test snapshot literal gains the lineage field
@task_file n=24 path=tests/cli_list.rs
what: the viola list integration tests: the tree piped, parent and role in --json, the links block apart from the parent edge, a reused name with no parent
@task_file n=25 path=tests/cli_revive.rs
what: the viola revive integration tests, now that revive keeps the recorded parent and role
@task_file n=26 path=tests/cli_links.rs
what: the link and send integration tests, now proving lineage changes no wheel, link or send
@task_file n=27 path=tests/cli_instance_state.rs
what: the instance state integration tests over the start records and a taken-over name
@task_file n=28 path=tests/chaos_snapshot_replay.rs
what: the chaos tests of the snapshot replay, which reads the lineage the wrapper wrote
@task_file n=29 path=tests/hook_events.rs
what: the hook events integration tests counting the start records
@task_file n=30 path=tests/path1_run_start.rs
what: the run start path tests over the record order before session-start
@task_file n=31 path=tests/hook_fail_open.rs
what: the hook fail-open tests; one start-record count moved
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: the server identity security negatives; one start-record count moved
@task_file n=33 path=tests/support/home.rs
what: the shared test home and wrapper helpers: a flagged wrapper boot and a second wrapper beside in one home
@task_file n=34 path=tests/contract_lineage_label.rs
what: a new contract test: a census that no wheel, link, send or answer path reads the lineage
@task_item n=1 part=touches
decision/no-link-column: still no link column, but the links now show on the human board as a separate -- LINKS -- block after the board and the unwrapped rack, one driver -> driven line per link.
@task_item n=2 part=touches
decision/board-ordered-by-name: wrapped rows are now a tree, each child drawn under its parent depth-first; order within each parent and among roots stays the board's own name order.
@task_item n=3 part=touches
contract/viola-list/columns and decision/shared-captions-fixed: the six captions stay unchanged, but the NAME cell now carries the tree indent, two spaces per depth.
@task_item n=4 part=touches
contract/viola-list-board/column-widths: the NAME column is now as wide as its widest cell with the indent included.
@task_item n=5 part=touches
contract/viola-list-board/name-truncation and decision/truncate-only-unwrapped-names: a wrapped name indented past the width prints whole and moves the columns right; no wrapped name is cut.
@task_item n=6 part=touches
contract/viola-list-board/single-write: the LINKS block is written in the same single write, after the board and the rack.
@task_item n=7 part=touches
contract/viola-list-board/unwrapped-rack and contract/viola-list/unwrapped-separator: a second separator line, -- LINKS --, now follows the rack in the same plain separator form.
@task_item n=8 part=touches
contract/viola-list-board/plain-lines, contract/viola-list-board/uncoloured-modes and decision/viola-list-styling: the LINKS lines carry no SGR; the tree piped equals the terminal less SGR.
@task_item n=9 part=touches
contract/viola-list-board/stale-row-dim: a stale child row keeps its dim span over its indent.
@task_item n=10 part=touches
contract/viola-list-board/empty-board: with no link the LINKS block is absent; a board with no parent draws today's literal.
@task_item n=11 part=touches
decision/only-header-bay-line: the -- LINKS -- separator is a new non-row line on the board, beside the rack separator, not a header.
@task_item n=12 part=touches
decision/cli-field-separators: the LINKS lines use the ascii arrow -> between driver and driven.
@task_item n=13 part=touches
decision/board-row-escaping: parent names and link pairs are re-parsed as names and dropped when invalid; the board rows still pass through the row escaping.
@task_item n=14 part=touches
contract/viola-list-json/document: each item may now carry parent and role, present only when valid, and ok may carry links, omitted when empty; additive, no version move; the item keys go up to ten.
@task_item n=15 part=touches
contract/mcp-tools/list: the MCP list tool carries the same parent, role and links unchanged from the list document.
@task_item n=16 part=touches
contract/cli-verbs/run and contract/viola-verb-help/usage: viola run gains --parent NAME and --role WORD, so its usage and per-verb help lines move.
@task_item n=17 part=touches
contract/cli-verbs/list: viola list now draws the tree and the LINKS block.
@task_item n=18 part=touches
contract/cli-verbs/revive and contract/viola-revive/start-output: revive restarts with the recorded parent and role; its printed start output is unchanged.
@task_item n=19 part=touches
contract/viola-run/start-output: the run start writes the lineage record with no process-log line and no terminal byte; the printed start output stays as it is.
@task_item n=20 part=touches
decision/targets-are-wrapped-names and decision/violaname-never-case-transformed: --parent takes a ViolaName through the same name parser as a target; the role must be a code word.
@task_item n=21 part=touches
decision/clap-derive-cli-parsing: the self-parent check runs right after the clap parse and is raised as a clap argument-conflict error, exit 2.
@task_item n=22 part=touches
decision/human-output-is-contract and decision/words-carry-every-state: the board's tree indent and the LINKS lines are new human output under the contract; no state is carried by colour alone.
@task_fact n=1 slug=surface-cli
fact: viola run takes --parent NAME and --role WORD; the parent is a ViolaName and the role a code word; a parent equal to the instance's own name is a usage error, exit 2, before anything is read.
@task_fact n=2 slug=surface-cli
fact: Without --parent, the inherited own-session name stands as the parent, dropped when it equals the instance's own name; the role is never inherited.
@task_fact n=3 slug=surface-cli
fact: The viola list board draws wrapped rows as a tree: each child under its parent, its NAME cell indented two spaces per depth; a row whose parent has no wrapped row is a root; cycle members are drawn once, from the first in order.
@task_fact n=4 slug=surface-cli
fact: The viola list board ends with a -- LINKS -- block after the board and the unwrapped rack, one driver -> driven line per link, sorted, deduplicated, name-checked, no SGR, absent when there is no link.
@task_fact n=5 slug=surface-cli
fact: The board shows a parent only when it parses as a ViolaName other than the row's own, and a role only when it is a code word; otherwise the item shows neither.
@task_fact n=6 slug=surface-cli
fact: viola revive keeps the parent and role recorded for the instance; a reused name started without a parent has none.
@task_fact n=7 slug=surface-cli
fact: The lineage is display-only: no wheel, link, send or answer path reads it, held by a contract census test.
@task_item n=1 part=untouched
The web surface absence and the deep-link absence: no web page or URL coupling was added.
@task_item n=2 part=untouched
The CLI token rendering rows (colours, roles, radius, spacing, motion): no token was added; the board styles only through the existing tokens.
@task_item n=3 part=untouched
viola verify output and its refusals: verify was not changed.
@task_item n=4 part=untouched
viola plugin install output, refusals and json documents: not changed.
@task_item n=5 part=untouched
viola wait, viola last and viola answer outputs and hints: unchanged.
@task_item n=6 part=untouched
The frame-verb exit-21 hints and the exit-21 json detail rows: no unreachable cause or hint was added.
@task_item n=7 part=untouched
The readback mirror and readback vocabulary rows: the send readback is unchanged.
@task_item n=8 part=untouched
viola link and unlink output rows: the link verbs print as before; links are only also listed on the board.
@task_item n=9 part=untouched
The spinner, banner, footer, prompt, pager and progress-glyph absences: none was added.
