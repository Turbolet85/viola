@task_prose part=session
Added a lineage registry: viola run takes --parent <NAME> and --role <WORD>, else the inherited VIOLA_NAME as parent only; a self-parent is a usage error, exit 2, before anything is read.
Every run and revive start now writes three records before the hook's session-start: the wheel start line, a new log-only lineage event (the 16th event kind, waking no wait), then session-start; the snapshot carries lineage and replay recovers it as a fourth field; revive keeps the recorded parent and role.
viola list draws wrapped rows as a tree: each child under its parent, its NAME cell indented 2 spaces per depth, in the board's own name order, an orphan as a root, a cycle drawn once; then after the unwrapped rack a -- LINKS -- separator and one driver -> driven line per link, no SGR, in the same single write, absent with no link.
list --json and MCP list gain item parent and role (only when the parent parses as a name other than the row's own and the role is a code word) and ok.links (sorted, deduplicated, name-checked), additive, no v move; captions stay the six.
The harness supervisor drops the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN on its wrapper spawn; a new e2e census test proves lineage is read by no wheel, link, send or answer path; the operator pass and CI ran green.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: edited — the normalised event-kind list and its per-kind data table, gaining the lineage kind (16 kinds), not a wake kind
@task_file n=2 path=crates/viola-state/src/snapshot.rs
what: edited — the instance snapshot shape, gaining the optional lineage (parent, role)
@task_file n=3 path=crates/viola-state/src/replay.rs
what: edited — the snapshot replay from the events log, recovering lineage as its fourth field (the last lineage line wins)
@task_file n=4 path=crates/viola-state/src/board.rs
what: edited — the board read model behind viola list and MCP list: item parent and role, the board's links list, names re-checked
@task_file n=5 path=crates/viola-state/src/liveness.rs
what: edited — the instance liveness read; its test snapshot literal gains the lineage field
@task_file n=6 path=crates/viola-state/src/strict.rs
what: edited — the home strict-modes check; its test snapshot literal gains the lineage field
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: edited — the event-kind tests, counting 16 kinds with the lineage kind's data keys
@task_file n=8 path=crates/viola-state/tests/state_replay.rs
what: edited — the snapshot replay tests, a cut-short snapshot replayed with the logged lineage
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs
what: edited — the MCP server tool handler tests; a snapshot literal gains the lineage field
@task_file n=10 path=crates/viola-e2e/src/harness/boot.rs
what: edited — the harness boot readiness check, now reading three start records (wheel, lineage, session-start)
@task_file n=11 path=crates/viola-e2e/src/harness/supervise.rs
what: edited — the harness supervisor's wrapper spawn, now dropping the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN
@task_file n=12 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited — the harness lifecycle test, asserting no parent on a harness-booted instance
@task_file n=13 path=src/cmd/run.rs
what: edited — the viola run command: the --parent and --role flags, the self-parent usage error, the start order writing the lineage line before the spawn
@task_file n=14 path=src/cmd/revive.rs
what: edited — the viola revive start, handing the recorded parent and role into the launch
@task_file n=15 path=src/cmd/list.rs
what: edited — the viola list command and its --json document (items parent, role; ok.links)
@task_file n=16 path=src/cmd/hook.rs
what: edited — the viola hook event and dialog calls; its tests' start-record counts gain the lineage line
@task_file n=17 path=src/human/board/rows.rs
what: edited — the viola list board rendering: the tree of wrapped rows, the NAME indent, the -- LINKS -- block after the unwrapped rack
@task_file n=18 path=src/human/board/cells.rs
what: edited — the board line cells, the NAME cell taking a depth indent
@task_file n=19 path=src/human/board/test_support.rs
what: edited — the board test fixtures, items gaining parent and role
@task_file n=20 path=src/run/mod.rs
what: edited — the TUI passthrough run component; a launch literal gains lineage
@task_file n=21 path=src/run/wheel.rs
what: edited — the wheel component; its test snapshot literal gains the lineage field
@task_file n=22 path=src/run/dialog.rs
what: edited — the dialog-pending component; its test snapshot literal gains the lineage field
@task_file n=23 path=src/run/send/slot.rs
what: edited — the send slot component; its test snapshot literal gains the lineage field
@task_file n=24 path=tests/cli_list.rs
what: edited — the viola list integration tests: a child drawn under its parent piped, --json parent and role, the links block apart from the parent edge, a reused name with no parent
@task_file n=25 path=tests/cli_revive.rs
what: edited — the viola revive integration tests, revive keeping the recorded parent and role
@task_file n=26 path=tests/cli_links.rs
what: edited — the link and send integration tests, lineage changing no wheel, link or send
@task_file n=27 path=tests/cli_instance_state.rs
what: edited — the instance state tests, the start writing three records before the spawn
@task_file n=28 path=tests/chaos_snapshot_replay.rs
what: edited — the chaos snapshot replay tests, the replay reading the lineage the wrapper wrote
@task_file n=29 path=tests/hook_events.rs
what: edited — the hook event record tests, start-record counts gaining the lineage line
@task_file n=30 path=tests/path1_run_start.rs
what: edited — the run start path tests, session-start now record three
@task_file n=31 path=tests/hook_fail_open.rs
what: edited — the hook fail-open tests, a start-record count gaining the lineage line
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: edited — the server identity security negative tests; a snapshot literal gains the lineage field
@task_file n=33 path=tests/support/home.rs
what: edited — the integration test support: booting a wrapper with flags, a second stamped home beside the first
@task_file n=34 path=tests/contract_lineage_label.rs
what: added (new) — an e2e census test that lineage is read by no wheel, link, send or answer path, with a planted positive
@task_item n=1 part=touches
contract/viola-list-board/order: the board's structure gains the tree of wrapped rows (children under their parent, in the board's own name order) and the -- LINKS -- block after the unwrapped rack.
@task_item n=2 part=touches
decision/terminal-structural-equivalents: a new structural equivalent, the -- LINKS -- separator beside the unwrapped one, and the NAME indent standing for tree depth.
@task_item n=3 part=touches
contract/viola-list-board/unwrapped-separator and contract/viola-list/unwrapped-separator: the unwrapped rack is no longer the board's last block when a link exists; the links block follows it.
@task_item n=4 part=touches
decision/list-captions-fixed, contract/viola-list-board/caption and contract/viola-list-caption/order: the six captions and their order stay unchanged; the NAME column's width now counts the widest indent.
@task_item n=5 part=touches
decision/name-truncation and decision/unwrapped-name-truncation: a wrapped NAME cell carries a depth indent; an indent past the width prints whole and moves the columns right.
@task_item n=6 part=touches
decision/stale-row-dim and contract/viola-list-stale-row/escape-sequences: a stale child's dim span covers its indent.
@task_item n=7 part=touches
decision/no-sgr-in-plain-modes, decision/no-sgr-off-terminal and decision/no-sgr-without-tty: the links lines carry no SGR in any mode; the piped tree is the terminal tree less SGR.
@task_item n=8 part=touches
decision/name-bold-sgr, decision/sgr-beside-its-word and decision/word-beside-every-sgr: SGR on the tree rows still only through the existing tokens, the indent plain spaces.
@task_item n=9 part=touches
decision/static-linear-output and decision/linear-static-cli-output: the tree and links block are printed linearly in the board's one write.
@task_item n=10 part=touches
decision/control-character-escaping, decision/c0-c1-escaped and decision/control-bytes-escaped: link pairs and parents shown on the board pass the name parse and row escaping.
@task_item n=11 part=touches
decision/printed-names-are-parsed-arguments: board parents and link names are re-parsed as viola names before being printed; a parent equal to its own row is dropped.
@task_item n=12 part=touches
contract/cli-usage-error/exit-2 and contract/cli-exit-codes/2: viola run --parent equal to its own name is a usage error, exit 2, before anything is read.
@task_item n=13 part=touches
contract/code-word/format: viola run --role accepts only a code word.
@task_item n=14 part=touches
contract/cli-help/usage-line and contract/cli-help/group-labels: viola run gains the --parent and --role flags in its help.
@task_item n=15 part=touches
decision/critical-path-cli-board-output: the critical board output now carries the tree and the links block.
@task_item n=16 part=touches
decision/state-carried-by-printed-word, decision/no-information-by-colour-alone and decision/colour-second-cue-only: parentage carried by indent and links by printed lines, never by colour.
@task_item n=17 part=touches
decision/fixed-cli-strings-pinned, decision/fixed-cli-strings-pinned-by-literal and decision/fixed-copy-from-design: a new fixed string, -- LINKS --, and the -> arrow in each link line, pinned by literal tests.
@task_item n=18 part=touches
contract/viola-list-board/empty-board: with no link the links block is absent; a board with no parent prints today's literal.
@task_item n=19 part=touches
decision/zero-viola-bytes-on-the-terminal and contract/tui-passthrough/viola-terminal-bytes: the new lineage start line writes no terminal byte.
@task_item n=20 part=touches
decision/run-diagnostics-off-terminal and contract/tui-passthrough/run-diagnostics: the lineage start writes no process-log line, span or terminal byte.
@task_item n=21 part=touches
decision/boundary-tests-sequenced-on-records: a start now writes three records (wheel, lineage, session-start), session-start moving from record two to three.
@task_item n=22 part=touches
decision/human-key-takes-the-wheel, decision/wheel-ignores-terminal-events and decision/send-refused-while-human-typing: indirect, lineage is read by no wheel, link, send or answer path, held by a new census test.
@task_item n=23 part=touches
decision/link-pair-validated-by-wrapper: the board's links list keeps a pair only when both names parse, sorted and deduplicated, a gone instance giving none.
@task_item n=24 part=touches
decision/a11y-inside-existing-suites: the new board a11y cases (piped equals terminal less SGR, dim over the indent) sit in the existing unit and integration suites.
@task_fact n=1 slug=a11y-scope-summary
fact: viola list board prints, after the unwrapped rack, a -- LINKS -- separator then one driver -> driven line per link, with no SGR, in the board's same single write; absent when no link exists.
@task_fact n=2 slug=a11y-scope-summary
fact: viola list board draws wrapped rows as a tree in the board's own name order: each child under its parent with its NAME cell indented 2 spaces per depth, a row whose parent has no wrapped row as a root, a cycle drawn once from its first name.
@task_fact n=3 slug=1-a11y-scope-summary
fact: Tree depth on the board is carried by a plain-space NAME indent and links by a -- LINKS -- separated block, as the terminal structural equivalents of a tree and a related-items group.
@task_fact n=4 slug=visual-design-verification
fact: The NAME column is as wide as its widest cell indent included; an indent past the width prints whole and moves the columns right, and a stale child's dim span covers its indent.
@task_fact n=5 slug=cognitive-accessibility
fact: viola run --parent equal to the instance's own name is a usage error, exit 2, before anything is read; --role accepts only a code word.
@task_fact n=6 slug=cognitive-accessibility
fact: The board shows a parent only when it parses as a viola name other than the row's own, a role only when it is a code word, and a link pair only when both names parse.
@task_item n=1 part=untouched
The viola verify step line and interrupted pair, and the viola wait waiting and timed-out lines: no change to their output.
@task_item n=2 part=untouched
viola plugin install human and --json output: unchanged.
@task_item n=3 part=untouched
CI integration, gate, SLO and flakiness rows: no gate, threshold, suite value or artifact moved; the new test binary runs inside the existing e2e layer.
@task_item n=4 part=untouched
Screen-reader targets and the manual keyboard and Orca passes: no change to their setup or standing.
@task_item n=5 part=untouched
The absences (internationalization, touch surface, web or graphical frontend, rendered page, user time limit): still absent.
@task_item n=6 part=untouched
Structured violation JSON rows: no change to the failure document shape.
@task_item n=7 part=untouched
The unable and hint refusal layout, reason codes, fault bodies and --json failure documents: no reason code, hint or refusal document added.
