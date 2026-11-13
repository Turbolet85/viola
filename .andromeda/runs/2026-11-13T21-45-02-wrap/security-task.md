@task_prose part=session
The session closed in code or tests the rule lines the rows landing left owed by the route, and changed no wire body, schema, config key or env var.
The strict-modes check now reads the path itself and refuses a symbolic link at the home, instances, an instance dir, the snapshot or the stamps file as Symlink before the mode verdict, leaving link and target as found.
The private append open (diagnostics, events, heartbeat) opens no-follow, so a link at the path is a failed open, and sets mode 0600 through the open handle.
verify --record's scrub now refuses an email-shaped token in any string or key of every payload kind, through the existing unclean-fixture refusal and email code; the fixture hygiene test walk refuses email on every kind too.
run takes its instance name through the one shared name parser; send refused unverified-cli and answer's not-delivered details now end with a hint.
The content-bearing hook, dialog and MCP input payload types derive veil Redact in place of Debug (veil pinned exactly, added to two crates); three content-free MCP inputs derive none, since veil refuses a derive with no field to redact.
@task_file n=1 path=src/human/hints.rs
what: the human-mode refusal hint tables for send and answer, the line that ends a refusal on stderr
@task_file n=2 path=src/cmd/send.rs
what: the viola send verb, home of the one instance name parser and the send refusal-to-hint mapping
@task_file n=3 path=src/cmd/answer.rs
what: the viola answer verb and its human-mode refusal output
@task_file n=4 path=src/cmd/run.rs
what: the viola run verb's arguments, its instance name taken through the shared name check
@task_file n=5 path=crates/viola-agent-claude/Cargo.toml
what: the agent crate's dependency list, a sync crate under the tokio ban, now with veil
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs
what: the fixture recording scrub and refusal behind verify --record, the unclean-fixture check over every recorded payload
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the fixture scrub's refusal checks
@task_file n=8 path=crates/viola-agent-claude/src/hook.rs
what: the hook payload types, normalised and known hook events carrying prompt and assistant text as content
@task_file n=9 path=crates/viola-agent-claude/src/dialog.rs
what: the dialog payload and dialog response types carrying the human's answers and free text
@task_file n=10 path=crates/viola-mcp/Cargo.toml
what: the MCP server crate's dependency list, now with veil
@task_file n=11 path=crates/viola-mcp/src/tools.rs
what: the MCP server's tool input types for send, answer, wait, last and list
@task_file n=12 path=crates/viola-state/src/strict.rs
what: the strict-modes check on trusted state, the home tree, instances, snapshot and stamps read rule and its refusals
@task_file n=13 path=crates/viola-state/src/fs.rs
what: the private file helpers, the owner-only append open for diagnostics, events and heartbeat files and the lock open
@task_file n=14 path=Cargo.lock
what: the root dependency graph lockfile that cargo-deny reads
@task_file n=15 path=tests/cross_exit_causes.rs
what: the test that every refusal code has its own hint, names no release and exits in its band
@task_file n=16 path=tests/cli_answer.rs
what: integration tests of viola answer's refusals and their hints
@task_file n=17 path=tests/contract_fixture_hygiene.rs
what: the recorded fixture hygiene walk over agents, statusline, claude, script, screen and MCP fixtures
@task_file n=18 path=tests/contract_os_gated_tests.rs
what: the list of OS-gated tests with each gate and its reason
@task_file n=19 path=tests/tui_pty_seam.rs
what: the PTY seam test on a real openpty, the resize reaching the child
@task_file n=20 path=tests/tui_channel_fds.rs
what: the test that the child inherits no channel handle, reading the live child's fd table
@task_file n=21 path=tests/security_negatives_server_identity.rs
what: security negative tests of the client's server verification and its refusals naming no identity values
@task_file n=22 path=tests/security_negatives_socket_dir.rs
what: security negative tests of the socket-directory check
@task_file n=23 path=tests/security_negatives_strict_modes.rs
what: security negative tests of the strict-modes refusal
@task_file n=24 path=tests/support/digits.rs
what: the shared test helper reading whole digit runs out of refusal text
@task_file n=25 path=tests/support/hygiene.rs
what: the shared fixture hygiene check and its violation classes, now with email
@task_file n=26 path=tests/support/mod.rs
what: the shared test support module list
@task_item n=1 part=touches
decision/strict-modes-check: the check now refuses a symbolic link at the path itself before the mode verdict; its owed route divergence closes.
@task_item n=2 part=touches
decision/cli-names-through-name-check: run's own name parser is gone, run takes its name through the one shared parser; one name parser in the product; its owed divergence closes.
@task_item n=3 part=touches
decision/fixture-scrub-and-refusal and decision/fixture-recording-scrubbed: the record scrub refuses an email-shaped token in any string or key on every payload kind, refused whole not stripped; the owed divergence closes.
@task_item n=4 part=touches
contract/fixture-refusal-codes/email and contract/verify-refusal-codes/unclean-fixture: the email code now fires on every non-screen kind too; no new code or line form.
@task_item n=5 part=touches
contract/strict-refusal-message/symlink and the strict-modes-refusal contracts: Symlink is now the refusal for a link at the home, instances, an instance dir, the snapshot or the stamps file on every caller; the messages do not change.
@task_item n=6 part=touches
Indirect, rows reading trusted state through the strict check: snapshot, stamps and home-tree rows (strict-modes-before-snapshot-read, snapshot-strict-checked-before-trust, stamps-strict-check-first, version-gate-strict-stamps-read, home-strict-modes-first, state-tree-strict-modes-before-use) now refuse a link there.
@task_item n=7 part=touches
decision/strict-refusal-leaves-no-trace: new tests pin a link refusal leaving the link and its target as found.
@task_item n=8 part=touches
Socket-directory check rows: a test that read the strict check following a link now pins check_read and check_path refusing the link itself.
@task_item n=9 part=touches
decision/private-diagnostics-files (both twins): the append open now refuses a link at the path (no-follow) and sets 0600 through the open handle, not by a path chmod.
@task_item n=10 part=touches
Owner-only mode rows (private-file-modes, owner-only-file-modes, explicit-mode-before-write, diagnostics-owner-only, logs-owner-only): the append file's mode is now narrowed through the handle.
@task_item n=11 part=touches
decision/diagnostics-open-fails-to-sink and decision/diagnostics-strict-modes-fail-open: a link at a diagnostics path is now a failed open, which falls to the existing sink.
@task_item n=12 part=touches
decision/fixture-hygiene: the test hygiene walk refuses a planted email on every fixture kind as class email; MCP fixtures go through the same check.
@task_item n=13 part=touches
Content-in-logs rows (hook-payload-is-content, user-content-only-to-diagnostics, upstream-text-is-content): hook, dialog and MCP send and answer input types now derive veil Redact, so Debug holds no content.
@task_item n=14 part=touches
Dependency rows (exact-dependency-pins, exact-third-party-pins, cargo-deny gate, sync-crate tokio ban): veil at the exact workspace pin =0.3.0, no toggle, enters the agent crate (a sync crate) and the MCP crate; deny and the bans read green.
@task_item n=15 part=touches
decision/unverified-cli-gate and contract/cli-exit-codes/12: a human-mode send refused unverified-cli now ends with the hint run viola verify for this CLI version.
@task_item n=16 part=touches
contract/send-refusal and contract/answer-refusal and channel-refusal items: answer's not-delivered refusals for control-character, empty-text, input-not-ready, no-prompt-submitted, turn-running now carry the same hint send holds; unknown-dialog its own.
@task_fact n=1 slug=data-protection
fact: The owner-only append open of diagnostics, events and heartbeat files does not follow a symbolic link: a link at the path is a failed open, and the 0600 mode is set through the open handle.
@task_fact n=2 slug=data-protection
fact: Every hook, dialog and MCP input payload type that carries content derives veil Redact so Debug never shows that content; a type with no content field derives none.
@task_item n=1 part=untouched
Channel frames, frame caps, depth limits and JSON-RPC error codes: no wire body changed.
@task_item n=2 part=untouched
Peer euid checks, server verification order, socket bind, socket chmod and abstract-namespace rows: no check order or bind changed.
@task_item n=3 part=untouched
Secret management, credential strip, child environment and the canary scan: untouched.
@task_item n=4 part=untouched
CI workflows, action pinning, zizmor, release build, fuzzing and toolchain pins: no script, CI step or tool version changed.
@task_item n=5 part=untouched
rmcp feature set and banned features, and the network, hosting, account and database absences: untouched.
@task_item n=6 part=untouched
revive codes and documents, link and unlink verbs, collision check, pinned binary and plugin install rows: untouched.
@task_item n=7 part=untouched
Config keys, budget file and statusline rows: no config key, env var or budget reader changed.
