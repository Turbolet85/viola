# test-plan · the rows that bind 2026-11-17-supply-chain-green-and-booted-tests-end — picked 159 of 1336 (79 by every draw) · rule any · pin 2304

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/no-advisory-ignored · No dependency advisory ignored — CI gates on cargo deny check with no advisory ignored. · except: An advisory that sits only in an rmcp feature banned or off in the release graph is ignored with a reason naming that feature, standing until rmcp leaves <3.5.
decision/private-filesystem-permissions · Private filesystem permissions enforced — A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
contract/instance-snapshot/pid · instance snapshot.json · pid — The wrapper pid lives in the instance's snapshot.json, with started_at and child_pid; there is no standalone pid file.
contract/test-harness/boot · test harness commands · boot — boot builds viola and starts the wrappers in a fresh home to readiness.
contract/test-harness/cleanup · test harness commands · cleanup — cleanup stops every wrapper and deletes the home, idempotently.
contract/unix-socket-permissions/directory · Unix socket permissions · directory — The Unix socket directory is mode 0700; a directory at 0755 or a symlink to a 0700 directory is refused and left as found.
decision/cargo-deny-tokio-and-c-crate-bans · Tokio and C-crate bans — The dependency policy bans tokio where it is not allowed and bans C crates. · guard: cargo deny
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/no-switch-disables-a-control · No switch disables a security control — No environment variable, flag or config key disables a security control.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and the standing live rig's live starts, never in CI.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/sync-crates-without-tokio · Sync crates build without tokio — Every listed sync crate builds without tokio and passes the tokio ban; a sync crate failing to build without tokio, or an empty list, fails the build. · guard: cargo check of the sync crates without tokio; deny-sync.toml tokio ban
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/instance-liveness/gone · instance liveness · gone — An instance whose pid and start time no longer name a running process is gone, whatever the heartbeat's age.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
contract/harness-cleanup/endpoint-gone · Harness cleanup command · endpoint check — Stops every booted wrapper, confirming exit on the process handle, deletes the temp home and verifies the endpoint is gone; it removes no leftover socket file.
decision/events-never-truncated · Events never truncated — Instance events are never truncated; their retention in tests is the lifetime of the temp home.
contract/harness-boot/readiness-failure · Harness boot command · readiness failure — Exits non-zero with a JSON reason on stdout; a run that refuses to start is surfaced as reason run-exited with its instance and the exit code of its process-exit line, never its own text.
contract/harness-cleanup/idempotent · Harness cleanup command · second run — A second cleanup exits 0 and reports nothing to clean.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/fault-injection-sources · Fault injection sources — Chaos faults come from process and file operations and from mockall faults on the viola-pty seam.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/socket-directory-refusal · Loosened socket directory refused — A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.
decision/supply-chain-ci-gates · Supply chain held by CI gates — The supply chain is held by CI gates: a sync-crate check, an orphans check, cargo deny including the fuzz lockfile, the sole-root tokio ban with ban probes, zizmor, a release check and weekly advisories on both lockfiles.
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/ipc-vector-test-layers · IPC vector test layers — The IPC vector is tested at integration and E2E, including an impostor server and a loosened socket directory.
decision/test-only-e2e-crate · Test-only E2E crate — One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root. · why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.

