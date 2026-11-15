# test-plan · the rows that bind 2026-11-15-mutation-and-coverage-baselines-green — picked 165 of 1326 (66 by every draw) · rule any · pin 1565

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/hooks-fail-open · Hooks fail open — A hook facing oversize stdin, malformed JSON, an argument error, a channel failure, a panic or a missing VIOLA_NAME exits 0 with empty stderr and no body.
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/plugin-rewritten-on-start · Plugin rewritten on start — The plugin directory is rewritten on every start.
decision/release-build-viola-only · Release build carries viola only — The release build carries viola alone and fails on any test-only feature or test-only binary in it, with no upload and no gate step. · guard: release-check.sh and its probe
decision/test-tier-single-source · Test tier is the single source of rigor — The test tier is the single source of truth for how much test rigor downstream skills apply.
contract/unix-socket-permissions/directory · Unix socket permissions · directory — The Unix socket directory is mode 0700; a directory at 0755 or a symlink to a 0700 directory is refused and left as found.
contract/viola-run/start-refusal-exit-1 · viola run start · exit 1 — A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
decision/chaos-tests-for-self-healing-state · Chaos tests for self-healing state — The crash-safe self-healing state is covered by chaos tests. · why: A driver of the Comprehensive tier.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/comprehensive-test-tier · Comprehensive test tier — The project is tested at the Comprehensive tier (2). · why: Nine attack vectors need negative tests, seven parser surfaces need property and fuzz coverage, and four drivers beyond the Standard baseline push the tier up.
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/pinned-exe-hash-checked · Pinned executable hash checked — A pinned executable whose hash does not match makes the command exit 1.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/single-test-only-crate · One test-only workspace member — The workspace has exactly one test-only member, crates/viola-e2e, never published.
contract/viola-hook/exit-code · viola hook exec-form entry points · exit code — viola hook <event> always exits 0.
contract/viola-hook/stderr · viola hook exec-form entry points · stderr — viola hook never writes to stderr.
contract/viola-hook/stdout · viola hook exec-form entry points · stdout — stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/fixtures-recorded-by-live-verify · Fixtures refreshed by live verify — The recorded agent hook, dialog, framing, resume and screen fixtures are refreshed only by a live viola verify run, locally; the recorded MCP exchange fixtures by the test-only viola-mcp-tap; the recorded Hyprland replies by the test-only viola-hypr-tap.
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
decision/snapshot-replay-fields · Snapshot replay recovers a fixed field set — A corrupt snapshot or one of unsupported v is replayed from the event log, recovering only links, agent_session_id and wheel with dialog_pending false; the replay writes no file and gives no cwd. · guard: replay_recovers, chaos_snapshot_replay
contract/cli-human-lines/stamped · CLI human lines · stamped — stamped <ver>  <n> pass  <n> fail
decision/snapshot-read-through-replay · Every product snapshot reader goes through read-or-replay — Every product reader of the snapshot reads it through the read-or-replay function; no product code calls the plain snapshot read.
contract/hook/impostor-server · Hook · impostor server — A hook whose server is not the recorded wrapper sends no frame and exits 0 with no body.
contract/revive-refusal-code/pinned-hash-mismatch · revive refusal codes · pinned-hash-mismatch — A closed code of the revive --json refusal document.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/integration-suite-per-contract · Integration suite for every standard contract — Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/mutation-run-failure-conditions · Mutation run failure conditions — A mutation run fails on a missed mutant not left out as host-excluded, on a timed-out mutant, or when unviable mutants outnumber caught ones.
decision/rstest-case-tables · Table-driven unit cases in rstest — Table-driven unit cases, including the refusal-order, exit-code and fixture-replay matrices, use rstest case tables, each case named with a readable label. · why: so no manifest edge is added for rstest alone · except: viola-e2e and viola-core carry no rstest dev-dependency; their table-driven cases are labelled literal tables in one test.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/unit-tests-inline · Inline unit tests — Unit tests sit inline in a cfg(test) tests module in the same source file, in every crate and the root binary.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/socket-directory-refusal · Loosened socket directory refused — A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.
decision/test-only-e2e-crate · Test-only E2E crate — One test-only crate, viola-e2e, hosts the harness binary and the tokio-based test clients, and it is never a root of the tokio ban, which runs once per sync crate as the sole root. · why: so the root package and the sync crates carry no tokio dev-dependency; an exclude false-fails on a feature-unified optional tokio
decision/hook-stdin-test-layers · Hook stdin test layer — Hook stdin is tested at integration, through the hook as a subprocess.
decision/ipc-vector-test-layers · IPC vector test layers — The IPC vector is tested at integration and E2E, including an impostor server and a loosened socket directory.
decision/mutation-run-verdict · Mutation run verdict — A mutation run fails on any missed outcome not left out as host-excluded, on any timeout, or when unviable outcomes exceed caught ones. · except: a mutant replacing a client seam's peer-admitted read with an always-admit survives by construction and stands as the version's known limit
decision/mutation-trigger-layer · Mutation trigger layer — Mutation testing is met by the mutation run.
decision/nextest-is-the-runner · nextest is the runner — nextest is the test runner; a plain cargo test run is not supported.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/llvm-cov-coverage · cargo-llvm-cov measures coverage — Coverage is measured by cargo-llvm-cov over nextest, producing LCOV and a JSON summary.
decision/nextest-test-runner · cargo-nextest runs the tests — Tests run on libtest through cargo-nextest, process per test, with JUnit output. · except: Doctests run through cargo test, because nextest cannot run them.
decision/rstest-fixtures-for-constructors · rstest fixtures for shared constructors — Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/socket-directory-check · Socket directory check — The socket directory is refused when a symlink or not a directory and otherwise held to the read rule; an absent path passes and the check never changes modes. · guard: socket_dir_verdict_needs_an_owned_directory_with_no_foreign_bit
contract/channel-endpoint/socket-path · the wrapper channel endpoint · socket path — The endpoint is viola- and 12 hex of the FNV-1a hash of the name, a NUL and the absolute home, as a socket in the per-user 0700 directory under XDG_RUNTIME_DIR/viola, with /tmp/viola-<uid> as fallback.
decision/server-checked-before-any-frame · Server checked before any frame — Every client verb and the hook refuse an unverified server inside their own span with no frame written. · guard: request_refuses_an_unverified_server_inside_its_dispatch_span
decision/session-end-direct-append · SessionEnd falls back to a direct append — A SessionEnd hook whose server check fails appends the session end directly.
contract/readiness-gate/gate-max-wait · the readiness gate · GATE_MAX_WAIT — On a verified CLI a quiet screen on which the input box does not read up (no footer literal and no box rule row directly above a prompt row) waits up to 8 500 ms and reads input-not-ready from that bound on.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
decision/ipc-tested-on-real-endpoint · IPC tested on a real endpoint — IPC integration binds a real endpoint in a temp home with jsonschema-validated frames, creates the squatter first through the server API, never try_overwrite, and reads the Unix socket modes back.
decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.
contract/viola-hook/exit-and-stderr · viola hook · exit and stderr — Every hook event exits 0 with empty stderr.
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/hook-exec-against-live-wrapper · Hook exec tested against a live wrapper — viola hook is fed fixture stdin against a live test wrapper, and its event line and hook trace are asserted.
decision/wrapper-channel-integration-driver · Wrapper channel integration driver — Wrapper channel integration tests drive the viola-channel sync client, a raw interprocess local socket stream for malformed frames, and jsonschema validators built from the committed schema files, which check diagnostics lines while reply frames are compared directly.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/private-dir-for-test-endpoints · Test endpoint directories made private — A test placing an endpoint in a temp directory makes that directory through viola_state's create_private_dir, never a bare tempfile tempdir. · why: A bare temp dir reads 0755 under the umask, and the client's socket directory check refuses it ahead of the server check.
decision/channel-tests-real-endpoint · Channel tests on a real endpoint — Wrapper channel tests bind a real endpoint with the viola-channel server in a temp home and validate frames against JSON schemas.
decision/hook-instance-check-first · Hook instance check comes first — The hook's event and dialog paths run the instance check before any config.json read or diagnostics init; a VIOLA_DIR naming no existing instance directory reads as no session and the hook writes nothing. · guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
decision/hook-tests-real-binary · Hook tests run the real binary — Hook integration tests run the viola binary's hook verb with fixture stdin and VIOLA_NAME / VIOLA_DIR against a live test wrapper, asserting the event line and the hook diagnostics trace.
decision/owner-only-socket-modes · Owner-only socket modes — The Unix socket directory is 0700 and the socket file 0600, the file's mode set by a chmod after the bind.
decision/server-identity-verified · Server identity verified before any frame — A server whose pid or start time does not match snapshot.json gets no frame: the CLI exits 21, the hook exits 0 with no body, and MCP frame tools report instance-unreachable with detail server-verify-failed. · guard: security_negatives_server_identity
decision/snapshot-replayed-from-log · Broken snapshot replayed from the log — A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.
decision/socket-dir-check · Socket directory check before use — The socket directory is checked before use: a directory with a foreign permission bit or a foreign owner, a symlink or a non-directory is refused and left as found, an absent path passes, and the check never chmods. · guard: security_negatives_socket_dir
decision/squatter-via-server-api · Squatter listener through the server API — A test squatter listener binds the computed endpoint before run through the interprocess server API, never through try_overwrite.
contract/verify-exit-detail/interrupted · viola verify process-exit detail · interrupted — Written on verify's process-exit when verify is stopped by a signal.
contract/verify-exit-detail/pinned-hash-mismatch · viola verify process-exit detail · pinned-hash-mismatch — Written on verify's process-exit under a parsed VIOLA_NAME when the pinned copy is tampered.
contract/verify-stop/signal · viola verify interrupted · SIGINT or SIGTERM — Removes only its own run dirs, stdout empty, stderr exactly unable: verify was stopped before it finished / hint: run viola verify again to stamp this CLI, exit 1, no stamps.json, role log ending process-exit detail interrupted.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/hook-fails-open · Hook fails open silently — The hook exits 0 with empty streams and no frame on oversize stdin, malformed JSON, a clap error, a channel failure, a forced panic, a missing VIOLA_NAME, no instance directory, a refused instance or socket directory, or a server that is not the recorded wrapper. · guard: hook_fails_open_silently_within_the_spine_bound, hook_dialog_tier_fails_open_silently
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the wheel and the agent session id included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, the e5_the_replay_reads_ case of cli_links
decision/socket-dir-refusal · Socket directory refusal — A socket directory at 0755 or a symlink refuses viola run exit 1 before any bind, frame verbs exit 21 with no frame and the hook exits 0 with empty streams; the directory is left as found and no path, pid or uid is named. · guard: security_negatives_socket_dir
contract/generated-plugin/hook-commands · the generated plugin · hook commands — A restart rewrites plugin/ with absolute hook commands, never a PATH lookup.
contract/viola-run-exit-codes/1 · viola run exit codes · 1 — A duplicate run, a squatted endpoint and a tampered pinned exe each exit 1.
decision/hook-suites-in-integration-layer · Non-path suites layer — Non-path trigger-coverage suites run in the E2E layer. · except: root hook_* files run in the integration layer
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/start-hands-wheel-to-driver · Start hands the wheel to the driver — Every start appends wheel{holder:"driver", cause:"start"}, so after a restart the wheel is the driver's.
decision/umask-independent-modes · Umask-independent modes — Under any umask, home files are 0600, directories 0700 and the pinned viola binary 0700, never group- or world-writable, and a clean start keeps the socket private. · guard: security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private
contract/wait-and-last/killed-wrapper · viola wait and last · killed wrapper — A killed wrapper gives CLI exit 21 and MCP instance-unreachable.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/socket-dir-check-never-chmods · Socket directory refused, never repaired — The per-user socket directory is refused when it is a symlink, wider than 0700, not a directory or of another owner, and the check never chmods it. · guard: security_negative_a_start_refuses_the_socket_directory_before_any_bind
contract/viola-run-start/plugin-hooks · viola run start · plugin hooks.json — Each start rewrites the instance's plugin hooks.json, its hook commands the absolute pinned_bin path, never a PATH lookup.
contract/viola-run-start/tampered-pinned-exe · viola run start · tampered pinned exe — A run whose pinned executable was altered exits 1.
contract/viola-run/child-on-wrapper-kill · viola run wrapper · wrapper killed — When the wrapper is killed, its child agent is gone within a bounded time.
decision/channel-e2e-driver · Channel tested with the sync client — The wrapper channel is tested with the viola-channel sync client, a raw interprocess stream and jsonschema validation.
decision/channel-endpoint-from-snapshot · Channel endpoint read from the snapshot — Channel tests read the wrapper endpoint only from the instance snapshot.
decision/compact-confirm-window · Compact confirmation window — `/compact` on a verified CLI awaits its post-condition inside its own window, `COMPACT_CONFIRM_WINDOW`, of 40 s, clock-driven; a missed post-condition is `not-delivered`/`no-prompt-submitted` with the cursor. · guard: send_compact_confirm_window_is_its_own_bound · except: `CONFIRM_WINDOW_FALLBACK` stays 10 s for every other confirmed send. · provisional
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/links-survive-wrapper-restart · Links survive a wrapper restart — A wrapper started over a log restores the link set from it, an unlinked pair restored as absent, writes that set into its first snapshot and appends no line. · guard: links_survive_a_wrapper_restart, links_an_unlink_survives_a_wrapper_restart
decision/list-reads-live-through-replay · list reads a live wrapper through the replay — list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line. · guard: e5_list_reads_a_live_wrapper_through_the_replay
decision/pinned-exe-hash-sha2 · sha2 for the pinned-exe content hash — The pinned-exe content hash is SHA-256 from sha2, truncated to 8 bytes as 16 hex. · guard: sha256_published_vector_matches_digest, sha256_truncated_to_eight_bytes_yields_sixteen_hex
decision/replay-writes-no-file · Replay writes no file — The snapshot replay writes no file, so no snapshot is rebuilt from it.
decision/replayed-snapshot-has-no-endpoint · Replayed snapshot has no endpoint or cwd — A replayed snapshot carries no endpoint, pid, started_at or cwd: it gives no live endpoint, hook connect reads the instance unreachable and fails open, and a revive over it is refused cwd-missing. · guard: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/viola-e2e-rmcp-client-only · viola-e2e takes rmcp client only — The viola-e2e crate takes rmcp feature client only, as a normal dependency, with no dev-dependencies and no child-process transport.
contract/hook-decision/socket-dir-refused · hook-decision record · socket-dir-refused — A refused socket directory logs one hook-decision line at WARN with detail "socket-dir-refused", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
contract/instance-snapshot/endpoint · instance snapshot · endpoint — The instance snapshot holds the wrapper channel endpoint.
contract/mcp-wait/timeout-ms · MCP wait · timeout_ms — The bound sent is clamped to 50 000 ms, which is also its default.
contract/start-exit-1/pinned-hash-mismatch · run and revive exit 1 causes · pinned-hash-mismatch — The pinned executable fails its SHA-256 re-hash.
contract/viola-send/compact-confirmed · viola send · /compact on a verified CLI — Confirmed by its post-condition alone, a session-start with cause compact and the same agent_session_id, within its confirmation window, else not-delivered/no-prompt-submitted.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint or log is shared between tests.
decision/tempfile-test-dirs · tempfile for test directories — tempfile is what provides temporary test directories.
decision/test-homes-made-private · Test-made homes are private directories — A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation. · why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
decision/ci-homes-kept-until-gates · CI homes kept until gate steps — In CI each job runs on an ephemeral runner, homes live under the workspace's target/e2e-home and are kept until obs-plan's gate steps (G2, G4, the secret scan and the scan-gated uploads) have run. · except: The viola-chaos home of the chaos feed panic test lives outside target/e2e-home.
decision/dirty-payload-refuses-recording · Dirty payload refuses the recording — A recorded payload, MCP exchange or Hyprland reply set that fails the unclean check refuses the whole recording.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/planted-hygiene-violations · planted inputs prove hygiene classes red — Every hygiene violation class is proven red by a planted input.
decision/recorder-scrub · recorder scrubs home paths and usernames — The viola verify recorder rewrites home paths to ~ and usernames to <user> in hook payloads before writing, and the test-only viola-mcp-tap and viola-hypr-tap apply the same scrub.
contract/verify-recording-refusal/unable-message · viola verify fixture recording refusal · unable message — A dirty screen fixture refuses with `unable: a recorded fixture is not clean: <file> row <n>[ seam] <code>`, code one of home-path, absolute-path, username, email.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
decision/retired-words-as-fixtures · Retired words kept as committed fixtures — The 0.1.0 own-format shapes and the retired words are committed fixtures in a viola-0.1.0 fixture directory outside the identifier-probed paths, read by the cases that must spell an old word, and walked by the fixture hygiene check. · why: A removal's identifier probe and a case that must spell the removed word cannot share a tree.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.
decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/mutation-not-a-ci-gate · mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
decision/no-test-binary-in-release · No test-only binary in a release build — A release build carrying a test-only binary is a build failure condition.
decision/fail-open-spine-bound-at-unit-speed · Fail-open spine bound asserted at unit speed — Each case of the hook fail-open matrix also asserts it completes within the 1.0 s spine bound at unit speed. · guard: hook_fails_open_silently_within_the_spine_bound
decision/hook-connect-deadline · Hook connect deadline below the spine deadline — The hook's own connect deadline is 750 ms, below the spine deadline the gate enforces. · guard: connect_deadline_is_below_the_spine_deadline · provisional
decision/mutation-gate-thresholds · Mutation gate thresholds — A mutation run passes only with missed == 0, timeout == 0 and unviable <= caught, missed read after host-excluded records are left out. · why: A run whose mutant builds mostly failed is no evidence of test strength; the unviable threshold was picked by measurement.
decision/spine-deadline-single-source · Spine deadline from the product constant — The perf gate reads the spine deadline from the product's own constant (1.0 s), never from a literal edited into CI or the test plan.
decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
decision/doctest-only-catch-is-missed · Doctest-only catch counts as missed — A mutant caught only by a doctest counts as missed, and the missing unit test must be added.
decision/killed-test-child-uninstrumented · Killed test child is uninstrumented — A test that kills its child straight after the start uses an uninstrumented host program as that child. · why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.
decision/mutation-harness-tests-ungated · Real cargo-mutants harness tests ungated — The two real-cargo-mutants harness tests run ungated, never ignored, and the CI profile's 120 s kill and zero retries stay unchanged.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/endpoint-from-snapshot-only · Endpoint taken only from the snapshot — Tests never use interprocess `try_overwrite` nor reuse an endpoint name across tests; the endpoint is taken only from `snapshot.json`.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/test-public-behaviour-only · Test public API and observable behaviour — Tests assert the public API and observable behaviour, never implementation details such as private methods or internal state.
decision/coverage-threshold-never-lowered · Coverage threshold never lowered to pass — The coverage threshold is never lowered to pass a build. · why: Raise quality, not the bar.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.
decision/real-hook-in-e2e · Real viola hook in E2E — `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/e2e-home-link-base · E2E home base may be a link — A link as the e2e home base must have an absolute target, created 0700 when gone with one non-recursive create, and is accepted only when it reads a real directory with no group or other bit; a refused base reads build-failed.
decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
decision/e2e-homes-under-target-e2e-home · E2E homes under target/e2e-home — Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only. · why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
decision/mutants-tested-by-owning-package · Mutants tested by their own package — Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/mutation-own-target-dir · Mutation run builds in its own target dir — The mutation run pre-builds the root package and builds every mutant in its own target dir, copied into each scratch tree. · why: a copied default target keeps the original tree's binary paths, so tests drove the unmutated binary
decision/scoped-mutants-not-the-verdict · Scoped mutation is the inner fix loop — A file-scoped mutation run is the inner fix loop; the counted chunk mutation run stays the verdict.
contract/boot-readiness/heartbeat · boot readiness · heartbeat — The heartbeat mtime is less than 5 s old; missing reads heartbeat.
contract/harness-boot/timeout · harness boot · timeout — Readiness is bounded at 20 s per instance after the build.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-fail-fast-wait · Mutants profile stops after one failure and waits — The mutants nextest profile stops scheduling at the first failure and lets the tests already running finish (terminate = wait). · why: Running tests then drop their temp dirs and session guards; immediate termination left leftover temp dirs.
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 10 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/send-window-paste-hint-30s-kill · 30 s mutants kill for send-window classes — Under the mutants profile the send_window_ and send_under_the_paste_hint_ test classes take a 30 s per-test kill. · why: The paste-hint cases wait out the product's 8.5 s gate maximum under a held paste hint, within a second of the 10 s default kill. · guard: mutants_paste_hint_override_holds_the_class_above_the_default_kill
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail.
decision/verify-window-60s-kill · 60 s kill for the verify_window_ class — Tests named verify_window_ take a 60 s per-test kill, in both the ci and mutants profiles, through an override that stands first. · why: Their cases wait out the gate's 8.5 s maximum several times by design. · guard: mutants_verify_window_override_stands_before_the_harness_package_override
decision/viola-e2e-30s-mutants-kill · 30 s mutants kill for the harness package — Under the mutants profile the viola-e2e package takes a 30 s per-test kill. · why: The harness's own tests wait out its 20 s boot deadline by design.
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 7 s is the bound of every test wait not bounded by a per-test kill.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/e2e-home-base-checks · Checked base of the test homes — The test-home base is a plain directory or, on Unix, a link to an absolute target accepted only when one lstat reads a real owner-only directory; a refusal panics with a fixed message naming the failed check and no path.
decision/chaos-home-outside-scan · One chaos home outside the scans — Only the chaos feed-panic tests use a home under the system temp dir, outside target/e2e-home and outside the CI scans by design. · why: A ratified carve-out.
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
