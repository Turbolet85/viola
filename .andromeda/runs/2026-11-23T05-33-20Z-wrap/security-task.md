@task_prose part=session
The session added a lineage registry: viola run takes --parent NAME and --role WORD, else reads the inherited VIOLA_NAME as the parent only, and a parent equal to the instance's own name is a usage error, exit 2, before anything is read.
At every run and revive start the wrapper writes the lineage into the snapshot and one new lineage event line (source wrapper, data parent and role) after the wheel start line; the event kinds go from 15 to 16, lineage is log-only and wakes no wait, and no process-log line, span or terminal byte is added.
The replay recovers a fourth field, the lineage, where the last lineage line wins; revive keeps the recorded parent and role from the real snapshot, and a replayed or absent snapshot gives none.
viola list draws wrapped rows as a tree under their parents (NAME indented two spaces a depth) and a separate -- LINKS -- block of driver -> driven lines; list --json and the MCP list tool gain item parent and role and ok.links, all additive with no v move; the board keeps a parent only when it re-parses as a viola name other than the row's own, a role only when it is a code word, and a link pair only when both names parse.
A census test asserts lineage is read by no wheel, link, send or answer path; the test harness's wrapper spawn now removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN so a harness session records no launching session as parent.
No dependency, config key, env var, closed word or channel method was added, and nothing was retired.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the core crate's shared validators and the closed list of normalised event kinds (gains the lineage kind, 16 kinds, not a wake kind)
@task_file n=2 path=crates/viola-state/src/snapshot.rs
what: the instance snapshot the wrapper writes (gains an optional lineage of parent and role)
@task_file n=3 path=crates/viola-state/src/replay.rs
what: the replay that rebuilds a replayed snapshot from the event log's own fields (gains the lineage line, last wins)
@task_file n=4 path=crates/viola-state/src/board.rs
what: the board read model behind viola list and the MCP list tool (gains item parent and role and the name-checked link pairs)
@task_file n=5 path=crates/viola-state/src/liveness.rs
what: the instance liveness read (a test snapshot literal gained the lineage field)
@task_file n=6 path=crates/viola-state/src/strict.rs
what: the strict-modes check on trusted state (a test snapshot literal gained the lineage field)
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: the event log tests (the lineage kind in the kind table)
@task_file n=8 path=crates/viola-state/tests/state_replay.rs
what: the replay tests (a snapshot cut short is replayed with the logged lineage)
@task_file n=9 path=crates/viola-mcp/tests/handlers.rs
what: the MCP tool handler tests (a board literal gained the new field)
@task_file n=10 path=crates/viola-e2e/src/harness/boot.rs
what: the e2e test harness's boot readiness read (now reads three start records)
@task_file n=11 path=crates/viola-e2e/src/harness/supervise.rs
what: the e2e test harness's supervisor that spawns test wrappers (now removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN)
@task_file n=12 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: the e2e harness lifecycle test (asserts no parent on harness items)
@task_file n=13 path=src/cmd/run.rs
what: the viola run command, its CLI input checks and its start order (gains the --parent and --role flags, the self-parent usage error and the lineage start line)
@task_file n=14 path=src/cmd/revive.rs
what: the viola revive command (keeps the recorded parent and role from the real snapshot)
@task_file n=15 path=src/cmd/list.rs
what: the viola list command and its --json document, also served by the MCP list tool (gains parent, role and ok.links)
@task_file n=16 path=src/cmd/hook.rs
what: the viola hook command (test snapshot literals gained the lineage field)
@task_file n=17 path=src/human/board/rows.rs
what: the human-mode board rows of viola list (the tree under parents and the -- LINKS -- block, printed through the control-character escaping)
@task_file n=18 path=src/human/board/cells.rs
what: the human-mode board cells (the NAME cell gains an indent)
@task_file n=19 path=src/human/board/test_support.rs
what: test support for the human-mode board (items gain parent and role)
@task_file n=20 path=src/run/mod.rs
what: the wrapper's run loop (a test snapshot literal gained the lineage field)
@task_file n=21 path=src/run/wheel.rs
what: the wrapper's wheel (a test snapshot literal gained the lineage field)
@task_file n=22 path=src/run/dialog.rs
what: the wrapper's dialog handling (a test snapshot literal gained the lineage field)
@task_file n=23 path=src/run/send/slot.rs
what: the wrapper's send slot (a test snapshot literal gained the lineage field)
@task_file n=24 path=tests/cli_list.rs
what: viola list integration tests (tree, json parent and role, links block, a reused name has no parent)
@task_file n=25 path=tests/cli_revive.rs
what: viola revive integration tests (revive keeps the recorded parent and role)
@task_file n=26 path=tests/cli_links.rs
what: link integration tests (lineage changes no wheel, link or send)
@task_file n=27 path=tests/cli_instance_state.rs
what: instance state integration tests (the start writes state before the spawn, now with the lineage line)
@task_file n=28 path=tests/chaos_snapshot_replay.rs
what: chaos tests of the snapshot replay (the replay reads the lineage the wrapper wrote)
@task_file n=29 path=tests/hook_events.rs
what: hook event integration tests (start record counts moved to three)
@task_file n=30 path=tests/path1_run_start.rs
what: run start integration tests (session-start is now record three)
@task_file n=31 path=tests/hook_fail_open.rs
what: the hook fail-open tests (a start record count moved)
@task_file n=32 path=tests/security_negatives_server_identity.rs
what: the server-identity security negative tests (a start record count moved)
@task_file n=33 path=tests/support/home.rs
what: test support for stamped test homes and wrappers (boot with flags, two wrappers in one home)
@task_file n=34 path=tests/contract_lineage_label.rs
what: a new contract test, the census that lineage is read by no wheel, link, send or answer path
@task_item n=1 part=touches
decision/cli-home-resolution-order: the inherited VIOLA_NAME gains a second use, read by run as the self-reported parent when no --parent flag is given, dropped when equal to the instance's own name.
@task_item n=2 part=touches
decision/from-field-authenticates-nothing and decision/channel-from-not-identity: the declared or inherited parent is one more self-reported name that authenticates nothing and grants no authority.
@task_item n=3 part=touches
decision/pause-release-from-viola-name: VIOLA_NAME is now read by run as well, as the parent only.
@task_item n=4 part=touches
decision/parent-identity-env-stripped: the parent identity variables a wrapped session hands on are now read as lineage, and the test harness's wrapper spawn strips VIOLA_NAME, VIOLA_DIR and VIOLA_BIN.
@task_item n=5 part=touches
decision/names-through-validated-type and decision/cli-names-through-name-check: --parent passes the viola name parser and --role the code-word check; the board re-parses stored parents and link names.
@task_item n=6 part=touches
decision/cli-input-checked-before-frame and decision/cli-input-trust-boundary: two new run flags cross the CLI boundary, the self-parent refused with exit 2 before anything is read.
@task_item n=7 part=touches
decision/replayed-snapshot-fallbacks, decision/replayed-snapshot-inert and decision/replayed-snapshot-yields-nothing: the replay recovers a fourth field (lineage), and revive takes no lineage from a replayed snapshot.
@task_item n=8 part=touches
decision/snapshot-written-only-by-wrapper and decision/snapshot-strict-checked-before-trust: the snapshot gains a lineage the wrapper writes and revive reads back.
@task_item n=9 part=touches
decision/link-restore-trusts-written-names: the board now reads the link sets it lists and keeps a pair only when both names parse.
@task_item n=10 part=touches
contract/mcp-tools/list and decision/data-classes: the MCP list tool and list --json now carry parent, role and the union of link pairs, widening logged names to MCP list.
@task_item n=11 part=touches
decision/human-output-escapes-controls and decision/escape-terminal-controls: viola list prints new text, the indented tree and the -- LINKS -- driver -> driven lines.
@task_item n=12 part=touches
decision/link-names-kept-out-of-process-log and decision/never-log-claude-values: the lineage line goes to the event log only, with no process-log line or span.
@task_item n=13 part=touches
decision/whole-board-refusal, decision/list-home-read-rule and decision/board-home-read-rule: the board scan now reads each listed instance's lineage and link set under the same home read rule.
@task_item n=14 part=touches
decision/new-env-read-needs-log-entry: a new read of the inherited VIOLA_NAME (as parent) entered the run path.
@task_item n=15 part=touches
decision/events-reader-bounded-lines and decision/readers-tolerate-torn-lines: the replay reads one more event kind, a non-string member read as absent.
@task_item n=16 part=touches
decision/tests-use-own-home and decision/hermetic-child-environment: the test harness's wrapper spawn now drops the inherited VIOLA_* names so a test session never records the launching session.
@task_item n=17 part=touches
decision/unwrapped-name-never-target: a declared parent is shown only and is never a send, link or answer target, held by a census test.
@task_fact n=1 slug=input-validation
fact: viola run's --parent is parsed as a viola name and --role as a code word, and a --parent equal to the instance's own name is a usage error, exit 2, before anything is read.
@task_fact n=2 slug=authentication-authorization
fact: The parent is self-reported (the --parent flag, else the inherited VIOLA_NAME, dropped when it is the instance's own name) and authenticates nothing; lineage is read by no wheel, link, send or answer path, a census test holding it.
@task_fact n=3 slug=input-validation
fact: The board shows a recorded parent only when it re-parses as a viola name other than the row's own, a role only when it is a code word, and a link pair only when both names parse; a gone instance gives no links.
@task_fact n=4 slug=data-protection
fact: The lineage (parent and role names) is kept only in the snapshot and one lineage line of the event log at start, with no process-log line, span or terminal byte.
@task_fact n=5 slug=data-protection
fact: viola list --json and the MCP list tool now carry each item's parent and role and the listed instances' link pairs, so logged instance names reach the MCP list result.
@task_fact n=6 slug=input-validation
fact: The replay takes the last lineage line as the lineage, a non-string member read as absent, and revive keeps the parent and role only from a real snapshot, never from a replayed one.
@task_fact n=7 slug=secret-management
fact: The test harness's wrapper spawn removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, so a harness session started inside a wrapped session records no parent.
@task_item n=1 part=untouched
dependency-security: no dependency, toolchain, pin, licence, advisory or CI workflow changed.
@task_item n=2 part=untouched
api-security: the rmcp feature set is unchanged.
@task_item n=3 part=untouched
Socket placement, socket modes, peer euid checks and server verification before a frame: no channel method or listener was added.
@task_item n=4 part=untouched
The CLAUDE credential strip, the env-kept list and the artifact canary scan: no credential or CLAUDE variable is read or kept.
@task_item n=5 part=untouched
The pinned binary, plugin install, verify, its stamps, probes and fixture hygiene rows.
@task_item n=6 part=untouched
MAX_FRAME, the MCP stdin line cap and the channel frame cap: no reader bound moved.
@task_item n=7 part=untouched
The absences (network, hosting, accounts, secrets, databases, release stage): no such capability was added.
