@task_prose part=session
The session added a lineage registry: each wrapped instance records a declared parent and a role, and the fleet board draws instances as a tree and shows links apart from that tree.
viola run gains --parent NAME and --role WORD. When no flag is given, the inherited VIOLA_NAME becomes the parent, and is dropped when it equals the instance's own name. A --parent equal to the name is a usage error, exit 2, raised before anything is read.
Every run and revive start now writes the snapshot with its lineage, then the wheel start line, then one new lineage event line (source wrapper, data parent and role), and only then spawns the child. A start now has three records before the hook's session-start instead of two.
A new 16th normalised event kind, lineage, is log-only and wakes no wait. Snapshot replay recovers a fourth field, lineage, and the last lineage line wins. Revive keeps the recorded parent and role.
The board read model and list --json, with MCP list carrying it unchanged, gain an optional parent and role per item and an ok.links list of driver, driven and since. These are additive, with no v move. The human board draws a tree in the NAME cell, followed by a -- LINKS -- block.
The e2e harness supervisor's wrapper spawn now drops the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN. Nothing was retired, and no dependency, config key, env var or closed word was added.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: edited — viola-core's list of normalised event kinds (the kinds, their wire names and data keys, the full kind set and the wait wake-kind set); gains the lineage kind, which is not a wake kind
@task_file n=2 path=crates/viola-state/src/snapshot.rs
what: edited — the instance snapshot shape (the snapshot data fields); gains an optional lineage with an optional parent and role
@task_file n=3 path=crates/viola-state/src/replay.rs
what: edited — snapshot replay, which recovers the log-derived fields from the events log; gains lineage as its fourth recovered field, the last lineage line winning
@task_file n=4 path=crates/viola-state/src/board.rs
what: edited — the board read model behind CLI list and MCP list (sync, Claude-free); items gain parent and role, and the board gains a sorted, deduplicated, name-checked links list
@task_file n=5 path=crates/viola-state/src/liveness.rs
what: edited — session liveness; one test snapshot literal gains the lineage field
@task_file n=6 path=crates/viola-state/src/strict.rs
what: edited — the strict-modes read rule; one test snapshot literal gains the lineage field
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: edited — the state crate's event-log tests over the event kinds; they count sixteen kinds
@task_file n=8 path=crates/viola-state/tests/state_replay.rs
what: edited — the snapshot replay tests; a snapshot cut short is replayed with the logged lineage
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs
what: edited — the MCP server handler tests; one snapshot literal gains the lineage field
@task_file n=10 path=crates/viola-e2e/src/harness/boot.rs
what: edited — the e2e harness boot readiness check over the start records; it now reads three lines (wheel, lineage, session-start)
@task_file n=11 path=crates/viola-e2e/src/harness/supervise.rs
what: edited — the e2e harness supervisor's wrapper spawn and the environment it hands the wrapper; it drops the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN
@task_file n=12 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited — the harness lifecycle test (boot, report, logs, teardown); asserts that no parent is recorded on harness instances
@task_file n=13 path=src/cmd/run.rs
what: edited — viola run: its CLI arguments and the start path shared with revive (snapshot, then event lines, then the child spawn last); gains --parent and --role, the self-parent usage error and the lineage start line
@task_file n=14 path=src/cmd/revive.rs
what: edited — viola revive, which resumes a gone instance in place through the start path; it hands the recorded lineage to the start
@task_file n=15 path=src/cmd/list.rs
what: edited — the viola list --json document (the list payload envelope, items, ok, skipped); gains ok.links, omitted when empty
@task_file n=16 path=src/cmd/hook.rs
what: edited — viola hook; four test snapshot literals gain the lineage field
@task_file n=17 path=src/human/board/rows.rs
what: edited — the human board of viola list (its rows, the unwrapped rack, the single write); rows are drawn as a tree under their parents, followed by a -- LINKS -- block
@task_file n=18 path=src/human/board/cells.rs
what: edited — the human board's cell line; the NAME cell takes an indent
@task_file n=19 path=src/human/board/test_support.rs
what: edited — the human board's test support items; they gain parent and role
@task_file n=20 path=src/run/mod.rs
what: edited — the wrapper's run module; one test snapshot literal gains the lineage field
@task_file n=21 path=src/run/wheel.rs
what: edited — the wrapper's wheel; one test snapshot literal gains the lineage field, with no wheel behaviour change
@task_file n=22 path=src/run/dialog.rs
what: edited — the wrapper's dialog hold; one test snapshot literal gains the lineage field, with no dialog behaviour change
@task_file n=23 path=src/run/send/slot.rs
what: edited — the wrapper's send slot; one test snapshot literal gains the lineage field, with no send behaviour change
@task_file n=24 path=tests/cli_list.rs
what: edited — the viola list CLI tests; they gain the tree, the JSON parent and role, the links block and a reused name without a parent
@task_file n=25 path=tests/cli_revive.rs
what: edited — the viola revive CLI tests; revive keeps the recorded parent and role
@task_file n=26 path=tests/cli_links.rs
what: edited — the link and unlink CLI tests; they show that lineage changes no wheel, link or send result
@task_file n=27 path=tests/cli_instance_state.rs
what: edited — the instance state tests (state written before the spawn, a gone name taken over); they count the lineage start line
@task_file n=28 path=tests/chaos_snapshot_replay.rs
what: edited — the chaos snapshot replay tests; the replay reads the lineage the wrapper wrote
@task_file n=29 path=tests/hook_events.rs
what: edited — the hook event-log tests; they count three start records before session-start
@task_file n=30 path=tests/path1_run_start.rs
what: edited — the run start path tests; session-start is now record three, after the lineage line
@task_file n=31 path=tests/hook_fail_open.rs
what: edited — the hook fail-open tests; one snapshot literal gains the lineage field
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: edited — the server identity security negatives; one snapshot literal gains the lineage field
@task_file n=33 path=tests/support/home.rs
what: edited — the root tests' stamped test home and wrapper boot support; adds a flagged boot and a second wrapper in the same home
@task_file n=34 path=tests/contract_lineage_label.rs
what: added (new file) — a contract test, a source census, asserting that lineage is read by no wheel, link, send or answer path, with a planted positive
@task_item n=1 part=touches
The normalised event kinds list (the per-kind contract rows) gains a 16th kind, lineage, with data keys parent and role. Each of the fifteen standing per-kind rows still holds. Any row that holds the kind count of 15 is now 16.
@task_item n=2 part=touches
decision/wrapper-events-log-only and the wrapper event appends rows: the wrapper now appends a lineage line (source wrapper) at each start, and it is log-only.
@task_item n=3 part=touches
contract/channel-wait/wake-kinds, contract/channel-methods/wait-wake-kinds and the channel wait driver-relevant events: lineage is not in the wake set. The rows stay true as written but are read against the new kind.
@task_item n=4 part=touches
The event data per-kind rows and contract/event-line/source: a new per-kind data entry for lineage. parent and role are each a string or null, and the source is wrapper.
@task_item n=5 part=touches
contract/instance-snapshot/data: the snapshot data gains an optional lineage with an optional parent and role, each omitted on write when absent (decision/absent-optional-fields-omitted, decision/missing-equals-null-on-read apply).
@task_item n=6 part=touches
decision/snapshot-replay-log-fields-only and decision/snapshots-read-through-replay: the replay's recovered fields move from 3 to 4 (links, agent session id, wheel, plus lineage). A lineage line replaces the held value, and no other kind changes it.
@task_item n=7 part=touches
decision/run-start-order and contract/instance-snapshot/spawn-order: the start order is now snapshot with lineage, then the wheel start line, then the lineage line, then the child spawn. A start's records before session-start move from 2 to 3.
@task_item n=8 part=touches
decision/revive-in-place and decision/revive-resumes-in-place: revive hands the real snapshot's recorded lineage to the start path. A replayed or absent snapshot gives none and is refused for a missing cwd anyway.
@task_item n=9 part=touches
contract/list-payload/items and the list payload item rows: an item gains parent and role, present only when the parent parses as a name other than the item's own and the role is a code word. An item now has up to 10 keys.
@task_item n=10 part=touches
contract/list-payload/ok and contract/list-json/skipped: ok gains links (driver, driven, since), omitted when empty. The change is additive with no v move (decision/protocol-version-bump-rule, decision/versioned-records-no-migrations).
@task_item n=11 part=touches
decision/board-read-model-shape and the board rows (CLI list and MCP list): the board gains a links list, the union of every listed live, stale or replayed-live instance's link set, sorted, deduplicated and name-checked. A gone instance gives none.
@task_item n=12 part=touches
contract/mcp-tools/list and decision/list-reads-state-directly: MCP list now carries parent, role and links unchanged from the board. This widens which logged names reach MCP list, an escalation the wrap carries as a delegate's provisional word.
@task_item n=13 part=touches
CLI verbs and arguments for run (name/command/run, decision/cli-flag-and-subcommand-style): viola run gains --parent NAME and --role WORD. A self-parent is a usage error, exit 2 (contract/cli-exit-codes/2).
@task_item n=14 part=touches
contract/child-environment/viola-name, name/other/viola-name and decision/viola-env-namespace-product-only: the inherited VIOLA_NAME gains a second use. run reads it as a self-reported parent when no --parent is given.
@task_item n=15 part=touches
decision/viola-home-resolution and decision/session-home-from-viola-dir (indirect): the harness supervisor's spawn now drops the inherited VIOLA_DIR, VIOLA_NAME and VIOLA_BIN. A harness session booted inside a wrapped session no longer inherits them.
@task_item n=16 part=touches
contract/code-word/format and decision/viola-name-instance-key: role is validated as a code word, and parent and link pairs are re-parsed as instance names on the board.
@task_item n=17 part=touches
decision/terminal-fleet-console and decision/single-human-writer (indirect, through the human board): wrapped rows are drawn as a tree with the NAME cell indented 2 per depth. A -- LINKS -- block follows the unwrapped rack in the same single write, with no SGR.
@task_item n=18 part=touches
decision/link-recording, decision/links-are-log-events and decision/link-set-changes-only-on-link-calls: the links are now also shown on the board. A declared parent is no link and moves no link set. The rows hold but are read against the parent edge.
@task_item n=19 part=touches
decision/event-kinds-agent-neutral: the new kind is added in viola-core only, which conforms to the rule. Read it for any count of kinds it holds.
@task_fact n=1 slug=conventions
fact: The normalised event kind lineage has wire name lineage and data keys parent and role, each a string or null. It is appended by the wrapper (source wrapper), is log-only, and is not a wait wake kind; it is the 16th kind.
@task_fact n=2 slug=standard-contracts
fact: The instance snapshot data holds an optional lineage with an optional parent and role. Snapshot replay recovers it from the log, the last lineage line replacing the held value.
@task_fact n=3 slug=standard-contracts
fact: A list payload item carries an optional parent and role. The parent is kept only when it parses as an instance name other than the item's own, and the role only when it is a code word.
@task_fact n=4 slug=standard-contracts
fact: The list payload ok carries an optional links list of driver, driven and since. It is the union of the listed instances' link sets, sorted by driver then driven, deduplicated, kept only when both names parse, and omitted when empty. CLI list --json and MCP list carry it alike, with no v move.
@task_fact n=5 slug=occupied-resources
fact: viola run takes --parent NAME and --role WORD. A --parent equal to the instance's own name is a usage error, exit 2, raised before anything is read. With no --parent, the inherited VIOLA_NAME is taken as parent only, and is dropped when it equals the instance's own name.
@task_fact n=6 slug=established-decisions
fact: Each run and revive start writes one lineage event line after the wheel start line and before the child spawn. It writes no process-log line, no span and no terminal byte.
@task_fact n=7 slug=established-decisions
fact: Revive keeps the parent and role recorded in the instance's real snapshot.
@task_fact n=8 slug=established-decisions
fact: Lineage is a label only. No wheel, link, send or answer path reads it, and a source census contract test guards this.
@task_fact n=9 slug=established-decisions
fact: The human list board draws wrapped rows depth-first under their declared parents in the board's own name order. A row whose parent has no wrapped row is a root, and cycle members are drawn once. The NAME cell is indented two spaces per depth while the six captions stay unchanged. After the board and the unwrapped rack, a -- LINKS -- separator and one driver -> driven line per link follow, with no SGR.
@task_fact n=10 slug=occupied-resources
fact: The e2e harness supervisor removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN from the wrapper it spawns. Harness instances therefore never record a launching session as their parent.
@task_item n=1 part=untouched
Stack, dependency and toolchain rows (cargo-deny, MSRV, rmcp features, crate naming, the crate dependency direction): no dependency, crate or toolchain was added or moved.
@task_item n=2 part=untouched
CI/CD, build-system and deployment rows (workflows, coverage floors, pre-push gate, release, fuzz, mutation): no gate, threshold or workflow moved.
@task_item n=3 part=untouched
Wrapper channel frame, JSON-RPC method and fault-code rows: no channel method, frame field or fault code was added, and lineage travels on no frame.
@task_item n=4 part=untouched
The hook to kind map, hook tiers and hook contract rows: lineage is written by the wrapper at start, never produced by any hook.
@task_item n=5 part=untouched
Capability ledger, verify, ledger stamps and pinned-copy rows: no ledger row, stamp or verify step changed.
@task_item n=6 part=untouched
PTY seam, vt100 screen model, readiness gate and screen signature rows: no screen reading changed.
@task_item n=7 part=untouched
config.json keys, colour environment and Hyprland seam rows: no config key, colour rule or compositor call was added.
@task_item n=8 part=untouched
Absences (daemon, database, broker, graphical or mobile frontend, network port, hosting): the tree and links block are terminal text read from disk, so no absence is broken.
