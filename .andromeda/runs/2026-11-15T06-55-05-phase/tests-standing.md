# tests · standing conventions

- S1 · each new test is named by the test function naming rule and lives in the single test-only E2E crate, tokio confined to it · 2-test-strategy · test-strategy
  decision/cargo-deny-tokio-and-c-crate-bans · Tokio and C-crate bans — The dependency policy bans tokio where it is not allowed and bans C crates. · guard: cargo deny
  decision/hook-suites-in-integration-layer · Non-path suites layer — Non-path trigger-coverage suites run in the E2E layer. · except: root hook_* files run in the integration layer
  decision/test-function-naming · Test function naming — Test functions are named <subject>_<condition>_<expected> in snake_case. · except: critical-path scenarios are named path<N>_<slug>
  decision/tokio-only-in-e2e-crate · Tokio test code confined to the E2E crate — One test-only, unpublished crate, viola-e2e, hosts the harness and the tokio-based test clients. · why: so the root package and the sync crates carry no tokio dev-dependency
- S2 · each new test or rig creates its own private home under target/e2e-home, cleaned up idempotently unless kept on request · 5-command-implementation · test-data-bootstrap
  decision/ci-homes-kept-until-gates · CI homes kept until gate steps — In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run. · except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.
  decision/cleanup-between-integration-tests · Cleanup between integration tests — Integration tests never skip cleanup between tests. · why: Order-dependent tests hide bugs.
  decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
  decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
  decision/keep-failed-home-opt-in · Failed test homes kept only on request — A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1. · why: So that logs can inspect the home.
  decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state.
  decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
  decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint or log is shared between tests.
  decision/private-dir-for-test-endpoints · Test endpoint directories made private — A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir. · why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
  decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.
  decision/session-home-retention · Session home retention — Logs are retained for the whole lifetime of the session home; events are never truncated, and cleanup deletes the home except where homes are kept.
  decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
  decision/tests-run-in-parallel · Tests run in parallel — Tests are never serialised with a single thread; isolation comes from the per-test home. · except: The fuzz stages are single jobs, by design.
- S3 · each new fixture is recorded per CLI version from a real run, synthetic and scrubbed, never hand-written · 7-test-data-fixtures · test-data-fixtures
  decision/hook-e2e-recorded-stdin · Hook verbs fed recorded stdin — The hook verbs are tested by running them as processes fed fixture stdin recorded per agent CLI version.
  decision/hook-payloads-recorded-by-verify · Hook payloads recorded by verify, replayed by CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed and unclean-checked before they are committed, a dirty payload refusing the whole recording, and CI replays all of them.
  decision/synthetic-fixture-text · synthetic fixture text only — Fixture prompts and checked-in fake agent scenario scripts hold synthetic probe text only, never real prompts.
- S4 · each new wait is keyed on a logged event and bounded, with no sleep and no retry · 11-test-anti-patterns-never-do-these
  decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
  decision/hook-deadline-driver · viola hook driven under an Instant deadline — viola hook is driven by std::process::Command with an Instant deadline over recorded fixture stdin.
  decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
- S5 · each new harness log line is a JSON tracing line with codes only, detail apart, carrying the owned fields · log-format
  contract/gate-output/detail · gate output · detail — A fixed code or a count, never file content.
  contract/log-files/modes · log files · modes and writes — Files are 0600, dirs 0700, with one write per line.
  contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
  contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
  contract/log-line/instance · process log line · instance — A ViolaName or null.
  contract/log-line/level · process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/log-line/message · process log line · message — fields.message, or the flattened message; required.
  contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
  contract/log-line/process · process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file.
  contract/log-line/target · process log line · target — A required field.
  contract/log-line/timestamp · process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/pre-push-output/paths · pre-push output · paths — Codes, counts and repo-relative names only, never a home or repository path.
  decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
  decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
  decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
  decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.
  decision/hook-panic-payload-to-detail-file · Hook panic payload kept to the detail file — A caught hook panic writes one payload-free role panic line, and its payload and backtrace go to one detail line only. · guard: hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line
  decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.
  decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
