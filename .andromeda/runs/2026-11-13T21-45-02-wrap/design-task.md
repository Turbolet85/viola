@task_prose part=session
The session closed the route-owed rule lines of the rows landing in code and tests; no verb, exit code, message string or colour changed.
Refusal hints: a human-mode send refused unverified-cli (exit 12) now ends with the hint run viola verify for this CLI version; answer's hint table now gives unverified-cli and human-typing by reason, and for not-delivered with control-character, empty-text, input-not-ready, no-prompt-submitted or turn-running the same string send holds for that detail; unknown, a fault, an opaque detail or no detail still get no hint.
verify --record now refuses an email-shaped token in any string or key of every fixture kind, through the existing unclean-fixture refusal and its email code; the test hygiene walk refuses a planted email on every fixture kind.
The strict-modes checks now refuse a symbolic link at the home, instances dir, an instance dir, snapshot.json or stamps.json before the mode verdict, printing the refusal each caller already prints; the private append open refuses a link and narrows to 0600 through the handle.
run's instance name now goes through send's one name parser (refusal unchanged: usage error, exit 2, invalid instance name); hook, dialog and MCP payload types that carry content derive veil Redact in place of Debug, three content-free MCP inputs derive none.
Tests only: the pty resize test sends no key after the resize, the fd test reads the live fd table with no count and no OS split, one shared digit_runs helper, the snapshot-replay guard proven by a control.
@task_file n=1 path=src/human/hints.rs
what: the refusal hint tables for send and answer, the one hint per refusal code and not-delivered detail printed on the hint line in human mode
@task_file n=2 path=src/cmd/send.rs
what: the send verb, its refusal-to-hint mapping and the one instance-name check shared by the verbs
@task_file n=3 path=src/cmd/answer.rs
what: the answer verb, its unable refusal line and hint in human mode
@task_file n=4 path=src/cmd/run.rs
what: the run verb, its instance-name argument and its start refusals
@task_file n=5 path=crates/viola-agent-claude/Cargo.toml
what: the agent adapter crate's dependencies, now veil
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs
what: the fixture scrub behind verify --record, the unclean-fixture refusal and its check codes including email
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the fixture scrub check
@task_file n=8 path=crates/viola-agent-claude/src/hook.rs
what: the hook payload types the hook verb reads, their redacted debug form
@task_file n=9 path=crates/viola-agent-claude/src/dialog.rs
what: the dialog and dialog-response payload types behind DIALOG and answer, their redacted debug form
@task_file n=10 path=crates/viola-mcp/Cargo.toml
what: the mcp crate's dependencies, now veil
@task_file n=11 path=crates/viola-mcp/src/tools.rs
what: the mcp verb's tool input types (send, answer, wait, last, list) and their redacted debug form
@task_file n=12 path=crates/viola-state/src/strict.rs
what: the strict-modes checks over the home tree, instances, instance state, snapshot and stamps that every strict-modes refusal reads
@task_file n=13 path=crates/viola-state/src/fs.rs
what: the private file opens for diagnostics, heartbeat and event logs, the 0600 append open
@task_file n=14 path=Cargo.lock
what: the workspace lockfile
@task_file n=15 path=tests/cross_exit_causes.rs
what: the cross-check of exit causes and refusal hints: no two codes share a hint, no hint names release, every code a code word in its exit range
@task_file n=16 path=tests/cli_answer.rs
what: end-to-end tests of the answer verb's output and refusals
@task_file n=17 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over agents, status-line, claude, script, screen and MCP fixtures
@task_file n=18 path=tests/contract_os_gated_tests.rs
what: the list of OS-gated tests with their gate and reason
@task_file n=19 path=tests/tui_pty_seam.rs
what: the pty seam test, resize reaching the child
@task_file n=20 path=tests/tui_channel_fds.rs
what: the test of the child's inherited file descriptors
@task_file n=21 path=tests/security_negatives_server_identity.rs
what: security negative tests on server identity
@task_file n=22 path=tests/security_negatives_socket_dir.rs
what: security negative tests on the socket directory
@task_file n=23 path=tests/security_negatives_strict_modes.rs
what: security negative tests on strict modes
@task_file n=24 path=tests/support/hygiene.rs
what: the shared hygiene check and its violation classes, now email
@task_file n=25 path=tests/support/mod.rs
what: the shared test support module list
@task_file n=26 path=tests/support/digits.rs
what: the shared test helper digit_runs
@task_item n=1 part=touches
contract/refusal-hints/unverified-cli: the string is unchanged but now printed on both send and answer; a human-mode send refused unverified-cli at exit 12 printed no hint before and now ends with it.
@task_item n=2 part=touches
contract/refusal-hints/not-delivered-control-character, -empty-text, -input-not-ready, -no-prompt-submitted, -turn-running: the same hint string, byte for byte with the instance name filled, now also ends answer's not-delivered refusal for that detail.
@task_item n=3 part=touches
contract/refusal-hints/not-delivered-unknown-dialog and contract/cli-verb-output/answer: answer's not-delivered unknown-dialog refusal keeps its own hint, that dialog is not pending; viola list shows the current DIALOG, now pinned end to end.
@task_item n=4 part=touches
contract/refusal-hints/human-typing: answer now gives it by reason, as send does.
@task_item n=5 part=touches
decision/no-hint-for-unknown and decision/no-hint-for-faults: answer's table gives no hint for unknown, a fault, an opaque detail or no detail; send's unknown maps to no hint.
@task_item n=6 part=touches
decision/no-hint-names-release, decision/no-hint-names-viola-release and the one-hint-per-code rule: the new unverified-cli hint is checked by the shared-hint and no-release checks.
@task_item n=7 part=touches
contract/cli-exit-codes/12 and contract/cli-exit-codes/10-14: the unverified-cli refusal at exit 12 now carries a hint line; codes in 10 to 14 are checked as code words.
@task_item n=8 part=touches
contract/cli-refusal-line/hint, contract/cli-stderr/hint-line, contract/refusal-hints/refusal-line and decision/hints-human-mode-only: more refusals now end with a hint line, human mode only, none under --json.
@task_item n=9 part=touches
contract/viola-verify-refusals/unclean-fixture: verify --record now refuses an email-shaped token in any string or key of every non-screen payload, through the existing unclean-fixture refusal and email code; no new code or message.
@task_item n=10 part=touches
contract/strict-modes-refusal/human-output, contract/strict-modes-refusal/exit-21, contract/strict-modes-refusal/json-document, contract/cli-strict-modes-refusal/instance-state-pair and contract/fixed-refusal-pairs/instance-state: a symbolic link at an instance dir or snapshot now reads as this same refusal; message unchanged.
@task_item n=11 part=touches
contract/fixed-refusal-pairs/home-state, contract/strict-modes/home-under-regular-file, contract/viola-run-start-refusals/home-tree-check, contract/viola-revive-refusals/home-tree-check and contract/viola-verify-refusals/home-tree-check: a link at the home or instances dir is now refused before the mode verdict, same refusal printed.
@task_item n=12 part=touches
contract/viola-verify-refusals/stamps-update and contract/viola-verify-output/stamps-refused: a symlinked stamps.json is now refused by the stamps check, same refusal printed.
@task_item n=13 part=touches
decision/list-refuses-board-on-strict-modes-failure, contract/viola-list-board/strict-modes-refusal, contract/viola-list-board/strict-modes-scan, contract/viola-revive-refusals/strict-modes-no-log, contract/frame-verb-refusals/strict-modes, contract/exit-21-cause/strict-modes-failed and contract/exit-21-json-cause/strict-modes-failed: the strict-modes scan now also fails on a link in place of a checked path.
@task_item n=14 part=touches
contract/cli-exit-codes/2 and name/command/run: run's invalid instance name stays clap's usage error at exit 2, now through send's one name check; no verb name taken or freed.
@task_item n=15 part=touches
contract/hook-verb/stderr, contract/mcp-verb/stdout and decision/silent-catch-sites: the private append open now fails on a link at the path, which is a dropped diagnostics line as before; hook and MCP payload debug forms are redacted; wire bodies and stdout untouched.
@task_item n=16 part=touches
decision/no-internals-in-errors: hook, dialog and MCP payload types that carry content no longer reveal it through their debug form.
@task_fact n=1 slug=surface
fact: answer's not-delivered refusals carry the same hint as send's for the same detail (control-character, empty-text, input-not-ready, no-prompt-submitted, turn-running), and unverified-cli and human-typing by reason; unknown-dialog keeps answer's own hint; an opaque detail or none gets no hint.
@task_fact n=2 slug=surface
fact: a symbolic link in place of the home, instances dir, an instance dir, snapshot.json or stamps.json is refused by the strict-modes check before its mode verdict, printed as the refusal that caller already prints, the link and its target left as found.
@task_item n=1 part=untouched
brand-identity, color-palette, typography, spacing, depth-strategy, border-radius, iconography: no colour, glyph, separator, column, font or expression rule moved.
@task_item n=2 part=untouched
viola list board and table layout rows (columns, row order, NAME width and cut, WHEEL width, reading age, skip counts, colour spans, BAY line): no board output changed.
@task_item n=3 part=untouched
send readback and readback-box mirror rows, wait and last output rows, link and unlink output rows: no readback, wait, last or link line changed.
@task_item n=4 part=untouched
revive --json exit-1 code rows, wrapper fault --json detail rows, peer reply screening rows, exit-21 causes other than strict-modes-failed: no code word added, renamed or retired.
@task_item n=5 part=untouched
CLI colour decision order and colour depth rows, the colour tokens, help verb groups and the name/command rows other than run: no verb added, hidden or renamed, no colour path changed.
@task_item n=6 part=untouched
anti-patterns and self-validation-protocol rows: no spinner, prompt, pager, sorting or width rule reached.
