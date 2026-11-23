@task_prose part=session
The session added a lineage registry: viola run takes a declared parent and a role, and every run and revive start records them as a new log-only lineage event, the sixteenth event kind, written after the wheel start line and before the hook's session-start.
The instance snapshot and its replay carry the lineage, and revive keeps the recorded parent and role. A reused name started without a parent has none.
viola list now draws the wrapped rows as a tree: a child sits under its parent, its NAME cell indented two spaces per depth, and siblings keep the board's own name order. The six column captions are unchanged.
viola list prints a separate -- LINKS -- block after the board and the unwrapped rack, one driver -> driven line per link, with no colour. list --json and the MCP list document gain optional parent and role on an item and an optional links list in ok. The change is additive and the document version is unchanged.
The test harness wrapper spawn drops the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, so a harness session records no launching session as its parent. A start now writes three records instead of two. A new contract test checks that no wheel, link, send or answer path reads the lineage.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: edited; the shared core: the normalised event kinds list (gains lineage, 16 kinds, not a wake kind), the code-word check and the instance name type
@task_file n=2 path=crates/viola-state/src/snapshot.rs
what: edited; the instance snapshot: gains an optional lineage with parent and role
@task_file n=3 path=crates/viola-state/src/replay.rs
what: edited; the snapshot replay from the events log: a lineage line replaces the held lineage, now the replay's fourth recovered field
@task_file n=4 path=crates/viola-state/src/board.rs
what: edited; the board read model behind viola list and MCP list: items gain name-checked parent and role, and the board gains its sorted, deduplicated, name-checked links
@task_file n=5 path=crates/viola-state/src/liveness.rs
what: edited; the instance liveness read: one snapshot literal gains the lineage field
@task_file n=6 path=crates/viola-state/src/strict.rs
what: edited; the state-tree strict-modes check: one snapshot literal gains the lineage field
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: edited; the event-log tests: the lineage kind and its data keys
@task_file n=8 path=crates/viola-state/tests/state_replay.rs
what: edited; the replay tests: a snapshot cut short is replayed with the logged lineage
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs
what: edited; the MCP handler tests: one board item literal gains the new fields
@task_file n=10 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the test harness boot readiness: reads three start records, the wheel start, the lineage line and the session-start
@task_file n=11 path=crates/viola-e2e/src/harness/supervise.rs
what: edited; the test harness supervisor: its wrapper spawn drops the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN
@task_file n=12 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited; the harness lifecycle test: a harness item reads no parent
@task_file n=13 path=src/cmd/run.rs
what: edited; the viola run verb: the --parent and --role flags, the self-parent usage error, the inherited-name parent, and the start order that writes the lineage line
@task_file n=14 path=src/cmd/revive.rs
what: edited; the viola revive verb: keeps the recorded parent and role on the revived start
@task_file n=15 path=src/cmd/list.rs
what: edited; the viola list verb and its --json document: items carry parent and role, and ok carries links when non-empty
@task_file n=16 path=src/cmd/hook.rs
what: edited; the hook verb: test literals for the third start record
@task_file n=17 path=src/human/board/rows.rs
what: edited; the viola list board rows: the tree drawn under parents with the NAME indent, and the -- LINKS -- block after the unwrapped rack
@task_file n=18 path=src/human/board/cells.rs
what: edited; the viola list board cells: a board line takes the NAME indent
@task_file n=19 path=src/human/board/test_support.rs
what: edited; the board test support: item builders with parent and role
@task_file n=20 path=src/run/mod.rs
what: edited; the wrapped run: one snapshot literal gains the lineage field
@task_file n=21 path=src/run/wheel.rs
what: edited; the wrapped run's wheel: one snapshot literal gains the lineage field
@task_file n=22 path=src/run/dialog.rs
what: edited; the wrapped run's dialog detection: one snapshot literal gains the lineage field
@task_file n=23 path=src/run/send/slot.rs
what: edited; the wrapped run's input slot: one snapshot literal gains the lineage field
@task_file n=24 path=tests/cli_list.rs
what: edited; the viola list integration tests: a child drawn under its parent piped, parent and role in --json, a send link shown apart from the parent edge, a reused name with no parent
@task_file n=25 path=tests/cli_revive.rs
what: edited; the viola revive integration tests: revive keeps the recorded parent and role
@task_file n=26 path=tests/cli_links.rs
what: edited; the link and send integration tests: lineage changes no wheel, link or send
@task_file n=27 path=tests/cli_instance_state.rs
what: edited; the instance state integration tests: the start writes the lineage line before the spawn
@task_file n=28 path=tests/chaos_snapshot_replay.rs
what: edited; the snapshot replay chaos tests: the replay reads the lineage the wrapper wrote
@task_file n=29 path=tests/hook_events.rs
what: edited; the hook event integration tests: start-record counts move from two to three
@task_file n=30 path=tests/path1_run_start.rs
what: edited; the run start path tests: the session-start is now record three
@task_file n=31 path=tests/hook_fail_open.rs
what: edited; the hook fail-open tests: the start-record count
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: edited; the server identity security negatives: the start-record count
@task_file n=33 path=tests/support/home.rs
what: edited; the integration test support: a wrapper booted with flags, and a second wrapper in the same stamped home
@task_file n=34 path=tests/contract_lineage_label.rs
what: added; the contract test: no wheel, link, send or answer path reads the lineage
@task_item n=1 part=touches
viola list has six fixed columns and the viola list output columns: the six captions and their slot order are unchanged, but the NAME cell now carries the tree indent of two spaces per depth.
@task_item n=2 part=touches
The viola list table row order and the stable session slots: wrapped rows are drawn depth-first under their parents. Siblings and roots keep the board's own name order. A row whose parent has no wrapped row is a root, and a cycle is drawn once, starting from its first member in order.
@task_item n=3 part=touches
No auto-sorting of strips: the tree regroups children under their parents. It does not sort by state or activity, and no name sort was added inside the drawing.
@task_item n=4 part=touches
The viola list board NAME column width: the NAME column is now the widest cell with its indent included.
@task_item n=5 part=touches
The viola list board NAME cut and the rule that only an unwrapped NAME is truncated: a wrapped name indented past the width prints whole and moves the other columns right.
@task_item n=6 part=touches
The viola list board unwrapped separator and the board's structural equivalents: a second separator, -- LINKS --, now follows the board and the unwrapped rack in the same single write.
@task_item n=7 part=touches
The CLI transfer arrow, the CLI glyphs arrow -> and the CLI separators: each LINKS line is the driver name, then ->, then the driven name.
@task_item n=8 part=touches
The viola list board colour spans and stale rows in SGR dim: a stale child keeps its dim span over its indent. The LINKS lines carry no SGR, so the uncoloured lines rule gains them.
@task_item n=9 part=touches
The viola list board row escaping and the rule that list rows escape every control character: the parent, the role and both link names are shown only when they parse as instance names or code words, and a parent equal to the row's own name is dropped.
@task_item n=10 part=touches
The viola list board empty board and the plain output modes: with no link, no LINKS block is printed. Piped output equals terminal output less SGR, the tree included.
@task_item n=11 part=touches
The --json result is still one stdout document: the list document's items gain optional parent and role keys, and ok gains links when non-empty. The change is additive and the version is unchanged. MCP list carries the same document.
@task_item n=12 part=touches
CLI exit code 2: a --parent equal to the instance's own name is a usage error with exit 2, raised before anything is read.
@task_item n=13 part=touches
The run command name and the flat CLI verbs: run gains the --parent NAME and --role WORD flags, and no verb is added. A role must pass the code-word check, the same word check that peer reply screening uses.
@task_item n=14 part=touches
viola run silent while wrapped, and the viola run and revive output after the child starts: the lineage line goes to the events log only, with no terminal byte and no process-log line.
@task_item n=15 part=touches
viola list is the board, the fleet console and the one strip per instance: each strip may now sit under a parent strip. The parent edge is a separate relation from a send link.
@task_item n=16 part=touches
The viola revive output: a revived instance keeps the parent and role recorded in its snapshot.
@task_item n=17 part=touches
Unwrapped sessions are never a target, and bare strips: links are drawn only between names that parse, and a gone instance contributes no link. Unwrapped rows stay in their rack, outside the tree.
@task_item n=18 part=touches
No hardcoded widths or wrapping, and lines never wrapped: the tree indent is a fixed two spaces per depth. A deep indent widens the line and never wraps it.
@task_fact n=1 slug=surface-cli
fact: viola run takes --parent NAME and --role WORD; a role must pass the code-word check; a parent equal to the instance's own name is a usage error with exit 2; without the flag, the inherited session name is the parent, dropped when it equals the instance's own name.
@task_fact n=2 slug=surface-cli
fact: viola list draws wrapped rows as a tree, each child under its parent with its NAME cell indented two spaces per depth; siblings keep the board's name order; a row whose parent has no wrapped row is a root, and a cycle is drawn once from its first member.
@task_fact n=3 slug=surface
fact: viola list prints a -- LINKS -- block after the board and the unwrapped rack, one uncoloured driver -> driven line per link, sorted and deduplicated, with both names checked; the block is absent when there is no link.
@task_fact n=4 slug=surface-cli
fact: list --json items carry parent and role only when present, and ok carries links only when non-empty; this addition does not move the document version.
@task_fact n=5 slug=brand-identity
fact: a declared parent and role are shown and never acted on: no wheel, link, send or answer path reads them, and the parent edge is drawn apart from send links.
@task_fact n=6 slug=surface-cli
fact: revive keeps the recorded parent and role, and a reused name started without a parent shows none.
@task_item n=1 part=untouched
viola verify output and its refusals: verification steps, stamps and the summary line are unchanged.
@task_item n=2 part=untouched
viola wait output and viola last output: lineage is not a wake kind and adds nothing to either verb's output.
@task_item n=3 part=untouched
Exit 21 causes, their --json causes and the wrapper fault --json detail: no cause, code or detail was added or moved.
@task_item n=4 part=untouched
The colour palette accent, the colour depths and the colour tokens: no colour or token was added; the board's SGR goes through the existing tokens only.
@task_item n=5 part=untouched
viola plugin install output and the plugin-install strict-modes refusal: not reached.
@task_item n=6 part=untouched
The absences of a rendered page, a web surface and a CLI tape, and the self-validation protocol checks: not reached.
