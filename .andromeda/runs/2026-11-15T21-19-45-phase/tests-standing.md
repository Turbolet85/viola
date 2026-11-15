# tests · standing conventions

- S1 · Each new test function is named by the test-naming rule and lives in the layer its binary prefix selects · 2-test-strategy · test-strategy
  decision/e2e-layer-by-binary-prefix · E2E layer selected by binary prefix — The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
  decision/integration-binary-surface-prefix · Root integration binaries prefixed by surface — Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix. · why: so nextest filtersets can select layers
  decision/module-boundary-integration-tests · Module boundaries tested by direct calls — The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
  decision/test-function-naming · Test function naming — Test functions are named <subject>_<condition>_<expected> in snake_case. · except: critical-path scenarios are named path<N>_<slug>
  decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
- S2 · Each new test creates its own fresh private home, seeds its own data, and drops inherited session environment · 7-test-data-fixtures · test-data-fixtures
  contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
  contract/secret-scan-class/claude-stripped · secret-scan hit class · claude-stripped — A hit of the fixed CLAUDE* canaries the root tests plant.
  decision/chaos-home-outside-scan · One chaos home outside the scans — Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design. · why: A ratified carve-out.
  decision/ci-homes-kept-until-gates · CI homes kept until gate steps — In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run. · except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.
  decision/env-clear-keeps-llvm-profile-file · env_clear tests re-add LLVM_PROFILE_FILE — Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
  decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
  decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
  decision/keep-failed-home-opt-in · Failed test homes kept only on request — A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1. · why: So that logs can inspect the home.
  decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state.
  decision/owner-record-removed-last · Owner record removed last — Removing a test home deletes its owner record last. · why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.
  decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
  decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint or log is shared between tests.
  decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.
  decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.
  decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.
  decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
  decision/tempfile-test-dirs · tempfile for test directories — tempfile is what provides temporary test directories.
  decision/test-home-owner-record-sweep · Owner-record sweep of test homes — Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record. · why: A test the runner kills never drops its home.
  decision/test-homes-made-private · Test-made homes are private directories — A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation. · why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
  decision/test-started-viola-drops-env · Test-started viola drops inherited env — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.
  decision/test-viola-drops-session-vars · Test-started viola drops session variables — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
  decision/tests-run-in-parallel · Tests run in parallel — Tests are never serialised with a single thread; isolation comes from the per-test home. · except: The fuzz stages are single jobs, by design.
  decision/tests-seed-own-data · self-bootstrapping tests — Tests seed their own data; no developer pre-populates a home.
- S3 · Each new test uses literal oracles, no sleeps, no retries, no Debug-string comparison and no process-wide env mutation · 11-test-anti-patterns-never-do-these
  decision/ci-profile-no-retries · CI test profile without retries — The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
  decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
  decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
  decision/env-per-child · environment set per child process — Test environment is set per child process with Command::env, never with std::env::set_var. · why: std::env::set_var is unsafe in edition 2024.
  decision/human-output-expected-as-literals · Human-mode expected output as literals — Human-mode expected output is held as literals in the test files that check it.
  decision/identity-floor-tested-against-literal-copy · Identity floor tested against a literal copy — The R8 strip is tested against a literal copy of the identity floor, never an imported one.
  decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
  decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
  decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
  decision/no-process-env-mutation-in-tests · No process-wide env mutation in tests — Tests never set process environment variables globally; environment is set per child command. · why: Setting the process environment is unsafe in edition 2024 and races across threads.
  decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.
- S4 · Each new log line is one structured JSON line with its fixed fields and codes only, its content apart in the detail tier · log-format
  contract/log-files/modes · log files · modes and writes — Files are 0600, dirs 0700, with one write per line.
  contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
  contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
  contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
  contract/log-line/instance · process log line · instance — A ViolaName or null.
  contract/log-line/level · process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/log-line/message · process log line · message — fields.message, or the flattened message; required.
  contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
  contract/log-line/process · process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file.
  contract/log-line/target · process log line · target — A required field.
  contract/log-line/timestamp · process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
  decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
  decision/no-names-in-wrapper-log · No session names in the wrapper log — The wrapper's process log carries no driver or driven name, and channel lines carry no name or param. · guard: link_verbs_log_the_channel_pair_and_no_names
  decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.
  decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
- S5 · Each long-lived wrapper a test spawns is held in a drop guard and cleaned up · integration-test-strategy · 5-integration-test-strategy
  decision/cleanup-between-integration-tests · Cleanup between integration tests — Integration tests never skip cleanup between tests. · why: Order-dependent tests hide bugs.
  decision/mcp-stdio-test-owns-child · MCP stdio test owns the child — MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
  decision/wrapper-drop-guard · Test wrappers held in a Drop guard — Wrappers started in a test are held in a Drop guard that kills the child, then waits for it.
  decision/wrapper-spawned-with-guard · Long-lived wrapper spawned with a drop guard — Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call. · why: A blocking call waits until the wrapper exits.
- S6 · Each new table of unit cases is an rstest case table inline beside its unit · 2-test-strategy · 4-unit-test-strategy
  decision/rstest-case-tables · Table-driven unit cases in rstest — Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label. · why: so no manifest edge is added for rstest alone · except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
  decision/rstest-fixtures · rstest for fixtures — rstest is what provides test fixtures and cases.
  decision/rstest-fixtures-for-constructors · rstest fixtures for shared constructors — Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.
  decision/unit-tests-inline · Inline unit tests — Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary.
