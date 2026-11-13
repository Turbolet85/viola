@task_prose part=chunk
Closes in code or tests the rule lines the rows landing owed the route: every refusal reason prints a hint, the email check reaches every committed fixture kind and the record write, run parses its name through the shared name check, trusted-state checks and the diagnostics open refuse a symlink, and the hook and MCP payload types derive veil Redact.
Tidies tests: the PTY resize test drops its needless key, the fd test always reads the live child table and drops its fd count assertion, and one shared digit-run test helper replaces three copies.
Gives the snapshot-replay row a guard that does not leave with the budget leg, and re-runs that guard under its one-off remove-the-guard control.
Adds no refusal reason, exit code, event kind or wire field; a hint is a human-output line. No mutation run and no live claude start.
@task_file n=1 path=src/cmd/send.rs mark=edits
what: the send verb and the shared CLI name check — edits: hint_cause (the refusal-reason hint map, unverified-cli and unknown today print none), parse_name made the one shared copy
@task_file n=2 path=src/cmd/run.rs mark=edits
what: the run verb's start — edits: its own copy of parse_name, replaced by the shared name check
@task_file n=3 path=src/cmd/answer.rs mark=edits
what: the answer verb — edits: the not-delivered refusal details, each gaining a hint
@task_file n=4 path=src/human/mirror.rs mark=edits
what: the human stderr output, the unable and hint pair — edits: write_unable, the hint no longer optional
@task_file n=5 path=crates/viola-agent-claude/src/ledger.rs mark=edits
what: the claude capability ledger, the fixture scrub and its record write, and the hook payload types — edits: text_fault (absolute path and user word today), has_email, screen_text_fault, the --record write, the hook payload types deriving veil Redact
@task_file n=6 path=tests/contract_fixture_hygiene.rs mark=edits
what: the fixture hygiene walk test over every committed fixture kind — edits: the walk filters, claude_fixtures, the hook, agents and statusline walks gaining the email check that check_mcp and the screen walk hold
@task_file n=7 path=tests/support/hygiene.rs mark=edits
what: the shared fixture hygiene support for tests — edits: Violation, gaining an email class beside absolute path, user name and schema
@task_file n=8 path=crates/viola-state/src/strict.rs mark=edits
what: the strict-modes trusted-state checks — edits: check_instance, check_stamps, check_path, check_read, unix::reading (stops following a symlink)
@task_file n=9 path=crates/viola-state/src/fs.rs mark=edits
what: the owner-only private file open for diagnostics — edits: open_private_append and restrict, the open refusing a symlink and narrowing the open file rather than the path
@task_file n=10 path=src/obs.rs mark=edits
what: the diagnostics init and the diagnostics files check — edits: check_diagnostics, the caller of the private open
@task_file n=11 path=tests/tui_pty_seam.rs mark=edits
what: the PTY seam test on a real openpty — edits: the resize test, its needless key write removed
@task_file n=12 path=tests/tui_channel_fds.rs mark=edits
what: the channel fd test over the child's open file table — edits: the agent's fds self-dump receipt and the linux-only split removed, the fd count assertion removed, the live child table read kept
@task_file n=13 path=tests/cli_links.rs mark=re-reads
what: the snapshot-replay guard test that the replay reads the link the wrapper wrote, re-run under its one-off remove-the-guard control
@task_file n=14 path=tests/security_negatives_server_identity.rs mark=edits
what: the server-identity security negative tests — edits: its digit_runs copy and its digit-run whole-numbers test, moved to the shared test support
@task_file n=15 path=tests/security_negatives_socket_dir.rs mark=edits
what: the socket-directory security negative tests — edits: its digit_runs copy and its digit-run whole-numbers test, moved to the shared test support
@task_file n=16 path=tests/security_negatives_strict_modes.rs mark=edits
what: the strict-modes security negative tests — edits: its digit_runs copy, replaced by the shared test support helper
@task_item n=1 part=needs
Type-level redaction of payload types: the hook and MCP payload types derive veil Redact as the second redaction layer, the veil toggle feature staying banned.
@task_item n=2 part=needs
The owner-only diagnostics files and their modes: the private open refuses a symlink, checks before open and narrows mode on the open file, closing the lstat-open-chmod race.
@task_item n=3 part=needs
A refused diagnostics path is a failed open and drops its line; the diagnostics caller's handling of a refused open as the chunk changes the open.
@task_item n=4 part=needs
The home strict-modes check before diagnostics and the strict-modes refusal that writes no line: a symlinked trusted-state path now refuses, and the refusal still writes no line before a log opens.
@task_item n=5 part=needs
How a skipped or refused symlink shows in telemetry: never named in logs, counted as parse-rejected with the symlink-ignored detail where it is skipped.
@task_item n=6 part=needs
The CLI stderr unable and hint pair, and the fixed stderr refusal lines: every refusal reason, the unverified-cli and unknown fallback included, prints its fixed pathless hint.
@task_item n=7 part=needs
The driver-side answer refusals, not-delivered and unverified-cli, whose details now each print a hint on stderr.
@task_item n=8 part=needs
The refusal reason words and their unknown fallback, which the hint map covers without adding a reason, exit code or detail word.
@task_item n=9 part=needs
Error and hint messages interpolate nothing: a hint is a fixed string that carries no path, name or content.
@task_item n=10 part=needs
The cli process log written only under a parsed name: run's start now parses its name through the shared name check.
@task_item n=11 part=needs
Claude fixtures are scrub-required: the email check reaches every committed fixture kind and the record write, beside the absolute-path and user-word checks.
@task_item n=12 part=needs
Recorded fixtures are no telemetry source, and the fixture scrub's findings name no matched bytes.
@task_item n=13 part=needs
Product readers read the snapshot through replay: the replayed snapshot's live fields and its unreachable reading, which the new replay guard holds.
@task_item n=14 part=needs
The state-recovered snapshot-replayed line and detail the replay path writes, as the guard test observes them.
@task_item n=15 part=needs
The PTY seam and the channel fd boundary: the resize receipt follows a resize alone, and the fd test reads the live child table on the Linux-only target.
@task_item n=16 part=needs
Linux-only OS target: removing the fd test's linux-only split leans on the single supported OS.
@task_item n=17 part=needs
No telemetry from screen content and no content in the payload types' Debug output, which the Redact derive serves.
@task_standing n=1
convention: each new or changed integration test that drives viola keeps its test home under the one scanned root, kept until the gates run
home: ci-integration · 9-ci-integration
@task_standing n=2
convention: each OS-gated test joins the OS-gated test census
home: ci-integration
@task_standing n=3
convention: emitted diagnostics lines in each integration test are schema-checked against the line schema
home: ci-integration
@task_standing n=4
convention: a test waits on a log line by reading for it, never by sleeping
home: 10-slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
Budget governor rows (budget spans, budget log lines, the budget-gate event, release with budget): the chunk removes no budget gate and only adds a replacement replay guard.
@task_item n=2 part=not_needed
Status-line rows (the statusline shell-out, its deadline, its command string, the statusline hook log fields and decision details): no status-line behaviour changes here.
@task_item n=3 part=not_needed
Heartbeat, liveness and staleness rows: no heartbeat or liveness reading changes.
@task_item n=4 part=not_needed
Performance budget rows (hook latency budgets, hyperfine gates, the perf job): no hook path or benchmark changes.
@task_item n=5 part=not_needed
CI upload, artifact retention and secret-scan hit report rows: no CI job, upload or scan step changes.
@task_item n=6 part=not_needed
Future error reporter, Sentry, OTel exporter and network egress rows: nothing is exported or sent.
@task_item n=7 part=not_needed
Derived metrics, counters, gauges and quantile rows: no metric is derived or regrouped.
@task_item n=8 part=not_needed
Panic capture rows (panic hook, panic line, panic outcome by role, catch sites): no panic path changes.
@task_item n=9 part=not_needed
Span chain and span attribute rows for the confirmed send, wait, last, dialog, wheel and run start paths: no span is added or moved.
@task_item n=10 part=not_needed
Harness logs and agent-run status and logs output rows: no harness reader changes.
@task_item n=11 part=not_needed
config.json key rows (five_hour, seven_day, dialog_hold_s, diagnostics_level): no configuration key changes.
