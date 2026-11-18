@task_prose part=session
viola run and viola revive now write the CLI's trust record for an untrusted session folder at a wrapped start, so no trust dialog stands and no key is typed (matrix v2-33).
The start's trust step runs after the version gate and before the channel bind, only on a verified CLI; it follows the CLI's walk (the exact folder key, then each parent up to the git root), skips the user's home folder, sets only the hasTrustDialogAccepted flag, and reads the entry back up to three times.
Each such start writes one new INFO log line, event trust-record, its detail one of a new closed set of 16 start words, with no path, key or value; no span covers the step, and the word never refuses the start (no exit code, hint or --json code moves).
The closed obs event set grew from 15 to 16, both diagnostic schemas gained trust-record, and CLAUDE_CONFIG_DIR's value is now read by the run and revive start too (when the strip keeps the name), not by verify alone.
Retired: the rule that run and revive write no trust record; the fake agent's oracle that inside a repository only the root's record counts.
Measured live on claude 2.1.325 only: S1 (viola run, first screen the prompt box, record written) against S2 (plain claude, first screen the trust form); CI green 7/7.
@task_file n=1 path=crates/viola-agent-claude/src/trust.rs
what: the Claude-shape trust-record logic with no file I/O: the CLI's trust walk, the flag-only edit, the exact read-back, and the closed start-word set
@task_file n=2 path=src/trust_record.rs
what: the guarded trust-record writer shared by verify and the run/revive start: the Claude config file choice (CLAUDE_CONFIG_DIR) and the start's trust step with its read-backs
@task_file n=3 path=src/cmd/run.rs
what: the viola run start (and viola revive's start arm): the start order with the version gate, the new trust step, the channel bind and the child spawn
@task_file n=4 path=src/cmd/verify.rs
what: viola verify, its own trust record and the Claude config file lookup it now shares
@task_file n=5 path=crates/viola-core/src/obs.rs
what: the closed obs event enum and its log-format event list, the obs_event line vocabulary
@task_file n=6 path=schemas/diag-line.v1.json
what: the diag-line v1 schema: event enum, per-event message and detail rules
@task_file n=7 path=schemas/diag-detail.v1.json
what: the self-contained detail-line schema and its event enum
@task_file n=8 path=src/bin/viola-fake-agent.rs
what: the fake agent's trust reading, now walking from its cwd to the git root like the CLI
@task_file n=9 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's test cases and options, including the trust-walk test and a bounded-reader boundary
@task_file n=10 path=tests/support/home.rs
what: test-home support: stamped homes, the unmade user home, the untrusted-boot and revive-as-user wrappers
@task_file n=11 path=tests/support/cli.rs
what: the test CLI runner, now giving every test run and revive an unmade user home
@task_file n=12 path=tests/run_cli.rs
what: viola run integration tests: start and exit lines, role-line counts, the new trust cases
@task_file n=13 path=tests/cli_revive.rs
what: viola revive integration tests, with the revive writing the record for its recorded cwd
@task_file n=14 path=tests/tui_passthrough.rs
what: the PTY passthrough tests: no viola byte on the terminal, now for a trust-writing start
@task_file n=15 path=tests/contract_diag_schema.rs
what: the diagnostic schema conformance tests: the schema event enum equals the log-format literals, real run lines validate
@task_file n=16 path=tests/contract_bounded_readers.rs
what: the bounded-readers contract check, each boundary naming a bounded read that stands
@task_file n=17 path=tests/chaos_snapshot_replay.rs
what: snapshot replay chaos tests, given a test-owned user home
@task_file n=18 path=tests/cli_controls_not_disableable.rs
what: the tests that redaction and controls cannot be disabled, given a test-owned user home
@task_file n=19 path=tests/cli_fake_agent.rs
what: CLI tests against the fake agent, given a test-owned user home
@task_file n=20 path=tests/cli_version_gate.rs
what: the version gate tests, given a test-owned user home
@task_file n=21 path=tests/cross_exit_causes.rs
what: cross-process exit cause detail code tests, given a test-owned user home
@task_file n=22 path=tests/cross_killed_test.rs
what: the killed-test cross-process tests, given a test-owned user home
@task_file n=23 path=tests/channel_error_bodies.rs
what: channel error body tests, given a test-owned user home
@task_file n=24 path=crates/viola-e2e/src/harness/boot.rs
what: the E2E harness boot, now giving a stamped session its own user home
@task_file n=25 path=crates/viola-e2e/src/harness/supervise.rs
what: the harness's supervised wrappers, started under the session user home
@task_file n=26 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig: its arms, the S1 and S2 trust arms, the floor rows and the round summary
@task_file n=27 path=crates/viola-e2e/src/rig/isolate.rs
what: the live rig's scratch isolation: the sentinel config entry, fresh cwd, entry words and the run trust word
@task_file n=28 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live rig binary's command line, with the new live and rehearse trust arms
@task_file n=29 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig's rehearsal tests, with the ts and tc trust rehearsals
@task_item n=1 part=touches
decision/trust-record-write-uninstrumented: the trust-record write is no longer verify's alone; the run and revive start now writes it and emits one trust-record log line, still with no span, so the row's scope and its no-line claim, if it holds one, are stale.
@task_item n=2 part=touches
Rows saying the CLAUDE_CONFIG_DIR value is read by verify only (the env-names, R8 strip and env-not-configuration rows): the run and revive start now reads that value too, through the same config-file lookup, when the strip keeps the name; the value is still never logged.
@task_item n=3 part=touches
The closed event set rows (event-values-closed, closed-event-and-corr-on-every-line, diag-schema-matches-event-enum, tests-event-list-adopted, contract/process-log/event, contract/log-line/event, contract/process-log-line/event and enum-values): a new event trust-record joins the set, in both schema enums; any cell listing the events or their count (now 16) moves.
@task_item n=4 part=touches
contract/run-start-events/order and the run process log rows (contract/run-process-log/*, contract/process-log/child-and-probe-lifecycle): a run start now writes one trust-record line after the version probe and before the channel bind, so a start's role-line count rises by one.
@task_item n=5 part=touches
contract/run-start-spans/span-chain and contract/run-span-attributes/*: the trust step sits between run.version_gate and channel.bind with no span of its own; a span chain that reads gate-then-bind gains an uninstrumented step between them.
@task_item n=6 part=touches
decision/revive-start-logs-as-run, contract/cli-log-file/revive-start, decision/revive-preflight-order: viola revive's start runs the same trust step, keyed by the snapshot's recorded cwd, and writes the same trust-record line.
@task_item n=7 part=touches
decision/diag-schema-additive-only, decision/log-schema-additive-only, decision/log-fields-additive-only, decision/additive-field-naming: the diag-line schema gains an event, its message const and a required detail enum; additive only, no field renamed.
@task_item n=8 part=touches
decision/detail-codes-only, decision/detail-codes-no-paths, decision/no-paths-or-pids-in-detail-codes, decision/home-level-lines-path-free: the trust-record detail is a closed code word from 16 start words, with no path, project key or config value.
@task_item n=9 part=touches
contract/diag-line-fields/* and contract/diag-detail/*: a new event row is owed beside them, trust-record with its detail words; verify's key-present and removed words map to lost on the start.
@task_item n=10 part=touches
decision/run-silent-on-terminal, decision/run-silent-terminal, contract/run-terminal-output/*, contract/stdout-by-role/run: the trust-writing start adds no viola byte on the terminal and no new start refusal, exit code, hint or --json code.
@task_item n=11 part=touches
decision/unreadable-ledger-is-unverified, decision/unreadable-ledger-means-unverified, contract/run-log/cli-verified: the trust step runs only when the version gate reads the CLI verified; an unverified CLI reads no config file.
@task_item n=12 part=touches
decision/uninstrumented-components: the start's trust step and the shared trust-record writer are a component left without a span by decision.
@task_item n=13 part=touches
decision/claude-internals-not-a-contract, decision/claude-boundary-only-instrumentation: viola now models and writes the CLI's trust-walk shape in the Claude config at every wrapped start, measured on claude 2.1.325 only.
@task_item n=14 part=touches
decision/verify-lines-redacted, decision/verify-lines-carry-no-content, decision/verify-logs-only-with-instance: verify's config-file lookup moved into the shared trust-record module; verify keeps calling it and its lines do not change.
@task_item n=15 part=touches
decision/schema-conformance-gate, decision/g4-schema-conformance-gate, decision/diag-lines-schema-checked-in-unit-tests: real run lines now carry trust-record and must validate; the conformance test's expected line counts moved.
@task_item n=16 part=touches
decision/test-homes-under-e2e-home, decision/test-homes-under-scan-root, decision/test-homes-kept-until-gates: every test run and revive and every supervised harness wrapper now gets a test-owned user home, so no test start touches the runner's real Claude config; the trust cases' folders sit under /tmp outside any repository.
@task_item n=17 part=touches
decision/fixtures-not-telemetry, decision/ci-verify-fake-agent-only, decision/real-cli-verify-never-in-ci: the fake agent's trust reading now walks from its cwd to the git root; the old root-only oracle is retired, and the live S1 and S2 starts ran off CI.
@task_fact n=1 slug=6-log-coverage
fact: Each wrapped start of viola run and viola revive on a verified CLI writes exactly one INFO trust-record line to the run role file, its detail one of a closed set of 16 start words, carrying no path, project key or config value; the word never refuses the start.
@task_fact n=2 slug=log-coverage
fact: The diag-line schema carries trust-record in its event enum with a fixed message and a required detail drawn from the 16 start words, and the detail-line schema's event enum carries trust-record too.
@task_fact n=3 slug=4-span-trace-coverage
fact: The run and revive start's trust step (walk, flag-only write, read-back) is uninstrumented by a span by decision; its only telemetry is the one trust-record line.
@task_fact n=4 slug=span-trace-coverage
fact: The start's trust step runs after the version gate and before the channel bind, only when the CLI is verified; the revive's trust record is keyed by the snapshot's recorded cwd.
@task_fact n=5 slug=1-obs-scope-summary
fact: CLAUDE_CONFIG_DIR's value is read by verify and by the run and revive start (to find the Claude config file the trust record lives in), and is never logged by either.
@task_item n=1 part=untouched
The viola mcp server, its tools, stdout frames and isError results: no MCP surface moved.
@task_item n=2 part=untouched
The hook role, hook decision and fail-open rows, hook latency budgets: the hook is untouched.
@task_item n=3 part=untouched
The channel, send, wait, last, dialog, link and wheel paths, their spans, corr and conn rows: no IPC method, endpoint or socket added.
@task_item n=4 part=untouched
Panic capture, error-chain and detail-file error rows: no panic or error path changed.
@task_item n=5 part=untouched
Metric coverage, heartbeat liveness and SLO budget rows: no counter, gauge or budget moved.
@task_item n=6 part=untouched
The absences (OTel exporters, error reporters, network egress, database, gRPC, deployment): none added or removed; no dependency added or bumped, so banned-dependency and obs-dependency rows stand.
@task_item n=7 part=untouched
CI job layout, artifact retention, upload and secret-scan rows: no CI step changed.
@task_item n=8 part=untouched
State recovery, snapshot replay and events.ndjson product stream rows: no product event added.
