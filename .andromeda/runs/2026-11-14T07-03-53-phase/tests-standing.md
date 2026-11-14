# tests · standing conventions

- S1 · Each new test function is named by the test-naming rule, written as an inline unit test or a table-driven rstest case where it fits, and run by nextest. · 2-test-strategy · test-strategy
  decision/doctests-via-cargo-test · Doctests run through cargo test — Doctests run through cargo test --workspace --doc. · why: nextest cannot run doctests
  decision/nextest-is-the-runner · nextest is the runner — nextest is the test runner; a plain cargo test run is not supported.
  decision/nextest-test-runner · cargo-nextest runs the tests — Tests run on libtest through cargo-nextest, process per test, with JUnit output. · except: Doctests run through cargo test, because nextest cannot run them.
  decision/rstest-case-tables · Table-driven unit cases in rstest — Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label. · why: so no manifest edge is added for rstest alone · except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
  decision/rstest-fixtures · rstest for fixtures — rstest is what provides test fixtures and cases.
  decision/security-sweep-under-nextest · Security sweep runs under nextest alone — The security sweep runs under nextest alone.
  decision/test-function-naming · Test function naming — Test functions are named <subject>_<condition>_<expected> in snake_case. · except: critical-path scenarios are named path<N>_<slug>
  decision/unit-tests-inline · Inline unit tests — Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary.
- S2 · Each new test creates its own private home and directories through tempfile and shares no mutable state with tests running in parallel. · 2-test-strategy · test-strategy
  contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
  decision/chaos-home-outside-scan · One chaos home outside the scans — Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design. · why: A ratified carve-out.
  decision/e2e-home-base-checks · Checked base of the test homes — The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.
  decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
  decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
  decision/keep-failed-home-opt-in · Failed test homes kept only on request — A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1. · why: So that logs can inspect the home.
  decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state.
  decision/owner-record-removed-last · Owner record removed last — Removing a test home deletes its owner record last. · why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.
  decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
  decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint, log or budget.json is shared between tests.
  decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.
  decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.
  decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.
  decision/tempfile-test-dirs · tempfile for test directories — tempfile is what provides temporary test directories.
  decision/test-home-owner-record-sweep · Owner-record sweep of test homes — Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record. · why: A test the runner kills never drops its home.
  decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.
  decision/tests-run-in-parallel · Tests run in parallel — Tests are never serialised with a single thread; isolation comes from the per-test home. · except: The fuzz stages are single jobs, by design.
- S3 · Each new fixture is synthetic or scrubbed, recorded rather than hand-written, and passes the fixture hygiene walk, its hits reported by class only. · 7-test-data-fixtures · test-data-fixtures
  contract/secret-scan-output/hits · secret-scan output · hits — Each hit is {file, line, offset, class}; the matched bytes are never printed.
  decision/fixtures-never-from-real-prompts · Fixtures never from real prompts — Fixtures must not come from real prompts and must be checked for absolute paths and usernames. · why: A security anti-pattern.
  decision/hook-payloads-recorded-locally · Hook payloads recorded locally, replayed in CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
  decision/hygiene-reports-class-only · hygiene checker reports class only — The hygiene checker reports a violation class only, never the offending content.
  decision/scan-hits-never-echo-bytes · Scan hits never echo bytes — A secret-scan hit is reported by place and class, never by the matched bytes.
  decision/scan-hits-never-print-bytes · Scan hits never print matched bytes — A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes. · guard: error_body_check_reports_no_byte_of_what_it_read
  decision/synthetic-inputs-with-canary · Synthetic test inputs carry a canary — Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
- S4 · Each new log line is one structured JSON line with the process log line fields, codes-only, with any detail kept apart. · log-format
  contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
  contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
  contract/log-line/instance · process log line · instance — A ViolaName or null.
  contract/log-line/level · process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/log-line/message · process log line · message — fields.message, or the flattened message; required.
  contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
  contract/log-line/process · process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file.
  contract/log-line/target · process log line · target — A required field.
  contract/log-line/timestamp · process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/process-start-claude-child/env-kept · process-start{subject:"claude-child"} record · env_kept — Carries the environment names kept for the claude child, names only, never values.
  contract/process-start-claude-child/env-stripped-known · process-start{subject:"claude-child"} record · env_stripped_known — Carries the known stripped environment names, names only, never values.
  decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.
