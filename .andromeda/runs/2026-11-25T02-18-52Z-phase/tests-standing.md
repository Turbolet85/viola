# tests · standing conventions

- S1 · each new test is named by the plan's test-function naming, runs under nextest in parallel with no retries or sleeps, and stands in the layer its binary prefix selects · 2-test-strategy · test-strategy
  decision/ci-profile-no-retries · CI test profile without retries — The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
  decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
  decision/integration-binary-surface-prefix · Root integration binaries prefixed by surface — Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix. · why: so nextest filtersets can select layers
  decision/module-boundary-integration-tests · Module boundaries tested by direct calls — The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
  decision/nextest-is-the-runner · nextest is the runner — nextest is the test runner; a plain cargo test run is not supported.
  decision/nextest-test-runner · cargo-nextest runs the tests — Tests run on libtest through cargo-nextest, process per test, with JUnit output. · except: Doctests run through cargo test, because nextest cannot run them.
  decision/security-sweep-under-nextest · Security sweep runs under nextest alone — The security sweep runs under nextest alone.
  decision/test-function-naming · Test function naming — Test functions are named <subject>_<condition>_<expected> in snake_case. · except: critical-path scenarios are named path<N>_<slug>
  decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
  decision/tests-run-in-parallel · Tests run in parallel — Tests are never serialised with a single thread; isolation comes from the per-test home. · except: The fuzz stages are single jobs, by design.
- S2 · each new test creates its own private home under the test-home root and seeds its own data; on-disk product state is produced by viola, never hand-written · 7-test-data-fixtures · test-data-fixtures
  decision/chaos-home-outside-scan · One chaos home outside the scans — Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design. · why: A ratified carve-out.
  decision/ci-homes-kept-until-gates · CI homes kept until gate steps — In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run. · except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.
  decision/e2e-home-base-checks · Checked base of the test homes — The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.
  decision/e2e-home-headroom-floor · Headroom floor of the e2e home base — A linked e2e home base whose filesystem reads less than the 9 GiB headroom floor free is refused; the floor is one compiled constant shared by the harness keeper and the test home, set by no env, variable, flag, feature or config key; a plain base reads no headroom. · why: The tmpfs's EDQUOT reading of about 8 GiB free plus 1 GiB. · guard: harness::tests::e2e_home_*
  decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
  decision/e2e-homes-under-target-e2e-home · E2E homes under target/e2e-home — Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only. · why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
  decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
  decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link; started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
  decision/keep-failed-home-opt-in · Failed test homes kept only on request — A failing root test keeps its home only when AGENT_RUN_KEEP_FAILED=1. · why: So that logs can inspect the home.
  decision/owner-record-removed-last · Owner record removed last — Removing a test home deletes its owner record last. · why: A removal that stops part-way keeps the record, so a later sweep can still take the dir.
  decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.
  decision/test-home-owner-record-sweep · Owner-record sweep of test homes — Every root test home records its owner pid and start time; a new test home, when no keep flag is set, first removes only dirs whose recorded owner process is gone, never by age or name, never a dir without a record. · why: A test the runner kills never drops its home. · except: A build's verify stamp dir carries no owner record; another build's stamp is swept only when its lock is taken without waiting.
  decision/test-homes-made-private · Test-made homes are private directories — A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation. · why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
  decision/tests-seed-own-data · self-bootstrapping tests — Tests seed their own data; no developer pre-populates a home.
- S3 · each new or re-recorded fixture is synthetic or scrubbed, recorded by its tap or verify, never from real prompts, and passes the fixture hygiene walk · 7-test-data-fixtures · test-data-fixtures
  contract/claude-fixtures/env-names · Recorded Claude fixtures · env names — The recorded env-names fixture Env.recorded-1.json holds names only, never a value, in the shape v 1 plus added; it is recorded by viola verify through the record scrub and checked by its own hygiene class.
  contract/mcp-fixture-scrub-refusal/absolute-path · MCP fixture scrub refusal codes · absolute-path — A closed refusal code of the MCP fixture scrub, naming the file only.
  contract/mcp-fixture-scrub-refusal/email · MCP fixture scrub refusal codes · email — A closed refusal code of the MCP fixture scrub, naming the file only.
  decision/synthetic-fixture-text · synthetic fixture text only — Fixture prompts and checked-in fake agent scenario scripts hold synthetic probe text only, never real prompts.
  decision/synthetic-inputs-with-canary · Synthetic test inputs carry a canary — Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
