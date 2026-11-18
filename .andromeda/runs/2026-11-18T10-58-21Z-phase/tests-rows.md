# test-plan · the rows that bind 2026-11-18-session-folder-trusted-by-viola — picked 229 of 1343 (91 by every draw) · rule any · pin 2658

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/no-blanket-approval · No blanket dialog approval — No path auto-approves a dialog without a verify stamp and the driver wheel, proven by a negative test.
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case is checked by a byte search of the outer terminal; the test-only live rig reads the child's screen by compiled literals into closed words its rehearsal asserts on.
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/r8-claude-env-strip · R8 CLAUDE* environment strip — Every inherited CLAUDE* environment name, CLAUDE_CODE_MESSAGING_TOKEN and _SOCKET among them, is stripped from the child, always including the 11-name identity floor. · except: Persistent-environment names listed in config.json claude_env_keep are kept.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and the standing live rig's live starts, never in CI.
decision/unverified-cli-gate · Unverified CLI version gate — On a claude CLI version not in the ledger stamps, viola still types, runs the wheel and emits events, but withholds dialog answers and refuses drivers with unverified-cli. · why: A non-null dialog decision needs a viola verify stamp.
contract/viola-run/start-refusal-exit-1 · viola run start · exit 1 — A second run of the same name, a squatted endpoint, a refused socket directory, a tampered pinned exe or a refused home tree makes viola run exit 1, before any child spawn.
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/ledger-row-verify-probe · Every ledger row has a verify probe — Every ledger row has a viola verify probe with a post-condition, run locally, reporting an exit code and a stamped/pass/fail line.
decision/property-fuzz-for-parsers · Property and fuzz tests for parsers — Property and fuzz generators cover the parser surfaces.
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough · except: The test-only live rig reads the child's screen by compiled literals only into closed words, which its rehearsal asserts on.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/run-checks-before-child-spawn · viola run checks before the child — viola run checks the home tree before its log opens, then the name collision, pinned copy, version gate and socket directory, binds its endpoint exclusively and writes its first snapshot, heartbeat and start events, and only then spawns the child. · guard: run_refuses_a_widened_home_tree_before_its_log
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
contract/revive-exit-codes/1 · revive exit codes · 1 — A refusal: the strict-modes check ahead of the log, a preflight reading or a refusal of the start.
contract/run-refusal/already-live · run and revive start refusals · already-live — A live holder or a fresh replayed beat refuses the start with exit 1.
contract/run-refusal/socket-dir-refused · run and revive start refusals · socket-dir-refused — A refused socket directory exits 1 with no bind and no child, the directory left as found, and one process-exit line of detail socket-dir-refused.
contract/run-refusal/stale-heartbeat · run and revive start refusals · stale-heartbeat — A stale holder refuses the start with exit 1.
contract/run-start-events/order · run start events · order — The events log reads wheel with cause start, then the agent's session-start with source hook.
contract/wrapper-channel/max-frame · Wrapper channel · MAX_FRAME — A frame is at most 16 MiB.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/fault-injection-sources · Fault injection sources — Chaos faults come from process and file operations and from mockall faults on the viola-pty seam.
decision/integration-binary-surface-prefix · Root integration binaries prefixed by surface — Root integration binaries use a <surface>_<topic>.rs prefix, the surface prefix naming the layer; crate-local integration binaries need no surface prefix. · why: so nextest filtersets can select layers
decision/integration-suite-per-contract · Integration suite for every standard contract — Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
decision/no-human-in-loop-verification · No human-in-loop verification — Verification has no human-in-loop step, no visual review service and no manual exploratory step.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/shared-e2e-helpers · Shared E2E helpers never copied — The sync E2E suites in the root share one set of helpers, which no root test file copies.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/chaos-trigger-layer · Chaos-test trigger layer — The chaos-test trigger is met by a chaos suite.
decision/child-spawn-test-layers · Child spawn test layers — Child spawn is tested at integration and E2E.
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer. · except: The test-only live rig reads the child's screen by compiled literals into closed words, and passes a read trust dialog under its key grant.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/filesystem-test-layers · Filesystem test layers — The filesystem vector is tested at integration and chaos, including strict-mode refusals.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
decision/chaos-fault-sources · Chaos fault sources — Chaos faults come from real process and file operations (log truncation, snapshot corruption, a heartbeat freeze by SIGSTOP, a no-EOF child) plus mockall faults on the PTY seam.
decision/cross-surface-trigger-layer · Cross-surface trigger layer — Cross-surface coordination is tested at E2E, with cross-surface parity checks.
decision/e2e-layer-by-binary-prefix · E2E layer selected by binary prefix — The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
decision/home-another-user-may-read-refused · Readable home refused — A home another user may read refuses run and viola list, and a start under umask 000 makes every home path owner-only.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/mutation-trigger-layer · Mutation trigger layer — Mutation testing is met by the mutation run.
decision/nextest-is-the-runner · nextest is the runner — nextest is the test runner; a plain cargo test run is not supported.
decision/one-shared-test-helper · One shared copy of each test helper — Root tests share the events reader and wait, the child guard and runner, the freeze guard and the result struct from the test support module; no root test file defines its own copy.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/socket-directory-refusal · Loosened socket directory refused — A socket directory at 0755 or a symlink to a 0700 directory refuses a start, every CLI frame verb and the hook, each left as found.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/llvm-cov-coverage · cargo-llvm-cov measures coverage — Coverage is measured by cargo-llvm-cov over nextest, producing LCOV and a JSON summary.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/bounded-readers-keep-the-cap · Bounded readers keep the frame cap — The version gate drain, the hyprctl reader and the dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command. · guard: the BOUNDARIES rows of contract_bounded_readers
decision/claude-env-strip · Agent identity environment stripped — Every inherited CLAUDE-prefixed variable is removed by exact name unless in the persistent keep set, and the eleven-name identity floor is removed whatever the keep set says; the floor stays eleven names. · guard: an R8 test against a literal copy of the floor, never imported from the product
decision/version-gate-needs-all-relied-rows · Version gate needs every relied row — A CLI version reads verified only when its stamp holds every relied ledger row as pass, a retired row it also holds changing nothing; a stamp of only older row ids reads unverified. · guard: stamps_verdict_needs_the_dialog_rows
contract/code-word/format · code words · format — A code word is 1 to 64 bytes of lower-case letters, digits and hyphens, starting with a lower-case letter.
decision/bounded-compare · Bounded compare of a shared target — Comparing a target to known bytes reads no more than one byte past their length, never the whole file. · guard: replace_private_shared_reads_no_more_than_one_byte_past_the_bytes
decision/screen-signatures-only · Screen read only for compiled signatures — The screen is evaluated only for compiled input-box and modal literals as the readiness gate's verdict; content beyond the compiled literals is never asserted.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link; started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
decision/on-disk-state-tested-on-real-files · On-disk state tested on real files — Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object.
decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.
contract/strict-modes-refusal/no-line · strict-modes refusal · no line — A refused check at run, viola revive, viola verify, the frame verbs, the hook and the MCP frame tools writes no line.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/no-verify-start-outside-test-home · Every test verify start tied to a test-owned home — A census over the test sources holds every test-started viola verify start tied to a test-owned home, with a closed exempt list whose every entry carries its reason, and is proven red on a planted untied start. · guard: every_test_verify_runs_under_a_test_home
contract/strict-modes-refusal/revive · Strict-modes refusal · viola revive — Exit 1 on the start arm and --list alike, with the state-files unable / hint pair or under --json the strict-modes-failed document on stdout; no log line, no child, the home's mode as found.
contract/strict-modes-refusal/run · Strict-modes refusal · viola run — Exit 1 with the home or instance pair, made before the wrapper log opens: no line and no child.
decision/no-names-in-wrapper-log · No session names in the wrapper log — The wrapper's process log carries no driver or driven name, and channel lines carry no name or param. · guard: link_verbs_log_the_channel_pair_and_no_names
decision/no-untied-viola-spawn · No root test spawns viola untied — No root test spawns viola untied: a census over every test source, recursively, counts any spawn whose path equals or ends in the viola binary constant outside the tied builder, and is proven on a planted positive. · why: A census over one spelling of a constant missed a spawn site; a census over a constant's spawns matches any path ending in the constant. · guard: cross_killed_test_census_finds_no_untied_viola_spawn
decision/on-disk-state-integration-tests · On-disk state integration tests — Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests. · why: The project has no database.
decision/tied-child-spawn-thread · Tied children spawned on a thread that outlives them — A tied viola child is spawned only on a thread that outlives it. · why: The parent-death signal follows the spawning thread.
decision/verify-failing-row-no-refusal · Failing ledger row is no refusal — A failing ledger row in viola verify is no refusal and carries no code.
decision/widened-home-tree-refused-at-start · Widened home tree refused at start — viola run and viola verify refuse a home tree widened after a clean start before their log opens, with no child, no step line and the widening left as found. · guard: run_refuses_a_widened_home_tree_before_its_log
decision/wrapper-drop-guard · Test wrappers held in a Drop guard — Every long-lived viola child a root test spawns is tied to its spawning test thread by a parent-death kill, exiting when its recorded parent is already gone, so a killed or panicked test leaves no wrapper or fake agent; a Drop guard that kills, then waits, still covers the normal end.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/child-env-strip · Child environment strip — No stripped environment name reaches the agent child. · guard: the secret scan of the canary values · except: a name listed in claude_env_keep survives
decision/env-clear-keeps-llvm-profile-file · env_clear tests re-add LLVM_PROFILE_FILE — Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
decision/error-payloads-sanitized · Error payloads sanitized — No error payload carries an absolute path, a serde path or a Caused by chain. · guard: tests/contract_error_bodies.rs census of writers
decision/live-failing-reading-not-verification · Live reading recorded as read, never as verification — A live reading that fails a ledger row is recorded as read and never as verification of the row; a confounded live reading leaves the row unverified on that CLI version and its live re-read owed.
decision/standing-live-rig · One standing committed live rig — Live rounds against the real claude reuse one committed test-only rig, viola-live-rig in viola-e2e: its rehearsals over the fake agent run in CI, its live starts run by hand under the founder's start cap for each chunk, each ledgered with its clock time first, and its live arm refuses under CI. · why: Ratified as the one rig every later live round reuses. · guard: rig_rehearsal
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/home-readable-by-others-refused · Home readable by others refused — A pre-existing home another user may read refuses viola run with exit 1 and viola list with exit 21. · guard: list_refuses_a_home_another_user_may_read_with_and_without_an_instance
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the wheel and the agent session id included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, the e5_the_replay_reads_ case of cli_links
decision/start-lines-read-from-start-offset · A start's lines read from its own offset — A reader of a start's first event lines reads that start's own lines, from the offset where the start began, never line 1 of events.ndjson, since a reused instance name keeps the earlier starts' lines at the head of the log. · why: Cursors are byte offsets and the event log is never rewritten.
contract/viola-run-start/settings · viola run start · settings — A start hands its child no --settings and writes no settings.json in the instance directory; the user's settings are left byte- and mtime-equal, and no default home is made under --home.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/claude-child-env-strip · claude child environment strip — The claude child receives none of the identity-floor names nor unknown CLAUDE_ names; only names on claude_env_keep survive. · guard: tui_env_strip
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/error-bodies-hold-no-path · Error bodies hold no path or upstream text — CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by, checked on each surface and by an enumeration of error documents with a census of writers. · guard: cli_error_documents_hold_no_path_or_upstream_text, channel_error_bodies, contract_error_bodies
decision/revive-preflight-order · revive preflight order — revive takes the strict-modes check first, then holder, session, session-live and recorded directory in that order, a later reading not taken once an earlier one refuses.
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
decision/thiserror-messages-interpolate-nothing · thiserror messages interpolate nothing — thiserror error messages interpolate no value. · guard: thiserror_messages_interpolate_nothing
contract/event-log/start-wheel-record · instance event log · first record of a start — Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} first and the hook's session-start last; where folder-trust keys pass through the wrapper a wheel{holder:"human", cause:"human-input"} record precedes session-start and a later human key adds none.
contract/viola-run-start/child-after-events · viola run start · child spawn — The child agent is spawned only after the start events are written.
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/error-writers-each-named · Every product error writer is named — Every product error writer is named in the error-body enumeration with a writer and a witness that stand. · guard: product_error_writers_are_each_named
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/os-gated-tests-each-named · OS-gated tests each named with a reason — Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named. · guard: os_gated_tests_are_each_named_with_their_reason
decision/outer-pty-reader-drained · Outer PTY reader drained on its own thread — An outer-PTY test drains the master reader on its own thread.
decision/property-tests-with-proptest · proptest for property tests — Input parsers and validators are property-tested with proptest at 512 cases; proptest-regressions files are committed for the viola-core, hook, agents and Hyprland reply suites.
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
decision/tui-outer-pty-driver · Passthrough tested through an outer PTY — The viola run passthrough is tested through a portable-pty outer PTY over the viola-pty seam, its exit read only by waiting on the child.
decision/unverified-cli-gates-answer-only · Unverified CLI version gates answer only — Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
contract/cli-human/internal-error · CLI human output · internal error line — Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, mcp and hook stay silent there.
contract/process-start-claude-child/env-kept · process-start{subject:"claude-child"} record · env_kept — Carries the environment names kept for the claude child, names only, never values.
contract/revive-json/refusal-document · revive --json · refusal document — An exit-1 refusal of the strict-modes check, the preflight or the start it shares with run prints {"v":1,"error":"<code>","detail":null} on stdout with empty stderr; without --json the human pair is unchanged.
contract/start-exit-1/already-live · run and revive exit 1 causes · already-live — The name is held by a live wrapper or a fresh replayed beat; the hint is viola list.
contract/start-exit-1/pinned-hash-mismatch · run and revive exit 1 causes · pinned-hash-mismatch — The pinned executable fails its SHA-256 re-hash.
contract/start-exit-1/socket-dir-refused · run and revive exit 1 causes · socket-dir-refused — The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
contract/start-exit-1/squatted-name · run and revive exit 1 causes · squatted-name — The endpoint name is squatted.
contract/start-exit-1/stale-heartbeat · run and revive exit 1 causes · stale-heartbeat — The holder's pid is live but its heartbeat is older than 5 s; stale heartbeat is exit 1's cause alone.
contract/start-exit-1/strict-modes-failed · run and revive exit 1 causes · strict-modes-failed — The home or instance state could be written, or read, by another user; no log opened, no line, no child.
contract/verify-output/last-line · viola verify output · last line — A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
contract/viola-run-start/duplicate-run · viola run start · duplicate run — A second run of an instance already running exits 1.
contract/viola-run-start/home-tree-check · viola run start · home-tree strict-modes check — The home-tree check runs before the wrapper log opens; a refused tree exits 1 with the home-state or instance-state unable/hint pair, opens no log, writes no line and starts no child.
contract/viola-run-start/socket-dir-check · viola run start · socket directory check — The socket directory is checked before any snapshot, child or bind, revive included; a refused directory exits 1, logs process-exit with detail socket-dir-refused, binds nothing and spawns no child.
contract/viola-run-start/squatted-endpoint · viola run start · squatted endpoint — A run whose channel endpoint is already held by another listener exits 1.
contract/viola-run-start/tampered-pinned-exe · viola run start · tampered pinned exe — A run whose pinned executable was altered exits 1.
contract/viola-wait/non-waking-records · viola wait · activity and wheel records — Activity and wheel records, and a harness-origin prompt-submitted, never wake a parked wait.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/harness-boot-creates-stamps-and-starts · Harness boot makes the home, stamps it and starts wrappers — The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
contract/fake-agent-cli/argv-only-options · viola-fake-agent command line · modes and options — The fake agent's modes and options are taken from argv only, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
contract/fake-agent-cli/cli-version · viola-fake-agent command line · --cli-version — The fake agent answers --version in the real claude CLI's format for the version given by --cli-version.
contract/fake-agent-cli/trust-config · viola-fake-agent command line · --trust-config — The fake agent reads a trust record from the named config file by its own key rule, the nearest ancestor holding a git entry else the canonical cwd, through a bounded read; any failure reads untrusted.
contract/fake-agent-events/session-start · viola-fake-agent fired events · SessionStart — After start_receipts the fake agent fires SessionStart (variant default) once, silent without a registered hook or a fixture, like every fire; under --screens with an untrusted cwd only after Enter on the trust row, which draws the ready screen.
contract/fake-agent-screens/trust-dialog · viola-fake-agent screens · trust dialog — Under --screens with an untrusted cwd the fake agent draws claude 2.1.325's unnumbered trust form over the set's recorded modal, the selector first on No, exit directly above the recorded trust row (row 16 with no recorded modal); arrows move the selector, and Enter on No exits 1.
decision/env-per-child · environment set per child process — Test environment is set per child process with Command::env, never with std::env::set_var. · why: std::env::set_var is unsafe in edition 2024.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
contract/fake-agent-exit/exit-triggers · viola-fake-agent exit · exit trigger — The fake agent exits on \x03 or at stdin EOF, and exits 1 on Enter on the trust form's No; before it exits it waits for a hook still running and starts no other.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/agents-stub-from-files-no-env · Agents list answered by a stub on PATH — claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH. · why: The no-env answer was ratified.
decision/error-body-check-reports-place-and-class · Error-body check reports place and class, never bytes — The error-body check reads a hit of a planted value as class planted, ahead of absolute-path, caused-by and serde-path, and its failure names the place and the class and never the bytes it read. · why: A failing witness must print no canary into JUnit, which the secret scan reads.
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume, its transcript_path under --transcript, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
decision/harness-cli-version-matches-fake · One default CLI version — The harness boot's default CLI version equals the fake agent's default reported version, which is the recorded fixture set's version.
decision/screen-fixture-box-rows-as-literals · Box rows recorded as their literals alone — A recorded screen fixture keeps a box rule row as its rule chars alone and the box's prompt row under that rule as the prompt char alone, and the fixture hygiene walk refuses a prompt row with text after the prompt char or a rule row with any other text. · guard: planted_box_row_beyond_its_literal_is_rejected
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/fake-agent-receipt/cwd · Fake-agent receipt · cwd — Carries cwd.
contract/fake-agent-receipt/fds · Fake-agent receipt · fds — Carries fds.
contract/fake-agent-receipt/hold · Fake-agent receipt · hold — Carries pid, written by the --exit-no-eof holder at its start.
contract/fake-agent-receipt/hook · Fake-agent receipt · hook — Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.
contract/fake-agent-receipt/key · Fake-agent receipt · key — Carries hex, one line per byte received outside a paste.
contract/fake-agent-receipt/line-format · Fake-agent receipt · line format — Receipt lines are ndjson written to the --receipt path, one write per line, each line carrying "v":1 and a kebab kind.
contract/fake-agent-receipt/prompt · Fake-agent receipt · prompt — Carries text, hex, bare_esc, origin human or harness, and submit fired, suppressed, local-command, no-fixture or no-hooks.
contract/fake-agent-receipt/size · Fake-agent receipt · size — Carries cols and rows, written at start and on every terminal size change with no key needed after it; nothing while stdin/stdout is not a terminal, and none in print mode.
contract/fake-agent-receipt/start · Fake-agent receipt · start — Carries cli_version; in interactive mode written after the fake agent has taken raw mode on its own terminal, so tests send no key before it; print mode takes no raw mode and writes it before its four fixture events.
contract/fake-agent-receipt/step · Fake-agent receipt · step — Carries index and event.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/real-filesystem-faults · Filesystem faults are real — A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found. · except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/jobs-end-in-harness-gate · CI jobs end in the harness gate — Each CI test job ends in viola-harness gate requiring its suites.
decision/mutation-not-a-ci-gate · mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
decision/single-instrumented-test-run · one instrumented coverage run in CI — CI's test job is one instrumented nextest run that replaces separate unit, integration and E2E runs; the per-suite agent-run runs are its local equivalents.
contract/ci-artifacts/diag · CI artifacts · diag-${{ matrix.os }} — Holds the E2E homes' diagnostics NDJSON files from the test job, uploaded only behind a passed secret scan.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
decision/ci-jobs-end-in-harness-gate · CI test jobs end in the harness gate — Each test-bearing CI job's last step is viola-harness gate requiring that job's own suites.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/child-process-coverage · Child viola processes counted in coverage — viola processes spawned by E2E tests count toward coverage, so a test that clears the environment must re-add LLVM_PROFILE_FILE. · why: LLVM_PROFILE_FILE propagates to child processes. · except: The harness self-tests' nested cargo over a throwaway crate, whose binaries write no profile into the outer run.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/coverage-includes-child-processes · Coverage includes child viola processes — Child viola processes spawned by tests are included in coverage; a test that clears the environment must re-add LLVM_PROFILE_FILE. · except: The harness self-tests' nested cargo over the throwaway mini crate removes cargo-llvm-cov's four variables, so its binaries write no profile into the outer run.
decision/killed-test-child-uninstrumented · Killed test child is uninstrumented — A test that kills its child straight after the start uses an uninstrumented host program as that child. · why: An instrumented child killed during its exit-time profile write leaves a short profile that llvm-profdata refuses.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/env-clear-keeps-profile-file · Cleared environment keeps LLVM_PROFILE_FILE — A test that calls `.env_clear()` on a child process always re-adds `LLVM_PROFILE_FILE`. · why: Without it the run silently drops out of coverage.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the 11 identity-floor names, the refusal order and exit codes are written as literals in the test.
decision/no-blanket-approval-tests · No blanket-approval tests — No test auto-approves a dialog. · except: The live rig passes a read trust dialog under its key grant, in its rehearsals and its live starts.
decision/no-debug-string-comparison · No comparison by Debug string — Refusal reasons and exit codes are never compared by their Debug string; tests assert on the serde kebab-case value and the integer exit code.
decision/no-process-env-mutation-in-tests · No process-wide env mutation in tests — Tests never set process environment variables globally; environment is set per child command. · why: Setting the process environment is unsafe in edition 2024 and races across threads.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt. · except: The test-only live rig reads its child's screen by compiled literals only into closed words its rehearsal asserts on.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.
decision/wrapper-spawned-with-guard · Long-lived wrapper spawned with a drop guard — Tests start the long-lived `viola run` wrapper by spawning it under a drop guard, never with a blocking output or status call. · why: A blocking call waits until the wrapper exits.
decision/chaos-faults-explicit · Chaos faults only from explicit operations — Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically.
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/no-mocking-unowned-code · No mocking of what is not owned — Libraries and the language runtime are never mocked without an abstraction layer.
decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-tokio-failpoint-crate · No failpoint crate pulling tokio — No failpoint crate that pulls tokio into the sync crates is introduced; fault seams go through mockall doubles of existing traits plus process and file faults.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/canary-list-single-sourced · Single-sourced canary list — Every canary-<kind>-value-<hex4> literal in the root tests and sources must be in the secret scan's table. · guard: crates/viola-e2e/tests/scan_patterns.rs
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/ci-gate-last-step · Gate as the last step of every test job — CI runs the harness gate as the last step of every job that runs a harness suite, requiring exactly the suites that job ran; lint, supply-chain and release are plain exit-code jobs with no gate step.
decision/cleanup-ctrl-c-repeated-until-exit · Cleanup stops agents with a repeated Ctrl-C — Harness cleanup stops each agent child by writing Ctrl-C into its outer PTY and pressing it again every 500 ms until the child exits, then force-kills survivors after a 10 s deadline. · why: A Ctrl-C written into a child that has not set raw mode yet is swallowed.
decision/e2e-homes-under-one-root · All test homes under target/e2e-home — Every harness and rstest home lives under target/e2e-home. · why: It is the root obs-plan's CI gates and upload read.
decision/e2e-homes-under-target-e2e-home · E2E homes under target/e2e-home — Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only. · why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
decision/local-live-runner-seam · Local-live through a runner seam — The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
decision/mutants-tested-by-owning-package · Mutants tested by their own package — Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/agent-run-logs-stream/diag-line · agent-run logs ndjson stream · diag line — Every per-process log line of the home's diagnostics files, wrapped as {"src":"diag","file":<basename>,"record":<line>}.
contract/local-live-failure/row-missing · local-live failure code · row-missing — A closed local-live failure code.
contract/local-live-suite/pass · the local-live suite · pass — Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.
contract/run-suite/local-live · run suite · local-live — Appears only under --local-live and is not accepted by gate --require.
contract/secret-scan-class/claude-stripped · secret-scan hit class · claude-stripped — A hit of the fixed CLAUDE* canaries the root tests plant.
contract/secret-scan-class/content-canary · secret-scan hit class · content-canary — A content canary, failing only in a home-level role file.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail; one verify stamp taken per build keeps each stamped-home test under it.
decision/fixture-git-no-auto-maintenance · Fixture git repos without auto maintenance — Fixture git repositories in tests run git with maintenance.auto=false. · why: The detached auto maintenance each fixture commit spawned wrote into .git while the temp dir's drop walked it, leaving half-removed dirs.
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.

