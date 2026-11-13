@task_prose part=session
Closed the rows landing's route-owed rule lines in code or tests; no capability retired, no harness, CI or gate surface changed.
Refusal hints: human-mode send refused unverified-cli (exit 12) now ends with a hint; answer's not-delivered details carry the same hint send holds for that detail, unknown-dialog its own; unknown and opaque details get none.
Fixture scrub and hygiene: verify --record refuses an email-shaped token in every payload kind, and the test hygiene walk gains the email class for every walk (agents, statusline, claude payloads, scripts, MCP exchanges), with planted cases per kind.
Trusted state: the strict-modes check refuses a symbolic link at the path before the mode verdict, and the private append open refuses a link and narrows the mode through the open handle.
Tests reshaped: the resize test sends no key after the resize; the fd test reads the live child's fd table always, with no fd count and no Linux split; the snapshot-replay link pass is proven guarded by an existing test; digit_runs is one shared test helper (3 copies to 1); run's instance-name check is the send one (2 copies to 1).
Hook and MCP payload types that carry content derive veil Redact so Debug shows no content; content-free MCP inputs derive none because veil 0.3.0 refuses that derive; veil joins two sync crates and the cargo-deny and per-sync-crate tokio bans stay green.
@task_file n=1 path=src/human/hints.rs
what: the human-mode refusal hint tables for send and answer, one hint per exit cause, none naming release
@task_file n=2 path=src/cmd/send.rs
what: the viola send verb, its instance-name check and its human hint cause mapping
@task_file n=3 path=src/cmd/answer.rs
what: the viola answer verb and its human-mode refusal lines
@task_file n=4 path=src/cmd/run.rs
what: the viola run start verb and its instance-name argument check
@task_file n=5 path=crates/viola-agent-claude/Cargo.toml
what: the agent adapter crate manifest, a sync crate under the per-sync-crate tokio ban
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs
what: the verify recorder scrub of recorded fixtures, refusing a dirty payload as unclean-fixture
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the recorder scrub checks
@task_file n=8 path=crates/viola-agent-claude/src/hook.rs
what: the hook stdin payload types and their normalisation
@task_file n=9 path=crates/viola-agent-claude/src/dialog.rs
what: the hook dialog and dialog answer payload types
@task_file n=10 path=crates/viola-mcp/Cargo.toml
what: the MCP server crate manifest
@task_file n=11 path=crates/viola-mcp/src/tools.rs
what: the MCP tool input types and handlers
@task_file n=12 path=crates/viola-state/src/strict.rs
what: the strict-modes write and read rules over the home tree, instances, snapshot and stamps, and the socket directory check
@task_file n=13 path=crates/viola-state/src/fs.rs
what: the private file opens for diagnostics, heartbeat and event log appends (owner-only 0600)
@task_file n=14 path=Cargo.lock
what: the workspace dependency lockfile
@task_file n=15 path=tests/cross_exit_causes.rs
what: the cross-surface exit cause table check that each cause has its own hint and no hint names release
@task_file n=16 path=tests/cli_answer.rs
what: viola answer CLI integration tests run as a subprocess
@task_file n=17 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene run-time walk over recorded fixtures with planted violation inputs
@task_file n=18 path=tests/contract_os_gated_tests.rs
what: the registry of OS-gated tests, each with its gate and reason
@task_file n=19 path=tests/tui_pty_seam.rs
what: the PTY passthrough test on a real openpty, including the resize case
@task_file n=20 path=tests/tui_channel_fds.rs
what: the fd test reading the live child's fd table for inherited descriptors
@task_file n=21 path=tests/security_negatives_server_identity.rs
what: security negative tests for the server identity check
@task_file n=22 path=tests/security_negatives_socket_dir.rs
what: security negative tests for the socket directory check
@task_file n=23 path=tests/security_negatives_strict_modes.rs
what: security negative tests for the strict-modes refusals
@task_file n=24 path=tests/support/hygiene.rs
what: the shared hygiene checker reporting violation class only
@task_file n=25 path=tests/support/mod.rs
what: the shared test helper module set
@task_file n=26 path=tests/support/digits.rs
what: the shared digit_runs test helper, its one copy
@task_item n=1 part=touches
decision/snapshot-replay-from-log: its guard field gains the cli_links test e5_the_replay_reads_the_link_the_wrapper_wrote, proven the guard of the replay's link pass by a remove-the-guard control.
@task_item n=2 part=touches
decision/one-shared-test-helper: the shared helper set now names digit_runs (3 copies to 1); digit_runs_reads_whole_numbers_only stays in the server-identity negatives alone.
@task_item n=3 part=touches
decision/pty-tested-on-real-openpty and decision/key-after-resize-waits-for-size: the resize test writes no key after the resize and waits on the size receipt alone; the owed divergence closes.
@task_item n=4 part=touches
decision/fd-test-reads-live-child-table and decision/no-fd-count-assertion: the fd test waits on the start receipt, reads the live fd table always, drops the fds receipt asserts, the fd count bound of 5 and the Linux-only split.
@task_item n=5 part=touches
decision/os-gated-tests-registered and decision/os-gated-tests-each-named: the fd test's registered gate is unix, its reason now inherited file descriptors are Unix's.
@task_item n=6 part=touches
contract/fake-agent-receipt/fds and contract/fake-agent-receipt/start: the fd test now waits on start and no longer reads fds; the fds receipt kind stays, read by the fake-agent tests.
@task_item n=7 part=touches
decision/fixture-hygiene-walk and decision/fixtures-walked-for-hygiene: every walk (agents, statusline, claude payloads, scripts, MCP exchanges) refuses an email-shaped string or key as class email; the MCP walk now runs the shared check with its schema.
@task_item n=8 part=touches
decision/planted-hygiene-violations and decision/hygiene-reports-class-only: planted email cases per kind added (value, nested, key; a.b@localhost passes); the class is reported as email only.
@task_item n=9 part=touches
decision/screen-fixture-scrub-as-detector, decision/recorder-scrub and decision/dirty-payload-refuses-recording: the recorder scrub refuses an email-shaped token in every non-screen payload too; the screen scrub's own email arm folded into the shared one.
@task_item n=10 part=touches
contract/mcp-fixture-scrub-refusal/email, contract/process-exit-detail/unclean-fixture and contract/verify-exit-detail/unclean-fixture: the email refusal now reaches every recorded kind through the existing unclean-fixture detail and email code; no new code.
@task_item n=11 part=touches
decision/hint-names-its-cause, decision/each-exit-cause-own-hint, decision/no-release-in-driver-hints, decision/driver-hints-never-name-release, decision/hints-never-suggest-release: unverified-cli (exit 12) joins the shared-hint and release checks through its own refusal table; exit range checked 10 to 14.
@task_item n=12 part=touches
contract/viola-send-human-mode/unable and contract/cli-human-lines/unable: a human-mode send refused unverified-cli now ends with hint: run viola verify for this CLI version.
@task_item n=13 part=touches
contract/viola-answer/unknown-dialog, contract/viola-answer/unverified-cli and contract/cli-answer/unverified-cli: answer's human-mode refusal for unknown-dialog ends with its own hint; not-delivered details end with send's hint for that detail.
@task_item n=14 part=touches
decision/unverified-cli-send-only, decision/unverified-cli-gates-answer-only, contract/cli-exit-codes/12 and contract/human-hint-cause rows: the human hint cause maps unverified-cli to its reason; unknown maps to none.
@task_item n=15 part=touches
decision/strict-modes-two-rules and decision/strict-modes-before-any-log: the write and read rules refuse a symbolic link at the path itself before the mode verdict, leaving link and target as found; no new refusal variant or message.
@task_item n=16 part=touches
decision/home-read-rule, decision/widened-home-tree-refused-at-start, decision/frame-verbs-refuse-widened-instance and the strict-modes-refusal contract rows: a link in place of the home, instances, an instance dir, snapshot or stamps file now reads the same strict-modes refusal.
@task_item n=17 part=touches
decision/socket-dir-check-leaves-dir-as-found and decision/socket-dir-check-never-chmods: the socket directory unit case now pins the read and write rules refusing a link (was accepted, following the link).
@task_item n=18 part=touches
decision/diagnostics-files-owner-only, decision/diagnostics-files-0600, decision/private-filesystem-permissions and contract/log-files/modes: the private append open refuses a link at the path and narrows to 0600 through the open handle.
@task_item n=19 part=touches
decision/diagnostics-checked-before-narrowing and decision/umask-independent-modes: narrowing now goes through the open handle in place of a path chmod.
@task_item n=20 part=touches
decision/secrets-never-in-logs, decision/no-secrets-in-logs and decision/logs-carry-no-secrets: content-bearing hook and MCP payload types derive Redact so Debug shows no prompt, text or answer; redacted payload types 1 to 7.
@task_item n=21 part=touches
decision/tokio-banned-per-sync-crate, decision/cargo-deny-supply-chain-gate and decision/sync-crates-compile-without-features: veil added to the agent adapter and MCP crates without toggle; the bans and cargo deny stay green.
@task_item n=22 part=touches
Rows on viola run's instance name check: run takes send's name parser; the refusal stays a usage error, exit 2, invalid instance name.
@task_item n=23 part=touches
decision/bounded-remove-the-guard-witness: a remove-the-guard control under nextest fail-fast can leave the guard's own new case unrun while another case carries the red.
@task_fact n=1 slug=unit-test-strategy
fact: The strict-modes write and read rules refuse a symbolic link at the checked path itself (home, instances, an instance dir, snapshot, stamps) before the mode verdict, and leave link and target as found.
@task_fact n=2 slug=6-e2e-test-strategy
fact: The private append open of diagnostics, heartbeat and event files refuses a symbolic link at the path and narrows the mode to 0600 through the open handle, never by path.
@task_fact n=3 slug=e2e-test-strategy
fact: A human-mode answer refused not-delivered ends with the hint send holds for that detail, unknown-dialog ends with its own hint, and unknown, an opaque detail or none ends with no hint.
@task_fact n=4 slug=unit-test-strategy
fact: Every hook and MCP payload type that carries content derives veil Redact so Debug reveals no content; a type with no content field derives none, since veil 0.3.0 refuses a derive with no redacted field.
@task_fact n=5 slug=test-data-fixtures
fact: verify --record refuses an email-shaped token in any string or key of every recorded payload kind, not only screen fixtures, through the existing unclean-fixture detail.
@task_fact n=6 slug=test-strategy
fact: A remove-the-guard control fires its entry whole under nextest fail-fast, so the guard's own new case may go unrun while another case carries the red; isolating it needs its own control.
@task_item n=1 part=untouched
CI workflows, artifacts, action pinning, zizmor, caches and the CI leg: no workflow, step or job changed.
@task_item n=2 part=untouched
Coverage floors, mutation gate thresholds and cadence, performance budgets and hyperfine gates: no threshold, floor or bound moved.
@task_item n=3 part=untouched
The test harness contract: boot, cleanup, status, run, pre-push, gate, schema-check and secret-scan documents, exit codes and reasons are unchanged.
@task_item n=4 part=untouched
Fuzzing and property tests: no fuzz target, corpus or proptest strategy changed.
@task_item n=5 part=untouched
Wrapper channel JSON-RPC errors, link and unlink methods, wait and last, revive refusals and the event line format: no wire body, code or record changed; the replay change is a guard named, not code.
@task_item n=6 part=untouched
Budget governor thresholds, pause and release, and the wheel rules: untouched.
@task_item n=7 part=untouched
Process log line schema, corr fields and the codes-only role logs: no log field or event added.
@task_item n=8 part=untouched
Mocking and stubbing discipline, injected clocks and fake external processes: untouched.