- S4 · each new test asserts public behaviour by literal oracles, mocks nothing it does not own, and keeps no shared mutable state or process-wide env mutation · 11-test-anti-patterns-never-do-these
  decision/cleanup-between-integration-tests · Cleanup between integration tests — Integration tests never skip cleanup between tests. · why: Order-dependent tests hide bugs.
  decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
  decision/echo-oracle-skips-allowed-word · Echo oracle skips the one allowed word — An oracle asserting that a refusal echoes none of the given words skips the one allowed word when the refusal's fixed line itself names it. · why: Otherwise the fixed line's own mention of the allowed word reads as an echo.
  decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
  decision/no-http-mocking · no HTTP mock library — No HTTP mock library is used. · why: viola makes no outbound HTTP calls.
  decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state.
  decision/real-hook-in-e2e · Real viola hook in E2E — `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
  decision/test-public-behaviour-only · Test public API and observable behaviour — Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state.
- S5 · each new log line is a structured JSON process-log line with the plan's fields, codes only, carrying no secret · log-format
  contract/log-event-corr/channel-request · log event corr · channel-request — corr is the JSON-RPC request id; null for an id-less hook.event notification.
  contract/log-event-corr/channel-response · log event corr · channel-response — corr is the JSON-RPC request id; null on a response to a frame that yielded no id (error_code -32700 or -32600).
  contract/log-event-corr/dialog-answered · log event corr · dialog-answered — corr is the dialog_id.
  contract/log-event-corr/dialog-raised · log event corr · dialog-raised — corr is the dialog_id.
  contract/log-event-corr/hook-decision · log event corr · hook-decision — corr is the dialog_id for dialog hooks, otherwise null; a hook-decision that carries detail may be null.
  contract/log-event-corr/hook-invoked · log event corr · hook-invoked — corr is the dialog_id for dialog hooks, otherwise null; a dialog hook-invoked may still be null when it fails open before the dialog_id is known.
  contract/log-event-corr/panic · log event corr · panic — corr is null.
  contract/log-event-corr/parse-rejected · log event corr · parse-rejected — corr is null.
  contract/log-event-corr/process-exit · log event corr · process-exit — corr is null.
  contract/log-event-corr/process-start · log event corr · process-start — corr is null.
  contract/log-event-corr/release-from-driver · log event corr · release-from-driver — corr is the JSON-RPC request id of a release the wrapper refused with -32602 for a string from; a from of another type is a plain -32602 with no such line.
  contract/log-event-corr/send-confirmed · log event corr · send-confirmed — corr is the send cursor.
  contract/log-event-corr/send-issued · log event corr · send-issued — corr is the send cursor.
  contract/log-event-corr/send-refused · log event corr · send-refused — corr is the send cursor; refused before send-issued, the wrapper line carries the target's events end offset at refusal, a log join key never returned to the caller; the client-side line has corr null.
  contract/log-event-corr/state-recovered · log event corr · state-recovered — corr is null.
  contract/log-files/modes · log files · modes and writes — Files are 0600, dirs 0700, with one write per line.
  contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
  contract/log-line/conn-and-rpc-id · process log line · conn, rpc_id — Wrapper-side send-* lines carry the originating send request's connection and JSON-RPC id, joining them to that call's channel-* lines.
  contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
  contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
  contract/log-line/hook-panic · process log line · hook panic — A panic in a hook is caught and logged as one level ERROR, event panic line.
  contract/log-line/instance · process log line · instance — A ViolaName or null.
  contract/log-line/level · process log line · level — One of DEBUG, INFO, WARN, ERROR.
  contract/log-line/message · process log line · message — fields.message, or the flattened message; required.
  contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
  contract/log-line/process · process log line · process — One of run, hook, mcp, cli; cli is a short-lived verb with a resolved instance, writing its own cli role file.
  contract/log-line/target · process log line · target — A required field.
  contract/log-line/timestamp · process log line · timestamp — RFC 3339 UTC with milliseconds and Z.
  contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
  decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
  decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
  decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
  decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
  decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.
  decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
  decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
  decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.
  decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
  decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
- S6 · each change passes the CI jobs ending in the harness gate: coverage floors, lint, supply-chain and secret scan, with no gate skipped · 9-ci-integration · ci-integration
  contract/ci-artifacts/diag · CI artifacts · diag-${{ matrix.os }} — Holds the E2E homes' diagnostics NDJSON files from the test job, uploaded only behind a passed secret scan.
  contract/ci-artifacts/diag-perf · CI artifacts · diag-perf-${{ matrix.os }} — Holds the perf run homes' diagnostics NDJSON files, uploaded only when the perf job's secret scan succeeded.
  contract/ci-artifacts/harness · CI artifacts · harness-${{ matrix.os }} — Uploaded by the test job on failure after a passed secret scan: the agent-run output minus the mutation diff, with per-suite JUnit, run summary, coverage summary and the failure-only logs and status capture; no perf export.
  contract/ci-artifacts/junit · CI artifacts · junit-${{ matrix.os }} — Holds the one JUnit report of the test job's single instrumented coverage nextest invocation.
  contract/ci-artifacts/perf · CI artifacts · perf-${{ matrix.os }} — Holds the perf job's hyperfine exports, uploaded only when the perf job's secret scan succeeded.
  contract/ci-artifacts/secret-scan · CI artifacts · secret-scan-${{ matrix.os }} — Holds the test job's secret-scan hit report, uploaded only when the scan failed.
  contract/ci-artifacts/secret-scan-perf · CI artifacts · secret-scan-perf-${{ matrix.os }} — Holds the perf job's secret-scan hit report, uploaded only when that scan failed.
  contract/coverage-failure/llvm-cov-exit · coverage suite failure codes · llvm-cov-exit-N — A red llvm-cov exit over green tests, such as a floor breach.
  contract/coverage-failure/llvm-cov-summary-missing · coverage suite failure codes · llvm-cov-summary-missing — The coverage summary was not written.
  contract/gate-breach/coverage · gate breach · coverage — Coverage below a floor, each floor its own breach.
  contract/gate-breach/suite-failed · gate breach · suite-failed — A required suite has failed > 0.
  contract/gate-breach/suite-missing · gate breach · suite-missing — A required suite is absent from the run summary.
  contract/gate-breach/suite-skipped · gate breach · suite-skipped — A required suite has skipped > 0.
  contract/gate-exit-codes/0 · gate exit codes · 0 — Exit 0 with no breaches.
  contract/gate-exit-codes/1 · gate exit codes · 1 — Exit 1 on any breach.
  contract/gate-exit-codes/2 · gate exit codes · 2 — Exit 2 on a usage error.
  contract/secret-scan-class/content-canary · secret-scan hit class · content-canary — A content canary, failing only in a home-level role file.
  contract/secret-scan-class/mode · secret-scan hit class · mode — Unix only: a diagnostics file that is not 0600.
  contract/secret-scan-class/unreadable · secret-scan hit class · unreadable — A file in scope that could not be read.
  contract/secret-scan-exit-codes/1 · secret-scan exit codes · 1 — Exit 1 on any hit.
  contract/secret-scan-exit-codes/2 · secret-scan exit codes · 2 — Exit 2 on a usage error.
  contract/secret-scan-hits-file/hits-json · secret-scan hits file · target/secret-scan/hits.json — Holds the same document as stdout, written only when there are hits; a stale one is deleted on every run.
  decision/artifacts-behind-secret-scan · Artifacts leave CI behind a secret scan — Every test or perf artifact leaves CI only after its job's secret scan passed; when the scan fails only the scan's hit report is uploaded.
  decision/build-fails-below-coverage · Build fails below coverage threshold — The build fails when coverage falls below the quality-gate threshold.
  decision/build-fails-on-fuzz-crash · Build fails on a fuzz replay crash — The build fails when the fuzz corpus replay crashes.
  decision/build-fails-on-perf-breach · Build fails on a performance budget breach — The build fails on any hyperfine performance budget breach.
  decision/build-fails-on-test-failure · Build fails on any test failure — The build fails when any unit, integration, E2E, contract or property test fails.
  decision/cargo-deny-gate · cargo-deny supply-chain gate — Any cargo deny ban, advisory, licence or source finding, including an advisory or source finding on the fuzz lockfile, fails the build. · guard: cargo deny; deny-probes, one probe per ban
  decision/cargo-deny-supply-chain-gate · cargo deny supply-chain gate — The supply-chain job fails closed on cargo deny advisories, licences, sources and bans for the workspace, and on advisories and sources for the fuzz workspace. · guard: CI job supply-chain
  decision/cargo-deny-tokio-and-c-crate-bans · Tokio and C-crate bans — The dependency policy bans tokio where it is not allowed and bans C crates. · guard: cargo deny
  decision/cargo-llvm-cov-coverage · cargo-llvm-cov for coverage — cargo-llvm-cov 0.9.1, installed through taiki-e/install-action, produces coverage as lcov and a JSON summary.
  decision/cargo-modules-orphans-gate · No orphan modules — Every lib and bin target is checked for orphan modules. · guard: cargo-modules orphans gate per lib/bin target
  decision/channel-tokio-free-by-default · viola-channel reaches tokio only by feature — The build fails when viola-channel reaches tokio without its feature enabled. · guard: channel-tokio-free
  decision/ci-actions-sha-pinned · SHA-pinned actions, no default permissions — Every CI action is SHA-pinned, with permissions: {} at the workflow's top level.
  decision/ci-artifact-retention · Seven-day artifact retention — Every CI test artifact is kept for 7 days.
  decision/ci-exit-code-gates · CI exit-code gates — CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check.
  decision/ci-gate-last-step · Gate as the last step of every test job — CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step.
  decision/ci-jobs-end-in-harness-gate · CI test jobs end in the harness gate — Each test-bearing CI job's last step is viola-harness gate requiring that job's own suites.
  decision/ci-lint-gate · CI lint gate — CI lint fails on fmt, clippy warnings, a tokio-free check of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion. · except: The acyclic module review is not a gate.
  decision/ci-scan-before-upload · CI secret scan before any upload — The canary scan runs in CI after each job's end-to-end run over the kept test homes, the harness capture and the junit report, before any upload. · except: The harness chunk.diff and the viola-chaos-* home are out of the scan.
  decision/ci-tools-version-pinned · Version-pinned CI tools — Every tool CI installs is version-pinned; binaries the runner image ships, such as jq and python3, run unpinned.
  decision/ci-uploads-only-after-secret-scan · CI uploads only after the secret scan — On CI failure the home's diagnostics, events and harness artifacts leave CI only in uploads gated on a passed secret scan; the only ungated uploads (the supply-chain reports among them) carry no home content.
  decision/clippy-deny-warnings · Clippy warnings are errors — Clippy runs with warnings denied. · guard: clippy -D warnings
  decision/comprehensive-coverage-floors · Comprehensive-tier coverage floors — viola's coverage must be at least 85% of lines, 80% of branches (enforced as region coverage) and 95% of functions, enforced in the ubuntu test job. · guard: cargo llvm-cov fail-under-lines 85, fail-under-functions 95, fail-under-regions 80
  decision/coverage-exclusions · Coverage exclusions — The fake agent, the E2E crate, test support code and fuzz targets are excluded from coverage by one separator-agnostic filename pattern held in the harness. · why: The separator class is kept from the retired Windows target, where llvm-cov reported paths with backslashes.
  decision/coverage-floors · Coverage floors — CI enforces coverage floors of 85 lines, 95 functions and 80 regions, each floor its own breach, measured with the fake agent, viola-e2e, test support and fuzz trees ignored.
  decision/coverage-ignore-regex-not-widened · Coverage ignore regex is never widened — The coverage ignore-filename regex, covering the fake agent, the E2E crate, test support and fuzz trees, is never widened to pass a gate without a Decisions Log entry first. · why: Widening it is a back door to lowering the coverage thresholds. · except: The path-separator class kept in the regex is not a widening, because the four trees are unchanged.
  decision/coverage-threshold-fails-build · Coverage below tier threshold fails the build — Coverage below the threshold for its tier, measured in the `test` job on `ubuntu-latest`, is a build failure condition.
  decision/coverage-threshold-never-lowered · Coverage threshold never lowered to pass — The coverage threshold is never lowered to pass a build. · why: Raise quality, not the bar.
  decision/coverage-workspace-rollup · Coverage rolled up across the workspace — The coverage number rolls up across all workspace crates.
  decision/deny-bans-probed · Every deny ban has a negative probe — Every cargo deny ban has a permanent negative probe that must fire, beside a clean control. · guard: deny-probes.sh
  decision/doctests-outside-coverage · Doctests outside the coverage number — Doctests are run but are not part of the coverage number.
  decision/doctests-via-cargo-test · Doctests run through cargo test — Doctests run through cargo test --workspace --doc. · why: nextest cannot run doctests
  decision/fmt-and-clippy-deny-warnings · Formatting and clippy warnings fail the build — Any rustfmt difference or clippy warning, with warnings denied, fails the build.
  decision/fuzz-corpus-replay-in-ci · Fuzz corpus replay in CI — Every push and pull request replays the fuzz corpus, and a nightly run fuzzes each target for a bounded time.
  decision/fuzz-corpus-replay-on-push · Fuzz corpus replay on every push — Every push and pull request replays each fuzz target's corpus without fuzzing. · guard: CI job fuzz-replay, gate --require fuzz-replay
  decision/g2-zero-panics · G2 zero panics — CI's log gate G2 fails closed on any event panic role line under the test homes, run as a probe then the check in the test and perf jobs. · except: A panic_location of exactly src/cmd/hook/seam.rs:<digits>.
  decision/harness-gate-fails-on-breach · Harness gate fails on any quality breach — viola-harness gate reads the junit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach.
  decision/jobs-end-in-harness-gate · CI jobs end in the harness gate — Each CI test job ends in viola-harness gate requiring its suites.
  decision/lint-fmt-and-clippy-deny-warnings · Lint denies format drift and clippy warnings — Lint fails on any rustfmt difference and on any clippy warning across the workspace and all targets, the fake-agent feature on. · guard: CI job lint
  decision/lint-gates-fail-build · Lint and audit gates fail the build — A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
  decision/llvm-cov-coverage · cargo-llvm-cov measures coverage — Coverage is measured by cargo-llvm-cov over nextest, producing LCOV and a JSON summary.
  decision/machine-parseable-ci-reports · Machine-parseable CI test reports — CI test reports are machine-parseable and leave CI as uploaded artifacts.
  decision/msrv-1-96 · Minimum supported Rust 1.96 — The workspace is checked and unit-tested on Rust 1.96, its minimum supported version. · why: 1.96 is the security toolchain floor and the requested workspace rust-version; sysinfo needs 1.95. · guard: CI job msrv, gate --require nextest-unit
  decision/msrv-check · MSRV check on the security toolchain floor — An ubuntu CI job checks the workspace and runs the unit suite on Rust 1.96, the minimum supported Rust version. · why: Rust 1.96 is the security toolchain floor.
  decision/no-advisory-ignored · No dependency advisory ignored — CI gates on cargo deny check with no advisory ignored. · except: Ignored with a reason: an advisory only in an rmcp feature banned or off in the release graph, until rmcp leaves <3.5; an unmaintained-upstream advisory with no safe upgrade, re-read each epoch.
  decision/no-ignored-advisories · No advisory ignored — cargo deny ignores no advisory. · guard: CI job supply-chain · except: Ignored with a reason: an advisory only in an rmcp feature banned or off in the release graph, until rmcp leaves <3.5; an unmaintained-upstream advisory with no safe upgrade, re-read each epoch.
  decision/no-orphan-modules · No orphan modules — Any cargo-modules orphan in any library or binary target fails the build. · guard: orphans-check.sh with its probe of two planted orphans
  decision/no-serde-json-unbounded-depth · serde_json unbounded depth banned — The serde_json unbounded-depth feature is banned in deny.toml. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml
  decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.
  decision/no-workflow-concurrency · No workflow concurrency block — No workflow has a concurrency block; zizmor's concurrency-limits finding is declined and stays visible. · why: A concurrency group cancels pending runs even without cancel-in-progress, which would drop a push's always() gate and upload chain.
  decision/one-instrumented-ci-test-run · One instrumented test run in CI — In CI one instrumented coverage run under nextest replaces the unit, integration and e2e runs, followed by the doctest run; the separate runs are the local equivalents. · guard: CI job test
  decision/one-version-source-per-tool · One version source per tool — Each CI tool has exactly one pinned version source; cargo-deny and zizmor run as pinned CLIs, never through their actions.
  decision/per-suite-junit-copies · Per-suite JUnit copies kept — Each suite's JUnit report is kept as its own copy among the run artifacts. · why: Each nextest invocation overwrites the raw JUnit file.
  decision/perf-verdict-from-harness · perf verdict from the harness gate — Perf runs in its own CI job, and its verdict comes from viola-harness gate, never from a jq binary.
  decision/quality-gate-command · One quality gate over the test outputs — The viola-harness gate reads the JUnit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach, failing the build.
  decision/ripgrep-pinned-checksum-verified · ripgrep pinned and checksum-verified — ripgrep is installed from a pinned, checksum-verified official release.
  decision/ripgrep-verified-install · ripgrep for G1 and G3, checksum-verified — ripgrep 15.2.0 with PCRE2 is the G1/G3 gate tool, installed by a script that downloads the official release asset and checks it against its published sha256. · why: taiki-e/install-action has no ripgrep manifest.
  decision/rmcp-feature-limit · rmcp limited to server and transport-io — The build fails when rmcp is absent from viola's release graph or any rmcp feature beyond server and transport-io is requested. · guard: rmcp-features
  decision/rmcp-features-server-transport-io · rmcp takes exactly server and transport-io — rmcp is present in the release graph, and packages other than rmcp request exactly its server and transport-io features. · guard: lint step rmcp features (rmcp-features.sh)
  decision/scan-hits-never-echo-bytes · Scan hits never echo bytes — A secret-scan hit is reported by place and class, never by the matched bytes.
  decision/secret-scan-scope · Secret scan scope — The secret scan covers every diagnostics ndjson under the test homes (detail files included), every file of the harness capture and the nextest CI junit report, and runs in CI before any upload. · why: The run --mutants chunk diff is repository source text that hits the scan's own needles. · except: The run --mutants chunk.diff at the capture root is never scanned and never uploaded; a chunk.diff anywhere else is still scanned.
  decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned from the dependency graph. · guard: deny probe serde_json-unbounded-depth
  decision/sha-pinned-actions · SHA-pinned Actions — Every Action a workflow uses is SHA-pinned, never referenced by a mutable tag. · guard: zizmor unpinned-uses
  decision/single-instrumented-test-run · one instrumented coverage run in CI — CI's test job is one instrumented nextest run that replaces separate unit, integration and E2E runs; the per-suite agent-run runs are its local equivalents.
  decision/supply-chain-ci-gates · Supply chain held by CI gates — The supply chain is held by CI gates: a sync-crate check, an orphans check, cargo deny including the fuzz lockfile, the sole-root tokio ban with ban probes, zizmor, a release check and weekly advisories on both lockfiles.
  decision/supply-chain-fail-closed · fail-closed supply-chain job — One fail-closed job runs cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate, the deny probes and zizmor, uploading their JSON, and a weekly nightly job re-runs the advisories.
  decision/sync-crates-compile-without-features · Sync crates compile alone without features — Each crate on the sync-crates list is checked alone with no features, so without tokio; a crate joins the list with its creation, and an empty list fails. · guard: CI job lint, one cargo check per listed crate
  decision/sync-crates-without-tokio · Sync crates build without tokio — Every listed sync crate builds without tokio and passes the tokio ban; a sync crate failing to build without tokio, or an empty list, fails the build. · guard: cargo check of the sync crates without tokio; deny-sync.toml tokio ban
  decision/test-failure-fails-build · Any test failure fails the build — A test failure at any level is a build failure condition.
  decision/tokio-banned-per-sync-crate · tokio banned from each sync crate — tokio is banned from each crate on the sync-crates list, checked with that crate as sole root. · guard: step Tokio ban per sync crate (sole root)
  decision/viola-channel-tokio-free · viola-channel normal graph is tokio-free — viola-channel's normal dependency graph holds no tokio; its Tokio client sits behind its tokio feature. · guard: channel-tokio-free.sh and deny probe channel-tokio-edge
  decision/weekly-advisories-run · Weekly advisories run — A weekly nightly workflow checks advisories on the root and fuzz lockfiles, and its failure is a build failure.
  decision/weekly-advisory-scan · Weekly advisory scan — Advisories for the workspace and the fuzz workspace are re-checked weekly on schedule and on manual dispatch. · guard: nightly job advisories
  decision/workflows-audited-and-sha-pinned · Workflows audited and Actions SHA-pinned — CI runs zizmor on the workflows and asserts every Action is SHA-pinned, gating on exit code.
  decision/workflows-audited-by-zizmor · Workflows audited by zizmor — The GitHub workflows are audited by zizmor in the supply-chain job. · guard: CI job supply-chain
  decision/zizmor-gate · zizmor findings fail the build — Any zizmor finding on the CI workflows fails the build.