## log-format · key of ## 3. Test Harness Contract
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.

## pid-file · key of ## 3. Test Harness Contract
decision/pid-start-time-verified-kill · Pid and start time verified before a kill — Every kill target is verified by pid and start time through sysinfo 0.39.6 before signalling. · why: A reused pid is never killed.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
decision/verify-under-test-owned-user-home · Test verifies run under a test-owned user home — Every test-started viola verify and every harness boot stamping runs under a test-owned user home whose config file is seeded empty and owner-only, and the fake agent is told that config file. · why: verify's --record scrub reads HOME, so a test-owned HOME would make a recording's absolute paths unclean. · except: A --record verify keeps the real HOME and points a kept CLAUDE_CONFIG_DIR at the test-owned user home instead.
decision/one-verify-stamp-per-build · One verify stamp per build — Stamped test homes copy one viola verify stamp taken per pair of built binaries, kept under target/e2e-home keyed by the binaries' hashes and taken under an exclusive lock. · why: Taking the stamp once keeps every stamped-home test under its unchanged per-test kill. · guard: stamped_home_reuses_one_stamp_for_every_home
decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.
decision/stamping-verify-passes-transcript · Stamping verifies give the fake agent a transcript — Every stamping viola verify against the fake agent, in the harness boot and the stamped-home helpers, passes the fake agent --transcript beside --framing. · why: Without a transcript Run B's echo probe is skipped and the local-command-echo row fails. · except: The framing-free verify helper passes no --transcript.

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
