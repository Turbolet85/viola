@task_prose part=session
The session closed rule lines the route still owed, in code and tests. It added no capability, retired nothing and changed no wire body.
Type-level redaction: the hook payload, dialog and response types, and the MCP send and answer inputs, now derive veil Redact instead of Debug, so Debug never shows content. Redacted payload types went from 1 to 7. The three MCP inputs with no content field (wait, last, list) derive neither, because veil 0.3.0 will not compile the Redact derive on a type with no redact field.
Private diagnostics append: the open now refuses a symbolic link at the path (no-follow) and sets mode 0600 through the open handle, so a link is a failed open. The strict-modes check of trusted state now refuses a symbolic link at the home, instances, an instance dir, the snapshot or the stamps file, before the mode verdict. It adds no new code or message.
Human refusal hints: a send refused unverified-cli (exit 12) now ends with the hint to run viola verify. An answer refused not-delivered now gets the send table's hint for five more details (2 to 6 hinted details). verify --record's fixture scrub now refuses an email-shaped token on every payload kind, and the fixture hygiene walk gains the email class.
Tests: the resize test sends no key after a resize. The fd test drops its fd count and its Linux-only split, and its OS gate reason now reads that inherited file descriptors are Unix's. One shared digit_runs helper replaces three copies. run uses send's name check (one parse_name). The snapshot-replay link test is now proven the guard of the replay's link pass. CI was green on ef625f20.
@task_file n=1 path=src/human/hints.rs
what: the human-mode refusal hint tables for send and answer, the hint line of the CLI stderr unable and hint pair
@task_file n=2 path=src/cmd/send.rs
what: the viola send verb, its refusal-reason to hint mapping and the one instance-name check the CLI verbs share
@task_file n=3 path=src/cmd/answer.rs
what: the viola answer verb, its driver-side answer refusal output on stderr
@task_file n=4 path=src/cmd/run.rs
what: the viola run verb's arguments, its instance name parsed through the shared name check
@task_file n=5 path=crates/viola-agent-claude/Cargo.toml
what: the claude agent crate's dependency list, a sync crate built without Tokio, now taking veil
@task_file n=6 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger and verify --record's fixture scrub and refusal (unclean, email class)
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: unit cases of the ledger's fixture scrub checks
@task_file n=8 path=crates/viola-agent-claude/src/hook.rs
what: the hook payload types carrying prompt and last_assistant_message, now deriving veil Redact
@task_file n=9 path=crates/viola-agent-claude/src/dialog.rs
what: the dialog payload and dialog response types carrying dialog content, now deriving veil Redact
@task_file n=10 path=crates/viola-mcp/Cargo.toml
what: the viola-mcp stdio server crate's dependency list, now taking veil
@task_file n=11 path=crates/viola-mcp/src/tools.rs
what: the viola-mcp stdio server tools' input types (send and answer redacted; wait, last and list content-free)
@task_file n=12 path=crates/viola-state/src/strict.rs
what: the strict-modes check of trusted state (home tree, instance, snapshot, stamps), now refusing a symbolic link
@task_file n=13 path=crates/viola-state/src/fs.rs
what: the private file opens, the owner-only diagnostics append open (no-follow, 0600 through the handle) and the lock open
@task_file n=14 path=Cargo.lock
what: the workspace lockfile, the dependency pins
@task_file n=15 path=tests/cross_exit_causes.rs
what: the exit-cause and refusal table check, no two codes sharing a hint and no hint naming release
@task_file n=16 path=tests/cli_answer.rs
what: the viola answer integration tests, the human-mode refusal and hint pair
@task_file n=17 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over agents, status-line, claude, script, screen and MCP fixtures
@task_file n=18 path=tests/contract_os_gated_tests.rs
what: the OS-gated test census, each gated test with its gate and reason
@task_file n=19 path=tests/tui_pty_seam.rs
what: the PTY seam tests on a real openpty, the resize reaching the child
@task_file n=20 path=tests/tui_channel_fds.rs
what: the inherited file descriptor test reading the live child's fd table
@task_file n=21 path=tests/security_negatives_server_identity.rs
what: the security negative tests of server identity (peer credentials)
@task_file n=22 path=tests/security_negatives_socket_dir.rs
what: the security negative tests of the socket directory check
@task_file n=23 path=tests/security_negatives_strict_modes.rs
what: the security negative tests of the strict-modes check
@task_file n=24 path=tests/support/hygiene.rs
what: the shared fixture hygiene check and its violation classes, now with email
@task_file n=25 path=tests/support/mod.rs
what: the shared test helper module list
@task_file n=26 path=tests/support/digits.rs
what: the shared test helper digit_runs, the one copy the security negative tests use
@task_item n=1 part=touches
The decision that payload types derive veil Redact (type-level redaction of payload types) changed. Every payload type that carries content derives Redact so Debug shows no content. A type with no content field derives none: veil 0.3.0 refuses the derive. The row's claim that all hook and MCP payload types derive it is disproved, and its owed route divergence closes.
@task_item n=2 part=touches
The rows on veil redacting payload types and on veil as the second redaction layer now read on seven redacted types (was one, the channel request): hook normalised and known, dialog and dialog response, and the MCP send and answer inputs.
@task_item n=3 part=touches
The decision on owner-only private diagnostics files changed, and its owed line closes. The append open refuses a symbolic link at the path and sets 0600 through the open handle, not by a path-level chmod.
@task_item n=4 part=touches
Rows on diagnostics file modes (owner-only, 0600, owner-only log files, diagnostics checked at open) now hold under a no-follow open. Mode 0600 is applied through the handle, and a link never gets its target narrowed.
@task_item n=5 part=touches
Rows saying a refused diagnostics path is a failed open, and that a failed log open discards or drops its line, now also cover a symbolic link at a role or detail file path.
@task_item n=6 part=touches
Rows that put the strict-modes check before diagnostics or before log init (home tree, instance, content sink behind a strict check, a pre-open strict refusal writes no line) now cover a symbolic link. The check refuses the link before the mode verdict, under the same refusal and message.
@task_item n=7 part=touches
The strict-modes-failed detail and exit contracts (exit 1, exit 21, the MCP frame tool refusal, CLI --json, verify, link, unlink and revive) now also fire for a symbolic link at a trusted-state path. The code word and message are unchanged.
@task_item n=8 part=touches
The driver-side answer refusal contracts (not-delivered, unverified-cli) and the CLI stderr unable and hint pair moved. An answer refused not-delivered now gets a hint for control-character, empty-text, input-not-ready, no-prompt-submitted and turn-running as well as unknown-dialog (2 to 6 hinted details). unknown, a fault or an opaque detail still gets no hint.
@task_item n=9 part=touches
The wrapper send-refused contracts and the send line's refusal output: a human-mode send refused unverified-cli (exit 12) now ends with the hint to run viola verify for this CLI version, the same string on both verbs. Before, it printed no hint.
@task_item n=10 part=touches
Rows on recorded fixtures (fixtures are scrub-required, fixtures are not a telemetry source): verify --record's scrub now refuses an email-shaped token in any string or key on every non-screen payload kind, through the existing unclean-fixture and email codes.
@task_item n=11 part=touches
The dependency rules over obs crates (obs deps Tokio-free and C-free, obs deps pass cargo deny, banned telemetry dependencies, sync crates without Tokio): veil 0.3.0 is now a dependency of the claude agent crate and the MCP server crate. cargo deny and the per-sync-crate Tokio bans read green.
@task_item n=12 part=touches
The veil toggle feature ban stays held: the new veil edges take the workspace pin without toggle, and redaction still cannot be disabled.
@task_item n=13 part=touches
The OS-gated test census row: the inherited file descriptor test keeps gate unix, its reason now saying inherited file descriptors are Unix's. Its Linux-only split and its fd count assertion are gone, and the live child fd table is read always.
@task_item n=14 part=touches
Rows over the CLI instance-name check (cli log only under a parsed name, instance from its name): run now parses its name through the one shared name check. The refusal is still clap's usage error, exit 2.
@task_item n=15 part=touches
Rows that hold no telemetry from screen content and keep dialog, prompt, assistant message and sent text out of logs are reinforced indirectly. The hook, dialog and MCP input types that carry this content can no longer print it through Debug.
@task_fact n=1 slug=8-pii-scrubbing-compliance
fact: A payload type with no content field derives neither veil Redact nor Debug, because veil 0.3.0 refuses a Redact derive with no redact field. This is the exception to the convention that payload types derive Redact.
@task_fact n=2 slug=otel-sdk-init
fact: The private diagnostics append open never follows a symbolic link: a link at a role or detail file path is a failed open, and mode 0600 is set through the open handle.
@task_fact n=3 slug=otel-sdk-init
fact: The strict-modes check refuses a symbolic link at the home, instances, an instance dir, the snapshot or the stamps file before the mode verdict, and leaves the link and its target as found.
@task_item n=1 part=untouched
Span naming, span kinds, span attributes and span chains: no span was added, renamed or given a field.
@task_item n=2 part=untouched
Metric coverage and derived metrics (counters, gauges, histograms, quantiles): no metric or grouping moved.
@task_item n=3 part=untouched
CI gates G1 to G4, the hook perf budgets, hyperfine thresholds, the zero-panics gate and the secret-scan upload rules: no gate, threshold or CI step changed.
@task_item n=4 part=untouched
Panic capture and panic outcome by role, the panic line and the detail-file line fields: unchanged.
@task_item n=5 part=untouched
Correlation (corr and conn propagation), the process log line field schema, log levels and third-party targets: no field, level or target moved.
@task_item n=6 part=untouched
Service identity, version constants, heartbeat liveness, config.json keys, the events log and log rotation: untouched.
@task_item n=7 part=untouched
Absences (OTLP receiver, OTel exporter, Sentry, network egress, database, gRPC, on-call): none taken up or freed.