- S5 · Each new test avoids the anti-patterns: no sleeps, no retries, no Debug-string comparison, literal oracles, no real network, no mocking of what viola does not own, no ignore parking. · 11-test-anti-patterns-never-do-these
  contract/gate-breach/suite-skipped · gate breach · suite-skipped — A required suite has skipped > 0.
  decision/ci-profile-no-retries · CI test profile without retries — The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
  decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
  decision/doc-examples-never-ignore · Doc examples never use ignore — A doc example that must not run uses no_run or a text fence, never ignore. · why: ignored doctests count as skipped
  decision/human-layouts-pinned-by-literal · Human-mode layouts pinned by literal — Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files. · guard: HELP literal in cli_output_plain; cli_list board tests
  decision/human-output-expected-as-literals · Human-mode expected output as literals — Human-mode expected output is held as literals in the test files that check it.
  decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
  decision/identity-floor-tested-against-literal-copy · Identity floor tested against a literal copy — The R8 strip is tested against a literal copy of the identity floor, never an imported one.
  decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
  decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
  decision/no-ignore-parking · No #[ignore] parking — `#[ignore]` is not allowed as a parking place for a failing or flaky test. · why: Agent-driven development cannot tell a flake from a real bug.
  decision/no-real-network-in-tests · No real network calls in tests — Tests never make real network calls; they use mocks or localhost stubs.
- S6 · Each new module and test counts toward the workspace coverage floor with no new exclusion, and any test failure fails the build. · 10-quality-gates-coverage-targets · quality-gates-coverage-targets
  contract/gate-breach/coverage · gate breach · coverage — Coverage below a floor, each floor its own breach.
  contract/gate-breach/suite-failed · gate breach · suite-failed — A required suite has failed > 0.
  contract/harness-run/exit · Harness run command · exit — Exits 0 only if every suite passes and no mutant survives; non-zero otherwise, with a JSON summary on stdout of suite, passed, failed and survived.
  contract/test-harness/run · test harness commands · run — run runs the unit and integration tests and the fake-agent E2E, plus mutation testing only under run --mutants, and exits 0 only if every suite passes and no mutant survives.
  decision/build-fails-below-coverage · Build fails below coverage threshold — The build fails when coverage falls below the quality-gate threshold.
  decision/build-fails-on-test-failure · Build fails on any test failure — The build fails when any unit, integration, E2E, contract or property test fails.
  decision/comprehensive-coverage-floors · Comprehensive-tier coverage floors — viola's coverage must be at least 85% of lines, 80% of branches (enforced as region coverage) and 95% of functions, enforced in the ubuntu test job. · guard: cargo llvm-cov fail-under-lines 85, fail-under-functions 95, fail-under-regions 80
  decision/coverage-exclusions · Coverage exclusions — The fake agent, the E2E crate, test support code and fuzz targets are excluded from coverage by one separator-agnostic filename pattern held in the harness. · why: The separator class is kept from the retired Windows target, where llvm-cov reported paths with backslashes.
  decision/coverage-floors · Coverage floors — CI enforces coverage floors of 85 lines, 95 functions and 80 regions, each floor its own breach, measured with the fake agent, viola-e2e, test support and fuzz trees ignored.
  decision/coverage-ignore-regex-not-widened · Coverage ignore regex is never widened — The coverage ignore-filename regex, covering the fake agent, the E2E crate, test support and fuzz trees, is never widened to pass a gate without a Decisions Log entry first. · why: Widening it is a back door to lowering the coverage thresholds. · except: The path-separator class kept in the regex is not a widening, because the four trees are unchanged.
  decision/coverage-threshold-fails-build · Coverage below tier threshold fails the build — Coverage below the threshold for its tier, measured in the `test` job on `ubuntu-latest`, is a build failure condition.
  decision/coverage-threshold-never-lowered · Coverage threshold never lowered to pass — The coverage threshold is never lowered to pass a build. · why: Raise quality, not the bar.
  decision/coverage-workspace-rollup · Coverage rolled up across the workspace — The coverage number rolls up across all workspace crates.
  decision/llvm-cov-coverage · cargo-llvm-cov measures coverage — Coverage is measured by cargo-llvm-cov over nextest, producing LCOV and a JSON summary.
  decision/quality-gate-command · One quality gate over the test outputs — The viola-harness gate reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach, failing the build.
  decision/test-failure-fails-build · Any test failure fails the build — A test failure at any level is a build failure condition.
