# test-plan · the rows that bind 2026-11-23-seam-requests-for-viola-open — picked 118 of 1374 (72 by every draw) · rule any · pin 4223

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/parser-property-fuzz-coverage · Parser surfaces under property and fuzz tests — Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation.
decision/property-fuzz-for-parsers · Property and fuzz tests for parsers — Property and fuzz generators cover the parser surfaces.
decision/release-build-viola-only · Release build carries viola only — The release build carries viola alone and fails on any test-only feature or test-only binary in it, with no upload and no gate step. · guard: release-check.sh and its probe
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for home, drive-letter and user-folder paths and usernames.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/single-test-only-crate · One test-only workspace member — The workspace has exactly one test-only member, crates/viola-e2e, never published.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/fixtures-never-from-real-prompts · Fixtures never from real prompts — Fixtures must not come from real prompts and must be checked for absolute paths and usernames. · why: A security anti-pattern.
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
decision/fuzz-the-parser-surfaces · Property and fuzz generators on parsers — Property and fuzz generators cover the parser surfaces the security plan lists.
decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
contract/wrapper-channel/max-frame · Wrapper channel · MAX_FRAME — A frame is at most 16 MiB.
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/no-human-in-loop-verification · No human-in-loop verification — Verification has no human-in-loop step, no visual review service and no manual exploratory step.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/property-and-fuzz-layer · Property tests and fuzzing — proptest runs over the parser surfaces with its regressions committed, and cargo-fuzz targets run in a time-boxed nightly job with a corpus replay gate.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.
decision/fault-injection-sources · Fault injection sources — Chaos faults come from process and file operations and from mockall faults on the viola-pty seam.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/property-trigger-layer · Property-test trigger layer — The property-test trigger is met by proptest and the fuzz job.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/test-only-e2e-crate · Test-only E2E crate — One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root. · why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
decision/chaos-fault-sources · Chaos fault sources — Chaos faults come from real process and file operations (log truncation, snapshot corruption, a heartbeat freeze by SIGSTOP, a no-EOF child) plus mockall faults on the PTY seam.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer. · except: The test-only live rig reads the child's screen into closed words, by compiled literals and the box row's cell attributes, never text, and passes a read trust dialog under its key grant.
decision/fuzz-weekly-and-corpus-gate · Fuzzing weekly with a corpus replay gate — cargo-fuzz targets run as a time-boxed weekly job, plus a corpus replay gate, with each parser getting a target as it lands.
decision/single-os-ci-leg · One CI leg on Linux — The CI matrix builds on one leg, ubuntu-latest.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/mocks-only-at-product-seams · Mocks only at product seams — Unit tests use mockall doubles only for traits the product already defines at its seams: Pty, the liveness probe and Clock.
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.
decision/rstest-fixtures-for-constructors · rstest fixtures for shared constructors — Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/bounded-readers-keep-the-cap · Bounded readers keep the frame cap — The version gate drain, the hyprctl reader, the dialog hold and env-keep readers and the config.json default-pair reader keep no more than the frame cap of what they read; a command the cut ends is a failed command. · guard: the BOUNDARIES rows of contract_bounded_readers

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link; started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/placed-copy-execs-at-once · Placed copy closed before it is persisted — A privately placed new copy is persisted only once its write handle is closed, so the copy can be executed right after placement. · why: Executing a copy whose write handle is still open fails with ExecutableFileBusy (ETXTBSY). · guard: place_private_new_placed_copy_execs_at_once

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails. · except: A hook whose payload names the send tool is left out of the dialog tier, so a side sender's hooks shift no position; every other dialog-tier hook is still compared by position, event and bytes.
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/live-failing-reading-not-verification · Live reading recorded as read, never as verification — A live reading that fails a ledger row is recorded as read and never as verification of the row; a confounded live reading leaves the row unverified on that CLI version and its live re-read owed.
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, agents and hyprctl reply parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/fuzzing-with-cargo-fuzz · cargo-fuzz in a separate workspace — Fuzz targets live in a separate fuzz/ cargo workspace under cargo-fuzz with shared arbitrary inputs and synthetic seeds; each parser joins with its own target.
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, agents and Hyprland reply suites.
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/fake-agent-panic-seam-test-only · Forced-panic seam is test-only — The hook forced-panic seam fires only on exactly 1 and compiles only with feature fake-agent, so a release build of viola never contains it. · why: Ratified in the security-plan Decisions Log. · guard: cli_controls_not_disableable
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift · except: A hook whose payload names the send tool is left out of the dialog tier, so a side sender's hooks shift no position; every other dialog-tier hook is still compared by position, event and bytes.
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/dirty-payload-refuses-recording · Dirty payload refuses the recording — A recorded payload, MCP exchange or Hyprland reply set that fails the unclean check refuses the whole recording.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/fixture-hygiene-walk · fixture hygiene run-time walk — A run-time directory walk checks every fixture for home, drive-letter, user-folder paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, validates each file by its JSON schema, and holds each Hyprland reply to its closed per-request string keep-list. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs
decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint or log is shared between tests.
decision/proptest-strategies · proptest for generated data — proptest strategies are what generate property-test data.
decision/recorder-scrub · recorder scrubs home paths and usernames — The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads before writing, and the test-only viola-mcp-tap and viola-hypr-tap apply the same scrub.
contract/fake-agent-cli/argv-only-options · viola-fake-agent command line · modes and options — The fake agent's modes and options are taken from argv and from a fake-args file beside its exe, one argument per line within MAX_FRAME, put ahead of argv so argv's own value wins, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
decision/env-per-child · environment set per child process — Test environment is set per child process with Command::env, never with std::env::set_var. · why: std::env::set_var is unsafe in edition 2024.
decision/injected-clock · injected clock for time-sensitive data — Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes. · except: A link line's since is stamped from the wall clock and asserted by its shape only.
decision/proptest-persisted-seeds · proptest with persisted seeds — Randomized test data comes from proptest strategies, and their seeds are persisted in committed proptest-regressions files.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for home, drive-letter and user-folder paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe; a fake-args file beside its exe goes ahead of argv, argv winning.
decision/proptest-for-randomized-data · proptest for randomized data — Randomized test data comes from proptest strategies, with seeds persisted in committed regression files.
decision/test-clocks-behind-clock-seam · Test clocks behind the clock seam — Time in the sync crates is faked through the core Clock seam with hand-written test clocks; no mocking crate stands behind it.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
decision/mockall-on-seam-traits-only · mockall doubles on seam traits only — Test doubles are mockall automock, under test only, on traits viola already defines at its seams, with dependency injection preferred over runtime patching of production code; external processes are real stub executables.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/real-process-faults · Process faults are real — A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.
decision/time-faked-through-seams · Time faked through injected clocks — Time is faked only through injected seams, with no mock_instant and no paused Tokio clock: the viola_core Clock, the MCP adapter's injected deadline provider, which tests fire by a signal and never by waiting out the 55 s MCP call deadline, and a pure decision takes its instant as an argument.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/no-test-binary-in-release · No test-only binary in a release build — A release build carrying a test-only binary is a build failure condition.
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.
decision/killed-test-child-uninstrumented · Killed test child is uninstrumented — A test that kills its child straight after the start uses an uninstrumented host program as that child. · why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/no-mocking-unowned-code · No mocking of what is not owned — Libraries and the language runtime are never mocked without an abstraction layer.
decision/no-over-mocking-integration · No over-mocking at integration level — Integration tests never over-mock. · why: Over-mocking stops testing the integration.
decision/no-process-env-mutation-in-tests · No process-wide env mutation in tests — Tests never set process environment variables globally; environment is set per child command. · why: Setting the process environment is unsafe in edition 2024 and races across threads.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.
decision/assert-on-contract-not-fixture · Assert on contract, not fixture internals — Assertions target the contract, never brittle fixture-internal values.
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-committed-test-secrets · No test dirs, env files or tokens committed — `--home` test directories, `.env` files and tokens are never committed.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-over-stubbing · No over-stubbing — Tests never over-stub. · why: Over-stubbing stops testing real behaviour.
decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-tokio-failpoint-crate · No failpoint crate pulling tokio — No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/mutants-tested-by-owning-package · Mutants tested by their own package — Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
contract/harness-run-exit/0 · harness run exit codes · 0 — Every selected suite passes, with no timed-out mutants, no missed mutants beyond the host-excluded, and unviable not outnumbering caught.
contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.

## pid-file · key of ## 3. Test Harness Contract
decision/pid-start-time-verified-kill · Pid and start time verified before a kill — Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling. · why: A reused pid is never killed.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/proptest-committed-seeds · proptest with committed seeds — Randomized inputs come from proptest 1.11.0 strategies with committed regression seeds.
decision/run-under-test-owned-user-home · Test runs and revives under a test-owned user home — Every test-started viola run and viola revive runs under a test-owned user home, never the runner's real HOME. · why: The start writes into the user's Claude config.

# test-plan · its absences, names and overviews, whole

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
absence/browser-suite — A browser test suite
absence/windows-and-macos-support — Windows and macOS support: code, tests, fixtures and CI jobs

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
absence/database — A database
absence/outbound-http — Outbound HTTP

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
Scenario: E4 — none in 0.1.0 › N/A — viola has no web or graphical frontend.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
absence/deploy-stage — a deploy stage
