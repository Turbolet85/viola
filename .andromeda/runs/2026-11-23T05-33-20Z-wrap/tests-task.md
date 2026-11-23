@task_prose part=session
Added a lineage registry: viola run takes --parent NAME and --role WORD, and every run and revive start writes a lineage event (source wrapper, data parent and role) and a lineage field in the instance snapshot.
A start now writes three records before the hook's session-start (wheel start, then lineage, then session-start), where it used to write two, and the snapshot replay recovers four fields (links, agent_session_id, wheel, lineage), where it used to recover three.
viola list draws wrapped rows as a tree (a child under its parent, NAME indented two spaces per depth) and adds a -- LINKS -- block of driver -> driven lines after the board and the unwrapped rack. list --json and MCP list gain item parent and role and ok.links, all additive.
revive keeps the recorded parent and role. The harness supervisor's wrapper spawn now drops the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, so harness instances record no parent.
A new census test binary checks that no wheel, link, send or answer path reads lineage. Six start-record counters in the tests moved from two records to three. Nothing was retired, and no config key, env var or closed word was added.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the normalised event kinds (now 16, with lineage, which is log-only and not a wake kind) and the code-word check
@task_file n=2 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot, which gains an optional lineage field holding parent and role
@task_file n=3 path=crates/viola-state/src/replay.rs
what: the snapshot replay from the event log, which recovers lineage as its fourth field, the last lineage line winning
@task_file n=4 path=crates/viola-state/src/board.rs
what: the list board read model, whose items gain parent and role (name-checked) and which gains a links list that is sorted, deduplicated and name-checked
@task_file n=5 path=crates/viola-state/src/liveness.rs
what: instance liveness classification; one line added, the snapshot's new lineage field in a literal
@task_file n=6 path=crates/viola-state/src/strict.rs
what: strict modes write and read rules; one line added, the snapshot's new lineage field in a literal
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: event log integration tests over the event kinds and their data keys
@task_file n=8 path=crates/viola-state/tests/state_replay.rs
what: snapshot replay integration tests; a cut-short snapshot is replayed with the logged lineage
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs
what: MCP handler tests; one line added, the snapshot's new lineage field in a literal
@task_file n=10 path=crates/viola-e2e/src/harness/boot.rs
what: harness boot readiness, which now reads three start records (wheel, lineage, session-start)
@task_file n=11 path=crates/viola-e2e/src/harness/supervise.rs
what: the harness supervisor, whose wrapper spawn now removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN
@task_file n=12 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: harness session lifecycle test (boot, status, logs, cleanup), which now asserts that no parent is recorded on harness instances
@task_file n=13 path=src/cmd/run.rs
what: viola run start, which takes the --parent and --role flags (a self-parent is a usage error, exit 2) and writes the snapshot, the wheel start line and the lineage line before the child spawn
@task_file n=14 path=src/cmd/revive.rs
what: viola revive, which keeps the recorded parent and role of the real snapshot
@task_file n=15 path=src/cmd/list.rs
what: the viola list --json document and the MCP list board document, which gain item parent and role and ok.links (omitted when empty)
@task_file n=16 path=src/cmd/hook.rs
what: the viola hook verb; its tests count start records and carry the snapshot's new lineage field
@task_file n=17 path=src/human/board/rows.rs
what: the viola list human board rows, now drawn as a tree with an indented NAME cell and followed by the -- LINKS -- block
@task_file n=18 path=src/human/board/cells.rs
what: the viola list human board cells, where the NAME cell line takes an indent
@task_file n=19 path=src/human/board/test_support.rs
what: test support for the human board rows
@task_file n=20 path=src/run/mod.rs
what: the wrapper run loop; one line added, the snapshot's new lineage field in a literal
@task_file n=21 path=src/run/wheel.rs
what: the wheel; one line added, the snapshot's new lineage field in a test literal
@task_file n=22 path=src/run/dialog.rs
what: the hook dialog hold; one line added, the snapshot's new lineage field in a test literal
@task_file n=23 path=src/run/send/slot.rs
what: the driver send slot; one line added, the snapshot's new lineage field in a test literal
@task_file n=24 path=tests/cli_list.rs
what: viola list integration tests (the tree under its parent, the JSON parent and role, the links block apart from the parent edge, a reused name with no parent)
@task_file n=25 path=tests/cli_revive.rs
what: viola revive integration tests; revive keeps the recorded parent and role
@task_file n=26 path=tests/cli_links.rs
what: wrapper link and send integration tests; lineage changes no wheel, link or send
@task_file n=27 path=tests/cli_instance_state.rs
what: instance state tests, where the start writes state before the spawn and a run takes over a gone name; start-record counts moved to three
@task_file n=28 path=tests/chaos_snapshot_replay.rs
what: chaos test of snapshot replay; the replay reads the lineage the wrapper wrote
@task_file n=29 path=tests/hook_events.rs
what: hook event integration tests; start-record counters moved to three
@task_file n=30 path=tests/path1_run_start.rs
what: critical-path run start e2e test; session-start is now record three
@task_file n=31 path=tests/hook_fail_open.rs
what: hook fail-open tests; one line added, the snapshot's new lineage field in a literal
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: server identity security negative tests; one line added, the snapshot's new lineage field in a literal
@task_file n=33 path=tests/support/home.rs
what: shared test helpers for the stamped home and the test wrapper, adding a flagged wrapper boot and a second wrapper beside the first in one home
@task_file n=34 path=tests/contract_lineage_label.rs
what: new e2e-layer census test binary (contract_ prefix) proving no wheel, link, send or answer path reads lineage, with a planted hit that it must catch
@task_item n=1 part=touches
contract/boot-readiness/events is stale. It says event lines 1 and 2; a start now writes three records before session-start (wheel start, lineage, session-start).
@task_item n=2 part=touches
contract/run-start-events/order is stale. The lineage line now comes between the wheel start line and the hook's session-start, on every run and revive start.
@task_item n=3 part=touches
contract/event-log/start-wheel-record still stands, since the wheel line is still the first record of a start, but a lineage line now follows it as record two.
@task_item n=4 part=touches
contract/viola-run-start/child-after-events is reached because the lineage line is now among the events written before the child spawn.
@task_item n=5 part=touches
decision/snapshot-replay-fields is stale. The replay now recovers four fields: links, agent_session_id, wheel and lineage.
@task_item n=6 part=touches
The contract/instance-snapshot rows gain a sibling: the snapshot carries an optional lineage with parent and role, and each absent member is omitted on write.
@task_item n=7 part=touches
contract/viola-list-json/items and contract/list-json/name are stale. An item gains parent and role when present, for up to 10 keys.
@task_item n=8 part=touches
The list --json ok document gains links when non-empty. This touches the rows on the list --json ok document shape and contract/viola-list-json/mirrors-channel, and decision/cli-mcp-identical-payloads holds through the MCP list carrying the same document.
@task_item n=9 part=touches
contract/wait/wake-kinds, contract/channel-wait/wake-kinds and contract/viola-wait/non-waking-records are reached: the new lineage kind is log-only and wakes no wait. The rows stand, and the new non-waking kind belongs beside them.
@task_item n=10 part=touches
contract/event-line/round-trip and the contract/event-data rows (Event data keys) are reached: there are now 16 event kinds, and the lineage kind's data keys are parent and role.
@task_item n=11 part=touches
contract/list-human/columns and contract/list-human/unwrapped-separator are reached. The six captions are unchanged, the NAME cell is indented per tree depth (the column widens to fit), and a -- LINKS -- separator with driver -> driven lines follows the unwrapped rack.
@task_item n=12 part=touches
decision/human-layouts-pinned-by-literal, decision/human-output-pinned-by-literals and decision/cli-outputs-pinned-by-literal are reached because new board tree and links-block literals were added. A board with no parent still prints today's literal.
@task_item n=13 part=touches
decision/stale-row-dim is reached: a stale child row keeps the dim span over its indent.
@task_item n=14 part=touches
decision/list-names-escaped and decision/list-rows-escape-control-chars are reached because the board now prints parent edges as a tree and adds link lines. Link and parent names are re-parsed as viola names before they show.
@task_item n=15 part=touches
decision/test-started-viola-drops-env, decision/test-viola-drops-session-vars and decision/status-strips-bridge-env are reached: the harness supervisor's wrapper spawn now removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN.
@task_item n=16 part=touches
decision/driver-name-self-reported and decision/link-driver-self-reported are reached: the inherited VIOLA_NAME gets a second use, read as a self-reported parent when no --parent flag is given.
@task_item n=17 part=touches
decision/boot-readiness-file-probe and decision/start-lines-read-from-start-offset are reached: readiness and per-start readers now expect three start records.
@task_item n=18 part=touches
decision/e2e-layer-by-binary-prefix and decision/integration-binary-surface-prefix are reached by the new contract_ binary, which is auto-discovered with no feature and runs in the e2e layer.
@task_item n=19 part=touches
The revive contracts (decision/revive-preflight-order and the revive refusal rows) are reached because revive now hands the recorded lineage to the start. A replayed or absent snapshot gives no lineage, and cwd-missing refuses before it matters.
@task_item n=20 part=touches
decision/no-names-in-wrapper-log is reached: lineage writes no process-log line, span or terminal byte, and names appear only in the event log and the snapshot.
@task_item n=21 part=touches
contract/agent-run-status-polled-fields and contract/status-document/list are reached: the list items the harness status polls may now carry parent and role, which harness instances leave absent.
@task_item n=22 part=touches
decision/harness-records-tolerate-unknown-fields and decision/forward-compatible-log-reading are reached because a new event kind and new item keys arrive at older readers.
@task_fact n=1 slug=integration-test-strategy
fact: The lineage event kind (the 16th) carries data parent and role, each a string or null, written with source wrapper. It is log-only and not a wake kind.
@task_fact n=2 slug=6-e2e-test-strategy
fact: viola run takes --parent NAME (a viola name) and --role WORD (a code word). A --parent equal to the instance's own name is a usage error (exit 2) before anything is read.
@task_fact n=3 slug=6-e2e-test-strategy
fact: With no --parent flag, viola run takes the inherited VIOLA_NAME as the parent only (never as the role), and drops it when it equals the instance's own name.
@task_fact n=4 slug=test-scope-summary
fact: viola revive keeps the parent and role recorded in the real snapshot.
@task_fact n=5 slug=test-scope-summary
fact: The replay takes the last lineage line. A later line with no parent clears it, so a reused name started without a parent has none.
@task_fact n=6 slug=e2e-test-strategy
fact: The viola list human board draws each wrapped row under its parent, depth-first, in the board's name order, with NAME indented two spaces per depth. A row whose parent has no wrapped row is a root, and cycle members are drawn once from the first in order.
@task_fact n=7 slug=e2e-test-strategy
fact: The viola list human board prints a -- LINKS -- separator, then one driver -> driven line per link, after the board and the unwrapped rack, with no SGR. There is no block when there are no links.
@task_fact n=8 slug=1-test-scope-summary
fact: list --json and MCP list items carry parent and role only when present. A parent is kept only when it parses as a viola name other than the row's own, and a role only when it passes the code-word check.
@task_fact n=9 slug=1-test-scope-summary
fact: list --json ok.links is the union of every listed live or stale instance's links, sorted by driver then driven, deduplicated, with each pair kept only when both names parse. A gone instance gives none, and the key is omitted when empty.
@task_fact n=10 slug=test-strategy
fact: An e2e census test holds that no wheel, link, send or answer path reads lineage. It is proven to bite by a planted hit.
@task_fact n=11 slug=5-command-implementation
fact: The harness supervisor's wrapper spawn removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, so a harness session booted inside a wrapped session records no parent on its instances.
@task_item n=1 part=untouched
CI integration, the CI artifacts, the supply-chain gates, the pinned tools and workflow permissions are untouched; no workflow or dependency moved.
@task_item n=2 part=untouched
Coverage floors, mutation gates and their kill lines, performance budgets and the pre-push gate contracts are untouched.
@task_item n=3 part=untouched
The fake agent command line, its receipts and screens, recorded fixtures, fixture scrubbing and the hygiene walk are untouched.
@task_item n=4 part=untouched
Mocking and stubbing discipline and the clock seams are untouched.
@task_item n=5 part=untouched
viola verify, the stamps, the ledger probes and the unverified-CLI gate are untouched.
@task_item n=6 part=untouched
The process log format, its line fields and the log-event corr rows are untouched: lineage adds no process-log line.
@task_item n=7 part=untouched
The harness run, gate, secret-scan, schema-check and cleanup document contracts are untouched.
@task_item n=8 part=untouched
The MCP tools list, the tool argument schemas and the invalid-params field rows are untouched; no tool or parameter was added.
@task_item n=9 part=untouched
The CLI exit-code rows for send, answer, release, wait and exit 21 are untouched, as are the channel JSON-RPC error rows.