## 3-test-harness-contract · ## 3. Test Harness Contract
contract/harness-session-record/home · harness session record · home — The session record carries the session's home.
contract/harness-session-record/instances · harness session record · instances — The session record lists instances, each with name, wrapper_pid, started_at and fake_args.
contract/harness-session-record/supervisor-pid · harness session record · supervisor_pid — The session record carries the supervisor's pid.
contract/harness-session-record/supervisor-started-at · harness session record · supervisor_started_at — The session record carries the supervisor's start time.
contract/harness-session-record/session · harness session record · session — The session record carries the session id; the default session is default.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/socket-directory-check · Socket directory check — The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes. · guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit
contract/channel-endpoint/socket-path · the wrapper channel endpoint · socket path — The endpoint is viola- and 12 hex of the FNV-1a hash of the name, a NUL and the absolute home, as a socket in the per-user 0700 directory under XDG_RUNTIME_DIR/viola, with /tmp/viola-<uid> as fallback.
decision/server-identity-check · Server identity check — A client checks the serving process against the recorded pid and start time, with a verdict of matches, mismatch or unverifiable; a replayed or absent snapshot gives no endpoint. · guard: server_verdict_rows
decision/socket-dir-checked-before-any-frame · Socket directory checked before any frame — A loosened socket directory is refused inside the verb's or the hook's span before any frame is written. · guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
decision/strict-modes-two-rules · Strict modes write and read rules — Strict modes have a write rule and a read rule, the read rule being the write rule plus a refusal of any group or other permission bit; content-bearing paths are held to the read rule. · guard: unix_read_verdict_needs_the_write_rule_and_no_foreign_bit

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/test-started-viola-drops-env · Test-started viola drops inherited env — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's own path.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/no-untied-viola-spawn · No root test spawns viola untied — No root test spawns viola untied: a census over every test source, recursively, counts any spawn whose path equals or ends in the viola binary constant outside the tied builder, and is proven on a planted positive. · why: A census over one spelling of a constant missed a spawn site; a census over a constant's spawns matches any path ending in the constant. · guard: cross_killed_test_census_finds_no_untied_viola_spawn
decision/private-dir-for-test-endpoints · Test endpoint directories made private — A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir. · why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
decision/test-viola-drops-session-vars · Test-started viola drops session variables — A test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN, and a path oracle compares the resolved home, never the test home's raw path.
decision/tied-child-spawn-thread · Tied children spawned on a thread that outlives them — A tied viola child is spawned only on a thread that outlives it. · why: The parent-death signal follows the spawning thread.
decision/wrapper-drop-guard · Test wrappers held in a Drop guard — Every long-lived viola child a root test spawns is tied to its spawning test thread by a parent-death kill, exiting when its recorded parent is already gone, so a killed or panicked test leaves no wrapper or fake agent; a Drop guard that kills, then waits, still covers the normal end.
contract/verify-stop/signal · viola verify interrupted · SIGINT or SIGTERM — Removes only its own run dirs, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted.
decision/owner-only-socket-modes · Owner-only socket modes — The Unix socket directory is 0700 and the socket file 0600, the file's mode set by a chmod after the bind.
decision/socket-dir-check · Socket directory check before use — The socket directory is checked before use: a directory with a foreign permission bit or a foreign owner, a symlink or a non-directory is refused and left as found, an absent path passes, and the check never chmods. · guard: security_negatives_socket_dir
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — Written on verify's process-exit when verify is stopped by a signal.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/fuzz-separate-workspace · Fuzz targets in a separate workspace — cargo-fuzz targets live in a separate fuzz workspace and join as their parsers land.
decision/mcp-stdio-test-owns-child · MCP stdio test owns the child — MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
decision/socket-dir-refusal · Socket directory refusal — A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named. · guard: security_negatives_socket_dir
decision/start-lines-read-from-start-offset · A start's lines read from its own offset — A reader of a start's first event lines reads that start's own lines, from the offset where the start began, never line 1 of events.ndjson, since a reused instance name keeps the earlier starts' lines at the head of the log. · why: Cursors are byte offsets and the event log is never rewritten.
contract/snapshot/run-fields · the instance snapshot · pinned path, pids and endpoint — The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/viola-e2e-rmcp-client-only · viola-e2e takes rmcp client only — The viola-e2e crate takes rmcp feature client only, as a normal dependency, with no dev-dependencies and no child-process transport.
contract/start-exit-1/socket-dir-refused · run and revive exit 1 causes · socket-dir-refused — The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
contract/viola-run-start/socket-dir-check · viola run start · socket directory check — The socket directory is checked before any snapshot, child or bind, revive included; a refused directory exits 1, logs process-exit with detail socket-dir-refused, binds nothing and spawns no child.
decision/fuzzing-with-cargo-fuzz · cargo-fuzz in a separate workspace — Fuzz targets live in a separate fuzz/ cargo workspace under cargo-fuzz with shared arbitrary inputs and synthetic seeds; each parser joins with its own target.
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/socket-dir-check-never-chmods · Socket directory refused, never repaired — The per-user socket directory is refused when it is a symlink, wider than 0700, not a directory or of another owner, and the check never chmods it. · guard: security_negative_a_start_refuses_the_socket_directory_before_any_bind
contract/event-log/start-wheel-record · instance event log · first record of a start — Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} first and the hook's session-start last; where folder-trust keys pass through the wrapper a wheel{holder:"human", cause:"human-input"} record precedes session-start and a later human key adds none.
contract/exit-21-detail/socket-dir-refused · exit 21 instance-unreachable detail · socket-dir-refused — The socket directory is refused, on the eight frame verbs alone; its own hint; process-exit at WARN with no channel-request; during connect.
contract/hook-decision/socket-dir-refused · hook-decision record · socket-dir-refused — A refused socket directory logs one hook-decision line at WARN with detail "socket-dir-refused", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
contract/instance-snapshot/child-pid · instance snapshot · child_pid — The instance snapshot holds child_pid.
contract/instance-snapshot/started-at · instance snapshot · started_at — The instance snapshot holds started_at.
contract/mcp-frame/socket-dir-refused · MCP frame · socket-dir-refused — A refused socket directory gives instance-unreachable with no line at MCP's frame.
contract/viola-run/child-on-wrapper-kill · viola run wrapper · wrapper killed — When the wrapper is killed, its child agent is gone within a bounded time.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/cleanup-removes-home-record-and-supervisor · Cleanup removes the home, session record and supervisor — Per session, harness cleanup stops the supervisor and kills it and the recorded processes if needed, removes the session home unless homes are kept, and drops the session record. · why: Boot refuses a session id whose record exists, and cleanup of all sessions finds sessions only by their records.
decision/fake-agent-children-own-process-group · fake agent children in their own process group — On Unix every command the fake agent spawns, each hook, runs in a process group of its own. · why: A hangup of the PTY's foreground group, at the agent's exit or when a killed wrapper closes the PTY, would leave a cut-short coverage profile from a viola hook.
decision/fake-agent-exit-ignores-sighup · fake agent exit ignores SIGHUP — On Unix the fake agent ignores SIGHUP from the first step of its exit (\x03 or stdin's end), while a hangup landing before the exit begins still ends it. · why: The agent is its PTY's session leader and cannot leave that group, so a hangup during exit would cut its own exit-time coverage write.
decision/harness-boot-creates-stamps-and-starts · Harness boot makes the home, stamps it and starts wrappers — The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
contract/fake-agent-exit/exit-triggers · viola-fake-agent exit · exit trigger — The fake agent exits on \x03 or at stdin EOF, and exits 1 on Enter on the trust form's No; before it exits it waits for a hook still running and starts no other.
decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
contract/fake-script/gated-step · fake agent turn script · gated step — A gated step waits for one more newline-terminated control line past the byte offset.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/real-process-faults · Process faults are real — A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/cargo-deny-gate · cargo-deny supply-chain gate — Any cargo deny ban, advisory, licence or source finding, including an advisory or source finding on the fuzz lockfile, fails the build. · guard: cargo deny; deny-probes, one probe per ban
decision/rmcp-feature-limit · rmcp limited to server and transport-io — The build fails when rmcp is absent from viola's release graph or any rmcp feature beyond server and transport-io is requested. · guard: rmcp-features
decision/supply-chain-fail-closed · fail-closed supply-chain job — One fail-closed job runs cargo deny over the root and fuzz lockfiles, the sole-root tokio ban per sync crate, the deny probes and zizmor, uploading their JSON, and a weekly nightly job re-runs the advisories.
decision/weekly-advisories-run · Weekly advisories run — A weekly nightly workflow checks advisories on the root and fuzz lockfiles, and its failure is a build failure.
decision/agent-reads-ci-through-gh · Agent reads CI through gh — The agent reads CI job conclusions and fetches artifacts with the gh CLI, and locally gets the same files from agent-run run and agent-run logs.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/cargo-deny-supply-chain-gate · cargo deny supply-chain gate — The supply-chain job fails closed on cargo deny advisories, licences, sources and bans for the workspace, and on advisories and sources for the fuzz workspace. · guard: CI job supply-chain
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
decision/no-ignored-advisories · No advisory ignored — cargo deny ignores no advisory. · guard: CI job supply-chain · except: An advisory that sits only in an rmcp feature banned or off in the release graph is ignored with a reason naming that feature, standing until rmcp leaves <3.5.
decision/rmcp-features-server-transport-io · rmcp takes exactly server and transport-io — rmcp is present in the release graph, and packages other than rmcp request exactly its server and transport-io features. · guard: lint step rmcp features (rmcp-features.sh)
decision/weekly-advisory-scan · Weekly advisory scan — Advisories for the workspace and the fuzz workspace are re-checked weekly on schedule and on manual dispatch. · guard: nightly job advisories
decision/tokio-banned-per-sync-crate · tokio banned from each sync crate — tokio is banned from each crate on the sync-crates list, checked with that crate as sole root. · guard: step Tokio ban per sync crate (sole root)
decision/workflows-audited-by-zizmor · Workflows audited by zizmor — The GitHub workflows are audited by zizmor in the supply-chain job. · guard: CI job supply-chain

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/lint-gates-fail-build · Lint and audit gates fail the build — A lint, clippy, cargo-deny, zizmor or cargo-modules orphans failure is a build failure condition.
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/killed-test-child-uninstrumented · Killed test child is uninstrumented — A test that kills its child straight after the start uses an uninstrumented host program as that child. · why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/cleanup-between-integration-tests · Cleanup between integration tests — Integration tests never skip cleanup between tests. · why: Order-dependent tests hide bugs.
decision/endpoint-from-snapshot-only · Endpoint taken only from the snapshot — Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/chaos-faults-explicit · Chaos faults only from explicit operations — Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically.
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/e2e-home-deletion-through-link · Test-home deletion through the dev-host link — On the Linux dev host, where target/e2e-home is a link, harness cleanup and a TestHome's drop and owner sweep delete outside the working directory, but only dirs the harness or the test side itself created. · why: A boundary widening, ratified.
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
decision/no-nested-harness-in-nextest · No nested cargo run inside nextest — No nextest test shells out to agent-run, a test calls the viola-harness binary only on paths that never build, and in-test sessions skip the build, so there is never a nested cargo run inside nextest.
decision/session-home-retention · Session home retention — Logs are retained for the whole lifetime of the session home; events are never truncated, and cleanup deletes the home except where homes are kept.
decision/supervisor-ordinary-child · Supervisor is an ordinary child — The supervisor is spawned as an ordinary child with no detach flags and outlives boot; the agent-run CLI boot leaves it untied, stopping only through its stop request, while an in-test harness session ties it and its stamping viola verify to the test thread by a parent-death kill.
decision/supervisor-spawned-only-by-boot · Supervisor spawned only by boot — The supervise subcommand is spawned only by boot, as an ordinary child process that outlives boot, never by an agent or directly by a test; it owns the outer PTYs and runs the stop steps when a stop request appears.
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/cleanup-document/home-removed · cleanup document · home_removed — True when the session home was removed; reports "kept" when homes are kept, which is not a failure.
contract/cleanup-document/processes-gone · cleanup document · processes_gone — True when the supervisor, every wrapper and every child are gone, checked by pid and start time; survivors are force-killed first.
contract/harness-session/cleanup-report · harness session · cleanup report — A harness cleanup report has no field false, an interim null allowed, the same shape as the cleanup command.
contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
decision/ci-gate-last-step · Gate as the last step of every test job — CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step.
decision/cleanup-ctrl-c-repeated-until-exit · Cleanup stops agents with a repeated Ctrl-C — Harness cleanup stops each agent child by writing Ctrl-C into its outer PTY and pressing it again every 500 ms until the child exits, then force-kills survivors after a 10 s deadline. · why: A Ctrl-C written into a child that has not set raw mode yet is swallowed.
decision/cleanup-is-idempotent · Idempotent cleanup — Harness cleanup must be idempotent: with no session record it prints {"v":1,"cmd":"cleanup","ok":true,"cleaned":[]} and exits 0.
decision/fuzz-toolchain-single-source · Fuzz toolchain from one source — The fuzz toolchain channel is read from the fuzz workspace's own toolchain file, the one source, and the fuzz dir is a separate cargo workspace excluded from the root.
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
decision/missing-artifact-is-breach · A missing artifact is a breach — A gate input that is missing or unreadable is a breach, never a pass.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
contract/cleanup-document/endpoint-gone · cleanup document · endpoint_gone — True when the recorded wrapper channel endpoint is gone: a connect is refused or the socket path is absent; a leftover socket file no one listens on after a force-killed wrapper reads true, and cleanup removes nothing of it.
contract/cleanup-exit-codes/1 · cleanup exit codes · 1 — Any false field in the cleanup document means exit 1.
contract/harness-boot-exit/1 · harness boot exit codes · 1 — Not ready, with a document of v 1, cmd boot, ok false, a reason, the instance or null, exit_code or null, and the unmet readiness checks in missing.
contract/harness-boot-reason/verify-failed · harness boot refusal reason · verify-failed — One of the four boot refusal reasons.
contract/harness-boot/timeout · harness boot · timeout — Readiness is bounded at 20 s per instance after the build.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/ci-exit-code-gates · CI exit-code gates — CI gates on the exit codes of cargo deny (the root graph and the fuzz lockfile), zizmor, the cargo-modules orphans check per lib and bin target, and the release check.
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/rmcp-client-only · rmcp client only in the harness — The harness uses rmcp with the client feature only, without transport-child-process. · why: The test owns the viola mcp child process.
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail; one verify stamp taken per build keeps each stamped-home test under it.
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.
decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.

## log-format · key of ## 3. Test Harness Contract
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
contract/log-event-corr/process-exit · log event corr · process-exit — corr is null.
contract/log-event-corr/process-start · log event corr · process-start — corr is null.

## pid-file · key of ## 3. Test Harness Contract
decision/pid-start-time-verified-kill · Pid and start time verified before a kill — Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling. · why: A reused pid is never killed.
contract/session-json/lifecycle · harness session.json · lifecycle — Written by boot once readiness passes, read by status, logs and cleanup, removed by cleanup.
contract/session-json/supervisor-pid · harness session.json · supervisor_pid — The supervisor's pid.
contract/session-json/wrapper-pid · harness session.json · wrapper_pid — The wrapper's pid.
decision/no-product-pid-file · No product pid file — The product has no pid file: product pids live in each instance's snapshot with liveness in its heartbeat, and the harness's own process record is its per-session session.json.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
decision/one-verify-stamp-per-build · One verify stamp per build — Stamped test homes copy one viola verify stamp taken per pair of built binaries, kept under target/e2e-home keyed by the binaries' hashes and taken under an exclusive lock. · why: Taking the stamp once keeps every stamped-home test under its unchanged per-test kill. · guard: stamped_home_reuses_one_stamp_for_every_home
decision/stamping-verify-passes-transcript · Stamping verifies give the fake agent a transcript — Every stamping viola verify against the fake agent, in the harness boot and the stamped-home helpers, passes the fake agent --transcript beside --framing. · why: Without a transcript Run B's echo probe is skipped and the local-command-echo row fails. · except: The framing-free verify helper passes no --transcript.
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
