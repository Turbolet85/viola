# test-plan · the rows that bind 2026-11-27-epoch-5-cleanup-2 — picked 194 of 1388 (65 by every draw) · rule any · pin 5179

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cargo-deny-tokio-and-c-crate-bans · Tokio and C-crate bans — The dependency policy bans tokio where it is not allowed and bans C crates. · guard: cargo deny
decision/cargo-modules-orphans-gate · No orphan modules — Every lib and bin target is checked for orphan modules. · guard: cargo-modules orphans gate per lib/bin target
decision/child-exit-read-on-the-process-handle · Child exit is read on the process handle, never on EOF — The pump detects the child's exit by polling the process handle and never by output EOF, then drains the output still in flight for a bounded time. · why: The wrapper's exit code relies on it, and the pump tests use a reader that never ends so only the handle can end the pump.
decision/clippy-deny-warnings · Clippy warnings are errors — Clippy runs with warnings denied. · guard: clippy -D warnings
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify, the standing live rig's live starts and the gated live-reading scripts, never in CI.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/sync-crates-without-tokio · Sync crates build without tokio — Every listed sync crate builds without tokio and passes the tokio ban; a sync crate failing to build without tokio, or an empty list, fails the build. · guard: cargo check of the sync crates without tokio; deny-sync.toml tokio ban
contract/test-harness/boot · test harness commands · boot — boot builds viola and starts the wrappers in a fresh home to readiness.
contract/test-harness/run · test harness commands · run — run runs the unit and integration tests and the fake-agent E2E, plus mutation testing only under run --mutants, and exits 0 only if every suite passes and no mutant survives.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/human-never-blocked · The human is never blocked — The human is never blocked: human typing still reaches the agent while automation is refused. · except: The start's automatic verify window, before any child exists: a key typed then waits in the terminal's own buffer until the child starts.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/no-blanket-approval · No blanket dialog approval — No path auto-approves a dialog without a verify stamp and the driver wheel, proven by a negative test.
decision/r8-claude-env-strip · R8 CLAUDE* environment strip — Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including every identity-floor name. · except: Persistent-environment names listed in config.json claude_env_keep are kept.
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg.
decision/vt100-panic-contained · vt100 panic contained — A forced vt100 panic refuses send with not-delivered/input-not-ready while passthrough continues on the outer terminal: the child's own bytes reach it and no viola literal appears on it.
contract/test-harness/cleanup · test harness commands · cleanup — cleanup stops every wrapper and deletes the home, idempotently.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
decision/run-start-verify-and-trust-after-bind · viola run's verify and trust steps after the bind — Between its exclusive endpoint bind and its first snapshot, a viola run start runs any automatic verify and then, on a verified final verdict, writes an untrusted spawn folder's trust record.
contract/harness-boot/readiness-failure · Harness boot command · readiness failure — Exits non-zero with a JSON reason on stdout; a run that refuses to start is surfaced as reason run-exited with its instance and the exit code of its process-exit line, never its own text.
contract/harness-run/exit · Harness run command · exit — Exits 0 only if every suite passes and no mutant survives; non-zero otherwise, with a JSON summary on stdout of suite, passed, failed and survived.
contract/run-start-events/order · run start events · order — The events log reads wheel with cause start, then lineage with source wrapper, then launch with source wrapper, then, for a start under --origin, one window line with source wrapper, then the agent's session-start with source hook.
decision/snapshot-read-through-replay · Every product snapshot reader goes through read-or-replay — Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
contract/cli-human-lines/unable · CLI human lines · unable — unable  <name>  <reason>  <detail>, followed by a last-line hint:.
contract/harness-cleanup/endpoint-gone · Harness cleanup command · endpoint check — Stops every booted wrapper, confirming exit on the process handle, deletes the temp home and verifies the endpoint is gone; it removes no leftover socket file.
contract/harness-cleanup/idempotent · Harness cleanup command · second run — A second cleanup exits 0 and reports nothing to clean.
contract/revive-json/pump-child-error · revive --json · child pump error — An error while pumping the child prints no document, the child having held the terminal.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/mutation-run-failure-conditions · Mutation run failure conditions — A mutation run fails on a missed mutant not left out as host-excluded, on a timed-out mutant, or when unviable mutants outnumber caught ones.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.
decision/tokio-only-in-e2e-crate · Tokio test code confined to the E2E crate — One test-only, unpublished crate, viola-e2e, hosts the harness and the tokio-based test clients. · why: so the root package and the sync crates carry no tokio dev-dependency
decision/unit-tests-inline · Inline unit tests — Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary.
decision/fault-injection-sources · Fault injection sources — Chaos faults come from process and file operations and from mockall faults on the viola-pty seam.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/mutation-run-verdict · Mutation run verdict — A mutation run fails on any missed outcome not left out as host-excluded, on any timeout, or when unviable outcomes exceed caught ones. · except: a mutant replacing a client seam's peer-admitted read with an always-admit survives by construction and stands as the version's known limit
decision/nextest-is-the-runner · nextest is the runner — nextest is the test runner; a plain cargo test run is not supported.
decision/supply-chain-ci-gates · Supply chain held by CI gates — The supply chain is held by CI gates: a sync-crate check, an orphans check, cargo deny including the fuzz lockfile, the sole-root tokio ban with ban probes, zizmor, a release check and weekly advisories on both lockfiles.
decision/test-only-e2e-crate · Test-only E2E crate — One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root. · why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
decision/chaos-fault-sources · Chaos fault sources — Chaos faults come from real process and file operations (log truncation, snapshot corruption, a heartbeat freeze by SIGSTOP, a no-EOF child) plus mockall faults on the PTY seam.
decision/os-gated-tests-registered · OS-gated tests registered with a reason — Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/doctests-via-cargo-test · Doctests run through cargo test — Doctests run through cargo test --workspace --doc. · why: nextest cannot run doctests
decision/nextest-test-runner · cargo-nextest runs the tests — Tests run on libtest through cargo-nextest, process per test, with JUnit output. · except: Doctests run through cargo test, because nextest cannot run them.
decision/pty-exit-from-wait-not-eof · PTY exit reported from wait, never from reader EOF — viola-pty reports a child's exit from try_wait on the handle, never from reader EOF; consumers are tested with a mockall double of the seven-method seam whose reader never returns EOF while try_wait returns an exit status.
decision/llvm-cov-coverage · cargo-llvm-cov measures coverage — Coverage is measured by cargo-llvm-cov over nextest, producing LCOV and a JSON summary.
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, as hyprctl in a test-scoped bin dir, and as foot, the terminal stand-in.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link; started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
contract/strict-modes-refusal/no-line · strict-modes refusal · no line — A refused check at run, viola revive, viola adopt, viola verify, viola focus, viola up's board scan, the frame verbs, the hook and the MCP frame tools writes no line.
decision/test-started-viola-drops-env · Test-started viola drops inherited env — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.
contract/strict-modes-refusal/21 · strict-modes refusal · 21 — Every other CLI verb exits 21 on a strict-modes refusal, the six frame verbs printing the instance pair.
contract/strict-modes-refusal/run-mcp-revive-verify · strict-modes refusal · run, mcp, viola revive, viola verify — A strict-modes refusal exits 1.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/resize-baseline-is-spawn-size · Pump resize baseline is the spawn size — The pump's resize baseline is the size the PTY was spawned with, never a second host read, so a resize landing before the pump's first look still reaches the child. · why: A resize can land between the spawn sizing and the pump's first look. · guard: pump_forwards_a_resize_that_lands_before_its_first_look
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/home-read-rule · Home readable by another user is refused — A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, and viola list refuses the whole board, opening nothing. · guard: a_home_another_user_may_read_refuses_run
decision/no-untied-viola-spawn · No root test spawns viola untied — No root test spawns viola untied: a census over every test source, recursively, counts any spawn whose path equals or ends in the viola binary constant outside the tied builder, and is proven on a planted positive. · why: A census over one spelling of a constant missed a spawn site; a census over a constant's spawns matches any path ending in the constant. · guard: cross_killed_test_census_finds_no_untied_viola_spawn
decision/no-verify-start-outside-test-home · Every test verify start tied to a test-owned home — A census over the test sources holds every test-started viola verify start tied to a test-owned home, with a closed exempt list whose every entry carries its reason, and is proven red on a planted untied start. · guard: every_test_verify_runs_under_a_test_home
decision/test-viola-drops-session-vars · Test-started viola drops session variables — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
decision/widened-home-tree-refused-at-start · Widened home tree refused at start — viola run and viola verify refuse a home tree widened after a clean start before their log opens, with no child, no step line and the widening left as found. · guard: run_refuses_a_widened_home_tree_before_its_log
contract/verify-exit-detail/untrusted-folder · viola verify process-exit detail · untrusted-folder — Written on verify's process-exit under a parsed VIOLA_NAME when any of Runs B to E first read the trust or imports screen.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/window-reopen-proven-live · Real fleet re-open proven only by the gated live reading R4 — A real re-open of the fleet's windows by viola up is proven only by the gated live reading R4; its R4b phases refuse first until granted, and its rehearse form reads each window from the instance's own window record and is never a live reading. · why: The fake compositor lists no app-id, so a rehearsal cannot read a window by it.
decision/child-env-names-viola-instance · Child given its viola name, instance dir and binary — The agent child is given VIOLA_NAME, VIOLA_DIR (its instance directory) and VIOLA_BIN (the pinned copy path).
decision/child-env-strip · Child environment strip — No stripped environment name reaches the agent child. · guard: the secret scan of the canary values · except: a name listed in claude_env_keep survives
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint · except: boot-time-unread of viola up is held by its unit pair alone, since a host's boot-time source cannot be hidden from a case.
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the wheel, the agent session id and the launch pair included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, the e5_the_replay_reads_ case of cli_links
decision/standing-live-rig · One standing committed live rig — Live rounds against the real claude reuse one committed test-only rig, viola-live-rig in viola-e2e: its rehearsals over the fake agent run in CI, its live starts run by hand under the founder's start cap for each chunk, each ledgered with its clock time first, and its live arm refuses under CI. · why: Ratified as the one rig every later live round reuses. · guard: rig_rehearsal · except: The SF arm's rehearsals run unwrapped in CI; its contained rehearsal is a local entry that skips by its printed sandbox probe where the sandbox or the runtime dir cannot serve it.
decision/umask-independent-modes · Umask-independent modes — Under any umask, home files are 0600, directories 0700 and the pinned viola binary 0700, never group- or world-writable, and a clean start keeps the socket private. · guard: security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private
contract/generated-plugin/hook-commands · the generated plugin · hook commands — A restart rewrites plugin/ with absolute hook commands, never a PATH lookup.
contract/process-start/claude-child · the event log · process-start{subject:"claude-child"} — The record carries the strip counts and the stripped names only, never their values.
contract/viola-run-start/settings · viola run start · settings — A start hands its child no --settings and writes no settings.json in the instance directory; the user's settings are left byte- and mtime-equal, and no default home is made under --home.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/exit-detected-on-handle · Child exit detected on the handle — The wrapper detects the child's exit on the process handle (exit_source "handle-wait"), not on output EOF.
decision/vt100-panic-keeps-passthrough · vt100 panic keeps human passthrough — After a caught vt100 feed panic the screen model reads input-not-ready and drops bytes, send is refused, human passthrough still reaches the child, and the outer terminal shows no panic text. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
decision/window-placement-proven-live · Real window placement proven only live — A real terminal window placed on its parent's workspace, or on the workspace its snapshot recorded when viola up re-opens it, on Hyprland is proven only by a live reading, a placing open's kept view only by the gated live reading R7; the fakes prove the placement words and the window records only.
contract/event-log/start-wheel-record · instance event log · first record of a start — Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} first and the hook's session-start last; where folder-trust keys pass through the wrapper a wheel{holder:"human", cause:"human-input"} record precedes session-start and a later human key adds none.
decision/claude-child-env-strip · claude child environment strip — The claude child receives none of the identity-floor names nor unknown CLAUDE_ names; only names on claude_env_keep survive. · guard: tui_env_strip
decision/claude-child-holds-nothing-of-viola · claude child holds nothing of viola's — The wrapped claude child holds no viola socket and no path under the viola home, and its fds 0 to 2 are the PTY. · guard: tui_channel_fds
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/os-gated-tests-each-named · OS-gated tests each named with a reason — Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named. · guard: os_gated_tests_are_each_named_with_their_reason
decision/revive-json-no-document-after-terminal · No revive document once the child holds the terminal — A pump_child error under revive --json prints no document. · why: The child has held the terminal.
decision/security-sweep-under-nextest · Security sweep runs under nextest alone — The security sweep runs under nextest alone.
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs, starts, viola focus and viola up's board scan take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
decision/tui-outer-pty-driver · Passthrough tested through an outer PTY — The viola run passthrough is tested through a portable-pty outer PTY over the viola-pty seam, its exit read only by waiting on the child.
contract/instance-snapshot/child-pid · instance snapshot · child_pid — The instance snapshot holds child_pid.
contract/instance-snapshot/pinned-bin · instance snapshot · pinned_bin — The instance snapshot holds pinned_bin.
contract/viola-run-passthrough/ctrl-z · viola run passthrough · Ctrl-Z — Ctrl-Z reaches the child, and later keys still do.
contract/viola-run-start/child-after-events · viola run start · child spawn — The child agent is spawned only after the start events are written.
contract/viola-send/turn-running · viola send · turn-running refusal — While a turn runs, or from a switch's request to the new child's session-start, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended, an API-error turn end included, a send is accepted with no pause or release.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl or as foot, the terminal stand-in, by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/harness-boot-creates-stamps-and-starts · Harness boot makes the home, stamps it and starts wrappers — The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint or log is shared between tests.
decision/tests-seed-own-data · self-bootstrapping tests — Tests seed their own data; no developer pre-populates a home.
decision/trust-start-folders-outside-repository · Trust-start test folders outside any repository — Trust-start test folders sit under /tmp, outside any repository, an exception to test homes under target/e2e-home. · why: A folder inside the workspace keys at its git root.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/agents-stub-from-files-no-env · Agents list answered by a stub on PATH — claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH. · why: The no-env answer was ratified.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, the real hyprctl by the same binary as hyprctl, and the real foot terminal by the same binary as foot; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/forced-window-hook-is-test-parameter · Forced-window hook is a test parameter — A forced-window hook passed to a private function under test is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/build-fails-on-test-failure · Build fails on any test failure — The build fails when any unit, integration, E2E, contract or property test fails.
decision/ci-lint-gate · CI lint gate — CI lint fails on fmt, clippy warnings, a tokio-free check of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion. · except: The acyclic module review is not a gate.
decision/single-instrumented-test-run · one instrumented coverage run in CI — CI's test job is one instrumented nextest run that replaces separate unit, integration and E2E runs; the per-suite agent-run runs are its local equivalents.
decision/fmt-and-clippy-deny-warnings · Formatting and clippy warnings fail the build — Any rustfmt difference or clippy warning, with warnings denied, fails the build.
decision/mutation-not-a-ci-gate · mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
decision/no-orphan-modules · No orphan modules — Any cargo-modules orphan in any library or binary target fails the build. · guard: orphans-check.sh with its probe of two planted orphans
decision/supply-chain-fail-closed · fail-closed supply-chain job — One fail-closed job runs cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate, the deny probes and zizmor, uploading their JSON, and a weekly nightly job re-runs the advisories.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/sync-crates-compile-without-features · Sync crates compile alone without features — Each crate on the sync-crates list is checked alone with no features, so without tokio; a crate joins the list with its creation, and an empty list fails. · guard: CI job lint, one cargo check per listed crate
decision/tokio-banned-per-sync-crate · tokio banned from each sync crate — tokio is banned from each crate on the sync-crates list, checked with that crate as sole root. · guard: step Tokio ban per sync crate (sole root)
decision/lint-fmt-and-clippy-deny-warnings · Lint denies format drift and clippy warnings — Lint fails on any rustfmt difference and on any clippy warning across the workspace and all targets, the fake-agent feature on. · guard: CI job lint
decision/mutation-testing-off-ci · Mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
decision/one-instrumented-ci-test-run · One instrumented test run in CI — In CI one instrumented coverage run under nextest replaces the unit, integration and e2e runs, followed by the doctest run; the separate runs are the local equivalents. · guard: CI job test

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/lint-gates-fail-build · Lint and audit gates fail the build — A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
decision/mutation-gate-thresholds · Mutation gate thresholds — A mutation run passes only with missed == 0, timeout == 0 and unviable <= caught, missed read after host-excluded records are left out. · why: A run whose mutant builds mostly failed is no evidence of test strength; the unviable threshold was picked by measurement.
decision/test-failure-fails-build · Any test failure fails the build — A test failure at any level is a build failure condition.
decision/comprehensive-coverage-floors · Comprehensive-tier coverage floors — viola's coverage must be at least 85% of lines, 80% of branches (enforced as region coverage) and 95% of functions, enforced in the ubuntu test job. · guard: cargo llvm-cov fail-under-lines 85, fail-under-functions 95, fail-under-regions 80
decision/coverage-threshold-fails-build · Coverage below tier threshold fails the build — Coverage below the threshold for its tier, measured in the `test` job on `ubuntu-latest`, is a build failure condition.
decision/mutants-verdict-counted-by-harness · Mutation verdict counted by the harness — At a `run --mutants` the harness counts missed, timeout, unviable and caught outcomes itself, leaving host-excluded missed mutants out; any other missed mutant, any timed-out mutant, or more unviable than caught mutants is red. · why: The cargo-mutants exit code and summary cannot be trusted alone: exit 3 masks exit 2, and the summary counts a host-excluded mutant as missed.
decision/no-ignore-parking · No #[ignore] parking — `#[ignore]` is not allowed as a parking place for a failing or flaky test. · why: Agent-driven development cannot tell a flake from a real bug.
decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/doctest-only-catch-is-missed · Doctest-only catch counts as missed — A mutant caught only by a doctest counts as missed, and the missing unit test must be added.
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
contract/mutants-verdict/counted · run --mutants verdict · counted — A closed mutants.verdict value: any other diff naming a .rs path, a mixed one included, is counted; a counted run that produces no fresh outcomes.json is red.
contract/mutants-verdict/unviable-exceeds-caught · Mutation run verdict · unviable-exceeds-caught — The run is red when unviable mutants outnumber caught ones.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-tokio-failpoint-crate · No failpoint crate pulling tokio — No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.
decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.
decision/test-public-behaviour-only · Test public API and observable behaviour — Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
decision/mutants-tested-by-owning-package · Mutants tested by their own package — Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
decision/boot-readiness-file-probe · Boot readiness is a file-state probe — boot polls file state at a bounded 100 ms interval against a deadline, in stages, never by sleep-based synchronisation.
decision/cleanup-ctrl-c-repeated-until-exit · Cleanup stops agents with a repeated Ctrl-C — Harness cleanup stops each agent child by writing Ctrl-C into its outer PTY and pressing it again every 500 ms until the child exits, then force-kills survivors after a 10 s deadline. · why: A Ctrl-C written into a child that has not set raw mode yet is swallowed.
decision/cleanup-is-idempotent · Idempotent cleanup — Harness cleanup must be idempotent: with no session record it prints {"v":1,"cmd":"cleanup","ok":true,"cleaned":[]} and exits 0.
decision/coverage-floors · Coverage floors — CI enforces coverage floors of 85 lines, 95 functions and 80 regions, each floor its own breach, measured with the fake agent, viola-e2e, test support and fuzz trees ignored.
decision/harness-abort-wait-below-ci-kill · Harness abort wait below the ci kill — After a failed readiness the harness boot waits at most 15 s for its abort, so a stuck boot ends within 17 s, below the 20 s ci per-test kill of the harness lifecycle binary; the 20 s readiness bound per instance stays.
decision/host-excluded-mutants · Host-excluded missed mutants — A missed mutant whose whole span sits under a cfg proved false for the build host, by compile-time facts, leaves the missed count; anything the reader cannot read stays counted.
decision/junit-copied-after-each-invocation · JUnit report copied after each nextest invocation — The run command deletes the nextest junit report before each invocation and copies it per suite right after; a report missing afterwards is an artifact-missing breach, never a copy of an earlier run's file. · why: nextest overwrites that path on every invocation.
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/mutants-survived-count · Mutants survived count — The mutants suite's survived count is missed minus host-excluded plus timeout, from the run's own cargo-mutants outcomes.
decision/mutation-base-whole-chunk · Mutation base is the whole chunk — The mutation base is derived from history as the last master flip before the oldest pre-CI commit of a pending record, so every run of a pass mutates the whole chunk; an unresolved base exits 1 base-missing and no empty diff is written. · why: an unbounded search moves mid-pass, and an empty diff reads as a false zero-mutant pass
decision/mutation-verdict-by-counts · Mutation verdict by counts — A mutation run passes only when missed minus host-excluded is zero, no mutant timed out and unviable mutants do not outnumber caught; only exits 0, 2 or 3 can pass, then the counts decide. · why: exit 3 masks exit 2, and more unviable than caught means the run tested almost nothing
decision/supervisor-ordinary-child · Supervisor is an ordinary child — The supervisor is spawned as an ordinary child with no detach flags and outlives boot; the agent-run CLI boot leaves it untied, stopping only through its stop request, while an in-test harness session ties it to the test thread by parent-death SIGKILL and its stamping viola verify by SIGTERM.
decision/supervisor-spawn-strips-bridge-env · Harness wrapper spawn strips the bridge env — The harness supervisor's wrapper spawn removes the inherited VIOLA_NAME, VIOLA_DIR and VIOLA_BIN. · why: so a harness session booted inside a wrapped session records no parent on its instances · guard: harness_session_boots_reports_logs_and_tears_down
decision/supervisor-spawned-only-by-boot · Supervisor spawned only by boot — The supervise subcommand is spawned only by boot, as an ordinary child process that outlives boot, never by an agent or directly by a test; it owns the outer PTYs and runs the stop steps when a stop request appears.
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/boot-readiness-missing/child-process-start · boot readiness missing code · <name>:child-process-start — The child process start record of an instance is missing.
contract/boot-readiness-missing/endpoint · boot readiness missing code · <name>:endpoint — The endpoint of an instance is missing.
contract/boot-readiness-missing/events · boot readiness missing code · <name>:events — The events of an instance are missing.
contract/boot-readiness-missing/heartbeat · boot readiness missing code · <name>:heartbeat — The heartbeat of an instance is missing.
contract/boot-readiness-missing/run-process-start · boot readiness missing code · <name>:run-process-start — The run process start record of an instance is missing.
contract/boot-readiness-missing/snapshot · boot readiness missing code · <name>:snapshot — The snapshot of an instance is missing.
contract/boot-readiness/events · boot readiness · events — Event lines 1, 2, 3 and 4 are a wheel line with cause start, a lineage line with source wrapper, a launch line with source wrapper, then session-start with source hook; missing reads events.
contract/boot-readiness/heartbeat · boot readiness · heartbeat — The heartbeat mtime is less than 5 s old; missing reads heartbeat.
contract/boot-readiness/process-start · boot readiness · process-start — The instance's run log holds a process-start for self and for claude-child, both pids alive by pid and start time; missing reads run-process-start or child-process-start, and a self process-exit first is run-exited.
contract/boot-readiness/snapshot · boot readiness · snapshot — The instance snapshot parses with pid, started_at, child_pid and endpoint in its data; missing reads snapshot, or endpoint when no endpoint.
contract/gate-breach/mutants · gate breach · mutants — The mutants suite has survived > 0.
contract/gate-breach/suite-failed · gate breach · suite-failed — A required suite has failed > 0.
contract/gate-breach/suite-missing · gate breach · suite-missing — A required suite is absent from the run summary.
contract/gate-breach/suite-skipped · gate breach · suite-skipped — A required suite has skipped > 0.
contract/harness-boot-exit/0 · harness boot exit codes · 0 — Every instance is ready within the timeout.
contract/harness-boot-exit/1 · harness boot exit codes · 1 — Not ready, with a document of v 1, cmd boot, ok false, a reason, the instance or null, exit_code or null, and the unmet readiness checks in missing.
contract/harness-boot-reason/readiness-timeout · harness boot refusal reason · readiness-timeout — One of the four boot refusal reasons.
contract/harness-boot-reason/run-exited · harness boot refusal reason · run-exited — The wrapper exited; exit_code is the wrapper's exit code verbatim.
contract/harness-boot/timeout · harness boot · timeout — Readiness is bounded at 20 s per instance after the build.
contract/harness-run-exit/0 · harness run exit codes · 0 — Every selected suite passes, with no timed-out mutants, no missed mutants beyond the host-excluded, and unviable not outnumbering caught.
contract/harness-run-exit/non-zero · harness run exit codes · non-zero — Anything else; survivors, outcomes-missing and unviable-exceeds-caught each make a --mutants run red.
contract/harness-run/default-selection · harness run · default selection — With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default.
contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
contract/run-document/suites · the harness run document · suites — Each entry names its suite and carries passed, failed, skipped, survived, artifact, and failures listing failing test names.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/agent-run-shim-over-harness · agent-run shim over viola-harness — The agent-run script is a shim over viola-harness for the five commands boot, run, status, cleanup and logs.
decision/ci-exit-code-gates · CI exit-code gates — CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check.
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/run-under-test-owned-user-home · Test runs and revives under a test-owned user home — Every test-started viola run and viola revive runs under a test-owned user home, never the runner's real HOME. · why: The start writes into the user's Claude config.
decision/harness-wrappers-under-session-user-home · Harness wrappers under a session user home — The harness's supervised wrappers of a stamped session run under a session user home seeded with an empty config. · why: The start writes into the user's Claude config.
decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.

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
