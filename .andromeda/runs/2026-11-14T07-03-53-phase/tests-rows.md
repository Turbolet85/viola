# test-plan · the rows that bind 2026-11-14-hyprland-seam — picked 219 of 1375 (131 by every draw) · rule any · pin 1006

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cargo-deny-tokio-and-c-crate-bans · Tokio and C-crate bans — The dependency policy bans tokio where it is not allowed and bans C crates. · guard: cargo deny
decision/cargo-modules-orphans-gate · No orphan modules — Every lib and bin target is checked for orphan modules. · guard: cargo-modules orphans gate per lib/bin target
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/ipc-vector-negative-tests · IPC security vector negative tests — The IPC vector is covered by negative tests for oversize frames, higher protocol versions, server pid or start-time mismatch, squatted endpoints, socket permissions, foreign owners, peers of another euid and inherited channel handles.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-switch-disables-a-control · No switch disables a security control — No environment variable, flag or config key disables a security control.
decision/parser-property-fuzz-coverage · Parser surfaces under property and fuzz tests — Property and fuzz tests cover paste-text validation, channel ndjson framing at the MAX_FRAME boundary, the hook stdin parser, the agents listing parser, the vt100 feed and prompt-submitted normalisation.
decision/property-fuzz-for-parsers · Property and fuzz tests for parsers — Property and fuzz generators cover the parser surfaces.
decision/release-build-viola-only · Release build carries viola only — The release build carries viola alone and fails on any test-only feature or test-only binary in it, with no upload and no gate step. · guard: release-check.sh and its probe
decision/scrubbed-fixtures · Fixtures free of real data — Fixtures never come from real prompts and are checked for absolute paths and usernames.
decision/single-test-only-crate · One test-only workspace member — The workspace has exactly one test-only member, crates/viola-e2e, never published.
decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
decision/sync-crates-without-tokio · Sync crates build without tokio — Every listed sync crate builds without tokio and passes the tokio ban; a sync crate failing to build without tokio, or an empty list, fails the build. · guard: cargo check of the sync crates without tokio; deny-sync.toml tokio ban
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
contract/unix-socket-permissions/directory · Unix socket permissions · directory — The Unix socket directory is mode 0700; a directory at 0755 or a symlink to a 0700 directory is refused and left as found.
decision/comprehensive-test-tier · Comprehensive test tier — The project is tested at the Comprehensive tier (2). · why: Nine attack vectors need negative tests, seven parser surfaces need property and fuzz coverage, and four drivers beyond the Standard baseline push the tier up.
decision/cross-user-tested-at-peer-check · Cross-user attacks tested at the peer-credential check — Cross-user peer rejection is asserted at the peer-credential check over injected euids, the real euid read on Linux and the wrapper's drop over an injected decision; no test seam is added, so each client seam's euid wiring has no committed test. · why: Hosted runners provide one account; the trade is that a mutant making a seam's peer-admission read always admit survives every committed test.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/private-filesystem-permissions · Private filesystem permissions enforced — A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
decision/r8-claude-env-strip · R8 CLAUDE* environment strip — Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including the 11-name identity floor. · except: Persistent-environment names listed in config.json claude_env_keep are kept.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, budget.json is seeded through viola hook statusline, stamps come from viola verify against the fake agent, and no state is developer-seeded.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, status line, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap.
decision/fuzz-the-parser-surfaces · Property and fuzz generators on parsers — Property and fuzz generators cover the parser surfaces the security plan lists.
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
contract/wrapper-channel/max-frame · Wrapper channel · MAX_FRAME — A frame is at most 16 MiB.
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, status line, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/statusline-shell-out-frame-cap · Statusline shell-out frame cap — The statusline shell-out keeps exactly the frame cap of a longer stdout, and a command the cut ends is a failed command. · guard: statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude.
decision/fixtures-schema-checked · Recorded fixtures checked against their schema — jsonschema checks every recorded fixture against its schema, each fixture kind against its own schema.
decision/no-human-in-loop-verification · No human-in-loop verification — Verification has no human-in-loop step, no visual review service and no manual exploratory step.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/property-and-fuzz-layer · Property tests and fuzzing — proptest runs over the parser surfaces with its regressions committed, and cargo-fuzz targets run in a time-boxed nightly job with a corpus replay gate.
decision/tokio-only-in-e2e-crate · Tokio test code confined to the E2E crate — One test-only, unpublished crate, viola-e2e, hosts the harness and the tokio-based test clients. · why: so the root package and the sync crates carry no tokio dev-dependency
decision/integration-binary-surface-prefix · Root integration binaries prefixed by surface — Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix. · why: so nextest filtersets can select layers

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/fuzz-weekly-and-corpus-gate · Fuzzing weekly with a corpus replay gate — cargo-fuzz targets run as a time-boxed weekly job, plus a corpus replay gate, with each parser getting a target as it lands.
decision/ipc-vector-test-layers · IPC vector test layers — The IPC vector is tested at integration and E2E, including an impostor server and a loosened socket directory.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/one-shared-test-helper · One shared copy of each test helper — Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
decision/os-gated-tests-registered · OS-gated tests registered with a reason — Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/property-trigger-layer · Property-test trigger layer — The property-test trigger is met by proptest and the fuzz job.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/single-os-ci-leg · One CI leg on Linux — The CI matrix builds on one leg, ubuntu-latest.
decision/supply-chain-ci-gates · Supply chain held by CI gates — The supply chain is held by CI gates: a sync-crate check, an orphans check, cargo deny including the fuzz lockfile, the sole-root tokio ban with ban probes, zizmor, a release check and weekly advisories on both lockfiles.
decision/test-only-e2e-crate · Test-only E2E crate — One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root. · why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
decision/mutation-trigger-layer · Mutation trigger layer — Mutation testing is met by the mutation run.
decision/os-gated-cases-in-binary-root · OS-gated cases stay in a split binary's root file — The OS-gated census walks only files directly under tests/ and crates/*/tests/, so an integration binary split into a module directory keeps every gated case and its gated helpers in its root file. · why: A gated case moved into the module directory would leave its gates row reading as no longer standing. · guard: contract_os_gated_tests
decision/socket-directory-refusal · Loosened socket directory refused — A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/mocks-only-at-product-seams · Mocks only at product seams — Unit tests use mockall doubles only for traits the product already defines at its seams: Pty, the liveness probe and Clock.
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.
decision/rstest-fixtures-for-constructors · rstest fixtures for shared constructors — Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/bounded-readers-keep-the-cap · Bounded readers keep the frame cap — The status line shell-out, the version gate drain and the thresholds, dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command. · guard: the BOUNDARIES rows of contract_bounded_readers
decision/scoped-capture-pinned-dispatch · Scoped log captures through a pinned dispatch — Every scoped log capture in the root binary's unit tests goes through one shared helper that first registers a pinned, always-interested dispatch, never the global default. · why: so no scoped capture is ever the only registered dispatcher and a line whose callsite another, subscriber-less thread registered first is not lost · guard: scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first · except: The three crate-level scoped captures run on a bare scoped default; never observed red, they have no owner.
decision/socket-directory-check · Socket directory check — The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes. · guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit
decision/strict-modes-two-rules · Strict modes write and read rules — Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule. · guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit
decision/one-global-tracing-install · One global tracing install — The binary installs a global tracing default in exactly one place, its observability init; a unit test installs one only as the one global-subscriber test of its process.
decision/socket-dir-checked-before-any-frame · Socket directory checked before any frame — A loosened socket directory is refused inside the verb's or the hook's span before any frame is written. · guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
contract/unreachable-cause/socket-dir-refused · instance-unreachable causes · socket-dir-refused — A refused socket directory is reported with the cause word socket-dir-refused, each cause its own code word.
contract/unreachable-log-cause/socket-dir-refused · the unreachable log cause · socket-dir-refused — Logged during connect when the socket directory is refused.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/ipc-tested-on-real-endpoint · IPC tested on a real endpoint — IPC integration binds a real endpoint in a temp home with jsonschema-validated frames, creates the squatter first through the server API, never try_overwrite, and reads the Unix socket modes back.
decision/controls-not-disableable · Controls not disableable — No setting, whether a VIOLA_* variable, global flag or config.json key, disables a control, the socket directory check among them. · guard: setting_does_not_disable_the_socket_dir_check, every_flag_variable_and_config_key_is_a_row_of_the_table
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
decision/module-integration-direct-calls · Module integration by direct calls — Module-to-module integration is tested by direct calls across crate APIs with shared rstest fixtures.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/owner-only-socket-modes · Owner-only socket modes — The Unix socket directory is 0700 and the socket file 0600, the file's mode set by a chmod after the bind.
decision/private-dir-for-test-endpoints · Test endpoint directories made private — A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir. · why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
decision/socket-dir-check · Socket directory check before use — The socket directory is checked before use: a directory with a foreign permission bit or a foreign owner, a symlink or a non-directory is refused and left as found, an absent path passes, and the check never chmods. · guard: security_negatives_socket_dir
decision/module-boundary-integration-tests · Module boundaries tested by direct calls — The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
decision/no-cli-snapshot-crates · No CLI snapshot crates — No manifest holds assert_cmd, predicates, trycmd or snapbox, and there is no command fixture directory.
decision/placed-copy-execs-at-once · Placed copy closed before it is persisted — A privately placed new copy is persisted only once its write handle is closed, so the copy can be executed right after placement. · why: Executing a copy whose write handle is still open fails with ExecutableFileBusy (ETXTBSY). · guard: place_private_new_placed_copy_execs_at_once
decision/wrapper-drop-guard · Test wrappers held in a Drop guard — Wrappers started in a test are held in a Drop guard that kills the child, then waits for it.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/env-clear-keeps-llvm-profile-file · env_clear tests re-add LLVM_PROFILE_FILE — Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/fixtures-match-tolerant-schema · Fixtures checked against the tolerant schema — jsonschema checks every recorded fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
decision/fuzz-separate-workspace · Fuzz targets in a separate workspace — cargo-fuzz targets live in a separate fuzz workspace and join as their parsers land.
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/no-unbounded-json-depth · No unbounded JSON depth — serde_json's unbounded_depth feature is banned. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml
decision/property-tests-on-parsers · Property tests on every parser surface — proptest at 512 cases covers every parser surface: paste validation, ndjson framing at the frame cap plus and minus one, the hook stdin, statusline and agents parsers, the vt100 feed and prompt-submitted normalisation, with its regressions directory committed.
decision/socket-dir-refusal · Socket directory refusal — A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named. · guard: security_negatives_socket_dir
decision/cross-user-peers-untestable · Cross-OS-user peers skipped as untestable — Cross-OS-user peers are skipped as untestable, covered by the peer-credential boundary unit test and same-user socket modes read-back; the euid wiring at the three client seams is held by no committed test.
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/schema-failure-no-content · Schema failure report without content — A schema failure prints file, line number and the failing schema keyword, never the line's content.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/fuzz-corpora-byte-exact · Fuzz corpora kept byte-exact — Byte-sensitive fuzz corpora are marked binary in .gitattributes so their seeds stay byte-exact.
decision/fuzzing-with-cargo-fuzz · cargo-fuzz in a separate workspace — Fuzz targets live in a separate fuzz/ cargo workspace under cargo-fuzz with shared arbitrary inputs and synthetic seeds; each parser joins with its own target.
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/no-serde-json-unbounded-depth · serde_json unbounded depth banned — The serde_json unbounded-depth feature is banned in deny.toml. · guard: serde_json_unbounded_depth_is_banned_in_deny_toml
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/os-gated-tests-each-named · OS-gated tests each named with a reason — Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named. · guard: os_gated_tests_are_each_named_with_their_reason
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, statusline and agents suites.
decision/socket-dir-check-never-chmods · Socket directory refused, never repaired — The per-user socket directory is refused when it is a symlink, wider than 0700, not a directory or of another owner, and the check never chmods it. · guard: security_negative_a_start_refuses_the_socket_directory_before_any_bind
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/terminal-reply-reading-live-measured · Terminal reply reading measured live — The classifier's reading of terminal replies on a real terminal is a local live measurement, not a suite. · why: The outer-PTY tier writes no terminal reply of its own.
contract/start-exit-1/socket-dir-refused · run and revive exit 1 causes · socket-dir-refused — The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
contract/viola-run-start/socket-dir-check · viola run start · socket directory check — The socket directory is checked before any snapshot, child or bind, revive included; a refused directory exits 1, logs process-exit with detail socket-dir-refused, binds nothing and spawns no child.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/env-per-child · environment set per child process — Test environment is set per child process with Command::env, never with std::env::set_var. · why: std::env::set_var is unsafe in edition 2024.
decision/fixture-hygiene-walk · fixture hygiene run-time walk — One run-time directory walk checks every fixture for absolute paths, non-placeholder usernames, email-shaped tokens and usernames split across a screen row seam, and validates each file against its JSON schema. · why: An rstest files case is not used because rstest refuses at compile time a glob that matches no file. · guard: claude_fixtures in tests/contract_fixture_hygiene.rs · except: A workspace.repo naming this repository's own remote is kept: the StatusLine variants' workspace.repo.owner is not refused.
decision/proptest-persisted-seeds · proptest with persisted seeds — Randomized test data comes from proptest strategies, and their seeds are persisted in committed proptest-regressions files.
decision/proptest-strategies · proptest for generated data — proptest strategies are what generate property-test data.
decision/recorder-scrub · recorder scrubs home paths and usernames — The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads and status line variants before writing, and the test-only viola-mcp-tap fixture applies the same scrub. · except: The StatusLine variants' workspace.repo.owner, the remote owner's handle, is not rewritten by the whole-word OS-user rule.
decision/dirty-payload-refuses-recording · Dirty payload refuses the recording — A recorded payload or MCP exchange that fails the unclean check refuses the whole recording.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/tests-seed-own-data · self-bootstrapping tests — Tests seed their own data; no developer pre-populates a home.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script and Claude fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/foreign-owner-by-injected-uid · Foreign socket-directory owner by injected uid — A socket directory with a foreign owner is not driven in tests; its verdict is taken by an injected uid.
decision/proptest-for-randomized-data · proptest for randomized data — Randomized test data comes from proptest strategies, with seeds persisted in committed regression files.
decision/socket-dir-check-leaves-dir-as-found · Socket-directory check never repairs — The socket-directory check never chmods the directory it checks; a refused socket directory is left as found.
decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and Anthropic usage values by generated statusline stdin; tests never reach the real services.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.
decision/mockall-on-seam-traits-only · mockall doubles on seam traits only — Test doubles are mockall automock, under test only, on traits viola already defines at its seams, with dependency injection preferred over runtime patching of production code; external processes are real stub executables.
decision/no-http-fake-servers · no HTTP fake servers — Fake servers are the interprocess server API binding the squatter before run and viola-fake-agent as the fake CLI; no HTTP fake server is used.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/real-filesystem-faults · Filesystem faults are real — A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found. · except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.
decision/real-process-faults · Process faults are real — A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/build-fails-on-fuzz-crash · Build fails on a fuzz replay crash — The build fails when the fuzz corpus replay crashes.
decision/cargo-deny-gate · cargo-deny supply-chain gate — Any cargo deny ban, advisory, licence or source finding, including an advisory or source finding on the fuzz lockfile, fails the build. · guard: cargo deny; deny-probes, one probe per ban
decision/ci-lint-gate · CI lint gate — CI lint fails on fmt, clippy warnings, a tokio-free check of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion. · except: The acyclic module review is not a gate.
decision/fuzz-corpus-replay-in-ci · Fuzz corpus replay in CI — Every push and pull request replays the fuzz corpus, and a nightly run fuzzes each target for a bounded time.
decision/mutation-not-a-ci-gate · mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
decision/no-orphan-modules · No orphan modules — Any cargo-modules orphan in any library or binary target fails the build. · guard: orphans-check.sh with its probe of two planted orphans
decision/supply-chain-fail-closed · fail-closed supply-chain job — One fail-closed job runs cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate, the deny probes and zizmor, uploading their JSON, and a weekly nightly job re-runs the advisories.
decision/linux-only-ci-matrix · Linux-only CI matrix — CI runs on ubuntu-latest as the one leg of the OS matrix in the test, perf, lint and release jobs, each job and artifact name keeping its matrix.os form, with fail-fast off.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-single-ubuntu-leg · CI runs on one ubuntu-latest leg — CI jobs run on a single ubuntu-latest leg.
decision/deny-bans-probed · Every deny ban has a negative probe — Every cargo deny ban has a permanent negative probe that must fire, beside a clean control. · guard: deny-probes.sh
decision/fuzz-corpus-replay-on-push · Fuzz corpus replay on every push — Every push and pull request replays each fuzz target's corpus without fuzzing. · guard: CI job fuzz-replay, gate --require fuzz-replay
decision/mutation-testing-off-ci · Mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
decision/nightly-fuzzing · Nightly fuzzing — Each fuzz target is fuzzed for 120 seconds nightly, its crash artifacts uploaded on failure. · guard: nightly job fuzz
decision/serde-json-unbounded-depth-banned · serde_json unbounded_depth banned — serde_json's unbounded_depth feature is banned from the dependency graph. · guard: deny probe serde_json-unbounded-depth
decision/sync-crates-compile-without-features · Sync crates compile alone without features — Each crate on the sync-crates list is checked alone with no features, so without tokio; a crate joins the list with its creation, and an empty list fails. · guard: CI job lint, one cargo check per listed crate
decision/tokio-banned-per-sync-crate · tokio banned from each sync crate — tokio is banned from each crate on the sync-crates list, checked with that crate as sole root. · guard: step Tokio ban per sync crate (sole root)
decision/cargo-deny-supply-chain-gate · cargo deny supply-chain gate — The supply-chain job fails closed on cargo deny advisories, licences, sources and bans for the workspace, and on advisories and sources for the fuzz workspace. · guard: CI job supply-chain

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.
decision/lint-gates-fail-build · Lint and audit gates fail the build — A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
decision/no-test-binary-in-release · No test-only binary in a release build — A release build carrying a test-only binary is a build failure condition.
decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
decision/killed-test-child-uninstrumented · Killed test child is uninstrumented — A test that kills its child straight after the start uses an uninstrumented host program as that child. · why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/no-process-env-mutation-in-tests · No process-wide env mutation in tests — Tests never set process environment variables globally; environment is set per child command. · why: Setting the process environment is unsafe in edition 2024 and races across threads.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text and are scrubbed of paths and usernames. · except: A `workspace.repo` naming this repository's own remote is kept.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/no-tokio-failpoint-crate · No failpoint crate pulling tokio — No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.
decision/proptest-seeds-committed · Property tests replay from committed seeds — Property tests replay deterministically: proptest regression seeds persist in committed regression files.
decision/seeded-generators · Seed-controlled generators — Non-deterministic generators are always seed-controlled, and `proptest-regressions/` is committed.
decision/test-public-behaviour-only · Test public API and observable behaviour — Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state.
decision/assert-on-contract-not-fixture · Assert on contract, not fixture internals — Assertions target the contract, never brittle fixture-internal values.
decision/chaos-faults-explicit · Chaos faults only from explicit operations — Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically.
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/cleanup-between-integration-tests · Cleanup between integration tests — Integration tests never skip cleanup between tests. · why: Order-dependent tests hide bugs.
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/endpoint-from-snapshot-only · Endpoint taken only from the snapshot — Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/no-committed-test-secrets · No test dirs, env files or tokens committed — `--home` test directories, `.env` files and tokens are never committed.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/no-mocking-unowned-code · No mocking of what is not owned — Libraries and the language runtime are never mocked without an abstraction layer.
decision/no-over-mocking-integration · No over-mocking at integration level — Integration tests never over-mock. · why: Over-mocking stops testing the integration.
decision/no-over-stubbing · No over-stubbing — Tests never over-stub. · why: Over-stubbing stops testing real behaviour.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/wrapper-spawned-with-guard · Long-lived wrapper spawned with a drop guard — Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call. · why: A blocking call waits until the wrapper exits.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/fuzz-corpus-never-vacuous · Fuzz replay never passes vacuously — A missing or empty corpus, or no target at all, fails corpus-empty, never a vacuous pass.
decision/fuzz-reproducers-committed · Fuzz corpus committed with reproducers — The replay corpus is committed, seeded from recorded payloads or synthetic seeds, and every reproducer is committed with its fix; nothing is cached between CI runs.
decision/members-declare-fake-agent-feature · Mutated members declare fake-agent — Every workspace member but viola-core declares a fake-agent feature, a no-op where it has none. · why: cargo-mutants passes the feature to package-scoped runs, and a missing feature fails the run
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs
decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
decision/fuzz-replay-linux-only · Fuzz replay on Linux only — Fuzz replay runs on Linux only; elsewhere it exits 2 fuzz-linux-only.
decision/fuzz-toolchain-single-source · Fuzz toolchain from one source — The fuzz toolchain channel is read from the fuzz workspace's own toolchain file, the one source, and the fuzz dir is a separate cargo workspace excluded from the root.
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
decision/nightly-fuzz-fails-on-crash · Nightly fuzz fails on any crash — The nightly time-boxed fuzz run uploads its artifacts and fails the job on any crash.
decision/pre-push-local-linux-gate · pre-push local Linux gate — The operator pass runs pre-push, Linux host only, on the uncommitted working tree before the pre-CI commit, and a red stage stops the pass; pre-push is not an agent command.
decision/skipped-counts-real-skips-only · skipped counts real skips only — skipped counts only selected tests not run; tests excluded by a layer filterset or compiled out by an OS cfg are never counted. · why: so the gate's suite-skipped breach fires only on a real skip
decision/status-bounded-read · Harness status reads a bounded stdout — Harness status reads the list output through a bound of one frame past the cap and parses it as one JSON object; with no supervisor record it spawns nothing.
decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
contract/harness-run/default-selection · harness run · default selection — With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default.
contract/harness-run/tool-missing · harness run · tool-missing — Exits 1 with a detail naming the missing tool or file, such as cargo-fuzz.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
contract/pre-push-exit-codes/2 · pre-push exit codes · 2 — Exit 2 on pre-push-linux-only or a usage error.
contract/pre-push-reason/pre-push-linux-only · pre-push reason · pre-push-linux-only — Exit 2.
contract/pre-push-stage/host · pre-push stage · host — Off Linux, exit 2 with pre-push-linux-only.
contract/pre-push-stage/linux-tests · pre-push stage · linux-tests — Runs run --coverage then gate --require coverage,doctest; the last stage, ok:true when green.
contract/run-reason/corpus-empty · run reason · corpus-empty — A closed run reason.
contract/run-reason/fuzz-linux-only · run reason · fuzz-linux-only — Exit 2.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.
contract/run-reason/tool-missing · run reason · tool-missing — run --perf and --fuzz-replay check their tool is on PATH first; when missing they exit 1 with this reason and the tool name, never a silent pass.
contract/secret-scan-exit-codes/1 · secret-scan exit codes · 1 — Exit 1 on any hit.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/ci-exit-code-gates · CI exit-code gates — CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check.
decision/g2-zero-panics · G2 zero panics — CI's log gate G2 fails closed on any event panic role line under the test homes, run as a probe then the check in the test and perf jobs. · except: A panic_location of exactly src/cmd/hook/seam.rs:<digits>.
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill.

## log-format · key of ## 3. Test Harness Contract
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp and cli process in the home's diagnostics.

## pid-file · key of ## 3. Test Harness Contract
decision/pid-start-time-verified-kill · Pid and start time verified before a kill — Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling. · why: A reused pid is never killed.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/proptest-committed-seeds · proptest with committed seeds — Randomized inputs come from proptest 1.11.0 strategies with committed regression seeds.

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
